//! MEI music content to MusicXML conversion.
//!
//! Handles traversal of MEI score/section/measure structure and conversion
//! to MusicXML part/measure format.
//!
//! MEI structure: Score → Section → Measure → Staff(@n) → Layer → Note/Rest/Chord
//! MusicXML structure: Part → Measure → Note/Rest (forward/backup for voices)

use std::collections::HashMap;

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::model::elements::{
    BarStyle, Barline, BarlineLocation, Measure as MxmlMeasure, MeasureContent, TimewiseMeasure,
    TimewisePart,
};
use tusk_model::elements::{
    Ending as MeiEnding, EndingChild, LayerChild, MeasureChild, Score as MeiScore, ScoreChild,
    ScoreDefChild, Section, SectionChild, Staff, StaffChild, StaffGrp, StaffGrpChild,
};

use super::direction::{
    convert_mei_dir, convert_mei_dynam, convert_mei_hairpin, convert_mei_pedal,
    convert_mei_tempo,
};
use super::note::{convert_mei_chord, convert_mei_mrest, convert_mei_note, convert_mei_rest};
use super::structure::convert_mei_measure;
use super::utils::find_score_def;

#[derive(Debug, Clone)]
struct ExportOctaveSpan {
    part_idx: usize,
    staff: usize,
    start_measure: usize,
    start_beat: f64,
    end_measure: usize,
    end_beat: f64,
    octave_delta: i16,
}

#[derive(Debug, Clone, Copy)]
struct MeiEventPosition {
    staff: usize,
    measure_idx: usize,
    beat: f64,
    duration_beats: f64,
}

/// Pre-assign MusicXML slur numbers using interval graph coloring.
///
/// MEI slurs can reference notes in any measure via startid/endid. On MusicXML
/// reimport, slurs are matched by (part_id, staff, number). If two slurs on the
/// same staff have pending starts at the same time, they MUST have different
/// numbers. This function assigns numbers by:
///
/// 1. Building a note-id → measure-index map from all notes/rests/chords
/// 2. Determining each slur's "lifetime" as [start_measure, stop_measure]
/// 3. Per staff, sorting by start_measure and using greedy interval coloring
fn pre_assign_slur_numbers(
    mei_measures: &[&tusk_model::elements::Measure],
) -> HashMap<(String, String), u8> {
    // Build note-id → measure-index map
    let mut note_to_measure: HashMap<String, usize> = HashMap::new();
    for (m_idx, measure) in mei_measures.iter().enumerate() {
        for child in &measure.children {
            if let MeasureChild::Staff(staff) = child {
                for sc in &staff.children {
                    if let StaffChild::Layer(layer) = sc {
                        collect_note_ids_from_layer(&layer.children, m_idx, &mut note_to_measure);
                    }
                }
            }
        }
    }

    // Collect slurs with their staff and lifetime (start_measure, stop_measure)
    struct SlurInfo {
        start_id: String,
        end_id: String,
        staff: usize,
        start_measure: usize,
        stop_measure: usize,
    }

    let mut slur_infos: Vec<SlurInfo> = Vec::new();
    for (m_idx, measure) in mei_measures.iter().enumerate() {
        for child in &measure.children {
            if let MeasureChild::Slur(slur) = child {
                let staff = slur
                    .slur_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(0) as usize;
                let start_id = slur
                    .slur_log
                    .startid
                    .as_ref()
                    .map(|uri| uri.to_string().trim_start_matches('#').to_string());
                let end_id = slur
                    .slur_log
                    .endid
                    .as_ref()
                    .map(|uri| uri.to_string().trim_start_matches('#').to_string());

                if let (Some(sid), Some(eid)) = (start_id, end_id) {
                    let start_m = note_to_measure.get(&sid).copied().unwrap_or(m_idx);
                    let stop_m = note_to_measure.get(&eid).copied().unwrap_or(m_idx);
                    slur_infos.push(SlurInfo {
                        start_id: sid,
                        end_id: eid,
                        staff,
                        start_measure: start_m,
                        stop_measure: stop_m,
                    });
                }
            }
        }
    }

    // Group by staff
    let mut by_staff: HashMap<usize, Vec<&SlurInfo>> = HashMap::new();
    for info in &slur_infos {
        by_staff.entry(info.staff).or_default().push(info);
    }

    // For each staff, sort by start_measure and use greedy interval coloring
    let mut result: HashMap<(String, String), u8> = HashMap::new();
    for slurs in by_staff.values_mut() {
        // Sort by start_measure, then stop_measure
        slurs.sort_by_key(|s| (s.start_measure, s.stop_measure));

        // Greedy coloring: track when each number becomes free
        // number_free_at[i] = first measure index where number i+1 is free
        let mut number_free_at: Vec<usize> = Vec::new();

        for s in slurs.iter() {
            // Find lowest number that's free at start_measure
            let mut assigned = None;
            for (i, free_at) in number_free_at.iter_mut().enumerate() {
                if *free_at <= s.start_measure {
                    *free_at = s.stop_measure + 1;
                    assigned = Some((i + 1) as u8);
                    break;
                }
            }
            let number = assigned.unwrap_or_else(|| {
                number_free_at.push(s.stop_measure + 1);
                number_free_at.len() as u8
            });

            result.insert((s.start_id.clone(), s.end_id.clone()), number);
        }
    }

    result
}

/// Collect note/rest/chord xml:ids from layer children (recursing into beams).
fn collect_note_ids_from_layer(
    children: &[LayerChild],
    measure_idx: usize,
    map: &mut HashMap<String, usize>,
) {
    for child in children {
        match child {
            LayerChild::Note(note) => {
                if let Some(ref id) = note.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
            }
            LayerChild::Rest(rest) => {
                if let Some(ref id) = rest.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
            }
            LayerChild::Chord(chord) => {
                if let Some(ref id) = chord.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
                // Also collect IDs from notes within the chord
                for note in &chord.children {
                    if let tusk_model::elements::ChordChild::Note(n) = note {
                        if let Some(ref id) = n.common.xml_id {
                            map.insert(id.clone(), measure_idx);
                        }
                    }
                }
            }
            LayerChild::Beam(beam) => {
                collect_note_ids_from_beam(&beam.children, measure_idx, map);
            }
            LayerChild::BTrem(btrem) => {
                collect_note_ids_from_btrem(&btrem.children, measure_idx, map);
            }
            LayerChild::FTrem(ftrem) => {
                collect_note_ids_from_ftrem(&ftrem.children, measure_idx, map);
            }
            LayerChild::MRest(mrest) => {
                if let Some(ref id) = mrest.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
            }
            LayerChild::DivLine(_)
            | LayerChild::Space(_)
            | LayerChild::KeySig(_)
            | LayerChild::MeterSig(_)
            | LayerChild::Clef(_) => {}
            _ => {}
        }
    }
}

/// Collect note/rest/chord xml:ids from beam children (recursing into nested beams).
fn collect_note_ids_from_beam(
    children: &[tusk_model::elements::BeamChild],
    measure_idx: usize,
    map: &mut HashMap<String, usize>,
) {
    use tusk_model::elements::BeamChild;

    for child in children {
        match child {
            BeamChild::Note(note) => {
                if let Some(ref id) = note.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
            }
            BeamChild::Rest(rest) => {
                if let Some(ref id) = rest.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
            }
            BeamChild::Chord(chord) => {
                if let Some(ref id) = chord.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
                for note in &chord.children {
                    if let tusk_model::elements::ChordChild::Note(n) = note {
                        if let Some(ref id) = n.common.xml_id {
                            map.insert(id.clone(), measure_idx);
                        }
                    }
                }
            }
            BeamChild::Beam(nested) => {
                collect_note_ids_from_beam(&nested.children, measure_idx, map);
            }
            BeamChild::BTrem(btrem) => {
                collect_note_ids_from_btrem(&btrem.children, measure_idx, map);
            }
            BeamChild::FTrem(ftrem) => {
                collect_note_ids_from_ftrem(&ftrem.children, measure_idx, map);
            }
            _ => {}
        }
    }
}

/// Collect note/chord xml:ids from bTrem children.
fn collect_note_ids_from_btrem(
    children: &[tusk_model::elements::BTremChild],
    measure_idx: usize,
    map: &mut HashMap<String, usize>,
) {
    use tusk_model::elements::BTremChild;
    for child in children {
        match child {
            BTremChild::Note(note) => {
                if let Some(ref id) = note.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
            }
            BTremChild::Chord(chord) => {
                if let Some(ref id) = chord.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
                for note in &chord.children {
                    if let tusk_model::elements::ChordChild::Note(n) = note {
                        if let Some(ref id) = n.common.xml_id {
                            map.insert(id.clone(), measure_idx);
                        }
                    }
                }
            }
        }
    }
}

/// Collect note/chord xml:ids from fTrem children.
fn collect_note_ids_from_ftrem(
    children: &[tusk_model::elements::FTremChild],
    measure_idx: usize,
    map: &mut HashMap<String, usize>,
) {
    use tusk_model::elements::FTremChild;
    for child in children {
        match child {
            FTremChild::Note(note) => {
                if let Some(ref id) = note.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
            }
            FTremChild::Chord(chord) => {
                if let Some(ref id) = chord.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
                for note in &chord.children {
                    if let tusk_model::elements::ChordChild::Note(n) = note {
                        if let Some(ref id) = n.common.xml_id {
                            map.insert(id.clone(), measure_idx);
                        }
                    }
                }
            }
            FTremChild::Clef(_) => {}
        }
    }
}

/// Convert MEI score content to MusicXML timewise measures.
///
/// This collects all measures from MEI sections and produces a list of
/// `TimewiseMeasure` entries where each measure contains per-part content.
/// This is a natural mapping from MEI's measure-centric structure.
///
/// The caller should then use `timewise_to_partwise()` to pivot into the
/// partwise format required by MusicXML serialization.
///
/// ## Staff number handling
///
/// MEI uses global staff numbers (1–N across all parts). MusicXML partwise
/// uses part-local staff numbers (typically 1 for single-staff parts).
/// This function remaps staff numbers to be part-local: each part's staff
/// content is assigned `staff = 1` (or the appropriate local number for
/// multi-staff instruments).
pub fn convert_mei_score_content(
    mei_score: &MeiScore,
    part_ids: &[String],
    ctx: &mut ConversionContext,
) -> ConversionResult<Vec<TimewiseMeasure>> {
    // Collect all MEI measures from sections (and ending spans)
    let (mei_measures, ending_spans) = collect_measures_from_score(mei_score);
    let note_positions = collect_mei_event_positions(&mei_measures);
    let octave_spans = collect_export_octave_spans(&mei_measures, &note_positions, part_ids, ctx);

    // Pre-assign slur numbers using interval graph coloring so that
    // cross-measure slurs get unique numbers on reimport.
    let slur_numbers = pre_assign_slur_numbers(&mei_measures);
    ctx.set_slur_number_map(slur_numbers);

    // Collect staffDefs from scoreDef for initial attributes
    let staff_defs = collect_staff_defs_from_score(mei_score);

    // Get the scoreDef for global key/time signatures
    let score_def = find_score_def(mei_score);

    // Set divisions from first staffDef's ppq before processing measures.
    // If no ppq is specified, calculate a smart default based on the smallest
    // note duration in the score to avoid fractional durations.
    let initial_divs = if let Some(staff_def) = staff_defs.first() {
        staff_def
            .staff_def_ges
            .ppq
            .as_ref()
            .and_then(|s| s.parse().ok())
            .map(|x: f64| x)
    } else {
        None
    };
    let divs = initial_divs.unwrap_or_else(|| calculate_smart_divisions(&mei_measures));
    ctx.set_divisions(divs);

    // Extract dur.default and oct.default from scoreDef → staffDef hierarchy.
    // staffDef overrides scoreDef; we use the first staffDef as representative.
    let mut dur_default = score_def.and_then(|sd| sd.score_def_log.dur_default.clone());
    let mut oct_default = score_def.and_then(|sd| sd.score_def_log.oct_default.clone());
    if let Some(sd) = staff_defs.first() {
        if sd.staff_def_log.dur_default.is_some() {
            dur_default = sd.staff_def_log.dur_default.clone();
        }
        if sd.staff_def_log.oct_default.is_some() {
            oct_default = sd.staff_def_log.oct_default.clone();
        }
    }
    ctx.set_dur_default(dur_default);
    ctx.set_oct_default(oct_default);

    // Track previous measures per part for cross-measure slur resolution.
    // Key: part index, Value: accumulated measures for that part.
    //
    // IMPORTANT: We build the timewise output from these accumulated measures
    // AFTER all measures are processed, not inside the loop. This is because
    // cross-measure slurs retroactively attach start notations to notes in
    // previous measures. If we consumed measure content eagerly, those
    // retroactive modifications would be lost.
    let mut part_prev_measures: Vec<Vec<MxmlMeasure>> =
        part_ids.iter().map(|_| Vec::new()).collect();

    // Collect measure metadata (number, implicit, etc.) for building output later
    struct MeasureMeta {
        number: String,
        text: Option<String>,
        implicit: Option<crate::model::data::YesNo>,
        non_controlling: Option<crate::model::data::YesNo>,
        width: Option<f64>,
    }
    let mut measure_metas: Vec<MeasureMeta> = Vec::new();

    // For each MEI measure, convert content and accumulate in part_prev_measures
    for (measure_idx, mei_measure) in mei_measures.iter().enumerate() {
        // Convert measure attributes (number, implicit, width, etc.)
        let mxml_measure_base = convert_mei_measure(mei_measure, "", ctx)?;

        measure_metas.push(MeasureMeta {
            number: mxml_measure_base.number.clone(),
            text: mxml_measure_base.text.clone(),
            implicit: mxml_measure_base.implicit,
            non_controlling: mxml_measure_base.non_controlling,
            width: mxml_measure_base.width,
        });

        // For each MusicXML part, extract its staff/staves content from the MEI measure.
        // Multi-staff parts (e.g., piano) have multiple MEI staves that must be merged
        // into a single MusicXML part with <backup> elements between staves.
        for (part_idx, part_id) in part_ids.iter().enumerate() {
            ctx.set_part(part_id.as_str());
            let num_staves = ctx.staves_for_part(part_id);

            // Set per-part divisions from the first staffDef of this part
            let first_global_staff = ctx
                .global_staff_for_part(part_id, 1)
                .unwrap_or((part_idx + 1) as u32) as usize;
            let first_staff_def_idx = staff_defs
                .iter()
                .position(|sd| {
                    sd.n_integer
                        .n
                        .as_ref()
                        .and_then(|n| n.parse::<usize>().ok())
                        == Some(first_global_staff)
                })
                .unwrap_or(part_idx);
            if let Some(staff_def) = staff_defs.get(first_staff_def_idx) {
                if let Some(ppq) = staff_def
                    .staff_def_ges
                    .ppq
                    .as_ref()
                    .and_then(|s| s.parse().ok())
                {
                    ctx.set_divisions(ppq);
                }
            }

            // Build a MxmlMeasure to collect content
            let mut mxml_measure = MxmlMeasure::new(&measure_metas[measure_idx].number);

            if num_staves <= 1 {
                // Single-staff part: existing logic
                let global_staff_n = first_global_staff;
                let local_staff_n = 1_usize;

                convert_direction_events(
                    mei_measure,
                    global_staff_n,
                    local_staff_n,
                    &mut mxml_measure,
                    ctx,
                )?;

                if let Some(staff) = find_staff_in_measure(mei_measure, global_staff_n) {
                    convert_staff_content(
                        staff,
                        local_staff_n,
                        num_staves as usize,
                        1, // single-staff: max_layers not used for voice offset
                        &mut mxml_measure,
                        ctx,
                    )?;
                }

                convert_slur_events(
                    mei_measure,
                    global_staff_n,
                    &mut mxml_measure,
                    &mut part_prev_measures[part_idx],
                    ctx,
                )?;

                convert_tuplet_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;

                convert_ornament_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;

                convert_fermata_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                convert_breath_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                convert_caesura_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                convert_arpeg_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                convert_gliss_events(
                    mei_measure,
                    global_staff_n,
                    &mut mxml_measure,
                    &mut part_prev_measures[part_idx],
                    ctx,
                )?;
                convert_technical_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                convert_notation_dynamics(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
            } else {
                // Multi-staff part: merge multiple staves with <backup> between them.
                // Pre-compute max layer count across all staves for unique voice numbering.
                let max_layers = (1..=num_staves)
                    .filter_map(|ls| {
                        let gs = ctx.global_staff_for_part(part_id, ls)? as usize;
                        find_staff_in_measure(mei_measure, gs)
                            .map(|s| s.children.len())
                    })
                    .max()
                    .unwrap_or(1);

                for local_staff in 1..=num_staves {
                    let global_staff_n =
                        ctx.global_staff_for_part(part_id, local_staff).unwrap() as usize;
                    let local_staff_n = local_staff as usize;

                    // Direction events for this staff
                    convert_direction_events(
                        mei_measure,
                        global_staff_n,
                        local_staff_n,
                        &mut mxml_measure,
                        ctx,
                    )?;

                    // Content offset before adding this staff's notes
                    let content_before = mxml_measure.content.len();

                    // Staff content
                    if let Some(staff) = find_staff_in_measure(mei_measure, global_staff_n) {
                        convert_staff_content(
                            staff,
                            local_staff_n,
                            num_staves as usize,
                            max_layers,
                            &mut mxml_measure,
                            ctx,
                        )?;
                    }

                    // Slur events
                    convert_slur_events(
                        mei_measure,
                        global_staff_n,
                        &mut mxml_measure,
                        &mut part_prev_measures[part_idx],
                        ctx,
                    )?;

                    // Tuplet events
                    convert_tuplet_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;

                    // Ornament events
                    convert_ornament_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;

                    // Fermata, breath, caesura, arpeg, gliss events
                    convert_fermata_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                    convert_breath_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                    convert_caesura_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                    convert_arpeg_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                    convert_gliss_events(
                        mei_measure,
                        global_staff_n,
                        &mut mxml_measure,
                        &mut part_prev_measures[part_idx],
                        ctx,
                    )?;
                    convert_technical_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
                    convert_notation_dynamics(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;

                    // Insert <backup> between staves (not after the last one)
                    if local_staff < num_staves {
                        let staff_duration =
                            calculate_staff_duration(&mxml_measure, content_before);
                        if staff_duration > 0.0 {
                            mxml_measure.content.push(MeasureContent::Backup(Box::new(
                                crate::model::note::Backup {
                                    duration: staff_duration,
                                    footnote: None,
                                    level: None,
                                },
                            )));
                        }
                    }
                }
            }

            // Add attributes to first measure of each part
            if measure_idx == 0 {
                if num_staves <= 1 {
                    let attrs = super::attributes::build_first_measure_attributes(
                        score_def,
                        staff_defs.get(first_staff_def_idx).copied(),
                        ctx,
                    );
                    mxml_measure
                        .content
                        .insert(0, MeasureContent::Attributes(Box::new(attrs)));
                } else {
                    let attrs = super::attributes::build_first_measure_attributes_multi(
                        score_def,
                        part_id,
                        num_staves,
                        &staff_defs,
                        ctx,
                    );
                    mxml_measure
                        .content
                        .insert(0, MeasureContent::Attributes(Box::new(attrs)));
                }
            }

            // Prepend left barline if present (MusicXML: first element in measure)
            if let Some(rend) = mei_measure.measure_log.left {
                let barline = mei_barrendition_to_barline(rend, BarlineLocation::Left);
                mxml_measure
                    .content
                    .insert(0, MeasureContent::Barline(Box::new(barline)));
            }
            // Append right barline if present
            if let Some(rend) = mei_measure.measure_log.right {
                let barline = mei_barrendition_to_barline(rend, BarlineLocation::Right);
                mxml_measure
                    .content
                    .push(MeasureContent::Barline(Box::new(barline)));
            }

            // Enrich basic barlines with extras from ExtensionStore
            let measure_n = &measure_metas[measure_idx].number;
            for loc_str in ["left", "right", "middle"] {
                let key = format!("barline:{measure_n}:{loc_str}");
                if let Some(data) = ctx.ext_store().barline(&key) {
                    let full_barline = build_barline_from_data(data);
                    let loc = full_barline.location.unwrap_or(BarlineLocation::Right);
                    replace_barline_at_location(
                        &mut mxml_measure.content,
                        &full_barline,
                        loc,
                    );
                }
            }

            apply_export_octave_spans_to_measure(
                measure_idx,
                part_idx,
                &mut mxml_measure,
                &octave_spans,
                &note_positions,
            );

            // Store the measure for future cross-measure slur resolution
            part_prev_measures[part_idx].push(mxml_measure);
        }
    }

    apply_octave_directions_to_parts(&mei_measures, &note_positions, part_ids, &mut part_prev_measures, ctx);

    // Apply ending/volta brackets from MEI <ending> containers.
    // For each ending span, add <barline><ending> to the first/last measures
    // of every part.
    for span in &ending_spans {
        apply_ending_barlines(span, &mut part_prev_measures);
    }

    // Build the timewise output from the accumulated measures.
    // At this point, all cross-measure slur notations have been retroactively
    // attached, so the measure content is complete.
    let mut timewise_measures = Vec::new();
    for (measure_idx, meta) in measure_metas.iter().enumerate() {
        let mut tw_measure = TimewiseMeasure {
            number: meta.number.clone(),
            text: meta.text.clone(),
            implicit: meta.implicit,
            non_controlling: meta.non_controlling,
            width: meta.width,
            parts: Vec::new(),
        };

        for (staff_idx, part_id) in part_ids.iter().enumerate() {
            let content = std::mem::take(&mut part_prev_measures[staff_idx][measure_idx].content);
            tw_measure.parts.push(TimewisePart {
                id: part_id.clone(),
                content,
            });
        }

        timewise_measures.push(tw_measure);
    }

    Ok(timewise_measures)
}

/// Ending boundary info to emit `<barline><ending>` on MusicXML measures.
///
/// Tracks the flat measure-index range covered by an MEI `<ending>` and
/// the ending metadata (number, label) needed for MusicXML `<ending>` elements.
struct EndingSpan {
    /// First measure index (inclusive) in the flat measure list.
    first: usize,
    /// Last measure index (inclusive) in the flat measure list.
    last: usize,
    /// Ending number from MEI `@n` (e.g. "1", "1, 2").
    number: String,
    /// Display label from MEI `@label` (optional text on the volta bracket).
    label: Option<String>,
    /// Stop type from MEI `@type`: "stop", "discontinue", or None (open-ended).
    stop_type: Option<String>,
}

/// Collect all measures from an MEI score by traversing sections.
///
/// Also returns ending spans that describe which measure ranges are wrapped
/// in MEI `<ending>` containers.
fn collect_measures_from_score(
    mei_score: &MeiScore,
) -> (Vec<&tusk_model::elements::Measure>, Vec<EndingSpan>) {
    let mut measures = Vec::new();
    let mut endings = Vec::new();

    for child in &mei_score.children {
        if let ScoreChild::Section(section) = child {
            collect_measures_from_section(section, &mut measures, &mut endings);
        }
    }

    (measures, endings)
}

/// Collect staffDefs from the scoreDef in an MEI score.
///
/// Returns a vector of staffDef references, ordered by staff number.
fn collect_staff_defs_from_score(mei_score: &MeiScore) -> Vec<&tusk_model::elements::StaffDef> {
    let mut staff_defs = Vec::new();

    // Find scoreDef in score children
    if let Some(score_def) = find_score_def(mei_score) {
        // Find staffGrp in scoreDef
        for child in &score_def.children {
            if let ScoreDefChild::StaffGrp(staff_grp) = child {
                collect_staff_defs_from_staff_grp(staff_grp, &mut staff_defs);
            }
        }
    }

    // Sort by @n attribute to ensure correct order; MEI @n is Option<String>
    staff_defs.sort_by_key(|sd| {
        sd.n_integer
            .n
            .as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    });

    staff_defs
}

/// Recursively collect staffDefs from a staffGrp.
fn collect_staff_defs_from_staff_grp<'a>(
    staff_grp: &'a StaffGrp,
    staff_defs: &mut Vec<&'a tusk_model::elements::StaffDef>,
) {
    for child in &staff_grp.children {
        match child {
            StaffGrpChild::StaffDef(staff_def) => {
                staff_defs.push(staff_def);
            }
            StaffGrpChild::StaffGrp(nested_grp) => {
                collect_staff_defs_from_staff_grp(nested_grp, staff_defs);
            }
            // These StaffGrp children carry display/grouping metadata only —
            // not needed when collecting staffDefs for part→staff mapping.
            StaffGrpChild::GrpSym(_) | StaffGrpChild::Label(_) | StaffGrpChild::LabelAbbr(_) => {}
            _ => {}
        }
    }
}

/// Recursively collect measures from a section (sections can be nested).
fn collect_measures_from_section<'a>(
    section: &'a Section,
    measures: &mut Vec<&'a tusk_model::elements::Measure>,
    endings: &mut Vec<EndingSpan>,
) {
    for child in &section.children {
        match child {
            SectionChild::Measure(measure) => {
                measures.push(measure);
            }
            SectionChild::Section(nested_section) => {
                collect_measures_from_section(nested_section, measures, endings);
            }
            SectionChild::Ending(ending) => {
                collect_measures_from_ending(ending, measures, endings);
            }
            // Expansion defines playback ordering — no MusicXML equivalent.
            SectionChild::Expansion(_) => {
                tracing::debug!("Skipping MEI <expansion> — no MusicXML equivalent");
            }
            _ => {}
        }
    }
}

/// Collect measures from an MEI `<ending>` and record its span.
fn collect_measures_from_ending<'a>(
    ending: &'a MeiEnding,
    measures: &mut Vec<&'a tusk_model::elements::Measure>,
    endings: &mut Vec<EndingSpan>,
) {
    let first = measures.len();

    for child in &ending.children {
        match child {
            EndingChild::Measure(measure) => {
                measures.push(measure);
            }
            EndingChild::Section(nested_section) => {
                collect_measures_from_section(nested_section, measures, endings);
            }
            EndingChild::Expansion(_) => {
                tracing::debug!("Skipping MEI <expansion> inside <ending>");
            }
            _ => {}
        }
    }

    let last = measures.len().saturating_sub(1);
    if first <= last {
        endings.push(EndingSpan {
            first,
            last,
            number: ending
                .common
                .n
                .as_ref()
                .map(|w| w.0.as_str())
                .unwrap_or("1")
                .to_string(),
            label: ending.common.label.clone(),
            stop_type: ending.common.r#type.clone().or_else(|| {
                // Derive from @lendsym if @type absent (externally-authored MEI)
                use tusk_model::data::DataLinestartendsymbol;
                match ending.ending_vis.lendsym {
                    Some(DataLinestartendsymbol::Angledown) => Some("stop".to_string()),
                    Some(DataLinestartendsymbol::None) => Some("discontinue".to_string()),
                    _ => None,
                }
            }),
        });
    }
}

/// Apply `<barline><ending>` to the first and last measures of an ending span.
///
/// MusicXML represents volta brackets as `<ending>` children of `<barline>` elements:
/// - First measure: `<barline location="left"><ending number="N" type="start">text</ending>`
/// - Last measure: `<barline location="right"><ending number="N" type="stop"/>` (or "discontinue")
///
/// This adds ending data to existing barlines, or creates new barlines if needed.
fn apply_ending_barlines(span: &EndingSpan, part_measures: &mut [Vec<MxmlMeasure>]) {
    use crate::model::data::StartStopDiscontinue;
    use crate::model::elements::Ending as MxmlEnding;

    let start_ending = MxmlEnding {
        number: span.number.clone(),
        ending_type: StartStopDiscontinue::Start,
        text: span.label.clone(),
        default_y: None,
        end_length: None,
        print_object: None,
        default_x: None,
        text_x: None,
        text_y: None,
    };

    // Determine stop type from @type: "stop" draws closing line, "discontinue" leaves open.
    // Only emit a stop ending if the original had one (None = open-ended volta).
    let stop_ending = span.stop_type.as_ref().map(|st| {
        let ending_type = if st == "discontinue" {
            StartStopDiscontinue::Discontinue
        } else {
            StartStopDiscontinue::Stop
        };
        MxmlEnding {
            number: span.number.clone(),
            ending_type,
            text: None,
            default_y: None,
            end_length: None,
            print_object: None,
            default_x: None,
            text_x: None,
            text_y: None,
        }
    });

    // Apply to every part's measures
    for part_measures in part_measures.iter_mut() {
        // Start ending on the left barline of the first measure
        if let Some(measure) = part_measures.get_mut(span.first) {
            add_ending_to_barline(&mut measure.content, &start_ending, BarlineLocation::Left);
        }
        // Stop ending on the right barline of the last measure (only if explicitly closed)
        if let Some(ref stop) = stop_ending {
            if let Some(measure) = part_measures.get_mut(span.last) {
                add_ending_to_barline(&mut measure.content, stop, BarlineLocation::Right);
            }
        }
    }
}

/// Add an `<ending>` element to an existing barline at the given location,
/// or create a new barline with the ending if none exists.
fn add_ending_to_barline(
    content: &mut Vec<MeasureContent>,
    ending: &crate::model::elements::Ending,
    location: BarlineLocation,
) {
    // Find existing barline at this location
    for item in content.iter_mut() {
        if let MeasureContent::Barline(bl) = item {
            if bl.location == Some(location) {
                bl.ending = Some(ending.clone());
                return;
            }
        }
    }
    // No barline at this location — create one with the ending
    let barline = Barline {
        location: Some(location),
        ending: Some(ending.clone()),
        ..Barline::default()
    };
    let barline_content = MeasureContent::Barline(Box::new(barline));
    match location {
        BarlineLocation::Left => content.insert(0, barline_content),
        _ => content.push(barline_content),
    }
}

/// Find a staff with the given @n attribute in an MEI measure.
fn find_staff_in_measure(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
) -> Option<&Staff> {
    for child in &mei_measure.children {
        if let MeasureChild::Staff(staff) = child {
            // Check if this staff has the matching @n; MEI @n is Option<String>
            if let Some(n) = staff
                .n_integer
                .n
                .as_ref()
                .and_then(|s| s.parse::<usize>().ok())
            {
                if n == staff_n {
                    return Some(staff);
                }
            }
        }
    }
    None
}

/// Map MEI DataBarrendition to MusicXML Barline (with location).
fn mei_barrendition_to_barline(
    rend: tusk_model::data::DataBarrendition,
    location: BarlineLocation,
) -> Barline {
    use crate::model::elements::{BackwardForward, Repeat};
    use tusk_model::data::DataBarrendition;

    let (bar_style, repeat) = match rend {
        DataBarrendition::Single => (BarStyle::Regular, None),
        DataBarrendition::Dotted => (BarStyle::Dotted, None),
        DataBarrendition::Dashed => (BarStyle::Dashed, None),
        DataBarrendition::Heavy => (BarStyle::Heavy, None),
        DataBarrendition::Dbl => (BarStyle::LightLight, None),
        DataBarrendition::Dblheavy => (BarStyle::HeavyHeavy, None),
        DataBarrendition::Invis => (BarStyle::None, None),
        DataBarrendition::Dbldashed => (BarStyle::Dashed, None),
        DataBarrendition::Dbldotted => (BarStyle::Dotted, None),
        DataBarrendition::End => (BarStyle::LightHeavy, None),
        DataBarrendition::Rptstart => (
            BarStyle::HeavyLight,
            Some(Repeat {
                direction: BackwardForward::Forward,
                times: None,
                after_jump: None,
                winged: None,
            }),
        ),
        DataBarrendition::Rptend => (
            BarStyle::LightHeavy,
            Some(Repeat {
                direction: BackwardForward::Backward,
                times: None,
                after_jump: None,
                winged: None,
            }),
        ),
        DataBarrendition::Rptboth => (
            BarStyle::LightHeavy,
            Some(Repeat {
                direction: if location == BarlineLocation::Left {
                    BackwardForward::Forward
                } else {
                    BackwardForward::Backward
                },
                times: None,
                after_jump: None,
                winged: None,
            }),
        ),
        DataBarrendition::Segno | DataBarrendition::Dblsegno => (BarStyle::Regular, None),
    };
    Barline {
        location: Some(location),
        bar_style: Some(bar_style),
        repeat,
        ..Barline::default()
    }
}

/// Build a MusicXML `Barline` from typed `BarlineData`.
fn build_barline_from_data(data: &tusk_model::musicxml_ext::BarlineData) -> Barline {
    use crate::model::data::YesNo;
    use crate::model::elements::{BackwardForward, Ending, Repeat, Winged};

    let location = data.location.as_deref().map(|s| match s {
        "left" => BarlineLocation::Left,
        "middle" => BarlineLocation::Middle,
        _ => BarlineLocation::Right,
    });

    let bar_style = data
        .bar_style
        .as_deref()
        .and_then(crate::model::elements::BarStyle::from_musicxml_str);

    let repeat = data.repeat.as_ref().map(|r| Repeat {
        direction: if r.direction == "forward" {
            BackwardForward::Forward
        } else {
            BackwardForward::Backward
        },
        times: r.times,
        after_jump: r.after_jump.map(|b| if b { YesNo::Yes } else { YesNo::No }),
        winged: r.winged.as_deref().and_then(|w| {
            serde_json::from_value::<Winged>(serde_json::Value::String(w.to_string())).ok()
        }),
    });

    let ending = data.ending.as_ref().map(|e| {
        let ending_type = match e.ending_type.as_str() {
            "stop" => crate::model::data::StartStopDiscontinue::Stop,
            "discontinue" => crate::model::data::StartStopDiscontinue::Discontinue,
            _ => crate::model::data::StartStopDiscontinue::Start,
        };
        Ending {
            number: e.number.clone(),
            ending_type,
            text: e.text.clone(),
            default_y: None,
            end_length: None,
            print_object: None,
            default_x: None,
            text_x: None,
            text_y: None,
        }
    });

    let fermatas = data.fermatas.clone();

    let segno = data.segno.clone();

    let coda = data.coda.clone();

    let wavy_line = data.wavy_line.clone();

    Barline {
        location,
        bar_style,
        repeat,
        ending,
        fermatas,
        segno,
        coda,
        wavy_line,
        segno_attr: data.segno_attr.clone(),
        coda_attr: data.coda_attr.clone(),
        divisions: data.divisions,
        footnote: None,
        level: None,
    }
}

/// Replace a basic barline in the measure content with the full decorated barline.
///
/// During export, basic barlines are generated from MEI @left/@right attributes.
/// When a barline dir carries the full JSON, the decorated version replaces
/// the basic one at the same location.
fn replace_barline_at_location(
    content: &mut Vec<MeasureContent>,
    full_barline: &Barline,
    location: BarlineLocation,
) {
    // Find and replace the existing barline at this location
    let mut found = false;
    for item in content.iter_mut() {
        if let MeasureContent::Barline(bl) = item {
            if bl.location == Some(location) {
                **bl = full_barline.clone();
                found = true;
                break;
            }
        }
    }
    // If no existing barline at this location, append/prepend
    if !found {
        let barline_content = MeasureContent::Barline(Box::new(full_barline.clone()));
        match location {
            BarlineLocation::Left => content.insert(0, barline_content),
            _ => content.push(barline_content),
        }
    }
}

fn collect_mei_event_positions(
    mei_measures: &[&tusk_model::elements::Measure],
) -> HashMap<String, MeiEventPosition> {
    let mut positions = HashMap::new();

    for (measure_idx, measure) in mei_measures.iter().enumerate() {
        for child in &measure.children {
            let MeasureChild::Staff(staff) = child else {
                continue;
            };
            for staff_child in &staff.children {
                let StaffChild::Layer(layer) = staff_child else {
                    continue;
                };
                let quarter_ppq = estimate_layer_quarter_ppq(&layer.children);
                collect_layer_event_positions(
                    &layer.children,
                    measure_idx,
                    staff
                        .n_integer
                        .n
                        .as_deref()
                        .and_then(|value| value.parse::<usize>().ok())
                        .unwrap_or(1),
                    0.0,
                    quarter_ppq,
                    &mut positions,
                );
            }
        }
    }

    positions
}

fn collect_export_octave_spans(
    mei_measures: &[&tusk_model::elements::Measure],
    note_positions: &HashMap<String, MeiEventPosition>,
    part_ids: &[String],
    ctx: &ConversionContext,
) -> Vec<ExportOctaveSpan> {
    let mut spans = Vec::new();

    for (measure_idx, measure) in mei_measures.iter().enumerate() {
        for child in &measure.children {
            let MeasureChild::Octave(octave) = child else {
                continue;
            };

            let Some(global_staff) = octave
                .octave_log
                .staff
                .as_deref()
                .and_then(|value| value.split_whitespace().next())
                .and_then(|value| value.parse::<usize>().ok())
            else {
                continue;
            };
            let Some((part_idx, local_staff)) =
                resolve_part_and_local_staff(global_staff, part_ids, ctx)
            else {
                continue;
            };
            let start_position =
                if let Some(start_beat) = octave.octave_log.tstamp.as_ref().map(|beat| beat.0) {
                    Some((measure_idx, start_beat))
                } else {
                    octave
                        .octave_log
                        .startid
                        .as_ref()
                        .and_then(|id| note_positions.get(strip_uri_fragment(&id.0)))
                        .map(|pos| (pos.measure_idx, pos.beat))
                };
            let Some((start_measure, start_beat)) = start_position else {
                continue;
            };
            let end_position = if let Some(endid) = octave.octave_log.endid.as_ref() {
                note_positions
                    .get(strip_uri_fragment(&endid.0))
                    .map(|pos| (pos.measure_idx, pos.beat + pos.duration_beats))
            } else if let Some(tstamp2) = octave.octave_log.tstamp2.as_ref() {
                let (measures_ahead, end_beat) = parse_measurebeat(&tstamp2.0);
                let end_measure = measure_idx + measures_ahead;
                Some((end_measure, end_beat))
            } else {
                octave
                    .octave_log
                    .endid
                    .as_ref()
                    .and_then(|id| note_positions.get(strip_uri_fragment(&id.0)))
                    .map(|pos| (pos.measure_idx, pos.beat + pos.duration_beats))
            };
            let Some((end_measure, end_beat)) = end_position else {
                continue;
            };
            let displacement = octave.octave_log.dis.as_ref().map(|dis| dis.0).unwrap_or(8);
            let octave_steps = match displacement {
                15 => 2,
                22 => 3,
                n if n >= 8 => (n / 8) as i16,
                _ => 1,
            };
            let Some(dis_place) = octave.octave_log.dis_place else {
                continue;
            };
            let octave_delta = match dis_place {
                tusk_model::data::DataStaffrelBasic::Above => octave_steps,
                tusk_model::data::DataStaffrelBasic::Below => -octave_steps,
            };

            spans.push(ExportOctaveSpan {
                part_idx,
                staff: local_staff,
                start_measure,
                start_beat,
                end_measure,
                end_beat,
                octave_delta,
            });
        }
    }

    spans
}

fn apply_export_octave_spans_to_measure(
    measure_idx: usize,
    part_idx: usize,
    mxml_measure: &mut MxmlMeasure,
    spans: &[ExportOctaveSpan],
    note_positions: &HashMap<String, MeiEventPosition>,
) {
    if spans.is_empty() {
        return;
    }

    let mut beat_position = 0.0;
    let mut current_event_beat = 1.0;
    for content in &mut mxml_measure.content {
        match content {
            MeasureContent::Note(note) => {
                let staff = note.staff.unwrap_or(1) as usize;
                let beat = note
                    .id
                    .as_ref()
                    .and_then(|id| note_positions.get(id))
                    .filter(|pos| pos.measure_idx == measure_idx)
                    .map(|pos| pos.beat)
                    .unwrap_or_else(|| {
                        if note.is_chord() {
                            current_event_beat
                        } else {
                            beat_position + 1.0
                        }
                    });
                let octave_delta: i16 = spans
                    .iter()
                    .filter(|span| {
                        span.part_idx == part_idx
                            && span.staff == staff
                            && span_contains_position_with_end(span, measure_idx, beat, span.end_beat)
                    })
                    .map(|span| span.octave_delta)
                    .sum();
                if octave_delta != 0 {
                    transpose_musicxml_note_pitch(note, octave_delta);
                }

                if !note.is_chord() && !note.is_grace() {
                    current_event_beat = beat;
                    beat_position += note.duration.unwrap_or(0.0);
                }
            }
            MeasureContent::Backup(backup) => {
                beat_position -= backup.duration;
            }
            MeasureContent::Forward(forward) => {
                beat_position += forward.duration;
            }
            MeasureContent::Attributes(_)
            | MeasureContent::Direction(_)
            | MeasureContent::Harmony(_)
            | MeasureContent::FiguredBass(_)
            | MeasureContent::Print(_)
            | MeasureContent::Sound(_)
            | MeasureContent::Listening(_)
            | MeasureContent::Barline(_)
            | MeasureContent::Grouping(_)
            | MeasureContent::Link(_)
            | MeasureContent::Bookmark(_) => {}
        }
    }
}

fn collect_layer_event_positions(
    children: &[LayerChild],
    measure_idx: usize,
    staff: usize,
    mut beat_cursor: f64,
    quarter_ppq: Option<f64>,
    positions: &mut HashMap<String, MeiEventPosition>,
) -> f64 {
    for child in children {
        let child_start = beat_cursor + 1.0;
        match child {
            LayerChild::Note(note) => {
                let duration_beats = mei_note_duration_beats(note, quarter_ppq);
                let beat = note
                    .note_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = note.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                beat_cursor += duration_beats;
            }
            LayerChild::Chord(chord) => {
                let duration_beats = mei_chord_duration_beats(chord, quarter_ppq);
                let beat = chord
                    .chord_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = chord.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                for chord_child in &chord.children {
                    let tusk_model::elements::ChordChild::Note(note) = chord_child else {
                        continue;
                    };
                    if let Some(id) = note.common.xml_id.as_ref() {
                        positions.insert(
                            id.clone(),
                            MeiEventPosition {
                                staff,
                                measure_idx,
                                beat,
                                duration_beats,
                            },
                        );
                    }
                }
                beat_cursor += duration_beats;
            }
            LayerChild::Rest(rest) => beat_cursor += mei_rest_duration_beats(rest, quarter_ppq),
            LayerChild::MRest(_) => beat_cursor += 4.0,
            LayerChild::Space(space) => beat_cursor += mei_space_duration_beats(space, quarter_ppq),
            LayerChild::Beam(beam) => {
                beat_cursor =
                    collect_beam_event_positions(&beam.children, measure_idx, staff, beat_cursor, quarter_ppq, positions);
            }
            LayerChild::Tuplet(tuplet) => {
                beat_cursor = collect_tuplet_event_positions(
                    &tuplet.children,
                    measure_idx,
                    staff,
                    beat_cursor,
                    quarter_ppq,
                    positions,
                );
            }
            LayerChild::BTrem(btrem) => {
                beat_cursor =
                    collect_btrem_event_positions(&btrem.children, measure_idx, staff, beat_cursor, quarter_ppq, positions);
            }
            LayerChild::FTrem(ftrem) => {
                beat_cursor =
                    collect_ftrem_event_positions(&ftrem.children, measure_idx, staff, beat_cursor, quarter_ppq, positions);
            }
            _ => {}
        }
    }

    beat_cursor
}

fn collect_beam_event_positions(
    children: &[tusk_model::elements::BeamChild],
    measure_idx: usize,
    staff: usize,
    mut beat_cursor: f64,
    quarter_ppq: Option<f64>,
    positions: &mut HashMap<String, MeiEventPosition>,
) -> f64 {
    use tusk_model::elements::BeamChild;

    for child in children {
        let child_start = beat_cursor + 1.0;
        match child {
            BeamChild::Note(note) => {
                let duration_beats = mei_note_duration_beats(note, quarter_ppq);
                let beat = note
                    .note_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = note.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                beat_cursor += duration_beats;
            }
            BeamChild::Chord(chord) => {
                let duration_beats = mei_chord_duration_beats(chord, quarter_ppq);
                let beat = chord
                    .chord_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = chord.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                for chord_child in &chord.children {
                    let tusk_model::elements::ChordChild::Note(note) = chord_child else {
                        continue;
                    };
                    if let Some(id) = note.common.xml_id.as_ref() {
                        positions.insert(
                            id.clone(),
                            MeiEventPosition {
                                staff,
                                measure_idx,
                                beat,
                                duration_beats,
                            },
                        );
                    }
                }
                beat_cursor += duration_beats;
            }
            BeamChild::Rest(rest) => beat_cursor += mei_rest_duration_beats(rest, quarter_ppq),
            BeamChild::Beam(beam) => {
                beat_cursor =
                    collect_beam_event_positions(&beam.children, measure_idx, staff, beat_cursor, quarter_ppq, positions);
            }
            BeamChild::BTrem(btrem) => {
                beat_cursor =
                    collect_btrem_event_positions(&btrem.children, measure_idx, staff, beat_cursor, quarter_ppq, positions);
            }
            BeamChild::FTrem(ftrem) => {
                beat_cursor =
                    collect_ftrem_event_positions(&ftrem.children, measure_idx, staff, beat_cursor, quarter_ppq, positions);
            }
            _ => {}
        }
    }

    beat_cursor
}

fn collect_tuplet_event_positions(
    children: &[tusk_model::elements::TupletChild],
    measure_idx: usize,
    staff: usize,
    mut beat_cursor: f64,
    quarter_ppq: Option<f64>,
    positions: &mut HashMap<String, MeiEventPosition>,
) -> f64 {
    use tusk_model::elements::TupletChild;

    for child in children {
        let child_start = beat_cursor + 1.0;
        match child {
            TupletChild::Note(note) => {
                let duration_beats = mei_note_duration_beats(note, quarter_ppq);
                let beat = note
                    .note_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = note.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                beat_cursor += duration_beats;
            }
            TupletChild::Beam(beam) => {
                beat_cursor =
                    collect_beam_event_positions(&beam.children, measure_idx, staff, beat_cursor, quarter_ppq, positions);
            }
            TupletChild::BTrem(btrem) => {
                beat_cursor =
                    collect_btrem_event_positions(&btrem.children, measure_idx, staff, beat_cursor, quarter_ppq, positions);
            }
            _ => {}
        }
    }

    beat_cursor
}

fn collect_btrem_event_positions(
    children: &[tusk_model::elements::BTremChild],
    measure_idx: usize,
    staff: usize,
    mut beat_cursor: f64,
    quarter_ppq: Option<f64>,
    positions: &mut HashMap<String, MeiEventPosition>,
) -> f64 {
    use tusk_model::elements::BTremChild;

    for child in children {
        let child_start = beat_cursor + 1.0;
        match child {
            BTremChild::Note(note) => {
                let duration_beats = mei_note_duration_beats(note, quarter_ppq);
                let beat = note
                    .note_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = note.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                beat_cursor += duration_beats;
            }
            BTremChild::Chord(chord) => {
                let duration_beats = mei_chord_duration_beats(chord, quarter_ppq);
                let beat = chord
                    .chord_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = chord.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                for chord_child in &chord.children {
                    let tusk_model::elements::ChordChild::Note(note) = chord_child else {
                        continue;
                    };
                    if let Some(id) = note.common.xml_id.as_ref() {
                        positions.insert(
                            id.clone(),
                            MeiEventPosition {
                                staff,
                                measure_idx,
                                beat,
                                duration_beats,
                            },
                        );
                    }
                }
                beat_cursor += duration_beats;
            }
        }
    }

    beat_cursor
}

fn collect_ftrem_event_positions(
    children: &[tusk_model::elements::FTremChild],
    measure_idx: usize,
    staff: usize,
    mut beat_cursor: f64,
    quarter_ppq: Option<f64>,
    positions: &mut HashMap<String, MeiEventPosition>,
) -> f64 {
    use tusk_model::elements::FTremChild;

    for child in children {
        let child_start = beat_cursor + 1.0;
        match child {
            FTremChild::Note(note) => {
                let duration_beats = mei_note_duration_beats(note, quarter_ppq);
                let beat = note
                    .note_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = note.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                beat_cursor += duration_beats;
            }
            FTremChild::Chord(chord) => {
                let duration_beats = mei_chord_duration_beats(chord, quarter_ppq);
                let beat = chord
                    .chord_log
                    .tstamp
                    .as_ref()
                    .map(|tstamp| tstamp.0)
                    .unwrap_or(child_start);
                if let Some(id) = chord.common.xml_id.as_ref() {
                    positions.insert(
                        id.clone(),
                        MeiEventPosition {
                            staff,
                            measure_idx,
                            beat,
                            duration_beats,
                        },
                    );
                }
                for chord_child in &chord.children {
                    let tusk_model::elements::ChordChild::Note(note) = chord_child else {
                        continue;
                    };
                    if let Some(id) = note.common.xml_id.as_ref() {
                        positions.insert(
                            id.clone(),
                            MeiEventPosition {
                                staff,
                                measure_idx,
                                beat,
                                duration_beats,
                            },
                        );
                    }
                }
                beat_cursor += duration_beats;
            }
            _ => {}
        }
    }

    beat_cursor
}

fn estimate_layer_quarter_ppq(children: &[LayerChild]) -> Option<f64> {
    children.iter().find_map(|child| estimate_layer_child_quarter_ppq(child))
}

fn estimate_layer_child_quarter_ppq(child: &LayerChild) -> Option<f64> {
    match child {
        LayerChild::Note(note) => {
            estimate_quarter_ppq(parse_ppq(note.note_ges.dur_ppq.as_deref()), duration_quarters(note.note_log.dur.as_ref(), note.note_log.dots.as_ref().map(|dots| dots.0)))
        }
        LayerChild::Rest(rest) => {
            estimate_quarter_ppq(parse_ppq(rest.rest_ges.dur_ppq.as_deref()), rest_duration_quarters(rest))
        }
        LayerChild::Chord(chord) => {
            estimate_quarter_ppq(parse_ppq(chord.chord_ges.dur_ppq.as_deref()), duration_quarters(chord.chord_log.dur.as_ref(), chord.chord_log.dots.as_ref().map(|dots| dots.0)))
        }
        LayerChild::Beam(beam) => beam.children.iter().find_map(estimate_beam_child_quarter_ppq),
        LayerChild::Tuplet(tuplet) => tuplet.children.iter().find_map(estimate_tuplet_child_quarter_ppq),
        LayerChild::BTrem(btrem) => btrem.children.iter().find_map(estimate_btrem_child_quarter_ppq),
        LayerChild::FTrem(ftrem) => ftrem.children.iter().find_map(estimate_ftrem_child_quarter_ppq),
        _ => None,
    }
}

fn estimate_beam_child_quarter_ppq(child: &tusk_model::elements::BeamChild) -> Option<f64> {
    use tusk_model::elements::BeamChild;

    match child {
        BeamChild::Note(note) => {
            estimate_quarter_ppq(parse_ppq(note.note_ges.dur_ppq.as_deref()), duration_quarters(note.note_log.dur.as_ref(), note.note_log.dots.as_ref().map(|dots| dots.0)))
        }
        BeamChild::Rest(rest) => {
            estimate_quarter_ppq(parse_ppq(rest.rest_ges.dur_ppq.as_deref()), rest_duration_quarters(rest))
        }
        BeamChild::Chord(chord) => {
            estimate_quarter_ppq(parse_ppq(chord.chord_ges.dur_ppq.as_deref()), duration_quarters(chord.chord_log.dur.as_ref(), chord.chord_log.dots.as_ref().map(|dots| dots.0)))
        }
        BeamChild::Beam(beam) => beam.children.iter().find_map(estimate_beam_child_quarter_ppq),
        BeamChild::BTrem(btrem) => btrem.children.iter().find_map(estimate_btrem_child_quarter_ppq),
        BeamChild::FTrem(ftrem) => ftrem.children.iter().find_map(estimate_ftrem_child_quarter_ppq),
        _ => None,
    }
}

fn estimate_tuplet_child_quarter_ppq(child: &tusk_model::elements::TupletChild) -> Option<f64> {
    use tusk_model::elements::TupletChild;

    match child {
        TupletChild::Note(note) => {
            estimate_quarter_ppq(parse_ppq(note.note_ges.dur_ppq.as_deref()), duration_quarters(note.note_log.dur.as_ref(), note.note_log.dots.as_ref().map(|dots| dots.0)))
        }
        TupletChild::Rest(rest) => {
            estimate_quarter_ppq(parse_ppq(rest.rest_ges.dur_ppq.as_deref()), rest_duration_quarters(rest))
        }
        TupletChild::Beam(beam) => beam.children.iter().find_map(estimate_beam_child_quarter_ppq),
        TupletChild::BTrem(btrem) => btrem.children.iter().find_map(estimate_btrem_child_quarter_ppq),
    }
}

fn estimate_btrem_child_quarter_ppq(child: &tusk_model::elements::BTremChild) -> Option<f64> {
    use tusk_model::elements::BTremChild;

    match child {
        BTremChild::Note(note) => {
            estimate_quarter_ppq(parse_ppq(note.note_ges.dur_ppq.as_deref()), duration_quarters(note.note_log.dur.as_ref(), note.note_log.dots.as_ref().map(|dots| dots.0)))
        }
        BTremChild::Chord(chord) => {
            estimate_quarter_ppq(parse_ppq(chord.chord_ges.dur_ppq.as_deref()), duration_quarters(chord.chord_log.dur.as_ref(), chord.chord_log.dots.as_ref().map(|dots| dots.0)))
        }
    }
}

fn estimate_ftrem_child_quarter_ppq(child: &tusk_model::elements::FTremChild) -> Option<f64> {
    use tusk_model::elements::FTremChild;

    match child {
        FTremChild::Note(note) => {
            estimate_quarter_ppq(parse_ppq(note.note_ges.dur_ppq.as_deref()), duration_quarters(note.note_log.dur.as_ref(), note.note_log.dots.as_ref().map(|dots| dots.0)))
        }
        FTremChild::Chord(chord) => {
            estimate_quarter_ppq(parse_ppq(chord.chord_ges.dur_ppq.as_deref()), duration_quarters(chord.chord_log.dur.as_ref(), chord.chord_log.dots.as_ref().map(|dots| dots.0)))
        }
        _ => None,
    }
}

fn estimate_quarter_ppq(dur_ppq: f64, duration_quarters: Option<f64>) -> Option<f64> {
    let quarters = duration_quarters?;
    if dur_ppq > 0.0 && quarters > 0.0 {
        Some(dur_ppq / quarters)
    } else {
        None
    }
}

fn mei_note_duration_beats(note: &tusk_model::elements::Note, quarter_ppq: Option<f64>) -> f64 {
    if note.note_log.grace.is_some() {
        return 0.0;
    }
    if let Some(quarter_ppq) = quarter_ppq {
        let dur_ppq = parse_ppq(note.note_ges.dur_ppq.as_deref());
        if dur_ppq > 0.0 {
            return dur_ppq / quarter_ppq;
        }
    }
    note.note_log
        .dur
        .as_ref()
        .map(super::utils::duration_to_quarter_notes)
        .map(|base| {
            super::utils::apply_dots(base, note.note_log.dots.as_ref().map(|dots| dots.0).unwrap_or(0))
        })
        .unwrap_or(0.0)
}

fn mei_chord_duration_beats(chord: &tusk_model::elements::Chord, quarter_ppq: Option<f64>) -> f64 {
    if let Some(quarter_ppq) = quarter_ppq {
        let dur_ppq = parse_ppq(chord.chord_ges.dur_ppq.as_deref());
        if dur_ppq > 0.0 {
            return dur_ppq / quarter_ppq;
        }
    }
    chord
        .chord_log
        .dur
        .as_ref()
        .map(super::utils::duration_to_quarter_notes)
        .map(|base| {
            super::utils::apply_dots(base, chord.chord_log.dots.as_ref().map(|dots| dots.0).unwrap_or(0))
        })
        .or_else(|| {
            chord.children.iter().find_map(|child| {
                let tusk_model::elements::ChordChild::Note(note) = child else {
                    return None;
                };
                Some(mei_note_duration_beats(note, quarter_ppq))
            })
        })
        .unwrap_or(0.0)
}

fn mei_rest_duration_beats(rest: &tusk_model::elements::Rest, quarter_ppq: Option<f64>) -> f64 {
    if let Some(quarter_ppq) = quarter_ppq {
        let dur_ppq = parse_ppq(rest.rest_ges.dur_ppq.as_deref());
        if dur_ppq > 0.0 {
            return dur_ppq / quarter_ppq;
        }
    }
    rest.rest_log
        .dur
        .as_ref()
        .map(super::utils::duration_rests_to_quarter_notes)
        .map(|base| {
            super::utils::apply_dots(base, rest.rest_log.dots.as_ref().map(|dots| dots.0).unwrap_or(0))
        })
        .unwrap_or(0.0)
}

fn mei_space_duration_beats(space: &tusk_model::elements::Space, quarter_ppq: Option<f64>) -> f64 {
    if let Some(quarter_ppq) = quarter_ppq {
        let dur_ppq = parse_ppq(space.space_ges.dur_ppq.as_deref());
        if dur_ppq > 0.0 {
            return dur_ppq / quarter_ppq;
        }
    }
    space
        .space_log
        .dur
        .as_ref()
        .map(super::utils::duration_to_quarter_notes)
        .map(|base| {
            super::utils::apply_dots(base, space.space_log.dots.as_ref().map(|dots| dots.0).unwrap_or(0))
        })
        .unwrap_or(0.0)
}

fn parse_ppq(ppq: Option<&str>) -> f64 {
    ppq.and_then(|value| value.parse::<f64>().ok()).unwrap_or(0.0)
}

fn rest_duration_quarters(rest: &tusk_model::elements::Rest) -> Option<f64> {
    use tusk_model::data::DataDurationrests;

    let base = match rest.rest_log.dur.as_ref()? {
        DataDurationrests::MeiDataDurationCmn(dur) => data_duration_cmn_to_quarters(dur),
        DataDurationrests::MeiDataDurationrestsMensural(_) => return None,
    };
    Some(super::utils::apply_dots(
        base,
        rest.rest_log.dots.as_ref().map(|dots| dots.0).unwrap_or(0),
    ))
}

fn duration_quarters(
    dur: Option<&tusk_model::data::DataDuration>,
    dots: Option<u64>,
) -> Option<f64> {
    use tusk_model::data::DataDuration;

    let base = match dur? {
        DataDuration::MeiDataDurationCmn(dur) => data_duration_cmn_to_quarters(dur),
        _ => return None,
    };
    Some(super::utils::apply_dots(base, dots.unwrap_or(0)))
}

fn data_duration_cmn_to_quarters(dur: &tusk_model::data::DataDurationCmn) -> f64 {
    use tusk_model::data::DataDurationCmn;

    match dur {
        DataDurationCmn::Long => 16.0,
        DataDurationCmn::Breve => 8.0,
        DataDurationCmn::N1 => 4.0,
        DataDurationCmn::N2 => 2.0,
        DataDurationCmn::N4 => 1.0,
        DataDurationCmn::N8 => 0.5,
        DataDurationCmn::N16 => 0.25,
        DataDurationCmn::N32 => 0.125,
        DataDurationCmn::N64 => 0.0625,
        DataDurationCmn::N128 => 0.03125,
        DataDurationCmn::N256 => 0.015625,
        DataDurationCmn::N512 => 0.0078125,
        DataDurationCmn::N1024 => 0.00390625,
        DataDurationCmn::N2048 => 0.001953125,
    }
}

fn transpose_musicxml_note_pitch(note: &mut crate::model::note::Note, octave_delta: i16) {
    let crate::model::note::FullNoteContent::Pitch(pitch) = &mut note.content else {
        return;
    };
    pitch.octave = ((pitch.octave as i16) + octave_delta).max(0) as u8;
}

fn parse_measurebeat(value: &str) -> (usize, f64) {
    let Some((measures, beat)) = value.split_once("m+") else {
        return (0, value.parse().unwrap_or(1.0));
    };
    (
        measures.parse::<usize>().unwrap_or(0),
        beat.parse::<f64>().unwrap_or(1.0),
    )
}

fn span_contains_position(span: &ExportOctaveSpan, measure_idx: usize, beat: f64) -> bool {
    span_contains_position_with_end(span, measure_idx, beat, span.end_beat)
}

fn span_contains_position_with_end(
    span: &ExportOctaveSpan,
    measure_idx: usize,
    beat: f64,
    end_beat: f64,
) -> bool {
    const EPSILON: f64 = 0.000_001;

    if measure_idx < span.start_measure || measure_idx > span.end_measure {
        return false;
    }
    if measure_idx == span.start_measure && beat + EPSILON < span.start_beat {
        return false;
    }
    if measure_idx == span.end_measure && beat + EPSILON >= end_beat {
        return false;
    }
    true
}

fn apply_octave_directions_to_parts(
    mei_measures: &[&tusk_model::elements::Measure],
    note_positions: &HashMap<String, MeiEventPosition>,
    part_ids: &[String],
    part_prev_measures: &mut [Vec<MxmlMeasure>],
    ctx: &ConversionContext,
) {
    let mut staff_numbers: HashMap<usize, u8> = HashMap::new();

    for (measure_idx, measure) in mei_measures.iter().enumerate() {
        for child in &measure.children {
            let MeasureChild::Octave(octave) = child else {
                continue;
            };

            let staff = octave
                .octave_log
                .staff
                .as_deref()
                .and_then(|value| value.split_whitespace().next())
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(1);
            let Some((part_idx, local_staff)) = resolve_part_and_local_staff(staff, part_ids, ctx) else {
                continue;
            };
            let number = {
                let next = staff_numbers.entry(staff).or_insert(1);
                let current = *next;
                *next = next.saturating_add(1);
                current
            };
            let size = octave.octave_log.dis.as_ref().map(|dis| dis.0 as u8).unwrap_or(8);

            let start_position = if let Some(beat) = octave.octave_log.tstamp.as_ref().map(|beat| beat.0) {
                Some((measure_idx, beat))
            } else {
                octave
                    .octave_log
                    .startid
                    .as_ref()
                    .and_then(|id| note_positions.get(strip_uri_fragment(&id.0)))
                    .map(|pos| (pos.measure_idx, pos.beat))
            };
            if let Some((start_measure_idx, start_beat)) = start_position {
                insert_direction_near_start(
                    &mut part_prev_measures[part_idx][start_measure_idx].content,
                    make_octave_direction(
                        local_staff,
                        start_beat,
                        octave_start_type(octave),
                        number,
                        size,
                        octave.common.xml_id.clone(),
                        ctx,
                    ),
                );
            }

            let stop_position = if let Some(endid) = octave.octave_log.endid.as_ref() {
                note_positions
                    .get(strip_uri_fragment(&endid.0))
                    .map(|pos| (pos.measure_idx, pos.beat + pos.duration_beats))
            } else if let Some(tstamp2) = octave.octave_log.tstamp2.as_ref() {
                let (measures_ahead, beat) = parse_measurebeat(&tstamp2.0);
                Some((measure_idx + measures_ahead, beat))
            } else {
                None
            };
            if let Some((stop_measure_idx, stop_beat)) = stop_position {
                insert_direction_near_start(
                    &mut part_prev_measures[part_idx][stop_measure_idx].content,
                    make_octave_direction(
                        local_staff,
                        stop_beat,
                        crate::model::direction::OctaveShiftType::Stop,
                        number,
                        size,
                        None,
                        ctx,
                    ),
                );
            }
        }
    }
}

fn resolve_part_and_local_staff(
    global_staff: usize,
    part_ids: &[String],
    ctx: &ConversionContext,
) -> Option<(usize, usize)> {
    for (part_idx, part_id) in part_ids.iter().enumerate() {
        for local_staff in 1..=ctx.staves_for_part(part_id) as usize {
            if ctx.global_staff_for_part(part_id, local_staff as u32)? as usize == global_staff {
                return Some((part_idx, local_staff));
            }
        }
    }
    None
}

fn octave_start_type(
    octave: &tusk_model::elements::Octave,
) -> crate::model::direction::OctaveShiftType {
    match octave.octave_log.dis_place {
        Some(tusk_model::data::DataStaffrelBasic::Above) => crate::model::direction::OctaveShiftType::Down,
        Some(tusk_model::data::DataStaffrelBasic::Below) => crate::model::direction::OctaveShiftType::Up,
        None => crate::model::direction::OctaveShiftType::Up,
    }
}

fn make_octave_direction(
    local_staff: usize,
    beat: f64,
    shift_type: crate::model::direction::OctaveShiftType,
    number: u8,
    size: u8,
    id: Option<String>,
    ctx: &ConversionContext,
) -> crate::model::elements::MeasureContent {
    use crate::model::direction::{Direction, DirectionType, DirectionTypeContent, OctaveShift};

    let mut shift = OctaveShift::new(shift_type);
    shift.number = Some(number);
    shift.size = Some(size);
    shift.id = id;

    let direction_type = DirectionType {
        content: DirectionTypeContent::OctaveShift(shift),
        id: None,
    };
    let mut direction = Direction::new(vec![direction_type]);
    direction.staff = Some(local_staff as u32);

    let beat_position = beat - 1.0;
    let offset = beat_position * ctx.divisions();
    if offset.abs() >= 0.001 {
        direction.offset = Some(crate::model::direction::Offset::new(offset));
    }

    MeasureContent::Direction(Box::new(direction))
}

fn insert_direction_near_start(
    content: &mut Vec<MeasureContent>,
    direction: MeasureContent,
) {
    let new_offset = direction_content_offset(&direction);
    let new_is_stop = direction_is_octave_stop(&direction);

    let mut index = 0;
    while index < content.len() {
        match &content[index] {
            MeasureContent::Barline(_) | MeasureContent::Attributes(_) | MeasureContent::Print(_) => {
                index += 1;
            }
            MeasureContent::Direction(existing) => {
                let existing_offset = existing.offset.as_ref().map(|offset| offset.value).unwrap_or(0.0);
                if existing_offset > new_offset
                    || ((existing_offset - new_offset).abs() < 0.001
                        && direction_precedes_existing(new_is_stop, &content[index]))
                {
                    break;
                }
                index += 1;
            }
            _ => break,
        }
    }
    content.insert(index, direction);
}

fn direction_content_offset(content: &MeasureContent) -> f64 {
    match content {
        MeasureContent::Direction(direction) => {
            direction.offset.as_ref().map(|offset| offset.value).unwrap_or(0.0)
        }
        _ => 0.0,
    }
}

fn direction_is_octave_stop(content: &MeasureContent) -> bool {
    let MeasureContent::Direction(direction) = content else {
        return false;
    };
    direction.direction_types.iter().any(|direction_type| {
        matches!(
            direction_type.content,
            crate::model::direction::DirectionTypeContent::OctaveShift(
                crate::model::direction::OctaveShift {
                    shift_type: crate::model::direction::OctaveShiftType::Stop,
                    ..
                }
            )
        )
    })
}

fn direction_precedes_existing(new_is_octave_stop: bool, existing: &MeasureContent) -> bool {
    if new_is_octave_stop {
        return false;
    }
    direction_is_octave_stop(existing)
}

fn strip_uri_fragment(uri: &str) -> &str {
    uri.strip_prefix('#').unwrap_or(uri)
}

/// Convert MEI direction events (dynam, hairpin, dir, tempo) to MusicXML directions.
///
/// Direction events in MEI are children of `<measure>`, not `<staff>`. Each event
/// has a `@staff` attribute indicating which staff/part it belongs to.
/// Events without `@staff` default to staff 1.
///
/// Must be called BEFORE note conversion so that on reimport, beat_position=0
/// and offset-based tstamp reconstruction is correct.
fn convert_direction_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    local_staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    // Resolve any deferred hairpin stops from previous measures
    super::direction::resolve_deferred_hairpin_stops(staff_n, mxml_measure, ctx);

    for child in &mei_measure.children {
        match child {
            MeasureChild::Dynam(dynam) => {
                // Skip notation-level dynamics — handled by convert_notation_dynamics()
                if let Some(id) = dynam.common.xml_id.as_deref() {
                    if matches!(
                        ctx.ext_store().ornament_detail(id),
                        Some(tusk_model::musicxml_ext::OrnamentDetailData::NotationDynamics)
                    ) {
                        continue;
                    }
                }
                let event_staff = dynam
                    .dynam_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if event_staff == staff_n
                    && let Some(mut direction) = convert_mei_dynam(dynam, ctx)
                {
                    direction.staff = Some(local_staff_n as u32);
                    mxml_measure
                        .content
                        .push(MeasureContent::Direction(Box::new(direction)));
                }
            }
            MeasureChild::Hairpin(hairpin) => {
                let event_staff = hairpin
                    .hairpin_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if event_staff == staff_n {
                    for mut direction in convert_mei_hairpin(hairpin, ctx) {
                        direction.staff = Some(local_staff_n as u32);
                        mxml_measure
                            .content
                            .push(MeasureContent::Direction(Box::new(direction)));
                    }
                }
            }
            MeasureChild::Dir(dir) => {
                // Standalone sound elements — emit on first staff only
                if dir
                    .common
                    .xml_id
                    .as_ref()
                    .is_some_and(|id| ctx.ext_store().sound(id).is_some())
                {
                    if local_staff_n == 1 {
                        if let Some(content) = super::sound::convert_mei_sound_dir(dir, ctx) {
                            mxml_measure.content.push(content);
                        }
                    }
                    continue;
                }

                // Measure-style elements — emit on first staff only
                if dir
                    .common
                    .xml_id
                    .as_ref()
                    .is_some_and(|id| ctx.ext_store().measure_style(id).is_some())
                {
                    if local_staff_n == 1 {
                        if let Some(content) =
                            super::measure_style::convert_mei_measure_style_dir(dir, ctx)
                        {
                            mxml_measure.content.push(content);
                        }
                    }
                    continue;
                }

                // Barline children — skip here, handled after basic barlines are added
                if dir
                    .common
                    .xml_id
                    .as_ref()
                    .is_some_and(|id| ctx.ext_store().barline(id).is_some())
                {
                    continue;
                }

                // Listening/grouping/link/bookmark — emit on first staff only
                if dir
                    .common
                    .xml_id
                    .as_ref()
                    .is_some_and(|id| ctx.ext_store().listening(id).is_some())
                {
                    if local_staff_n == 1 {
                        if let Some(c) = super::listening::convert_mei_listening_dir(dir, ctx) {
                            mxml_measure.content.push(c);
                        }
                    }
                    continue;
                }

                let event_staff = dir
                    .dir_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if event_staff == staff_n
                    && let Some(mut direction) = convert_mei_dir(dir, ctx)
                {
                    direction.staff = Some(local_staff_n as u32);
                    mxml_measure
                        .content
                        .push(MeasureContent::Direction(Box::new(direction)));
                }
            }
            MeasureChild::Tempo(tempo) => {
                let event_staff = tempo
                    .tempo_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if event_staff == staff_n
                    && let Some(mut direction) = convert_mei_tempo(tempo, ctx)
                {
                    direction.staff = Some(local_staff_n as u32);
                    mxml_measure
                        .content
                        .push(MeasureContent::Direction(Box::new(direction)));
                }
            }
            MeasureChild::Pedal(pedal) => {
                let event_staff = pedal
                    .pedal_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if event_staff == staff_n
                    && let Some(mut direction) = convert_mei_pedal(pedal, ctx)
                {
                    direction.staff = Some(local_staff_n as u32);
                    mxml_measure
                        .content
                        .push(MeasureContent::Direction(Box::new(direction)));
                }
            }
            MeasureChild::Harm(harm) => {
                let event_staff = harm
                    .harm_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if event_staff == staff_n {
                    if let Some(content) =
                        super::harmony::convert_mei_harm(harm, local_staff_n, ctx)
                    {
                        mxml_measure.content.push(content);
                    }
                }
            }
            MeasureChild::Fb(fb) => {
                // Figured bass has no @staff attribute — emit on first staff only
                if local_staff_n == 1 {
                    if let Some(content) =
                        super::figured_bass::convert_mei_fb(fb, local_staff_n, ctx)
                    {
                        mxml_measure.content.push(content);
                    }
                }
            }
            MeasureChild::Sb(sb) => {
                // System break — emit on first staff only
                if local_staff_n == 1 {
                    if let Some(content) = super::print::convert_mei_sb(sb, ctx) {
                        mxml_measure.content.push(content);
                    }
                }
            }
            MeasureChild::Pb(pb) => {
                // Page break — emit on first staff only
                if local_staff_n == 1 {
                    if let Some(content) = super::print::convert_mei_pb(pb, ctx) {
                        mxml_measure.content.push(content);
                    }
                }
            }
            // These MeasureChild variants are handled by other conversion functions:
            // - Staff/Layer: structural, processed by convert_staff_content
            // - Slur/Gliss/Arpeg/Fermata: notation events, processed by convert_slur_events etc.
            // - Trill/Mordent/Turn/Ornam: ornament events, processed by convert_ornament_events
            // - TupletSpan: processed by convert_tuplet_events
            // - Fing: processed by convert_technical_events
            // - MNum: measure number display, no MusicXML equivalent
            // - Breath/Caesura: native MEI elements, processed by convert_breath_caesura_events
            MeasureChild::Staff(_)
            | MeasureChild::Layer(_)
            | MeasureChild::Slur(_)
            | MeasureChild::Gliss(_)
            | MeasureChild::Arpeg(_)
            | MeasureChild::Fermata(_)
            | MeasureChild::Trill(_)
            | MeasureChild::Mordent(_)
            | MeasureChild::Turn(_)
            | MeasureChild::Ornam(_)
            | MeasureChild::TupletSpan(_)
            | MeasureChild::Fing(_)
            | MeasureChild::MNum(_)
            | MeasureChild::Breath(_)
            | MeasureChild::Caesura(_) => {}
            _ => {}
        }
    }
    Ok(())
}

/// Convert MEI slur events to MusicXML notations on the referenced notes.
///
/// Must be called AFTER note conversion so that referenced notes exist in the measure.
/// Slurs are matched by startid/endid note references, not by @staff attribute,
/// because MEI @staff on control events indicates display staff, not the staff
/// containing the referenced notes.
///
/// For cross-measure slurs (endid references a note in a future measure),
/// the stop notation is deferred to the context and applied when the target
/// measure is processed.
fn convert_slur_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    prev_measures: &mut [MxmlMeasure],
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    // First, resolve any deferred slur stops from previous measures
    resolve_deferred_slur_stops(staff_n, mxml_measure, ctx);

    for child in &mei_measure.children {
        if let MeasureChild::Slur(slur) = child {
            // Only process slurs belonging to this staff; MEI @staff is Option<String> (space-separated list)
            let slur_staff = slur
                .slur_log
                .staff
                .as_ref()
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .unwrap_or(0) as usize;
            if slur_staff == staff_n {
                convert_mei_slur_to_notations(slur, staff_n, mxml_measure, prev_measures, ctx);
            }
        }
    }
    Ok(())
}

/// Resolve deferred slur stops from cross-measure slurs in previous measures.
fn resolve_deferred_slur_stops(
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) {
    use crate::model::data::StartStopContinue;
    use crate::model::notations::{Notations, Slur as MxmlSlur};

    let deferred = ctx.drain_deferred_slur_stops();
    for stop in deferred {
        if stop.staff != staff_n {
            // Not for this staff — re-defer
            ctx.add_deferred_slur_stop(stop);
            continue;
        }
        if let Some(note) = find_note_by_id_mut(mxml_measure, &stop.end_id) {
            let mut mxml_slur = MxmlSlur::new(StartStopContinue::Stop);
            mxml_slur.number = Some(stop.number);
            let notations = note.notations.get_or_insert_with(Notations::default);
            notations.slurs.push(mxml_slur);
        } else {
            // Still not found — re-defer to next measure
            ctx.add_deferred_slur_stop(stop);
        }
    }
}

/// Convert an MEI slur control event to MusicXML slur notations on the referenced notes.
///
/// MEI slurs use `@startid`/`@endid` to reference the notes they connect.
/// MusicXML slurs are `<notations><slur>` elements on individual notes.
/// For cross-measure slurs, the stop is deferred via the context.
fn convert_mei_slur_to_notations(
    slur: &tusk_model::elements::Slur,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    prev_measures: &mut [MxmlMeasure],
    ctx: &mut ConversionContext,
) {
    use crate::model::data::StartStopContinue;
    use crate::model::notations::{Notations, Slur as MxmlSlur};

    // Extract start and end note IDs (strip leading '#' from URI references)
    let start_id = slur
        .slur_log
        .startid
        .as_ref()
        .map(|uri| uri.to_string().trim_start_matches('#').to_string());
    let end_id = slur
        .slur_log
        .endid
        .as_ref()
        .map(|uri| uri.to_string().trim_start_matches('#').to_string());

    // Look up pre-assigned slur number (computed by interval graph coloring)
    let number = match (&start_id, &end_id) {
        (Some(sid), Some(eid)) => ctx.get_slur_number(sid, eid).unwrap_or(1),
        _ => 1,
    };

    // Add slur start notation to the start note
    if let Some(ref sid) = start_id {
        if let Some(note) = find_note_by_id_mut(mxml_measure, sid) {
            let mut mxml_slur = MxmlSlur::new(StartStopContinue::Start);
            mxml_slur.number = Some(number);
            let notations = note.notations.get_or_insert_with(Notations::default);
            notations.slurs.push(mxml_slur);
        } else {
            // Start note in a previous measure (cross-measure slur)
            for prev in prev_measures.iter_mut().rev() {
                if let Some(note) = find_note_by_id_mut(prev, sid) {
                    let mut mxml_slur = MxmlSlur::new(StartStopContinue::Start);
                    mxml_slur.number = Some(number);
                    let notations = note.notations.get_or_insert_with(Notations::default);
                    notations.slurs.push(mxml_slur);
                    break;
                }
            }
        }
    }

    // Add slur stop notation to the end note (may be in this or a future measure)
    if let Some(ref eid) = end_id {
        if let Some(note) = find_note_by_id_mut(mxml_measure, eid) {
            let mut mxml_slur = MxmlSlur::new(StartStopContinue::Stop);
            mxml_slur.number = Some(number);
            let notations = note.notations.get_or_insert_with(Notations::default);
            notations.slurs.push(mxml_slur);
        } else {
            // End note not in this measure — defer to future measure
            ctx.add_deferred_slur_stop(crate::context::DeferredSlurStop {
                end_id: eid.clone(),
                number,
                staff: staff_n,
            });
        }
    }
}

/// Convert MEI tupletSpan events to MusicXML time-modification and tuplet notations.
///
/// For each tupletSpan control event on this staff:
/// 1. Add time-modification (actual-notes/normal-notes) to all notes between startid and endid
/// 2. Add tuplet start notation to the start note
/// 3. Add tuplet stop notation to the end note
fn convert_tuplet_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    _ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::data::YesNo;
    use crate::model::notations::{Notations, ShowTuplet, Tuplet as MxmlTuplet};
    use crate::model::note::TimeModification;

    for child in &mei_measure.children {
        if let MeasureChild::TupletSpan(ts) = child {
            let ts_staff = ts
                .tuplet_span_log
                .staff
                .as_ref()
                .and_then(|s| s.split_whitespace().next())
                .and_then(|s| s.parse().ok())
                .unwrap_or(1) as usize;
            if ts_staff != staff_n {
                continue;
            }

            let start_id = ts
                .tuplet_span_log
                .startid
                .as_ref()
                .map(|uri| uri.to_string().trim_start_matches('#').to_string());
            let end_id = ts
                .tuplet_span_log
                .endid
                .as_ref()
                .map(|uri| uri.to_string().trim_start_matches('#').to_string());

            let num: u32 = ts
                .tuplet_span_log
                .num
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3);
            let numbase: u32 = ts
                .tuplet_span_log
                .numbase
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2);

            // Decode visual attributes
            let bracket = ts
                .tuplet_span_vis
                .bracket_visible
                .as_ref()
                .map(|b| matches!(b, tusk_model::data::DataBoolean::True));
            let show_number = if let Some(ref nv) = ts.tuplet_span_vis.num_visible {
                match nv {
                    tusk_model::data::DataBoolean::False => Some(ShowTuplet::None),
                    tusk_model::data::DataBoolean::True => {
                        if ts.tuplet_span_vis.num_format.as_deref() == Some("ratio") {
                            Some(ShowTuplet::Both)
                        } else {
                            Some(ShowTuplet::Actual)
                        }
                    }
                }
            } else {
                None
            };
            let placement = ts.tuplet_span_vis.num_place.as_ref().map(|p| match p {
                tusk_model::data::DataStaffrelBasic::Above => crate::model::data::AboveBelow::Above,
                tusk_model::data::DataStaffrelBasic::Below => crate::model::data::AboveBelow::Below,
            });

            // Find note indices between startid and endid (inclusive)
            let (Some(sid), Some(eid)) = (start_id, end_id) else {
                continue;
            };

            // Resolve chord IDs to first-note IDs: tupletSpan may reference a chord's
            // xml:id, but MusicXML notes carry individual note IDs, not the chord ID.
            let resolved_sid = resolve_chord_to_first_note_id(mei_measure, &sid).unwrap_or(sid);
            let resolved_eid = resolve_chord_to_first_note_id(mei_measure, &eid).unwrap_or(eid);

            // Find start and end positions in measure content
            let start_pos = mxml_measure.content.iter().position(
                |c| matches!(c, MeasureContent::Note(n) if n.id.as_deref() == Some(&resolved_sid)),
            );
            let end_pos = mxml_measure.content.iter().position(
                |c| matches!(c, MeasureContent::Note(n) if n.id.as_deref() == Some(&resolved_eid)),
            );

            let (Some(start_pos), Some(end_pos)) = (start_pos, end_pos) else {
                continue;
            };

            // Add time-modification to all notes in range
            let time_mod = TimeModification::new(num, numbase);
            for i in start_pos..=end_pos {
                if let MeasureContent::Note(ref mut note) = mxml_measure.content[i] {
                    note.time_modification = Some(time_mod.clone());
                }
            }

            // Add tuplet start notation to start note
            if let MeasureContent::Note(ref mut note) = mxml_measure.content[start_pos] {
                let mut t = MxmlTuplet::start();
                t.number = Some(1);
                if let Some(b) = bracket {
                    t.bracket = Some(if b { YesNo::Yes } else { YesNo::No });
                }
                t.show_number = show_number;
                t.placement = placement;
                let notations = note.notations.get_or_insert_with(Notations::default);
                notations.tuplets.push(t);
            }

            // Add tuplet stop notation to end note
            if let MeasureContent::Note(ref mut note) = mxml_measure.content[end_pos] {
                let mut t = MxmlTuplet::stop();
                t.number = Some(1);
                let notations = note.notations.get_or_insert_with(Notations::default);
                notations.tuplets.push(t);
            }
        }
    }
    Ok(())
}

/// Resolve a chord xml:id to its first child note's xml:id.
/// TupletSpan uses chord IDs, but MusicXML notes carry individual note IDs.
fn resolve_chord_to_first_note_id(
    mei_measure: &tusk_model::elements::Measure,
    id: &str,
) -> Option<String> {
    for mc in &mei_measure.children {
        if let MeasureChild::Staff(staff) = mc {
            for sc in &staff.children {
                if let tusk_model::elements::StaffChild::Layer(layer) = sc {
                    for lc in &layer.children {
                        if let tusk_model::elements::LayerChild::Chord(chord) = lc {
                            if chord.common.xml_id.as_deref() == Some(id) {
                                return chord.children.iter().find_map(|cc| {
                                    if let tusk_model::elements::ChordChild::Note(n) = cc {
                                        n.common.xml_id.clone()
                                    } else {
                                        None
                                    }
                                });
                            }
                        }
                        // Also check inside beams
                        if let tusk_model::elements::LayerChild::Beam(beam) = lc {
                            for bc in &beam.children {
                                if let tusk_model::elements::BeamChild::Chord(chord) = bc {
                                    if chord.common.xml_id.as_deref() == Some(id) {
                                        return chord.children.iter().find_map(|cc| {
                                            if let tusk_model::elements::ChordChild::Note(n) = cc {
                                                n.common.xml_id.clone()
                                            } else {
                                                None
                                            }
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

/// Convert MEI ornament control events to MusicXML ornament notations on notes.
///
/// For each trill/mordent/turn/ornam control event, finds the referenced note
/// by startid and adds the appropriate ornament to its notations.
fn convert_ornament_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use super::direction::convert_place_to_placement;
    use crate::model::data::{StartStopContinue, TremoloType, YesNo};
    use crate::model::notations::{
        EmptyPlacement, EmptyTrillSound, HorizontalTurn, Mordent as MxmlMordent, Notations,
        Ornaments, OtherOrnament, Tremolo, WavyLine,
    };
    use tusk_model::musicxml_ext::OrnamentDetailData;

    for child in &mei_measure.children {
        match child {
            MeasureChild::Trill(trill) => {
                let trill_staff = trill
                    .trill_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if trill_staff != staff_n {
                    continue;
                }
                let start_id = trill
                    .trill_log
                    .startid
                    .as_ref()
                    .map(|uri| uri.to_string().trim_start_matches('#').to_string());
                let Some(sid) = start_id else { continue };
                let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
                    continue;
                };
                let notations = note.notations.get_or_insert_with(Notations::default);
                let ornaments = notations.ornaments.get_or_insert_with(Ornaments::default);
                ornaments.trill_mark = Some(EmptyTrillSound {
                    placement: convert_place_to_placement(&trill.trill_vis.place),
                    ..Default::default()
                });
            }
            MeasureChild::Mordent(mordent) => {
                let mordent_staff = mordent
                    .mordent_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if mordent_staff != staff_n {
                    continue;
                }
                let start_id = mordent
                    .mordent_log
                    .startid
                    .as_ref()
                    .map(|uri| uri.to_string().trim_start_matches('#').to_string());
                let Some(sid) = start_id else { continue };
                let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
                    continue;
                };
                let notations = note.notations.get_or_insert_with(Notations::default);
                let ornaments = notations.ornaments.get_or_insert_with(Ornaments::default);
                let placement = convert_place_to_placement(&mordent.mordent_vis.place);
                let long = mordent.mordent_log.long.as_ref().and_then(|b| {
                    if matches!(b, tusk_model::data::DataBoolean::True) {
                        Some(YesNo::Yes)
                    } else {
                        None
                    }
                });
                let mxml_mordent = MxmlMordent {
                    placement,
                    long,
                    ..Default::default()
                };
                let form = mordent.mordent_log.form.as_deref().unwrap_or("lower");
                if form == "upper" {
                    ornaments.inverted_mordent = Some(mxml_mordent);
                } else {
                    ornaments.mordent = Some(mxml_mordent);
                }
            }
            MeasureChild::Turn(turn) => {
                let turn_staff = turn
                    .turn_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if turn_staff != staff_n {
                    continue;
                }
                let start_id = turn
                    .turn_log
                    .startid
                    .as_ref()
                    .map(|uri| uri.to_string().trim_start_matches('#').to_string());
                let Some(sid) = start_id else { continue };
                let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
                    continue;
                };
                let notations = note.notations.get_or_insert_with(Notations::default);
                let ornaments = notations.ornaments.get_or_insert_with(Ornaments::default);
                let placement = convert_place_to_placement(&turn.turn_vis.place);
                let form = turn.turn_log.form.as_deref().unwrap_or("upper");
                let delayed = matches!(
                    turn.turn_log.delayed,
                    Some(tusk_model::data::DataBoolean::True)
                );
                let ht = HorizontalTurn {
                    placement,
                    ..Default::default()
                };
                match (form, delayed) {
                    ("upper", false) => ornaments.turn = Some(ht),
                    ("upper", true) => ornaments.delayed_turn = Some(ht),
                    ("lower", false) => ornaments.inverted_turn = Some(ht),
                    ("lower", true) => ornaments.delayed_inverted_turn = Some(ht),
                    _ => ornaments.turn = Some(ht),
                }
            }
            MeasureChild::Ornam(ornam) => {
                let ornam_staff = ornam
                    .ornam_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if ornam_staff != staff_n {
                    continue;
                }
                let start_id = ornam
                    .ornam_log
                    .startid
                    .as_ref()
                    .map(|uri| uri.to_string().trim_start_matches('#').to_string());
                let Some(sid) = start_id else { continue };
                let ornam_id = ornam.common.xml_id.as_deref().unwrap_or("");
                let placement = convert_place_to_placement(&ornam.ornam_vis.place);

                let Some(detail) = ctx.ext_store().ornament_detail(ornam_id) else {
                    continue;
                };
                let detail = detail.clone();

                // AccidentalMark and OtherNotation go on notations, not ornaments
                match &detail {
                    OrnamentDetailData::AccidentalMark {
                        value,
                        placement: p,
                    } => {
                        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
                            continue;
                        };
                        let notations = note.notations.get_or_insert_with(Notations::default);
                        notations
                            .accidental_marks
                            .push(crate::model::notations::AccidentalMark {
                                value: value.clone(),
                                placement: placement.or_else(|| {
                                    p.as_deref().and_then(|s| match s {
                                        "above" => Some(crate::model::data::AboveBelow::Above),
                                        "below" => Some(crate::model::data::AboveBelow::Below),
                                        _ => None,
                                    })
                                }),
                            });
                        continue;
                    }
                    OrnamentDetailData::OtherNotation {
                        notation_type,
                        number,
                        smufl,
                        text,
                    } => {
                        use crate::model::data::StartStopSingle;
                        let nt = match notation_type.as_str() {
                            "start" => StartStopSingle::Start,
                            "stop" => StartStopSingle::Stop,
                            _ => StartStopSingle::Single,
                        };
                        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
                            continue;
                        };
                        let notations = note.notations.get_or_insert_with(Notations::default);
                        notations
                            .other_notations
                            .push(crate::model::notations::OtherNotation {
                                notation_type: nt,
                                number: *number,
                                placement,
                                smufl: smufl.clone(),
                                text: text.clone(),
                            });
                        continue;
                    }
                    _ => {}
                }

                let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
                    continue;
                };
                let notations = note.notations.get_or_insert_with(Notations::default);
                let ornaments = notations.ornaments.get_or_insert_with(Ornaments::default);

                match &detail {
                    OrnamentDetailData::VerticalTurn => {
                        ornaments.vertical_turn = Some(EmptyTrillSound {
                            placement,
                            ..Default::default()
                        });
                    }
                    OrnamentDetailData::InvertedVerticalTurn => {
                        ornaments.inverted_vertical_turn = Some(EmptyTrillSound {
                            placement,
                            ..Default::default()
                        });
                    }
                    OrnamentDetailData::Shake => {
                        ornaments.shake = Some(EmptyTrillSound {
                            placement,
                            ..Default::default()
                        });
                    }
                    OrnamentDetailData::Schleifer => {
                        ornaments.schleifer = Some(EmptyPlacement {
                            placement,
                            ..Default::default()
                        });
                    }
                    OrnamentDetailData::Haydn => {
                        ornaments.haydn = Some(EmptyTrillSound {
                            placement,
                            ..Default::default()
                        });
                    }
                    OrnamentDetailData::UnmeasuredTremolo {
                        tremolo_type,
                        value,
                    } => {
                        let ttype = match tremolo_type.as_str() {
                            "start" => TremoloType::Start,
                            "stop" => TremoloType::Stop,
                            "unmeasured" => TremoloType::Unmeasured,
                            _ => TremoloType::Single,
                        };
                        ornaments.tremolo = Some(Tremolo {
                            tremolo_type: ttype,
                            value: *value,
                            placement,
                            default_x: None,
                            default_y: None,
                            color: None,
                            smufl: None,
                        });
                    }
                    OrnamentDetailData::WavyLine {
                        wavy_line_type,
                        number,
                    } => {
                        let wtype = match wavy_line_type.as_str() {
                            "stop" => StartStopContinue::Stop,
                            "continue" => StartStopContinue::Continue,
                            _ => StartStopContinue::Start,
                        };
                        ornaments.wavy_line = Some(WavyLine {
                            wavy_line_type: wtype,
                            number: *number,
                            placement,
                            default_x: None,
                            default_y: None,
                            color: None,
                            smufl: None,
                            trill_sound: Default::default(),
                        });
                    }
                    OrnamentDetailData::OtherOrnament { .. } => {
                        // Collect text content from ornam children
                        let text: String = ornam
                            .children
                            .iter()
                            .map(|c| {
                                let tusk_model::elements::OrnamChild::Text(t) = c;
                                t.as_str()
                            })
                            .collect::<Vec<_>>()
                            .join("");
                        ornaments.other_ornament = Some(OtherOrnament {
                            value: text,
                            placement,
                        });
                    }
                    OrnamentDetailData::OrnamentAccidentalMark {
                        value,
                        placement: p,
                    } => {
                        let acc_placement = p.as_deref().and_then(|s| match s {
                            "above" => Some(crate::model::data::AboveBelow::Above),
                            "below" => Some(crate::model::data::AboveBelow::Below),
                            _ => None,
                        });
                        ornaments
                            .accidental_marks
                            .push(crate::model::notations::AccidentalMark {
                                value: value.clone(),
                                placement: acc_placement,
                            });
                    }
                    // AccidentalMark and OtherNotation handled above
                    _ => {}
                }
            }
            // Non-ornament MeasureChild variants — handled by other conversion functions.
            MeasureChild::Fing(_)
            | MeasureChild::Dynam(_)
            | MeasureChild::Dir(_)
            | MeasureChild::Harm(_)
            | MeasureChild::Pb(_)
            | MeasureChild::Slur(_)
            | MeasureChild::Arpeg(_)
            | MeasureChild::Fermata(_)
            | MeasureChild::Sb(_)
            | MeasureChild::Gliss(_)
            | MeasureChild::TupletSpan(_)
            | MeasureChild::MNum(_)
            | MeasureChild::Staff(_)
            | MeasureChild::Layer(_)
            | MeasureChild::Hairpin(_)
            | MeasureChild::Tempo(_)
            | MeasureChild::Fb(_)
            | MeasureChild::Breath(_)
            | MeasureChild::Caesura(_) => {}
            _ => {}
        }
    }
    Ok(())
}

/// Convert MEI `<fermata>` control events to MusicXML fermata notations on notes.
fn convert_fermata_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    _ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::data::UprightInverted;
    use crate::model::notations::{Fermata as MxmlFermata, FermataShape, Notations};

    for child in &mei_measure.children {
        let MeasureChild::Fermata(fermata) = child else {
            continue;
        };
        let fermata_staff = fermata
            .fermata_log
            .staff
            .as_ref()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1) as usize;
        if fermata_staff != staff_n {
            continue;
        }
        let start_id = fermata
            .fermata_log
            .startid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());
        let Some(sid) = start_id else { continue };
        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
            continue;
        };

        // Map MEI @shape → MusicXML shape
        let shape = fermata.fermata_vis.shape.as_deref().and_then(|s| match s {
            "angular" => Some(FermataShape::Angled),
            "square" => Some(FermataShape::Square),
            "double-angular" => Some(FermataShape::DoubleAngled),
            "double-square" => Some(FermataShape::DoubleSquare),
            "double-dot" => Some(FermataShape::DoubleDot),
            "half-curve" => Some(FermataShape::HalfCurve),
            "curlew" => Some(FermataShape::Curlew),
            _ => None,
        });

        // Map MEI @form → MusicXML type
        let fermata_type = fermata.fermata_vis.form.as_deref().and_then(|f| match f {
            "inv" => Some(UprightInverted::Inverted),
            _ => None,
        });
        // If no explicit form but place=below, still set inverted
        let fermata_type = fermata_type.or_else(|| {
            use tusk_model::data::{DataStaffrel, DataStaffrelBasic};
            match &fermata.fermata_vis.place {
                Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Below)) => {
                    Some(UprightInverted::Inverted)
                }
                Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above)) => {
                    Some(UprightInverted::Upright)
                }
                _ => None,
            }
        });

        let notations = note.notations.get_or_insert_with(Notations::default);
        notations.fermatas.push(MxmlFermata {
            shape,
            fermata_type,
            ..Default::default()
        });
    }
    Ok(())
}

/// Convert MEI `<breath>` control events to MusicXML breath-mark articulations.
fn convert_breath_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::notations::{Articulations, BreathMark, BreathMarkValue, Notations};

    for child in &mei_measure.children {
        let MeasureChild::Breath(breath) = child else {
            continue;
        };
        let breath_staff = breath
            .breath_log
            .staff
            .as_ref()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1) as usize;
        if breath_staff != staff_n {
            continue;
        }
        let start_id = breath
            .breath_log
            .startid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());
        let Some(sid) = start_id else { continue };
        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
            continue;
        };

        // Read breath-mark value from ExtensionStore
        let value = breath
            .common
            .xml_id
            .as_deref()
            .and_then(|id| ctx.ext_store().ornament_detail(id))
            .and_then(|d| match d {
                tusk_model::musicxml_ext::OrnamentDetailData::BreathMark { value } => {
                    value.as_deref().and_then(|v| match v {
                        "comma" => Some(BreathMarkValue::Comma),
                        "tick" => Some(BreathMarkValue::Tick),
                        "upbow" => Some(BreathMarkValue::Upbow),
                        "salzedo" => Some(BreathMarkValue::Salzedo),
                        _ => None,
                    })
                }
                _ => None,
            });

        // Map MEI @place → MusicXML placement
        let placement = mei_place_to_above_below(&breath.breath_vis.place);

        let notations = note.notations.get_or_insert_with(Notations::default);
        let artics = notations
            .articulations
            .get_or_insert_with(Articulations::default);
        artics.breath_mark = Some(BreathMark { value, placement });
    }
    Ok(())
}

/// Convert MEI `<caesura>` control events to MusicXML caesura articulations.
fn convert_caesura_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::notations::{Articulations, Caesura as MxmlCaesura, CaesuraValue, Notations};

    for child in &mei_measure.children {
        let MeasureChild::Caesura(caesura) = child else {
            continue;
        };
        let caesura_staff = caesura
            .caesura_log
            .staff
            .as_ref()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1) as usize;
        if caesura_staff != staff_n {
            continue;
        }
        let start_id = caesura
            .caesura_log
            .startid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());
        let Some(sid) = start_id else { continue };
        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
            continue;
        };

        // Read caesura value from ExtensionStore
        let value = caesura
            .common
            .xml_id
            .as_deref()
            .and_then(|id| ctx.ext_store().ornament_detail(id))
            .and_then(|d| match d {
                tusk_model::musicxml_ext::OrnamentDetailData::Caesura { value } => {
                    value.as_deref().and_then(|v| match v {
                        "normal" => Some(CaesuraValue::Normal),
                        "short" => Some(CaesuraValue::Short),
                        "thick" => Some(CaesuraValue::Thick),
                        "curved" => Some(CaesuraValue::Curved),
                        "single" => Some(CaesuraValue::Single),
                        _ => None,
                    })
                }
                _ => None,
            });

        // Map MEI @place → MusicXML placement
        let placement = mei_place_to_above_below(&caesura.caesura_vis.place);

        let notations = note.notations.get_or_insert_with(Notations::default);
        let artics = notations
            .articulations
            .get_or_insert_with(Articulations::default);
        artics.caesura = Some(MxmlCaesura { value, placement });
    }
    Ok(())
}

/// Convert MEI `<arpeg>` control events to MusicXML arpeggiate/non-arpeggiate notations.
fn convert_arpeg_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::data::{TopBottom, UpDown};
    use crate::model::notations::{Arpeggiate, NonArpeggiate, Notations};
    use tusk_model::musicxml_ext::OrnamentDetailData;

    for child in &mei_measure.children {
        let MeasureChild::Arpeg(arpeg) = child else {
            continue;
        };
        let arpeg_staff = arpeg
            .arpeg_log
            .staff
            .as_ref()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1) as usize;
        if arpeg_staff != staff_n {
            continue;
        }
        let start_id = arpeg
            .arpeg_log
            .startid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());
        let Some(sid) = start_id else { continue };
        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
            continue;
        };

        let arpeg_id = arpeg.common.xml_id.as_deref().unwrap_or("");
        let is_nonarp = matches!(
            ctx.ext_store().ornament_detail(arpeg_id),
            Some(OrnamentDetailData::NonArpeggiate)
        );
        let notations = note.notations.get_or_insert_with(Notations::default);

        if is_nonarp {
            notations.non_arpeggiate = Some(NonArpeggiate {
                non_arpeggiate_type: TopBottom::Top,
                number: None,
                default_x: None,
                default_y: None,
                placement: None,
                color: None,
            });
        } else {
            let direction = arpeg.arpeg_log.order.as_deref().and_then(|o| match o {
                "up" => Some(UpDown::Up),
                "down" => Some(UpDown::Down),
                _ => None,
            });
            notations.arpeggiate = Some(Arpeggiate {
                direction,
                ..Default::default()
            });
        }
    }
    Ok(())
}

/// Convert MEI `<gliss>` control events to MusicXML glissando/slide notations on notes.
///
/// For cross-measure glissandos, the start note may be in a previous measure.
fn convert_gliss_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    prev_measures: &mut [MxmlMeasure],
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::data::LineType;
    use crate::model::notations::{Glissando, Slide};
    use tusk_model::data::DataLineform;
    use tusk_model::musicxml_ext::OrnamentDetailData;

    for child in &mei_measure.children {
        let MeasureChild::Gliss(gliss) = child else {
            continue;
        };
        let gliss_staff = gliss
            .gliss_log
            .staff
            .as_ref()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1) as usize;
        if gliss_staff != staff_n {
            continue;
        }

        let start_id = gliss
            .gliss_log
            .startid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());
        let end_id = gliss
            .gliss_log
            .endid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());

        let line_type = gliss.gliss_vis.lform.as_ref().map(|lf| match lf {
            DataLineform::Solid => LineType::Solid,
            DataLineform::Dashed => LineType::Dashed,
            DataLineform::Dotted => LineType::Dotted,
            DataLineform::Wavy => LineType::Wavy,
        });

        // Get text content from children
        let text: String = gliss
            .children
            .iter()
            .map(|c| {
                let tusk_model::elements::GlissChild::Text(t) = c;
                t.as_str()
            })
            .collect::<Vec<_>>()
            .join("");

        let gliss_id = gliss.common.xml_id.as_deref().unwrap_or("");
        let is_slide = matches!(
            ctx.ext_store().ornament_detail(gliss_id),
            Some(OrnamentDetailData::Slide)
        );

        // Helper to create start notation
        let make_start = |line_type: &Option<LineType>, text: &str, is_slide: bool| {
            if is_slide {
                GlissNotation::Slide(Slide {
                    slide_type: crate::model::StartStop::Start,
                    number: Some(1),
                    line_type: *line_type,
                    default_x: None,
                    default_y: None,
                    color: None,
                    text: text.to_string(),
                })
            } else {
                GlissNotation::Glissando(Glissando {
                    glissando_type: crate::model::StartStop::Start,
                    number: Some(1),
                    line_type: *line_type,
                    default_x: None,
                    default_y: None,
                    color: None,
                    text: text.to_string(),
                })
            }
        };

        // Start notation on the start note (may be in a previous measure)
        if let Some(ref sid) = start_id {
            let notation = make_start(&line_type, &text, is_slide);
            if let Some(note) = find_note_by_id_mut(mxml_measure, sid) {
                apply_gliss_notation(note, notation);
            } else {
                // Search previous measures
                for prev in prev_measures.iter_mut().rev() {
                    if let Some(note) = find_note_by_id_mut(prev, sid) {
                        apply_gliss_notation(note, notation);
                        break;
                    }
                }
            }
        }

        // Stop notation on the end note
        if let Some(ref eid) = end_id {
            let notation = if is_slide {
                GlissNotation::Slide(Slide {
                    slide_type: crate::model::StartStop::Stop,
                    number: Some(1),
                    line_type: None,
                    default_x: None,
                    default_y: None,
                    color: None,
                    text: String::new(),
                })
            } else {
                GlissNotation::Glissando(Glissando {
                    glissando_type: crate::model::StartStop::Stop,
                    number: Some(1),
                    line_type: None,
                    default_x: None,
                    default_y: None,
                    color: None,
                    text: String::new(),
                })
            };
            if let Some(note) = find_note_by_id_mut(mxml_measure, eid) {
                apply_gliss_notation(note, notation);
            }
        }
    }
    Ok(())
}

enum GlissNotation {
    Glissando(crate::model::notations::Glissando),
    Slide(crate::model::notations::Slide),
}

fn apply_gliss_notation(note: &mut crate::model::note::Note, notation: GlissNotation) {
    use crate::model::notations::Notations;
    let notations = note.notations.get_or_insert_with(Notations::default);
    match notation {
        GlissNotation::Glissando(g) => notations.glissandos.push(g),
        GlissNotation::Slide(s) => notations.slides.push(s),
    }
}

/// Find a MusicXML note in the measure by its ID (mutable).
/// Convert MEI `@place` (DataStaffrel) to MusicXML `AboveBelow`.
fn mei_place_to_above_below(
    place: &Option<tusk_model::data::DataStaffrel>,
) -> Option<crate::model::data::AboveBelow> {
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic};
    match place {
        Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above)) => {
            Some(crate::model::data::AboveBelow::Above)
        }
        Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Below)) => {
            Some(crate::model::data::AboveBelow::Below)
        }
        _ => None,
    }
}

fn find_note_by_id_mut<'a>(
    mxml_measure: &'a mut MxmlMeasure,
    id: &str,
) -> Option<&'a mut crate::model::note::Note> {
    for content in &mut mxml_measure.content {
        if let MeasureContent::Note(note) = content
            && note.id.as_deref() == Some(id)
        {
            return Some(note);
        }
    }
    None
}

/// Resolve the effective MusicXML local staff number for an MEI note/chord/rest.
///
/// If the element has an MEI `@staff` attribute (indicating cross-staff rendering),
/// convert from global MEI staff to local within-part staff. Otherwise use the
/// default `enclosing_staff_n` from the containing `<staff>` element.
fn resolve_note_staff(
    mei_staff_attr: &Option<String>,
    enclosing_staff_n: usize,
    ctx: &ConversionContext,
) -> u32 {
    if let Some(staff_str) = mei_staff_attr {
        if let Ok(global_staff) = staff_str.parse::<u32>() {
            if let Some(part_id) = ctx.current_part_id() {
                if let Some(local) = ctx.local_staff_for_global(&part_id, global_staff) {
                    return local;
                }
            }
        }
    }
    enclosing_staff_n as u32
}

/// Convert an MEI staff's content to MusicXML measure content.
///
/// The `staff_n` parameter is the 1-based within-part staff number, used to set
/// the `<staff>` element on notes for multi-staff part roundtrip fidelity.
///
/// The `num_staves` parameter is the total number of staves in the part, used
/// to compute voice numbers that are unique across staves.
fn convert_staff_content(
    staff: &Staff,
    staff_n: usize,
    num_staves: usize,
    max_layers_across_staves: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::attributes::Attributes;

    // Find all layers in the staff
    let layers: Vec<_> = staff.children.iter().filter_map(|c| {
        if let StaffChild::Layer(layer) = c { Some(layer) } else { None }
    }).collect();
    let layer_count = layers.len();
    // Track reference duration from first non-empty layer for filling empty layers
    let mut reference_duration: Option<f64> = None;

    for (layer_idx, layer) in layers.iter().enumerate() {
        // Derive MusicXML voice number from MEI layer @n and staff number.
        // For single-staff parts: voice = layer @n (typically 1).
        // For multi-staff parts: offset by (staff-1) * layers_per_staff
        // to ensure unique voice numbers across staves.
        let layer_n: usize = layer
            .n_integer
            .n
            .as_ref()
            .and_then(|s| s.parse().ok())
            .unwrap_or(layer_idx + 1);
        let voice_number = if num_staves > 1 {
            // Use max of (max_layers_across_staves, 2) so voice numbers are unique
            let layers_per_staff = max_layers_across_staves.max(2);
            (staff_n - 1) * layers_per_staff + layer_n
        } else {
            layer_n
        };
        let voice_str = voice_number.to_string();

        // Track content start index so we can assign voice after processing
        let content_start = mxml_measure.content.len();

        // Process layer children, merging consecutive inline attribute changes
        // (KeySig, MeterSig, Clef) into a single MusicXML <attributes> block.
        let mut pending_attrs: Option<Attributes> = None;
        let mut i = 0;
        while i < layer.children.len() {
            let layer_child = &layer.children[i];

            // Check if this is an inline attribute element
            let is_attr_child = matches!(
                layer_child,
                LayerChild::KeySig(_) | LayerChild::MeterSig(_) | LayerChild::Clef(_)
            );

            if is_attr_child {
                let attrs = pending_attrs.get_or_insert_with(Attributes::default);
                match layer_child {
                    LayerChild::KeySig(keysig) => {
                        merge_inline_keysig(keysig, attrs);
                    }
                    LayerChild::MeterSig(metersig) => {
                        merge_inline_metersig(metersig, attrs);
                    }
                    LayerChild::Clef(clef) => {
                        merge_inline_clef(clef, staff_n, attrs);
                    }
                    _ => unreachable!(),
                }
                i += 1;
                continue;
            }

            // Flush pending attributes before non-attribute content
            if let Some(attrs) = pending_attrs.take() {
                mxml_measure
                    .content
                    .push(MeasureContent::Attributes(Box::new(attrs)));
            }

            match layer_child {
                LayerChild::Note(note) => {
                    let effective_staff =
                        resolve_note_staff(&note.note_log.staff, staff_n, ctx);
                    let mut mxml_note = convert_mei_note(note, ctx)?;
                    mxml_note.staff = Some(effective_staff);
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(mxml_note)));
                }
                LayerChild::Rest(rest) => {
                    let effective_staff =
                        resolve_note_staff(&rest.rest_log.staff, staff_n, ctx);
                    let mut mxml_note = convert_mei_rest(rest, ctx)?;
                    mxml_note.staff = Some(effective_staff);
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(mxml_note)));
                }
                LayerChild::Chord(chord) => {
                    let effective_staff =
                        resolve_note_staff(&chord.chord_log.staff, staff_n, ctx);
                    let mxml_notes = convert_mei_chord(chord, ctx)?;
                    for mut note in mxml_notes {
                        note.staff = Some(effective_staff);
                        mxml_measure
                            .content
                            .push(MeasureContent::Note(Box::new(note)));
                    }
                }
                LayerChild::Beam(beam) => {
                    convert_beam_content(beam, staff_n, mxml_measure, ctx)?;
                }
                LayerChild::BTrem(btrem) => {
                    convert_btrem_content(btrem, staff_n, mxml_measure, ctx)?;
                }
                LayerChild::FTrem(ftrem) => {
                    convert_ftrem_content(ftrem, staff_n, mxml_measure, ctx)?;
                }
                LayerChild::MRest(mrest) => {
                    let effective_staff =
                        resolve_note_staff(&mrest.m_rest_log.staff, staff_n, ctx);
                    let mut mxml_note = convert_mei_mrest(mrest, ctx)?;
                    mxml_note.staff = Some(effective_staff);
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(mxml_note)));
                }
                LayerChild::Space(space) => {
                    let forward = convert_mei_space(space, ctx);
                    mxml_measure
                        .content
                        .push(MeasureContent::Forward(Box::new(forward)));
                }
                // DivLine is an MEI-only concept (division line in mensural notation)
                // with no MusicXML equivalent.
                LayerChild::DivLine(_) => {
                    ctx.add_warning(
                        "divLine",
                        "MEI <divLine> (mensural) has no MusicXML equivalent — skipped",
                    );
                }
                // KeySig, MeterSig, Clef handled as inline attributes above.
                LayerChild::KeySig(_) | LayerChild::MeterSig(_) | LayerChild::Clef(_) => {
                    unreachable!("inline attribute elements handled before this match")
                }
                _ => {}
            }
            i += 1;
        }

        // Flush any remaining pending attributes at end of layer
        if let Some(attrs) = pending_attrs.take() {
            mxml_measure
                .content
                .push(MeasureContent::Attributes(Box::new(attrs)));
        }

        // Update reference duration from this layer
        let layer_duration = calculate_staff_duration(mxml_measure, content_start);
        if layer_duration > 0.0 {
            if reference_duration.is_none() {
                reference_duration = Some(layer_duration);
            }
        } else if layer_count > 1 {
            // Layer has no sounding content (only attributes or truly empty).
            // Insert Forward to preserve voice presence in MusicXML. Without
            // this, empty voices disappear on reimport causing instability.
            if let Some(ref_dur) = reference_duration {
                let mut fwd = crate::model::note::Forward::new(ref_dur);
                fwd.voice = Some(voice_str.clone());
                fwd.staff = Some(staff_n as u32);
                mxml_measure.content.push(MeasureContent::Forward(Box::new(fwd)));
            }
        }

        // Assign voice and staff to all notes and forwards added by this layer
        for item in &mut mxml_measure.content[content_start..] {
            match item {
                MeasureContent::Note(note) => {
                    note.voice = Some(voice_str.clone());
                }
                MeasureContent::Forward(forward) => {
                    forward.voice = Some(voice_str.clone());
                    forward.staff = Some(staff_n as u32);
                }
                MeasureContent::Backup(_)
                | MeasureContent::Attributes(_)
                | MeasureContent::Direction(_)
                | MeasureContent::Harmony(_)
                | MeasureContent::FiguredBass(_)
                | MeasureContent::Print(_)
                | MeasureContent::Sound(_)
                | MeasureContent::Listening(_)
                | MeasureContent::Barline(_)
                | MeasureContent::Grouping(_)
                | MeasureContent::Link(_)
                | MeasureContent::Bookmark(_) => {}
            }
        }

        // Insert <backup> between layers (not after the last one) to reset
        // beat position for the next voice in this staff.
        if layer_idx + 1 < layer_count {
            let layer_duration = calculate_staff_duration(mxml_measure, content_start);
            if layer_duration > 0.0 {
                mxml_measure.content.push(MeasureContent::Backup(Box::new(
                    crate::model::note::Backup {
                        duration: layer_duration,
                        footnote: None,
                        level: None,
                    },
                )));
            }
        }
    }
    Ok(())
}

/// Merge inline MEI `<keySig>` into a MusicXML `<attributes>` block.
fn merge_inline_keysig(
    keysig: &tusk_model::elements::KeySig,
    attrs: &mut crate::model::attributes::Attributes,
) {
    use super::attributes::convert_mei_keysig_to_fifths;
    use crate::model::attributes::Key;

    if let Some(ref sig) = keysig.key_sig_log.sig {
        if let Some(fifths) = convert_mei_keysig_to_fifths(&sig.0) {
            let mut key = Key::traditional(fifths, None);
            // Restore print-object="yes" from visible=true (redundant-but-forced keySig).
            if keysig.key_sig_vis.visible == Some(tusk_model::data::DataBoolean::True) {
                key.print_object = Some(crate::model::data::YesNo::Yes);
            }
            attrs.keys.push(key);
        }
    }
}

/// Merge inline MEI `<meterSig>` into a MusicXML `<attributes>` block.
fn merge_inline_metersig(
    metersig: &tusk_model::elements::MeterSig,
    attrs: &mut crate::model::attributes::Attributes,
) {
    use super::attributes::convert_mei_meter_sym_to_mxml;
    use crate::model::attributes::Time;

    let count = metersig.meter_sig_log.count.clone();
    let unit = metersig.meter_sig_log.unit.clone();

    if count.is_some() || unit.is_some() {
        let beats = count.unwrap_or_default();
        let beat_type = unit.unwrap_or_default();
        let mut time = Time::new(&beats, &beat_type);

        if let Some(ref sym) = metersig.meter_sig_log.sym {
            time.symbol = convert_mei_meter_sym_to_mxml(sym);
        }

        attrs.times.push(time);
    }
}

/// Merge inline MEI `<clef>` into a MusicXML `<attributes>` block.
fn merge_inline_clef(
    clef: &tusk_model::elements::Clef,
    staff_n: usize,
    attrs: &mut crate::model::attributes::Attributes,
) {
    use super::attributes::{
        convert_mei_clef_dis_to_octave_change, convert_mei_clef_shape_to_mxml,
    };

    let sign = clef
        .clef_log
        .shape
        .as_ref()
        .map(convert_mei_clef_shape_to_mxml)
        .unwrap_or(crate::model::attributes::ClefSign::G);
    let line = clef.clef_log.line.as_ref().map(|l| l.0 as u32);
    let octave_change = convert_mei_clef_dis_to_octave_change(
        clef.clef_log.dis.as_ref(),
        clef.clef_log.dis_place.as_ref(),
    );

    let mut mxml_clef = crate::model::attributes::Clef::new(sign, line);
    mxml_clef.clef_octave_change = octave_change;
    // Use MEI @staff if present (within-part staff number from import),
    // otherwise fall back to the layer's staff_n
    let clef_number = clef
        .event
        .staff
        .as_ref()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(staff_n as u32);
    mxml_clef.number = Some(clef_number);
    // Restore print-object="yes" from visible=true (redundant-but-forced clef).
    if clef.clef_vis.visible == Some(tusk_model::data::DataBoolean::True) {
        mxml_clef.print_object = Some(crate::model::data::YesNo::Yes);
    }

    attrs.clefs.push(mxml_clef);
}

/// Convert beam content (beams can contain notes, chords, rests).
///
/// Assigns MusicXML beam attributes (begin/continue/end) to notes within the beam group.
/// Rests inside beams do not get beam attributes. Nested beams are flattened.
fn convert_beam_content(
    beam: &tusk_model::elements::Beam,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    // Collect all beamable events (notes/chords) at this level, tracking indices
    let events = collect_beam_events(beam, staff_n, mxml_measure, ctx)?;

    // Count beamable events (non-rest)
    let beamable_count = events.iter().filter(|(_, is_rest)| !is_rest).count();

    if beamable_count >= 2 {
        // Assign beam level 1: begin on first, continue on middle, end on last
        let mut beam_idx = 0;
        for (note_idx, is_rest) in &events {
            if *is_rest {
                continue;
            }
            let beam_value = if beam_idx == 0 {
                crate::model::note::BeamValue::Begin
            } else if beam_idx == beamable_count - 1 {
                crate::model::note::BeamValue::End
            } else {
                crate::model::note::BeamValue::Continue
            };
            // Apply beam to the note at this measure content index
            if let MeasureContent::Note(ref mut note) = mxml_measure.content[*note_idx] {
                note.beams
                    .push(crate::model::note::Beam::with_number(beam_value, 1));
            }
            beam_idx += 1;
        }
    }

    Ok(())
}

/// Collect beam events by converting beam children and pushing them to the measure.
///
/// Returns a vec of (measure_content_index, is_rest) for each event pushed.
fn collect_beam_events(
    beam: &tusk_model::elements::Beam,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<Vec<(usize, bool)>> {
    use tusk_model::elements::BeamChild;

    let mut events = Vec::new();

    for child in &beam.children {
        match child {
            BeamChild::Note(note) => {
                let effective_staff =
                    resolve_note_staff(&note.note_log.staff, staff_n, ctx);
                let mut mxml_note = convert_mei_note(note, ctx)?;
                mxml_note.staff = Some(effective_staff);
                let idx = mxml_measure.content.len();
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
                events.push((idx, false));
            }
            BeamChild::Rest(rest) => {
                let effective_staff =
                    resolve_note_staff(&rest.rest_log.staff, staff_n, ctx);
                let mut mxml_note = convert_mei_rest(rest, ctx)?;
                mxml_note.staff = Some(effective_staff);
                let idx = mxml_measure.content.len();
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
                events.push((idx, true));
            }
            BeamChild::Chord(chord) => {
                let effective_staff =
                    resolve_note_staff(&chord.chord_log.staff, staff_n, ctx);
                let mxml_notes = convert_mei_chord(chord, ctx)?;
                let first_idx = mxml_measure.content.len();
                for mut note in mxml_notes {
                    note.staff = Some(effective_staff);
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(note)));
                }
                // The chord's first note carries the beam; all chord notes
                // share the same beam grouping in MusicXML
                events.push((first_idx, false));
            }
            BeamChild::Beam(nested_beam) => {
                // Flatten nested beams into the same beam group
                let nested = collect_beam_events(nested_beam, staff_n, mxml_measure, ctx)?;
                events.extend(nested);
            }
            BeamChild::BTrem(btrem) => {
                let first_idx = mxml_measure.content.len();
                convert_btrem_content(btrem, staff_n, mxml_measure, ctx)?;
                // bTrem has one note/chord
                if mxml_measure.content.len() > first_idx {
                    events.push((first_idx, false));
                }
            }
            BeamChild::FTrem(ftrem) => {
                let first_idx = mxml_measure.content.len();
                convert_ftrem_content(ftrem, staff_n, mxml_measure, ctx)?;
                // fTrem has two notes/chords; first is beamable
                if mxml_measure.content.len() > first_idx {
                    events.push((first_idx, false));
                }
            }
            _ => {}
        }
    }
    Ok(events)
}

/// Convert MEI `<space>` → MusicXML `<forward>`.
///
/// Calculates duration in divisions from `@dur`/`@dots`, falling back to
/// `@dur.ppq` (raw divisions) if written duration is absent.
fn convert_mei_space(
    space: &tusk_model::elements::Space,
    ctx: &ConversionContext,
) -> crate::model::note::Forward {
    use super::utils::{apply_dots, duration_to_quarter_notes};

    let divisions = ctx.divisions();

    let duration = if let Some(ref dur) = space.space_log.dur {
        let base = duration_to_quarter_notes(dur);
        let dot_count = space.space_log.dots.as_ref().map(|d| d.0).unwrap_or(0);
        apply_dots(base, dot_count) * divisions
    } else if let Some(ref ppq) = space.space_ges.dur_ppq {
        ppq.parse::<f64>().unwrap_or(divisions)
    } else {
        // Default: one quarter note
        divisions
    };

    crate::model::note::Forward::new(duration)
}

/// Derive tremolo mark count from MEI bTrem/fTrem @unitdur attribute.
///
/// MusicXML tremolo value = number of beams (1=8th, 2=16th, 3=32nd).
/// MEI @unitdur gives the performed note duration: 8→1, 16→2, 32→3, 64→4.
fn unitdur_to_tremolo_marks(unitdur: &tusk_model::DataDurationCmn) -> u8 {
    use tusk_model::DataDurationCmn;
    match unitdur {
        DataDurationCmn::N8 => 1,
        DataDurationCmn::N16 => 2,
        DataDurationCmn::N32 => 3,
        DataDurationCmn::N64 => 4,
        DataDurationCmn::N128 => 5,
        DataDurationCmn::N256 => 6,
        _ => 3, // default
    }
}

/// Add tremolo notation to a MusicXML note.
fn add_tremolo_to_note(
    note: &mut crate::model::note::Note,
    tremolo_type: crate::model::data::TremoloType,
    value: u8,
) {
    use crate::model::notations::{Notations, Ornaments, Tremolo};

    let tremolo = Tremolo {
        tremolo_type,
        value: Some(value),
        placement: None,
        default_x: None,
        default_y: None,
        color: None,
        smufl: None,
    };

    let notations = note.notations.get_or_insert_with(Notations::default);
    let ornaments = notations.ornaments.get_or_insert_with(Ornaments::default);
    ornaments.tremolo = Some(tremolo);
}

/// Convert MEI bTrem (bowed tremolo) to MusicXML notes with tremolo type="single".
///
/// bTrem wraps a single note or chord. The contained note gets a
/// `<tremolo type="single">N</tremolo>` ornament where N = number of marks.
fn convert_btrem_content(
    btrem: &tusk_model::elements::BTrem,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use tusk_model::elements::BTremChild;

    // Determine tremolo marks from @unitdur or @num
    let marks = if let Some(ref unitdur) = btrem.b_trem_ges.unitdur {
        unitdur_to_tremolo_marks(unitdur)
    } else if let Some(ref num) = btrem.b_trem_log.num {
        num.parse::<u8>().unwrap_or(3)
    } else {
        3 // default: 3 marks (32nd note tremolo)
    };

    for child in &btrem.children {
        match child {
            BTremChild::Note(note) => {
                let effective_staff =
                    resolve_note_staff(&note.note_log.staff, staff_n, ctx);
                let mut mxml_note = convert_mei_note(note, ctx)?;
                mxml_note.staff = Some(effective_staff);
                add_tremolo_to_note(
                    &mut mxml_note,
                    crate::model::data::TremoloType::Single,
                    marks,
                );
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
            }
            BTremChild::Chord(chord) => {
                let effective_staff =
                    resolve_note_staff(&chord.chord_log.staff, staff_n, ctx);
                let mxml_notes = convert_mei_chord(chord, ctx)?;
                let mut first = true;
                for mut note in mxml_notes {
                    note.staff = Some(effective_staff);
                    if first {
                        add_tremolo_to_note(
                            &mut note,
                            crate::model::data::TremoloType::Single,
                            marks,
                        );
                        first = false;
                    }
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(note)));
                }
            }
        }
    }
    Ok(())
}

/// Convert MEI fTrem (fingered tremolo) to MusicXML notes with tremolo type="start/stop".
///
/// fTrem wraps two notes/chords. The first gets `<tremolo type="start">N</tremolo>`,
/// the second gets `<tremolo type="stop">N</tremolo>`.
fn convert_ftrem_content(
    ftrem: &tusk_model::elements::FTrem,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use tusk_model::elements::FTremChild;

    // Determine tremolo marks from @unitdur or @form
    let marks = if let Some(ref unitdur) = ftrem.f_trem_ges.unitdur {
        unitdur_to_tremolo_marks(unitdur)
    } else {
        2 // default: 2 marks (16th note fingered tremolo)
    };

    // Collect note/chord children (skip clefs)
    let note_children: Vec<_> = ftrem
        .children
        .iter()
        .filter(|c| !matches!(c, FTremChild::Clef(_)))
        .collect();
    for (i, child) in note_children.iter().enumerate() {
        let ttype = if i == 0 {
            crate::model::data::TremoloType::Start
        } else {
            crate::model::data::TremoloType::Stop
        };

        match child {
            FTremChild::Note(note) => {
                let effective_staff =
                    resolve_note_staff(&note.note_log.staff, staff_n, ctx);
                let mut mxml_note = convert_mei_note(note, ctx)?;
                mxml_note.staff = Some(effective_staff);
                add_tremolo_to_note(&mut mxml_note, ttype, marks);
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
            }
            FTremChild::Chord(chord) => {
                let effective_staff =
                    resolve_note_staff(&chord.chord_log.staff, staff_n, ctx);
                let mxml_notes = convert_mei_chord(chord, ctx)?;
                let mut first = true;
                for mut note in mxml_notes {
                    note.staff = Some(effective_staff);
                    if first {
                        add_tremolo_to_note(&mut note, ttype, marks);
                        first = false;
                    }
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(note)));
                }
            }
            FTremChild::Clef(_) => {} // already filtered
        }
    }
    Ok(())
}

/// Create initial attributes for the first measure (divisions, etc.).
/// Calculate smart divisions value based on the smallest note duration in the score.
///
/// MusicXML `<duration>` must be an integer. This function finds the smallest note
/// duration and sets divisions so that all durations become integers.
/// Default is 4 (supports up to 16th notes).
fn calculate_smart_divisions(mei_measures: &[&tusk_model::elements::Measure]) -> f64 {
    use tusk_model::elements::{MeasureChild, StaffChild};

    let mut smallest_duration = 1.0_f64; // Start with quarter note

    for measure in mei_measures {
        for child in &measure.children {
            if let MeasureChild::Staff(staff) = child {
                for sc in &staff.children {
                    if let StaffChild::Layer(layer) = sc {
                        find_smallest_duration_in_layer(&layer.children, &mut smallest_duration);
                    }
                }
            }
        }
    }

    // Convert smallest duration (in quarter notes) to divisions
    // E.g., if smallest = 0.25 (16th), we need divisions = 4
    // If smallest = 0.125 (32nd), we need divisions = 8
    // If smallest = 0.0625 (64th), we need divisions = 16
    let divisions = (1.0 / smallest_duration).ceil();

    // Ensure minimum of 1
    divisions.max(1.0)
}

/// Recursively find the smallest note duration in layer children.
fn find_smallest_duration_in_layer(
    children: &[tusk_model::elements::LayerChild],
    smallest: &mut f64,
) {
    use super::utils::{duration_rests_to_quarter_notes, duration_to_quarter_notes};
    use tusk_model::elements::LayerChild;

    for child in children {
        match child {
            LayerChild::Note(note) => {
                if let Some(ref dur) = note.note_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            LayerChild::Rest(rest) => {
                if let Some(ref dur) = rest.rest_log.dur {
                    let quarters = duration_rests_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            LayerChild::Chord(chord) => {
                if let Some(ref dur) = chord.chord_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            LayerChild::Beam(beam) => {
                find_smallest_duration_in_beam(&beam.children, smallest);
            }
            LayerChild::BTrem(btrem) => {
                find_smallest_duration_in_btrem(&btrem.children, smallest);
            }
            LayerChild::FTrem(ftrem) => {
                find_smallest_duration_in_ftrem(&ftrem.children, smallest);
            }
            LayerChild::Space(space) => {
                if let Some(ref dur) = space.space_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            LayerChild::MRest(_)
            | LayerChild::DivLine(_)
            | LayerChild::KeySig(_)
            | LayerChild::MeterSig(_)
            | LayerChild::Clef(_) => {}
            _ => {}
        }
    }
}

/// Recursively find the smallest note duration in beam children.
fn find_smallest_duration_in_beam(
    children: &[tusk_model::elements::BeamChild],
    smallest: &mut f64,
) {
    use super::utils::{duration_rests_to_quarter_notes, duration_to_quarter_notes};
    use tusk_model::elements::BeamChild;

    for child in children {
        match child {
            BeamChild::Note(note) => {
                if let Some(ref dur) = note.note_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            BeamChild::Rest(rest) => {
                if let Some(ref dur) = rest.rest_log.dur {
                    let quarters = duration_rests_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            BeamChild::Chord(chord) => {
                if let Some(ref dur) = chord.chord_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            BeamChild::Beam(nested) => {
                find_smallest_duration_in_beam(&nested.children, smallest);
            }
            BeamChild::BTrem(btrem) => {
                find_smallest_duration_in_btrem(&btrem.children, smallest);
            }
            BeamChild::FTrem(ftrem) => {
                find_smallest_duration_in_ftrem(&ftrem.children, smallest);
            }
            _ => {}
        }
    }
}

/// Find smallest note duration in bTrem children.
fn find_smallest_duration_in_btrem(
    children: &[tusk_model::elements::BTremChild],
    smallest: &mut f64,
) {
    use super::utils::duration_to_quarter_notes;
    use tusk_model::elements::BTremChild;
    for child in children {
        match child {
            BTremChild::Note(note) => {
                if let Some(ref dur) = note.note_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            BTremChild::Chord(chord) => {
                if let Some(ref dur) = chord.chord_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
        }
    }
}

/// Find smallest note duration in fTrem children.
fn find_smallest_duration_in_ftrem(
    children: &[tusk_model::elements::FTremChild],
    smallest: &mut f64,
) {
    use super::utils::duration_to_quarter_notes;
    use tusk_model::elements::FTremChild;
    for child in children {
        match child {
            FTremChild::Note(note) => {
                if let Some(ref dur) = note.note_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            FTremChild::Chord(chord) => {
                if let Some(ref dur) = chord.chord_log.dur {
                    let quarters = duration_to_quarter_notes(dur);
                    if quarters < *smallest && quarters > 0.0 {
                        *smallest = quarters;
                    }
                }
            }
            FTremChild::Clef(_) => {}
        }
    }
}

/// Calculate the total duration of notes/rests added to a measure from a given content offset.
///
/// Used to compute the `<backup>` duration between staves in multi-staff parts.
/// Chord notes (note.chord.is_some()) don't advance time, nor do grace notes.
fn calculate_staff_duration(mxml_measure: &MxmlMeasure, content_from: usize) -> f64 {
    let mut total = 0.0;
    for item in &mxml_measure.content[content_from..] {
        match item {
            MeasureContent::Note(note) => {
                // Chord notes and grace notes don't advance time
                if note.chord.is_none() && note.grace.is_none() {
                    total += note.duration.unwrap_or(0.0);
                }
            }
            MeasureContent::Forward(fwd) => {
                total += fwd.duration;
            }
            MeasureContent::Backup(bk) => {
                total -= bk.duration;
            }
            // Non-duration-carrying elements don't affect timing.
            MeasureContent::Attributes(_)
            | MeasureContent::Direction(_)
            | MeasureContent::Harmony(_)
            | MeasureContent::FiguredBass(_)
            | MeasureContent::Print(_)
            | MeasureContent::Sound(_)
            | MeasureContent::Listening(_)
            | MeasureContent::Barline(_)
            | MeasureContent::Grouping(_)
            | MeasureContent::Link(_)
            | MeasureContent::Bookmark(_) => {}
        }
    }
    total.max(0.0)
}

// build_first_measure_attributes and build_first_measure_attributes_multi
// are in super::attributes to keep this module under the line limit.

/// Convert MEI `<ornam>` and `<fing>` control events with `TechnicalDetailData` in ExtensionStore
/// back to MusicXML `<technical>` notations on the referenced notes.
fn convert_technical_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use super::direction::convert_place_to_placement;
    use crate::model::data::{StartStop, YesNo};
    use crate::model::notations::Notations;
    use crate::model::technical::*;
    use tusk_model::musicxml_ext::TechnicalDetailData;

    for child in &mei_measure.children {
        let MeasureChild::Ornam(ornam) = child else {
            continue;
        };
        let ornam_id = match ornam.common.xml_id.as_deref() {
            Some(id) => id,
            None => continue,
        };
        let data = match ctx.ext_store().technical_detail(ornam_id) {
            Some(d) => d.clone(),
            None => continue,
        };

        let ornam_staff = ornam
            .ornam_log
            .staff
            .as_ref()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1) as usize;
        if ornam_staff != staff_n {
            continue;
        }

        let start_id = ornam
            .ornam_log
            .startid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());
        let Some(sid) = start_id else { continue };
        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
            continue;
        };
        let notations = note.notations.get_or_insert_with(Notations::default);
        let tech = notations.technical.get_or_insert_with(Technical::default);

        let placement = convert_place_to_placement(&ornam.ornam_vis.place);
        let ep = || crate::model::notations::EmptyPlacement {
            placement,
            ..Default::default()
        };

        match data {
            // Simple placement-only types
            TechnicalDetailData::UpBow => tech.up_bow.push(ep()),
            TechnicalDetailData::DownBow => tech.down_bow.push(ep()),
            TechnicalDetailData::OpenString => tech.open_string.push(ep()),
            TechnicalDetailData::ThumbPosition => tech.thumb_position.push(ep()),
            TechnicalDetailData::DoubleTongue => tech.double_tongue.push(ep()),
            TechnicalDetailData::TripleTongue => tech.triple_tongue.push(ep()),
            TechnicalDetailData::SnapPizzicato => tech.snap_pizzicato.push(ep()),
            TechnicalDetailData::Fingernails => tech.fingernails.push(ep()),
            TechnicalDetailData::BrassBend => tech.brass_bend.push(ep()),
            TechnicalDetailData::Flip => tech.flip.push(ep()),
            TechnicalDetailData::Smear => tech.smear.push(ep()),
            TechnicalDetailData::Golpe => tech.golpe.push(ep()),

            // Placement + SMuFL types
            TechnicalDetailData::Stopped { smufl } => tech.stopped.push(EmptyPlacementSmufl {
                placement,
                smufl,
                ..Default::default()
            }),
            TechnicalDetailData::Open { smufl } => tech.open.push(EmptyPlacementSmufl {
                placement,
                smufl,
                ..Default::default()
            }),
            TechnicalDetailData::HalfMuted { smufl } => tech.half_muted.push(EmptyPlacementSmufl {
                placement,
                smufl,
                ..Default::default()
            }),

            // Text content types
            TechnicalDetailData::Pluck { value } => tech.pluck.push(PlacementText {
                value,
                placement,
                default_x: None,
                default_y: None,
                font_style: None,
                color: None,
            }),
            TechnicalDetailData::Fret { value } => tech.fret.push(Fret { value, color: None }),
            TechnicalDetailData::StringNum { value } => tech.string.push(TechString {
                value,
                placement,
                default_x: None,
                default_y: None,
                color: None,
            }),
            TechnicalDetailData::Handbell { value } => tech.handbell.push(Handbell {
                value,
                placement,
                default_x: None,
                default_y: None,
                color: None,
            }),

            // Parameterized types
            TechnicalDetailData::HammerOn {
                ho_type,
                number,
                text,
            } => {
                let ss = match ho_type.as_str() {
                    "stop" => StartStop::Stop,
                    _ => StartStop::Start,
                };
                tech.hammer_on.push(HammerOnPullOff {
                    ho_type: ss,
                    number,
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                    text,
                });
            }
            TechnicalDetailData::PullOff {
                po_type,
                number,
                text,
            } => {
                let ss = match po_type.as_str() {
                    "stop" => StartStop::Stop,
                    _ => StartStop::Start,
                };
                tech.pull_off.push(HammerOnPullOff {
                    ho_type: ss,
                    number,
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                    text,
                });
            }
            TechnicalDetailData::Tap { hand, value } => {
                let h = hand.and_then(|s| match s.as_str() {
                    "left" => Some(TapHand::Left),
                    "right" => Some(TapHand::Right),
                    _ => None,
                });
                tech.tap.push(Tap {
                    value,
                    hand: h,
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }
            TechnicalDetailData::Heel { substitution } => tech.heel.push(HeelToe {
                substitution: if substitution == Some(true) {
                    Some(YesNo::Yes)
                } else {
                    None
                },
                placement,
                ..Default::default()
            }),
            TechnicalDetailData::Toe { substitution } => tech.toe.push(HeelToe {
                substitution: if substitution == Some(true) {
                    Some(YesNo::Yes)
                } else {
                    None
                },
                placement,
                ..Default::default()
            }),

            // Complex types
            TechnicalDetailData::Bend {
                alter,
                pre_bend,
                release,
                shape,
                with_bar,
            } => {
                let bend_release = release.map(|offset| BendRelease { offset });
                let bend_shape = shape.and_then(|s| match s.as_str() {
                    "straight" => Some(BendShape::Straight),
                    "curved" => Some(BendShape::Curved),
                    _ => None,
                });
                let with_bar_pt = with_bar.map(|wb| PlacementText {
                    value: wb,
                    placement: None,
                    default_x: None,
                    default_y: None,
                    font_style: None,
                    color: None,
                });
                tech.bend.push(Bend {
                    bend_alter: alter,
                    pre_bend,
                    release: bend_release,
                    with_bar: with_bar_pt,
                    shape: bend_shape,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }
            TechnicalDetailData::Hole {
                closed,
                location,
                hole_type,
                hole_shape,
            } => {
                let closed_val = match closed.as_str() {
                    "no" => HoleClosedValue::No,
                    "half" => HoleClosedValue::Half,
                    _ => HoleClosedValue::Yes,
                };
                let loc = location.and_then(|l| match l.as_str() {
                    "right" => Some(HoleClosedLocation::Right),
                    "bottom" => Some(HoleClosedLocation::Bottom),
                    "left" => Some(HoleClosedLocation::Left),
                    "top" => Some(HoleClosedLocation::Top),
                    _ => None,
                });
                tech.hole.push(Hole {
                    hole_type,
                    hole_closed: HoleClosed {
                        value: closed_val,
                        location: loc,
                    },
                    hole_shape,
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }
            TechnicalDetailData::Arrow { content } => {
                let ac = match content {
                    tusk_model::musicxml_ext::ArrowContentData::Directional {
                        direction,
                        style,
                        arrowhead,
                    } => ArrowContent::Directional {
                        direction,
                        style,
                        arrowhead,
                    },
                    tusk_model::musicxml_ext::ArrowContentData::Circular(v) => {
                        ArrowContent::Circular(v)
                    }
                };
                tech.arrow.push(Arrow {
                    content: ac,
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                    smufl: None,
                });
            }
            TechnicalDetailData::HarmonMute { closed, location } => {
                let closed_val = match closed.as_str() {
                    "no" => HarmonClosedValue::No,
                    "half" => HarmonClosedValue::Half,
                    _ => HarmonClosedValue::Yes,
                };
                let loc = location.and_then(|l| match l.as_str() {
                    "right" => Some(HarmonClosedLocation::Right),
                    "bottom" => Some(HarmonClosedLocation::Bottom),
                    "left" => Some(HarmonClosedLocation::Left),
                    "top" => Some(HarmonClosedLocation::Top),
                    _ => None,
                });
                tech.harmon_mute.push(HarmonMute {
                    harmon_closed: HarmonClosed {
                        value: closed_val,
                        location: loc,
                    },
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }
            TechnicalDetailData::Harmonic {
                natural,
                artificial,
                base_pitch,
                touching_pitch,
                sounding_pitch,
            } => {
                tech.harmonic.push(Harmonic {
                    natural,
                    artificial,
                    base_pitch,
                    touching_pitch,
                    sounding_pitch,
                    placement,
                    ..Default::default()
                });
            }
            TechnicalDetailData::OtherTechnical { smufl, text } => {
                tech.other_technical.push(OtherTechnical {
                    value: text,
                    placement,
                    smufl,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }

            // Fingering is handled separately via <fing> below;
            // TechArticulation is handled in convert_mei_note_label_technical.
            TechnicalDetailData::Fingering { .. }
            | TechnicalDetailData::TechArticulation { .. } => {}
        }
    }

    // Handle native MEI <fing> elements → MusicXML <fingering>
    for child in &mei_measure.children {
        let MeasureChild::Fing(fing) = child else {
            continue;
        };
        let fing_staff = fing
            .fing_log
            .staff
            .as_ref()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1) as usize;
        if fing_staff != staff_n {
            continue;
        }
        let start_id = fing
            .fing_log
            .startid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());
        let Some(sid) = start_id else { continue };
        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
            continue;
        };
        let notations = note.notations.get_or_insert_with(Notations::default);
        let tech = notations.technical.get_or_insert_with(Technical::default);

        let placement = convert_place_to_placement(&fing.fing_vis.place);

        // Extract text content
        let text: String = fing
            .children
            .iter()
            .map(|c| match c {
                tusk_model::elements::FingChild::Text(t) => t.as_str(),
            })
            .collect::<Vec<_>>()
            .join("");

        // Read substitution/alternate from ExtensionStore
        let mut substitution = None;
        let mut alternate = None;
        if let Some(fing_id) = fing.common.xml_id.as_deref() {
            if let Some(TechnicalDetailData::Fingering {
                substitution: sub,
                alternate: alt,
            }) = ctx.ext_store().technical_detail(fing_id)
            {
                if *sub == Some(true) {
                    substitution = Some(YesNo::Yes);
                }
                if *alt == Some(true) {
                    alternate = Some(YesNo::Yes);
                }
            }
        }

        tech.fingering.push(Fingering {
            value: text,
            substitution,
            alternate,
            placement,
            default_x: None,
            default_y: None,
            color: None,
        });
    }

    Ok(())
}

/// Convert MEI `<dynam>` control events marked as NotationDynamics in ExtensionStore
/// back to MusicXML `<dynamics>` within `<notations>` on the referenced notes.
fn convert_notation_dynamics(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use super::direction::{convert_place_to_placement, parse_dynamics_text};
    use crate::model::direction::Dynamics;
    use crate::model::notations::Notations;
    use tusk_model::elements::DynamChild;
    use tusk_model::musicxml_ext::OrnamentDetailData;

    for child in &mei_measure.children {
        let MeasureChild::Dynam(dynam) = child else {
            continue;
        };
        // Only process notation-level dynamics (identified via ExtensionStore)
        let is_notation_dyn = dynam.common.xml_id.as_deref().is_some_and(|id| {
            matches!(
                ctx.ext_store().ornament_detail(id),
                Some(OrnamentDetailData::NotationDynamics)
            )
        });
        if !is_notation_dyn {
            continue;
        }
        let event_staff = dynam
            .dynam_log
            .staff
            .as_ref()
            .and_then(|s| s.split_whitespace().next())
            .and_then(|s| s.parse().ok())
            .unwrap_or(1) as usize;
        if event_staff != staff_n {
            continue;
        }
        let start_id = dynam
            .dynam_log
            .startid
            .as_ref()
            .map(|uri| uri.to_string().trim_start_matches('#').to_string());
        let Some(sid) = start_id else { continue };
        let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
            continue;
        };

        // Extract text content
        let text_content: String = dynam
            .children
            .iter()
            .filter_map(|child| {
                if let DynamChild::Text(t) = child {
                    Some(t.as_str())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("");

        let dynamics_value = parse_dynamics_text(&text_content);
        let placement = convert_place_to_placement(&dynam.dynam_vis.place);

        let notations = note.notations.get_or_insert_with(Notations::default);
        notations.dynamics.push(Dynamics {
            values: vec![dynamics_value],
            placement,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;
    use tusk_model::elements::{
        Layer, LayerChild, Measure as MeiMeasure, MeasureChild, Note as MeiNote, Score as MeiScore,
        ScoreChild, Section, SectionChild, Staff, StaffChild,
    };

    fn create_simple_mei_score() -> MeiScore {
        use tusk_model::data::{
            DataDuration, DataDurationCmn, DataOctave, DataPitchname, DataWord,
        };

        let mut score = MeiScore::default();

        // Create section with one measure
        let mut section = Section::default();
        let mut measure = MeiMeasure::default();
        measure.common.n = Some(DataWord("1".to_string()));

        // Create staff with layer containing a note
        let mut staff = Staff::default();
        staff.n_integer.n = Some("1".to_string());

        let mut layer = Layer::default();
        let mut note = MeiNote::default();
        note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note.note_log.oct = Some(DataOctave::from(4u64));
        note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

        layer.children.push(LayerChild::Note(Box::new(note)));
        staff.children.push(StaffChild::Layer(Box::new(layer)));
        measure.children.push(MeasureChild::Staff(Box::new(staff)));
        section
            .children
            .push(SectionChild::Measure(Box::new(measure)));
        score.children.push(ScoreChild::Section(Box::new(section)));

        score
    }

    #[test]
    fn test_collect_measures_from_score() {
        let score = create_simple_mei_score();
        let (measures, endings) = collect_measures_from_score(&score);
        assert_eq!(measures.len(), 1);
        assert!(endings.is_empty());
    }

    #[test]
    fn test_convert_mei_score_content_single_part() {
        let score = create_simple_mei_score();
        let part_ids = vec!["P1".to_string()];
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_score_content(&score, &part_ids, &mut ctx);
        assert!(result.is_ok());

        let tw_measures = result.unwrap();
        assert_eq!(tw_measures.len(), 1);
        assert_eq!(tw_measures[0].number, "1");
        assert_eq!(tw_measures[0].parts.len(), 1);
        assert_eq!(tw_measures[0].parts[0].id, "P1");
        // Should have attributes + note
        assert!(!tw_measures[0].parts[0].content.is_empty());
    }

    #[test]
    fn test_find_staff_in_measure() {
        let mut measure = MeiMeasure::default();
        let mut staff1 = Staff::default();
        staff1.n_integer.n = Some("1".to_string());
        let mut staff2 = Staff::default();
        staff2.n_integer.n = Some("2".to_string());

        measure.children.push(MeasureChild::Staff(Box::new(staff1)));
        measure.children.push(MeasureChild::Staff(Box::new(staff2)));

        assert!(find_staff_in_measure(&measure, 1).is_some());
        assert!(find_staff_in_measure(&measure, 2).is_some());
        assert!(find_staff_in_measure(&measure, 3).is_none());
    }

    #[test]
    fn test_btrem_export_produces_tremolo_single() {
        use tusk_model::data::{
            DataDuration, DataDurationCmn, DataOctave, DataPitchname, DataWord,
        };
        use tusk_model::elements::{BTrem, BTremChild};

        let mut score = MeiScore::default();
        let mut section = Section::default();
        let mut measure = MeiMeasure::default();
        measure.common.n = Some(DataWord("1".to_string()));

        let mut staff = Staff::default();
        staff.n_integer.n = Some("1".to_string());

        // Build a bTrem wrapping a quarter note C4 with unitdur=32 (3 marks)
        let mut btrem = BTrem::default();
        btrem.b_trem_ges.unitdur = Some(DataDurationCmn::N32);

        let mut note = MeiNote::default();
        note.common.xml_id = Some("n1".to_string());
        note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note.note_log.oct = Some(DataOctave::from(4u64));
        note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));
        btrem.children.push(BTremChild::Note(Box::new(note)));

        let mut layer = Layer::default();
        layer.children.push(LayerChild::BTrem(Box::new(btrem)));
        staff.children.push(StaffChild::Layer(Box::new(layer)));
        measure.children.push(MeasureChild::Staff(Box::new(staff)));
        section
            .children
            .push(SectionChild::Measure(Box::new(measure)));
        score.children.push(ScoreChild::Section(Box::new(section)));

        let part_ids = vec!["P1".to_string()];
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let tw_measures = convert_mei_score_content(&score, &part_ids, &mut ctx).unwrap();
        assert_eq!(tw_measures.len(), 1);

        // Find the note in measure content (skip attributes)
        let part = &tw_measures[0].parts[0];
        let notes: Vec<_> = part
            .content
            .iter()
            .filter_map(|c| {
                if let crate::model::elements::MeasureContent::Note(n) = c {
                    Some(n.as_ref())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(notes.len(), 1, "should have one note from bTrem");

        // Verify tremolo notation
        let notations = notes[0]
            .notations
            .as_ref()
            .expect("note should have notations");
        let ornaments = notations.ornaments.as_ref().expect("should have ornaments");
        let tremolo = ornaments.tremolo.as_ref().expect("should have tremolo");
        assert_eq!(
            tremolo.tremolo_type,
            crate::model::data::TremoloType::Single
        );
        assert_eq!(tremolo.value, Some(3)); // 32nd → 3 marks
    }

    #[test]
    fn test_ftrem_export_produces_tremolo_start_stop() {
        use tusk_model::data::{
            DataDuration, DataDurationCmn, DataOctave, DataPitchname, DataWord,
        };
        use tusk_model::elements::{FTrem, FTremChild};

        let mut score = MeiScore::default();
        let mut section = Section::default();
        let mut measure = MeiMeasure::default();
        measure.common.n = Some(DataWord("1".to_string()));

        let mut staff = Staff::default();
        staff.n_integer.n = Some("1".to_string());

        // Build an fTrem with two eighth notes (C4 and E4) with unitdur=16 (2 marks)
        let mut ftrem = FTrem::default();
        ftrem.f_trem_ges.unitdur = Some(DataDurationCmn::N16);

        let mut note1 = MeiNote::default();
        note1.common.xml_id = Some("n1".to_string());
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));
        note1.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N8));
        ftrem.children.push(FTremChild::Note(Box::new(note1)));

        let mut note2 = MeiNote::default();
        note2.common.xml_id = Some("n2".to_string());
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave::from(4u64));
        note2.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N8));
        ftrem.children.push(FTremChild::Note(Box::new(note2)));

        let mut layer = Layer::default();
        layer.children.push(LayerChild::FTrem(Box::new(ftrem)));
        staff.children.push(StaffChild::Layer(Box::new(layer)));
        measure.children.push(MeasureChild::Staff(Box::new(staff)));
        section
            .children
            .push(SectionChild::Measure(Box::new(measure)));
        score.children.push(ScoreChild::Section(Box::new(section)));

        let part_ids = vec!["P1".to_string()];
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(2.0);

        let tw_measures = convert_mei_score_content(&score, &part_ids, &mut ctx).unwrap();
        assert_eq!(tw_measures.len(), 1);

        let part = &tw_measures[0].parts[0];
        let notes: Vec<_> = part
            .content
            .iter()
            .filter_map(|c| {
                if let crate::model::elements::MeasureContent::Note(n) = c {
                    Some(n.as_ref())
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(notes.len(), 2, "should have two notes from fTrem");

        // First note: tremolo start
        let n1 = notes[0]
            .notations
            .as_ref()
            .expect("note 1 should have notations");
        let orn1 = n1.ornaments.as_ref().expect("should have ornaments");
        let t1 = orn1.tremolo.as_ref().expect("should have tremolo");
        assert_eq!(t1.tremolo_type, crate::model::data::TremoloType::Start);
        assert_eq!(t1.value, Some(2)); // 16th → 2 marks

        // Second note: tremolo stop
        let n2 = notes[1]
            .notations
            .as_ref()
            .expect("note 2 should have notations");
        let orn2 = n2.ornaments.as_ref().expect("should have ornaments");
        let t2 = orn2.tremolo.as_ref().expect("should have tremolo");
        assert_eq!(t2.tremolo_type, crate::model::data::TremoloType::Stop);
        assert_eq!(t2.value, Some(2));
    }

    // ========================================================================
    // Space → Forward Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_space_quarter_note() {
        use tusk_model::data::{DataDuration, DataDurationCmn};
        use tusk_model::elements::Space;
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let mut space = Space::default();
        space.space_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

        let forward = convert_mei_space(&space, &ctx);
        assert!((forward.duration - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_convert_mei_space_dotted_half() {
        use tusk_model::data::{DataAugmentdot, DataDuration, DataDurationCmn};
        use tusk_model::elements::Space;
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let mut space = Space::default();
        space.space_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N2));
        space.space_log.dots = Some(DataAugmentdot(1));

        let forward = convert_mei_space(&space, &ctx);
        // Dotted half = 2.0 + 1.0 = 3.0 quarter notes × 4 divisions = 12
        assert!((forward.duration - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_convert_mei_space_dur_ppq_fallback() {
        use tusk_model::elements::Space;
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let mut space = Space::default();
        space.space_ges.dur_ppq = Some("7".to_string());

        let forward = convert_mei_space(&space, &ctx);
        assert!((forward.duration - 7.0).abs() < 1e-10);
    }

    #[test]
    fn test_convert_mei_space_no_duration_defaults_to_quarter() {
        use tusk_model::elements::Space;
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let space = Space::default();

        let forward = convert_mei_space(&space, &ctx);
        // Default: one quarter note = 4 divisions
        assert!((forward.duration - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_apply_export_octave_spans_to_measure_transposes_entire_chord_on_same_onset() {
        use crate::model::data::Step;
        use crate::model::elements::{Empty, MeasureContent};
        use crate::model::note::{Note as MxmlNote, Pitch};

        let mut measure = MxmlMeasure::new("1");

        let mut first = MxmlNote::pitched(
            Pitch {
                step: Step::C,
                alter: None,
                octave: 4,
            },
            1.0,
        );
        first.staff = Some(1);
        measure.content.push(MeasureContent::Note(Box::new(first)));

        let mut chord_root = MxmlNote::pitched(
            Pitch {
                step: Step::E,
                alter: None,
                octave: 4,
            },
            1.0,
        );
        chord_root.staff = Some(1);
        measure.content.push(MeasureContent::Note(Box::new(chord_root)));

        let mut chord_top = MxmlNote::pitched(
            Pitch {
                step: Step::G,
                alter: None,
                octave: 4,
            },
            1.0,
        );
        chord_top.staff = Some(1);
        chord_top.chord = Some(Empty);
        measure.content.push(MeasureContent::Note(Box::new(chord_top)));

        apply_export_octave_spans_to_measure(
            0,
            0,
            &mut measure,
            &[ExportOctaveSpan {
                part_idx: 0,
                staff: 1,
                start_measure: 0,
                start_beat: 2.0,
                end_measure: 0,
                end_beat: 3.0,
                octave_delta: 1,
            }],
            &HashMap::new(),
        );

        let MeasureContent::Note(first) = &measure.content[0] else {
            panic!("expected first note");
        };
        let MeasureContent::Note(chord_root) = &measure.content[1] else {
            panic!("expected chord root");
        };
        let MeasureContent::Note(chord_top) = &measure.content[2] else {
            panic!("expected chord top");
        };

        let crate::model::note::FullNoteContent::Pitch(first_pitch) = &first.content else {
            panic!("expected pitched first note");
        };
        let crate::model::note::FullNoteContent::Pitch(root_pitch) = &chord_root.content else {
            panic!("expected pitched chord root");
        };
        let crate::model::note::FullNoteContent::Pitch(top_pitch) = &chord_top.content else {
            panic!("expected pitched chord top");
        };

        assert_eq!(first_pitch.octave, 4);
        assert_eq!(root_pitch.octave, 5);
        assert_eq!(top_pitch.octave, 5);
    }

    #[test]
    fn test_insert_direction_near_start_orders_octave_start_before_stop() {
        use crate::model::elements::MeasureContent;

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(12.0);

        let mut content = vec![
            MeasureContent::Attributes(Box::default()),
            MeasureContent::Print(Box::new(crate::model::print::Print {
                page_layout: None,
                system_layout: None,
                staff_layouts: Vec::new(),
                measure_layout: None,
                measure_numbering: None,
                part_name_display: None,
                part_abbreviation_display: None,
                staff_spacing: None,
                new_system: None,
                new_page: None,
                blank_page: None,
                page_number: None,
                id: None,
            })),
        ];

        insert_direction_near_start(
            &mut content,
            make_octave_direction(
                1,
                6.0,
                crate::model::direction::OctaveShiftType::Stop,
                1,
                8,
                None,
                &ctx,
            ),
        );
        insert_direction_near_start(
            &mut content,
            make_octave_direction(
                1,
                5.0,
                crate::model::direction::OctaveShiftType::Down,
                1,
                8,
                Some("tusk-octave-1".to_string()),
                &ctx,
            ),
        );

        let MeasureContent::Direction(first_direction) = &content[2] else {
            panic!("expected first direction");
        };
        let MeasureContent::Direction(second_direction) = &content[3] else {
            panic!("expected second direction");
        };

        let first_shift = first_direction.direction_types.first().and_then(|direction_type| {
            match &direction_type.content {
                crate::model::direction::DirectionTypeContent::OctaveShift(shift) => Some(shift),
                _ => None,
            }
        });
        let second_shift = second_direction.direction_types.first().and_then(|direction_type| {
            match &direction_type.content {
                crate::model::direction::DirectionTypeContent::OctaveShift(shift) => Some(shift),
                _ => None,
            }
        });

        assert!(matches!(
            first_shift.map(|shift| shift.shift_type),
            Some(crate::model::direction::OctaveShiftType::Down)
        ));
        assert!(matches!(
            second_shift.map(|shift| shift.shift_type),
            Some(crate::model::direction::OctaveShiftType::Stop)
        ));
    }

    #[test]
    fn test_apply_octave_directions_prefers_endid_over_tstamp2_for_stop() {
        use crate::model::data::Step;
        use crate::model::elements::MeasureContent;
        use crate::model::note::{Note as MxmlNote, Pitch};
        use tusk_model::data::{
            DataBeat, DataMeasurebeat, DataOctaveDis, DataStaffrelBasic, DataUri,
        };
        use tusk_model::elements::Octave as MeiOctave;

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(8.0);
        ctx.register_part_staff("P1", 1, 1);

        let mut measures = vec![MxmlMeasure::new("1")];
        let mut written = MxmlNote::pitched(
            Pitch {
                step: Step::C,
                alter: None,
                octave: 5,
            },
            1.0,
        );
        written.staff = Some(1);
        written.id = Some("n-after".to_string());
        measures[0].content.push(MeasureContent::Note(Box::new(written)));

        let mut octave = MeiOctave::default();
        octave.octave_log.staff = Some("1".to_string());
        octave.octave_log.tstamp = Some(DataBeat::from(1.0));
        octave.octave_log.tstamp2 = Some(DataMeasurebeat::from("0m+4".to_string()));
        octave.octave_log.endid = Some(DataUri::from("#n-stop".to_string()));
        octave.octave_log.dis = Some(DataOctaveDis(8));
        octave.octave_log.dis_place = Some(DataStaffrelBasic::Above);

        let mei_measure = tusk_model::elements::Measure {
            children: vec![tusk_model::elements::MeasureChild::Octave(Box::new(octave))],
            ..Default::default()
        };

        let mut note_positions = HashMap::new();
        note_positions.insert(
            "n-stop".to_string(),
            MeiEventPosition {
                staff: 1,
                measure_idx: 0,
                beat: 2.0,
                duration_beats: 1.0,
            },
        );
        note_positions.insert(
            "n-after".to_string(),
            MeiEventPosition {
                staff: 1,
                measure_idx: 0,
                beat: 4.0,
                duration_beats: 1.0,
            },
        );

        let part_ids = vec!["P1".to_string()];
        let mut part_measures = vec![measures];
        apply_octave_directions_to_parts(
            &[&mei_measure],
            &note_positions,
            &part_ids,
            &mut part_measures,
            &ctx,
        );

        let directions: Vec<_> = part_measures[0][0]
            .content
            .iter()
            .filter_map(|content| match content {
                MeasureContent::Direction(direction) => Some(direction.as_ref()),
                _ => None,
            })
            .collect();
        assert_eq!(directions.len(), 2);

        let start = directions[0]
            .direction_types
            .first()
            .and_then(|direction_type| match &direction_type.content {
                crate::model::direction::DirectionTypeContent::OctaveShift(shift) => Some(shift),
                _ => None,
            })
            .expect("expected octave shift start");
        let stop = directions[1]
            .direction_types
            .first()
            .and_then(|direction_type| match &direction_type.content {
                crate::model::direction::DirectionTypeContent::OctaveShift(shift) => Some(shift),
                _ => None,
            })
            .expect("expected octave shift stop");

        assert!(matches!(
            start.shift_type,
            crate::model::direction::OctaveShiftType::Down
        ));
        assert!(matches!(
            stop.shift_type,
            crate::model::direction::OctaveShiftType::Stop
        ));
        assert_eq!(directions[0].offset.as_ref().map(|offset| offset.value), None);
        assert_eq!(
            directions[1].offset.as_ref().map(|offset| offset.value),
            Some(16.0)
        );
    }

}

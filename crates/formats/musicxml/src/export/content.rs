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
    LayerChild, MeasureChild, Score as MeiScore, ScoreChild, ScoreDefChild, Section, SectionChild,
    Staff, StaffChild, StaffGrp, StaffGrpChild,
};

use super::direction::{
    convert_mei_dir, convert_mei_dynam, convert_mei_hairpin, convert_mei_tempo,
};
use super::note::{convert_mei_chord, convert_mei_mrest, convert_mei_note, convert_mei_rest};
use super::structure::convert_mei_measure;
use super::utils::find_score_def;

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
                    let StaffChild::Layer(layer) = sc;
                    collect_note_ids_from_layer(&layer.children, m_idx, &mut note_to_measure);
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
                    let tusk_model::elements::ChordChild::Note(n) = note;
                    if let Some(ref id) = n.common.xml_id {
                        map.insert(id.clone(), measure_idx);
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
            LayerChild::DivLine(_) => {}
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
                    let tusk_model::elements::ChordChild::Note(n) = note;
                    if let Some(ref id) = n.common.xml_id {
                        map.insert(id.clone(), measure_idx);
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
                    let tusk_model::elements::ChordChild::Note(n) = note;
                    if let Some(ref id) = n.common.xml_id {
                        map.insert(id.clone(), measure_idx);
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
                    let tusk_model::elements::ChordChild::Note(n) = note;
                    if let Some(ref id) = n.common.xml_id {
                        map.insert(id.clone(), measure_idx);
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
    // Collect all MEI measures from sections
    let mei_measures = collect_measures_from_score(mei_score);

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
            implicit: mxml_measure_base.implicit,
            non_controlling: mxml_measure_base.non_controlling,
            width: mxml_measure_base.width,
        });

        // For each MusicXML part, extract its staff/staves content from the MEI measure.
        // Multi-staff parts (e.g., piano) have multiple MEI staves that must be merged
        // into a single MusicXML part with <backup> elements between staves.
        for (part_idx, part_id) in part_ids.iter().enumerate() {
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
                    convert_staff_content(staff, local_staff_n, &mut mxml_measure, ctx)?;
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
                // Multi-staff part: merge multiple staves with <backup> between them
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
                        convert_staff_content(staff, local_staff_n, &mut mxml_measure, ctx)?;
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

                    // Fermata, arpeg, gliss events
                    convert_fermata_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;
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

            // Store the measure for future cross-measure slur resolution
            part_prev_measures[part_idx].push(mxml_measure);
        }
    }

    // Build the timewise output from the accumulated measures.
    // At this point, all cross-measure slur notations have been retroactively
    // attached, so the measure content is complete.
    let mut timewise_measures = Vec::new();
    for (measure_idx, meta) in measure_metas.iter().enumerate() {
        let mut tw_measure = TimewiseMeasure {
            number: meta.number.clone(),
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

/// Collect all measures from an MEI score by traversing sections.
fn collect_measures_from_score(mei_score: &MeiScore) -> Vec<&tusk_model::elements::Measure> {
    let mut measures = Vec::new();

    for child in &mei_score.children {
        if let ScoreChild::Section(section) = child {
            collect_measures_from_section(section, &mut measures);
        }
    }

    measures
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
            _ => {}
        }
    }
}

/// Recursively collect measures from a section (sections can be nested).
fn collect_measures_from_section<'a>(
    section: &'a Section,
    measures: &mut Vec<&'a tusk_model::elements::Measure>,
) {
    for child in &section.children {
        match child {
            SectionChild::Measure(measure) => {
                measures.push(measure);
            }
            SectionChild::Section(nested_section) => {
                collect_measures_from_section(nested_section, measures);
            }
            _ => {
                // Other section children (ending, expansion, etc.) not handled yet
            }
        }
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
    use tusk_model::data::DataBarrendition;
    let bar_style = match rend {
        DataBarrendition::Single => BarStyle::Regular,
        DataBarrendition::Dotted => BarStyle::Dotted,
        DataBarrendition::Dashed => BarStyle::Dashed,
        DataBarrendition::Heavy => BarStyle::Heavy,
        DataBarrendition::Dbl => BarStyle::LightLight,
        DataBarrendition::Dblheavy => BarStyle::HeavyHeavy,
        DataBarrendition::Invis => BarStyle::None,
        DataBarrendition::Dbldashed => BarStyle::Dashed,
        DataBarrendition::Dbldotted => BarStyle::Dotted,
        _ => BarStyle::Regular,
    };
    Barline {
        location: Some(location),
        bar_style: Some(bar_style),
    }
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
    for child in &mei_measure.children {
        match child {
            MeasureChild::Dynam(dynam) => {
                // Skip notation-level dynamics — handled by convert_notation_dynamics()
                if dynam
                    .common
                    .label
                    .as_deref()
                    .is_some_and(|l| l == "musicxml:notation-dynamics")
                {
                    continue;
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

            // Find start and end positions in measure content
            let start_pos = mxml_measure.content.iter().position(
                |c| matches!(c, MeasureContent::Note(n) if n.id.as_deref() == Some(&sid)),
            );
            let end_pos = mxml_measure.content.iter().position(
                |c| matches!(c, MeasureContent::Note(n) if n.id.as_deref() == Some(&eid)),
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

/// Convert MEI ornament control events to MusicXML ornament notations on notes.
///
/// For each trill/mordent/turn/ornam control event, finds the referenced note
/// by startid and adds the appropriate ornament to its notations.
fn convert_ornament_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    _ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use super::direction::convert_place_to_placement;
    use crate::model::data::{StartStopContinue, TremoloType, YesNo};
    use crate::model::notations::{
        EmptyPlacement, EmptyTrillSound, HorizontalTurn, Mordent as MxmlMordent, Notations,
        Ornaments, OtherOrnament, Tremolo, WavyLine,
    };

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
                let label = ornam.common.label.as_deref().unwrap_or("");
                let placement = convert_place_to_placement(&ornam.ornam_vis.place);

                let Some(note) = find_note_by_id_mut(mxml_measure, &sid) else {
                    continue;
                };
                let notations = note.notations.get_or_insert_with(Notations::default);
                let ornaments = notations.ornaments.get_or_insert_with(Ornaments::default);

                if label == "musicxml:vertical-turn" {
                    ornaments.vertical_turn = Some(EmptyTrillSound {
                        placement,
                        ..Default::default()
                    });
                } else if label == "musicxml:inverted-vertical-turn" {
                    ornaments.inverted_vertical_turn = Some(EmptyTrillSound {
                        placement,
                        ..Default::default()
                    });
                } else if label == "musicxml:shake" {
                    ornaments.shake = Some(EmptyTrillSound {
                        placement,
                        ..Default::default()
                    });
                } else if label == "musicxml:schleifer" {
                    ornaments.schleifer = Some(EmptyPlacement {
                        placement,
                        ..Default::default()
                    });
                } else if label == "musicxml:haydn" {
                    ornaments.haydn = Some(EmptyTrillSound {
                        placement,
                        ..Default::default()
                    });
                } else if label.starts_with("musicxml:tremolo,") {
                    // Parse "musicxml:tremolo,type=<type>,value=<value>"
                    let mut ttype = TremoloType::Single;
                    let mut tvalue: Option<u8> = None;
                    for part in label.trim_start_matches("musicxml:tremolo,").split(',') {
                        if let Some(v) = part.strip_prefix("type=") {
                            ttype = match v {
                                "start" => TremoloType::Start,
                                "stop" => TremoloType::Stop,
                                "unmeasured" => TremoloType::Unmeasured,
                                _ => TremoloType::Single,
                            };
                        } else if let Some(v) = part.strip_prefix("value=") {
                            tvalue = v.parse().ok();
                        }
                    }
                    ornaments.tremolo = Some(Tremolo {
                        tremolo_type: ttype,
                        value: tvalue,
                        placement,
                        default_x: None,
                        default_y: None,
                        color: None,
                        smufl: None,
                    });
                } else if label.starts_with("musicxml:wavy-line,") {
                    // Parse "musicxml:wavy-line,type=<type>,number=<num>"
                    let mut wtype = StartStopContinue::Start;
                    let mut wnumber: Option<u8> = None;
                    for part in label.trim_start_matches("musicxml:wavy-line,").split(',') {
                        if let Some(v) = part.strip_prefix("type=") {
                            wtype = match v {
                                "stop" => StartStopContinue::Stop,
                                "continue" => StartStopContinue::Continue,
                                _ => StartStopContinue::Start,
                            };
                        } else if let Some(v) = part.strip_prefix("number=") {
                            wnumber = v.parse().ok();
                        }
                    }
                    ornaments.wavy_line = Some(WavyLine {
                        wavy_line_type: wtype,
                        number: wnumber,
                        placement,
                        default_x: None,
                        default_y: None,
                        color: None,
                        smufl: None,
                        trill_sound: Default::default(),
                    });
                } else if label == "musicxml:other-ornament" {
                    // Collect text content from ornam children
                    let text: String = ornam
                        .children
                        .iter()
                        .filter_map(|c| {
                            if let tusk_model::elements::OrnamChild::Text(t) = c {
                                Some(t.as_str())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("");
                    ornaments.other_ornament = Some(OtherOrnament {
                        value: text,
                        placement,
                    });
                } else if let Some(rest) = label.strip_prefix("musicxml:ornament-accidental-mark,")
                {
                    // Accidental-mark within ornaments
                    let mut value = String::new();
                    let mut acc_placement = None;
                    for part in rest.split(',') {
                        if let Some(v) = part.strip_prefix("value=") {
                            value = v.to_string();
                        } else if let Some(v) = part.strip_prefix("placement=") {
                            acc_placement = match v {
                                "above" => Some(crate::model::data::AboveBelow::Above),
                                "below" => Some(crate::model::data::AboveBelow::Below),
                                _ => None,
                            };
                        }
                    }
                    ornaments
                        .accidental_marks
                        .push(crate::model::notations::AccidentalMark {
                            value,
                            placement: acc_placement,
                        });
                } else if let Some(rest) = label.strip_prefix("musicxml:accidental-mark,") {
                    // Standalone accidental-mark → goes on notations (not ornaments)
                    let mut value = String::new();
                    for part in rest.split(',') {
                        if let Some(v) = part.strip_prefix("value=") {
                            value = v.to_string();
                        }
                    }
                    let notations = note.notations.get_or_insert_with(Notations::default);
                    notations
                        .accidental_marks
                        .push(crate::model::notations::AccidentalMark { value, placement });
                    // Skip adding to ornaments for accidental-mark
                    continue;
                }
            }
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

/// Convert MEI `<arpeg>` control events to MusicXML arpeggiate/non-arpeggiate notations.
fn convert_arpeg_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    _ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::data::{TopBottom, UpDown};
    use crate::model::notations::{Arpeggiate, NonArpeggiate, Notations};

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

        let is_nonarp = arpeg.common.label.as_deref() == Some("musicxml:non-arpeggiate");
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
    _ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use crate::model::data::LineType;
    use crate::model::notations::{Glissando, Slide};
    use tusk_model::data::DataLineform;

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

        let is_slide = gliss.common.label.as_deref() == Some("musicxml:slide");

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

/// Convert an MEI staff's content to MusicXML measure content.
///
/// The `staff_n` parameter is the 1-based staff number, used to set the
/// `<staff>` element on notes for multi-staff part roundtrip fidelity.
fn convert_staff_content(
    staff: &Staff,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    // Find all layers in the staff
    for child in &staff.children {
        let StaffChild::Layer(layer) = child;
        // Convert layer content (notes, rests, chords)
        for layer_child in &layer.children {
            match layer_child {
                LayerChild::Note(note) => {
                    let mut mxml_note = convert_mei_note(note, ctx)?;
                    mxml_note.staff = Some(staff_n as u32);
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(mxml_note)));
                }
                LayerChild::Rest(rest) => {
                    let mut mxml_note = convert_mei_rest(rest, ctx)?;
                    mxml_note.staff = Some(staff_n as u32);
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(mxml_note)));
                }
                LayerChild::Chord(chord) => {
                    let mxml_notes = convert_mei_chord(chord, ctx)?;
                    for mut note in mxml_notes {
                        note.staff = Some(staff_n as u32);
                        mxml_measure
                            .content
                            .push(MeasureContent::Note(Box::new(note)));
                    }
                }
                LayerChild::Beam(beam) => {
                    // Recursively process beam content
                    convert_beam_content(beam, staff_n, mxml_measure, ctx)?;
                }
                LayerChild::BTrem(btrem) => {
                    convert_btrem_content(btrem, staff_n, mxml_measure, ctx)?;
                }
                LayerChild::FTrem(ftrem) => {
                    convert_ftrem_content(ftrem, staff_n, mxml_measure, ctx)?;
                }
                LayerChild::MRest(mrest) => {
                    // Measure rest
                    let mut mxml_note = convert_mei_mrest(mrest, ctx)?;
                    mxml_note.staff = Some(staff_n as u32);
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(mxml_note)));
                }
                _ => {
                    // Other layer children (space, tuplet, etc.) not handled yet
                }
            }
        }
    }
    Ok(())
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
                let mut mxml_note = convert_mei_note(note, ctx)?;
                mxml_note.staff = Some(staff_n as u32);
                let idx = mxml_measure.content.len();
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
                events.push((idx, false));
            }
            BeamChild::Rest(rest) => {
                let mut mxml_note = convert_mei_rest(rest, ctx)?;
                mxml_note.staff = Some(staff_n as u32);
                let idx = mxml_measure.content.len();
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
                events.push((idx, true));
            }
            BeamChild::Chord(chord) => {
                let mxml_notes = convert_mei_chord(chord, ctx)?;
                let first_idx = mxml_measure.content.len();
                for mut note in mxml_notes {
                    note.staff = Some(staff_n as u32);
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
        }
    }
    Ok(events)
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
                let mut mxml_note = convert_mei_note(note, ctx)?;
                mxml_note.staff = Some(staff_n as u32);
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
                let mxml_notes = convert_mei_chord(chord, ctx)?;
                let mut first = true;
                for mut note in mxml_notes {
                    note.staff = Some(staff_n as u32);
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
                let mut mxml_note = convert_mei_note(note, ctx)?;
                mxml_note.staff = Some(staff_n as u32);
                add_tremolo_to_note(&mut mxml_note, ttype, marks);
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
            }
            FTremChild::Chord(chord) => {
                let mxml_notes = convert_mei_chord(chord, ctx)?;
                let mut first = true;
                for mut note in mxml_notes {
                    note.staff = Some(staff_n as u32);
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
                    let StaffChild::Layer(layer) = sc;
                    find_smallest_duration_in_layer(&layer.children, &mut smallest_duration);
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
            LayerChild::MRest(_) | LayerChild::DivLine(_) => {}
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
            _ => {}
        }
    }
    total.max(0.0)
}

// build_first_measure_attributes and build_first_measure_attributes_multi
// are in super::attributes to keep this module under the line limit.

/// Convert MEI `<ornam>` control events with `musicxml:` technical labels back to MusicXML
/// `<technical>` notations on the referenced notes.
fn convert_technical_events(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    _ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use super::direction::convert_place_to_placement;
    use crate::model::data::{StartStop, YesNo};
    use crate::model::notations::Notations;
    use crate::model::technical::*;

    for child in &mei_measure.children {
        let MeasureChild::Ornam(ornam) = child else {
            continue;
        };
        let label = match ornam.common.label.as_deref() {
            Some(l) if l.starts_with("musicxml:") => l,
            _ => continue,
        };
        // Extract the technical element name (after "musicxml:" prefix, before first comma)
        let after_prefix = &label["musicxml:".len()..];
        let element_name = after_prefix.split(',').next().unwrap_or("");

        // Only handle technical element names
        if !is_technical_label(element_name) {
            continue;
        }

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

        // Collect text from ornam children
        let text: String = ornam
            .children
            .iter()
            .map(|c| {
                let tusk_model::elements::OrnamChild::Text(t) = c;
                t.as_str()
            })
            .collect::<Vec<_>>()
            .join("");

        // Parse comma-separated key=value params from label
        let params: Vec<&str> = after_prefix.split(',').skip(1).collect();

        let find_param = |key: &str| -> Option<&str> {
            params
                .iter()
                .find_map(|p| p.strip_prefix(key).and_then(|rest| rest.strip_prefix('=')))
        };
        let has_flag = |flag: &str| -> bool { params.contains(&flag) };

        match element_name {
            // Simple empty-placement types
            "up-bow" => tech.up_bow.push(ep()),
            "down-bow" => tech.down_bow.push(ep()),
            "open-string" => tech.open_string.push(ep()),
            "thumb-position" => tech.thumb_position.push(ep()),
            "double-tongue" => tech.double_tongue.push(ep()),
            "triple-tongue" => tech.triple_tongue.push(ep()),
            "snap-pizzicato" => tech.snap_pizzicato.push(ep()),
            "fingernails" => tech.fingernails.push(ep()),
            "brass-bend" => tech.brass_bend.push(ep()),
            "flip" => tech.flip.push(ep()),
            "smear" => tech.smear.push(ep()),
            "golpe" => tech.golpe.push(ep()),

            // Empty-placement-smufl types
            "stopped" => tech.stopped.push(EmptyPlacementSmufl {
                placement,
                smufl: find_param("smufl").map(|s| s.to_string()),
                ..Default::default()
            }),
            "open" => tech.open.push(EmptyPlacementSmufl {
                placement,
                smufl: find_param("smufl").map(|s| s.to_string()),
                ..Default::default()
            }),
            "half-muted" => tech.half_muted.push(EmptyPlacementSmufl {
                placement,
                smufl: find_param("smufl").map(|s| s.to_string()),
                ..Default::default()
            }),

            // Text-content types
            "fingering" => tech.fingering.push(Fingering {
                value: text,
                substitution: if has_flag("substitution=yes") {
                    Some(YesNo::Yes)
                } else {
                    None
                },
                alternate: if has_flag("alternate=yes") {
                    Some(YesNo::Yes)
                } else {
                    None
                },
                placement,
                default_x: None,
                default_y: None,
                color: None,
            }),
            "pluck" => tech.pluck.push(PlacementText {
                value: text,
                placement,
                default_x: None,
                default_y: None,
                font_style: None,
                color: None,
            }),
            "fret" => tech.fret.push(Fret {
                value: text.parse().unwrap_or(0),
                color: None,
            }),
            "string" => tech.string.push(TechString {
                value: text.parse().unwrap_or(1),
                placement,
                default_x: None,
                default_y: None,
                color: None,
            }),
            "hammer-on" => {
                let ho_type = match find_param("type") {
                    Some("stop") => StartStop::Stop,
                    _ => StartStop::Start,
                };
                let number = find_param("number").and_then(|n| n.parse().ok());
                tech.hammer_on.push(HammerOnPullOff {
                    ho_type,
                    number,
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                    text,
                });
            }
            "pull-off" => {
                let ho_type = match find_param("type") {
                    Some("stop") => StartStop::Stop,
                    _ => StartStop::Start,
                };
                let number = find_param("number").and_then(|n| n.parse().ok());
                tech.pull_off.push(HammerOnPullOff {
                    ho_type,
                    number,
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                    text,
                });
            }
            "tap" => {
                let hand = find_param("hand").and_then(|h| match h {
                    "left" => Some(TapHand::Left),
                    "right" => Some(TapHand::Right),
                    _ => None,
                });
                tech.tap.push(Tap {
                    value: text,
                    hand,
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }
            "heel" => tech.heel.push(HeelToe {
                substitution: if has_flag("substitution=yes") {
                    Some(YesNo::Yes)
                } else {
                    None
                },
                placement,
                ..Default::default()
            }),
            "toe" => tech.toe.push(HeelToe {
                substitution: if has_flag("substitution=yes") {
                    Some(YesNo::Yes)
                } else {
                    None
                },
                placement,
                ..Default::default()
            }),
            "handbell" => tech.handbell.push(Handbell {
                value: text,
                placement,
                default_x: None,
                default_y: None,
                color: None,
            }),

            // Bend
            "bend" => {
                let alter: f64 = find_param("alter")
                    .and_then(|a| a.parse().ok())
                    .unwrap_or(0.0);
                let pre_bend = if has_flag("pre-bend") {
                    Some(true)
                } else {
                    None
                };
                let release = params.iter().find_map(|p| {
                    if *p == "release" {
                        Some(BendRelease { offset: None })
                    } else {
                        p.strip_prefix("release=").map(|offset_str| BendRelease {
                            offset: offset_str.parse().ok(),
                        })
                    }
                });
                let shape = find_param("shape").and_then(|s| match s {
                    "straight" => Some(BendShape::Straight),
                    "curved" => Some(BendShape::Curved),
                    _ => None,
                });
                let with_bar = if !text.is_empty() {
                    Some(PlacementText {
                        value: text,
                        placement: None,
                        default_x: None,
                        default_y: None,
                        font_style: None,
                        color: None,
                    })
                } else {
                    None
                };
                tech.bend.push(Bend {
                    bend_alter: alter,
                    pre_bend,
                    release,
                    with_bar,
                    shape,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }

            // Hole
            "hole" => {
                let closed_val = match find_param("closed") {
                    Some("no") => HoleClosedValue::No,
                    Some("half") => HoleClosedValue::Half,
                    _ => HoleClosedValue::Yes,
                };
                let location = find_param("location").and_then(|l| match l {
                    "right" => Some(HoleClosedLocation::Right),
                    "bottom" => Some(HoleClosedLocation::Bottom),
                    "left" => Some(HoleClosedLocation::Left),
                    "top" => Some(HoleClosedLocation::Top),
                    _ => None,
                });
                tech.hole.push(Hole {
                    hole_type: find_param("hole-type").map(|s| s.to_string()),
                    hole_closed: HoleClosed {
                        value: closed_val,
                        location,
                    },
                    hole_shape: find_param("hole-shape").map(|s| s.to_string()),
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }

            // Arrow
            "arrow" => {
                if let Some(dir) = find_param("direction") {
                    let style = find_param("style").map(|s| s.to_string());
                    let arrowhead = has_flag("arrowhead");
                    tech.arrow.push(Arrow {
                        content: ArrowContent::Directional {
                            direction: dir.to_string(),
                            style,
                            arrowhead,
                        },
                        placement,
                        default_x: None,
                        default_y: None,
                        color: None,
                        smufl: None,
                    });
                } else if let Some(circ) = find_param("circular") {
                    tech.arrow.push(Arrow {
                        content: ArrowContent::Circular(circ.to_string()),
                        placement,
                        default_x: None,
                        default_y: None,
                        color: None,
                        smufl: None,
                    });
                }
            }

            // Harmon mute
            "harmon-mute" => {
                let closed_val = match find_param("closed") {
                    Some("no") => HarmonClosedValue::No,
                    Some("half") => HarmonClosedValue::Half,
                    _ => HarmonClosedValue::Yes,
                };
                let location = find_param("location").and_then(|l| match l {
                    "right" => Some(HarmonClosedLocation::Right),
                    "bottom" => Some(HarmonClosedLocation::Bottom),
                    "left" => Some(HarmonClosedLocation::Left),
                    "top" => Some(HarmonClosedLocation::Top),
                    _ => None,
                });
                tech.harmon_mute.push(HarmonMute {
                    harmon_closed: HarmonClosed {
                        value: closed_val,
                        location,
                    },
                    placement,
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }

            // Harmonic
            "harmonic" => {
                tech.harmonic.push(Harmonic {
                    natural: if has_flag("natural") {
                        Some(true)
                    } else {
                        None
                    },
                    artificial: if has_flag("artificial") {
                        Some(true)
                    } else {
                        None
                    },
                    base_pitch: if has_flag("base-pitch") {
                        Some(true)
                    } else {
                        None
                    },
                    touching_pitch: if has_flag("touching-pitch") {
                        Some(true)
                    } else {
                        None
                    },
                    sounding_pitch: if has_flag("sounding-pitch") {
                        Some(true)
                    } else {
                        None
                    },
                    placement,
                    ..Default::default()
                });
            }

            // Other-technical
            "other-technical" => {
                tech.other_technical.push(OtherTechnical {
                    value: text,
                    placement,
                    smufl: find_param("smufl").map(|s| s.to_string()),
                    default_x: None,
                    default_y: None,
                    color: None,
                });
            }

            _ => {}
        }
    }
    Ok(())
}

/// Check if an element name corresponds to a MusicXML technical notation.
fn is_technical_label(name: &str) -> bool {
    matches!(
        name,
        "up-bow"
            | "down-bow"
            | "open-string"
            | "thumb-position"
            | "double-tongue"
            | "triple-tongue"
            | "snap-pizzicato"
            | "fingernails"
            | "brass-bend"
            | "flip"
            | "smear"
            | "golpe"
            | "stopped"
            | "open"
            | "half-muted"
            | "fingering"
            | "pluck"
            | "fret"
            | "string"
            | "hammer-on"
            | "pull-off"
            | "tap"
            | "heel"
            | "toe"
            | "handbell"
            | "bend"
            | "hole"
            | "arrow"
            | "harmon-mute"
            | "harmonic"
            | "other-technical"
    )
}

/// Convert MEI `<dynam>` control events with `musicxml:notation-dynamics` label back to
/// MusicXML `<dynamics>` within `<notations>` on the referenced notes.
fn convert_notation_dynamics(
    mei_measure: &tusk_model::elements::Measure,
    staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
    _ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use super::direction::{convert_place_to_placement, parse_dynamics_text};
    use crate::model::direction::Dynamics;
    use crate::model::notations::Notations;
    use tusk_model::elements::DynamChild;

    for child in &mei_measure.children {
        let MeasureChild::Dynam(dynam) = child else {
            continue;
        };
        // Only process notation-level dynamics
        if dynam
            .common
            .label
            .as_deref()
            .is_none_or(|l| l != "musicxml:notation-dynamics")
        {
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
            .map(|child| {
                let DynamChild::Text(t) = child;
                t.as_str()
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
        let measures = collect_measures_from_score(&score);
        assert_eq!(measures.len(), 1);
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
}

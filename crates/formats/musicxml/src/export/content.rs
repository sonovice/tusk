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
    Measure as MxmlMeasure, MeasureContent, TimewiseMeasure, TimewisePart,
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
            LayerChild::MRest(mrest) => {
                if let Some(ref id) = mrest.common.xml_id {
                    map.insert(id.clone(), measure_idx);
                }
            }
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
            _ => {}
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
    let divs = initial_divs.unwrap_or_else(|| {
        calculate_smart_divisions(&mei_measures)
    });
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

        // For each part/staff, extract that staff's content
        for (staff_idx, _part_id) in part_ids.iter().enumerate() {
            let global_staff_n = staff_idx + 1; // MEI staff numbers are 1-based global
            let local_staff_n = 1_usize; // MusicXML: part-local staff number

            // Set per-part divisions from the staffDef so that direction offset
            // calculations use the correct value for this part.
            if let Some(staff_def) = staff_defs.get(staff_idx) {
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

            // Convert direction events BEFORE notes
            convert_direction_events(mei_measure, global_staff_n, &mut mxml_measure, ctx)?;

            // Find the staff with matching global @n in this MEI measure
            if let Some(staff) = find_staff_in_measure(mei_measure, global_staff_n) {
                // Convert staff content with part-LOCAL staff number
                convert_staff_content(staff, local_staff_n, &mut mxml_measure, ctx)?;
            }

            // Convert slur events AFTER notes (need note IDs to attach notations).
            // This may retroactively modify notes in part_prev_measures to attach
            // cross-measure slur start notations.
            convert_slur_events(
                mei_measure,
                global_staff_n,
                &mut mxml_measure,
                &mut part_prev_measures[staff_idx],
                ctx,
            )?;

            // Add attributes to first measure of each part
            if measure_idx == 0 {
                let attrs = build_first_measure_attributes(
                    score_def,
                    staff_defs.get(staff_idx).copied(),
                    ctx,
                );
                mxml_measure
                    .content
                    .insert(0, MeasureContent::Attributes(Box::new(attrs)));
            }

            // Store the measure for future cross-measure slur resolution
            part_prev_measures[staff_idx].push(mxml_measure);
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
                if n as usize == staff_n {
                    return Some(staff);
                }
            }
        }
    }
    None
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
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    for child in &mei_measure.children {
        match child {
            MeasureChild::Dynam(dynam) => {
                let event_staff = dynam
                    .dynam_log
                    .staff
                    .as_ref()
                    .and_then(|s| s.split_whitespace().next())
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1) as usize;
                if event_staff == staff_n
                    && let Some(direction) = convert_mei_dynam(dynam, ctx)
                {
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
                    for direction in convert_mei_hairpin(hairpin, ctx) {
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
                    && let Some(direction) = convert_mei_dir(dir, ctx)
                {
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
                    && let Some(direction) = convert_mei_tempo(tempo, ctx)
                {
                    mxml_measure
                        .content
                        .push(MeasureContent::Direction(Box::new(direction)));
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
        if let StaffChild::Layer(layer) = child {
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
            _ => {
                // Other beam children not handled yet
            }
        }
    }
    Ok(events)
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
            _ => {}
        }
    }
}

/// Build MusicXML Attributes for the first measure by merging scoreDef and staffDef.
///
/// - scoreDef provides: key signature, time signature (global)
/// - staffDef provides: clef, transposition, staff lines (per-staff)
fn build_first_measure_attributes(
    score_def: Option<&tusk_model::elements::ScoreDef>,
    staff_def: Option<&tusk_model::elements::StaffDef>,
    ctx: &mut ConversionContext,
) -> crate::model::attributes::Attributes {
    use super::attributes::{
        convert_mei_clef_shape_to_mxml, convert_mei_keysig_to_fifths, convert_mei_meter_sym_to_mxml,
    };
    use crate::model::attributes::{
        Attributes, Clef, Key, KeyContent, SenzaMisura, StaffDetails, StandardTime, Time,
        TimeContent, TimeSignature, TraditionalKey, Transpose,
    };

    let mut attrs = Attributes::default();

    // Set divisions from context
    let divisions = ctx.divisions();
    attrs.divisions = Some(divisions);

    // Get key signature from scoreDef first, then staffDef as fallback (@keysig is Option<String>)
    let keysig = score_def
        .and_then(|sd| sd.score_def_log.keysig.as_ref())
        .or_else(|| staff_def.and_then(|sd| sd.staff_def_log.keysig.as_ref()));

    if let Some(keysig) = keysig
        && let Some(fifths) = convert_mei_keysig_to_fifths(keysig.0.as_str())
    {
        attrs.keys.push(Key {
            number: None,
            print_object: None,
            id: None,
            content: KeyContent::Traditional(TraditionalKey {
                cancel: None,
                fifths,
                mode: None,
            }),
            key_octaves: Vec::new(),
        });
    }

    // Get time signature from scoreDef first, then staffDef as fallback
    let meter_sym = score_def
        .and_then(|sd| sd.score_def_log.meter_sym.as_ref())
        .or_else(|| staff_def.and_then(|sd| sd.staff_def_log.meter_sym.as_ref()));
    let meter_count = score_def
        .and_then(|sd| sd.score_def_log.meter_count.as_ref())
        .or_else(|| staff_def.and_then(|sd| sd.staff_def_log.meter_count.as_ref()));
    let meter_unit = score_def
        .and_then(|sd| sd.score_def_log.meter_unit.as_ref().cloned())
        .or_else(|| staff_def.and_then(|sd| sd.staff_def_log.meter_unit.as_ref().cloned()));

    if meter_sym.as_deref() == Some(&tusk_model::data::DataMetersign::Open) {
        // Senza misura
        attrs.times.push(Time {
            number: None,
            symbol: None,
            separator: None,
            print_object: None,
            id: None,
            content: TimeContent::SenzaMisura(SenzaMisura { symbol: None }),
        });
    } else if meter_count.is_some() || meter_unit.is_some() {
        let beats = meter_count
            .map(|s| s.to_string())
            .unwrap_or_else(|| "4".to_string());
        let beat_type = meter_unit.unwrap_or_else(|| "4".to_string());

        let symbol = meter_sym.as_ref().and_then(|s| convert_mei_meter_sym_to_mxml(s));

        attrs.times.push(Time {
            number: None,
            symbol,
            separator: None,
            print_object: None,
            id: None,
            content: TimeContent::Standard(StandardTime {
                signatures: vec![TimeSignature { beats, beat_type }],
                interchangeable: None,
            }),
        });
    }

    // Get clef from staffDef (per-staff attribute)
    if let Some(staff_def) = staff_def {
        if let Some(shape) = &staff_def.staff_def_log.clef_shape {
            let sign = convert_mei_clef_shape_to_mxml(shape);
            let line = staff_def
                .staff_def_log
                .clef_line
                .as_ref()
                .map(|c| c.0 as u32);

            // Convert octave displacement
            let octave_change = super::attributes::convert_mei_clef_dis_to_octave_change(
                staff_def.staff_def_log.clef_dis.as_ref(),
                staff_def.staff_def_log.clef_dis_place.as_ref(),
            );

            attrs.clefs.push(Clef {
                number: None,
                additional: None,
                size: None,
                after_barline: None,
                print_object: None,
                id: None,
                sign,
                line,
                clef_octave_change: octave_change,
            });
        }

        // Get transposition from staffDef (MEI uses Option<String>)
        if staff_def.staff_def_log.trans_diat.is_some()
            || staff_def.staff_def_log.trans_semi.is_some()
        {
            let chromatic = staff_def
                .staff_def_log
                .trans_semi
                .as_ref()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0) as f64;
            let diatonic = staff_def
                .staff_def_log
                .trans_diat
                .as_ref()
                .and_then(|s| s.parse().ok())
                .map(|d: i32| d);

            attrs.transposes.push(Transpose {
                number: None,
                id: None,
                diatonic,
                chromatic,
                octave_change: None,
                double: None,
            });
        }

        // Get staff lines from staffDef (MEI uses Option<String>)
        if let Some(lines) = staff_def
            .staff_def_log
            .lines
            .as_ref()
            .and_then(|s| s.parse::<u64>().ok())
        {
            attrs.staff_details.push(StaffDetails {
                number: None,
                show_frets: None,
                print_object: None,
                print_spacing: None,
                staff_type: None,
                staff_lines: Some(lines as u32), // lines is u64 from parse
                line_details: Vec::new(),
                staff_tunings: Vec::new(),
                capo: None,
                staff_size: None,
            });
        }
    }

    attrs
}

/// Convert MEI octave displacement (clef.dis + clef.dis.place) to MusicXML octave-change.
fn convert_clef_dis_to_octave_change(
    dis: Option<&tusk_model::data::DataOctaveDis>,
    dis_place: Option<&tusk_model::data::DataStaffrelBasic>,
) -> Option<i32> {
    use tusk_model::data::DataStaffrelBasic;

    let dis_value = dis?;
    let octaves = match dis_value.0 {
        8 => 1,
        15 => 2,
        22 => 3,
        _ => return None,
    };

    let direction = dis_place.map_or(1, |place| match place {
        DataStaffrelBasic::Above => 1,
        DataStaffrelBasic::Below => -1,
    });

    Some(octaves * direction)
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
        let mut score = MeiScore::default();

        // Create section with one measure
        let mut section = Section::default();
        let mut measure = MeiMeasure::default();
        measure.common.n = Some("1".to_string());

        // Create staff with layer containing a note (MEI uses Option<String> for @n)
        let mut staff = Staff::default();
        staff.n_integer.n = Some("1".to_string());

        let mut layer = Layer::default();
        let mut note = MeiNote::default();
        note.note_log.pname = Some("c".to_string());
        note.note_log.oct = Some("4".to_string());
        note.note_log.dur = Some("4".to_string());

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
        assert!(tw_measures[0].parts[0].content.len() >= 1);
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
}

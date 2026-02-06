//! MEI music content to MusicXML conversion.
//!
//! Handles traversal of MEI score/section/measure structure and conversion
//! to MusicXML part/measure format.
//!
//! MEI structure: Score → Section → Measure → Staff(@n) → Layer → Note/Rest/Chord
//! MusicXML structure: Part → Measure → Note/Rest (forward/backup for voices)

use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;
use crate::model::elements::{Measure as MxmlMeasure, MeasureContent, Part};
use tusk_model::elements::{
    LayerChild, MeasureChild, Score as MeiScore, ScoreChild, ScoreDefChild, Section, SectionChild,
    Staff, StaffChild, StaffGrp, StaffGrpChild,
};

use super::attributes::convert_staff_def_to_attributes;
use super::direction::{
    convert_mei_dir, convert_mei_dynam, convert_mei_hairpin, convert_mei_tempo,
};
use super::note::{convert_mei_chord, convert_mei_mrest, convert_mei_note, convert_mei_rest};
use super::structure::convert_mei_measure;
use super::utils::find_score_def;

/// Convert MEI score content to MusicXML parts.
///
/// This collects all measures from MEI sections and reorganizes them into
/// MusicXML part-oriented structure. MEI stores staff content within measures,
/// while MusicXML stores measures within parts.
pub fn convert_score_content(
    mei_score: &MeiScore,
    part_ids: &[String],
    ctx: &mut ConversionContext,
) -> ConversionResult<Vec<Part>> {
    // Collect all MEI measures from sections
    let mei_measures = collect_measures_from_score(mei_score);

    // Collect staffDefs from scoreDef for initial attributes
    let staff_defs = collect_staff_defs_from_score(mei_score);

    // Create a part for each part ID
    let mut parts: Vec<Part> = part_ids.iter().map(|id| Part::new(id)).collect();

    // Set divisions from first staffDef's ppq before processing measures,
    // so that direction offset calculations have the correct value.
    if let Some(staff_def) = staff_defs.first() {
        let divs = staff_def
            .staff_def_ges
            .ppq
            .map(|ppq| ppq as f64)
            .unwrap_or_else(|| {
                let ctx_divs = ctx.divisions();
                if ctx_divs > 0.0 { ctx_divs } else { 1.0 }
            });
        ctx.set_divisions(divs);
    }

    // For each MEI measure, extract staff content and add to corresponding part
    for (measure_idx, mei_measure) in mei_measures.iter().enumerate() {
        // Convert measure attributes
        let mxml_measure_base = convert_mei_measure(mei_measure, "", ctx)?;

        // For each part/staff, extract that staff's content from the measure
        for (staff_idx, part) in parts.iter_mut().enumerate() {
            let staff_n = staff_idx + 1; // Staff numbers are 1-based

            // Create a new measure for this part
            let mut mxml_measure = MxmlMeasure {
                number: mxml_measure_base.number.clone(),
                id: mxml_measure_base.id.clone(),
                implicit: mxml_measure_base.implicit,
                non_controlling: mxml_measure_base.non_controlling,
                width: mxml_measure_base.width,
                content: vec![],
            };

            // Convert direction events BEFORE notes so that on reimport,
            // beat_position=0 and the offset-based tstamp calculation is correct.
            convert_direction_events(mei_measure, staff_n, &mut mxml_measure, ctx)?;

            // Find the staff with matching @n in this MEI measure
            if let Some(staff) = find_staff_in_measure(mei_measure, staff_n) {
                // Convert the staff's layer content to MusicXML
                convert_staff_content(staff, staff_n, &mut mxml_measure, ctx)?;
            }

            // Convert slur events AFTER notes (need note IDs to attach notations)
            convert_slur_events(mei_measure, staff_n, &mut mxml_measure)?;

            // Add attributes to first measure of each part
            if measure_idx == 0 {
                // Get the staffDef for this staff number and convert to attributes
                let attrs = if let Some(staff_def) = staff_defs.get(staff_idx) {
                    let mut attrs = convert_staff_def_to_attributes(staff_def, ctx);
                    // Get divisions from staffDef ppq attribute, or use context default
                    let divs = staff_def
                        .staff_def_ges
                        .ppq
                        .map(|ppq| ppq as f64)
                        .unwrap_or_else(|| {
                            let ctx_divs = ctx.divisions();
                            if ctx_divs > 0.0 { ctx_divs } else { 1.0 }
                        });
                    attrs.divisions = Some(divs);
                    // Also set the context divisions for note duration calculations
                    ctx.set_divisions(divs);
                    attrs
                } else {
                    create_initial_attributes(ctx)
                };
                mxml_measure
                    .content
                    .insert(0, MeasureContent::Attributes(Box::new(attrs)));
            }

            part.measures.push(mxml_measure);
        }
    }

    Ok(parts)
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

    // Sort by @n attribute to ensure correct order
    staff_defs.sort_by_key(|sd| sd.n_integer.n.unwrap_or(0));

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
            // Check if this staff has the matching @n
            if let Some(n) = staff.n_integer.n {
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
                let event_staff = dynam.dynam_log.staff.first().copied().unwrap_or(1) as usize;
                if event_staff == staff_n
                    && let Some(direction) = convert_mei_dynam(dynam, ctx)
                {
                    mxml_measure
                        .content
                        .push(MeasureContent::Direction(Box::new(direction)));
                }
            }
            MeasureChild::Hairpin(hairpin) => {
                let event_staff = hairpin.hairpin_log.staff.first().copied().unwrap_or(1) as usize;
                if event_staff == staff_n {
                    for direction in convert_mei_hairpin(hairpin, ctx) {
                        mxml_measure
                            .content
                            .push(MeasureContent::Direction(Box::new(direction)));
                    }
                }
            }
            MeasureChild::Dir(dir) => {
                let event_staff = dir.dir_log.staff.first().copied().unwrap_or(1) as usize;
                if event_staff == staff_n
                    && let Some(direction) = convert_mei_dir(dir, ctx)
                {
                    mxml_measure
                        .content
                        .push(MeasureContent::Direction(Box::new(direction)));
                }
            }
            MeasureChild::Tempo(tempo) => {
                let event_staff = tempo.tempo_log.staff.first().copied().unwrap_or(1) as usize;
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
fn convert_slur_events(
    mei_measure: &tusk_model::elements::Measure,
    _staff_n: usize,
    mxml_measure: &mut MxmlMeasure,
) -> ConversionResult<()> {
    for child in &mei_measure.children {
        if let MeasureChild::Slur(slur) = child {
            // Try to attach slur notations to notes in this part's measure.
            // If the referenced notes aren't in this measure, they belong to
            // another part and will be handled when that part is processed.
            convert_mei_slur_to_notations(slur, mxml_measure);
        }
    }
    Ok(())
}

/// Convert an MEI slur control event to MusicXML slur notations on the referenced notes.
///
/// MEI slurs use `@startid`/`@endid` to reference the notes they connect.
/// MusicXML slurs are `<notations><slur>` elements on individual notes.
fn convert_mei_slur_to_notations(
    slur: &tusk_model::elements::Slur,
    mxml_measure: &mut MxmlMeasure,
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

    // Add slur start notation to the start note
    if let Some(ref sid) = start_id
        && let Some(note) = find_note_by_id_mut(mxml_measure, sid)
    {
        let mut mxml_slur = MxmlSlur::new(StartStopContinue::Start);
        mxml_slur.number = Some(1);
        let notations = note.notations.get_or_insert_with(Notations::default);
        notations.slurs.push(mxml_slur);
    }

    // Add slur stop notation to the end note
    if let Some(ref eid) = end_id
        && let Some(note) = find_note_by_id_mut(mxml_measure, eid)
    {
        let mut mxml_slur = MxmlSlur::new(StartStopContinue::Stop);
        mxml_slur.number = Some(1);
        let notations = note.notations.get_or_insert_with(Notations::default);
        notations.slurs.push(mxml_slur);
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
fn create_initial_attributes(ctx: &ConversionContext) -> crate::model::attributes::Attributes {
    let mut attrs = crate::model::attributes::Attributes::default();

    // Set divisions (number of units per quarter note)
    let divisions = ctx.divisions();
    if divisions > 0.0 {
        attrs.divisions = Some(divisions);
    } else {
        // Default divisions if not set
        attrs.divisions = Some(1.0);
    }

    attrs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname, DataWord};
    use tusk_model::elements::{
        Layer, LayerChild, Measure as MeiMeasure, MeasureChild, Note as MeiNote, Score as MeiScore,
        ScoreChild, Section, SectionChild, Staff, StaffChild,
    };

    fn create_simple_mei_score() -> MeiScore {
        let mut score = MeiScore::default();

        // Create section with one measure
        let mut section = Section::default();
        let mut measure = MeiMeasure::default();
        measure.common.n = Some(DataWord::from("1".to_string()));

        // Create staff with layer containing a note
        let mut staff = Staff::default();
        staff.n_integer.n = Some(1);

        let mut layer = Layer::default();
        let mut note = MeiNote::default();
        note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note.note_log.oct = Some(DataOctave::from(4u64));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

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
    fn test_convert_score_content_single_part() {
        let score = create_simple_mei_score();
        let part_ids = vec!["P1".to_string()];
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_score_content(&score, &part_ids, &mut ctx);
        assert!(result.is_ok());

        let parts = result.unwrap();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].id, "P1");
        assert_eq!(parts[0].measures.len(), 1);
        assert_eq!(parts[0].measures[0].number, "1");
        // Should have attributes + note
        assert!(parts[0].measures[0].content.len() >= 1);
    }

    #[test]
    fn test_find_staff_in_measure() {
        let mut measure = MeiMeasure::default();
        let mut staff1 = Staff::default();
        staff1.n_integer.n = Some(1);
        let mut staff2 = Staff::default();
        staff2.n_integer.n = Some(2);

        measure.children.push(MeasureChild::Staff(Box::new(staff1)));
        measure.children.push(MeasureChild::Staff(Box::new(staff2)));

        assert!(find_staff_in_measure(&measure, 1).is_some());
        assert!(find_staff_in_measure(&measure, 2).is_some());
        assert!(find_staff_in_measure(&measure, 3).is_none());
    }
}

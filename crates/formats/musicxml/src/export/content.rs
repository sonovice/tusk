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

            // Find the staff with matching @n in this MEI measure
            if let Some(staff) = find_staff_in_measure(mei_measure, staff_n) {
                // Convert the staff's layer content to MusicXML
                convert_staff_content(staff, &mut mxml_measure, ctx)?;
            }

            // Convert control events targeting this staff
            convert_control_events(mei_measure, staff_n, &mut mxml_measure, ctx)?;

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

/// Convert MEI control events (dynam, hairpin, dir, tempo) to MusicXML directions.
///
/// Control events in MEI are children of `<measure>`, not `<staff>`. Each control event
/// has a `@staff` attribute indicating which staff/part it belongs to.
/// Events without `@staff` default to staff 1.
fn convert_control_events(
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

/// Convert an MEI staff's content to MusicXML measure content.
fn convert_staff_content(
    staff: &Staff,
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
                        let mxml_note = convert_mei_note(note, ctx)?;
                        mxml_measure
                            .content
                            .push(MeasureContent::Note(Box::new(mxml_note)));
                    }
                    LayerChild::Rest(rest) => {
                        let mxml_note = convert_mei_rest(rest, ctx)?;
                        mxml_measure
                            .content
                            .push(MeasureContent::Note(Box::new(mxml_note)));
                    }
                    LayerChild::Chord(chord) => {
                        let mxml_notes = convert_mei_chord(chord, ctx)?;
                        for note in mxml_notes {
                            mxml_measure
                                .content
                                .push(MeasureContent::Note(Box::new(note)));
                        }
                    }
                    LayerChild::Beam(beam) => {
                        // Recursively process beam content
                        convert_beam_content(beam, mxml_measure, ctx)?;
                    }
                    LayerChild::MRest(mrest) => {
                        // Measure rest
                        let mxml_note = convert_mei_mrest(mrest, ctx)?;
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
fn convert_beam_content(
    beam: &tusk_model::elements::Beam,
    mxml_measure: &mut MxmlMeasure,
    ctx: &mut ConversionContext,
) -> ConversionResult<()> {
    use tusk_model::elements::BeamChild;

    for child in &beam.children {
        match child {
            BeamChild::Note(note) => {
                let mxml_note = convert_mei_note(note, ctx)?;
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
            }
            BeamChild::Rest(rest) => {
                let mxml_note = convert_mei_rest(rest, ctx)?;
                mxml_measure
                    .content
                    .push(MeasureContent::Note(Box::new(mxml_note)));
            }
            BeamChild::Chord(chord) => {
                let mxml_notes = convert_mei_chord(chord, ctx)?;
                for note in mxml_notes {
                    mxml_measure
                        .content
                        .push(MeasureContent::Note(Box::new(note)));
                }
            }
            BeamChild::Beam(nested_beam) => {
                // Recursively process nested beams
                convert_beam_content(nested_beam, mxml_measure, ctx)?;
            }
            _ => {
                // Other beam children not handled yet
            }
        }
    }
    Ok(())
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

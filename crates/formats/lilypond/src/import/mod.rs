//! Conversion from LilyPond AST to MEI.

use thiserror::Error;
use tusk_model::elements::{
    Accid, Body, BodyChild, FileDesc, FileDescChild, Layer, LayerChild, MRest, Mdiv, MdivChild,
    Measure, MeasureChild, Mei, MeiChild, MeiHead, MeiHeadChild, Note, NoteChild, Rest, Score,
    ScoreChild, ScoreDef, ScoreDefChild, Section, SectionChild, Staff, StaffChild, StaffDef,
    StaffGrp, StaffGrpChild, TitleStmt,
};
use tusk_model::generated::data::{
    DataAccidentalGestural, DataAccidentalGesturalBasic, DataAccidentalWritten,
    DataAccidentalWrittenBasic, DataAugmentdot, DataDuration, DataDurationCmn, DataDurationrests,
    DataOctave, DataPitchname,
};

use crate::model::{
    self, Duration, LilyPondFile, Music, NoteEvent, RestEvent, ScoreItem, ToplevelExpression,
};

#[derive(Debug, Error)]
pub enum ImportError {
    #[error("LilyPond import is not yet implemented")]
    NotImplemented,
    #[error("no music found in LilyPond file")]
    NoMusic,
    #[error("import error: {0}")]
    Other(String),
}

/// Convert a parsed LilyPond AST to an MEI document.
pub fn import(file: &LilyPondFile) -> Result<Mei, ImportError> {
    let music = find_music(file).ok_or(ImportError::NoMusic)?;

    let mut mei = Mei::default();
    mei.mei_version.meiversion = Some("6.0-dev".to_string());

    // Minimal meiHead with empty fileDesc/titleStmt
    let mei_head = build_mei_head();
    mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

    // Music → Body → Mdiv → Score
    let mei_music = build_music(music)?;
    mei.children.push(MeiChild::Music(Box::new(mei_music)));

    Ok(mei)
}

/// Find the first music expression in the LilyPond file.
fn find_music(file: &LilyPondFile) -> Option<&Music> {
    for item in &file.items {
        match item {
            ToplevelExpression::Score(score) => {
                for si in &score.items {
                    if let ScoreItem::Music(m) = si {
                        return Some(m);
                    }
                }
            }
            ToplevelExpression::Music(m) => return Some(m),
            _ => {}
        }
    }
    None
}

/// Build a minimal MeiHead.
fn build_mei_head() -> MeiHead {
    let title_stmt = TitleStmt::default();
    let mut file_desc = FileDesc::default();
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
    let mut head = MeiHead::default();
    head.children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
    head
}

/// Build MEI Music → Body → Mdiv → Score from LilyPond music.
fn build_music(ly_music: &Music) -> Result<tusk_model::elements::Music, ImportError> {
    let mut score = Score::default();

    // ScoreDef with one staffDef
    let score_def = build_score_def();
    score
        .children
        .push(ScoreChild::ScoreDef(Box::new(score_def)));

    // Section with measure(s) containing the notes
    let section = build_section(ly_music)?;
    score.children.push(ScoreChild::Section(Box::new(section)));

    let mut mdiv = Mdiv::default();
    mdiv.children.push(MdivChild::Score(Box::new(score)));

    let mut body = Body::default();
    body.children.push(BodyChild::Mdiv(Box::new(mdiv)));

    let mut music = tusk_model::elements::Music::default();
    music
        .children
        .push(tusk_model::elements::MusicChild::Body(Box::new(body)));

    Ok(music)
}

/// Build a minimal ScoreDef with one staff.
fn build_score_def() -> ScoreDef {
    let mut staff_def = StaffDef::default();
    staff_def.n_integer.n = Some("1".to_string());

    let mut staff_grp = StaffGrp::default();
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let mut score_def = ScoreDef::default();
    score_def
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));
    score_def
}

/// Build a Section from LilyPond music. Flattens music into a single
/// measure with all events in one layer (simple model for Phase 3).
fn build_section(ly_music: &Music) -> Result<Section, ImportError> {
    let mut section = Section::default();

    // Collect all note/rest events into a flat list
    let mut events = Vec::new();
    collect_events(ly_music, &mut events);

    // Build a single measure with all events
    let mut layer = Layer::default();
    layer.n_integer.n = Some("1".to_string());

    let mut id_counter = 0u32;
    for event in &events {
        match event {
            LyEvent::Note(note) => {
                id_counter += 1;
                let mei_note = convert_note(note, id_counter);
                layer.children.push(LayerChild::Note(Box::new(mei_note)));
            }
            LyEvent::Rest(rest) => {
                id_counter += 1;
                let mei_rest = convert_rest(rest, id_counter);
                layer.children.push(LayerChild::Rest(Box::new(mei_rest)));
            }
            LyEvent::PitchedRest(note) => {
                id_counter += 1;
                let mei_rest = convert_pitched_rest(note, id_counter);
                layer.children.push(LayerChild::Rest(Box::new(mei_rest)));
            }
            LyEvent::MeasureRest(rest) => {
                id_counter += 1;
                let mei_mrest = convert_mrest(rest, id_counter);
                layer.children.push(LayerChild::MRest(Box::new(mei_mrest)));
            }
            LyEvent::Skip(_) => {
                // Skips have no MEI equivalent in the layer; ignore for now
            }
        }
    }

    let mut staff = Staff::default();
    staff.n_integer.n = Some("1".to_string());
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let mut measure = Measure::default();
    measure.common.n = Some(tusk_model::generated::data::DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    Ok(section)
}

/// Internal event representation for collecting from the AST.
enum LyEvent<'a> {
    Note(&'a NoteEvent),
    Rest(&'a RestEvent),
    PitchedRest(&'a NoteEvent),
    MeasureRest(&'a model::MultiMeasureRestEvent),
    Skip(()),
}

/// Recursively collect note/rest/skip events from LilyPond music.
fn collect_events<'a>(music: &'a Music, events: &mut Vec<LyEvent<'a>>) {
    match music {
        Music::Note(note) => {
            if note.pitched_rest {
                events.push(LyEvent::PitchedRest(note));
            } else {
                events.push(LyEvent::Note(note));
            }
        }
        Music::Rest(rest) => events.push(LyEvent::Rest(rest)),
        Music::Skip(_) => events.push(LyEvent::Skip(())),
        Music::MultiMeasureRest(mrest) => events.push(LyEvent::MeasureRest(mrest)),
        Music::Sequential(items) | Music::Simultaneous(items) => {
            for item in items {
                collect_events(item, events);
            }
        }
        Music::Relative { body, .. } | Music::Fixed { body, .. } => {
            collect_events(body, events);
        }
        Music::ContextedMusic { music, .. } => {
            collect_events(music, events);
        }
        Music::Event(_) | Music::Identifier(_) | Music::Unparsed(_) => {}
    }
}

// ---------------------------------------------------------------------------
// Pitch / duration / accidental conversion
// ---------------------------------------------------------------------------

/// Convert LilyPond step char to MEI pitch name string.
fn step_to_pname(step: char) -> DataPitchname {
    DataPitchname(step.to_string())
}

/// Convert LilyPond octave marks to MEI absolute octave.
///
/// LilyPond absolute octave convention: c (no marks) = octave 3,
/// c' = 4, c'' = 5, c, = 2, c,, = 1.
fn octave_to_mei(octave_marks: i8) -> DataOctave {
    DataOctave((3 + octave_marks as i64).max(0) as u64)
}

/// Convert LilyPond alter (half-steps) to MEI gestural accidental.
fn alter_to_accid_ges(alter: f32) -> Option<DataAccidentalGestural> {
    let key = (alter * 2.0) as i32;
    match key {
        0 => None,
        2 => Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::S,
        )),
        4 => Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::Ss,
        )),
        -2 => Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::F,
        )),
        -4 => Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::Ff,
        )),
        _ => None,
    }
}

/// Convert LilyPond alter (half-steps) to MEI written accidental.
fn alter_to_accid_written(alter: f32) -> Option<DataAccidentalWritten> {
    let key = (alter * 2.0) as i32;
    match key {
        0 => None,
        2 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::S,
        )),
        4 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::Ss,
        )),
        -2 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::F,
        )),
        -4 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::Ff,
        )),
        _ => None,
    }
}

/// Convert LilyPond duration base to MEI DataDurationCmn.
fn duration_base_to_mei(base: u32) -> Option<DataDurationCmn> {
    match base {
        1 => Some(DataDurationCmn::N1),
        2 => Some(DataDurationCmn::N2),
        4 => Some(DataDurationCmn::N4),
        8 => Some(DataDurationCmn::N8),
        16 => Some(DataDurationCmn::N16),
        32 => Some(DataDurationCmn::N32),
        64 => Some(DataDurationCmn::N64),
        128 => Some(DataDurationCmn::N128),
        _ => None,
    }
}

/// Apply duration to an MEI note's @dur and @dots.
fn apply_duration_to_note(dur: &Duration, note: &mut Note) {
    if let Some(cmn) = duration_base_to_mei(dur.base) {
        note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(cmn));
    }
    if dur.dots > 0 {
        note.note_log.dots = Some(DataAugmentdot(dur.dots as u64));
    }
}

/// Apply duration to an MEI rest's @dur and @dots.
fn apply_duration_to_rest(dur: &Duration, rest: &mut Rest) {
    if let Some(cmn) = duration_base_to_mei(dur.base) {
        rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(cmn));
    }
    if dur.dots > 0 {
        rest.rest_log.dots = Some(DataAugmentdot(dur.dots as u64));
    }
}

// ---------------------------------------------------------------------------
// Event conversion
// ---------------------------------------------------------------------------

/// Convert a LilyPond NoteEvent to an MEI Note.
fn convert_note(note: &NoteEvent, id: u32) -> Note {
    let mut mei_note = Note::default();
    mei_note.common.xml_id = Some(format!("ly-note-{id}"));

    // Pitch
    mei_note.note_log.pname = Some(step_to_pname(note.pitch.step));
    mei_note.note_log.oct = Some(octave_to_mei(note.pitch.octave));

    // Gestural accidental
    if let Some(accid_ges) = alter_to_accid_ges(note.pitch.alter) {
        mei_note.note_ges.accid_ges = Some(accid_ges);
    }

    // Written accidental (force or cautionary)
    if (note.pitch.force_accidental || note.pitch.cautionary)
        && let Some(accid_written) = alter_to_accid_written(note.pitch.alter)
    {
        let mut accid = Accid::default();
        accid.accid_log.accid = Some(accid_written);
        if note.pitch.cautionary {
            accid.accid_log.func = Some("cautionary".to_string());
        }
        mei_note.children.push(NoteChild::Accid(Box::new(accid)));
    }

    // Duration
    if let Some(ref dur) = note.duration {
        apply_duration_to_note(dur, &mut mei_note);
    }

    mei_note
}

/// Convert a LilyPond RestEvent to an MEI Rest.
fn convert_rest(rest: &RestEvent, id: u32) -> Rest {
    let mut mei_rest = Rest::default();
    mei_rest.common.xml_id = Some(format!("ly-rest-{id}"));

    if let Some(ref dur) = rest.duration {
        apply_duration_to_rest(dur, &mut mei_rest);
    }

    mei_rest
}

/// Convert a pitched rest (note with \rest) to an MEI Rest with label.
fn convert_pitched_rest(note: &NoteEvent, id: u32) -> Rest {
    let mut mei_rest = Rest::default();
    mei_rest.common.xml_id = Some(format!("ly-rest-{id}"));

    // Store pitch position as label for roundtrip
    mei_rest.common.label = Some(format!(
        "lilypond:pitched-rest,{}{}",
        note.pitch.to_note_name(),
        note.pitch.octave_marks()
    ));

    if let Some(ref dur) = note.duration {
        apply_duration_to_rest(dur, &mut mei_rest);
    }

    mei_rest
}

/// Convert a LilyPond MultiMeasureRestEvent to an MEI MRest.
fn convert_mrest(rest: &model::MultiMeasureRestEvent, id: u32) -> MRest {
    let mut mei_mrest = MRest::default();
    mei_mrest.common.xml_id = Some(format!("ly-mrest-{id}"));

    // Store full duration info in label for lossless roundtrip
    if let Some(ref dur) = rest.duration {
        let mut label_parts = Vec::new();
        label_parts.push(format!("dur={}", dur.base));
        if dur.dots > 0 {
            label_parts.push(format!("dots={}", dur.dots));
        }
        for (num, den) in &dur.multipliers {
            if *den == 1 {
                label_parts.push(format!("mul={num}"));
            } else {
                label_parts.push(format!("mul={num}/{den}"));
            }
        }
        mei_mrest.common.label = Some(format!("lilypond:mrest,{}", label_parts.join(",")));
    }

    mei_mrest
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    fn parse_and_import(src: &str) -> Mei {
        let file = Parser::new(src).unwrap().parse().unwrap();
        import(&file).unwrap()
    }

    /// Walk MEI to find layer children.
    fn layer_children(mei: &Mei) -> &[LayerChild] {
        for child in &mei.children {
            if let MeiChild::Music(music) = child {
                for mc in &music.children {
                    let tusk_model::elements::MusicChild::Body(body) = mc;
                    for bc in &body.children {
                        let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                        for dc in &mdiv.children {
                            let tusk_model::elements::MdivChild::Score(score) = dc;
                            for sc in &score.children {
                                if let ScoreChild::Section(section) = sc {
                                    for sec_c in &section.children {
                                        if let SectionChild::Measure(measure) = sec_c {
                                            for mc2 in &measure.children {
                                                if let MeasureChild::Staff(staff) = mc2
                                                    && let Some(StaffChild::Layer(layer)) =
                                                        staff.children.first()
                                                {
                                                    return &layer.children;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        &[]
    }

    #[test]
    fn import_single_note() {
        let mei = parse_and_import("{ c'4 }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 1);
        if let LayerChild::Note(note) = &children[0] {
            assert_eq!(note.note_log.pname.as_ref().unwrap().0, "c");
            assert_eq!(note.note_log.oct.as_ref().unwrap().0, 4); // c' = oct 4
            assert!(matches!(
                note.note_log.dur,
                Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4))
            ));
        } else {
            panic!("expected Note");
        }
    }

    #[test]
    fn import_note_with_accidental() {
        let mei = parse_and_import("{ cis''2 }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 1);
        if let LayerChild::Note(note) = &children[0] {
            assert_eq!(note.note_log.pname.as_ref().unwrap().0, "c");
            assert_eq!(note.note_log.oct.as_ref().unwrap().0, 5); // c'' = oct 5
            assert!(note.note_ges.accid_ges.is_some()); // sharp
            assert!(matches!(
                note.note_log.dur,
                Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N2))
            ));
        } else {
            panic!("expected Note");
        }
    }

    #[test]
    fn import_rest() {
        let mei = parse_and_import("{ r4 }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 1);
        if let LayerChild::Rest(rest) = &children[0] {
            assert!(matches!(
                rest.rest_log.dur,
                Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N4))
            ));
        } else {
            panic!("expected Rest");
        }
    }

    #[test]
    fn import_dotted_rest() {
        let mei = parse_and_import("{ r2. }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 1);
        if let LayerChild::Rest(rest) = &children[0] {
            assert!(matches!(
                rest.rest_log.dur,
                Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N2))
            ));
            assert_eq!(rest.rest_log.dots.as_ref().unwrap().0, 1);
        } else {
            panic!("expected Rest");
        }
    }

    #[test]
    fn import_multi_measure_rest() {
        let mei = parse_and_import("{ R1*4 }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 1);
        if let LayerChild::MRest(mrest) = &children[0] {
            assert!(mrest.common.label.is_some());
            let label = mrest.common.label.as_ref().unwrap();
            assert!(label.starts_with("lilypond:mrest,"));
            assert!(label.contains("dur=1"));
            assert!(label.contains("mul=4"));
        } else {
            panic!("expected MRest");
        }
    }

    #[test]
    fn import_pitched_rest() {
        let mei = parse_and_import("{ c4\\rest }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 1);
        if let LayerChild::Rest(rest) = &children[0] {
            assert!(rest.common.label.is_some());
            assert!(
                rest.common
                    .label
                    .as_ref()
                    .unwrap()
                    .starts_with("lilypond:pitched-rest,")
            );
        } else {
            panic!("expected Rest for pitched rest");
        }
    }

    #[test]
    fn import_multiple_events() {
        let mei = parse_and_import("{ c4 d8 r4 e16 }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 4);
        assert!(matches!(&children[0], LayerChild::Note(_)));
        assert!(matches!(&children[1], LayerChild::Note(_)));
        assert!(matches!(&children[2], LayerChild::Rest(_)));
        assert!(matches!(&children[3], LayerChild::Note(_)));
    }

    #[test]
    fn import_skip_ignored() {
        let mei = parse_and_import("{ c4 s4 d4 }");
        let children = layer_children(&mei);
        // Skip is ignored, so only c4 and d4
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn import_from_score_block() {
        let mei = parse_and_import("\\score { { c4 d4 } }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 2);
    }

    #[test]
    fn import_nested_relative() {
        let mei = parse_and_import("\\relative c' { c4 d e f }");
        let children = layer_children(&mei);
        assert_eq!(children.len(), 4);
    }
}

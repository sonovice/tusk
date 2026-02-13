//! Integration tests for MEI → MusicXML conversion.
//!
//! These tests verify that MEI elements can be correctly converted to MusicXML format.
//! The tests focus on individual element conversions: notes, rests, and chords.

use tusk_model::data::{
    DataAccidentalGestural, DataAccidentalGesturalBasic, DataAugmentdot, DataBoolean, DataDuration,
    DataDurationCmn, DataDurationrests, DataGrace, DataOctave, DataPitchname, DataStemdirection,
    DataStemdirectionBasic,
};
use tusk_model::elements::{Chord, ChordChild, Note, Rest};
use tusk_musicxml::context::{ConversionContext, ConversionDirection};
use tusk_musicxml::export::{convert_mei_chord, convert_mei_note, convert_mei_rest};

// ============================================================================
// Helper Functions
// ============================================================================

fn dur_str_to_cmn(s: &str) -> DataDurationCmn {
    match s {
        "0" => DataDurationCmn::Breve,
        "1" => DataDurationCmn::N1,
        "2" => DataDurationCmn::N2,
        "4" => DataDurationCmn::N4,
        "8" => DataDurationCmn::N8,
        "16" => DataDurationCmn::N16,
        _ => DataDurationCmn::N4,
    }
}

fn dur_str_to_data_duration(s: &str) -> DataDuration {
    DataDuration::MeiDataDurationCmn(dur_str_to_cmn(s))
}

fn dur_str_to_data_duration_rests(s: &str) -> DataDurationrests {
    DataDurationrests::MeiDataDurationCmn(dur_str_to_cmn(s))
}

/// MEI duration string: "1"=whole, "2"=half, "4"=quarter, "8"=eighth, "16"=sixteenth, "0"=breve.
fn create_mei_note(pname: &str, octave: u64, dur: &str) -> Note {
    let mut note = Note::default();
    note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
    note.note_log.oct = Some(DataOctave::from(octave));
    note.note_log.dur = Some(dur_str_to_data_duration(dur));
    note
}

// ============================================================================
// Note Conversion Tests
// ============================================================================

#[test]
fn test_convert_mei_note_quarter_c4() {
    use tusk_musicxml::model::data::Step;
    use tusk_musicxml::model::note::FullNoteContent;

    let mei_note = create_mei_note("c", 4, "4");

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Verify pitch
    if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
        assert_eq!(pitch.step, Step::C);
        assert_eq!(pitch.octave, 4);
    } else {
        panic!("Expected pitched note");
    }

    // Verify duration (quarter note = 4 divisions at divisions=4)
    assert_eq!(mxml_note.duration, Some(4.0));
}

#[test]
fn test_convert_mei_note_half_g5() {
    use tusk_musicxml::model::data::Step;
    use tusk_musicxml::model::note::{FullNoteContent, NoteTypeValue};

    let mei_note = create_mei_note("g", 5, "2");

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Verify pitch
    if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
        assert_eq!(pitch.step, Step::G);
        assert_eq!(pitch.octave, 5);
    } else {
        panic!("Expected pitched note");
    }

    // Verify duration (half note = 8 divisions at divisions=4)
    assert_eq!(mxml_note.duration, Some(8.0));

    // Verify note type
    assert!(mxml_note.note_type.is_some());
    assert_eq!(
        mxml_note.note_type.as_ref().unwrap().value,
        NoteTypeValue::Half
    );
}

#[test]
fn test_convert_mei_note_eighth_a3() {
    use tusk_musicxml::model::data::Step;
    use tusk_musicxml::model::note::{FullNoteContent, NoteTypeValue};

    let mei_note = create_mei_note("a", 3, "8");

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Verify pitch
    if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
        assert_eq!(pitch.step, Step::A);
        assert_eq!(pitch.octave, 3);
    } else {
        panic!("Expected pitched note");
    }

    // Verify duration (eighth note = 2 divisions at divisions=4)
    assert_eq!(mxml_note.duration, Some(2.0));

    // Verify note type
    assert!(mxml_note.note_type.is_some());
    assert_eq!(
        mxml_note.note_type.as_ref().unwrap().value,
        NoteTypeValue::Eighth
    );
}

#[test]
fn test_convert_mei_note_whole_e4() {
    use tusk_musicxml::model::data::Step;
    use tusk_musicxml::model::note::{FullNoteContent, NoteTypeValue};

    let mei_note = create_mei_note("e", 4, "1");

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Verify pitch
    if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
        assert_eq!(pitch.step, Step::E);
        assert_eq!(pitch.octave, 4);
    } else {
        panic!("Expected pitched note");
    }

    // Verify duration (whole note = 16 divisions at divisions=4)
    assert_eq!(mxml_note.duration, Some(16.0));

    // Verify note type
    assert!(mxml_note.note_type.is_some());
    assert_eq!(
        mxml_note.note_type.as_ref().unwrap().value,
        NoteTypeValue::Whole
    );
}

#[test]
fn test_convert_mei_note_with_high_divisions() {
    // Test with divisions=96 (common in notation software)
    let mei_note = create_mei_note("c", 4, "4");

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(96.0);

    let result = convert_mei_note(&mei_note, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Quarter note at divisions=96 should be 96 divisions
    assert_eq!(mxml_note.duration, Some(96.0));
}

#[test]
fn test_convert_mei_note_sixteenth() {
    use tusk_musicxml::model::note::NoteTypeValue;

    let mei_note = create_mei_note("d", 4, "16");

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Sixteenth note = 1 division at divisions=4
    assert_eq!(mxml_note.duration, Some(1.0));

    // Verify note type
    assert!(mxml_note.note_type.is_some());
    assert_eq!(
        mxml_note.note_type.as_ref().unwrap().value,
        NoteTypeValue::N16th
    );
}

#[test]
fn test_convert_mei_note_breve() {
    use tusk_musicxml::model::note::NoteTypeValue;

    let mei_note = create_mei_note("c", 4, "0");

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Breve = 8 quarters = 32 divisions at divisions=4
    assert_eq!(mxml_note.duration, Some(32.0));

    // Verify note type
    assert!(mxml_note.note_type.is_some());
    assert_eq!(
        mxml_note.note_type.as_ref().unwrap().value,
        NoteTypeValue::Breve
    );
}

// ============================================================================
// Rest Conversion Tests
// ============================================================================

#[test]
fn test_convert_mei_rest_quarter() {
    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some(dur_str_to_data_duration_rests("4"));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_rest(&mei_rest, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Verify it's a rest
    assert!(mxml_note.is_rest());

    // Verify duration (quarter rest = 4 divisions at divisions=4)
    assert_eq!(mxml_note.duration, Some(4.0));
}

#[test]
fn test_convert_mei_rest_half() {
    use tusk_musicxml::model::note::NoteTypeValue;

    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some(dur_str_to_data_duration_rests("2"));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_rest(&mei_rest, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Verify it's a rest
    assert!(mxml_note.is_rest());

    // Verify duration (half rest = 8 divisions at divisions=4)
    assert_eq!(mxml_note.duration, Some(8.0));

    // Verify note type
    assert!(mxml_note.note_type.is_some());
    assert_eq!(
        mxml_note.note_type.as_ref().unwrap().value,
        NoteTypeValue::Half
    );
}

#[test]
fn test_convert_mei_rest_whole() {
    use tusk_musicxml::model::note::NoteTypeValue;

    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some(dur_str_to_data_duration_rests("1"));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_rest(&mei_rest, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Verify it's a rest
    assert!(mxml_note.is_rest());

    // Verify duration (whole rest = 16 divisions at divisions=4)
    assert_eq!(mxml_note.duration, Some(16.0));

    // Verify note type
    assert!(mxml_note.note_type.is_some());
    assert_eq!(
        mxml_note.note_type.as_ref().unwrap().value,
        NoteTypeValue::Whole
    );
}

#[test]
fn test_convert_mei_rest_eighth() {
    use tusk_musicxml::model::note::NoteTypeValue;

    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some(dur_str_to_data_duration_rests("8"));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_rest(&mei_rest, &mut ctx);
    assert!(result.is_ok());

    let mxml_note = result.unwrap();

    // Verify it's a rest
    assert!(mxml_note.is_rest());

    // Verify duration (eighth rest = 2 divisions at divisions=4)
    assert_eq!(mxml_note.duration, Some(2.0));

    // Verify note type
    assert!(mxml_note.note_type.is_some());
    assert_eq!(
        mxml_note.note_type.as_ref().unwrap().value,
        NoteTypeValue::Eighth
    );
}

// ============================================================================
// Chord Conversion Tests
// ============================================================================

#[test]
fn test_convert_mei_chord_c_major() {
    use tusk_musicxml::model::data::Step;
    use tusk_musicxml::model::note::FullNoteContent;

    // Create a C major chord (C4, E4, G4)
    let mut mei_chord = Chord::default();
    mei_chord.chord_log.dur = Some(dur_str_to_data_duration("4"));

    let note_c = create_mei_note("c", 4, "4");
    let note_e = create_mei_note("e", 4, "4");
    let note_g = create_mei_note("g", 4, "4");

    mei_chord.children.push(ChordChild::Note(Box::new(note_c)));
    mei_chord.children.push(ChordChild::Note(Box::new(note_e)));
    mei_chord.children.push(ChordChild::Note(Box::new(note_g)));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_chord(&mei_chord, &mut ctx);
    assert!(result.is_ok());

    let mxml_notes = result.unwrap();

    // Should have 3 notes
    assert_eq!(mxml_notes.len(), 3);

    // First note should NOT have chord flag
    assert!(mxml_notes[0].chord.is_none());

    // Second and third notes should have chord flag
    assert!(mxml_notes[1].chord.is_some());
    assert!(mxml_notes[2].chord.is_some());

    // Verify pitches
    let pitches: Vec<Step> = mxml_notes
        .iter()
        .filter_map(|n| {
            if let FullNoteContent::Pitch(p) = &n.content {
                Some(p.step)
            } else {
                None
            }
        })
        .collect();
    assert_eq!(pitches, vec![Step::C, Step::E, Step::G]);
}

#[test]
fn test_convert_mei_chord_duration() {
    // Create a half note chord
    let mut mei_chord = Chord::default();
    mei_chord.chord_log.dur = Some(dur_str_to_data_duration("2"));

    let note_c = create_mei_note("c", 4, "4");
    let note_e = create_mei_note("e", 4, "4");

    mei_chord.children.push(ChordChild::Note(Box::new(note_c)));
    mei_chord.children.push(ChordChild::Note(Box::new(note_e)));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_chord(&mei_chord, &mut ctx);
    assert!(result.is_ok());

    let mxml_notes = result.unwrap();

    // Both notes should have the same duration (half note = 8 divisions)
    for note in &mxml_notes {
        assert_eq!(note.duration, Some(8.0));
    }
}

#[test]
fn test_convert_mei_chord_g_minor() {
    use tusk_musicxml::model::data::Step;
    use tusk_musicxml::model::note::FullNoteContent;

    // Create a G minor chord (G4, Bb4, D5)
    let mut mei_chord = Chord::default();
    mei_chord.chord_log.dur = Some(dur_str_to_data_duration("4"));

    let note_g = create_mei_note("g", 4, "4");
    let note_b = create_mei_note("b", 4, "4");
    let note_d = create_mei_note("d", 5, "4");

    mei_chord.children.push(ChordChild::Note(Box::new(note_g)));
    mei_chord.children.push(ChordChild::Note(Box::new(note_b)));
    mei_chord.children.push(ChordChild::Note(Box::new(note_d)));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_chord(&mei_chord, &mut ctx);
    assert!(result.is_ok());

    let mxml_notes = result.unwrap();

    // Verify pitches in order
    let pitches: Vec<(Step, u8)> = mxml_notes
        .iter()
        .filter_map(|n| {
            if let FullNoteContent::Pitch(p) = &n.content {
                Some((p.step, p.octave))
            } else {
                None
            }
        })
        .collect();
    assert_eq!(pitches, vec![(Step::G, 4), (Step::B, 4), (Step::D, 5)]);
}

// ============================================================================
// Division Calculation Tests
// ============================================================================

#[test]
fn test_duration_divisions_relationship() {
    // Test that durations scale correctly with different division values

    let test_cases = [
        // (duration_cmn, divisions, expected_mxml_duration)
        ("1", 4.0, 16.0),   // Whole: 4 quarters * 4 div/quarter
        ("2", 4.0, 8.0),    // Half: 2 quarters * 4 div/quarter
        ("4", 4.0, 4.0),    // Quarter: 1 quarter * 4 div/quarter
        ("8", 4.0, 2.0),    // Eighth: 0.5 quarters * 4 div/quarter
        ("16", 4.0, 1.0),   // 16th: 0.25 quarters * 4 div/quarter
        ("4", 1.0, 1.0),    // Quarter with div=1
        ("4", 96.0, 96.0),  // Quarter with div=96
        ("8", 96.0, 48.0),  // Eighth with div=96
        ("16", 96.0, 24.0), // 16th with div=96
    ];

    for (dur_cmn, divisions, expected) in test_cases {
        let mei_note = create_mei_note("c", 4, dur_cmn);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(divisions);

        let result = convert_mei_note(&mei_note, &mut ctx).unwrap();
        assert!(
            (result.duration.unwrap() - expected).abs() < 1e-10,
            "Duration {:?} with divisions {} expected {} but got {}",
            dur_cmn,
            divisions,
            expected,
            result.duration.unwrap()
        );
    }
}

// ============================================================================
// Dotted Note Duration Tests
// ============================================================================

#[test]
fn test_convert_dotted_quarter_note() {
    let mut mei_note = create_mei_note("c", 4, "4");
    mei_note.note_log.dots = Some(DataAugmentdot::from(1u64));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    // Dotted quarter = 1.5 quarters = 6 divisions at div=4
    assert_eq!(result.duration, Some(6.0));
    assert_eq!(result.dots.len(), 1);
}

#[test]
fn test_convert_double_dotted_half_note() {
    let mut mei_note = create_mei_note("c", 4, "2");
    mei_note.note_log.dots = Some(DataAugmentdot::from(2u64));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    // Double-dotted half = 2 + 1 + 0.5 = 3.5 quarters = 14 divisions at div=4
    assert_eq!(result.duration, Some(14.0));
    assert_eq!(result.dots.len(), 2);
}

#[test]
fn test_convert_dotted_eighth_note() {
    let mut mei_note = create_mei_note("f", 4, "8");
    mei_note.note_log.dots = Some(DataAugmentdot::from(1u64));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    // Dotted eighth = 0.75 quarters = 3 divisions at div=4
    assert_eq!(result.duration, Some(3.0));
    assert_eq!(result.dots.len(), 1);
}

// ============================================================================
// Grace Note Tests
// ============================================================================

#[test]
fn test_convert_grace_note() {
    let mut mei_note = create_mei_note("c", 4, "8");
    mei_note.note_log.grace = Some(DataGrace::Unacc);

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    // Grace notes should not have duration
    assert!(result.duration.is_none());
    assert!(result.is_grace());
}

#[test]
fn test_convert_accented_grace_note() {
    use tusk_musicxml::model::data::YesNo;

    let mut mei_note = create_mei_note("d", 4, "16");
    mei_note.note_log.grace = Some(DataGrace::Acc);

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    // Grace notes should not have duration
    assert!(result.duration.is_none());
    assert!(result.is_grace());

    // Accented grace should NOT have slash
    assert_eq!(result.grace.as_ref().unwrap().slash, Some(YesNo::No));
}

#[test]
fn test_convert_unaccented_grace_note_has_slash() {
    use tusk_musicxml::model::data::YesNo;

    let mut mei_note = create_mei_note("e", 4, "16");
    mei_note.note_log.grace = Some(DataGrace::Unacc);

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    // Unaccented grace should have slash
    assert_eq!(result.grace.as_ref().unwrap().slash, Some(YesNo::Yes));
}

// ============================================================================
// ID Mapping Tests
// ============================================================================

#[test]
fn test_note_id_preserved() {
    let mut mei_note = create_mei_note("c", 4, "4");
    mei_note.common.xml_id = Some("note-1".to_string());

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    assert_eq!(result.id.as_deref(), Some("note-1"));
}

#[test]
fn test_rest_id_preserved() {
    let mut mei_rest = Rest::default();
    mei_rest.common.xml_id = Some("rest-1".to_string());
    mei_rest.rest_log.dur = Some(dur_str_to_data_duration_rests("4"));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_rest(&mei_rest, &mut ctx).unwrap();

    assert_eq!(result.id.as_deref(), Some("rest-1"));
}

#[test]
fn test_chord_id_mapped() {
    let mut mei_chord = Chord::default();
    mei_chord.common.xml_id = Some("chord-1".to_string());
    mei_chord.chord_log.dur = Some(dur_str_to_data_duration("4"));

    let note_c = create_mei_note("c", 4, "4");
    mei_chord.children.push(ChordChild::Note(Box::new(note_c)));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let _ = convert_mei_chord(&mei_chord, &mut ctx).unwrap();

    // Chord ID should be mapped in context
    assert!(ctx.get_mei_id("chord-1").is_some());
}

// ============================================================================
// Accidental Tests
// ============================================================================

#[test]
fn test_note_with_gestural_sharp() {
    use tusk_musicxml::model::note::FullNoteContent;

    let mut mei_note = create_mei_note("f", 4, "4");
    mei_note.note_ges.accid_ges = Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
        DataAccidentalGesturalBasic::S,
    ));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    if let FullNoteContent::Pitch(pitch) = &result.content {
        assert_eq!(pitch.alter, Some(1.0)); // Sharp = +1
    } else {
        panic!("Expected pitched note");
    }
}

#[test]
fn test_note_with_gestural_flat() {
    use tusk_musicxml::model::note::FullNoteContent;

    let mut mei_note = create_mei_note("b", 4, "4");
    mei_note.note_ges.accid_ges = Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
        DataAccidentalGesturalBasic::F,
    ));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    if let FullNoteContent::Pitch(pitch) = &result.content {
        assert_eq!(pitch.alter, Some(-1.0)); // Flat = -1
    } else {
        panic!("Expected pitched note");
    }
}

#[test]
fn test_note_with_gestural_double_sharp() {
    use tusk_musicxml::model::note::FullNoteContent;

    let mut mei_note = create_mei_note("c", 4, "4");
    mei_note.note_ges.accid_ges = Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
        DataAccidentalGesturalBasic::Ss,
    ));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    if let FullNoteContent::Pitch(pitch) = &result.content {
        assert_eq!(pitch.alter, Some(2.0)); // Double sharp = +2
    } else {
        panic!("Expected pitched note");
    }
}

// ============================================================================
// Stem Direction Tests
// ============================================================================

#[test]
fn test_note_with_stem_up() {
    use tusk_musicxml::model::note::StemValue;

    let mut mei_note = create_mei_note("c", 4, "4");
    mei_note.note_vis.stem_dir = Some(DataStemdirection::MeiDataStemdirectionBasic(
        DataStemdirectionBasic::Up,
    ));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    assert!(result.stem.is_some());
    assert_eq!(result.stem.as_ref().unwrap().value, StemValue::Up);
}

#[test]
fn test_note_with_stem_down() {
    use tusk_musicxml::model::note::StemValue;

    let mut mei_note = create_mei_note("a", 5, "4");
    mei_note.note_vis.stem_dir = Some(DataStemdirection::MeiDataStemdirectionBasic(
        DataStemdirectionBasic::Down,
    ));

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    assert!(result.stem.is_some());
    assert_eq!(result.stem.as_ref().unwrap().value, StemValue::Down);
}

// ============================================================================
// Cue Note Tests
// ============================================================================

#[test]
fn test_cue_note() {
    let mut mei_note = create_mei_note("c", 4, "4");
    mei_note.note_log.cue = Some(DataBoolean::True);

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    assert!(result.is_cue());
}

#[test]
fn test_cue_rest() {
    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some(dur_str_to_data_duration_rests("4"));
    mei_rest.rest_log.cue = Some(DataBoolean::True);

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_rest(&mei_rest, &mut ctx).unwrap();

    assert!(result.is_cue());
}

// ============================================================================
// Native MEI Element Export Tests (Phase 36.4)
//
// These tests construct MEI documents with native bTrem, fTrem, and Fing
// elements (not produced by MusicXML import) and verify they export correctly.
// ============================================================================

use tusk_model::data::{
    DataClefline, DataClefshape, DataKeyfifths, DataStaffrel, DataStaffrelBasic, DataUri,
};
use tusk_model::elements::{
    BTrem, BTremChild, Body, BodyChild, FTrem, FTremChild, FileDesc, FileDescChild, Fing,
    FingChild, Layer, LayerChild, Mdiv, MdivChild, Measure, MeasureChild, Mei, MeiChild, MeiHead,
    MeiHeadChild, Music, MusicChild, Score, ScoreChild, ScoreDef, ScoreDefChild, Section,
    SectionChild, Staff, StaffChild, StaffDef, StaffGrp, StaffGrpChild, Title, TitleStmt,
    TitleStmtChild,
};

/// Build a minimal MEI document with a single staff and the given layer/measure children.
fn build_mei_doc(
    layer_children: Vec<LayerChild>,
    measure_control_events: Vec<MeasureChild>,
) -> Mei {
    // MeiHead
    let mut mei_head = MeiHead::default();
    let mut file_desc = FileDesc::default();
    let mut title_stmt = TitleStmt::default();
    let mut title = Title::default();
    title
        .children
        .push(tusk_model::elements::TitleChild::Text("Test".to_string()));
    title_stmt
        .children
        .push(TitleStmtChild::Title(Box::new(title)));
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
    mei_head
        .children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));

    // StaffDef
    let mut staff_def = StaffDef::default();
    staff_def.n_integer.n = Some("1".to_string());
    staff_def.staff_def_log.lines = Some("5".to_string());
    staff_def.staff_def_log.clef_shape = Some(DataClefshape::G);
    staff_def.staff_def_log.clef_line = Some(DataClefline::from(2u64));
    staff_def.staff_def_log.keysig = Some(DataKeyfifths::from("0".to_string()));
    staff_def.staff_def_log.meter_count = Some("4".to_string());
    staff_def.staff_def_log.meter_unit = Some("4".to_string());

    let mut staff_grp = StaffGrp::default();
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let mut score_def = ScoreDef::default();
    score_def
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));

    // Layer
    let mut layer = Layer::default();
    layer.n_integer.n = Some("1".to_string());
    layer.children = layer_children;

    // Staff
    let mut staff = Staff::default();
    staff.n_integer.n = Some("1".to_string());
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    // Measure
    let mut measure = Measure::default();
    measure.common.n = Some(tusk_model::data::DataWord::from("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));
    for ce in measure_control_events {
        measure.children.push(ce);
    }

    // Section → Score → Mdiv → Body → Music
    let mut section = Section::default();
    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    let mut score = Score::default();
    score
        .children
        .push(ScoreChild::ScoreDef(Box::new(score_def)));
    score.children.push(ScoreChild::Section(Box::new(section)));

    let mut mdiv = Mdiv::default();
    mdiv.children.push(MdivChild::Score(Box::new(score)));

    let mut body = Body::default();
    body.children.push(BodyChild::Mdiv(Box::new(mdiv)));

    let mut music = Music::default();
    music.children.push(MusicChild::Body(Box::new(body)));

    let mut mei = Mei::default();
    mei.mei_version.meiversion = Some("6.0-dev".to_string());
    mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));
    mei.children.push(MeiChild::Music(Box::new(music)));
    mei
}

#[test]
fn test_export_btrem_single_tremolo() {
    use tusk_musicxml::model::data::TremoloType;

    // Create a note wrapped in bTrem with unitdur=N32 (3 marks)
    let mut note = create_mei_note("c", 4, "4");
    note.common.xml_id = Some("n1".to_string());

    let mut btrem = BTrem::default();
    btrem.b_trem_ges.unitdur = Some(DataDurationCmn::N32);
    btrem.children.push(BTremChild::Note(Box::new(note)));

    let mei = build_mei_doc(vec![LayerChild::BTrem(Box::new(btrem))], vec![]);

    let result = tusk_musicxml::export(&mei).unwrap();

    // Find the note in the exported MusicXML
    let part = &result.parts[0].measures[0];
    let notes: Vec<_> = part
        .content
        .iter()
        .filter_map(|c| {
            if let tusk_musicxml::model::elements::MeasureContent::Note(n) = c {
                Some(n.as_ref())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(notes.len(), 1);

    // Verify tremolo notation
    let notations = notes[0].notations.as_ref().expect("should have notations");
    let ornaments = notations.ornaments.as_ref().expect("should have ornaments");
    let tremolo = ornaments.tremolo.as_ref().expect("should have tremolo");
    assert_eq!(tremolo.tremolo_type, TremoloType::Single);
    assert_eq!(tremolo.value, Some(3));
}

#[test]
fn test_export_ftrem_fingered_tremolo() {
    use tusk_musicxml::model::data::TremoloType;

    // Create two notes wrapped in fTrem with unitdur=N16 (2 marks)
    let mut note1 = create_mei_note("c", 4, "4");
    note1.common.xml_id = Some("n1".to_string());
    let mut note2 = create_mei_note("e", 4, "4");
    note2.common.xml_id = Some("n2".to_string());

    let mut ftrem = FTrem::default();
    ftrem.f_trem_ges.unitdur = Some(DataDurationCmn::N16);
    ftrem.children.push(FTremChild::Note(Box::new(note1)));
    ftrem.children.push(FTremChild::Note(Box::new(note2)));

    let mei = build_mei_doc(vec![LayerChild::FTrem(Box::new(ftrem))], vec![]);

    let result = tusk_musicxml::export(&mei).unwrap();

    // Find notes in the exported MusicXML
    let part = &result.parts[0].measures[0];
    let notes: Vec<_> = part
        .content
        .iter()
        .filter_map(|c| {
            if let tusk_musicxml::model::elements::MeasureContent::Note(n) = c {
                Some(n.as_ref())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(notes.len(), 2);

    // First note: tremolo type=start, value=2
    let t1 = notes[0]
        .notations
        .as_ref()
        .unwrap()
        .ornaments
        .as_ref()
        .unwrap()
        .tremolo
        .as_ref()
        .unwrap();
    assert_eq!(t1.tremolo_type, TremoloType::Start);
    assert_eq!(t1.value, Some(2));

    // Second note: tremolo type=stop, value=2
    let t2 = notes[1]
        .notations
        .as_ref()
        .unwrap()
        .ornaments
        .as_ref()
        .unwrap()
        .tremolo
        .as_ref()
        .unwrap();
    assert_eq!(t2.tremolo_type, TremoloType::Stop);
    assert_eq!(t2.value, Some(2));
}

#[test]
fn test_export_fing_control_event() {
    // Create a note and a fing control event pointing to it
    let mut note = create_mei_note("c", 4, "4");
    note.common.xml_id = Some("n1".to_string());

    let mut fing = Fing::default();
    fing.common.xml_id = Some("fing1".to_string());
    fing.fing_log.startid = Some(DataUri::from("#n1".to_string()));
    fing.fing_log.staff = Some("1".to_string());
    fing.fing_vis.place = Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above));
    fing.children.push(FingChild::Text("3".to_string()));

    let mei = build_mei_doc(
        vec![LayerChild::Note(Box::new(note))],
        vec![MeasureChild::Fing(Box::new(fing))],
    );

    let result = tusk_musicxml::export(&mei).unwrap();

    // Find the note in the exported MusicXML
    let part = &result.parts[0].measures[0];
    let notes: Vec<_> = part
        .content
        .iter()
        .filter_map(|c| {
            if let tusk_musicxml::model::elements::MeasureContent::Note(n) = c {
                Some(n.as_ref())
            } else {
                None
            }
        })
        .collect();
    assert_eq!(notes.len(), 1);

    // Verify fingering notation
    let notations = notes[0].notations.as_ref().expect("should have notations");
    let technical = notations.technical.as_ref().expect("should have technical");
    assert_eq!(technical.fingering.len(), 1);
    assert_eq!(technical.fingering[0].value, "3");
    assert_eq!(
        technical.fingering[0].placement,
        Some(tusk_musicxml::model::data::AboveBelow::Above)
    );
}

#[test]
fn test_export_btrem_with_different_unitdur() {
    use tusk_musicxml::model::data::TremoloType;

    // Test unitdur=N8 (1 mark) and unitdur=N64 (4 marks)
    for (unitdur, expected_marks) in [
        (DataDurationCmn::N8, 1u8),
        (DataDurationCmn::N16, 2),
        (DataDurationCmn::N64, 4),
    ] {
        let mut note = create_mei_note("g", 4, "4");
        note.common.xml_id = Some("n1".to_string());

        let mut btrem = BTrem::default();
        btrem.b_trem_ges.unitdur = Some(unitdur);
        btrem.children.push(BTremChild::Note(Box::new(note)));

        let mei = build_mei_doc(vec![LayerChild::BTrem(Box::new(btrem))], vec![]);

        let result = tusk_musicxml::export(&mei).unwrap();

        let part = &result.parts[0].measures[0];
        let notes: Vec<_> = part
            .content
            .iter()
            .filter_map(|c| {
                if let tusk_musicxml::model::elements::MeasureContent::Note(n) = c {
                    Some(n.as_ref())
                } else {
                    None
                }
            })
            .collect();

        let tremolo = notes[0]
            .notations
            .as_ref()
            .unwrap()
            .ornaments
            .as_ref()
            .unwrap()
            .tremolo
            .as_ref()
            .unwrap();
        assert_eq!(tremolo.tremolo_type, TremoloType::Single);
        assert_eq!(
            tremolo.value,
            Some(expected_marks),
            "unitdur→marks mismatch"
        );
    }
}

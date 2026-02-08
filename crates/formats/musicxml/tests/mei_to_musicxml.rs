//! Integration tests for MEI → MusicXML conversion.
//!
//! These tests verify that MEI elements can be correctly converted to MusicXML format.
//! The tests focus on individual element conversions: notes, rests, and chords.
//!
//! The MEI model uses Option<String> for attributes (pname, oct, dur, etc.).

use tusk_model::elements::{Chord, ChordChild, Note, Rest};
use tusk_musicxml::context::{ConversionContext, ConversionDirection};
use tusk_musicxml::export::{convert_mei_chord, convert_mei_note, convert_mei_rest};

// ============================================================================
// Helper Functions
// ============================================================================

/// MEI duration string: "1"=whole, "2"=half, "4"=quarter, "8"=eighth, "16"=sixteenth, "0"=breve.
fn create_mei_note(pname: &str, octave: u64, dur: &str) -> Note {
    let mut note = Note::default();
    note.note_log.pname = Some(pname.to_string());
    note.note_log.oct = Some(octave.to_string());
    note.note_log.dur = Some(dur.to_string());
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
    mei_rest.rest_log.dur = Some("4".to_string());

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
    use tusk_model::data::DataDurationrests;
    use tusk_musicxml::model::note::NoteTypeValue;

    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some("2".to_string());

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
    use tusk_model::data::DataDurationrests;
    use tusk_musicxml::model::note::NoteTypeValue;

    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some("1".to_string());

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
    use tusk_model::data::DataDurationrests;
    use tusk_musicxml::model::note::NoteTypeValue;

    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some("8".to_string());

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
    mei_chord.chord_log.dur = Some("4".to_string());

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
    mei_chord.chord_log.dur = Some("2".to_string());

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
    mei_chord.chord_log.dur = Some("4".to_string());

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
        ("1", 4.0, 16.0), // Whole: 4 quarters * 4 div/quarter
        ("2", 4.0, 8.0),  // Half: 2 quarters * 4 div/quarter
        ("4", 4.0, 4.0),  // Quarter: 1 quarter * 4 div/quarter
        ("8", 4.0, 2.0),  // Eighth: 0.5 quarters * 4 div/quarter
        ("16", 4.0, 1.0), // 16th: 0.25 quarters * 4 div/quarter
        ("4", 1.0, 1.0),  // Quarter with div=1
        ("4", 96.0, 96.0), // Quarter with div=96
        ("8", 96.0, 48.0), // Eighth with div=96
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
    mei_note.note_log.dots = Some("1".to_string());

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
    mei_note.note_log.dots = Some("2".to_string());

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
    mei_note.note_log.dots = Some("1".to_string());

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
    mei_note.note_log.grace = Some("unacc".to_string());

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
    mei_note.note_log.grace = Some("acc".to_string());

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
    mei_note.note_log.grace = Some("unacc".to_string());

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
    mei_rest.rest_log.dur = Some("4".to_string());

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_rest(&mei_rest, &mut ctx).unwrap();

    assert_eq!(result.id.as_deref(), Some("rest-1"));
}

#[test]
fn test_chord_id_mapped() {
    let mut mei_chord = Chord::default();
    mei_chord.common.xml_id = Some("chord-1".to_string());
    mei_chord.chord_log.dur = Some("4".to_string());

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
    mei_note.note_ges.accid_ges = Some("s".to_string());

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
    mei_note.note_ges.accid_ges = Some("f".to_string());

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
    mei_note.note_ges.accid_ges = Some("ss".to_string());

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
    mei_note.note_vis.stem_dir = Some("up".to_string());

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
    mei_note.note_vis.stem_dir = Some("down".to_string());

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
    mei_note.note_log.cue = Some("true".to_string());

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_note(&mei_note, &mut ctx).unwrap();

    assert!(result.is_cue());
}

#[test]
fn test_cue_rest() {
    let mut mei_rest = Rest::default();
    mei_rest.rest_log.dur = Some("4".to_string());
    mei_rest.rest_log.cue = Some("true".to_string());

    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    ctx.set_divisions(4.0);

    let result = convert_mei_rest(&mei_rest, &mut ctx).unwrap();

    assert!(result.is_cue());
}

//! Round-trip serialization tests for MEI elements.
//!
//! These tests verify that MEI elements can be serialized to XML and
//! deserialized back without data loss. This is critical for the converter
//! to preserve musical information accurately.
//!
//! # Test Strategy
//!
//! 1. Create an element with specific attribute values
//! 2. Serialize to MEI XML string
//! 3. Deserialize back to Rust struct
//! 4. Verify all attributes match the original
//!
//! Some tests also verify XML → Struct → XML for external MEI documents.

mod note;

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;
use tusk_model::data::{
    DataArticulation, DataAugmentdot, DataDuration, DataDurationCmn, DataDurationrests, DataOctave,
    DataPitchname,
};
use tusk_model::elements::{Note, Rest};

// ============================================================================
// Space Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_space() {
    use tusk_model::elements::Space;

    let original = Space::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Space::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.space_log.dur.is_none());
}

#[test]
fn roundtrip_space_with_xml_id() {
    use tusk_model::elements::Space;

    let mut original = Space::default();
    original.common.xml_id = Some("space-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"space-1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Space::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("space-1".to_string()));
}

#[test]
fn roundtrip_space_with_duration_quarter() {
    use tusk_model::elements::Space;

    let mut original = Space::default();
    original.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Space::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.space_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn roundtrip_space_with_duration_whole() {
    use tusk_model::elements::Space;

    let mut original = Space::default();
    original.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N1));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Space::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.space_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N1))
    );
}

#[test]
fn roundtrip_space_with_dots() {
    use tusk_model::elements::Space;

    let mut original = Space::default();
    original.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.space_log.dots = Some(DataAugmentdot(2));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Space::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.space_log.dots, Some(DataAugmentdot(2)));
}

#[test]
fn roundtrip_space_complete_cmn() {
    use tusk_model::elements::Space;

    // Common Music Notation space with typical attributes
    let mut original = Space::default();
    original.common.xml_id = Some("s42".to_string());
    original.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
    original.space_log.dots = Some(DataAugmentdot(1));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Space::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.space_log.dur, original.space_log.dur);
    assert_eq!(parsed.space_log.dots, original.space_log.dots);
}

#[test]
fn roundtrip_space_with_staff_and_layer() {
    use tusk_model::elements::Space;

    let mut original = Space::default();
    original.space_log.staff = vec![1u64];
    original.space_log.layer = vec![1u64];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Space::from_mei_str(&xml).expect("deserialize");

    assert!(
        !parsed.space_log.staff.is_empty(),
        "staff should be preserved"
    );
    assert!(
        !parsed.space_log.layer.is_empty(),
        "layer should be preserved"
    );
}

#[test]
fn roundtrip_space_with_label() {
    use tusk_model::elements::Space;

    let mut original = Space::default();
    original.common.label = Some("fill space".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Space::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("fill space".to_string()));
}

// ============================================================================
// Space External XML Parsing Tests
// ============================================================================

#[test]
fn parse_external_space_minimal() {
    use tusk_model::elements::Space;

    let xml = r#"<space/>"#;
    let parsed = Space::from_mei_str(xml).expect("deserialize");

    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Space::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.common.xml_id.is_none());
}

#[test]
fn parse_external_space_with_attributes() {
    use tusk_model::elements::Space;

    let xml = r#"<space xml:id="s1" dur="4"/>"#;
    let parsed = Space::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("s1".to_string()));
    assert_eq!(
        parsed.space_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Space::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.common.xml_id, Some("s1".to_string()));
    assert_eq!(reparsed.space_log.dur, parsed.space_log.dur);
}

#[test]
fn parse_external_space_all_cmn_durations() {
    use tusk_model::elements::Space;

    // Test common music notation duration values for spaces
    for (dur_str, expected) in [
        ("long", DataDurationCmn::Long),
        ("breve", DataDurationCmn::Breve),
        ("1", DataDurationCmn::N1),
        ("2", DataDurationCmn::N2),
        ("4", DataDurationCmn::N4),
        ("8", DataDurationCmn::N8),
        ("16", DataDurationCmn::N16),
        ("32", DataDurationCmn::N32),
        ("64", DataDurationCmn::N64),
        ("128", DataDurationCmn::N128),
        ("256", DataDurationCmn::N256),
    ] {
        let xml = format!(r#"<space dur="{}"/>"#, dur_str);
        let parsed =
            Space::from_mei_str(&xml).unwrap_or_else(|_| panic!("deserialize dur={}", dur_str));
        assert_eq!(
            parsed.space_log.dur,
            Some(DataDuration::DataDurationCmn(expected)),
            "dur {} should parse correctly",
            dur_str
        );
    }
}

#[test]
fn space_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Space;

    let xml = r#"<space xml:id="s1" unknown="value" dur="4"/>"#;
    let space = Space::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(space.common.xml_id, Some("s1".to_string()));
}

// ============================================================================
// Tests Against specs/mei/examples/ CMN Files
// ============================================================================
//
// These tests verify that note, rest, chord, and space elements from real MEI
// example files can be parsed correctly. The XML fragments are extracted from
// the following files in specs/mei/examples/:
//
// - tchaikovsky_scherzo.mei (CMN with notes, rests, chords)
// - accid-03.mei (notes with accidentals)
// - tempo-01.mei (notes in beams)
// - notes_rests.mei (mensural durations)
// ============================================================================

// ----------------------------------------------------------------------------
// Tests from tchaikovsky_scherzo.mei
// ----------------------------------------------------------------------------

/// Note with staccato articulation child from Tchaikovsky scherzo
#[test]
fn mei_example_tchaikovsky_note_with_staccato() {
    let xml = r#"<note xml:id="n2apf6t" dur="8" pname="f" oct="5">
        <artic artic="stacc" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("n2apf6t".to_string()));
    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
    );
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("f".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(5)));

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Artic(artic) => {
            assert_eq!(artic.artic_log.artic[0], DataArticulation::Stacc);
        }
        other => panic!("Expected Artic, got {:?}", other),
    }
}

/// Note with tenuto and gestural flat from Tchaikovsky scherzo
#[test]
fn mei_example_tchaikovsky_note_with_tenuto_and_accid() {
    let xml = r#"<note xml:id="n1v2c23j" dur="4" pname="e" oct="5">
        <artic artic="ten" />
        <accid accid.ges="f" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("n1v2c23j".to_string()));
    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("e".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(5)));

    assert_eq!(note.children.len(), 2);

    // First child: tenuto articulation
    match &note.children[0] {
        tusk_model::elements::NoteChild::Artic(artic) => {
            assert_eq!(artic.artic_log.artic[0], DataArticulation::Ten);
        }
        other => panic!("Expected Artic, got {:?}", other),
    }

    // Second child: gestural flat accidental
    match &note.children[1] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_ges.accid_ges.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Note with only gestural accidental from Tchaikovsky scherzo
#[test]
fn mei_example_tchaikovsky_note_with_gestural_accid_only() {
    let xml = r#"<note xml:id="nz8c5kj" dur="8" pname="d" oct="5">
        <accid accid.ges="f" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("nz8c5kj".to_string()));
    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
    );
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("d".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(5)));

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_ges.accid_ges.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Quarter rest from Tchaikovsky scherzo
#[test]
fn mei_example_tchaikovsky_quarter_rest() {
    let xml = r#"<rest xml:id="r12gwbz0" dur="4" />"#;

    let rest = Rest::from_mei_str(xml).expect("should parse");

    assert_eq!(rest.common.xml_id, Some("r12gwbz0".to_string()));
    assert_eq!(
        rest.rest_log.dur,
        Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4))
    );
}

/// Eighth rest from Tchaikovsky scherzo
#[test]
fn mei_example_tchaikovsky_eighth_rest() {
    let xml = r#"<rest xml:id="r1e6h2le" dur="8" />"#;

    let rest = Rest::from_mei_str(xml).expect("should parse");

    assert_eq!(rest.common.xml_id, Some("r1e6h2le".to_string()));
    assert_eq!(
        rest.rest_log.dur,
        Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N8))
    );
}

/// Dotted quarter rest from Tchaikovsky scherzo
#[test]
fn mei_example_tchaikovsky_dotted_quarter_rest() {
    let xml = r#"<rest xml:id="r176694i" dots="1" dur="4" />"#;

    let rest = Rest::from_mei_str(xml).expect("should parse");

    assert_eq!(rest.common.xml_id, Some("r176694i".to_string()));
    assert_eq!(
        rest.rest_log.dur,
        Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(rest.rest_log.dots, Some(DataAugmentdot(1)));
}

/// Chord with two notes and gestural accidentals from Tchaikovsky scherzo
#[test]
fn mei_example_tchaikovsky_chord_with_two_notes() {
    use tusk_model::elements::Chord;

    let xml = r#"<chord xml:id="c1xfnie3" dots="1" dur="4">
        <note xml:id="n9stwxq" pname="d" oct="4">
            <accid accid.ges="f" />
        </note>
        <note xml:id="n103nrpj" pname="a" oct="4">
            <accid accid.ges="f" />
        </note>
    </chord>"#;

    let chord = Chord::from_mei_str(xml).expect("should parse");

    assert_eq!(chord.common.xml_id, Some("c1xfnie3".to_string()));
    assert_eq!(
        chord.chord_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(chord.chord_log.dots, Some(DataAugmentdot(1)));

    assert_eq!(chord.children.len(), 2);

    // First note: D4
    match &chord.children[0] {
        tusk_model::elements::ChordChild::Note(note) => {
            assert_eq!(note.common.xml_id, Some("n9stwxq".to_string()));
            assert_eq!(
                note.note_log.pname,
                Some(DataPitchname::from("d".to_string()))
            );
            assert_eq!(note.note_log.oct, Some(DataOctave(4)));
        }
        other => panic!("Expected Note, got {:?}", other),
    }

    // Second note: A4
    match &chord.children[1] {
        tusk_model::elements::ChordChild::Note(note) => {
            assert_eq!(note.common.xml_id, Some("n103nrpj".to_string()));
            assert_eq!(
                note.note_log.pname,
                Some(DataPitchname::from("a".to_string()))
            );
            assert_eq!(note.note_log.oct, Some(DataOctave(4)));
        }
        other => panic!("Expected Note, got {:?}", other),
    }
}

/// Chord with artic child from Tchaikovsky scherzo (measure 3)
#[test]
fn mei_example_tchaikovsky_chord_with_artic() {
    use tusk_model::elements::Chord;

    let xml = r#"<chord xml:id="c8kn0ob" dots="1" dur="4">
        <artic artic="acc" />
        <note xml:id="n1ao12g4" pname="a" oct="3">
            <accid accid.ges="f" />
        </note>
        <note xml:id="n1wm5qw9" pname="f" oct="4" />
    </chord>"#;

    let chord = Chord::from_mei_str(xml).expect("should parse");

    assert_eq!(chord.common.xml_id, Some("c8kn0ob".to_string()));
    assert_eq!(
        chord.chord_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(chord.chord_log.dots, Some(DataAugmentdot(1)));

    assert_eq!(chord.children.len(), 3);

    // First child: accent articulation
    match &chord.children[0] {
        tusk_model::elements::ChordChild::Artic(artic) => {
            assert_eq!(artic.artic_log.artic[0], DataArticulation::Acc);
        }
        other => panic!("Expected Artic, got {:?}", other),
    }

    // Second child: A3 note
    match &chord.children[1] {
        tusk_model::elements::ChordChild::Note(note) => {
            assert_eq!(
                note.note_log.pname,
                Some(DataPitchname::from("a".to_string()))
            );
            assert_eq!(note.note_log.oct, Some(DataOctave(3)));
        }
        other => panic!("Expected Note, got {:?}", other),
    }

    // Third child: F4 note
    match &chord.children[2] {
        tusk_model::elements::ChordChild::Note(note) => {
            assert_eq!(
                note.note_log.pname,
                Some(DataPitchname::from("f".to_string()))
            );
            assert_eq!(note.note_log.oct, Some(DataOctave(4)));
        }
        other => panic!("Expected Note, got {:?}", other),
    }
}

/// Note with dotted quarter and accent from Tchaikovsky scherzo
#[test]
fn mei_example_tchaikovsky_dotted_note_with_accent() {
    let xml = r#"<note xml:id="n2epqtj" dots="1" dur="4" pname="c" oct="5">
        <artic artic="acc" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("n2epqtj".to_string()));
    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(note.note_log.dots, Some(DataAugmentdot(1)));
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(5)));

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Artic(artic) => {
            assert_eq!(artic.artic_log.artic[0], DataArticulation::Acc);
        }
        other => panic!("Expected Artic, got {:?}", other),
    }
}

// ----------------------------------------------------------------------------
// Tests from accid-03.mei
// ----------------------------------------------------------------------------

/// Note with written sharp accidental from accid-03 example
#[test]
fn mei_example_accid03_note_with_sharp() {
    let xml = r#"<note dur="1" oct="5" pname="f">
        <accid accid="s" func="edit" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N1))
    );
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("f".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(5)));

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_log.accid.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Note with written flat accidental from accid-03 example
#[test]
fn mei_example_accid03_note_with_flat() {
    let xml = r#"<note dur="1" oct="5" pname="f">
        <accid accid="f" func="edit" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N1))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(5)));

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_log.accid.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Note with written natural accidental from accid-03 example
#[test]
fn mei_example_accid03_note_with_natural() {
    let xml = r#"<note dur="1" oct="5" pname="f">
        <accid accid="n" func="edit" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_log.accid.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Note with double sharp from accid-03 example
#[test]
fn mei_example_accid03_note_with_double_sharp() {
    let xml = r#"<note dur="1" oct="5" pname="f">
        <accid accid="x" func="edit" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_log.accid.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Note with double flat from accid-03 example
#[test]
fn mei_example_accid03_note_with_double_flat() {
    let xml = r#"<note dur="1" oct="5" pname="f">
        <accid accid="ff" func="edit" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_log.accid.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

// ----------------------------------------------------------------------------
// Tests from tempo-01.mei
// ----------------------------------------------------------------------------

/// Note from tempo example (self-closing element)
#[test]
fn mei_example_tempo01_note_self_closing() {
    let xml = r#"<note xml:id="m0_s2_e1" dur="8" oct="5" pname="e" />"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("m0_s2_e1".to_string()));
    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
    );
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("e".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(5)));
    assert!(note.children.is_empty());
}

/// Note with dots attribute (not dots child) from tempo example
#[test]
fn mei_example_tempo01_note_with_dots_attr() {
    let xml = r#"<note dots="1" dur="4" oct="5" pname="g" />"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(note.note_log.dots, Some(DataAugmentdot(1)));
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("g".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(5)));
}

// ----------------------------------------------------------------------------
// Tests from notes_rests.mei (mensural notation)
// ----------------------------------------------------------------------------

/// Note with mensural maxima duration from notes_rests example
#[test]
fn mei_example_notes_rests_maxima() {
    // Note: mensural durations use the same DataDuration type but different variants
    let xml = r#"<note dur="maxima" />"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    // Verify it parsed something for dur (may be mensural variant)
    assert!(note.note_log.dur.is_some());
}

/// Note with mensural longa duration from notes_rests example
#[test]
fn mei_example_notes_rests_longa() {
    let xml = r#"<note dur="longa" />"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert!(note.note_log.dur.is_some());
}

/// Note with mensural brevis duration from notes_rests example
#[test]
fn mei_example_notes_rests_brevis() {
    let xml = r#"<note dur="brevis" />"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert!(note.note_log.dur.is_some());
}

/// Note with mensural semibrevis duration from notes_rests example
#[test]
fn mei_example_notes_rests_semibrevis() {
    let xml = r#"<note dur="semibrevis" />"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert!(note.note_log.dur.is_some());
}

/// Note with mensural minima duration from notes_rests example
#[test]
fn mei_example_notes_rests_minima() {
    let xml = r#"<note dur="minima" />"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert!(note.note_log.dur.is_some());
}

/// Rest with mensural maxima duration from notes_rests example
#[test]
fn mei_example_notes_rests_rest_maxima() {
    let xml = r#"<rest dur="maxima" />"#;

    let rest = Rest::from_mei_str(xml).expect("should parse");

    assert!(rest.rest_log.dur.is_some());
}

/// Rest with mensural longa duration from notes_rests example
#[test]
fn mei_example_notes_rests_rest_longa() {
    let xml = r#"<rest dur="longa" />"#;

    let rest = Rest::from_mei_str(xml).expect("should parse");

    assert!(rest.rest_log.dur.is_some());
}

/// Rest with mensural brevis duration from notes_rests example
#[test]
fn mei_example_notes_rests_rest_brevis() {
    let xml = r#"<rest dur="brevis" />"#;

    let rest = Rest::from_mei_str(xml).expect("should parse");

    assert!(rest.rest_log.dur.is_some());
}

// ----------------------------------------------------------------------------
// Edge case tests from real MEI files
// ----------------------------------------------------------------------------

/// Note without xml:id (common in hand-written MEI)
#[test]
fn mei_example_note_without_id() {
    let xml = r#"<note dur="8" pname="g" oct="4" />"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert!(note.common.xml_id.is_none());
    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
    );
}

/// Rest without xml:id
#[test]
fn mei_example_rest_without_id() {
    let xml = r#"<rest dur="2" />"#;

    let rest = Rest::from_mei_str(xml).expect("should parse");

    assert!(rest.common.xml_id.is_none());
    assert_eq!(
        rest.rest_log.dur,
        Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2))
    );
}

/// Chord without xml:id
#[test]
fn mei_example_chord_without_id() {
    use tusk_model::elements::Chord;

    let xml = r#"<chord dur="4">
        <note pname="c" oct="4" />
        <note pname="e" oct="4" />
    </chord>"#;

    let chord = Chord::from_mei_str(xml).expect("should parse");

    assert!(chord.common.xml_id.is_none());
    assert_eq!(chord.children.len(), 2);
}

/// Note with written sharp accidental (like C# in Tchaikovsky)
#[test]
fn mei_example_tchaikovsky_written_sharp() {
    let xml = r#"<note xml:id="n1jlp1q2" pname="c" oct="4">
        <accid accid="s" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("n1jlp1q2".to_string()));
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(4)));

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_log.accid.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Note with both accent and gestural accidental
#[test]
fn mei_example_tchaikovsky_accent_and_accid() {
    let xml = r#"<note xml:id="ni3fhhf" dots="1" dur="4" pname="a" oct="2">
        <artic artic="acc" />
        <accid accid.ges="f" />
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("ni3fhhf".to_string()));
    assert_eq!(note.note_log.dots, Some(DataAugmentdot(1)));
    assert_eq!(
        note.note_log.pname,
        Some(DataPitchname::from("a".to_string()))
    );
    assert_eq!(note.note_log.oct, Some(DataOctave(2)));

    assert_eq!(note.children.len(), 2);

    match &note.children[0] {
        tusk_model::elements::NoteChild::Artic(artic) => {
            assert_eq!(artic.artic_log.artic[0], DataArticulation::Acc);
        }
        other => panic!("Expected Artic, got {:?}", other),
    }

    match &note.children[1] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_ges.accid_ges.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

// ----------------------------------------------------------------------------
// Complex real-world scenarios
// ----------------------------------------------------------------------------

/// Multiple notes in sequence (simulating layer content)
#[test]
fn mei_example_multiple_notes_sequence() {
    // Parse multiple notes individually (as our parser handles single elements)
    let notes = [
        r#"<note xml:id="n1" dur="8" pname="f" oct="5"><artic artic="stacc"/></note>"#,
        r#"<note xml:id="n2" dur="8" pname="f" oct="5"><artic artic="stacc"/></note>"#,
        r#"<note xml:id="n3" dur="8" pname="f" oct="5"><artic artic="stacc"/></note>"#,
    ];

    for (i, xml) in notes.iter().enumerate() {
        let note = Note::from_mei_str(xml).expect("should parse");
        assert_eq!(note.common.xml_id, Some(format!("n{}", i + 1)));
        assert_eq!(
            note.note_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
        );
        assert_eq!(note.children.len(), 1);
    }
}

/// Notes with varying octaves (testing octave range)
#[test]
fn mei_example_notes_various_octaves() {
    for oct in 0u64..=9 {
        let xml = format!(r#"<note dur="4" pname="c" oct="{}" />"#, oct);
        let note = Note::from_mei_str(&xml).expect("should parse");
        assert_eq!(note.note_log.oct, Some(DataOctave(oct)));
    }
}

/// Notes with all pitch names
#[test]
fn mei_example_notes_all_pitch_names() {
    for pname in ["a", "b", "c", "d", "e", "f", "g"] {
        let xml = format!(r#"<note dur="4" pname="{}" oct="4" />"#, pname);
        let note = Note::from_mei_str(&xml).expect("should parse");
        assert_eq!(
            note.note_log.pname,
            Some(DataPitchname::from(pname.to_string()))
        );
    }
}

/// Chord with three notes (triad)
#[test]
fn mei_example_triad_chord() {
    use tusk_model::elements::Chord;

    let xml = r#"<chord xml:id="c1" dur="2">
        <note pname="c" oct="4" />
        <note pname="e" oct="4" />
        <note pname="g" oct="4" />
    </chord>"#;

    let chord = Chord::from_mei_str(xml).expect("should parse");

    assert_eq!(chord.common.xml_id, Some("c1".to_string()));
    assert_eq!(
        chord.chord_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N2))
    );
    assert_eq!(chord.children.len(), 3);

    // Verify C major triad (C-E-G)
    let expected_pitches = ["c", "e", "g"];
    for (i, expected_pname) in expected_pitches.iter().enumerate() {
        match &chord.children[i] {
            tusk_model::elements::ChordChild::Note(note) => {
                assert_eq!(
                    note.note_log.pname,
                    Some(DataPitchname::from(expected_pname.to_string()))
                );
            }
            other => panic!("Expected Note at index {}, got {:?}", i, other),
        }
    }
}

// ============================================================================
// Measure Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_measure() {
    use tusk_model::elements::Measure;

    let original = Measure::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.measure_log.right.is_none());
    assert!(parsed.measure_log.left.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_measure_with_xml_id() {
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"m1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
}

#[test]
fn roundtrip_measure_with_n_attribute() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());
    original.common.n = Some(DataWord("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("n=\"1\""), "xml should contain n: {}", xml);

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));
}

#[test]
fn roundtrip_measure_with_barline_right() {
    use tusk_model::data::DataBarrendition;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());
    original.measure_log.right = Some(DataBarrendition::Dbl);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("right=\"dbl\""),
        "xml should contain right: {}",
        xml
    );

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.measure_log.right, Some(DataBarrendition::Dbl));
}

#[test]
fn roundtrip_measure_with_barline_left() {
    use tusk_model::data::DataBarrendition;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_log.left = Some(DataBarrendition::Rptstart);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.measure_log.left, Some(DataBarrendition::Rptstart));
}

#[test]
fn roundtrip_measure_with_metcon() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_log.metcon = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.measure_log.metcon, Some(DataBoolean::True));
}

#[test]
fn roundtrip_measure_with_control() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_log.control = Some(DataBoolean::False);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.measure_log.control, Some(DataBoolean::False));
}

#[test]
fn roundtrip_measure_with_visual_width() {
    use tusk_model::data::DataMeasurementunsigned;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_vis.width = Some(DataMeasurementunsigned("100vu".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.measure_vis.width,
        Some(DataMeasurementunsigned("100vu".to_string()))
    );
}

#[test]
fn roundtrip_measure_with_bar_len() {
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_vis.bar_len = Some(8.0);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.measure_vis.bar_len, Some(8.0));
}

#[test]
fn roundtrip_measure_with_gestural_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_ges.tstamp_ges = Some(DataBeat(0.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.measure_ges.tstamp_ges.is_some());
}

#[test]
fn roundtrip_measure_with_staff_child() {
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("s1".to_string());
    staff.n_integer.n = Some(1);

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());
    original.children.push(MeasureChild::Staff(Box::new(staff)));

    let xml = original.to_mei_string().expect("serialize");

    // Verify the serialized XML contains the staff child
    assert!(
        xml.contains("<staff"),
        "should contain staff element: {}",
        xml
    );
    assert!(
        xml.contains("</measure>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MeasureChild::Staff(staff) => {
            assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn roundtrip_measure_with_multiple_staff_children() {
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff1 = Staff::default();
    staff1.basic.xml_id = Some("s1".to_string());
    staff1.n_integer.n = Some(1);

    let mut staff2 = Staff::default();
    staff2.basic.xml_id = Some("s2".to_string());
    staff2.n_integer.n = Some(2);

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());
    original
        .children
        .push(MeasureChild::Staff(Box::new(staff1)));
    original
        .children
        .push(MeasureChild::Staff(Box::new(staff2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);
}

#[test]
fn roundtrip_measure_complete_cmn() {
    use tusk_model::data::{DataBarrendition, DataBoolean, DataWord};
    use tusk_model::elements::Measure;

    // Common Music Notation measure with all typical attributes
    let mut original = Measure::default();
    original.common.xml_id = Some("m42".to_string());
    original.common.n = Some(DataWord("42".to_string()));
    original.measure_log.right = Some(DataBarrendition::Single);
    original.measure_log.metcon = Some(DataBoolean::True);
    original.measure_log.control = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.common.n, original.common.n);
    assert_eq!(parsed.measure_log.right, original.measure_log.right);
    assert_eq!(parsed.measure_log.metcon, original.measure_log.metcon);
    assert_eq!(parsed.measure_log.control, original.measure_log.control);
}

#[test]
fn measure_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Measure;

    let xml = r#"<measure xml:id="m1" unknown="value" n="1"/>"#;
    let measure = Measure::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
}

#[test]
fn measure_ignores_unknown_child_elements() {
    use tusk_model::elements::Measure;

    let xml = r#"<measure xml:id="m1"><unknownElement/></measure>"#;
    let measure = Measure::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
    assert!(measure.children.is_empty());
}

#[test]
fn measure_deserializes_with_xml_declaration() {
    use tusk_model::elements::Measure;

    let xml = r#"<?xml version="1.0"?><measure xml:id="m1" n="1"/>"#;
    let measure = Measure::from_mei_str(xml).expect("should deserialize");

    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
}

// ============================================================================
// Staff Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_staff() {
    use tusk_model::elements::Staff;

    let original = Staff::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n_integer.n.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_staff_with_xml_id() {
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.basic.xml_id = Some("s1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"s1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Staff::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.basic.xml_id, Some("s1".to_string()));
}

#[test]
fn roundtrip_staff_with_n_attribute() {
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.basic.xml_id = Some("s1".to_string());
    original.n_integer.n = Some(1);

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("n=\"1\""), "xml should contain n: {}", xml);

    let parsed = Staff::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn roundtrip_staff_with_label() {
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.labelled.label = Some("Violin I".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.labelled.label, Some("Violin I".to_string()));
}

#[test]
fn roundtrip_staff_with_visible_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.staff_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.staff_vis.visible, Some(DataBoolean::True));
}

#[test]
fn roundtrip_staff_with_def_attribute() {
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.staff_log.def = Some(tusk_model::data::DataUri("staffdef1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.staff_log.def.is_some());
}

#[test]
fn roundtrip_staff_with_metcon() {
    use tusk_model::elements::Staff;

    // Parse from XML to test metcon attribute deserialization
    let xml = r#"<staff n="1" metcon="c" />"#;
    let parsed = Staff::from_mei_str(xml).expect("deserialize");

    assert!(parsed.staff_log.metcon.is_some());

    // Serialize and verify round-trip
    let reserialized = parsed.to_mei_string().expect("serialize");
    assert!(
        reserialized.contains("metcon=\"c\""),
        "metcon should be preserved: {}",
        reserialized
    );
}

#[test]
fn roundtrip_staff_complete_cmn() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Staff;

    // Common Music Notation staff with all typical attributes
    let mut original = Staff::default();
    original.basic.xml_id = Some("s1".to_string());
    original.n_integer.n = Some(1);
    original.labelled.label = Some("Piano".to_string());
    original.staff_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, original.basic.xml_id);
    assert_eq!(parsed.n_integer.n, original.n_integer.n);
    assert_eq!(parsed.labelled.label, original.labelled.label);
    assert_eq!(parsed.staff_vis.visible, original.staff_vis.visible);
}

#[test]
fn staff_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff xml:id="s1" unknown="value" n="1"/>"#;
    let staff = Staff::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
    assert_eq!(staff.n_integer.n, Some(1));
}

#[test]
fn staff_deserializes_with_xml_declaration() {
    use tusk_model::elements::Staff;

    let xml = r#"<?xml version="1.0"?><staff xml:id="s1" n="1"/>"#;
    let staff = Staff::from_mei_str(xml).expect("should deserialize");

    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
}

#[test]
fn staff_ignores_unknown_child_elements() {
    use tusk_model::elements::Staff;

    // Staff with unknown child element should parse gracefully
    let xml = r#"<staff xml:id="s1"><unknownElement/></staff>"#;
    let staff = Staff::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
    // Children should be empty since we skip unknown children
    assert!(staff.children.is_empty());
}

// ============================================================================
// Staff External XML Parsing Tests
// ============================================================================

#[test]
fn parse_external_staff_minimal() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff/>"#;
    let parsed = Staff::from_mei_str(xml).expect("deserialize");

    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Staff::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.basic.xml_id.is_none());
}

#[test]
fn parse_external_staff_with_attributes() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff xml:id="s1" n="1"/>"#;
    let parsed = Staff::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("s1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Staff::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.basic.xml_id, Some("s1".to_string()));
    assert_eq!(reparsed.n_integer.n, Some(1));
}

#[test]
fn parse_external_staff_various_n_values() {
    use tusk_model::elements::Staff;

    // Test various staff numbers
    for n in 1u64..=10 {
        let xml = format!(r#"<staff n="{}"/>"#, n);
        let parsed = Staff::from_mei_str(&xml).expect("should parse");
        assert_eq!(parsed.n_integer.n, Some(n));

        let reserialized = parsed.to_mei_string().expect("re-serialize");
        let reparsed = Staff::from_mei_str(&reserialized).expect("re-deserialize");
        assert_eq!(reparsed.n_integer.n, Some(n));
    }
}

// ============================================================================
// Tests from MEI Example Files
// ============================================================================

/// Staff from tempo-01.mei
#[test]
fn mei_example_tempo01_staff() {
    use tusk_model::elements::Staff;

    // From specs/mei/examples/verovio/tempo-01.mei
    // Note: Layer children are not yet parsed (next task), so we just verify attributes
    let xml = r#"<staff n="1">
                <layer n="1">
                  <note dots="1" dur="4" oct="5" pname="g" />
                  <note dur="8" oct="5" pname="g" />
                </layer>
              </staff>"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert_eq!(staff.n_integer.n, Some(1));
    // Children are skipped for now until layer parsing is implemented
}

/// Staff with multiple layers (from Tchaikovsky example pattern)
#[test]
fn mei_example_staff_structure() {
    use tusk_model::elements::Staff;

    // Structure from typical CMN MEI files
    let xml = r#"<staff xml:id="s1" n="1" label="Piano Right Hand">
        <layer n="1">
            <note dur="4" pname="c" oct="5"/>
        </layer>
    </staff>"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
    assert_eq!(staff.n_integer.n, Some(1));
    assert_eq!(staff.labelled.label, Some("Piano Right Hand".to_string()));
}

/// Self-closing staff element
#[test]
fn mei_example_staff_self_closing() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff xml:id="s1" n="1" />"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
    assert_eq!(staff.n_integer.n, Some(1));
    assert!(staff.children.is_empty());
}

/// Staff without xml:id (common pattern)
#[test]
fn mei_example_staff_without_id() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff n="2" />"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert!(staff.basic.xml_id.is_none());
    assert_eq!(staff.n_integer.n, Some(2));
}

/// Staff visibility attribute
#[test]
fn mei_example_staff_hidden() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Staff;

    let xml = r#"<staff n="1" visible="false" />"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert_eq!(staff.staff_vis.visible, Some(DataBoolean::False));
}

// ============================================================================
// Staff in Measure Context Tests
// ============================================================================

/// Test that Staff parsed as child of Measure round-trips correctly
#[test]
fn roundtrip_staff_in_measure_context() {
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("s1".to_string());
    staff.n_integer.n = Some(1);
    staff.labelled.label = Some("Violin".to_string());

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let xml = measure.to_mei_string().expect("serialize");

    // Verify structure
    assert!(xml.contains("<measure"), "should have measure: {}", xml);
    assert!(xml.contains("<staff"), "should have staff: {}", xml);
    assert!(
        xml.contains("label=\"Violin\""),
        "should have label: {}",
        xml
    );

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MeasureChild::Staff(staff) => {
            assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
            assert_eq!(staff.n_integer.n, Some(1));
            assert_eq!(staff.labelled.label, Some("Violin".to_string()));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

/// Multiple staves in a measure
#[test]
fn roundtrip_multiple_staves_in_measure() {
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff1 = Staff::default();
    staff1.basic.xml_id = Some("s1".to_string());
    staff1.n_integer.n = Some(1);
    staff1.labelled.label = Some("Violin I".to_string());

    let mut staff2 = Staff::default();
    staff2.basic.xml_id = Some("s2".to_string());
    staff2.n_integer.n = Some(2);
    staff2.labelled.label = Some("Violin II".to_string());

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.children.push(MeasureChild::Staff(Box::new(staff1)));
    measure.children.push(MeasureChild::Staff(Box::new(staff2)));

    let xml = measure.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // First staff
    match &parsed.children[0] {
        MeasureChild::Staff(staff) => {
            assert_eq!(staff.n_integer.n, Some(1));
            assert_eq!(staff.labelled.label, Some("Violin I".to_string()));
        }
        other => panic!("Expected Staff 1, got {:?}", other),
    }

    // Second staff
    match &parsed.children[1] {
        MeasureChild::Staff(staff) => {
            assert_eq!(staff.n_integer.n, Some(2));
            assert_eq!(staff.labelled.label, Some("Violin II".to_string()));
        }
        other => panic!("Expected Staff 2, got {:?}", other),
    }
}

// ============================================================================
// Layer Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_layer() {
    use tusk_model::elements::Layer;

    let original = Layer::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n_integer.n.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_layer_with_xml_id() {
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.basic.xml_id = Some("l1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"l1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Layer::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.basic.xml_id, Some("l1".to_string()));
}

#[test]
fn roundtrip_layer_with_n_attribute() {
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.basic.xml_id = Some("l1".to_string());
    original.n_integer.n = Some(1);

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("n=\"1\""), "xml should contain n: {}", xml);

    let parsed = Layer::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn roundtrip_layer_with_label() {
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.labelled.label = Some("Voice 1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.labelled.label, Some("Voice 1".to_string()));
}

#[test]
fn roundtrip_layer_with_visible_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.layer_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.layer_vis.visible, Some(DataBoolean::True));
}

#[test]
fn roundtrip_layer_with_def_attribute() {
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.layer_log.def = Some(tusk_model::data::DataUri("layerdef1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.layer_log.def.is_some());
}

#[test]
fn roundtrip_layer_with_metcon() {
    use tusk_model::att::AttLayerLogMetcon;
    use tusk_model::elements::Layer;

    // Parse from XML to test metcon attribute deserialization
    let xml = r#"<layer n="1" metcon="c" />"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.layer_log.metcon, Some(AttLayerLogMetcon::C));

    // Serialize and verify round-trip
    let reserialized = parsed.to_mei_string().expect("serialize");
    assert!(
        reserialized.contains("metcon=\"c\""),
        "metcon should be preserved: {}",
        reserialized
    );
}

#[test]
fn roundtrip_layer_with_cue() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.layer_log.cue = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("cue=\"true\""),
        "should contain cue attribute: {}",
        xml
    );

    let parsed = Layer::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.layer_log.cue, Some(DataBoolean::True));
}

#[test]
fn roundtrip_layer_complete_cmn() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Layer;

    // Common Music Notation layer with all typical attributes
    let mut original = Layer::default();
    original.basic.xml_id = Some("l1".to_string());
    original.n_integer.n = Some(1);
    original.labelled.label = Some("Voice 1".to_string());
    original.layer_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, original.basic.xml_id);
    assert_eq!(parsed.n_integer.n, original.n_integer.n);
    assert_eq!(parsed.labelled.label, original.labelled.label);
    assert_eq!(parsed.layer_vis.visible, original.layer_vis.visible);
}

#[test]
fn layer_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer xml:id="l1" unknown="value" n="1"/>"#;
    let layer = Layer::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(layer.basic.xml_id, Some("l1".to_string()));
    assert_eq!(layer.n_integer.n, Some(1));
}

#[test]
fn layer_deserializes_with_xml_declaration() {
    use tusk_model::elements::Layer;

    let xml = r#"<?xml version="1.0"?><layer xml:id="l1" n="1"/>"#;
    let layer = Layer::from_mei_str(xml).expect("should deserialize");

    assert_eq!(layer.basic.xml_id, Some("l1".to_string()));
}

#[test]
fn layer_ignores_unknown_child_elements() {
    use tusk_model::elements::Layer;

    // Layer with unknown child element should parse gracefully
    let xml = r#"<layer xml:id="l1"><unknownElement/></layer>"#;
    let layer = Layer::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(layer.basic.xml_id, Some("l1".to_string()));
    // Children should be empty since we skip unknown children
    assert!(layer.children.is_empty());
}

// ============================================================================
// Layer with Child Elements Tests
// ============================================================================

#[test]
fn roundtrip_layer_with_note_child() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note};

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));

    let mut original = Layer::default();
    original.basic.xml_id = Some("l1".to_string());
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Note(Box::new(note)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("l1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Note(note) => {
            assert_eq!(note.common.xml_id, Some("n1".to_string()));
            assert_eq!(
                note.note_log.dur,
                Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
            );
        }
        other => panic!("Expected Note child, got {:?}", other),
    }
}

#[test]
fn roundtrip_layer_with_rest_child() {
    use tusk_model::data::{DataDurationCmn, DataDurationrests};
    use tusk_model::elements::{Layer, LayerChild, Rest};

    let mut rest = Rest::default();
    rest.common.xml_id = Some("r1".to_string());
    rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));

    let mut original = Layer::default();
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Rest(Box::new(rest)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Rest(rest) => {
            assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        }
        other => panic!("Expected Rest child, got {:?}", other),
    }
}

#[test]
fn roundtrip_layer_with_chord_child() {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_model::elements::{Chord, Layer, LayerChild};

    let mut chord = Chord::default();
    chord.common.xml_id = Some("c1".to_string());
    chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut original = Layer::default();
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Chord(Box::new(chord)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Chord(chord) => {
            assert_eq!(chord.common.xml_id, Some("c1".to_string()));
        }
        other => panic!("Expected Chord child, got {:?}", other),
    }
}

#[test]
fn roundtrip_layer_with_space_child() {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_model::elements::{Layer, LayerChild, Space};

    let mut space = Space::default();
    space.common.xml_id = Some("s1".to_string());
    space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut original = Layer::default();
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Space(Box::new(space)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Space(space) => {
            assert_eq!(space.common.xml_id, Some("s1".to_string()));
        }
        other => panic!("Expected Space child, got {:?}", other),
    }
}

#[test]
fn roundtrip_layer_with_multiple_children() {
    use tusk_model::data::{
        DataDuration, DataDurationCmn, DataDurationrests, DataOctave, DataPitchname,
    };
    use tusk_model::elements::{Layer, LayerChild, Note, Rest};

    let mut note1 = Note::default();
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));

    let mut note2 = Note::default();
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note2.note_log.pname = Some(DataPitchname::from("d".to_string()));
    note2.note_log.oct = Some(DataOctave(4));

    let mut rest = Rest::default();
    rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2));

    let mut original = Layer::default();
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Note(Box::new(note1)));
    original.children.push(LayerChild::Note(Box::new(note2)));
    original.children.push(LayerChild::Rest(Box::new(rest)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 3);

    // Verify order is preserved
    match &parsed.children[0] {
        LayerChild::Note(note) => {
            assert_eq!(
                note.note_log.pname,
                Some(DataPitchname::from("c".to_string()))
            );
        }
        other => panic!("Expected Note 1, got {:?}", other),
    }

    match &parsed.children[1] {
        LayerChild::Note(note) => {
            assert_eq!(
                note.note_log.pname,
                Some(DataPitchname::from("d".to_string()))
            );
        }
        other => panic!("Expected Note 2, got {:?}", other),
    }

    match &parsed.children[2] {
        LayerChild::Rest(_) => {}
        other => panic!("Expected Rest, got {:?}", other),
    }
}

// ============================================================================
// Layer External XML Parsing Tests
// ============================================================================

#[test]
fn parse_external_layer_minimal() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer/>"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Layer::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.basic.xml_id.is_none());
}

#[test]
fn parse_external_layer_with_attributes() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer xml:id="l1" n="1"/>"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("l1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Layer::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.basic.xml_id, Some("l1".to_string()));
    assert_eq!(reparsed.n_integer.n, Some(1));
}

#[test]
fn parse_external_layer_various_n_values() {
    use tusk_model::elements::Layer;

    for n in [1, 2, 3, 10] {
        let xml = format!(r#"<layer n="{}"/>"#, n);
        let parsed = Layer::from_mei_str(&xml).expect("deserialize");
        assert_eq!(parsed.n_integer.n, Some(n));
    }
}

#[test]
fn mei_example_layer_structure() {
    use tusk_model::elements::{Layer, LayerChild};

    // Based on specs/mei/examples/usersymbols/usersymbols-sample347.txt
    let xml = r#"<layer n="1">
        <rest dur="4" xml:id="r1"/>
        <note dur="8" oct="4" pname="c" xml:id="n1"/>
    </layer>"#;

    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 2);

    // First child should be rest
    match &parsed.children[0] {
        LayerChild::Rest(rest) => {
            assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        }
        other => panic!("Expected Rest, got {:?}", other),
    }

    // Second child should be note
    match &parsed.children[1] {
        LayerChild::Note(note) => {
            assert_eq!(note.common.xml_id, Some("n1".to_string()));
        }
        other => panic!("Expected Note, got {:?}", other),
    }
}

#[test]
fn mei_example_layer_self_closing() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer n="1"/>"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
    assert!(parsed.children.is_empty());
}

#[test]
fn mei_example_layer_without_id() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer n="2"><rest dur="4"/></layer>"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert_eq!(parsed.n_integer.n, Some(2));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_layer_in_staff_context() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note, Staff, StaffChild};

    // Create a note
    let mut note = Note::default();
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));

    // Create a layer containing the note
    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    // Create a staff containing the layer
    let mut staff = Staff::default();
    staff.basic.xml_id = Some("s1".to_string());
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let xml = staff.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("s1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        StaffChild::Layer(layer) => {
            assert_eq!(layer.n_integer.n, Some(1));
            assert_eq!(layer.children.len(), 1);

            match &layer.children[0] {
                LayerChild::Note(note) => {
                    assert_eq!(
                        note.note_log.pname,
                        Some(DataPitchname::from("c".to_string()))
                    );
                }
                other => panic!("Expected Note, got {:?}", other),
            }
        }
        other => panic!("Expected Layer, got {:?}", other),
    }
}

#[test]
fn roundtrip_multiple_layers_in_staff() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note, Staff, StaffChild};

    // First layer with note
    let mut note1 = Note::default();
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));

    let mut layer1 = Layer::default();
    layer1.n_integer.n = Some(1);
    layer1.children.push(LayerChild::Note(Box::new(note1)));

    // Second layer with different note
    let mut note2 = Note::default();
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2));
    note2.note_log.pname = Some(DataPitchname::from("g".to_string()));
    note2.note_log.oct = Some(DataOctave(3));

    let mut layer2 = Layer::default();
    layer2.n_integer.n = Some(2);
    layer2.children.push(LayerChild::Note(Box::new(note2)));

    // Staff with both layers
    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer1)));
    staff.children.push(StaffChild::Layer(Box::new(layer2)));

    let xml = staff.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // First layer
    match &parsed.children[0] {
        StaffChild::Layer(layer) => {
            assert_eq!(layer.n_integer.n, Some(1));
        }
        other => panic!("Expected Layer 1, got {:?}", other),
    }

    // Second layer
    match &parsed.children[1] {
        StaffChild::Layer(layer) => {
            assert_eq!(layer.n_integer.n, Some(2));
        }
        other => panic!("Expected Layer 2, got {:?}", other),
    }
}

// ============================================================================
// Section Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_section() {
    use tusk_model::elements::Section;

    let original = Section::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_section_with_xml_id() {
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"sec1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Section::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
}

#[test]
fn roundtrip_section_with_n_attribute() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original.common.n = Some(DataWord("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("n=\"1\""), "xml should contain n: {}", xml);

    let parsed = Section::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));
}

#[test]
fn roundtrip_section_with_label() {
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.common.label = Some("Introduction".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("Introduction".to_string()));
}

#[test]
fn roundtrip_section_with_restart_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.section_vis.restart = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.section_vis.restart, Some(DataBoolean::True));
}

#[test]
fn roundtrip_section_with_attacca_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.section_ges.attacca = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.section_ges.attacca, Some(DataBoolean::True));
}

#[test]
fn roundtrip_section_with_measure_child() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, Section, SectionChild};

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    let xml = original.to_mei_string().expect("serialize");

    assert!(xml.contains("<section"), "should have section: {}", xml);
    assert!(xml.contains("<measure"), "should have measure: {}", xml);
    assert!(
        xml.contains("</section>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Measure(measure) => {
            assert_eq!(measure.common.xml_id, Some("m1".to_string()));
        }
        other => panic!("Expected Measure, got {:?}", other),
    }
}

#[test]
fn roundtrip_section_with_multiple_measure_children() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, Section, SectionChild};

    let mut measure1 = Measure::default();
    measure1.common.xml_id = Some("m1".to_string());
    measure1.common.n = Some(DataWord("1".to_string()));

    let mut measure2 = Measure::default();
    measure2.common.xml_id = Some("m2".to_string());
    measure2.common.n = Some(DataWord("2".to_string()));

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original
        .children
        .push(SectionChild::Measure(Box::new(measure1)));
    original
        .children
        .push(SectionChild::Measure(Box::new(measure2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);
}

#[test]
fn roundtrip_section_with_staff_child() {
    use tusk_model::elements::{Section, SectionChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("s1".to_string());
    staff.n_integer.n = Some(1);

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original.children.push(SectionChild::Staff(Box::new(staff)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Staff(staff) => {
            assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn roundtrip_section_with_nested_section() {
    use tusk_model::elements::{Section, SectionChild};

    let mut inner_section = Section::default();
    inner_section.common.xml_id = Some("sec2".to_string());

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original
        .children
        .push(SectionChild::Section(Box::new(inner_section)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Section(section) => {
            assert_eq!(section.common.xml_id, Some("sec2".to_string()));
        }
        other => panic!("Expected nested Section, got {:?}", other),
    }
}

#[test]
fn roundtrip_section_complete_cmn() {
    use tusk_model::data::{DataBoolean, DataWord};
    use tusk_model::elements::Section;

    // Common Music Notation section with all typical attributes
    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original.common.n = Some(DataWord("1".to_string()));
    original.common.label = Some("First Section".to_string());
    original.section_vis.restart = Some(DataBoolean::False);
    original.section_ges.attacca = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.common.n, original.common.n);
    assert_eq!(parsed.common.label, original.common.label);
    assert_eq!(parsed.section_vis.restart, original.section_vis.restart);
    assert_eq!(parsed.section_ges.attacca, original.section_ges.attacca);
}

#[test]
fn section_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Section;

    let xml = r#"<section xml:id="sec1" unknown="value" n="1"/>"#;
    let section = Section::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
}

#[test]
fn section_ignores_unknown_child_elements() {
    use tusk_model::elements::Section;

    let xml = r#"<section xml:id="sec1"><unknownElement/></section>"#;
    let section = Section::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
    assert!(section.children.is_empty());
}

#[test]
fn section_deserializes_with_xml_declaration() {
    use tusk_model::elements::Section;

    let xml = r#"<?xml version="1.0"?><section xml:id="sec1" n="1"/>"#;
    let section = Section::from_mei_str(xml).expect("should deserialize");

    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
}

// ============================================================================
// Section External XML Parsing Tests
// ============================================================================

#[test]
fn parse_external_section_minimal() {
    use tusk_model::elements::Section;

    let xml = r#"<section/>"#;
    let parsed = Section::from_mei_str(xml).expect("deserialize");

    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Section::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.common.xml_id.is_none());
}

#[test]
fn parse_external_section_with_attributes() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Section;

    let xml = r#"<section xml:id="sec1" n="1"/>"#;
    let parsed = Section::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Section::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(reparsed.common.n, Some(DataWord("1".to_string())));
}

#[test]
fn mei_example_section_structure() {
    use tusk_model::elements::{Section, SectionChild};

    // Basic section structure
    let xml = r#"<section xml:id="section1" label="Movement I">
        <measure xml:id="m1" n="1"/>
        <measure xml:id="m2" n="2"/>
    </section>"#;

    let parsed = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(parsed.common.xml_id, Some("section1".to_string()));
    assert_eq!(parsed.common.label, Some("Movement I".to_string()));
    assert_eq!(parsed.children.len(), 2);

    // First measure
    match &parsed.children[0] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m1".to_string()));
        }
        other => panic!("Expected Measure 1, got {:?}", other),
    }

    // Second measure
    match &parsed.children[1] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m2".to_string()));
        }
        other => panic!("Expected Measure 2, got {:?}", other),
    }
}

#[test]
fn mei_example_section_self_closing() {
    use tusk_model::elements::Section;

    let xml = r#"<section xml:id="sec1" n="1" />"#;
    let section = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
    assert!(section.children.is_empty());
}

#[test]
fn mei_example_section_without_id() {
    use tusk_model::elements::Section;

    let xml = r#"<section n="2" />"#;
    let section = Section::from_mei_str(xml).expect("should parse");

    assert!(section.common.xml_id.is_none());
}

#[test]
fn mei_example_section_with_restart() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Section;

    // Section with restart attribute (indicates staves restart)
    let xml = r#"<section xml:id="sec1" restart="true" />"#;
    let section = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(section.section_vis.restart, Some(DataBoolean::True));
}

#[test]
fn mei_example_section_with_attacca() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Section;

    // Section with attacca attribute (indicates next section should begin immediately)
    let xml = r#"<section xml:id="sec1" attacca="true" />"#;
    let section = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(section.section_ges.attacca, Some(DataBoolean::True));
}

// ============================================================================
// Mdiv Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_mdiv() {
    use tusk_model::elements::Mdiv;

    let original = Mdiv::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_mdiv_with_xml_id() {
    use tusk_model::elements::Mdiv;

    let mut original = Mdiv::default();
    original.common.xml_id = Some("mdiv1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"mdiv1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("mdiv1".to_string()));
}

#[test]
fn roundtrip_mdiv_with_n_attribute() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Mdiv;

    let mut original = Mdiv::default();
    original.common.n = Some(DataWord("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));
}

#[test]
fn roundtrip_mdiv_with_label() {
    use tusk_model::elements::Mdiv;

    let mut original = Mdiv::default();
    original.common.label = Some("Movement I".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("Movement I".to_string()));
}

#[test]
fn roundtrip_mdiv_with_attacca_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Mdiv;

    let mut original = Mdiv::default();
    original.mdiv_ges.attacca = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.mdiv_ges.attacca, Some(DataBoolean::True));
}

#[test]
fn roundtrip_mdiv_with_nested_mdiv() {
    use tusk_model::elements::{Mdiv, MdivChild};

    let mut inner_mdiv = Mdiv::default();
    inner_mdiv.common.xml_id = Some("mdiv2".to_string());
    inner_mdiv.common.label = Some("Movement I-A".to_string());

    let mut original = Mdiv::default();
    original.common.xml_id = Some("mdiv1".to_string());
    original
        .children
        .push(MdivChild::Mdiv(Box::new(inner_mdiv)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mdiv1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MdivChild::Mdiv(nested) => {
            assert_eq!(nested.common.xml_id, Some("mdiv2".to_string()));
            assert_eq!(nested.common.label, Some("Movement I-A".to_string()));
        }
        other => panic!("Expected nested Mdiv, got {:?}", other),
    }
}

#[test]
fn roundtrip_mdiv_with_multiple_nested_mdivs() {
    use tusk_model::elements::{Mdiv, MdivChild};

    let mut mdiv_a = Mdiv::default();
    mdiv_a.common.xml_id = Some("mdiv-a".to_string());

    let mut mdiv_b = Mdiv::default();
    mdiv_b.common.xml_id = Some("mdiv-b".to_string());

    let mut original = Mdiv::default();
    original.common.xml_id = Some("mdiv-root".to_string());
    original.children.push(MdivChild::Mdiv(Box::new(mdiv_a)));
    original.children.push(MdivChild::Mdiv(Box::new(mdiv_b)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        MdivChild::Mdiv(nested) => {
            assert_eq!(nested.common.xml_id, Some("mdiv-a".to_string()));
        }
        other => panic!("Expected Mdiv, got {:?}", other),
    }

    match &parsed.children[1] {
        MdivChild::Mdiv(nested) => {
            assert_eq!(nested.common.xml_id, Some("mdiv-b".to_string()));
        }
        other => panic!("Expected Mdiv, got {:?}", other),
    }
}

#[test]
fn roundtrip_mdiv_complete_cmn() {
    use tusk_model::data::{DataBoolean, DataWord};
    use tusk_model::elements::Mdiv;

    // Common Music Notation mdiv with all typical attributes
    let mut original = Mdiv::default();
    original.common.xml_id = Some("mdiv1".to_string());
    original.common.n = Some(DataWord("1".to_string()));
    original.common.label = Some("Allegro".to_string());
    original.mdiv_ges.attacca = Some(DataBoolean::False);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.common.n, original.common.n);
    assert_eq!(parsed.common.label, original.common.label);
    assert_eq!(parsed.mdiv_ges.attacca, original.mdiv_ges.attacca);
}

#[test]
fn mdiv_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Mdiv;

    let xml = r#"<mdiv xml:id="mdiv1" unknown="value" n="1"/>"#;
    let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(mdiv.common.xml_id, Some("mdiv1".to_string()));
}

#[test]
fn mdiv_ignores_unknown_child_elements() {
    use tusk_model::elements::Mdiv;

    let xml = r#"<mdiv xml:id="mdiv1"><unknownElement/></mdiv>"#;
    let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(mdiv.common.xml_id, Some("mdiv1".to_string()));
    assert!(mdiv.children.is_empty());
}

#[test]
fn mdiv_deserializes_with_xml_declaration() {
    use tusk_model::elements::Mdiv;

    let xml = r#"<?xml version="1.0"?><mdiv xml:id="mdiv1" n="1"/>"#;
    let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

    assert_eq!(mdiv.common.xml_id, Some("mdiv1".to_string()));
}

// ============================================================================
// Structural Hierarchy Tests
// ============================================================================
// These tests verify the complete structural hierarchy:
// mdiv → section → measure → staff → layer → note/rest/chord/space

#[test]
fn hierarchy_layer_contains_note() {
    use tusk_model::data::{DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note};

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(tusk_model::data::DataDuration::DataDurationCmn(
        DataDurationCmn::N4,
    ));

    let mut layer = Layer::default();
    layer.basic.xml_id = Some("layer1".to_string());
    layer.children.push(LayerChild::Note(Box::new(note)));

    let xml = layer.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("layer1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Note(n) => {
            assert_eq!(n.common.xml_id, Some("n1".to_string()));
            assert_eq!(n.note_log.pname, Some(DataPitchname::from("c".to_string())));
            assert_eq!(n.note_log.oct, Some(DataOctave(4)));
        }
        other => panic!("Expected Note, got {:?}", other),
    }
}

#[test]
fn hierarchy_layer_contains_mixed_children() {
    use tusk_model::data::{
        DataDuration, DataDurationCmn, DataDurationrests, DataOctave, DataPitchname,
    };
    use tusk_model::elements::{Chord, Layer, LayerChild, Note, Rest, Space};

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut rest = Rest::default();
    rest.common.xml_id = Some("r1".to_string());
    rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));

    let mut chord = Chord::default();
    chord.common.xml_id = Some("c1".to_string());
    chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2));

    let mut space = Space::default();
    space.common.xml_id = Some("s1".to_string());
    space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut layer = Layer::default();
    layer.basic.xml_id = Some("layer1".to_string());
    layer.children.push(LayerChild::Note(Box::new(note)));
    layer.children.push(LayerChild::Rest(Box::new(rest)));
    layer.children.push(LayerChild::Chord(Box::new(chord)));
    layer.children.push(LayerChild::Space(Box::new(space)));

    let xml = layer.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 4);

    // Verify order and types preserved
    assert!(matches!(&parsed.children[0], LayerChild::Note(_)));
    assert!(matches!(&parsed.children[1], LayerChild::Rest(_)));
    assert!(matches!(&parsed.children[2], LayerChild::Chord(_)));
    assert!(matches!(&parsed.children[3], LayerChild::Space(_)));
}

#[test]
fn hierarchy_staff_contains_layer() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note, Staff, StaffChild};

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut layer = Layer::default();
    layer.basic.xml_id = Some("layer1".to_string());
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("staff1".to_string());
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let xml = staff.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("staff1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        StaffChild::Layer(l) => {
            assert_eq!(l.basic.xml_id, Some("layer1".to_string()));
            assert_eq!(l.children.len(), 1);
            match &l.children[0] {
                LayerChild::Note(n) => {
                    assert_eq!(n.common.xml_id, Some("n1".to_string()));
                }
                other => panic!("Expected Note, got {:?}", other),
            }
        }
        other => panic!("Expected Layer, got {:?}", other),
    }
}

#[test]
fn hierarchy_staff_contains_multiple_layers() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note, Staff, StaffChild};

    // Create two layers for polyphony (e.g., soprano and alto in one staff)
    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    note1.note_log.pname = Some(DataPitchname::from("g".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut layer1 = Layer::default();
    layer1.basic.xml_id = Some("layer1".to_string());
    layer1.n_integer.n = Some(1);
    layer1.children.push(LayerChild::Note(Box::new(note1)));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut layer2 = Layer::default();
    layer2.basic.xml_id = Some("layer2".to_string());
    layer2.n_integer.n = Some(2);
    layer2.children.push(LayerChild::Note(Box::new(note2)));

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("staff1".to_string());
    staff.children.push(StaffChild::Layer(Box::new(layer1)));
    staff.children.push(StaffChild::Layer(Box::new(layer2)));

    let xml = staff.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // Verify layer order and content
    match &parsed.children[0] {
        StaffChild::Layer(l) => {
            assert_eq!(l.n_integer.n, Some(1));
        }
        other => panic!("Expected Layer, got {:?}", other),
    }

    match &parsed.children[1] {
        StaffChild::Layer(l) => {
            assert_eq!(l.n_integer.n, Some(2));
        }
        other => panic!("Expected Layer, got {:?}", other),
    }
}

#[test]
fn hierarchy_measure_contains_staff() {
    // Note: Currently, Measure's deserializer uses read_children_raw + parse_staff_from_raw
    // which only parses staff attributes, not staff's children (layers).
    // This test documents the current behavior - staff children are preserved in serialization
    // but not fully parsed in deserialization (layers within staff not parsed).
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("staff1".to_string());
    staff.n_integer.n = Some(1);
    // Note: even if we added layer children here, they wouldn't be parsed back

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let xml = measure.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.basic.xml_id, Some("staff1".to_string()));
            assert_eq!(s.n_integer.n, Some(1));
            // Staff children (layers) are not parsed from within Measure - known limitation
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn hierarchy_measure_contains_multiple_staves() {
    // Test that measure can contain multiple staff elements
    // Note: Staff children (layers) are not parsed - known limitation
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff_rh = Staff::default();
    staff_rh.basic.xml_id = Some("staff1".to_string());
    staff_rh.n_integer.n = Some(1);

    let mut staff_lh = Staff::default();
    staff_lh.basic.xml_id = Some("staff2".to_string());
    staff_lh.n_integer.n = Some(2);

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure
        .children
        .push(MeasureChild::Staff(Box::new(staff_rh)));
    measure
        .children
        .push(MeasureChild::Staff(Box::new(staff_lh)));

    let xml = measure.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // Verify staff order preserved
    match &parsed.children[0] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.n_integer.n, Some(1));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }

    match &parsed.children[1] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.n_integer.n, Some(2));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn hierarchy_section_contains_measure() {
    // Test section → measure hierarchy
    // Note: Measure's staff children are not parsed with full hierarchy
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Section, SectionChild, Staff};

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let mut section = Section::default();
    section.common.xml_id = Some("sec1".to_string());
    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    let xml = section.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m1".to_string()));
            assert_eq!(m.children.len(), 1); // Staff is parsed
        }
        other => panic!("Expected Measure, got {:?}", other),
    }
}

#[test]
fn hierarchy_section_contains_multiple_measures() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, Section, SectionChild};

    let mut measure1 = Measure::default();
    measure1.common.xml_id = Some("m1".to_string());
    measure1.common.n = Some(DataWord("1".to_string()));

    let mut measure2 = Measure::default();
    measure2.common.xml_id = Some("m2".to_string());
    measure2.common.n = Some(DataWord("2".to_string()));

    let mut measure3 = Measure::default();
    measure3.common.xml_id = Some("m3".to_string());
    measure3.common.n = Some(DataWord("3".to_string()));

    let mut section = Section::default();
    section.common.xml_id = Some("sec1".to_string());
    section
        .children
        .push(SectionChild::Measure(Box::new(measure1)));
    section
        .children
        .push(SectionChild::Measure(Box::new(measure2)));
    section
        .children
        .push(SectionChild::Measure(Box::new(measure3)));

    let xml = section.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 3);

    // Verify measure order preserved
    for (i, child) in parsed.children.iter().enumerate() {
        match child {
            SectionChild::Measure(m) => {
                let expected_n = DataWord(format!("{}", i + 1));
                assert_eq!(m.common.n, Some(expected_n));
            }
            other => panic!("Expected Measure, got {:?}", other),
        }
    }
}

#[test]
fn hierarchy_section_contains_nested_sections() {
    use tusk_model::elements::{Section, SectionChild};

    let mut inner_section = Section::default();
    inner_section.common.xml_id = Some("sec2".to_string());
    inner_section.common.label = Some("Coda".to_string());

    let mut outer_section = Section::default();
    outer_section.common.xml_id = Some("sec1".to_string());
    outer_section.common.label = Some("Movement I".to_string());
    outer_section
        .children
        .push(SectionChild::Section(Box::new(inner_section)));

    let xml = outer_section.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Section(s) => {
            assert_eq!(s.common.xml_id, Some("sec2".to_string()));
            assert_eq!(s.common.label, Some("Coda".to_string()));
        }
        other => panic!("Expected Section, got {:?}", other),
    }
}

#[test]
fn hierarchy_mdiv_contains_nested_mdiv() {
    use tusk_model::elements::{Mdiv, MdivChild};

    let mut inner_mdiv = Mdiv::default();
    inner_mdiv.common.xml_id = Some("mdiv2".to_string());
    inner_mdiv.common.label = Some("Movement I".to_string());

    let mut outer_mdiv = Mdiv::default();
    outer_mdiv.common.xml_id = Some("mdiv1".to_string());
    outer_mdiv.common.label = Some("Symphony No. 1".to_string());
    outer_mdiv
        .children
        .push(MdivChild::Mdiv(Box::new(inner_mdiv)));

    let xml = outer_mdiv.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mdiv1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MdivChild::Mdiv(m) => {
            assert_eq!(m.common.xml_id, Some("mdiv2".to_string()));
        }
        other => panic!("Expected Mdiv, got {:?}", other),
    }
}

#[test]
fn hierarchy_full_cmn_structure() {
    // Test section → measure → staff structure
    // Note: Staff's children (layers) are not parsed when staff is a child of measure
    // This tests the current behavior; full recursive parsing is a future enhancement
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Section, SectionChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("staff1".to_string());
    staff.n_integer.n = Some(1);
    // Staff children won't be parsed when inside measure

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let mut section = Section::default();
    section.common.xml_id = Some("sec1".to_string());
    section.common.label = Some("Movement I".to_string());
    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    // Serialize and parse
    let xml = section.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    // Traverse the hierarchy to verify structure preserved
    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.common.label, Some("Movement I".to_string()));
    assert_eq!(parsed.children.len(), 1);

    let measure = match &parsed.children[0] {
        SectionChild::Measure(m) => m,
        other => panic!("Expected Measure, got {:?}", other),
    };
    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
    assert_eq!(measure.children.len(), 1);

    let staff = match &measure.children[0] {
        MeasureChild::Staff(s) => s,
        other => panic!("Expected Staff, got {:?}", other),
    };
    assert_eq!(staff.basic.xml_id, Some("staff1".to_string()));
    assert_eq!(staff.n_integer.n, Some(1));
    // Staff children are not parsed - this is the current limitation
}

#[test]
fn hierarchy_realistic_piano_measure() {
    // Test a realistic piano measure with two staves
    // Note: Staff children (layers) are not parsed when inside measure
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut rh_staff = Staff::default();
    rh_staff.basic.xml_id = Some("rh".to_string());
    rh_staff.n_integer.n = Some(1);
    // Layers won't be parsed from within measure

    let mut lh_staff = Staff::default();
    lh_staff.basic.xml_id = Some("lh".to_string());
    lh_staff.n_integer.n = Some(2);
    // Layers won't be parsed from within measure

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure
        .children
        .push(MeasureChild::Staff(Box::new(rh_staff)));
    measure
        .children
        .push(MeasureChild::Staff(Box::new(lh_staff)));

    let xml = measure.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    // Verify structure - both staves should be parsed
    assert_eq!(parsed.children.len(), 2);

    // RH staff
    match &parsed.children[0] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.n_integer.n, Some(1));
            assert_eq!(s.basic.xml_id, Some("rh".to_string()));
            // Staff children are not parsed from within measure
        }
        other => panic!("Expected Staff, got {:?}", other),
    }

    // LH staff
    match &parsed.children[1] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.n_integer.n, Some(2));
            assert_eq!(s.basic.xml_id, Some("lh".to_string()));
            // Staff children are not parsed from within measure
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn hierarchy_from_external_xml() {
    // Parse a multi-level structure from external XML string
    // Note: Measure's staff parsing doesn't recursively parse staff children (layers)
    use tusk_model::elements::{MeasureChild, Section, SectionChild};

    let xml = r#"<section xml:id="sec1" label="Introduction">
        <measure xml:id="m1" n="1">
            <staff xml:id="s1" n="1">
                <layer xml:id="l1" n="1">
                    <note xml:id="n1" pname="c" oct="4" dur="4"/>
                </layer>
            </staff>
        </measure>
        <measure xml:id="m2" n="2">
            <staff xml:id="s2" n="1"/>
        </measure>
    </section>"#;

    let section = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
    assert_eq!(section.common.label, Some("Introduction".to_string()));
    assert_eq!(section.children.len(), 2);

    // First measure - verify measure and staff parsing
    match &section.children[0] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m1".to_string()));
            assert_eq!(m.children.len(), 1);
            match &m.children[0] {
                MeasureChild::Staff(s) => {
                    assert_eq!(s.basic.xml_id, Some("s1".to_string()));
                    assert_eq!(s.n_integer.n, Some(1));
                    // Staff children (layers) are not parsed when inside measure
                }
                other => panic!("Expected Staff, got {:?}", other),
            }
        }
        other => panic!("Expected Measure, got {:?}", other),
    }

    // Second measure
    match &section.children[1] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m2".to_string()));
            assert_eq!(m.children.len(), 1);
            match &m.children[0] {
                MeasureChild::Staff(s) => {
                    assert_eq!(s.basic.xml_id, Some("s2".to_string()));
                }
                other => panic!("Expected Staff, got {:?}", other),
            }
        }
        other => panic!("Expected Measure, got {:?}", other),
    }
}

#[test]
fn hierarchy_deep_nesting_preserved() {
    // Test deeply nested mdiv structure preserves all IDs through serialization
    use tusk_model::elements::{Mdiv, MdivChild};

    // Build nested mdiv hierarchy
    let mut inner_inner_mdiv = Mdiv::default();
    inner_inner_mdiv.common.xml_id = Some("mdiv-inner-inner".to_string());
    inner_inner_mdiv.common.label = Some("Third Level".to_string());

    let mut inner_mdiv = Mdiv::default();
    inner_mdiv.common.xml_id = Some("mdiv-inner".to_string());
    inner_mdiv.common.label = Some("Second Level".to_string());
    inner_mdiv
        .children
        .push(MdivChild::Mdiv(Box::new(inner_inner_mdiv)));

    let mut outer_mdiv = Mdiv::default();
    outer_mdiv.common.xml_id = Some("mdiv-outer".to_string());
    outer_mdiv.common.label = Some("First Level".to_string());
    outer_mdiv
        .children
        .push(MdivChild::Mdiv(Box::new(inner_mdiv)));

    let xml = outer_mdiv.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    // Verify three-level nesting is preserved
    assert_eq!(parsed.common.xml_id, Some("mdiv-outer".to_string()));
    assert_eq!(parsed.common.label, Some("First Level".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MdivChild::Mdiv(inner) => {
            assert_eq!(inner.common.xml_id, Some("mdiv-inner".to_string()));
            assert_eq!(inner.common.label, Some("Second Level".to_string()));
            assert_eq!(inner.children.len(), 1);

            match &inner.children[0] {
                MdivChild::Mdiv(inner_inner) => {
                    assert_eq!(
                        inner_inner.common.xml_id,
                        Some("mdiv-inner-inner".to_string())
                    );
                    assert_eq!(inner_inner.common.label, Some("Third Level".to_string()));
                }
                other => panic!("Expected Mdiv, got {:?}", other),
            }
        }
        other => panic!("Expected Mdiv, got {:?}", other),
    }
}

// ============================================================================
// ScoreDef Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_scoredef() {
    use tusk_model::elements::ScoreDef;

    let original = ScoreDef::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.score_def_log.meter_count.is_none());
    assert!(parsed.score_def_log.meter_unit.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_scoredef_with_xml_id() {
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.common.xml_id = Some("sd1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"sd1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("sd1".to_string()));
}

#[test]
fn roundtrip_scoredef_with_meter_attributes() {
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_log.meter_count = Some("4".to_string());
    original.score_def_log.meter_unit = Some(4.0);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("meter.count=\"4\""),
        "xml should contain meter.count: {}",
        xml
    );
    assert!(
        xml.contains("meter.unit=\"4\""),
        "xml should contain meter.unit: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
}

#[test]
fn roundtrip_scoredef_with_meter_sym() {
    use tusk_model::data::DataMetersign;
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_log.meter_count = Some("4".to_string());
    original.score_def_log.meter_unit = Some(4.0);
    original.score_def_log.meter_sym = Some(DataMetersign::Common);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("meter.sym=\"common\""),
        "xml should contain meter.sym: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_log.meter_sym, Some(DataMetersign::Common));
}

#[test]
fn roundtrip_scoredef_with_keysig() {
    use tusk_model::data::DataKeyfifths;
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    // 3 flats (e.g., E-flat major or C minor)
    original.score_def_log.keysig = vec![DataKeyfifths("-3".to_string())];

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("keysig=\"-3\"") || xml.contains("keysig=\"3f\""),
        "xml should contain keysig: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert!(!parsed.score_def_log.keysig.is_empty());
}

#[test]
fn roundtrip_scoredef_with_clef_attributes() {
    use tusk_model::data::{DataClefline, DataClefshape};
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_log.clef_shape = Some(DataClefshape::G);
    original.score_def_log.clef_line = Some(DataClefline(2));

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("clef.shape=\"G\""),
        "xml should contain clef.shape: {}",
        xml
    );
    assert!(
        xml.contains("clef.line=\"2\""),
        "xml should contain clef.line: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_log.clef_shape, Some(DataClefshape::G));
    assert_eq!(parsed.score_def_log.clef_line, Some(DataClefline(2)));
}

#[test]
fn roundtrip_scoredef_with_gestural_ppq() {
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_ges.ppq = Some(480);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("ppq=\"480\""),
        "xml should contain ppq: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_ges.ppq, Some(480));
}

#[test]
fn roundtrip_scoredef_with_midi_bpm() {
    use tusk_model::data::DataMidibpm;
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_ges.midi_bpm = Some(DataMidibpm(120.0));

    let xml = original.to_mei_string().expect("serialize");
    // midi.bpm may serialize as "120" or "120.0" depending on float representation
    assert!(
        xml.contains("midi.bpm=\"120") && xml.contains('"'),
        "xml should contain midi.bpm: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.score_def_ges.midi_bpm.is_some());
}

#[test]
fn roundtrip_scoredef_with_analytical_key() {
    use tusk_model::data::{DataMode, DataModeCmn, DataPitchname};
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_anl.key_pname = Some(DataPitchname::from("c".to_string()));
    original.score_def_anl.key_mode = Some(DataMode::DataModeCmn(DataModeCmn::Major));

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("key.pname=\"c\""),
        "xml should contain key.pname: {}",
        xml
    );
    assert!(
        xml.contains("key.mode=\"major\""),
        "xml should contain key.mode: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(
        parsed.score_def_anl.key_pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(
        parsed.score_def_anl.key_mode,
        Some(DataMode::DataModeCmn(DataModeCmn::Major))
    );
}

#[test]
fn roundtrip_scoredef_with_tune_hz() {
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_ges.tune_hz = Some(440.0);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("tune.Hz=\"440\""),
        "xml should contain tune.Hz: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_ges.tune_hz, Some(440.0));
}

#[test]
fn roundtrip_scoredef_with_visual_meter_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_vis.meter_visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("meter.visible=\"true\""),
        "xml should contain meter.visible: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_vis.meter_visible, Some(DataBoolean::True));
}

#[test]
fn roundtrip_scoredef_comprehensive() {
    use tusk_model::data::{DataClefline, DataClefshape, DataMode, DataModeCmn, DataPitchname};
    use tusk_model::elements::ScoreDef;

    // A realistic scoreDef with common attributes
    let mut original = ScoreDef::default();
    original.common.xml_id = Some("sd1".to_string());
    original.score_def_log.clef_shape = Some(DataClefshape::G);
    original.score_def_log.clef_line = Some(DataClefline(2));
    original.score_def_log.meter_count = Some("4".to_string());
    original.score_def_log.meter_unit = Some(4.0);
    original.score_def_anl.key_pname = Some(DataPitchname::from("c".to_string()));
    original.score_def_anl.key_mode = Some(DataMode::DataModeCmn(DataModeCmn::Major));
    original.score_def_ges.ppq = Some(960);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.score_def_log.clef_shape, Some(DataClefshape::G));
    assert_eq!(parsed.score_def_log.clef_line, Some(DataClefline(2)));
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
    assert_eq!(
        parsed.score_def_anl.key_pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(
        parsed.score_def_anl.key_mode,
        Some(DataMode::DataModeCmn(DataModeCmn::Major))
    );
    assert_eq!(parsed.score_def_ges.ppq, Some(960));
}

// External XML parsing tests for scoreDef

#[test]
fn parse_external_scoredef_minimal() {
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef/>"#;
    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn parse_external_scoredef_with_meter() {
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef meter.count="4" meter.unit="4"/>"#;
    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
}

#[test]
fn parse_external_scoredef_with_keysig_fifths() {
    use tusk_model::elements::ScoreDef;

    // keysig="3f" means 3 flats
    let xml = r#"<scoreDef keysig="3f" meter.count="4" meter.sym="common" meter.unit="4"/>"#;
    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    // keysig should be parsed
    assert!(!parsed.score_def_log.keysig.is_empty());
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
}

#[test]
fn parse_external_scoredef_with_staffgrp_child() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild};

    let xml = r#"<scoreDef>
        <staffGrp xml:id="sg1">
            <staffDef xml:id="sd1" n="1" lines="5" clef.shape="G" clef.line="2"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.children.len(), 1);
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn parse_external_scoredef_mensural_example() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    // From specs/mei/examples/verovio/notes_rests.mei
    let xml = r#"<scoreDef>
        <staffGrp>
            <staffDef label="notes" n="1" notationtype="mensural.white" lines="5" clef.shape="G" clef.line="2"/>
            <staffDef label="rests" n="2" notationtype="mensural.white" lines="5" clef.shape="G" clef.line="2"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.children.len(), 2);
            // Check first staffDef - label is in labelled.label for StaffDef
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.labelled.label, Some("notes".to_string()));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
            // Check second staffDef
            match &sg.children[1] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.labelled.label, Some("rests".to_string()));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn roundtrip_scoredef_with_staffgrp_child() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffDef, StaffGrp, StaffGrpChild};

    let mut staff_def = StaffDef::default();
    staff_def.basic.xml_id = Some("staff1".to_string());
    staff_def.n_integer.n = Some(1);

    let mut staff_grp = StaffGrp::default();
    staff_grp.common.xml_id = Some("sg1".to_string());
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let mut original = ScoreDef::default();
    original.common.xml_id = Some("sd1".to_string());
    original
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.children.len(), 1);
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.basic.xml_id, Some("staff1".to_string()));
                    assert_eq!(sd.n_integer.n, Some(1));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn roundtrip_scoredef_with_keysig_child() {
    use tusk_model::elements::{KeySig, ScoreDef, ScoreDefChild};

    let mut keysig = KeySig::default();
    keysig.common.xml_id = Some("ks1".to_string());

    let mut original = ScoreDef::default();
    original.common.xml_id = Some("sd1".to_string());
    original
        .children
        .push(ScoreDefChild::KeySig(Box::new(keysig)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::KeySig(ks) => {
            assert_eq!(ks.common.xml_id, Some("ks1".to_string()));
        }
        other => panic!("Expected KeySig, got {:?}", other),
    }
}

#[test]
fn roundtrip_scoredef_with_metersig_child() {
    use tusk_model::elements::{MeterSig, ScoreDef, ScoreDefChild};

    let mut metersig = MeterSig::default();
    metersig.common.xml_id = Some("ms1".to_string());

    let mut original = ScoreDef::default();
    original
        .children
        .push(ScoreDefChild::MeterSig(Box::new(metersig)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::MeterSig(ms) => {
            assert_eq!(ms.common.xml_id, Some("ms1".to_string()));
        }
        other => panic!("Expected MeterSig, got {:?}", other),
    }
}

// ============================================================================
// StaffDef parsing tests
// ============================================================================

#[test]
fn staffdef_deserializes_from_empty_element() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n_integer.n.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn staffdef_deserializes_xml_id() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef xml:id="sd1"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
}

#[test]
fn staffdef_deserializes_n_attribute() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef n="1"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn staffdef_deserializes_label() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef label="Soprano"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.labelled.label, Some("Soprano".to_string()));
}

#[test]
fn staffdef_deserializes_lines() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef lines="5"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.lines, Some(5));
}

#[test]
fn staffdef_deserializes_clef_attributes() {
    use tusk_model::data::{DataClefline, DataClefshape};
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef clef.shape="G" clef.line="2"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.clef_shape, Some(DataClefshape::G));
    assert_eq!(parsed.staff_def_log.clef_line, Some(DataClefline(2)));
}

#[test]
fn staffdef_deserializes_clef_dis_attributes() {
    use tusk_model::data::{DataOctaveDis, DataStaffrelBasic};
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef clef.dis="8" clef.dis.place="below"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.clef_dis, Some(DataOctaveDis(8)));
    assert_eq!(
        parsed.staff_def_log.clef_dis_place,
        Some(DataStaffrelBasic::Below)
    );
}

#[test]
fn staffdef_deserializes_notationtype() {
    use tusk_model::data::DataNotationtype;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef notationtype="mensural.white"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.staff_def_log.notationtype,
        Some(DataNotationtype::MensuralWhite)
    );
}

#[test]
fn staffdef_deserializes_meter_attributes() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef meter.count="4" meter.unit="4"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.staff_def_log.meter_unit, Some(4.0));
}

#[test]
fn staffdef_deserializes_meter_sym() {
    use tusk_model::data::DataMetersign;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef meter.sym="common"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.meter_sym, Some(DataMetersign::Common));
}

#[test]
fn staffdef_deserializes_transposition() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef trans.diat="-2" trans.semi="-3"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.trans_diat, Some(-2));
    assert_eq!(parsed.staff_def_log.trans_semi, Some(-3));
}

#[test]
fn staffdef_deserializes_keysig() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef keysig="2s"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert!(!parsed.staff_def_log.keysig.is_empty());
}

#[test]
fn staffdef_deserializes_ppq() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef ppq="960"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_ges.ppq, Some(960));
}

#[test]
fn staffdef_deserializes_tuning_attributes() {
    use tusk_model::data::DataPitchname;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef tune.Hz="440" tune.pname="a"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_ges.tune_hz, Some(440.0));
    assert_eq!(
        parsed.staff_def_ges.tune_pname,
        Some(DataPitchname::from("a".to_string()))
    );
}

#[test]
fn staffdef_deserializes_key_attributes() {
    use tusk_model::data::{DataMode, DataModeCmn, DataPitchname};
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef key.pname="c" key.mode="major"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.staff_def_anl.key_pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(
        parsed.staff_def_anl.key_mode,
        Some(DataMode::DataModeCmn(DataModeCmn::Major))
    );
}

#[test]
fn staffdef_deserializes_visual_attributes() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef clef.visible="true" lines.visible="true"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_vis.clef_visible, Some(DataBoolean::True));
    assert_eq!(parsed.staff_def_vis.lines_visible, Some(DataBoolean::True));
}

#[test]
fn staffdef_deserializes_scale() {
    use tusk_model::data::DataPercent;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef scale="75%"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.staff_def_vis.scale,
        Some(DataPercent("75%".to_string()))
    );
}

#[test]
fn staffdef_deserializes_full_common_staff_attributes() {
    use tusk_model::data::{DataClefline, DataClefshape, DataNotationtype};
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef xml:id="sd1" n="1" label="Tenor" lines="5" clef.shape="C" clef.line="4" notationtype="cmn"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.labelled.label, Some("Tenor".to_string()));
    assert_eq!(parsed.staff_def_log.lines, Some(5));
    assert_eq!(parsed.staff_def_log.clef_shape, Some(DataClefshape::C));
    assert_eq!(parsed.staff_def_log.clef_line, Some(DataClefline(4)));
    assert_eq!(
        parsed.staff_def_log.notationtype,
        Some(DataNotationtype::Cmn)
    );
}

#[test]
fn staffdef_deserializes_with_clef_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><clef xml:id="c1" shape="G" line="2"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::Clef(clef) => {
            assert_eq!(clef.common.xml_id, Some("c1".to_string()));
        }
        other => panic!("Expected Clef, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_with_keysig_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><keySig xml:id="ks1"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::KeySig(ks) => {
            assert_eq!(ks.common.xml_id, Some("ks1".to_string()));
        }
        other => panic!("Expected KeySig, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_with_metersig_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><meterSig xml:id="ms1" count="4" unit="4"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::MeterSig(ms) => {
            assert_eq!(ms.common.xml_id, Some("ms1".to_string()));
        }
        other => panic!("Expected MeterSig, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_with_label_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><label xml:id="l1">Violin I</label></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::Label(label) => {
            assert_eq!(label.common.xml_id, Some("l1".to_string()));
        }
        other => panic!("Expected Label, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_with_layerdef_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><layerDef xml:id="ld1" n="1"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::LayerDef(ld) => {
            assert_eq!(ld.basic.xml_id, Some("ld1".to_string()));
        }
        other => panic!("Expected LayerDef, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_multiple_children() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1">
        <label>Violin</label>
        <clef shape="G" line="2"/>
        <keySig/>
    </staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 3);
    assert!(matches!(&parsed.children[0], StaffDefChild::Label(_)));
    assert!(matches!(&parsed.children[1], StaffDefChild::Clef(_)));
    assert!(matches!(&parsed.children[2], StaffDefChild::KeySig(_)));
}

#[test]
fn staffdef_handles_unknown_attributes_leniently() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef xml:id="sd1" unknown="value" n="1"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn staffdef_roundtrip_basic() {
    use tusk_model::elements::StaffDef;

    let mut original = StaffDef::default();
    original.basic.xml_id = Some("sd1".to_string());
    original.n_integer.n = Some(1);
    original.staff_def_log.lines = Some(5);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = StaffDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.staff_def_log.lines, Some(5));
}

// Note: Full roundtrip with children requires serialization implementation.
// Testing deserialization from manually constructed XML instead.
#[test]
fn staffdef_parses_with_clef_from_xml() {
    use tusk_model::data::{DataClefline, DataClefshape};
    use tusk_model::elements::{StaffDef, StaffDefChild};

    // Use manually constructed XML to verify deserialization
    let xml = r#"<staffDef xml:id="sd1"><clef xml:id="c1" shape="G" line="2"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::Clef(c) => {
            assert_eq!(c.common.xml_id, Some("c1".to_string()));
            assert_eq!(c.clef_log.shape, Some(DataClefshape::G));
            assert_eq!(c.clef_log.line, Some(DataClefline(2)));
        }
        other => panic!("Expected Clef, got {:?}", other),
    }
}

// ============================================================================
// LayerDef parsing tests
// ============================================================================

#[test]
fn layerdef_deserializes_from_empty_element() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n_integer.n.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn layerdef_deserializes_xml_id() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef xml:id="ld1"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
}

#[test]
fn layerdef_deserializes_n_attribute() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef n="1"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn layerdef_deserializes_label() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef label="Voice 1"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.labelled.label, Some("Voice 1".to_string()));
}

#[test]
fn layerdef_deserializes_dur_default() {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef dur.default="4"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.layer_def_log.dur_default,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn layerdef_deserializes_oct_default() {
    use tusk_model::data::DataOctave;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef oct.default="4"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.oct_default, Some(DataOctave(4)));
}

#[test]
fn layerdef_deserializes_num_default_and_numbase_default() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef num.default="3" numbase.default="2"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.num_default, Some(3));
    assert_eq!(parsed.layer_def_log.numbase_default, Some(2));
}

#[test]
fn layerdef_deserializes_beam_group() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef beam.group="4,4,4,4"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.beam_group, Some("4,4,4,4".to_string()));
}

#[test]
fn layerdef_deserializes_beam_rests() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef beam.rests="true"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.beam_rests, Some(DataBoolean::True));
}

#[test]
fn layerdef_deserializes_transposition() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef trans.diat="-1" trans.semi="-2"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.trans_diat, Some(-1));
    assert_eq!(parsed.layer_def_log.trans_semi, Some(-2));
}

#[test]
fn layerdef_deserializes_gestural_instr() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::LayerDef;

    let xml = r##"<layerDef instr="#instr1"/>"##;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.layer_def_ges.instr,
        Some(DataUri("#instr1".to_string()))
    );
}

#[test]
fn layerdef_deserializes_tuning_attributes() {
    use tusk_model::data::DataPitchname;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef tune.Hz="442" tune.pname="a"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_ges.tune_hz, Some(442.0));
    assert_eq!(
        parsed.layer_def_ges.tune_pname,
        Some(DataPitchname::from("a".to_string()))
    );
}

#[test]
fn layerdef_deserializes_visual_beam_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef beam.color="red"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.layer_def_vis.beam_color,
        Some(DataColor::DataColornames(DataColornames::Red))
    );
}

#[test]
fn layerdef_deserializes_beam_rend() {
    use tusk_model::att::AttLayerDefVisBeamRend;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef beam.rend="acc"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.layer_def_vis.beam_rend,
        Some(AttLayerDefVisBeamRend::Acc)
    );
}

#[test]
fn layerdef_deserializes_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef visible="false"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_vis.visible, Some(DataBoolean::False));
}

#[test]
fn layerdef_deserializes_full_common_attributes() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave};
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef xml:id="ld1" n="1" label="Melody" dur.default="8" oct.default="5"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.labelled.label, Some("Melody".to_string()));
    assert_eq!(
        parsed.layer_def_log.dur_default,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
    );
    assert_eq!(parsed.layer_def_log.oct_default, Some(DataOctave(5)));
}

#[test]
fn layerdef_deserializes_with_label_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1"><label xml:id="l1">Voice I</label></layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LayerDefChild::Label(label) => {
            assert_eq!(label.common.xml_id, Some("l1".to_string()));
        }
        other => panic!("Expected Label, got {:?}", other),
    }
}

#[test]
fn layerdef_deserializes_with_labelabbr_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1"><labelAbbr xml:id="la1">V.I</labelAbbr></layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LayerDefChild::LabelAbbr(label_abbr) => {
            assert_eq!(label_abbr.common.xml_id, Some("la1".to_string()));
        }
        other => panic!("Expected LabelAbbr, got {:?}", other),
    }
}

#[test]
fn layerdef_deserializes_with_instrdef_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1"><instrDef xml:id="id1"/></layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LayerDefChild::InstrDef(instr_def) => {
            assert_eq!(instr_def.basic.xml_id, Some("id1".to_string()));
        }
        other => panic!("Expected InstrDef, got {:?}", other),
    }
}

#[test]
fn layerdef_deserializes_with_metersig_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1"><meterSig xml:id="ms1" count="4" unit="4"/></layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LayerDefChild::MeterSig(meter_sig) => {
            assert_eq!(meter_sig.common.xml_id, Some("ms1".to_string()));
        }
        other => panic!("Expected MeterSig, got {:?}", other),
    }
}

#[test]
fn layerdef_deserializes_multiple_children() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1">
        <label>Voice</label>
        <labelAbbr>V.</labelAbbr>
        <instrDef/>
    </layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 3);
    assert!(matches!(&parsed.children[0], LayerDefChild::Label(_)));
    assert!(matches!(&parsed.children[1], LayerDefChild::LabelAbbr(_)));
    assert!(matches!(&parsed.children[2], LayerDefChild::InstrDef(_)));
}

#[test]
fn layerdef_handles_unknown_attributes_leniently() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef xml:id="ld1" unknown="value" n="1"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn layerdef_roundtrip_basic() {
    use tusk_model::elements::LayerDef;

    let mut original = LayerDef::default();
    original.basic.xml_id = Some("ld1".to_string());
    original.n_integer.n = Some(1);
    original.labelled.label = Some("Voice 1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = LayerDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.labelled.label, Some("Voice 1".to_string()));
}

#[test]
fn layerdef_roundtrip_with_log_attributes() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave};
    use tusk_model::elements::LayerDef;

    let mut original = LayerDef::default();
    original.layer_def_log.dur_default = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.layer_def_log.oct_default = Some(DataOctave(4));
    original.layer_def_log.beam_group = Some("8,8,8,8".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = LayerDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.layer_def_log.dur_default,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(parsed.layer_def_log.oct_default, Some(DataOctave(4)));
    assert_eq!(parsed.layer_def_log.beam_group, Some("8,8,8,8".to_string()));
}

#[test]
fn layerdef_roundtrip_with_ges_attributes() {
    use tusk_model::data::{DataPitchname, DataUri};
    use tusk_model::elements::LayerDef;

    let mut original = LayerDef::default();
    original.layer_def_ges.instr = Some(DataUri("#piano".to_string()));
    original.layer_def_ges.tune_hz = Some(440.0);
    original.layer_def_ges.tune_pname = Some(DataPitchname::from("a".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = LayerDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.layer_def_ges.instr,
        Some(DataUri("#piano".to_string()))
    );
    assert_eq!(parsed.layer_def_ges.tune_hz, Some(440.0));
    assert_eq!(
        parsed.layer_def_ges.tune_pname,
        Some(DataPitchname::from("a".to_string()))
    );
}

#[test]
fn layerdef_roundtrip_with_vis_attributes() {
    use tusk_model::att::AttLayerDefVisBeamRend;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::LayerDef;

    let mut original = LayerDef::default();
    original.layer_def_vis.beam_rend = Some(AttLayerDefVisBeamRend::Rit);
    original.layer_def_vis.beam_slope = Some(0.5);
    original.layer_def_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = LayerDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.layer_def_vis.beam_rend,
        Some(AttLayerDefVisBeamRend::Rit)
    );
    assert_eq!(parsed.layer_def_vis.beam_slope, Some(0.5));
    assert_eq!(parsed.layer_def_vis.visible, Some(DataBoolean::True));
}

// ============================================================================
// StaffGrp tests with full attribute support
// ============================================================================

#[test]
fn staffgrp_parse_with_vis_attributes() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::{DataBarmethod, DataBoolean};
    use tusk_model::elements::{ScoreDef, ScoreDefChild};

    let xml = r#"<scoreDef>
        <staffGrp xml:id="sg1" symbol="bracket" bar.thru="true" bar.method="mensur" bar.len="8" visible="true">
            <staffDef n="1" lines="5"/>
            <staffDef n="2" lines="5"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.staff_grp_vis.symbol, Some(AttStaffGrpVisSymbol::Bracket));
            assert_eq!(sg.staff_grp_vis.bar_thru, Some(DataBoolean::True));
            assert_eq!(sg.staff_grp_vis.bar_method, Some(DataBarmethod::Mensur));
            assert_eq!(sg.staff_grp_vis.bar_len, Some(8.0));
            assert_eq!(sg.staff_grp_vis.visible, Some(DataBoolean::True));
            assert_eq!(sg.children.len(), 2);
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn staffgrp_parse_with_ges_attributes() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{ScoreDef, ScoreDefChild};

    let xml = r##"<scoreDef>
        <staffGrp xml:id="sg1" instr="#piano">
            <staffDef n="1" lines="5"/>
        </staffGrp>
    </scoreDef>"##;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.staff_grp_ges.instr, Some(DataUri("#piano".to_string())));
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn staffgrp_roundtrip_with_vis_attributes() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::{DataBarmethod, DataBoolean, DataStaffloc};
    use tusk_model::elements::{StaffDef, StaffGrp, StaffGrpChild};

    let mut staff_grp = StaffGrp::default();
    staff_grp.common.xml_id = Some("sg1".to_string());
    staff_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Brace);
    staff_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);
    staff_grp.staff_grp_vis.bar_method = Some(DataBarmethod::Mensur);
    staff_grp.staff_grp_vis.bar_len = Some(8.0);
    staff_grp.staff_grp_vis.bar_place = Some(DataStaffloc::from(0));
    staff_grp.staff_grp_vis.visible = Some(DataBoolean::True);

    let mut staff_def = StaffDef::default();
    staff_def.n_integer.n = Some(1);
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let xml = staff_grp.to_mei_string().expect("serialize");
    let parsed = StaffGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sg1".to_string()));
    assert_eq!(
        parsed.staff_grp_vis.symbol,
        Some(AttStaffGrpVisSymbol::Brace)
    );
    assert_eq!(parsed.staff_grp_vis.bar_thru, Some(DataBoolean::True));
    assert_eq!(parsed.staff_grp_vis.bar_method, Some(DataBarmethod::Mensur));
    assert_eq!(parsed.staff_grp_vis.bar_len, Some(8.0));
    assert_eq!(parsed.staff_grp_vis.bar_place, Some(DataStaffloc::from(0)));
    assert_eq!(parsed.staff_grp_vis.visible, Some(DataBoolean::True));
}

#[test]
fn staffgrp_roundtrip_with_ges_attributes() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{StaffDef, StaffGrp, StaffGrpChild};

    let mut staff_grp = StaffGrp::default();
    staff_grp.common.xml_id = Some("sg1".to_string());
    staff_grp.staff_grp_ges.instr = Some(DataUri("#strings".to_string()));

    let mut staff_def = StaffDef::default();
    staff_def.n_integer.n = Some(1);
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let xml = staff_grp.to_mei_string().expect("serialize");
    let parsed = StaffGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sg1".to_string()));
    assert_eq!(
        parsed.staff_grp_ges.instr,
        Some(DataUri("#strings".to_string()))
    );
}

#[test]
fn staffgrp_nested_with_attributes() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef>
        <staffGrp xml:id="outer" symbol="bracket" bar.thru="true">
            <staffGrp xml:id="inner" symbol="brace">
                <staffDef n="1" lines="5"/>
                <staffDef n="2" lines="5"/>
            </staffGrp>
            <staffDef n="3" lines="5"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(outer) => {
            assert_eq!(outer.common.xml_id, Some("outer".to_string()));
            assert_eq!(
                outer.staff_grp_vis.symbol,
                Some(AttStaffGrpVisSymbol::Bracket)
            );
            assert_eq!(outer.staff_grp_vis.bar_thru, Some(DataBoolean::True));
            assert_eq!(outer.children.len(), 2);

            // Check nested staffGrp
            match &outer.children[0] {
                StaffGrpChild::StaffGrp(inner) => {
                    assert_eq!(inner.common.xml_id, Some("inner".to_string()));
                    assert_eq!(
                        inner.staff_grp_vis.symbol,
                        Some(AttStaffGrpVisSymbol::Brace)
                    );
                    assert_eq!(inner.children.len(), 2);
                }
                other => panic!("Expected nested StaffGrp, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

// ============================================================================
// Score Definition Integration Tests
// ============================================================================
//
// These tests verify that scoreDef, staffDef, layerDef, and staffGrp elements
// work together correctly as complete score definition structures. Test cases
// include:
//
// - Complete score definitions from real MEI example files
// - Score redefinition scenarios (key/meter changes mid-score)
// - Complex staffGrp hierarchies with multiple staves
// - Round-trip serialization of complete score definitions
// ============================================================================

// ----------------------------------------------------------------------------
// Tests from specs/mei/examples/verovio/04-score-redefinition.mei
// Score definition with key signature and meter changes
// ----------------------------------------------------------------------------

/// Initial scoreDef with keysig="4f" and meter.sym="common" from 04-score-redefinition.mei
#[test]
fn scoredef_example_key_changes_initial() {
    use tusk_model::data::{DataClefline, DataClefshape, DataMetersign};
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef keysig="4f" meter.sym="common">
        <staffGrp>
            <staffDef n="1" lines="5" clef.shape="G" clef.line="2" />
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    // Check keysig attribute - should contain "4f"
    assert!(
        !parsed.score_def_log.keysig.is_empty(),
        "keysig should be set"
    );
    // Check meter symbol
    assert_eq!(parsed.score_def_log.meter_sym, Some(DataMetersign::Common));

    // Verify staffGrp structure
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.children.len(), 1);
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.n_integer.n, Some(1));
                    assert_eq!(sd.staff_def_log.lines, Some(5));
                    assert_eq!(sd.staff_def_log.clef_shape, Some(DataClefshape::G));
                    assert_eq!(sd.staff_def_log.clef_line, Some(DataClefline(2)));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Score redefinition mid-score: keysig="0" keysig.cancelaccid="none"
#[test]
fn scoredef_example_key_changes_cancel_none() {
    use tusk_model::data::DataCancelaccid;
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef keysig="0" keysig.cancelaccid="none" />"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    // keysig="0" means no key signature
    assert!(!parsed.score_def_log.keysig.is_empty());
    // Verify cancel accidental attribute
    assert_eq!(
        parsed.score_def_vis.keysig_cancelaccid,
        Some(DataCancelaccid::None)
    );
}

/// Score redefinition with new key and meter: keysig="2s" meter.sym="cut"
#[test]
fn scoredef_example_key_changes_new_key_and_meter() {
    use tusk_model::data::{DataCancelaccid, DataMetersign};
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef keysig="2s" keysig.cancelaccid="before" meter.sym="cut" />"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert!(!parsed.score_def_log.keysig.is_empty());
    assert_eq!(
        parsed.score_def_vis.keysig_cancelaccid,
        Some(DataCancelaccid::Before)
    );
    assert_eq!(parsed.score_def_log.meter_sym, Some(DataMetersign::Cut));
}

/// Score redefinition with keysig.visible="false"
#[test]
fn scoredef_example_key_changes_invisible_keysig() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef keysig.visible="false" keysig="5f" meter.count="4" meter.unit="4" />"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.score_def_vis.keysig_visible,
        Some(DataBoolean::False)
    );
    assert!(!parsed.score_def_log.keysig.is_empty());
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
}

/// Score redefinition with keysig.cancelaccid="before-bar"
#[test]
fn scoredef_example_key_changes_cancel_before_bar() {
    use tusk_model::data::DataCancelaccid;
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef keysig="2s" keysig.cancelaccid="before-bar" />"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert!(!parsed.score_def_log.keysig.is_empty());
    assert_eq!(
        parsed.score_def_vis.keysig_cancelaccid,
        Some(DataCancelaccid::BeforeBar)
    );
}

// ----------------------------------------------------------------------------
// Tests from specs/mei/examples/verovio/tchaikovsky_scherzo.mei
// String quartet score definition with multiple staves
// ----------------------------------------------------------------------------

/// Complete scoreDef from Tchaikovsky string quartet with 4 staves
#[test]
fn scoredef_example_tchaikovsky_string_quartet() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::{DataClefline, DataClefshape};
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffDefChild, StaffGrpChild};

    let xml = r#"<scoreDef>
        <staffGrp symbol="bracket">
            <staffDef n="1" lines="5" keysig="5f">
                <clef shape="G" line="2" />
            </staffDef>
            <staffDef n="2" lines="5" keysig="5f">
                <clef shape="G" line="2" />
            </staffDef>
            <staffDef n="3" lines="5" keysig="5f">
                <clef shape="C" line="3" />
            </staffDef>
            <staffDef n="4" lines="5" keysig="5f">
                <clef shape="F" line="4" />
            </staffDef>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    // Verify staffGrp with bracket symbol
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.staff_grp_vis.symbol, Some(AttStaffGrpVisSymbol::Bracket));
            assert_eq!(sg.children.len(), 4);

            // Check each staff definition
            for (i, child) in sg.children.iter().enumerate() {
                match child {
                    StaffGrpChild::StaffDef(sd) => {
                        assert_eq!(sd.n_integer.n, Some((i + 1) as u64));
                        assert_eq!(sd.staff_def_log.lines, Some(5));
                        assert!(!sd.staff_def_log.keysig.is_empty(), "keysig should be set");

                        // Check clef child element
                        assert_eq!(sd.children.len(), 1);
                        match &sd.children[0] {
                            StaffDefChild::Clef(clef) => {
                                match i {
                                    0 | 1 => {
                                        // Violin I & II: treble clef
                                        assert_eq!(clef.clef_log.shape, Some(DataClefshape::G));
                                        assert_eq!(clef.clef_log.line, Some(DataClefline(2)));
                                    }
                                    2 => {
                                        // Viola: alto clef
                                        assert_eq!(clef.clef_log.shape, Some(DataClefshape::C));
                                        assert_eq!(clef.clef_log.line, Some(DataClefline(3)));
                                    }
                                    3 => {
                                        // Cello: bass clef
                                        assert_eq!(clef.clef_log.shape, Some(DataClefshape::F));
                                        assert_eq!(clef.clef_log.line, Some(DataClefline(4)));
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            other => panic!("Expected Clef, got {:?}", other),
                        }
                    }
                    other => panic!("Expected StaffDef, got {:?}", other),
                }
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

// ----------------------------------------------------------------------------
// Score Definition Round-Trip Tests
// Verify complete score definition structures serialize and deserialize correctly
// ----------------------------------------------------------------------------

/// Round-trip test for complete orchestral score definition
#[test]
fn scoredef_roundtrip_complete_orchestral() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::{
        DataBoolean, DataClefline, DataClefshape, DataMetersign, DataNotationtype,
    };
    use tusk_model::elements::{Clef, ScoreDef, ScoreDefChild, StaffDef, StaffGrp, StaffGrpChild};

    // Build a complete orchestral-style score definition
    let mut score_def = ScoreDef::default();
    score_def.common.xml_id = Some("sd1".to_string());
    score_def.score_def_log.meter_count = Some("4".to_string());
    score_def.score_def_log.meter_unit = Some(4.0);
    score_def.score_def_log.meter_sym = Some(DataMetersign::Common);
    score_def.score_def_ges.ppq = Some(480);

    // Create outer bracket group
    let mut outer_grp = StaffGrp::default();
    outer_grp.common.xml_id = Some("sg-outer".to_string());
    outer_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Bracket);
    outer_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);

    // Create inner brace group for piano grand staff
    let mut piano_grp = StaffGrp::default();
    piano_grp.common.xml_id = Some("sg-piano".to_string());
    // Note: StaffGrp uses child <label> elements for labelling, not an attribute
    piano_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Brace);

    // Piano right hand staff
    let mut rh_staff = StaffDef::default();
    rh_staff.basic.xml_id = Some("sd-rh".to_string());
    rh_staff.n_integer.n = Some(1);
    rh_staff.staff_def_log.lines = Some(5);
    rh_staff.staff_def_log.notationtype = Some(DataNotationtype::Cmn);
    let mut rh_clef = Clef::default();
    rh_clef.clef_log.shape = Some(DataClefshape::G);
    rh_clef.clef_log.line = Some(DataClefline(2));
    rh_staff
        .children
        .push(tusk_model::elements::StaffDefChild::Clef(Box::new(rh_clef)));

    // Piano left hand staff
    let mut lh_staff = StaffDef::default();
    lh_staff.basic.xml_id = Some("sd-lh".to_string());
    lh_staff.n_integer.n = Some(2);
    lh_staff.staff_def_log.lines = Some(5);
    lh_staff.staff_def_log.notationtype = Some(DataNotationtype::Cmn);
    let mut lh_clef = Clef::default();
    lh_clef.clef_log.shape = Some(DataClefshape::F);
    lh_clef.clef_log.line = Some(DataClefline(4));
    lh_staff
        .children
        .push(tusk_model::elements::StaffDefChild::Clef(Box::new(lh_clef)));

    piano_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(rh_staff)));
    piano_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(lh_staff)));

    outer_grp
        .children
        .push(StaffGrpChild::StaffGrp(Box::new(piano_grp)));

    score_def
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(outer_grp)));

    // Round-trip
    let xml = score_def.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    // Verify top-level attributes
    assert_eq!(parsed.common.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
    assert_eq!(parsed.score_def_log.meter_sym, Some(DataMetersign::Common));
    assert_eq!(parsed.score_def_ges.ppq, Some(480));

    // Verify nested structure
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(outer) => {
            assert_eq!(outer.common.xml_id, Some("sg-outer".to_string()));
            assert_eq!(
                outer.staff_grp_vis.symbol,
                Some(AttStaffGrpVisSymbol::Bracket)
            );
            assert_eq!(outer.staff_grp_vis.bar_thru, Some(DataBoolean::True));
            assert_eq!(outer.children.len(), 1);

            match &outer.children[0] {
                StaffGrpChild::StaffGrp(piano) => {
                    assert_eq!(piano.common.xml_id, Some("sg-piano".to_string()));
                    // StaffGrp uses child <label> elements, not an attribute
                    assert_eq!(
                        piano.staff_grp_vis.symbol,
                        Some(AttStaffGrpVisSymbol::Brace)
                    );
                    assert_eq!(piano.children.len(), 2);

                    // Check staff definitions
                    match &piano.children[0] {
                        StaffGrpChild::StaffDef(sd) => {
                            assert_eq!(sd.basic.xml_id, Some("sd-rh".to_string()));
                            assert_eq!(sd.n_integer.n, Some(1));
                        }
                        other => panic!("Expected StaffDef, got {:?}", other),
                    }
                    match &piano.children[1] {
                        StaffGrpChild::StaffDef(sd) => {
                            assert_eq!(sd.basic.xml_id, Some("sd-lh".to_string()));
                            assert_eq!(sd.n_integer.n, Some(2));
                        }
                        other => panic!("Expected StaffDef, got {:?}", other),
                    }
                }
                other => panic!("Expected nested StaffGrp, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Parse test for staffDef with layerDef children
/// Note: StaffDef child serialization not yet implemented, so this is a parse test
#[test]
fn staffdef_parse_with_layerdef_children() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave};
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef xml:id="sd1" n="1" lines="5">
        <layerDef xml:id="ld1" n="1" label="Soprano" dur.default="4" oct.default="5"/>
        <layerDef xml:id="ld2" n="2" label="Alto" dur.default="4" oct.default="4"/>
    </staffDef>"#;

    let parsed = StaffDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 2);

    // Verify layer definitions
    match &parsed.children[0] {
        StaffDefChild::LayerDef(ld) => {
            assert_eq!(ld.basic.xml_id, Some("ld1".to_string()));
            assert_eq!(ld.n_integer.n, Some(1));
            assert_eq!(ld.labelled.label, Some("Soprano".to_string()));
            assert_eq!(
                ld.layer_def_log.dur_default,
                Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
            );
            assert_eq!(ld.layer_def_log.oct_default, Some(DataOctave(5)));
        }
        other => panic!("Expected LayerDef, got {:?}", other),
    }

    match &parsed.children[1] {
        StaffDefChild::LayerDef(ld) => {
            assert_eq!(ld.basic.xml_id, Some("ld2".to_string()));
            assert_eq!(ld.n_integer.n, Some(2));
            assert_eq!(ld.labelled.label, Some("Alto".to_string()));
            assert_eq!(ld.layer_def_log.oct_default, Some(DataOctave(4)));
        }
        other => panic!("Expected LayerDef, got {:?}", other),
    }
}

/// Parse test for score definition with transposing instruments
/// Note: This is a parse test, not round-trip, because StaffDef serialization
/// doesn't yet include trans.diat/trans.semi attributes
#[test]
fn scoredef_parse_transposing_instruments() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef xml:id="sd-trans">
        <staffGrp>
            <staffDef xml:id="sd-clarinet" label="Clarinet in Bb" n="1" lines="5"
                      clef.shape="G" clef.line="2" trans.diat="-1" trans.semi="-2"/>
            <staffDef xml:id="sd-horn" label="Horn in F" n="2" lines="5"
                      clef.shape="G" clef.line="2" trans.diat="-4" trans.semi="-7"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("sd-trans".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.children.len(), 2);

            // Check clarinet transposition
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.labelled.label, Some("Clarinet in Bb".to_string()));
                    assert_eq!(sd.staff_def_log.trans_diat, Some(-1));
                    assert_eq!(sd.staff_def_log.trans_semi, Some(-2));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }

            // Check horn transposition
            match &sg.children[1] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.labelled.label, Some("Horn in F".to_string()));
                    assert_eq!(sd.staff_def_log.trans_diat, Some(-4));
                    assert_eq!(sd.staff_def_log.trans_semi, Some(-7));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Test parsing scoreDef with multiple child types (keySig, meterSig, staffGrp)
/// Verifies that different child element types are correctly identified and ordered
#[test]
fn scoredef_parse_mixed_children() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef xml:id="sd-mixed">
        <keySig xml:id="ks1"/>
        <meterSig xml:id="ms1"/>
        <staffGrp xml:id="sg1">
            <staffDef n="1" lines="5"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("sd-mixed".to_string()));
    assert_eq!(parsed.children.len(), 3);

    // Verify child order preserved
    match &parsed.children[0] {
        ScoreDefChild::KeySig(ks) => {
            assert_eq!(ks.common.xml_id, Some("ks1".to_string()));
        }
        other => panic!("Expected KeySig, got {:?}", other),
    }

    match &parsed.children[1] {
        ScoreDefChild::MeterSig(ms) => {
            // Note: MeterSig-specific attributes (count, unit) not yet parsed
            assert_eq!(ms.common.xml_id, Some("ms1".to_string()));
        }
        other => panic!("Expected MeterSig, got {:?}", other),
    }

    match &parsed.children[2] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.children.len(), 1);
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.n_integer.n, Some(1));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Test staffGrp structure with nested staffDef children
/// Note: grpSym parsing not yet implemented, so testing staffDef children only
#[test]
fn staffgrp_parse_with_staffdef_children() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef>
        <staffGrp xml:id="sg1">
            <staffDef n="1" lines="5"/>
            <staffDef n="2" lines="5"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.children.len(), 2);

            // First child should be staffDef n="1"
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.n_integer.n, Some(1));
                    assert_eq!(sd.staff_def_log.lines, Some(5));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }

            // Second child should be staffDef n="2"
            match &sg.children[1] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.n_integer.n, Some(2));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Test staffDef with instrDef child
#[test]
fn staffdef_parse_with_instrdef_child() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef xml:id="sd1" n="1" lines="5">
        <instrDef xml:id="id1" midi.instrnum="1" />
    </staffDef>"#;

    let parsed = StaffDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        tusk_model::elements::StaffDefChild::InstrDef(id) => {
            assert_eq!(id.basic.xml_id, Some("id1".to_string()));
            // Note: InstrDef midi_instrnum not yet parsed from attributes
        }
        other => panic!("Expected InstrDef, got {:?}", other),
    }
}

/// Test staffDef with label and labelAbbr children
#[test]
fn staffdef_parse_with_label_children() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef xml:id="sd1" n="1" lines="5">
        <label xml:id="l1">Violin I</label>
        <labelAbbr xml:id="la1">Vln. I</labelAbbr>
    </staffDef>"#;

    let parsed = StaffDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        StaffDefChild::Label(label) => {
            assert_eq!(label.common.xml_id, Some("l1".to_string()));
            // Label content should be in text children
        }
        other => panic!("Expected Label, got {:?}", other),
    }

    match &parsed.children[1] {
        StaffDefChild::LabelAbbr(la) => {
            assert_eq!(la.common.xml_id, Some("la1".to_string()));
        }
        other => panic!("Expected LabelAbbr, got {:?}", other),
    }
}

/// Test layerDef with instrDef child
#[test]
fn layerdef_parse_with_instrdef_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef xml:id="ld1" n="1">
        <instrDef xml:id="id1" />
    </layerDef>"#;

    let parsed = LayerDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerDefChild::InstrDef(id) => {
            assert_eq!(id.basic.xml_id, Some("id1".to_string()));
            // Note: InstrDef midi attributes not yet parsed
        }
        other => panic!("Expected InstrDef, got {:?}", other),
    }
}

/// Test parsing layerDef with multiple children
/// Note: LayerDef child serialization not yet implemented, so this is a parse test
#[test]
fn layerdef_parse_with_multiple_children() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef xml:id="ld1" n="1">
        <label xml:id="l1">Voice I</label>
        <labelAbbr xml:id="la1">V.I</labelAbbr>
        <instrDef xml:id="id1"/>
        <meterSig xml:id="ms1"/>
    </layerDef>"#;

    let parsed = LayerDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.children.len(), 4);

    // Verify child types preserved in order
    assert!(matches!(&parsed.children[0], LayerDefChild::Label(_)));
    assert!(matches!(&parsed.children[1], LayerDefChild::LabelAbbr(_)));
    assert!(matches!(&parsed.children[2], LayerDefChild::InstrDef(_)));
    assert!(matches!(&parsed.children[3], LayerDefChild::MeterSig(_)));
}

// ============================================================================
// Control Event Tests - Slur
// ============================================================================

#[test]
fn slur_parse_empty() {
    use tusk_model::elements::Slur;

    let xml = r#"<slur/>"#;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.slur_log.startid.is_none());
    assert!(parsed.slur_log.endid.is_none());
}

#[test]
fn slur_parse_with_id() {
    use tusk_model::elements::Slur;

    let xml = r#"<slur xml:id="slur-1"/>"#;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("slur-1".to_string()));
}

#[test]
fn slur_parse_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Slur;

    let xml = r##"<slur xml:id="slur-1" startid="#note1" endid="#note2"/>"##;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("slur-1".to_string()));
    assert_eq!(parsed.slur_log.startid, Some(DataUri("#note1".to_string())));
    assert_eq!(parsed.slur_log.endid, Some(DataUri("#note2".to_string())));
}

#[test]
fn slur_parse_with_staff_layer() {
    use tusk_model::elements::Slur;

    let xml = r#"<slur staff="1" layer="1"/>"#;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.slur_log.staff, vec![1]);
    assert_eq!(parsed.slur_log.layer, vec![1]);
}

#[test]
fn slur_parse_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Slur;

    let xml = r#"<slur tstamp="1" tstamp2="0m+4"/>"#;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.slur_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.slur_log.tstamp2,
        Some(DataMeasurebeat("0m+4".to_string()))
    );
}

#[test]
fn slur_parse_complete() {
    use tusk_model::elements::Slur;

    let xml = r##"<slur xml:id="slur1" startid="#n1" endid="#n4" staff="1" layer="1"/>"##;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("slur1".to_string()));
    assert!(parsed.slur_log.startid.is_some());
    assert!(parsed.slur_log.endid.is_some());
    assert_eq!(parsed.slur_log.staff, vec![1]);
    assert_eq!(parsed.slur_log.layer, vec![1]);
}

// ============================================================================
// Control Event Tests - Tie
// ============================================================================

#[test]
fn tie_parse_empty() {
    use tusk_model::elements::Tie;

    let xml = r#"<tie/>"#;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.tie_log.startid.is_none());
    assert!(parsed.tie_log.endid.is_none());
}

#[test]
fn tie_parse_with_id() {
    use tusk_model::elements::Tie;

    let xml = r#"<tie xml:id="tie-1"/>"#;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tie-1".to_string()));
}

#[test]
fn tie_parse_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Tie;

    let xml = r##"<tie xml:id="tie-1" startid="#note1" endid="#note2"/>"##;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tie-1".to_string()));
    assert_eq!(parsed.tie_log.startid, Some(DataUri("#note1".to_string())));
    assert_eq!(parsed.tie_log.endid, Some(DataUri("#note2".to_string())));
}

#[test]
fn tie_parse_with_staff_layer() {
    use tusk_model::elements::Tie;

    let xml = r#"<tie staff="1" layer="1"/>"#;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tie_log.staff, vec![1]);
    assert_eq!(parsed.tie_log.layer, vec![1]);
}

#[test]
fn tie_parse_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Tie;

    let xml = r#"<tie tstamp="2.5" tstamp2="1m+1"/>"#;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tie_log.tstamp, Some(DataBeat(2.5)));
    assert_eq!(
        parsed.tie_log.tstamp2,
        Some(DataMeasurebeat("1m+1".to_string()))
    );
}

#[test]
fn tie_parse_complete() {
    use tusk_model::elements::Tie;

    let xml = r##"<tie xml:id="tie1" startid="#n1" endid="#n2" staff="1"/>"##;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tie1".to_string()));
    assert!(parsed.tie_log.startid.is_some());
    assert!(parsed.tie_log.endid.is_some());
    assert_eq!(parsed.tie_log.staff, vec![1]);
}

// ============================================================================
// Control Event Tests - Dynam
// ============================================================================

#[test]
fn dynam_parse_empty() {
    use tusk_model::elements::Dynam;

    let xml = r#"<dynam/>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn dynam_parse_with_id() {
    use tusk_model::elements::Dynam;

    let xml = r#"<dynam xml:id="dyn-1"/>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("dyn-1".to_string()));
}

#[test]
fn dynam_parse_with_text() {
    use tusk_model::elements::{Dynam, DynamChild};

    let xml = r#"<dynam xml:id="dyn-1">ff</dynam>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("dyn-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DynamChild::Text(t) => assert_eq!(t, "ff"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn dynam_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Dynam;

    let xml = r#"<dynam staff="1" tstamp="1"/>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.dynam_log.staff, vec![1]);
    assert_eq!(parsed.dynam_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn dynam_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Dynam;

    let xml = r##"<dynam startid="#note1"/>"##;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.dynam_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn dynam_parse_complete() {
    use tusk_model::elements::{Dynam, DynamChild};

    let xml = r#"<dynam xml:id="d1" staff="1" tstamp="1">p</dynam>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("d1".to_string()));
    assert_eq!(parsed.dynam_log.staff, vec![1]);
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DynamChild::Text(t) => assert_eq!(t, "p"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn dynam_parse_with_tstamp2() {
    use tusk_model::elements::{Dynam, DynamChild};

    // Test a dynamic with crescendo text and tstamp2
    let xml = r#"<dynam xml:id="d1" staff="1" tstamp="1" tstamp2="0m+4">cresc.</dynam>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("d1".to_string()));
    assert!(parsed.dynam_log.tstamp.is_some());
    assert!(parsed.dynam_log.tstamp2.is_some());
    match &parsed.children[0] {
        DynamChild::Text(t) => assert_eq!(t, "cresc."),
        _ => panic!("Expected text child"),
    }
}

// ============================================================================
// Control Event Tests - Hairpin
// ============================================================================

#[test]
fn hairpin_parse_empty() {
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.hairpin_log.form.is_none());
}

#[test]
fn hairpin_parse_with_id() {
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin xml:id="hp-1"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("hp-1".to_string()));
}

#[test]
fn hairpin_parse_crescendo() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin xml:id="hp-1" form="cres"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("hp-1".to_string()));
    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Cres));
}

#[test]
fn hairpin_parse_diminuendo() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin xml:id="hp-1" form="dim"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("hp-1".to_string()));
    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Dim));
}

#[test]
fn hairpin_parse_with_niente() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin form="dim" niente="true"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Dim));
    assert_eq!(parsed.hairpin_log.niente, Some(DataBoolean::True));
}

#[test]
fn hairpin_parse_with_staff_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin staff="1" tstamp="1" tstamp2="0m+3"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.hairpin_log.staff, vec![1]);
    assert_eq!(parsed.hairpin_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.hairpin_log.tstamp2,
        Some(DataMeasurebeat("0m+3".to_string()))
    );
}

#[test]
fn hairpin_parse_complete() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin xml:id="hp1" form="cres" staff="1" tstamp="1" tstamp2="0m+4"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("hp1".to_string()));
    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Cres));
    assert_eq!(parsed.hairpin_log.staff, vec![1]);
    assert!(parsed.hairpin_log.tstamp.is_some());
    assert!(parsed.hairpin_log.tstamp2.is_some());
}

// ============================================================================
// Control Event Tests - Dir
// ============================================================================

#[test]
fn dir_parse_empty() {
    use tusk_model::elements::Dir;

    let xml = r#"<dir/>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn dir_parse_with_id() {
    use tusk_model::elements::Dir;

    let xml = r#"<dir xml:id="dir-1"/>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("dir-1".to_string()));
}

#[test]
fn dir_parse_with_text() {
    use tusk_model::elements::{Dir, DirChild};

    let xml = r#"<dir xml:id="dir-1">legato</dir>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("dir-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DirChild::Text(t) => assert_eq!(t, "legato"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn dir_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Dir;

    let xml = r#"<dir staff="1" tstamp="1"/>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.dir_log.staff, vec![1]);
    assert_eq!(parsed.dir_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn dir_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Dir;

    let xml = r##"<dir startid="#note1"/>"##;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.dir_log.startid, Some(DataUri("#note1".to_string())));
}

#[test]
fn dir_parse_complete() {
    use tusk_model::elements::{Dir, DirChild};

    let xml = r#"<dir xml:id="d1" staff="1" tstamp="1">dolce</dir>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("d1".to_string()));
    assert_eq!(parsed.dir_log.staff, vec![1]);
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DirChild::Text(t) => assert_eq!(t, "dolce"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn dir_parse_with_endid() {
    use tusk_model::elements::{Dir, DirChild};

    // Test a directive with extended duration
    let xml = r##"<dir xml:id="d1" staff="1" tstamp="1" endid="#n4">sempre legato</dir>"##;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("d1".to_string()));
    assert!(parsed.dir_log.tstamp.is_some());
    assert!(parsed.dir_log.endid.is_some());
    match &parsed.children[0] {
        DirChild::Text(t) => assert_eq!(t, "sempre legato"),
        _ => panic!("Expected text child"),
    }
}

// ============================================================================
// Control Event Tests - Tempo
// ============================================================================

#[test]
fn tempo_parse_empty() {
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn tempo_parse_with_id() {
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo xml:id="tempo-1"/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tempo-1".to_string()));
}

#[test]
fn tempo_parse_with_text() {
    use tusk_model::elements::{Tempo, TempoChild};

    let xml = r#"<tempo xml:id="tempo-1">Allegro</tempo>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tempo-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        TempoChild::Text(t) => assert_eq!(t, "Allegro"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn tempo_parse_with_mm() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataTempovalue};
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo xml:id="tempo-1" mm="120" mm.unit="4"/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tempo-1".to_string()));
    assert_eq!(parsed.tempo_log.mm, Some(DataTempovalue(120.0)));
    assert_eq!(
        parsed.tempo_log.mm_unit,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn tempo_parse_with_func() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo func="instantaneous"/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tempo_log.func, Some(AttTempoLogFunc::Instantaneous));
}

#[test]
fn tempo_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo staff="1" tstamp="1"/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tempo_log.staff, vec![1]);
    assert_eq!(parsed.tempo_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn tempo_parse_complete() {
    use tusk_model::elements::{Tempo, TempoChild};

    let xml = r#"<tempo xml:id="t1" staff="1" tstamp="1" mm="120" mm.unit="4">Allegro</tempo>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
    assert_eq!(parsed.tempo_log.staff, vec![1]);
    assert!(parsed.tempo_log.mm.is_some());
    assert!(parsed.tempo_log.mm_unit.is_some());
    match &parsed.children[0] {
        TempoChild::Text(t) => assert_eq!(t, "Allegro"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn tempo_parse_continuous_func() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::elements::{Tempo, TempoChild};

    // Test a tempo marking with continuous function (like rit. or accel.)
    let xml =
        r#"<tempo xml:id="t1" staff="1" tstamp="1" tstamp2="0m+4" func="continuous">rit.</tempo>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tempo_log.func, Some(AttTempoLogFunc::Continuous));
    assert!(parsed.tempo_log.tstamp2.is_some());
    match &parsed.children[0] {
        TempoChild::Text(t) => assert_eq!(t, "rit."),
        _ => panic!("Expected text child"),
    }
}

// ============================================================================
// Control Event Tests - Fermata
// ============================================================================

#[test]
fn fermata_parse_empty() {
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.fermata_vis.form.is_none());
    assert!(parsed.fermata_vis.shape.is_none());
}

#[test]
fn fermata_parse_with_id() {
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata xml:id="ferm-1"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("ferm-1".to_string()));
}

#[test]
fn fermata_parse_with_form_norm() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata xml:id="ferm-1" form="norm"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("ferm-1".to_string()));
    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Norm));
}

#[test]
fn fermata_parse_with_form_inv() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata form="inv"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Inv));
}

#[test]
fn fermata_parse_with_shape_curved() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata shape="curved"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Curved));
}

#[test]
fn fermata_parse_with_shape_square() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata shape="square"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Square));
}

#[test]
fn fermata_parse_with_shape_angular() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata shape="angular"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Angular));
}

#[test]
fn fermata_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata staff="1" tstamp="4"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_log.staff, vec![1]);
    assert_eq!(parsed.fermata_log.tstamp, Some(DataBeat(4.0)));
}

#[test]
fn fermata_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Fermata;

    let xml = r##"<fermata startid="#note1"/>"##;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.fermata_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn fermata_parse_complete() {
    use tusk_model::att::{AttFermataVisForm, AttFermataVisShape};
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata xml:id="f1" staff="1" tstamp="4" form="norm" shape="curved"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("f1".to_string()));
    assert_eq!(parsed.fermata_log.staff, vec![1]);
    assert!(parsed.fermata_log.tstamp.is_some());
    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Norm));
    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Curved));
}

#[test]
fn fermata_parse_inverted() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let xml = r##"<fermata xml:id="f1" startid="#n1" form="inv"/>"##;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert!(parsed.fermata_log.startid.is_some());
    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Inv));
}

// ============================================================================
// Grouping Element Tests - GraceGrp
// ============================================================================

#[test]
fn gracegrp_parse_empty() {
    use tusk_model::elements::GraceGrp;

    let xml = r#"<graceGrp/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.grace_grp_log.grace.is_none());
    assert!(parsed.grace_grp_log.attach.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn gracegrp_parse_with_id() {
    use tusk_model::elements::GraceGrp;

    let xml = r#"<graceGrp xml:id="gg1"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("gg1".to_string()));
}

#[test]
fn gracegrp_parse_with_grace_unknown() {
    use tusk_model::data::DataGrace;
    use tusk_model::elements::GraceGrp;

    let xml = r#"<graceGrp grace="unknown"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Unknown));
}

#[test]
fn gracegrp_parse_with_grace_unacc() {
    use tusk_model::data::DataGrace;
    use tusk_model::elements::GraceGrp;

    let xml = r#"<graceGrp grace="unacc"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Unacc));
}

#[test]
fn gracegrp_parse_with_grace_acc() {
    use tusk_model::data::DataGrace;
    use tusk_model::elements::GraceGrp;

    let xml = r#"<graceGrp grace="acc"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Acc));
}

#[test]
fn gracegrp_parse_with_attach_pre() {
    use tusk_model::att::AttGraceGrpLogAttach;
    use tusk_model::elements::GraceGrp;

    let xml = r#"<graceGrp attach="pre"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.grace_grp_log.attach, Some(AttGraceGrpLogAttach::Pre));
}

#[test]
fn gracegrp_parse_with_attach_post() {
    use tusk_model::att::AttGraceGrpLogAttach;
    use tusk_model::elements::GraceGrp;

    // "post" attach indicates a Nachschlag (grace notes after main note)
    let xml = r#"<graceGrp attach="post"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.grace_grp_log.attach,
        Some(AttGraceGrpLogAttach::Post)
    );
}

#[test]
fn gracegrp_parse_with_staff_layer() {
    use tusk_model::elements::GraceGrp;

    let xml = r#"<graceGrp staff="1" layer="1"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.grace_grp_log.staff, vec![1]);
    assert_eq!(parsed.grace_grp_log.layer, vec![1]);
}

#[test]
fn gracegrp_parse_with_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::GraceGrp;

    let xml = r#"<graceGrp tstamp="2.5"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.grace_grp_log.tstamp, Some(DataBeat(2.5)));
}

#[test]
fn gracegrp_parse_with_grace_time() {
    use tusk_model::data::DataPercent;
    use tusk_model::elements::GraceGrp;

    // grace.time records the amount of time to be "stolen" from a non-grace note
    let xml = r#"<graceGrp grace.time="25%"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.grace_grp_log.grace_time,
        Some(DataPercent("25%".to_string()))
    );
}

#[test]
fn gracegrp_parse_with_single_note_child() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    let xml = r#"<graceGrp xml:id="gg1" grace="acc">
        <note xml:id="n1" pname="d" oct="5" dur="8"/>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("gg1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::Note(note) => {
            assert_eq!(note.common.xml_id, Some("n1".to_string()));
        }
        _ => panic!("Expected Note child"),
    }
}

#[test]
fn gracegrp_parse_with_multiple_note_children() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    let xml = r#"<graceGrp xml:id="gg1" grace="unacc">
        <note xml:id="n1" pname="d" oct="5" dur="16"/>
        <note xml:id="n2" pname="e" oct="5" dur="16"/>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 2);
    for child in &parsed.children {
        match child {
            GraceGrpChild::Note(_) => {}
            _ => panic!("Expected Note children"),
        }
    }
}

#[test]
fn gracegrp_parse_with_chord_child() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    let xml = r#"<graceGrp xml:id="gg1">
        <chord xml:id="c1" dur="8">
            <note pname="c" oct="4"/>
            <note pname="e" oct="4"/>
        </chord>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::Chord(chord) => {
            assert_eq!(chord.common.xml_id, Some("c1".to_string()));
        }
        _ => panic!("Expected Chord child"),
    }
}

#[test]
fn gracegrp_parse_with_beam_child() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    // Grace notes are often beamed together
    let xml = r#"<graceGrp xml:id="gg1" grace="acc">
        <beam>
            <note xml:id="n1" pname="a" oct="4" dur="16"/>
            <note xml:id="n2" pname="b" oct="4" dur="16"/>
        </beam>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::Beam(beam) => {
            assert_eq!(beam.children.len(), 2);
        }
        _ => panic!("Expected Beam child"),
    }
}

#[test]
fn gracegrp_parse_with_rest_child() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    // While unusual, rests can appear in graceGrp
    let xml = r#"<graceGrp xml:id="gg1">
        <rest xml:id="r1" dur="16"/>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::Rest(rest) => {
            assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        }
        _ => panic!("Expected Rest child"),
    }
}

#[test]
fn gracegrp_parse_with_space_child() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    let xml = r#"<graceGrp xml:id="gg1">
        <space xml:id="sp1" dur="16"/>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::Space(_) => {}
        _ => panic!("Expected Space child"),
    }
}

#[test]
fn gracegrp_parse_with_tuplet_child() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    // Grace notes can be in tuplet groupings
    let xml = r#"<graceGrp xml:id="gg1" grace="acc">
        <tuplet num="3" numbase="2">
            <note pname="c" oct="5" dur="16"/>
            <note pname="d" oct="5" dur="16"/>
            <note pname="e" oct="5" dur="16"/>
        </tuplet>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::Tuplet(tuplet) => {
            assert_eq!(tuplet.children.len(), 3);
        }
        _ => panic!("Expected Tuplet child"),
    }
}

#[test]
fn gracegrp_parse_nested_gracegrp() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    // Nested graceGrp elements are allowed by MEI schema
    let xml = r#"<graceGrp xml:id="gg1">
        <graceGrp xml:id="gg2">
            <note pname="c" oct="4"/>
        </graceGrp>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::GraceGrp(inner) => {
            assert_eq!(inner.common.xml_id, Some("gg2".to_string()));
        }
        _ => panic!("Expected nested GraceGrp child"),
    }
}

#[test]
fn gracegrp_parse_with_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::GraceGrp;

    // Visual attribute from AttGraceGrpVis
    let xml = r#"<graceGrp color="red"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.grace_grp_vis.color,
        Some(DataColor::DataColornames(DataColornames::Red))
    );
}

#[test]
fn gracegrp_parse_complete() {
    use tusk_model::att::AttGraceGrpLogAttach;
    use tusk_model::data::DataGrace;
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    let xml = r#"<graceGrp xml:id="gg1" grace="acc" attach="pre" staff="1" layer="1">
        <note xml:id="n1" pname="d" oct="5" dur="8"/>
        <note xml:id="n2" pname="c" oct="5" dur="8"/>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("gg1".to_string()));
    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Acc));
    assert_eq!(parsed.grace_grp_log.attach, Some(AttGraceGrpLogAttach::Pre));
    assert_eq!(parsed.grace_grp_log.staff, vec![1]);
    assert_eq!(parsed.grace_grp_log.layer, vec![1]);
    assert_eq!(parsed.children.len(), 2);
    for child in &parsed.children {
        match child {
            GraceGrpChild::Note(_) => {}
            _ => panic!("Expected Note children"),
        }
    }
}

// ============================================================================
// Grouping Element Tests - Beam
// ============================================================================

#[test]
fn beam_parse_empty() {
    use tusk_model::elements::Beam;

    let xml = r#"<beam/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.beam_log.staff.is_empty());
    assert!(parsed.beam_log.layer.is_empty());
    assert!(parsed.children.is_empty());
}

#[test]
fn beam_parse_with_id() {
    use tusk_model::elements::Beam;

    let xml = r#"<beam xml:id="beam-1"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("beam-1".to_string()));
}

#[test]
fn beam_parse_with_notes() {
    use tusk_model::elements::{Beam, BeamChild};

    let xml = r#"<beam xml:id="b1">
        <note xml:id="n1" pname="c" oct="4" dur="8"/>
        <note xml:id="n2" pname="d" oct="4" dur="8"/>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("b1".to_string()));
    assert_eq!(parsed.children.len(), 2);
    for child in &parsed.children {
        match child {
            BeamChild::Note(_) => {}
            _ => panic!("Expected Note children"),
        }
    }
}

#[test]
fn beam_parse_with_staff_layer() {
    use tusk_model::elements::Beam;

    let xml = r#"<beam staff="1" layer="1"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_log.staff, vec![1]);
    assert_eq!(parsed.beam_log.layer, vec![1]);
}

#[test]
fn beam_parse_with_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Beam;

    let xml = r#"<beam tstamp="2.5"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_log.tstamp, Some(DataBeat(2.5)));
}

#[test]
fn beam_parse_with_cross_staff() {
    use tusk_model::data::DataNeighboringlayer;
    use tusk_model::elements::Beam;

    let xml = r#"<beam beam.with="above"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_log.beam_with, Some(DataNeighboringlayer::Above));
}

#[test]
fn beam_parse_with_cross_staff_below() {
    use tusk_model::data::DataNeighboringlayer;
    use tusk_model::elements::Beam;

    let xml = r#"<beam beam.with="below"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_log.beam_with, Some(DataNeighboringlayer::Below));
}

#[test]
fn beam_parse_with_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::Beam;

    let xml = r#"<beam color="red"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.beam_vis.color,
        Some(DataColor::DataColornames(DataColornames::Red))
    );
}

#[test]
fn beam_parse_with_form_acc() {
    use tusk_model::att::AttBeamVisForm;
    use tusk_model::elements::Beam;

    // Accelerando (feathered beam opening)
    let xml = r#"<beam form="acc"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.form, Some(AttBeamVisForm::Acc));
}

#[test]
fn beam_parse_with_form_rit() {
    use tusk_model::att::AttBeamVisForm;
    use tusk_model::elements::Beam;

    // Ritardando (feathered beam closing)
    let xml = r#"<beam form="rit"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.form, Some(AttBeamVisForm::Rit));
}

#[test]
fn beam_parse_with_form_norm() {
    use tusk_model::att::AttBeamVisForm;
    use tusk_model::elements::Beam;

    let xml = r#"<beam form="norm"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.form, Some(AttBeamVisForm::Norm));
}

#[test]
fn beam_parse_with_form_mixed() {
    use tusk_model::att::AttBeamVisForm;
    use tusk_model::elements::Beam;

    let xml = r#"<beam form="mixed"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.form, Some(AttBeamVisForm::Mixed));
}

#[test]
fn beam_parse_with_place_above() {
    use tusk_model::data::DataBeamplace;
    use tusk_model::elements::Beam;

    let xml = r#"<beam place="above"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.place, Some(DataBeamplace::Above));
}

#[test]
fn beam_parse_with_place_below() {
    use tusk_model::data::DataBeamplace;
    use tusk_model::elements::Beam;

    let xml = r#"<beam place="below"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.place, Some(DataBeamplace::Below));
}

#[test]
fn beam_parse_with_place_mixed() {
    use tusk_model::data::DataBeamplace;
    use tusk_model::elements::Beam;

    let xml = r#"<beam place="mixed"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.place, Some(DataBeamplace::Mixed));
}

#[test]
fn beam_parse_with_slash() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Beam;

    // Grace note beams often have slashes
    let xml = r#"<beam slash="true"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.slash, Some(DataBoolean::True));
}

#[test]
fn beam_parse_with_slope() {
    use tusk_model::elements::Beam;

    let xml = r#"<beam slope="0.5"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.slope, Some(0.5));
}

#[test]
fn beam_parse_with_cue() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Beam;

    let xml = r#"<beam cue="true"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.cue, Some(DataBoolean::True));
}

#[test]
fn beam_parse_with_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Beam;

    let xml = r#"<beam visible="false"/>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.beam_vis.visible, Some(DataBoolean::False));
}

#[test]
fn beam_parse_nested_beam() {
    use tusk_model::elements::{Beam, BeamChild};

    // Nested beams for 32nd notes within 16th note beam
    let xml = r#"<beam xml:id="b1">
        <note xml:id="n1" dur="16"/>
        <beam xml:id="b2">
            <note xml:id="n2" dur="32"/>
            <note xml:id="n3" dur="32"/>
        </beam>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("b1".to_string()));
    assert_eq!(parsed.children.len(), 2);
    match &parsed.children[1] {
        BeamChild::Beam(inner) => {
            assert_eq!(inner.common.xml_id, Some("b2".to_string()));
            assert_eq!(inner.children.len(), 2);
        }
        _ => panic!("Expected nested Beam"),
    }
}

#[test]
fn beam_parse_with_chord() {
    use tusk_model::elements::{Beam, BeamChild};

    let xml = r#"<beam xml:id="b1">
        <chord xml:id="c1" dur="8">
            <note pname="c" oct="4"/>
            <note pname="e" oct="4"/>
        </chord>
        <note xml:id="n1" pname="d" oct="4" dur="8"/>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 2);
    match &parsed.children[0] {
        BeamChild::Chord(chord) => {
            assert_eq!(chord.common.xml_id, Some("c1".to_string()));
        }
        _ => panic!("Expected Chord child"),
    }
}

#[test]
fn beam_parse_with_rest() {
    use tusk_model::elements::{Beam, BeamChild};

    let xml = r#"<beam xml:id="b1">
        <note xml:id="n1" pname="c" oct="4" dur="8"/>
        <rest xml:id="r1" dur="8"/>
        <note xml:id="n2" pname="d" oct="4" dur="8"/>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 3);
    match &parsed.children[1] {
        BeamChild::Rest(rest) => {
            assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        }
        _ => panic!("Expected Rest child"),
    }
}

#[test]
fn beam_parse_with_space() {
    use tusk_model::elements::{Beam, BeamChild};

    let xml = r#"<beam xml:id="b1">
        <note xml:id="n1" pname="c" oct="4" dur="8"/>
        <space xml:id="sp1" dur="8"/>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 2);
    match &parsed.children[1] {
        BeamChild::Space(_) => {}
        _ => panic!("Expected Space child"),
    }
}

#[test]
fn beam_parse_with_tuplet() {
    use tusk_model::elements::{Beam, BeamChild};

    // Beamed tuplet (common in CMN)
    let xml = r#"<beam xml:id="b1">
        <tuplet num="3" numbase="2">
            <note pname="c" oct="4" dur="8"/>
            <note pname="d" oct="4" dur="8"/>
            <note pname="e" oct="4" dur="8"/>
        </tuplet>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        BeamChild::Tuplet(tuplet) => {
            assert_eq!(tuplet.children.len(), 3);
        }
        _ => panic!("Expected Tuplet child"),
    }
}

#[test]
fn beam_parse_with_gracegrp() {
    use tusk_model::elements::{Beam, BeamChild};

    let xml = r#"<beam xml:id="b1">
        <graceGrp grace="acc">
            <note pname="a" oct="4" dur="16"/>
        </graceGrp>
        <note pname="b" oct="4" dur="8"/>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 2);
    match &parsed.children[0] {
        BeamChild::GraceGrp(_) => {}
        _ => panic!("Expected GraceGrp child"),
    }
}

#[test]
fn beam_parse_complete() {
    use tusk_model::att::AttBeamVisForm;
    use tusk_model::data::{DataBeamplace, DataBoolean, DataNeighboringlayer};
    use tusk_model::elements::{Beam, BeamChild};

    let xml = r#"<beam xml:id="b1" staff="1" layer="1" form="norm" place="above" slash="false" beam.with="below">
        <note xml:id="n1" pname="c" oct="5" dur="8"/>
        <note xml:id="n2" pname="d" oct="5" dur="8"/>
        <note xml:id="n3" pname="e" oct="5" dur="8"/>
        <note xml:id="n4" pname="f" oct="5" dur="8"/>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("b1".to_string()));
    assert_eq!(parsed.beam_log.staff, vec![1]);
    assert_eq!(parsed.beam_log.layer, vec![1]);
    assert_eq!(parsed.beam_vis.form, Some(AttBeamVisForm::Norm));
    assert_eq!(parsed.beam_vis.place, Some(DataBeamplace::Above));
    assert_eq!(parsed.beam_vis.slash, Some(DataBoolean::False));
    assert_eq!(parsed.beam_log.beam_with, Some(DataNeighboringlayer::Below));
    assert_eq!(parsed.children.len(), 4);
    for child in &parsed.children {
        match child {
            BeamChild::Note(_) => {}
            _ => panic!("Expected Note children"),
        }
    }
}

// ============================================================================
// Grouping Element Tests - Tuplet
// ============================================================================

#[test]
fn tuplet_parse_empty() {
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.tuplet_log.num.is_none());
    assert!(parsed.tuplet_log.numbase.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn tuplet_parse_with_id() {
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet xml:id="tuplet-1"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tuplet-1".to_string()));
}

#[test]
fn tuplet_parse_triplet() {
    use tusk_model::elements::{Tuplet, TupletChild};

    // Standard triplet: 3 in the space of 2
    let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
        <note xml:id="n1" pname="c" oct="4" dur="8"/>
        <note xml:id="n2" pname="d" oct="4" dur="8"/>
        <note xml:id="n3" pname="e" oct="4" dur="8"/>
    </tuplet>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
    assert_eq!(parsed.tuplet_log.num, Some(3));
    assert_eq!(parsed.tuplet_log.numbase, Some(2));
    assert_eq!(parsed.children.len(), 3);
    for child in &parsed.children {
        match child {
            TupletChild::Note(_) => {}
            _ => panic!("Expected Note children"),
        }
    }
}

#[test]
fn tuplet_parse_quintuplet() {
    use tusk_model::elements::Tuplet;

    // Quintuplet: 5 in the space of 4
    let xml = r#"<tuplet num="5" numbase="4"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_log.num, Some(5));
    assert_eq!(parsed.tuplet_log.numbase, Some(4));
}

#[test]
fn tuplet_parse_septuplet() {
    use tusk_model::elements::Tuplet;

    // Septuplet: 7 in the space of 4
    let xml = r#"<tuplet num="7" numbase="4"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_log.num, Some(7));
    assert_eq!(parsed.tuplet_log.numbase, Some(4));
}

#[test]
fn tuplet_parse_duplet() {
    use tusk_model::elements::Tuplet;

    // Duplet: 2 in the space of 3 (compound meter)
    let xml = r#"<tuplet num="2" numbase="3"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_log.num, Some(2));
    assert_eq!(parsed.tuplet_log.numbase, Some(3));
}

#[test]
fn tuplet_parse_with_staff_layer() {
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet staff="2" layer="1"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_log.staff, vec![2]);
    assert_eq!(parsed.tuplet_log.layer, vec![1]);
}

#[test]
fn tuplet_parse_with_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet tstamp="3"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_log.tstamp, Some(DataBeat(3.0)));
}

#[test]
fn tuplet_parse_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Tuplet;

    let xml = r##"<tuplet startid="#n1" endid="#n3"/>"##;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_log.startid, Some(DataUri("#n1".to_string())));
    assert_eq!(parsed.tuplet_log.endid, Some(DataUri("#n3".to_string())));
}

#[test]
fn tuplet_parse_with_dur() {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet dur="4"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.tuplet_log.dur,
        vec![DataDuration::DataDurationCmn(DataDurationCmn::N4)]
    );
}

#[test]
fn tuplet_parse_with_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet color="blue"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.tuplet_vis.color,
        Some(DataColor::DataColornames(DataColornames::Blue))
    );
}

#[test]
fn tuplet_parse_with_num_place_above() {
    use tusk_model::data::DataStaffrelBasic;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet num.place="above"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_vis.num_place, Some(DataStaffrelBasic::Above));
}

#[test]
fn tuplet_parse_with_num_place_below() {
    use tusk_model::data::DataStaffrelBasic;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet num.place="below"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_vis.num_place, Some(DataStaffrelBasic::Below));
}

#[test]
fn tuplet_parse_with_num_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet num.visible="true"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_vis.num_visible, Some(DataBoolean::True));
}

#[test]
fn tuplet_parse_with_num_visible_false() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet num.visible="false"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_vis.num_visible, Some(DataBoolean::False));
}

#[test]
fn tuplet_parse_with_bracket_place_above() {
    use tusk_model::data::DataStaffrelBasic;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet bracket.place="above"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.tuplet_vis.bracket_place,
        Some(DataStaffrelBasic::Above)
    );
}

#[test]
fn tuplet_parse_with_bracket_place_below() {
    use tusk_model::data::DataStaffrelBasic;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet bracket.place="below"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.tuplet_vis.bracket_place,
        Some(DataStaffrelBasic::Below)
    );
}

#[test]
fn tuplet_parse_with_bracket_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet bracket.visible="true"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_vis.bracket_visible, Some(DataBoolean::True));
}

#[test]
fn tuplet_parse_with_bracket_visible_false() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet bracket.visible="false"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tuplet_vis.bracket_visible, Some(DataBoolean::False));
}

#[test]
fn tuplet_parse_with_num_format_count() {
    use tusk_model::att::AttTupletVisNumFormat;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet num.format="count"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.tuplet_vis.num_format,
        Some(AttTupletVisNumFormat::Count)
    );
}

#[test]
fn tuplet_parse_with_num_format_ratio() {
    use tusk_model::att::AttTupletVisNumFormat;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet num.format="ratio"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.tuplet_vis.num_format,
        Some(AttTupletVisNumFormat::Ratio)
    );
}

#[test]
fn tuplet_parse_with_chord() {
    use tusk_model::elements::{Tuplet, TupletChild};

    let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
        <chord xml:id="c1" dur="8">
            <note pname="c" oct="4"/>
            <note pname="e" oct="4"/>
        </chord>
        <chord xml:id="c2" dur="8">
            <note pname="d" oct="4"/>
            <note pname="f" oct="4"/>
        </chord>
        <chord xml:id="c3" dur="8">
            <note pname="e" oct="4"/>
            <note pname="g" oct="4"/>
        </chord>
    </tuplet>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 3);
    for child in &parsed.children {
        match child {
            TupletChild::Chord(_) => {}
            _ => panic!("Expected Chord children"),
        }
    }
}

#[test]
fn tuplet_parse_with_rest() {
    use tusk_model::elements::{Tuplet, TupletChild};

    let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
        <note xml:id="n1" pname="c" oct="4" dur="8"/>
        <rest xml:id="r1" dur="8"/>
        <note xml:id="n2" pname="e" oct="4" dur="8"/>
    </tuplet>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 3);
    match &parsed.children[1] {
        TupletChild::Rest(rest) => {
            assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        }
        _ => panic!("Expected Rest child"),
    }
}

#[test]
fn tuplet_parse_with_beam() {
    use tusk_model::elements::{Tuplet, TupletChild};

    // Beamed tuplet is very common
    let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
        <beam>
            <note xml:id="n1" pname="c" oct="4" dur="8"/>
            <note xml:id="n2" pname="d" oct="4" dur="8"/>
            <note xml:id="n3" pname="e" oct="4" dur="8"/>
        </beam>
    </tuplet>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        TupletChild::Beam(beam) => {
            assert_eq!(beam.children.len(), 3);
        }
        _ => panic!("Expected Beam child"),
    }
}

#[test]
fn tuplet_parse_nested_tuplet() {
    use tusk_model::elements::{Tuplet, TupletChild};

    // Nested tuplets (e.g., triplet within triplet)
    let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
        <note xml:id="n1" pname="c" oct="4" dur="8"/>
        <tuplet xml:id="t2" num="3" numbase="2">
            <note xml:id="n2" pname="d" oct="4" dur="16"/>
            <note xml:id="n3" pname="e" oct="4" dur="16"/>
            <note xml:id="n4" pname="f" oct="4" dur="16"/>
        </tuplet>
        <note xml:id="n5" pname="g" oct="4" dur="8"/>
    </tuplet>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
    assert_eq!(parsed.children.len(), 3);
    match &parsed.children[1] {
        TupletChild::Tuplet(inner) => {
            assert_eq!(inner.common.xml_id, Some("t2".to_string()));
            assert_eq!(inner.children.len(), 3);
        }
        _ => panic!("Expected nested Tuplet"),
    }
}

#[test]
fn tuplet_parse_with_gracegrp() {
    use tusk_model::elements::{Tuplet, TupletChild};

    let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
        <graceGrp grace="acc">
            <note pname="b" oct="3" dur="16"/>
        </graceGrp>
        <note pname="c" oct="4" dur="8"/>
        <note pname="d" oct="4" dur="8"/>
        <note pname="e" oct="4" dur="8"/>
    </tuplet>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 4);
    match &parsed.children[0] {
        TupletChild::GraceGrp(_) => {}
        _ => panic!("Expected GraceGrp child"),
    }
}

#[test]
fn tuplet_parse_with_cross_staff() {
    use tusk_model::data::DataNeighboringlayer;
    use tusk_model::elements::Tuplet;

    let xml = r#"<tuplet beam.with="above"/>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.tuplet_log.beam_with,
        Some(DataNeighboringlayer::Above)
    );
}

#[test]
fn tuplet_parse_complete() {
    use tusk_model::att::AttTupletVisNumFormat;
    use tusk_model::data::{DataBoolean, DataStaffrelBasic};
    use tusk_model::elements::{Tuplet, TupletChild};

    let xml = r#"<tuplet xml:id="t1" num="3" numbase="2" staff="1" layer="1"
                         num.place="above" num.visible="true" num.format="count"
                         bracket.place="above" bracket.visible="true">
        <note xml:id="n1" pname="c" oct="5" dur="8"/>
        <note xml:id="n2" pname="d" oct="5" dur="8"/>
        <note xml:id="n3" pname="e" oct="5" dur="8"/>
    </tuplet>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
    assert_eq!(parsed.tuplet_log.num, Some(3));
    assert_eq!(parsed.tuplet_log.numbase, Some(2));
    assert_eq!(parsed.tuplet_log.staff, vec![1]);
    assert_eq!(parsed.tuplet_log.layer, vec![1]);
    assert_eq!(parsed.tuplet_vis.num_place, Some(DataStaffrelBasic::Above));
    assert_eq!(parsed.tuplet_vis.num_visible, Some(DataBoolean::True));
    assert_eq!(
        parsed.tuplet_vis.num_format,
        Some(AttTupletVisNumFormat::Count)
    );
    assert_eq!(
        parsed.tuplet_vis.bracket_place,
        Some(DataStaffrelBasic::Above)
    );
    assert_eq!(parsed.tuplet_vis.bracket_visible, Some(DataBoolean::True));
    assert_eq!(parsed.children.len(), 3);
    for child in &parsed.children {
        match child {
            TupletChild::Note(_) => {}
            _ => panic!("Expected Note children"),
        }
    }
}

// ============================================================================
// Grouping Element Tests - GraceGrp additional tests
// (Most GraceGrp tests are above; these add coverage for missing cases)
// ============================================================================

#[test]
fn gracegrp_parse_attach_unknown_value() {
    use tusk_model::att::AttGraceGrpLogAttach;
    use tusk_model::elements::GraceGrp;

    // Tests the "unknown" attach value (previous tests cover pre/post)
    let xml = r#"<graceGrp attach="unknown"/>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.grace_grp_log.attach,
        Some(AttGraceGrpLogAttach::Unknown)
    );
}

// ============================================================================
// Combined Grouping Element Tests (Complex Nesting)
// ============================================================================

#[test]
fn grouping_beam_containing_gracegrp_and_tuplet() {
    use tusk_model::elements::{Beam, BeamChild};

    // Complex nested structure: beam with grace notes and a tuplet
    let xml = r#"<beam xml:id="b1">
        <graceGrp grace="acc">
            <note pname="a" oct="4" dur="32"/>
        </graceGrp>
        <note xml:id="n1" pname="b" oct="4" dur="8"/>
        <tuplet num="3" numbase="2">
            <note pname="c" oct="5" dur="16"/>
            <note pname="d" oct="5" dur="16"/>
            <note pname="e" oct="5" dur="16"/>
        </tuplet>
        <note xml:id="n2" pname="f" oct="5" dur="8"/>
    </beam>"#;
    let parsed = Beam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 4);
    match &parsed.children[0] {
        BeamChild::GraceGrp(_) => {}
        _ => panic!("Expected GraceGrp"),
    }
    match &parsed.children[2] {
        BeamChild::Tuplet(t) => {
            assert_eq!(t.children.len(), 3);
        }
        _ => panic!("Expected Tuplet"),
    }
}

#[test]
fn grouping_tuplet_containing_beamed_groups() {
    use tusk_model::elements::{Tuplet, TupletChild};

    // Tuplet with multiple beamed groups inside
    let xml = r#"<tuplet xml:id="t1" num="3" numbase="2">
        <beam xml:id="b1">
            <note pname="c" oct="4" dur="16"/>
            <note pname="d" oct="4" dur="16"/>
        </beam>
        <beam xml:id="b2">
            <note pname="e" oct="4" dur="16"/>
            <note pname="f" oct="4" dur="16"/>
        </beam>
        <beam xml:id="b3">
            <note pname="g" oct="4" dur="16"/>
            <note pname="a" oct="4" dur="16"/>
        </beam>
    </tuplet>"#;
    let parsed = Tuplet::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 3);
    for child in &parsed.children {
        match child {
            TupletChild::Beam(b) => {
                assert_eq!(b.children.len(), 2);
            }
            _ => panic!("Expected Beam children"),
        }
    }
}

#[test]
fn grouping_gracegrp_containing_beamed_notes() {
    use tusk_model::elements::{GraceGrp, GraceGrpChild};

    // Grace notes often beamed together
    let xml = r#"<graceGrp xml:id="gg1" grace="acc" attach="pre">
        <beam>
            <note xml:id="n1" pname="f" oct="5" dur="32"/>
            <note xml:id="n2" pname="e" oct="5" dur="32"/>
            <note xml:id="n3" pname="d" oct="5" dur="32"/>
            <note xml:id="n4" pname="c" oct="5" dur="32"/>
        </beam>
    </graceGrp>"#;
    let parsed = GraceGrp::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::Beam(beam) => {
            assert_eq!(beam.children.len(), 4);
        }
        _ => panic!("Expected Beam child"),
    }
}

// ============================================================================
// Full Round-Trip Tests (parse → serialize → parse)
// These tests verify that MEI elements can be serialized to XML and
// deserialized back without data loss for all CMN elements.
// ============================================================================

// ----------------------------------------------------------------------------
// Slur Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_slur_empty() {
    use tusk_model::elements::Slur;

    let original = Slur::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.slur_log.startid.is_none());
    assert!(parsed.slur_log.endid.is_none());
}

#[test]
fn roundtrip_slur_with_xml_id() {
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.common.xml_id = Some("slur-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("slur-1".to_string()));
}

#[test]
fn roundtrip_slur_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.common.xml_id = Some("s1".to_string());
    original.slur_log.startid = Some(DataUri("#n1".to_string()));
    original.slur_log.endid = Some(DataUri("#n4".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.slur_log.startid, Some(DataUri("#n1".to_string())));
    assert_eq!(parsed.slur_log.endid, Some(DataUri("#n4".to_string())));
}

#[test]
fn roundtrip_slur_with_staff_layer() {
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.slur_log.staff = vec![1];
    original.slur_log.layer = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.slur_log.staff, vec![1]);
    assert_eq!(parsed.slur_log.layer, vec![1]);
}

#[test]
fn roundtrip_slur_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.slur_log.tstamp = Some(DataBeat(1.0));
    original.slur_log.tstamp2 = Some(DataMeasurebeat("0m+4".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.slur_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.slur_log.tstamp2,
        Some(DataMeasurebeat("0m+4".to_string()))
    );
}

#[test]
fn roundtrip_slur_with_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.slur_vis.color = Some(DataColor::DataColornames(DataColornames::Blue));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.slur_vis.color,
        Some(DataColor::DataColornames(DataColornames::Blue))
    );
}

#[test]
fn roundtrip_slur_complete() {
    use tusk_model::data::{DataBeat, DataColor, DataColornames, DataMeasurebeat, DataUri};
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.common.xml_id = Some("slur-complete".to_string());
    original.slur_log.startid = Some(DataUri("#n1".to_string()));
    original.slur_log.endid = Some(DataUri("#n8".to_string()));
    original.slur_log.staff = vec![1];
    original.slur_log.layer = vec![1];
    original.slur_log.tstamp = Some(DataBeat(1.0));
    original.slur_log.tstamp2 = Some(DataMeasurebeat("1m+1".to_string()));
    original.slur_vis.color = Some(DataColor::DataColornames(DataColornames::Red));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.slur_log.startid, original.slur_log.startid);
    assert_eq!(parsed.slur_log.endid, original.slur_log.endid);
    assert_eq!(parsed.slur_log.staff, original.slur_log.staff);
    assert_eq!(parsed.slur_log.layer, original.slur_log.layer);
    assert_eq!(parsed.slur_log.tstamp, original.slur_log.tstamp);
    assert_eq!(parsed.slur_log.tstamp2, original.slur_log.tstamp2);
    assert_eq!(parsed.slur_vis.color, original.slur_vis.color);
}

// ----------------------------------------------------------------------------
// Tie Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_tie_empty() {
    use tusk_model::elements::Tie;

    let original = Tie::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.tie_log.startid.is_none());
    assert!(parsed.tie_log.endid.is_none());
}

#[test]
fn roundtrip_tie_with_xml_id() {
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.common.xml_id = Some("tie-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tie-1".to_string()));
}

#[test]
fn roundtrip_tie_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.tie_log.startid = Some(DataUri("#n1".to_string()));
    original.tie_log.endid = Some(DataUri("#n2".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tie_log.startid, Some(DataUri("#n1".to_string())));
    assert_eq!(parsed.tie_log.endid, Some(DataUri("#n2".to_string())));
}

#[test]
fn roundtrip_tie_with_staff() {
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.tie_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tie_log.staff, vec![1]);
}

#[test]
fn roundtrip_tie_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.tie_log.tstamp = Some(DataBeat(2.5));
    original.tie_log.tstamp2 = Some(DataMeasurebeat("1m+1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tie_log.tstamp, Some(DataBeat(2.5)));
    assert_eq!(
        parsed.tie_log.tstamp2,
        Some(DataMeasurebeat("1m+1".to_string()))
    );
}

#[test]
fn roundtrip_tie_with_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.tie_vis.color = Some(DataColor::DataColornames(DataColornames::Blue));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.tie_vis.color,
        Some(DataColor::DataColornames(DataColornames::Blue))
    );
}

#[test]
fn roundtrip_tie_complete() {
    use tusk_model::data::{DataBeat, DataColor, DataColornames, DataMeasurebeat, DataUri};
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.common.xml_id = Some("tie-complete".to_string());
    original.tie_log.startid = Some(DataUri("#n1".to_string()));
    original.tie_log.endid = Some(DataUri("#n2".to_string()));
    original.tie_log.staff = vec![1];
    original.tie_log.layer = vec![1];
    original.tie_log.tstamp = Some(DataBeat(4.0));
    original.tie_log.tstamp2 = Some(DataMeasurebeat("1m+1".to_string()));
    original.tie_vis.color = Some(DataColor::DataColornames(DataColornames::Red));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.tie_log.startid, original.tie_log.startid);
    assert_eq!(parsed.tie_log.endid, original.tie_log.endid);
    assert_eq!(parsed.tie_log.staff, original.tie_log.staff);
    assert_eq!(parsed.tie_log.layer, original.tie_log.layer);
    assert_eq!(parsed.tie_log.tstamp, original.tie_log.tstamp);
    assert_eq!(parsed.tie_log.tstamp2, original.tie_log.tstamp2);
    assert_eq!(parsed.tie_vis.color, original.tie_vis.color);
}

// ----------------------------------------------------------------------------
// Dynam Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_dynam_empty() {
    use tusk_model::elements::Dynam;

    let original = Dynam::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_dynam_with_xml_id() {
    use tusk_model::elements::Dynam;

    let mut original = Dynam::default();
    original.common.xml_id = Some("dyn-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("dyn-1".to_string()));
}

#[test]
fn roundtrip_dynam_with_text() {
    use tusk_model::elements::{Dynam, DynamChild};

    let mut original = Dynam::default();
    original.common.xml_id = Some("d1".to_string());
    original.children.push(DynamChild::Text("ff".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DynamChild::Text(t) => assert_eq!(t, "ff"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn roundtrip_dynam_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Dynam;

    let mut original = Dynam::default();
    original.dynam_log.staff = vec![1];
    original.dynam_log.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dynam_log.staff, vec![1]);
    assert_eq!(parsed.dynam_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn roundtrip_dynam_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Dynam;

    let mut original = Dynam::default();
    original.dynam_log.startid = Some(DataUri("#note1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.dynam_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn roundtrip_dynam_with_tstamp2() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::{Dynam, DynamChild};

    let mut original = Dynam::default();
    original.dynam_log.tstamp = Some(DataBeat(1.0));
    original.dynam_log.tstamp2 = Some(DataMeasurebeat("0m+4".to_string()));
    original
        .children
        .push(DynamChild::Text("cresc.".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dynam_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.dynam_log.tstamp2,
        Some(DataMeasurebeat("0m+4".to_string()))
    );
}

#[test]
fn roundtrip_dynam_complete() {
    use tusk_model::data::{DataBeat, DataMeasurebeat, DataUri};
    use tusk_model::elements::{Dynam, DynamChild};

    let mut original = Dynam::default();
    original.common.xml_id = Some("dyn-complete".to_string());
    original.dynam_log.staff = vec![1, 2];
    original.dynam_log.layer = vec![1];
    original.dynam_log.tstamp = Some(DataBeat(1.0));
    original.dynam_log.tstamp2 = Some(DataMeasurebeat("2m+1".to_string()));
    original.dynam_log.startid = Some(DataUri("#n1".to_string()));
    original.children.push(DynamChild::Text("sfz".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.dynam_log.staff, original.dynam_log.staff);
    assert_eq!(parsed.dynam_log.layer, original.dynam_log.layer);
    assert_eq!(parsed.dynam_log.tstamp, original.dynam_log.tstamp);
    assert_eq!(parsed.dynam_log.tstamp2, original.dynam_log.tstamp2);
    assert_eq!(parsed.dynam_log.startid, original.dynam_log.startid);
    assert_eq!(parsed.children.len(), 1);
}

// ----------------------------------------------------------------------------
// Hairpin Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_hairpin_empty() {
    use tusk_model::elements::Hairpin;

    let original = Hairpin::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.hairpin_log.form.is_none());
}

#[test]
fn roundtrip_hairpin_with_xml_id() {
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.common.xml_id = Some("hp-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("hp-1".to_string()));
}

#[test]
fn roundtrip_hairpin_crescendo() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.hairpin_log.form = Some(AttHairpinLogForm::Cres);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Cres));
}

#[test]
fn roundtrip_hairpin_diminuendo() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.hairpin_log.form = Some(AttHairpinLogForm::Dim);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Dim));
}

#[test]
fn roundtrip_hairpin_with_niente() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.hairpin_log.form = Some(AttHairpinLogForm::Dim);
    original.hairpin_log.niente = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Dim));
    assert_eq!(parsed.hairpin_log.niente, Some(DataBoolean::True));
}

#[test]
fn roundtrip_hairpin_with_staff_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.hairpin_log.staff = vec![1];
    original.hairpin_log.tstamp = Some(DataBeat(1.0));
    original.hairpin_log.tstamp2 = Some(DataMeasurebeat("0m+3".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.hairpin_log.staff, vec![1]);
    assert_eq!(parsed.hairpin_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.hairpin_log.tstamp2,
        Some(DataMeasurebeat("0m+3".to_string()))
    );
}

#[test]
fn roundtrip_hairpin_complete() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::data::{DataBeat, DataBoolean, DataMeasurebeat, DataUri};
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.common.xml_id = Some("hp-complete".to_string());
    original.hairpin_log.form = Some(AttHairpinLogForm::Cres);
    original.hairpin_log.niente = Some(DataBoolean::False);
    original.hairpin_log.staff = vec![1];
    original.hairpin_log.layer = vec![1];
    original.hairpin_log.tstamp = Some(DataBeat(1.0));
    original.hairpin_log.tstamp2 = Some(DataMeasurebeat("0m+4".to_string()));
    original.hairpin_log.startid = Some(DataUri("#n1".to_string()));
    original.hairpin_log.endid = Some(DataUri("#n4".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.hairpin_log.form, original.hairpin_log.form);
    assert_eq!(parsed.hairpin_log.niente, original.hairpin_log.niente);
    assert_eq!(parsed.hairpin_log.staff, original.hairpin_log.staff);
    assert_eq!(parsed.hairpin_log.layer, original.hairpin_log.layer);
    assert_eq!(parsed.hairpin_log.tstamp, original.hairpin_log.tstamp);
    assert_eq!(parsed.hairpin_log.tstamp2, original.hairpin_log.tstamp2);
    assert_eq!(parsed.hairpin_log.startid, original.hairpin_log.startid);
    assert_eq!(parsed.hairpin_log.endid, original.hairpin_log.endid);
}

// ----------------------------------------------------------------------------
// Dir Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_dir_empty() {
    use tusk_model::elements::Dir;

    let original = Dir::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_dir_with_xml_id() {
    use tusk_model::elements::Dir;

    let mut original = Dir::default();
    original.common.xml_id = Some("dir-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("dir-1".to_string()));
}

#[test]
fn roundtrip_dir_with_text() {
    use tusk_model::elements::{Dir, DirChild};

    let mut original = Dir::default();
    original.children.push(DirChild::Text("legato".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DirChild::Text(t) => assert_eq!(t, "legato"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn roundtrip_dir_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Dir;

    let mut original = Dir::default();
    original.dir_log.staff = vec![1];
    original.dir_log.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dir_log.staff, vec![1]);
    assert_eq!(parsed.dir_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn roundtrip_dir_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Dir;

    let mut original = Dir::default();
    original.dir_log.startid = Some(DataUri("#note1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dir_log.startid, Some(DataUri("#note1".to_string())));
}

#[test]
fn roundtrip_dir_with_endid() {
    use tusk_model::data::{DataBeat, DataUri};
    use tusk_model::elements::{Dir, DirChild};

    let mut original = Dir::default();
    original.dir_log.tstamp = Some(DataBeat(1.0));
    original.dir_log.endid = Some(DataUri("#n4".to_string()));
    original
        .children
        .push(DirChild::Text("sempre legato".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dir_log.endid, Some(DataUri("#n4".to_string())));
}

#[test]
fn roundtrip_dir_complete() {
    use tusk_model::data::{DataBeat, DataMeasurebeat, DataUri};
    use tusk_model::elements::{Dir, DirChild};

    let mut original = Dir::default();
    original.common.xml_id = Some("dir-complete".to_string());
    original.dir_log.staff = vec![1];
    original.dir_log.layer = vec![1];
    original.dir_log.tstamp = Some(DataBeat(1.0));
    original.dir_log.tstamp2 = Some(DataMeasurebeat("2m+1".to_string()));
    original.dir_log.startid = Some(DataUri("#n1".to_string()));
    original.dir_log.endid = Some(DataUri("#n8".to_string()));
    original
        .children
        .push(DirChild::Text("dolce espressivo".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.dir_log.staff, original.dir_log.staff);
    assert_eq!(parsed.dir_log.layer, original.dir_log.layer);
    assert_eq!(parsed.dir_log.tstamp, original.dir_log.tstamp);
    assert_eq!(parsed.dir_log.tstamp2, original.dir_log.tstamp2);
    assert_eq!(parsed.dir_log.startid, original.dir_log.startid);
    assert_eq!(parsed.dir_log.endid, original.dir_log.endid);
    assert_eq!(parsed.children.len(), 1);
}

// ----------------------------------------------------------------------------
// Tempo Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_tempo_empty() {
    use tusk_model::elements::Tempo;

    let original = Tempo::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_tempo_with_xml_id() {
    use tusk_model::elements::Tempo;

    let mut original = Tempo::default();
    original.common.xml_id = Some("tempo-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tempo-1".to_string()));
}

#[test]
fn roundtrip_tempo_with_text() {
    use tusk_model::elements::{Tempo, TempoChild};

    let mut original = Tempo::default();
    original
        .children
        .push(TempoChild::Text("Allegro".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        TempoChild::Text(t) => assert_eq!(t, "Allegro"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn roundtrip_tempo_with_mm() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataTempovalue};
    use tusk_model::elements::Tempo;

    let mut original = Tempo::default();
    original.tempo_log.mm = Some(DataTempovalue(120.0));
    original.tempo_log.mm_unit = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tempo_log.mm, Some(DataTempovalue(120.0)));
    assert_eq!(
        parsed.tempo_log.mm_unit,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn roundtrip_tempo_with_func() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::elements::Tempo;

    let mut original = Tempo::default();
    original.tempo_log.func = Some(AttTempoLogFunc::Instantaneous);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tempo_log.func, Some(AttTempoLogFunc::Instantaneous));
}

#[test]
fn roundtrip_tempo_continuous_func() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::{Tempo, TempoChild};

    let mut original = Tempo::default();
    original.tempo_log.func = Some(AttTempoLogFunc::Continuous);
    original.tempo_log.tstamp = Some(DataBeat(1.0));
    original.tempo_log.tstamp2 = Some(DataMeasurebeat("0m+4".to_string()));
    original.children.push(TempoChild::Text("rit.".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tempo_log.func, Some(AttTempoLogFunc::Continuous));
    assert_eq!(
        parsed.tempo_log.tstamp2,
        Some(DataMeasurebeat("0m+4".to_string()))
    );
}

#[test]
fn roundtrip_tempo_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Tempo;

    let mut original = Tempo::default();
    original.tempo_log.staff = vec![1];
    original.tempo_log.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tempo_log.staff, vec![1]);
    assert_eq!(parsed.tempo_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn roundtrip_tempo_complete() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::data::{DataBeat, DataDuration, DataDurationCmn, DataTempovalue, DataUri};
    use tusk_model::elements::{Tempo, TempoChild};

    let mut original = Tempo::default();
    original.common.xml_id = Some("tempo-complete".to_string());
    original.tempo_log.staff = vec![1];
    original.tempo_log.layer = vec![1];
    original.tempo_log.tstamp = Some(DataBeat(1.0));
    original.tempo_log.mm = Some(DataTempovalue(120.0));
    original.tempo_log.mm_unit = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.tempo_log.func = Some(AttTempoLogFunc::Instantaneous);
    original.tempo_log.startid = Some(DataUri("#n1".to_string()));
    original
        .children
        .push(TempoChild::Text("Allegro moderato".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.tempo_log.staff, original.tempo_log.staff);
    assert_eq!(parsed.tempo_log.layer, original.tempo_log.layer);
    assert_eq!(parsed.tempo_log.tstamp, original.tempo_log.tstamp);
    assert_eq!(parsed.tempo_log.mm, original.tempo_log.mm);
    assert_eq!(parsed.tempo_log.mm_unit, original.tempo_log.mm_unit);
    assert_eq!(parsed.tempo_log.func, original.tempo_log.func);
    assert_eq!(parsed.tempo_log.startid, original.tempo_log.startid);
    assert_eq!(parsed.children.len(), 1);
}

// ----------------------------------------------------------------------------
// Fermata Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_fermata_empty() {
    use tusk_model::elements::Fermata;

    let original = Fermata::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.fermata_vis.form.is_none());
    assert!(parsed.fermata_vis.shape.is_none());
}

#[test]
fn roundtrip_fermata_with_xml_id() {
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.common.xml_id = Some("ferm-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ferm-1".to_string()));
}

#[test]
fn roundtrip_fermata_with_form_norm() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.form = Some(AttFermataVisForm::Norm);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Norm));
}

#[test]
fn roundtrip_fermata_with_form_inv() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.form = Some(AttFermataVisForm::Inv);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Inv));
}

#[test]
fn roundtrip_fermata_with_shape_curved() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.shape = Some(AttFermataVisShape::Curved);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Curved));
}

#[test]
fn roundtrip_fermata_with_shape_square() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.shape = Some(AttFermataVisShape::Square);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Square));
}

#[test]
fn roundtrip_fermata_with_shape_angular() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.shape = Some(AttFermataVisShape::Angular);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Angular));
}

#[test]
fn roundtrip_fermata_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_log.staff = vec![1];
    original.fermata_log.tstamp = Some(DataBeat(4.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_log.staff, vec![1]);
    assert_eq!(parsed.fermata_log.tstamp, Some(DataBeat(4.0)));
}

#[test]
fn roundtrip_fermata_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_log.startid = Some(DataUri("#note1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.fermata_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn roundtrip_fermata_complete() {
    use tusk_model::att::{AttFermataVisForm, AttFermataVisShape};
    use tusk_model::data::{DataBeat, DataUri};
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.common.xml_id = Some("ferm-complete".to_string());
    original.fermata_log.staff = vec![1];
    original.fermata_log.layer = vec![1];
    original.fermata_log.tstamp = Some(DataBeat(4.0));
    original.fermata_log.startid = Some(DataUri("#n4".to_string()));
    original.fermata_vis.form = Some(AttFermataVisForm::Norm);
    original.fermata_vis.shape = Some(AttFermataVisShape::Curved);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.fermata_log.staff, original.fermata_log.staff);
    assert_eq!(parsed.fermata_log.layer, original.fermata_log.layer);
    assert_eq!(parsed.fermata_log.tstamp, original.fermata_log.tstamp);
    assert_eq!(parsed.fermata_log.startid, original.fermata_log.startid);
    assert_eq!(parsed.fermata_vis.form, original.fermata_vis.form);
    assert_eq!(parsed.fermata_vis.shape, original.fermata_vis.shape);
}

// ----------------------------------------------------------------------------
// Beam Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_beam_empty() {
    use tusk_model::elements::Beam;

    let original = Beam::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Beam::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_beam_with_xml_id() {
    use tusk_model::elements::Beam;

    let mut original = Beam::default();
    original.common.xml_id = Some("beam-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Beam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("beam-1".to_string()));
}

#[test]
fn roundtrip_beam_with_form() {
    use tusk_model::att::AttBeamVisForm;
    use tusk_model::elements::Beam;

    let mut original = Beam::default();
    original.beam_vis.form = Some(AttBeamVisForm::Acc);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Beam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.beam_vis.form, Some(AttBeamVisForm::Acc));
}

#[test]
fn roundtrip_beam_with_note_children() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Beam, BeamChild, Note};

    let mut original = Beam::default();
    original.common.xml_id = Some("b1".to_string());

    // Add beamed eighth notes
    let pitches = [("c", 5), ("d", 5), ("e", 5), ("f", 5)];
    for (i, (pname, oct)) in pitches.iter().enumerate() {
        let mut note = Note::default();
        note.common.xml_id = Some(format!("n{}", i + 1));
        note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
        note.note_log.oct = Some(DataOctave(*oct));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
        original.children.push(BeamChild::Note(Box::new(note)));
    }

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Beam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("b1".to_string()));
    assert_eq!(parsed.children.len(), 4);

    for (i, child) in parsed.children.iter().enumerate() {
        match child {
            BeamChild::Note(n) => {
                assert_eq!(n.common.xml_id, Some(format!("n{}", i + 1)));
                assert_eq!(
                    n.note_log.dur,
                    Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
                );
            }
            other => panic!("Expected Note, got {:?}", other),
        }
    }
}

#[test]
fn roundtrip_beam_with_mixed_children() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Beam, BeamChild, Chord, ChordChild, Note, Rest};

    let mut original = Beam::default();
    original.common.xml_id = Some("b-mixed".to_string());

    // Note
    let mut note = Note::default();
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(5));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
    original.children.push(BeamChild::Note(Box::new(note)));

    // Rest (use DataDurationCmn for CMN rests)
    let mut rest = Rest::default();
    rest.rest_log.dur = Some(tusk_model::data::DataDurationrests::DataDurationCmn(
        DataDurationCmn::N8,
    ));
    original.children.push(BeamChild::Rest(Box::new(rest)));

    // Chord
    let mut chord = Chord::default();
    chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
    let mut chord_note = Note::default();
    chord_note.note_log.pname = Some(DataPitchname::from("e".to_string()));
    chord_note.note_log.oct = Some(DataOctave(5));
    chord.children.push(ChordChild::Note(Box::new(chord_note)));
    original.children.push(BeamChild::Chord(Box::new(chord)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Beam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 3);
    assert!(matches!(parsed.children[0], BeamChild::Note(_)));
    assert!(matches!(parsed.children[1], BeamChild::Rest(_)));
    assert!(matches!(parsed.children[2], BeamChild::Chord(_)));
}

#[test]
fn roundtrip_beam_nested() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Beam, BeamChild, Note};

    let mut outer = Beam::default();
    outer.common.xml_id = Some("b-outer".to_string());

    // Inner beam
    let mut inner = Beam::default();
    inner.common.xml_id = Some("b-inner".to_string());

    let mut note = Note::default();
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(5));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N16));
    inner.children.push(BeamChild::Note(Box::new(note)));

    let mut note2 = Note::default();
    note2.note_log.pname = Some(DataPitchname::from("d".to_string()));
    note2.note_log.oct = Some(DataOctave(5));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N16));
    inner.children.push(BeamChild::Note(Box::new(note2)));

    outer.children.push(BeamChild::Beam(Box::new(inner)));

    let xml = outer.to_mei_string().expect("serialize");
    let parsed = Beam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("b-outer".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        BeamChild::Beam(inner) => {
            assert_eq!(inner.common.xml_id, Some("b-inner".to_string()));
            assert_eq!(inner.children.len(), 2);
        }
        other => panic!("Expected Beam, got {:?}", other),
    }
}

#[test]
fn roundtrip_beam_complete() {
    use tusk_model::att::AttBeamVisForm;
    use tusk_model::data::{DataColor, DataColornames, DataDuration, DataDurationCmn};
    use tusk_model::elements::{Beam, BeamChild, Note};

    let mut original = Beam::default();
    original.common.xml_id = Some("beam-complete".to_string());
    original.beam_vis.form = Some(AttBeamVisForm::Mixed);
    original.beam_vis.color = Some(DataColor::DataColornames(DataColornames::Black));

    // Add notes
    for i in 0..4 {
        let mut note = Note::default();
        note.common.xml_id = Some(format!("n{}", i + 1));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N16));
        original.children.push(BeamChild::Note(Box::new(note)));
    }

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Beam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.beam_vis.form, original.beam_vis.form);
    assert_eq!(parsed.beam_vis.color, original.beam_vis.color);
    assert_eq!(parsed.children.len(), 4);
}

// ----------------------------------------------------------------------------
// Tuplet Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_tuplet_empty() {
    use tusk_model::elements::Tuplet;

    let original = Tuplet::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.tuplet_log.num.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_tuplet_with_xml_id() {
    use tusk_model::elements::Tuplet;

    let mut original = Tuplet::default();
    original.common.xml_id = Some("tuplet-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tuplet-1".to_string()));
}

#[test]
fn roundtrip_tuplet_with_num() {
    use tusk_model::elements::Tuplet;

    let mut original = Tuplet::default();
    original.tuplet_log.num = Some(3);
    original.tuplet_log.numbase = Some(2);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tuplet_log.num, Some(3));
    assert_eq!(parsed.tuplet_log.numbase, Some(2));
}

#[test]
fn roundtrip_tuplet_with_dur() {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_model::elements::Tuplet;

    let mut original = Tuplet::default();
    original
        .tuplet_log
        .dur
        .push(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.tuplet_log.dur,
        vec![DataDuration::DataDurationCmn(DataDurationCmn::N4)]
    );
}

#[test]
fn roundtrip_tuplet_with_bracket_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Tuplet;

    let mut original = Tuplet::default();
    original.tuplet_vis.bracket_visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tuplet_vis.bracket_visible, Some(DataBoolean::True));
}

#[test]
fn roundtrip_tuplet_with_num_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Tuplet;

    let mut original = Tuplet::default();
    original.tuplet_vis.num_visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tuplet_vis.num_visible, Some(DataBoolean::True));
}

#[test]
fn roundtrip_tuplet_triplet_with_notes() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Note, Tuplet, TupletChild};

    let mut original = Tuplet::default();
    original.common.xml_id = Some("t1".to_string());
    original.tuplet_log.num = Some(3);
    original.tuplet_log.numbase = Some(2);

    // Add three eighth notes for a triplet
    let pitches = ["c", "d", "e"];
    for (i, pname) in pitches.iter().enumerate() {
        let mut note = Note::default();
        note.common.xml_id = Some(format!("n{}", i + 1));
        note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
        note.note_log.oct = Some(DataOctave(4));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
        original.children.push(TupletChild::Note(Box::new(note)));
    }

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
    assert_eq!(parsed.tuplet_log.num, Some(3));
    assert_eq!(parsed.tuplet_log.numbase, Some(2));
    assert_eq!(parsed.children.len(), 3);
}

#[test]
fn roundtrip_tuplet_with_beam_child() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Beam, BeamChild, Note, Tuplet, TupletChild};

    let mut original = Tuplet::default();
    original.common.xml_id = Some("t-beam".to_string());
    original.tuplet_log.num = Some(3);
    original.tuplet_log.numbase = Some(2);

    // Beamed triplet
    let mut beam = Beam::default();
    beam.common.xml_id = Some("b1".to_string());

    for pname in ["c", "d", "e"] {
        let mut note = Note::default();
        note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
        note.note_log.oct = Some(DataOctave(4));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
        beam.children.push(BeamChild::Note(Box::new(note)));
    }

    original.children.push(TupletChild::Beam(Box::new(beam)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        TupletChild::Beam(b) => {
            assert_eq!(b.common.xml_id, Some("b1".to_string()));
            assert_eq!(b.children.len(), 3);
        }
        other => panic!("Expected Beam, got {:?}", other),
    }
}

#[test]
fn roundtrip_tuplet_complete() {
    use tusk_model::data::{DataBoolean, DataDuration, DataDurationCmn};
    use tusk_model::elements::{Note, Tuplet, TupletChild};

    let mut original = Tuplet::default();
    original.common.xml_id = Some("tuplet-complete".to_string());
    original.tuplet_log.num = Some(5);
    original.tuplet_log.numbase = Some(4);
    original
        .tuplet_log
        .dur
        .push(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.tuplet_log.staff = vec![1];
    original.tuplet_log.layer = vec![1];
    original.tuplet_vis.bracket_visible = Some(DataBoolean::True);
    original.tuplet_vis.num_visible = Some(DataBoolean::True);

    // Add notes
    for i in 0..5 {
        let mut note = Note::default();
        note.common.xml_id = Some(format!("n{}", i + 1));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N16));
        original.children.push(TupletChild::Note(Box::new(note)));
    }

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tuplet::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.tuplet_log.num, original.tuplet_log.num);
    assert_eq!(parsed.tuplet_log.numbase, original.tuplet_log.numbase);
    assert_eq!(parsed.tuplet_log.dur, original.tuplet_log.dur);
    assert_eq!(parsed.tuplet_log.staff, original.tuplet_log.staff);
    assert_eq!(parsed.tuplet_log.layer, original.tuplet_log.layer);
    assert_eq!(
        parsed.tuplet_vis.bracket_visible,
        original.tuplet_vis.bracket_visible
    );
    assert_eq!(
        parsed.tuplet_vis.num_visible,
        original.tuplet_vis.num_visible
    );
    assert_eq!(parsed.children.len(), 5);
}

// ----------------------------------------------------------------------------
// GraceGrp Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn roundtrip_gracegrp_empty() {
    use tusk_model::elements::GraceGrp;

    let original = GraceGrp::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.grace_grp_log.grace.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_gracegrp_with_xml_id() {
    use tusk_model::elements::GraceGrp;

    let mut original = GraceGrp::default();
    original.common.xml_id = Some("gg1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("gg1".to_string()));
}

#[test]
fn roundtrip_gracegrp_with_grace_unknown() {
    use tusk_model::data::DataGrace;
    use tusk_model::elements::GraceGrp;

    let mut original = GraceGrp::default();
    original.grace_grp_log.grace = Some(DataGrace::Unknown);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Unknown));
}

#[test]
fn roundtrip_gracegrp_with_grace_unacc() {
    use tusk_model::data::DataGrace;
    use tusk_model::elements::GraceGrp;

    let mut original = GraceGrp::default();
    original.grace_grp_log.grace = Some(DataGrace::Unacc);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Unacc));
}

#[test]
fn roundtrip_gracegrp_with_grace_acc() {
    use tusk_model::data::DataGrace;
    use tusk_model::elements::GraceGrp;

    let mut original = GraceGrp::default();
    original.grace_grp_log.grace = Some(DataGrace::Acc);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Acc));
}

#[test]
fn roundtrip_gracegrp_with_attach_pre() {
    use tusk_model::att::AttGraceGrpLogAttach;
    use tusk_model::elements::GraceGrp;

    let mut original = GraceGrp::default();
    original.grace_grp_log.attach = Some(AttGraceGrpLogAttach::Pre);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.grace_grp_log.attach, Some(AttGraceGrpLogAttach::Pre));
}

#[test]
fn roundtrip_gracegrp_with_attach_post() {
    use tusk_model::att::AttGraceGrpLogAttach;
    use tusk_model::elements::GraceGrp;

    let mut original = GraceGrp::default();
    original.grace_grp_log.attach = Some(AttGraceGrpLogAttach::Post);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.grace_grp_log.attach,
        Some(AttGraceGrpLogAttach::Post)
    );
}

#[test]
fn roundtrip_gracegrp_with_note_children() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataGrace, DataOctave, DataPitchname};
    use tusk_model::elements::{GraceGrp, GraceGrpChild, Note};

    let mut original = GraceGrp::default();
    original.common.xml_id = Some("gg1".to_string());
    original.grace_grp_log.grace = Some(DataGrace::Unacc);

    // Grace notes
    let pitches = [("d", 5), ("c", 5)];
    for (i, (pname, oct)) in pitches.iter().enumerate() {
        let mut note = Note::default();
        note.common.xml_id = Some(format!("gn{}", i + 1));
        note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
        note.note_log.oct = Some(DataOctave(*oct));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N16));
        original.children.push(GraceGrpChild::Note(Box::new(note)));
    }

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("gg1".to_string()));
    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Unacc));
    assert_eq!(parsed.children.len(), 2);
}

#[test]
fn roundtrip_gracegrp_with_beam_child() {
    use tusk_model::att::AttGraceGrpLogAttach;
    use tusk_model::data::{DataDuration, DataDurationCmn, DataGrace, DataOctave, DataPitchname};
    use tusk_model::elements::{Beam, BeamChild, GraceGrp, GraceGrpChild, Note};

    let mut original = GraceGrp::default();
    original.common.xml_id = Some("gg-beam".to_string());
    original.grace_grp_log.grace = Some(DataGrace::Acc);
    original.grace_grp_log.attach = Some(AttGraceGrpLogAttach::Pre);

    // Beamed grace notes
    let mut beam = Beam::default();
    for pname in ["f", "e", "d", "c"] {
        let mut note = Note::default();
        note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
        note.note_log.oct = Some(DataOctave(5));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N32));
        beam.children.push(BeamChild::Note(Box::new(note)));
    }

    original.children.push(GraceGrpChild::Beam(Box::new(beam)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.grace_grp_log.grace, Some(DataGrace::Acc));
    assert_eq!(parsed.grace_grp_log.attach, Some(AttGraceGrpLogAttach::Pre));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        GraceGrpChild::Beam(b) => {
            assert_eq!(b.children.len(), 4);
        }
        other => panic!("Expected Beam, got {:?}", other),
    }
}

#[test]
fn roundtrip_gracegrp_with_chord_child() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataGrace, DataOctave, DataPitchname};
    use tusk_model::elements::{Chord, ChordChild, GraceGrp, GraceGrpChild, Note};

    let mut original = GraceGrp::default();
    original.common.xml_id = Some("gg-chord".to_string());
    original.grace_grp_log.grace = Some(DataGrace::Unacc);

    // Grace chord
    let mut chord = Chord::default();
    chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    for (pname, oct) in [("c", 5), ("e", 5)] {
        let mut note = Note::default();
        note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
        note.note_log.oct = Some(DataOctave(oct));
        chord.children.push(ChordChild::Note(Box::new(note)));
    }

    original
        .children
        .push(GraceGrpChild::Chord(Box::new(chord)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GraceGrpChild::Chord(c) => {
            assert_eq!(c.children.len(), 2);
        }
        other => panic!("Expected Chord, got {:?}", other),
    }
}

#[test]
fn roundtrip_gracegrp_complete() {
    use tusk_model::att::AttGraceGrpLogAttach;
    use tusk_model::data::{DataDuration, DataDurationCmn, DataGrace, DataOctave, DataPitchname};
    use tusk_model::elements::{GraceGrp, GraceGrpChild, Note};

    let mut original = GraceGrp::default();
    original.common.xml_id = Some("gracegrp-complete".to_string());
    original.grace_grp_log.grace = Some(DataGrace::Acc);
    original.grace_grp_log.attach = Some(AttGraceGrpLogAttach::Pre);

    // Add grace notes
    for (i, pname) in ["e", "d", "c"].iter().enumerate() {
        let mut note = Note::default();
        note.common.xml_id = Some(format!("gn{}", i + 1));
        note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
        note.note_log.oct = Some(DataOctave(5));
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N16));
        original.children.push(GraceGrpChild::Note(Box::new(note)));
    }

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GraceGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.grace_grp_log.grace, original.grace_grp_log.grace);
    assert_eq!(parsed.grace_grp_log.attach, original.grace_grp_log.attach);
    assert_eq!(parsed.children.len(), 3);
}

// ----------------------------------------------------------------------------
// ExpressionList Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn deserialize_expressionlist_empty() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::ExpressionList;

    let xml = r#"<expressionList/>"#;
    let expr_list = ExpressionList::from_mei_str(xml).expect("should deserialize");

    assert!(expr_list.common.xml_id.is_none());
    assert!(expr_list.children.is_empty());
}

#[test]
fn deserialize_expressionlist_with_xml_id() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::ExpressionList;

    let xml = r#"<expressionList xml:id="exprl1"/>"#;
    let expr_list = ExpressionList::from_mei_str(xml).expect("should deserialize");

    assert_eq!(expr_list.common.xml_id, Some("exprl1".to_string()));
    assert!(expr_list.children.is_empty());
}

#[test]
fn deserialize_expressionlist_with_head() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{ExpressionList, ExpressionListChild};

    let xml = r#"<expressionList>
        <head>Expression List Title</head>
    </expressionList>"#;
    let expr_list = ExpressionList::from_mei_str(xml).expect("should deserialize");

    assert_eq!(expr_list.children.len(), 1);
    match &expr_list.children[0] {
        ExpressionListChild::Head(head) => {
            // Head contains text as HeadChild::Text
            assert!(!head.children.is_empty());
        }
        other => panic!("Expected Head, got {:?}", other),
    }
}

#[test]
fn deserialize_expressionlist_with_expression() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{ExpressionList, ExpressionListChild};

    let xml = r#"<expressionList xml:id="exprl1">
        <expression xml:id="expr1">
            <title>First Edition</title>
        </expression>
    </expressionList>"#;
    let expr_list = ExpressionList::from_mei_str(xml).expect("should deserialize");

    assert_eq!(expr_list.common.xml_id, Some("exprl1".to_string()));
    assert_eq!(expr_list.children.len(), 1);
    match &expr_list.children[0] {
        ExpressionListChild::Expression(expr) => {
            assert_eq!(expr.common.xml_id, Some("expr1".to_string()));
            assert_eq!(expr.children.len(), 1);
        }
        other => panic!("Expected Expression, got {:?}", other),
    }
}

#[test]
fn deserialize_expressionlist_with_multiple_expressions() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{ExpressionList, ExpressionListChild};

    let xml = r#"<expressionList xml:id="exprl1">
        <head>Expressions</head>
        <expression xml:id="expr1">
            <title>First Edition</title>
        </expression>
        <expression xml:id="expr2">
            <title>Second Edition</title>
        </expression>
    </expressionList>"#;
    let expr_list = ExpressionList::from_mei_str(xml).expect("should deserialize");

    assert_eq!(expr_list.children.len(), 3);

    // First child should be head
    match &expr_list.children[0] {
        ExpressionListChild::Head(_) => {}
        other => panic!("Expected Head as first child, got {:?}", other),
    }

    // Second child should be expression
    match &expr_list.children[1] {
        ExpressionListChild::Expression(expr) => {
            assert_eq!(expr.common.xml_id, Some("expr1".to_string()));
        }
        other => panic!("Expected Expression as second child, got {:?}", other),
    }

    // Third child should be expression
    match &expr_list.children[2] {
        ExpressionListChild::Expression(expr) => {
            assert_eq!(expr.common.xml_id, Some("expr2".to_string()));
        }
        other => panic!("Expected Expression as third child, got {:?}", other),
    }
}

// ----------------------------------------------------------------------------
// Expression Round-Trip Tests
// ----------------------------------------------------------------------------

#[test]
fn deserialize_expression_empty() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::Expression;

    let xml = r#"<expression/>"#;
    let expr = Expression::from_mei_str(xml).expect("should deserialize");

    assert!(expr.common.xml_id.is_none());
    assert!(expr.children.is_empty());
}

#[test]
fn deserialize_expression_with_xml_id() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::Expression;

    let xml = r#"<expression xml:id="expr1"/>"#;
    let expr = Expression::from_mei_str(xml).expect("should deserialize");

    assert_eq!(expr.common.xml_id, Some("expr1".to_string()));
}

#[test]
fn deserialize_expression_with_authorized_attrs() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::data::DataUri;
    use tusk_model::elements::Expression;

    let xml = r#"<expression xml:id="expr1" auth="RISM" auth.uri="https://rism.online/"/>"#;
    let expr = Expression::from_mei_str(xml).expect("should deserialize");

    assert_eq!(expr.common.xml_id, Some("expr1".to_string()));
    assert_eq!(expr.authorized.auth, Some("RISM".to_string()));
    assert_eq!(
        expr.authorized.auth_uri,
        Some(DataUri("https://rism.online/".to_string()))
    );
}

#[test]
fn deserialize_expression_with_title() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{Expression, ExpressionChild};

    let xml = r#"<expression xml:id="expr1">
        <title>First Edition</title>
    </expression>"#;
    let expr = Expression::from_mei_str(xml).expect("should deserialize");

    assert_eq!(expr.children.len(), 1);
    match &expr.children[0] {
        ExpressionChild::Title(title) => {
            // Title contains text as TitleChild::Text
            assert!(!title.children.is_empty());
        }
        other => panic!("Expected Title, got {:?}", other),
    }
}

#[test]
fn deserialize_expression_with_multiple_children() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{Expression, ExpressionChild};

    let xml = r#"<expression xml:id="expr1">
        <head>Expression Header</head>
        <identifier>12345</identifier>
        <title>Piano Sonata Op. 13 - First Edition</title>
        <respStmt>
            <persName role="editor">John Editor</persName>
        </respStmt>
        <creation>
            <date>1801</date>
        </creation>
        <langUsage>
            <language xml:id="en">English</language>
        </langUsage>
        <perfMedium>
            <perfResList>
                <perfRes>Piano</perfRes>
            </perfResList>
        </perfMedium>
        <notesStmt>
            <annot>First published edition</annot>
        </notesStmt>
    </expression>"#;
    let expr = Expression::from_mei_str(xml).expect("should deserialize");

    assert_eq!(expr.common.xml_id, Some("expr1".to_string()));
    // Should have multiple children
    assert!(expr.children.len() >= 5);

    // Check various child types are present
    let mut has_head = false;
    let mut has_identifier = false;
    let mut has_title = false;
    let mut has_resp_stmt = false;
    let mut has_creation = false;

    for child in &expr.children {
        match child {
            ExpressionChild::Head(_) => has_head = true,
            ExpressionChild::Identifier(_) => has_identifier = true,
            ExpressionChild::Title(_) => has_title = true,
            ExpressionChild::RespStmt(_) => has_resp_stmt = true,
            ExpressionChild::Creation(_) => has_creation = true,
            _ => {}
        }
    }

    assert!(has_head, "Should have head child");
    assert!(has_identifier, "Should have identifier child");
    assert!(has_title, "Should have title child");
    assert!(has_resp_stmt, "Should have respStmt child");
    assert!(has_creation, "Should have creation child");
}

#[test]
fn deserialize_expression_with_extent_and_score_format() {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{Expression, ExpressionChild};

    let xml = r#"<expression xml:id="expr1">
        <title>Symphony No. 5</title>
        <extent unit="pages">120</extent>
        <scoreFormat>score</scoreFormat>
    </expression>"#;
    let expr = Expression::from_mei_str(xml).expect("should deserialize");

    let mut has_extent = false;
    let mut has_score_format = false;

    for child in &expr.children {
        match child {
            ExpressionChild::Extent(_) => has_extent = true,
            ExpressionChild::ScoreFormat(_) => has_score_format = true,
            _ => {}
        }
    }

    assert!(has_extent, "Should have extent child");
    assert!(has_score_format, "Should have scoreFormat child");
}

// ============================================================================
// Header Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_mei_head() {
    use tusk_model::elements::MeiHead;

    let original = MeiHead::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MeiHead::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_mei_head_with_xml_id() {
    use tusk_model::elements::MeiHead;

    let mut original = MeiHead::default();
    original.basic.xml_id = Some("header-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"header-1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = MeiHead::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.basic.xml_id, Some("header-1".to_string()));
}

#[test]
fn roundtrip_revision_desc_with_change() {
    use tusk_model::data::DataIsodate;
    use tusk_model::elements::{
        Change, ChangeChild, ChangeDesc, ChangeDescChild, P, PChild, RevisionDesc,
        RevisionDescChild,
    };

    let mut revision_desc = RevisionDesc::default();
    revision_desc.common.xml_id = Some("revdesc1".to_string());

    // Create a change element
    let mut change = Change::default();
    change.common.xml_id = Some("change1".to_string());
    change.datable.isodate = Some(DataIsodate("2025-01-15".to_string()));

    // Add changeDesc with a paragraph
    let mut change_desc = ChangeDesc::default();
    let mut p = P::default();
    p.children
        .push(PChild::Text("Initial encoding".to_string()));
    change_desc.children.push(ChangeDescChild::P(Box::new(p)));
    change
        .children
        .push(ChangeChild::ChangeDesc(Box::new(change_desc)));

    revision_desc
        .children
        .push(RevisionDescChild::Change(Box::new(change)));

    // Serialize and deserialize
    let xml = revision_desc.to_mei_string().expect("serialize");
    assert!(
        xml.contains("revisionDesc"),
        "should have revisionDesc: {}",
        xml
    );
    assert!(xml.contains("change"), "should have change: {}", xml);
    assert!(
        xml.contains("isodate=\"2025-01-15\""),
        "should have isodate: {}",
        xml
    );
    assert!(
        xml.contains("changeDesc"),
        "should have changeDesc: {}",
        xml
    );
    assert!(
        xml.contains("Initial encoding"),
        "should have text: {}",
        xml
    );

    let parsed = RevisionDesc::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("revdesc1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    if let RevisionDescChild::Change(change) = &parsed.children[0] {
        assert_eq!(change.common.xml_id, Some("change1".to_string()));
        assert_eq!(
            change.datable.isodate,
            Some(DataIsodate("2025-01-15".to_string()))
        );
    } else {
        panic!("Expected Change child");
    }
}

#[test]
fn serialize_title_with_text_content() {
    use tusk_model::elements::{Title, TitleChild};

    let mut title = Title::default();
    title.basic.xml_id = Some("title1".to_string());
    title
        .children
        .push(TitleChild::Text("Symphony No. 5".to_string()));

    let xml = title.to_mei_string().expect("serialize");
    assert!(xml.contains("<title"), "should have title: {}", xml);
    assert!(
        xml.contains("xml:id=\"title1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Symphony No. 5"), "should have text: {}", xml);
    assert!(xml.contains("</title>"), "should have closing tag: {}", xml);
}

#[test]
fn serialize_file_desc_structure() {
    use tusk_model::elements::{
        FileDesc, FileDescChild, PubStmt, Title, TitleChild, TitleStmt, TitleStmtChild,
    };

    let mut file_desc = FileDesc::default();
    file_desc.common.xml_id = Some("fd1".to_string());

    // Add titleStmt with title
    let mut title_stmt = TitleStmt::default();
    let mut title = Title::default();
    title
        .children
        .push(TitleChild::Text("Test Work".to_string()));
    title_stmt
        .children
        .push(TitleStmtChild::Title(Box::new(title)));
    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));

    // Add pubStmt
    let pub_stmt = PubStmt::default();
    file_desc
        .children
        .push(FileDescChild::PubStmt(Box::new(pub_stmt)));

    // Serialize
    let xml = file_desc.to_mei_string().expect("serialize");
    assert!(xml.contains("<fileDesc"), "should have fileDesc: {}", xml);
    assert!(
        xml.contains("xml:id=\"fd1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("<titleStmt"), "should have titleStmt: {}", xml);
    assert!(xml.contains("<title"), "should have title: {}", xml);
    assert!(xml.contains("Test Work"), "should have title text: {}", xml);
    assert!(xml.contains("<pubStmt"), "should have pubStmt: {}", xml);
    assert!(
        xml.contains("</fileDesc>"),
        "should have closing tag: {}",
        xml
    );
}

#[test]
fn serialize_date_with_isodate() {
    use tusk_model::data::DataIsodate;
    use tusk_model::elements::{Date, DateChild};

    let mut date = Date::default();
    date.common.xml_id = Some("date1".to_string());
    date.datable.isodate = Some(DataIsodate("2025-06-15".to_string()));
    date.children
        .push(DateChild::Text("June 15, 2025".to_string()));

    let xml = date.to_mei_string().expect("serialize");
    assert!(xml.contains("<date"), "should have date: {}", xml);
    assert!(
        xml.contains("isodate=\"2025-06-15\""),
        "should have isodate: {}",
        xml
    );
    assert!(xml.contains("June 15, 2025"), "should have text: {}", xml);
    assert!(xml.contains("</date>"), "should have closing tag: {}", xml);
}

#[test]
fn serialize_p_with_text_content() {
    use tusk_model::elements::{P, PChild};

    let mut p = P::default();
    p.common.xml_id = Some("p1".to_string());
    p.children
        .push(PChild::Text("This is a paragraph.".to_string()));

    let xml = p.to_mei_string().expect("serialize");
    assert!(xml.contains("<p"), "should have p: {}", xml);
    assert!(xml.contains("xml:id=\"p1\""), "should have xml:id: {}", xml);
    assert!(
        xml.contains("This is a paragraph."),
        "should have text: {}",
        xml
    );
    assert!(xml.contains("</p>"), "should have closing tag: {}", xml);
}

#[test]
fn serialize_head_with_text() {
    use tusk_model::elements::{Head, HeadChild};

    let mut head = Head::default();
    head.common.xml_id = Some("head1".to_string());
    head.children
        .push(HeadChild::Text("Section Title".to_string()));

    let xml = head.to_mei_string().expect("serialize");
    assert!(xml.contains("<head"), "should have head: {}", xml);
    assert!(
        xml.contains("xml:id=\"head1\""),
        "should have xml:id: {}",
        xml
    );
    assert!(xml.contains("Section Title"), "should have text: {}", xml);
    assert!(xml.contains("</head>"), "should have closing tag: {}", xml);
}

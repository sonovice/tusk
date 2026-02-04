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

mod control;
mod defs;
mod header;
mod note;
mod structure;

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

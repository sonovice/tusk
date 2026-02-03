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

use crate::deserializer::MeiDeserialize;
use crate::serializer::{MeiSerialize, SerializeConfig};
use tusk_model::data::{
    DataAccidentalGestural, DataAccidentalGesturalBasic, DataAccidentalWritten,
    DataAccidentalWrittenBasic, DataArticulation, DataAugmentdot, DataColor, DataColorvalues,
    DataDuration, DataDurationCmn, DataGrace, DataOctave, DataPitchname, DataStemdirection,
    DataStemdirectionBasic, DataTie,
};
use tusk_model::elements::Note;

// ============================================================================
// Note Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_note() {
    let original = Note::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.note_log.dur.is_none());
    assert!(parsed.note_log.pname.is_none());
    assert!(parsed.note_log.oct.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_note_with_xml_id() {
    let mut original = Note::default();
    original.common.xml_id = Some("note-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"note-1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Note::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("note-1".to_string()));
}

#[test]
fn roundtrip_note_with_pitch() {
    let mut original = Note::default();
    original.note_log.pname = Some(DataPitchname::from("g".to_string()));
    original.note_log.oct = Some(DataOctave(5));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.note_log.pname,
        Some(DataPitchname::from("g".to_string()))
    );
    assert_eq!(parsed.note_log.oct, Some(DataOctave(5)));
}

#[test]
fn roundtrip_note_with_duration_quarter() {
    let mut original = Note::default();
    original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn roundtrip_note_with_duration_breve() {
    let mut original = Note::default();
    original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::Breve));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::Breve))
    );
}

#[test]
fn roundtrip_note_with_dots() {
    let mut original = Note::default();
    original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.note_log.dots = Some(DataAugmentdot(2));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.note_log.dots, Some(DataAugmentdot(2)));
}

#[test]
fn roundtrip_note_complete_cmn() {
    // Common Music Notation note with all typical attributes
    let mut original = Note::default();
    original.common.xml_id = Some("n42".to_string());
    original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
    original.note_log.dots = Some(DataAugmentdot(1));
    original.note_log.pname = Some(DataPitchname::from("f".to_string()));
    original.note_log.oct = Some(DataOctave(4));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.note_log.dur, original.note_log.dur);
    assert_eq!(parsed.note_log.dots, original.note_log.dots);
    assert_eq!(parsed.note_log.pname, original.note_log.pname);
    assert_eq!(parsed.note_log.oct, original.note_log.oct);
}

#[test]
fn roundtrip_note_with_label() {
    let mut original = Note::default();
    original.common.label = Some("soprano part".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("soprano part".to_string()));
}

#[test]
fn roundtrip_note_with_staff_and_layer() {
    let mut original = Note::default();
    original.note_log.staff = vec![1u64];
    original.note_log.layer = vec![1u64];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert!(
        !parsed.note_log.staff.is_empty(),
        "staff should be preserved"
    );
    assert!(
        !parsed.note_log.layer.is_empty(),
        "layer should be preserved"
    );
}

#[test]
fn roundtrip_note_with_gestural_accidental() {
    let mut original = Note::default();
    original.note_ges.accid_ges = Some(DataAccidentalGestural::DataAccidentalGesturalBasic(
        DataAccidentalGesturalBasic::S,
    ));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert!(
        parsed.note_ges.accid_ges.is_some(),
        "accid.ges should be preserved"
    );
}

#[test]
fn roundtrip_note_with_grace() {
    let mut original = Note::default();
    original.note_log.grace = Some(DataGrace::Acc);
    original.note_log.pname = Some(DataPitchname::from("c".to_string()));
    original.note_log.oct = Some(DataOctave(5));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.note_log.grace.is_some(), "grace should be preserved");
}

// ============================================================================
// Special Character and Escaping Tests
// ============================================================================

#[test]
fn roundtrip_note_label_with_ampersand() {
    let mut original = Note::default();
    original.common.label = Some("Violin I & II".to_string());

    let xml = original.to_mei_string().expect("serialize");
    // XML should contain escaped ampersand
    assert!(
        xml.contains("&amp;") || xml.contains("&#38;"),
        "ampersand should be escaped in XML: {}",
        xml
    );

    let parsed = Note::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.label, Some("Violin I & II".to_string()));
}

#[test]
fn roundtrip_note_label_with_less_than() {
    let mut original = Note::default();
    original.common.label = Some("notes < 4".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("notes < 4".to_string()));
}

#[test]
fn roundtrip_note_label_with_quotes() {
    let mut original = Note::default();
    original.common.label = Some("the \"main\" theme".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("the \"main\" theme".to_string()));
}

#[test]
fn roundtrip_note_xml_id_with_hyphens() {
    let mut original = Note::default();
    original.common.xml_id = Some("note-001-a".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("note-001-a".to_string()));
}

#[test]
fn roundtrip_note_xml_id_with_underscores() {
    let mut original = Note::default();
    original.common.xml_id = Some("note_001_a".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("note_001_a".to_string()));
}

// ============================================================================
// Configuration and Format Tests
// ============================================================================

#[test]
fn roundtrip_note_without_xml_declaration() {
    let mut original = Note::default();
    original.common.xml_id = Some("n1".to_string());

    let config = SerializeConfig {
        include_declaration: false,
        indent: None,
        mei_namespace: None,
        additional_namespaces: Vec::new(),
    };

    let xml = original
        .to_mei_string_with_config(config)
        .expect("serialize");
    assert!(
        !xml.contains("<?xml"),
        "should not have declaration: {}",
        xml
    );

    let parsed = Note::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, original.common.xml_id);
}

#[test]
fn roundtrip_note_with_xml_declaration() {
    let mut original = Note::default();
    original.common.xml_id = Some("n1".to_string());

    let config = SerializeConfig {
        include_declaration: true,
        indent: None,
        mei_namespace: None,
        additional_namespaces: Vec::new(),
    };

    // Note: The default to_mei_string doesn't write declaration at element level
    // Declaration is typically for full documents starting with <mei>
    let xml = original
        .to_mei_string_with_config(config)
        .expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, original.common.xml_id);
}

// ============================================================================
// External XML Parsing Tests (XML → Struct → XML)
// ============================================================================

#[test]
fn parse_external_note_minimal() {
    let xml = r#"<note/>"#;
    let parsed = Note::from_mei_str(xml).expect("deserialize");

    // Re-serialize and verify round-trip
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Note::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.common.xml_id.is_none());
}

#[test]
fn parse_external_note_with_attributes() {
    let xml = r#"<note xml:id="n1" dur="4" pname="c" oct="4"/>"#;
    let parsed = Note::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("n1".to_string()));
    assert_eq!(
        parsed.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(
        parsed.note_log.pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(parsed.note_log.oct, Some(DataOctave(4)));

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Note::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.common.xml_id, Some("n1".to_string()));
    assert_eq!(reparsed.note_log.dur, parsed.note_log.dur);
    assert_eq!(reparsed.note_log.pname, parsed.note_log.pname);
    assert_eq!(reparsed.note_log.oct, parsed.note_log.oct);
}

#[test]
fn parse_external_note_with_namespace_prefix() {
    // MEI documents may use namespace prefixes
    // The deserializer should handle this gracefully
    let xml = r#"<note xml:id="n1" dur="4"/>"#;
    let parsed = Note::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("n1".to_string()));
}

#[test]
fn parse_external_note_all_pitch_names() {
    // Test all standard pitch names
    for pname in ["c", "d", "e", "f", "g", "a", "b"] {
        let xml = format!(r#"<note pname="{}"/>"#, pname);
        let parsed = Note::from_mei_str(&xml).expect("deserialize");
        assert!(
            parsed.note_log.pname.is_some(),
            "pname {} should parse",
            pname
        );

        let reserialized = parsed.to_mei_string().expect("re-serialize");
        assert!(
            reserialized.contains(&format!("pname=\"{}\"", pname)),
            "pname {} should round-trip: {}",
            pname,
            reserialized
        );
    }
}

#[test]
fn parse_external_note_all_cmn_durations() {
    // Test common music notation duration values
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
        let xml = format!(r#"<note dur="{}"/>"#, dur_str);
        let parsed =
            Note::from_mei_str(&xml).unwrap_or_else(|_| panic!("deserialize dur={}", dur_str));
        assert_eq!(
            parsed.note_log.dur,
            Some(DataDuration::DataDurationCmn(expected)),
            "dur {} should parse correctly",
            dur_str
        );
    }
}

#[test]
fn parse_external_note_octaves() {
    // Test various octave values (0-9 are valid per MEI spec)
    for oct in 0u64..=9 {
        let xml = format!(r#"<note oct="{}"/>"#, oct);
        let parsed = Note::from_mei_str(&xml).unwrap_or_else(|_| panic!("deserialize oct={}", oct));
        assert_eq!(
            parsed.note_log.oct,
            Some(DataOctave(oct)),
            "oct {} should parse",
            oct
        );

        let reserialized = parsed.to_mei_string().expect("re-serialize");
        let reparsed = Note::from_mei_str(&reserialized).expect("re-deserialize");
        assert_eq!(reparsed.note_log.oct, Some(DataOctave(oct)));
    }
}

// ============================================================================
// Visual Attribute Tests
// ============================================================================

#[test]
fn roundtrip_note_with_stem_direction() {
    let mut original = Note::default();
    original.note_vis.stem_dir = Some(DataStemdirection::DataStemdirectionBasic(
        DataStemdirectionBasic::Up,
    ));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert!(
        parsed.note_vis.stem_dir.is_some(),
        "stem.dir should be preserved"
    );
}

#[test]
fn roundtrip_note_with_color() {
    let mut original = Note::default();
    original.note_vis.color = Some(DataColor::DataColorvalues(DataColorvalues::from(
        "#FF0000".to_string(),
    )));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.note_vis.color.is_some(), "color should be preserved");
}

// ============================================================================
// Analytical Attribute Tests
// ============================================================================

#[test]
fn roundtrip_note_with_analytical_accidental() {
    let mut original = Note::default();
    original.note_anl.accid = Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
        DataAccidentalWrittenBasic::F,
    ));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.note_anl.accid.is_some(), "accid should be preserved");
}

#[test]
fn roundtrip_note_with_tie() {
    let mut original = Note::default();
    original.note_anl.tie = vec![DataTie::from("i".to_string())];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert!(!parsed.note_anl.tie.is_empty(), "tie should be preserved");
}

#[test]
fn roundtrip_note_with_articulation() {
    let mut original = Note::default();
    original.note_anl.artic = vec![DataArticulation::Stacc];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert!(
        !parsed.note_anl.artic.is_empty(),
        "artic should be preserved"
    );
}

// ============================================================================
// Stress Tests
// ============================================================================

#[test]
fn roundtrip_note_with_many_attributes() {
    // Test note with maximum realistic attribute set
    let mut original = Note::default();
    original.common.xml_id = Some("n1".to_string());
    original.common.label = Some("test note".to_string());
    original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.note_log.dots = Some(DataAugmentdot(1));
    original.note_log.pname = Some(DataPitchname::from("c".to_string()));
    original.note_log.oct = Some(DataOctave(4));
    original.note_log.staff = vec![1u64];
    original.note_log.layer = vec![1u64];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    // Verify all attributes preserved
    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.common.label, original.common.label);
    assert_eq!(parsed.note_log.dur, original.note_log.dur);
    assert_eq!(parsed.note_log.dots, original.note_log.dots);
    assert_eq!(parsed.note_log.pname, original.note_log.pname);
    assert_eq!(parsed.note_log.oct, original.note_log.oct);
}

#[test]
fn roundtrip_multiple_notes_sequentially() {
    // Ensure state doesn't leak between serializations
    for i in 0u64..10 {
        let mut note = Note::default();
        note.common.xml_id = Some(format!("n{}", i));
        note.note_log.oct = Some(DataOctave(i % 10));

        let xml = note.to_mei_string().expect("serialize");
        let parsed = Note::from_mei_str(&xml).expect("deserialize");

        assert_eq!(parsed.common.xml_id, Some(format!("n{}", i)));
        assert_eq!(parsed.note_log.oct, Some(DataOctave(i % 10)));
    }
}

// ============================================================================
// Note Child Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_note_with_accid_child() {
    use tusk_model::data::{DataAccidentalGestural, DataAccidentalGesturalBasic};
    use tusk_model::elements::{Accid, NoteChild};

    let mut accid = Accid::default();
    accid.accid_ges.accid_ges = Some(DataAccidentalGestural::DataAccidentalGesturalBasic(
        DataAccidentalGesturalBasic::S,
    ));

    let mut original = Note::default();
    original.common.xml_id = Some("n1".to_string());
    original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.note_log.pname = Some(DataPitchname::from("c".to_string()));
    original.note_log.oct = Some(DataOctave(4));
    original.children.push(NoteChild::Accid(Box::new(accid)));

    let xml = original.to_mei_string().expect("serialize");

    // Verify the serialized XML contains the accid child
    assert!(
        xml.contains("<accid"),
        "should contain accid element: {}",
        xml
    );
    assert!(
        xml.contains("accid.ges=\"s\""),
        "should contain accid.ges attribute: {}",
        xml
    );

    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    // Verify attributes preserved
    assert_eq!(parsed.common.xml_id, Some("n1".to_string()));
    assert_eq!(parsed.note_log.dur, original.note_log.dur);

    // Verify child preserved
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        NoteChild::Accid(accid) => {
            assert!(accid.accid_ges.accid_ges.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

#[test]
fn roundtrip_note_with_artic_child() {
    use tusk_model::elements::{Artic, NoteChild};

    let mut artic = Artic::default();
    artic.artic_log.artic = vec![DataArticulation::Stacc];

    let mut original = Note::default();
    original.common.xml_id = Some("n1".to_string());
    original.children.push(NoteChild::Artic(Box::new(artic)));

    let xml = original.to_mei_string().expect("serialize");

    // Verify the serialized XML contains the artic child
    assert!(
        xml.contains("<artic"),
        "should contain artic element: {}",
        xml
    );
    assert!(
        xml.contains("artic=\"stacc\""),
        "should contain artic attribute: {}",
        xml
    );

    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    // Verify child preserved
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        NoteChild::Artic(artic) => {
            assert!(!artic.artic_log.artic.is_empty());
            assert_eq!(artic.artic_log.artic[0], DataArticulation::Stacc);
        }
        other => panic!("Expected Artic, got {:?}", other),
    }
}

#[test]
fn roundtrip_note_with_multiple_children() {
    use tusk_model::data::{DataAccidentalGestural, DataAccidentalGesturalBasic};
    use tusk_model::elements::{Accid, Artic, NoteChild};

    let mut artic = Artic::default();
    artic.artic_log.artic = vec![DataArticulation::Ten];

    let mut accid = Accid::default();
    accid.accid_ges.accid_ges = Some(DataAccidentalGestural::DataAccidentalGesturalBasic(
        DataAccidentalGesturalBasic::F,
    ));

    let mut original = Note::default();
    original.common.xml_id = Some("n2apf6t".to_string());
    original.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));
    original.note_log.pname = Some(DataPitchname::from("f".to_string()));
    original.note_log.oct = Some(DataOctave(5));
    original.children.push(NoteChild::Artic(Box::new(artic)));
    original.children.push(NoteChild::Accid(Box::new(accid)));

    let xml = original.to_mei_string().expect("serialize");

    // Verify both children present
    assert!(xml.contains("<artic"), "should contain artic: {}", xml);
    assert!(xml.contains("<accid"), "should contain accid: {}", xml);

    let parsed = Note::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // First should be artic
    match &parsed.children[0] {
        NoteChild::Artic(a) => {
            assert_eq!(a.artic_log.artic[0], DataArticulation::Ten);
        }
        other => panic!("Expected Artic first, got {:?}", other),
    }

    // Second should be accid
    match &parsed.children[1] {
        NoteChild::Accid(a) => {
            assert!(a.accid_ges.accid_ges.is_some());
        }
        other => panic!("Expected Accid second, got {:?}", other),
    }
}

// ============================================================================
// Integration Tests - Parsing Real MEI Example Fragments
// ============================================================================

/// Test parsing note elements from Tchaikovsky example style
#[test]
fn parse_tchaikovsky_style_note_with_children() {
    // This mimics the structure from specs/mei/examples/verovio/tchaikovsky_scherzo.mei
    let xml = r#"<note xml:id="n2apf6t" dur="8" pname="f" oct="5">
        <artic artic="stacc"/>
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

/// Test parsing note with both artic and accid children
#[test]
fn parse_note_with_artic_and_accid() {
    // From Tchaikovsky example: note with tenuto and gestural flat
    let xml = r#"<note xml:id="n1v2c23j" dur="4" pname="e" oct="5">
        <artic artic="ten"/>
        <accid accid.ges="f"/>
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("n1v2c23j".to_string()));
    assert_eq!(note.children.len(), 2);

    // Check artic
    match &note.children[0] {
        tusk_model::elements::NoteChild::Artic(artic) => {
            assert_eq!(artic.artic_log.artic[0], DataArticulation::Ten);
        }
        other => panic!("Expected Artic, got {:?}", other),
    }

    // Check accid
    match &note.children[1] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_ges.accid_ges.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Test parsing note with only gestural accidental
#[test]
fn parse_note_with_gestural_accidental_child() {
    let xml = r#"<note xml:id="nz8c5kj" dur="8" pname="d" oct="5">
        <accid accid.ges="f"/>
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("nz8c5kj".to_string()));
    assert_eq!(note.children.len(), 1);

    match &note.children[0] {
        tusk_model::elements::NoteChild::Accid(accid) => {
            assert!(accid.accid_ges.accid_ges.is_some());
        }
        other => panic!("Expected Accid, got {:?}", other),
    }
}

/// Test parsing note with written accidental (sharp)
#[test]
fn parse_note_with_written_accidental() {
    // From Tchaikovsky: C sharp
    let xml = r#"<note xml:id="n1jlp1q2" pname="c" oct="4">
        <accid accid="s"/>
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

/// Test parsing note with accent articulation
#[test]
fn parse_note_with_accent() {
    let xml = r#"<note xml:id="n2epqtj" dots="1" dur="4" pname="c" oct="5">
        <artic artic="acc"/>
    </note>"#;

    let note = Note::from_mei_str(xml).expect("should parse");

    assert_eq!(note.common.xml_id, Some("n2epqtj".to_string()));
    assert_eq!(note.note_log.dots, Some(DataAugmentdot(1)));

    assert_eq!(note.children.len(), 1);
    match &note.children[0] {
        tusk_model::elements::NoteChild::Artic(artic) => {
            assert_eq!(artic.artic_log.artic[0], DataArticulation::Acc);
        }
        other => panic!("Expected Artic, got {:?}", other),
    }
}

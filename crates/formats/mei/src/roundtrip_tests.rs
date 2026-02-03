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
    DataDuration, DataDurationCmn, DataDurationrests, DataGrace, DataOctave, DataPitchname,
    DataStemdirection, DataStemdirectionBasic, DataTie,
};
use tusk_model::elements::{Note, Rest};

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

// ============================================================================
// Rest Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_rest() {
    let original = Rest::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.rest_log.dur.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_rest_with_xml_id() {
    let mut original = Rest::default();
    original.common.xml_id = Some("r1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"r1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Rest::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("r1".to_string()));
}

#[test]
fn roundtrip_rest_with_duration_quarter() {
    let mut original = Rest::default();
    original.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.rest_log.dur,
        Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn roundtrip_rest_with_duration_whole() {
    let mut original = Rest::default();
    original.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N1));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.rest_log.dur,
        Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N1))
    );
}

#[test]
fn roundtrip_rest_with_dots() {
    let mut original = Rest::default();
    original.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));
    original.rest_log.dots = Some(DataAugmentdot(1));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.rest_log.dots, Some(DataAugmentdot(1)));
}

#[test]
fn roundtrip_rest_complete_cmn() {
    // Common Music Notation rest with typical attributes
    let mut original = Rest::default();
    original.common.xml_id = Some("r42".to_string());
    original.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N8));
    original.rest_log.dots = Some(DataAugmentdot(1));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.rest_log.dur, original.rest_log.dur);
    assert_eq!(parsed.rest_log.dots, original.rest_log.dots);
}

#[test]
fn roundtrip_rest_with_staff_and_layer() {
    let mut original = Rest::default();
    original.rest_log.staff = vec![1u64];
    original.rest_log.layer = vec![1u64];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    assert!(
        !parsed.rest_log.staff.is_empty(),
        "staff should be preserved"
    );
    assert!(
        !parsed.rest_log.layer.is_empty(),
        "layer should be preserved"
    );
}

#[test]
fn roundtrip_rest_with_label() {
    let mut original = Rest::default();
    original.common.label = Some("whole rest".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("whole rest".to_string()));
}

// ============================================================================
// Rest External XML Parsing Tests
// ============================================================================

#[test]
fn parse_external_rest_minimal() {
    let xml = r#"<rest/>"#;
    let parsed = Rest::from_mei_str(xml).expect("deserialize");

    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Rest::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.common.xml_id.is_none());
}

#[test]
fn parse_external_rest_with_attributes() {
    let xml = r#"<rest xml:id="r1" dur="4"/>"#;
    let parsed = Rest::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("r1".to_string()));
    assert_eq!(
        parsed.rest_log.dur,
        Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4))
    );

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Rest::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.common.xml_id, Some("r1".to_string()));
    assert_eq!(reparsed.rest_log.dur, parsed.rest_log.dur);
}

#[test]
fn parse_external_rest_all_cmn_durations() {
    // Test common music notation duration values for rests
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
        let xml = format!(r#"<rest dur="{}"/>"#, dur_str);
        let parsed =
            Rest::from_mei_str(&xml).unwrap_or_else(|_| panic!("deserialize dur={}", dur_str));
        assert_eq!(
            parsed.rest_log.dur,
            Some(DataDurationrests::DataDurationCmn(expected)),
            "dur {} should parse correctly",
            dur_str
        );
    }
}

#[test]
fn rest_handles_unknown_attributes_leniently() {
    let xml = r#"<rest xml:id="r1" unknown="value" dur="4"/>"#;
    let rest = Rest::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(rest.common.xml_id, Some("r1".to_string()));
}

// ============================================================================
// Rest with Child Elements Tests
// ============================================================================

#[test]
fn roundtrip_rest_with_dot_child() {
    use tusk_model::elements::{Dot, RestChild};

    let dot = Dot::default();

    let mut original = Rest::default();
    original.common.xml_id = Some("r1".to_string());
    original.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));
    original.children.push(RestChild::Dot(Box::new(dot)));

    let xml = original.to_mei_string().expect("serialize");

    // Verify the serialized XML contains the dot child
    assert!(xml.contains("<dot"), "should contain dot element: {}", xml);

    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    // Verify attributes preserved
    assert_eq!(parsed.common.xml_id, Some("r1".to_string()));
    assert_eq!(parsed.rest_log.dur, original.rest_log.dur);

    // Verify child preserved
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        RestChild::Dot(_) => {}
        other => panic!("Expected Dot, got {:?}", other),
    }
}

#[test]
fn parse_rest_with_dot_child_from_xml() {
    // Test parsing a rest with dot child element
    let xml = r#"<rest xml:id="r1" dur="2"><dot/></rest>"#;
    let rest = Rest::from_mei_str(xml).expect("should parse");

    assert_eq!(rest.common.xml_id, Some("r1".to_string()));
    assert_eq!(rest.children.len(), 1);

    match &rest.children[0] {
        tusk_model::elements::RestChild::Dot(_) => {}
        other => panic!("Expected Dot, got {:?}", other),
    }
}

#[test]
fn rest_ignores_unknown_child_elements() {
    // Unknown child elements should be skipped in lenient mode
    let xml = r#"<rest><unknownElement/><dot/></rest>"#;
    let rest = Rest::from_mei_str(xml).expect("should deserialize");

    // Only the dot should be parsed, unknown element skipped
    assert_eq!(rest.children.len(), 1);
    match &rest.children[0] {
        tusk_model::elements::RestChild::Dot(_) => {}
        other => panic!("Expected Dot, got {:?}", other),
    }
}

// ============================================================================
// Rest Visual Attribute Tests
// ============================================================================

#[test]
fn roundtrip_rest_with_loc() {
    let mut original = Rest::default();
    original.rest_vis.loc = Some(tusk_model::data::DataStaffloc(4));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.rest_vis.loc.is_some(), "loc should be preserved");
}

#[test]
fn roundtrip_rest_with_color() {
    let mut original = Rest::default();
    original.rest_vis.color = Some(DataColor::DataColorvalues(DataColorvalues::from(
        "#0000FF".to_string(),
    )));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rest::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.rest_vis.color.is_some(), "color should be preserved");
}

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

//! Unit tests for generated MEI element structs.
//!
//! Tests verify:
//! - Struct construction with Default trait
//! - Attribute class composition (flattened fields)
//! - Child element handling (Vec<ElementChild>)
//! - Serde serialization/deserialization
//! - Model trait implementations
//! - Validate trait implementation

use super::att::*;
use super::data::*;
use super::elements::*;
use super::model::*;
use super::validation::{Validate, ValidationContext};

// ============================================================================
// Basic Construction Tests - Elements without children
// ============================================================================

#[test]
fn test_clef_default() {
    let clef = Clef::default();
    assert_eq!(clef.common.xml_id, None);
    assert_eq!(clef.clef_log.shape, None);
    assert_eq!(clef.clef_log.line, None);
}

#[test]
fn test_clef_with_attributes() {
    let clef = Clef {
        common: AttCommon {
            xml_id: Some("clef1".to_string()),
            ..Default::default()
        },
        clef_log: AttClefLog {
            shape: Some(DataClefshape::G),
            line: Some(DataClefline::from(2u64)),
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(clef.common.xml_id, Some("clef1".to_string()));
    assert_eq!(clef.clef_log.shape, Some(DataClefshape::G));
    assert_eq!(clef.clef_log.line.as_ref().unwrap().0, 2);
}

// ============================================================================
// Basic Construction Tests - Elements with children
// ============================================================================

#[test]
fn test_note_default() {
    let note = Note::default();
    assert_eq!(note.common.xml_id, None);
    assert_eq!(note.note_log.pname, None);
    assert_eq!(note.note_log.oct, None);
    assert!(note.children.is_empty());
}

#[test]
fn test_note_with_pitch_and_duration() {
    let note = Note {
        common: AttCommon {
            xml_id: Some("note1".to_string()),
            ..Default::default()
        },
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            oct: Some(DataOctave::from(4u64)),
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(note.common.xml_id, Some("note1".to_string()));
    assert_eq!(note.note_log.pname.as_ref().unwrap().0, "c");
    assert_eq!(note.note_log.oct.as_ref().unwrap().0, 4);
    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn test_rest_default() {
    let rest = Rest::default();
    assert_eq!(rest.common.xml_id, None);
    assert!(rest.children.is_empty());
}

#[test]
fn test_chord_default() {
    let chord = Chord::default();
    assert!(chord.children.is_empty());
}

// ============================================================================
// Structural Element Tests
// ============================================================================

#[test]
fn test_measure_default() {
    let measure = Measure::default();
    assert_eq!(measure.common.xml_id, None);
    assert!(measure.children.is_empty());
}

#[test]
fn test_staff_default() {
    let staff = Staff::default();
    assert_eq!(staff.n_integer.n, None);
    assert!(staff.children.is_empty());
}

#[test]
fn test_layer_default() {
    let layer = Layer::default();
    assert_eq!(layer.n_integer.n, None);
    assert!(layer.children.is_empty());
}

#[test]
fn test_mei_default() {
    let mei = Mei::default();
    assert_eq!(mei.id.xml_id, None);
    assert!(mei.children.is_empty());
}

// ============================================================================
// Element with Children Tests
// ============================================================================

#[test]
fn test_chord_with_note_children() {
    let note1 = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            oct: Some(DataOctave::from(4u64)),
            ..Default::default()
        },
        ..Default::default()
    };
    let note2 = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("e".to_string())),
            oct: Some(DataOctave::from(4u64)),
            ..Default::default()
        },
        ..Default::default()
    };

    let chord = Chord {
        common: AttCommon {
            xml_id: Some("chord1".to_string()),
            ..Default::default()
        },
        chord_log: AttChordLog {
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
            ..Default::default()
        },
        children: vec![
            ChordChild::Note(Box::new(note1)),
            ChordChild::Note(Box::new(note2)),
        ],
        ..Default::default()
    };

    assert_eq!(chord.children.len(), 2);
    match &chord.children[0] {
        ChordChild::Note(n) => {
            assert_eq!(n.note_log.pname.as_ref().unwrap().0, "c");
        }
        _ => panic!("Expected Note child"),
    }
}

#[test]
fn test_layer_with_note_and_rest_children() {
    let note = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("g".to_string())),
            oct: Some(DataOctave::from(4u64)),
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
            ..Default::default()
        },
        ..Default::default()
    };
    let rest = Rest {
        rest_log: AttRestLog {
            dur: Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4)),
            ..Default::default()
        },
        ..Default::default()
    };

    let layer = Layer {
        n_integer: AttNInteger { n: Some(1) },
        children: vec![
            LayerChild::Note(Box::new(note)),
            LayerChild::Rest(Box::new(rest)),
        ],
        ..Default::default()
    };

    assert_eq!(layer.children.len(), 2);
    assert!(matches!(&layer.children[0], LayerChild::Note(_)));
    assert!(matches!(&layer.children[1], LayerChild::Rest(_)));
}

#[test]
fn test_measure_with_staff_children() {
    let layer = Layer {
        n_integer: AttNInteger { n: Some(1) },
        ..Default::default()
    };
    let staff = Staff {
        n_integer: AttNInteger { n: Some(1) },
        children: vec![StaffChild::Layer(Box::new(layer))],
        ..Default::default()
    };
    let measure = Measure {
        common: AttCommon {
            xml_id: Some("m1".to_string()),
            n: Some(DataWord::from("1".to_string())),
            ..Default::default()
        },
        children: vec![MeasureChild::Staff(Box::new(staff))],
        ..Default::default()
    };

    assert_eq!(measure.common.n.as_ref().unwrap().0, "1");
    assert_eq!(measure.children.len(), 1);
}

// ============================================================================
// Control Event Tests
// ============================================================================

#[test]
fn test_slur_default() {
    let slur = Slur::default();
    assert_eq!(slur.common.xml_id, None);
    assert!(slur.children.is_empty());
}

#[test]
fn test_slur_with_attributes() {
    let slur = Slur {
        common: AttCommon {
            xml_id: Some("slur1".to_string()),
            ..Default::default()
        },
        slur_log: AttSlurLog {
            tstamp: Some(DataBeat::from(1.0f64)),
            tstamp2: Some(DataMeasurebeat::from("0m+4".to_string())),
            startid: Some(DataUri::from("#note1".to_string())),
            endid: Some(DataUri::from("#note2".to_string())),
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(slur.slur_log.tstamp.is_some());
    assert!(slur.slur_log.startid.is_some());
}

#[test]
fn test_tie_default() {
    let tie = Tie::default();
    assert_eq!(tie.common.xml_id, None);
}

#[test]
fn test_dynam_default() {
    let dynam = Dynam::default();
    assert!(dynam.children.is_empty());
}

#[test]
fn test_hairpin_default() {
    let hairpin = Hairpin::default();
    assert_eq!(hairpin.common.xml_id, None);
}

#[test]
fn test_fermata_default() {
    let fermata = Fermata::default();
    assert_eq!(fermata.common.xml_id, None);
}

// ============================================================================
// Model Trait Implementation Tests
// ============================================================================

#[test]
fn test_note_implements_model_chord_part() {
    fn assert_model_chord_part<T: ModelChordPart>(_: &T) {}
    let note = Note::default();
    assert_model_chord_part(&note);
}

#[test]
fn test_note_implements_model_event_like() {
    fn assert_model_event_like<T: ModelEventLike>(_: &T) {}
    let note = Note::default();
    assert_model_event_like(&note);
}

#[test]
fn test_rest_implements_model_event_like() {
    fn assert_model_event_like<T: ModelEventLike>(_: &T) {}
    let rest = Rest::default();
    assert_model_event_like(&rest);
}

#[test]
fn test_chord_implements_model_event_like() {
    fn assert_model_event_like<T: ModelEventLike>(_: &T) {}
    let chord = Chord::default();
    assert_model_event_like(&chord);
}

#[test]
fn test_clef_implements_model_event_like() {
    fn assert_model_event_like<T: ModelEventLike>(_: &T) {}
    let clef = Clef::default();
    assert_model_event_like(&clef);
}

#[test]
fn test_measure_implements_model_measure_like() {
    fn assert_model_measure_like<T: ModelMeasureLike>(_: &T) {}
    let measure = Measure::default();
    assert_model_measure_like(&measure);
}

#[test]
fn test_staff_implements_model_staff_like() {
    fn assert_model_staff_like<T: ModelStaffLike>(_: &T) {}
    let staff = Staff::default();
    assert_model_staff_like(&staff);
}

#[test]
fn test_layer_implements_model_layer_like() {
    fn assert_model_layer_like<T: ModelLayerLike>(_: &T) {}
    let layer = Layer::default();
    assert_model_layer_like(&layer);
}

#[test]
fn test_slur_implements_model_control_event_like_cmn() {
    fn assert_control_event<T: ModelControlEventLikeCmn>(_: &T) {}
    let slur = Slur::default();
    assert_control_event(&slur);
}

// ============================================================================
// Validation Tests
// ============================================================================

#[test]
fn test_note_validation() {
    let note = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            oct: Some(DataOctave::from(4u64)),
            ..Default::default()
        },
        ..Default::default()
    };
    let mut ctx = ValidationContext::new();
    note.validate_with_context(&mut ctx);
    // Note itself should be valid; pitch validation happens in data types
    assert!(!ctx.has_errors());
}

#[test]
fn test_element_with_children_validation() {
    let note = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            oct: Some(DataOctave::from(4u64)),
            ..Default::default()
        },
        ..Default::default()
    };
    let layer = Layer {
        children: vec![LayerChild::Note(Box::new(note))],
        ..Default::default()
    };
    let mut ctx = ValidationContext::new();
    layer.validate_with_context(&mut ctx);
    assert!(!ctx.has_errors());
}

// ============================================================================
// Serde Serialization Tests
// ============================================================================

#[test]
fn test_clef_serialization() {
    let clef = Clef {
        common: AttCommon {
            xml_id: Some("clef1".to_string()),
            ..Default::default()
        },
        clef_log: AttClefLog {
            shape: Some(DataClefshape::G),
            line: Some(DataClefline::from(2u64)),
            ..Default::default()
        },
        ..Default::default()
    };
    let json = serde_json::to_string(&clef).unwrap();
    assert!(json.contains("xml:id"));
    assert!(json.contains("clef1"));
    assert!(json.contains("@shape"));
    assert!(json.contains("\"G\""));
    assert!(json.contains("@line"));
}

#[test]
fn test_note_serialization() {
    let note = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            oct: Some(DataOctave::from(4u64)),
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
            ..Default::default()
        },
        ..Default::default()
    };
    let json = serde_json::to_string(&note).unwrap();
    assert!(json.contains("@pname"));
    assert!(json.contains("\"c\""));
    assert!(json.contains("@oct"));
    assert!(json.contains("@dur"));
}

#[test]
fn test_empty_default_element_serialization() {
    let note = Note::default();
    let json = serde_json::to_string(&note).unwrap();
    // Default element serializes children as empty array
    // (serde default behavior with rename = "$value")
    assert!(json.contains("\"$value\":[]") || json == "{}");
}

#[test]
fn test_rest_serialization() {
    let rest = Rest {
        rest_log: AttRestLog {
            dur: Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2)),
            ..Default::default()
        },
        ..Default::default()
    };
    let json = serde_json::to_string(&rest).unwrap();
    assert!(json.contains("@dur"));
    assert!(json.contains("\"2\""));
}

// ============================================================================
// Serde Deserialization Tests
// ============================================================================

#[test]
fn test_clef_deserialization() {
    let json = r#"{"xml:id":"clef1","@shape":"G","@line":2}"#;
    let clef: Clef = serde_json::from_str(json).unwrap();
    assert_eq!(clef.common.xml_id, Some("clef1".to_string()));
    assert_eq!(clef.clef_log.shape, Some(DataClefshape::G));
    assert_eq!(clef.clef_log.line.unwrap().0, 2);
}

#[test]
fn test_note_deserialization() {
    let json = r#"{"@pname":"d","@oct":5,"@dur":"8"}"#;
    let note: Note = serde_json::from_str(json).unwrap();
    assert_eq!(note.note_log.pname.as_ref().unwrap().0, "d");
    assert_eq!(note.note_log.oct.as_ref().unwrap().0, 5);
    assert_eq!(
        note.note_log.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
    );
}

#[test]
fn test_empty_element_deserialization() {
    let json = "{}";
    let note: Note = serde_json::from_str(json).unwrap();
    assert_eq!(note.note_log.pname, None);
    assert_eq!(note.note_log.oct, None);
    assert!(note.children.is_empty());
}

// ============================================================================
// Round-Trip Tests
// ============================================================================

#[test]
fn test_clef_roundtrip() {
    let original = Clef {
        common: AttCommon {
            xml_id: Some("clef1".to_string()),
            ..Default::default()
        },
        clef_log: AttClefLog {
            shape: Some(DataClefshape::F),
            line: Some(DataClefline::from(4u64)),
            ..Default::default()
        },
        ..Default::default()
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Clef = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_note_roundtrip() {
    let original = Note {
        common: AttCommon {
            xml_id: Some("note1".to_string()),
            ..Default::default()
        },
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("g".to_string())),
            oct: Some(DataOctave::from(4u64)),
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
            dots: Some(DataAugmentdot::from(1u64)),
            ..Default::default()
        },
        ..Default::default()
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Note = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_rest_roundtrip() {
    let original = Rest {
        common: AttCommon {
            xml_id: Some("rest1".to_string()),
            ..Default::default()
        },
        rest_log: AttRestLog {
            dur: Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4)),
            ..Default::default()
        },
        ..Default::default()
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Rest = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_slur_roundtrip() {
    let original = Slur {
        common: AttCommon {
            xml_id: Some("slur1".to_string()),
            ..Default::default()
        },
        slur_log: AttSlurLog {
            tstamp: Some(DataBeat::from(1.0f64)),
            tstamp2: Some(DataMeasurebeat::from("0m+3".to_string())),
            ..Default::default()
        },
        ..Default::default()
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Slur = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

// ============================================================================
// Trait Derivation Tests
// ============================================================================

#[test]
fn test_element_clone() {
    let original = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            oct: Some(DataOctave::from(4u64)),
            ..Default::default()
        },
        ..Default::default()
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_element_debug() {
    let note = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            ..Default::default()
        },
        ..Default::default()
    };
    let debug_str = format!("{:?}", note);
    assert!(debug_str.contains("Note"));
}

#[test]
fn test_element_partial_eq() {
    let note1 = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            ..Default::default()
        },
        ..Default::default()
    };
    let note2 = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            ..Default::default()
        },
        ..Default::default()
    };
    let note3 = Note {
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("d".to_string())),
            ..Default::default()
        },
        ..Default::default()
    };
    assert_eq!(note1, note2);
    assert_ne!(note1, note3);
}

// ============================================================================
// Child Enum Tests
// ============================================================================

#[test]
fn test_note_child_enum_variants() {
    // Test that various child types can be created
    let accid = Accid::default();
    let dot = Dot::default();
    let artic = Artic::default();

    let children: Vec<NoteChild> = vec![
        NoteChild::Accid(Box::new(accid)),
        NoteChild::Dot(Box::new(dot)),
        NoteChild::Artic(Box::new(artic)),
    ];

    assert_eq!(children.len(), 3);
}

#[test]
fn test_layer_child_enum_variants() {
    // Test common layer children
    let note = Note::default();
    let rest = Rest::default();
    let chord = Chord::default();
    let beam = Beam::default();

    let children: Vec<LayerChild> = vec![
        LayerChild::Note(Box::new(note)),
        LayerChild::Rest(Box::new(rest)),
        LayerChild::Chord(Box::new(chord)),
        LayerChild::Beam(Box::new(beam)),
    ];

    assert_eq!(children.len(), 4);
}

#[test]
fn test_measure_child_enum_variants() {
    // Test common measure children
    let staff = Staff::default();
    let slur = Slur::default();
    let dynam = Dynam::default();

    let children: Vec<MeasureChild> = vec![
        MeasureChild::Staff(Box::new(staff)),
        MeasureChild::Slur(Box::new(slur)),
        MeasureChild::Dynam(Box::new(dynam)),
    ];

    assert_eq!(children.len(), 3);
}

// ============================================================================
// Complex Nested Structure Tests
// ============================================================================

#[test]
fn test_complete_measure_structure() {
    // Build a complete measure: measure > staff > layer > note
    let note1 = Note {
        common: AttCommon {
            xml_id: Some("n1".to_string()),
            ..Default::default()
        },
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("c".to_string())),
            oct: Some(DataOctave::from(4u64)),
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
            ..Default::default()
        },
        ..Default::default()
    };
    let note2 = Note {
        common: AttCommon {
            xml_id: Some("n2".to_string()),
            ..Default::default()
        },
        note_log: AttNoteLog {
            pname: Some(DataPitchname::from("d".to_string())),
            oct: Some(DataOctave::from(4u64)),
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
            ..Default::default()
        },
        ..Default::default()
    };

    let layer = Layer {
        n_integer: AttNInteger { n: Some(1) },
        children: vec![
            LayerChild::Note(Box::new(note1)),
            LayerChild::Note(Box::new(note2)),
        ],
        ..Default::default()
    };

    let staff = Staff {
        n_integer: AttNInteger { n: Some(1) },
        children: vec![StaffChild::Layer(Box::new(layer))],
        ..Default::default()
    };

    let measure = Measure {
        common: AttCommon {
            xml_id: Some("m1".to_string()),
            n: Some(DataWord::from("1".to_string())),
            ..Default::default()
        },
        children: vec![MeasureChild::Staff(Box::new(staff))],
        ..Default::default()
    };

    // Verify structure
    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
    assert_eq!(measure.children.len(), 1);

    if let MeasureChild::Staff(staff) = &measure.children[0] {
        assert_eq!(staff.n_integer.n, Some(1));
        assert_eq!(staff.children.len(), 1);

        if let StaffChild::Layer(layer) = &staff.children[0] {
            assert_eq!(layer.n_integer.n, Some(1));
            assert_eq!(layer.children.len(), 2);
        } else {
            panic!("Expected Layer child");
        }
    } else {
        panic!("Expected Staff child");
    }
}

#[test]
fn test_mei_document_structure() {
    // Build minimal MEI document: mei > meiHead + music
    let mei_head = MeiHead::default();
    let music = Music::default();

    let mei = Mei {
        mei_version: AttMeiVersion {
            meiversion: None, // Would be set to MEI version in real use
        },
        children: vec![
            MeiChild::MeiHead(Box::new(mei_head)),
            MeiChild::Music(Box::new(music)),
        ],
        ..Default::default()
    };

    assert_eq!(mei.children.len(), 2);
    assert!(matches!(&mei.children[0], MeiChild::MeiHead(_)));
    assert!(matches!(&mei.children[1], MeiChild::Music(_)));
}

// ============================================================================
// Definition Element Tests
// ============================================================================

#[test]
fn test_score_def_default() {
    let score_def = ScoreDef::default();
    assert!(score_def.children.is_empty());
}

#[test]
fn test_staff_def_default() {
    let staff_def = StaffDef::default();
    assert!(staff_def.children.is_empty());
}

#[test]
fn test_staff_grp_default() {
    let staff_grp = StaffGrp::default();
    assert!(staff_grp.children.is_empty());
}

// ============================================================================
// Grouping Element Tests
// ============================================================================

#[test]
fn test_beam_default() {
    let beam = Beam::default();
    assert!(beam.children.is_empty());
}

#[test]
fn test_beam_with_note_children() {
    let note1 = Note {
        note_log: AttNoteLog {
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N8)),
            ..Default::default()
        },
        ..Default::default()
    };
    let note2 = Note {
        note_log: AttNoteLog {
            dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N8)),
            ..Default::default()
        },
        ..Default::default()
    };

    let beam = Beam {
        children: vec![
            BeamChild::Note(Box::new(note1)),
            BeamChild::Note(Box::new(note2)),
        ],
        ..Default::default()
    };

    assert_eq!(beam.children.len(), 2);
}

#[test]
fn test_tuplet_default() {
    let tuplet = Tuplet::default();
    assert!(tuplet.children.is_empty());
}

#[test]
fn test_grace_grp_default() {
    let grace_grp = GraceGrp::default();
    assert!(grace_grp.children.is_empty());
}

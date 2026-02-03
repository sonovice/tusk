//! Unit tests for generated MEI attribute classes.
//!
//! Tests verify:
//! - Struct construction with Default and builder-style initialization
//! - Serde serialization/deserialization with proper attribute naming
//! - Clone, PartialEq, Debug trait derivations
//! - Optional fields skip serialization when None
//! - Vec fields skip serialization when empty

use super::att::*;
use super::data::*;

// ============================================================================
// Basic Construction Tests
// ============================================================================

#[test]
fn test_att_duration_log_default() {
    let att = AttDurationLog::default();
    assert_eq!(att.dur, None);
}

#[test]
fn test_att_duration_log_with_value() {
    let att = AttDurationLog {
        dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
    };
    assert!(att.dur.is_some());
    assert_eq!(
        att.dur.unwrap(),
        DataDuration::DataDurationCmn(DataDurationCmn::N4)
    );
}

#[test]
fn test_att_pitch_default() {
    let att = AttPitch::default();
    assert_eq!(att.pname, None);
}

#[test]
fn test_att_pitch_with_value() {
    let att = AttPitch {
        pname: Some(DataPitchname::from("c".to_string())),
    };
    assert!(att.pname.is_some());
    assert_eq!(att.pname.unwrap().0, "c");
}

#[test]
fn test_att_octave_default() {
    let att = AttOctave::default();
    assert_eq!(att.oct, None);
}

#[test]
fn test_att_octave_with_value() {
    let att = AttOctave {
        oct: Some(DataOctave::from(4u64)),
    };
    assert!(att.oct.is_some());
    assert_eq!(att.oct.unwrap().0, 4);
}

#[test]
fn test_att_clef_shape_default() {
    let att = AttClefShape::default();
    assert_eq!(att.shape, None);
}

#[test]
fn test_att_clef_shape_with_value() {
    let att = AttClefShape {
        shape: Some(DataClefshape::G),
    };
    assert_eq!(att.shape, Some(DataClefshape::G));
}

#[test]
fn test_att_accidental_default() {
    let att = AttAccidental::default();
    assert_eq!(att.accid, None);
}

#[test]
fn test_att_accidental_with_value() {
    let att = AttAccidental {
        accid: Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::S,
        )),
    };
    assert!(att.accid.is_some());
}

// ============================================================================
// Struct with Vec Fields Tests
// ============================================================================

#[test]
fn test_att_articulation_default() {
    let att = AttArticulation::default();
    assert!(att.artic.is_empty());
}

#[test]
fn test_att_articulation_with_values() {
    let att = AttArticulation {
        artic: vec![DataArticulation::Acc, DataArticulation::Stacc],
    };
    assert_eq!(att.artic.len(), 2);
    assert_eq!(att.artic[0], DataArticulation::Acc);
    assert_eq!(att.artic[1], DataArticulation::Stacc);
}

#[test]
fn test_att_staff_ident_default() {
    let att = AttStaffIdent::default();
    assert!(att.staff.is_empty());
}

#[test]
fn test_att_staff_ident_with_values() {
    let att = AttStaffIdent {
        staff: vec![1, 2, 3],
    };
    assert_eq!(att.staff.len(), 3);
    assert_eq!(att.staff, vec![1, 2, 3]);
}

// ============================================================================
// Complex Struct Tests (AttCommon)
// ============================================================================

#[test]
fn test_att_common_default() {
    let att = AttCommon::default();
    assert_eq!(att.xml_id, None);
    assert_eq!(att.xml_base, None);
    assert_eq!(att.label, None);
    assert_eq!(att.copyof, None);
    assert!(att.corresp.is_empty());
    assert!(att.follows.is_empty());
    assert!(att.next.is_empty());
    assert!(att.precedes.is_empty());
    assert!(att.prev.is_empty());
    assert!(att.sameas.is_empty());
    assert!(att.synch.is_empty());
    assert_eq!(att.n, None);
    assert!(att.resp.is_empty());
    assert!(att.class.is_empty());
    assert!(att.r#type.is_empty());
}

#[test]
fn test_att_common_with_partial_values() {
    let att = AttCommon {
        xml_id: Some("note1".to_string()),
        n: Some(DataWord::from("1".to_string())),
        label: Some("First note".to_string()),
        ..Default::default()
    };
    assert_eq!(att.xml_id, Some("note1".to_string()));
    assert_eq!(att.n.as_ref().unwrap().0, "1");
    assert_eq!(att.label, Some("First note".to_string()));
    // Other fields should be default
    assert!(att.corresp.is_empty());
}

// ============================================================================
// Inline Enum Tests (AttMeiVersion)
// ============================================================================

#[test]
fn test_att_mei_version_default() {
    let att = AttMeiVersion::default();
    assert_eq!(att.meiversion, None);
}

// Note: AttMeiVersionMeiversion is not re-exported from the att module,
// so we test that the struct can hold a value by deserializing from JSON
#[test]
fn test_att_mei_version_from_json() {
    let json = r#"{"@meiversion":"6.0-dev+cmn"}"#;
    let att: AttMeiVersion = serde_json::from_str(json).unwrap();
    assert!(att.meiversion.is_some());
}

// ============================================================================
// Serde Serialization Tests
// ============================================================================

#[test]
fn test_att_duration_log_serialization() {
    let att = AttDurationLog {
        dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
    };
    let json = serde_json::to_string(&att).unwrap();
    // Should use "@dur" as the key
    assert!(json.contains("@dur"));
    assert!(json.contains("\"4\""));
}

#[test]
fn test_att_duration_log_serialization_none_skipped() {
    let att = AttDurationLog { dur: None };
    let json = serde_json::to_string(&att).unwrap();
    // Empty object because None fields are skipped
    assert_eq!(json, "{}");
}

#[test]
fn test_att_pitch_serialization() {
    let att = AttPitch {
        pname: Some(DataPitchname::from("g".to_string())),
    };
    let json = serde_json::to_string(&att).unwrap();
    assert!(json.contains("@pname"));
    assert!(json.contains("\"g\""));
}

#[test]
fn test_att_clef_shape_serialization() {
    let att = AttClefShape {
        shape: Some(DataClefshape::F),
    };
    let json = serde_json::to_string(&att).unwrap();
    assert!(json.contains("@shape"));
    assert!(json.contains("\"F\""));
}

#[test]
fn test_att_articulation_empty_vec_skipped() {
    let att = AttArticulation { artic: vec![] };
    let json = serde_json::to_string(&att).unwrap();
    // Empty vec should be skipped
    assert_eq!(json, "{}");
}

#[test]
fn test_att_articulation_serialization() {
    let att = AttArticulation {
        artic: vec![DataArticulation::Stacc],
    };
    let json = serde_json::to_string(&att).unwrap();
    assert!(json.contains("@artic"));
    assert!(json.contains("stacc"));
}

#[test]
fn test_att_common_serialization() {
    let att = AttCommon {
        xml_id: Some("measure1".to_string()),
        n: Some(DataWord::from("1".to_string())),
        ..Default::default()
    };
    let json = serde_json::to_string(&att).unwrap();
    // xml:id should serialize as "xml:id"
    assert!(json.contains("xml:id"));
    assert!(json.contains("measure1"));
    // @n attribute
    assert!(json.contains("@n"));
}

// ============================================================================
// Serde Deserialization Tests
// ============================================================================

#[test]
fn test_att_duration_log_deserialization() {
    let json = r#"{"@dur":"8"}"#;
    let att: AttDurationLog = serde_json::from_str(json).unwrap();
    assert_eq!(
        att.dur,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
    );
}

#[test]
fn test_att_duration_log_deserialization_empty() {
    let json = "{}";
    let att: AttDurationLog = serde_json::from_str(json).unwrap();
    assert_eq!(att.dur, None);
}

#[test]
fn test_att_pitch_deserialization() {
    let json = r#"{"@pname":"d"}"#;
    let att: AttPitch = serde_json::from_str(json).unwrap();
    assert!(att.pname.is_some());
    assert_eq!(att.pname.unwrap().0, "d");
}

#[test]
fn test_att_clef_shape_deserialization() {
    let json = r#"{"@shape":"C"}"#;
    let att: AttClefShape = serde_json::from_str(json).unwrap();
    assert_eq!(att.shape, Some(DataClefshape::C));
}

#[test]
fn test_att_articulation_deserialization() {
    let json = r#"{"@artic":["acc","ten"]}"#;
    let att: AttArticulation = serde_json::from_str(json).unwrap();
    assert_eq!(att.artic.len(), 2);
    assert_eq!(att.artic[0], DataArticulation::Acc);
    assert_eq!(att.artic[1], DataArticulation::Ten);
}

#[test]
fn test_att_common_deserialization() {
    let json = r#"{"xml:id":"note42","@n":"5","@label":"Test"}"#;
    let att: AttCommon = serde_json::from_str(json).unwrap();
    assert_eq!(att.xml_id, Some("note42".to_string()));
    assert_eq!(att.n.as_ref().unwrap().0, "5");
    assert_eq!(att.label, Some("Test".to_string()));
}

#[test]
fn test_att_mei_version_deserialization() {
    let json = r#"{"@meiversion":"6.0-dev+basic"}"#;
    let att: AttMeiVersion = serde_json::from_str(json).unwrap();
    assert!(att.meiversion.is_some());
}

// ============================================================================
// Round-Trip Tests
// ============================================================================

#[test]
fn test_att_duration_log_roundtrip() {
    let original = AttDurationLog {
        dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N16)),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AttDurationLog = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_att_pitch_roundtrip() {
    let original = AttPitch {
        pname: Some(DataPitchname::from("f".to_string())),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AttPitch = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_att_common_roundtrip() {
    let original = AttCommon {
        xml_id: Some("elem1".to_string()),
        label: Some("Label".to_string()),
        n: Some(DataWord::from("42".to_string())),
        corresp: vec![
            DataUri::from("#note1".to_string()),
            DataUri::from("#note2".to_string()),
        ],
        r#type: vec!["primary".to_string(), "editorial".to_string()],
        ..Default::default()
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AttCommon = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

// ============================================================================
// Trait Derivation Tests
// ============================================================================

#[test]
fn test_att_duration_log_clone() {
    let original = AttDurationLog {
        dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N2)),
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

#[test]
fn test_att_duration_log_debug() {
    let att = AttDurationLog {
        dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
    };
    let debug_str = format!("{:?}", att);
    assert!(debug_str.contains("AttDurationLog"));
    assert!(debug_str.contains("dur"));
}

#[test]
fn test_att_duration_log_partial_eq() {
    let att1 = AttDurationLog {
        dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
    };
    let att2 = AttDurationLog {
        dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N4)),
    };
    let att3 = AttDurationLog {
        dur: Some(DataDuration::DataDurationCmn(DataDurationCmn::N8)),
    };

    assert_eq!(att1, att2);
    assert_ne!(att1, att3);
}

#[test]
fn test_att_common_clone() {
    let original = AttCommon {
        xml_id: Some("id1".to_string()),
        corresp: vec![DataUri::from("#ref1".to_string())],
        ..Default::default()
    };
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

// ============================================================================
// Domain-Specific Attribute Class Tests (.log, .ges, .vis, .anl)
// ============================================================================

#[test]
fn test_att_duration_ges() {
    // Gestural duration (performed duration)
    let att = AttDurationGes {
        dur_ges: Some(DataDurationGestural::DataDurationCmn(DataDurationCmn::N4)),
        dots_ges: None,
        dur_metrical: None,
        dur_ppq: None,
        dur_real: None,
        dur_recip: None,
    };
    assert!(att.dur_ges.is_some());
}

#[test]
fn test_att_pitch_ges() {
    // Gestural pitch (sounding pitch)
    let att = AttPitchGes {
        oct_ges: Some(DataOctave::from(4u64)),
        pname_ges: Some(DataPitchnameGestural::from("c".to_string())),
        pnum: None,
    };
    assert!(att.pname_ges.is_some());
    assert!(att.oct_ges.is_some());
}

#[test]
fn test_att_accid_ges() {
    // Gestural accidental (sounding accidental after key signature)
    let att = AttAccidGes {
        accid_ges: Some(DataAccidentalGestural::DataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::F,
        )),
    };
    assert!(att.accid_ges.is_some());
}

#[test]
fn test_att_control_event() {
    // Control events have timestamp
    let att = AttControlEvent {
        when: None,
        layer: vec![],
        part: vec![],
        partstaff: vec![],
        plist: vec![],
        staff: vec![1],
        evaluate: None,
        tstamp: Some(DataBeat::from(1.0f64)),
        tstamp_ges: None,
        tstamp_real: None,
    };
    assert!(att.tstamp.is_some());
    assert_eq!(att.staff, vec![1]);
}

#[test]
fn test_att_timestamp_log() {
    let att = AttTimestampLog {
        tstamp: Some(DataBeat::from(1.5f64)),
    };
    assert!(att.tstamp.is_some());
    assert_eq!(att.tstamp.unwrap().0, 1.5);
}

// ============================================================================
// Additional Attribute Class Tests
// ============================================================================

#[test]
fn test_att_layer_ident() {
    let att = AttLayerIdent { layer: vec![1] };
    assert_eq!(att.layer, vec![1]);
}

#[test]
fn test_att_augment_dots() {
    let att = AttAugmentDots {
        dots: Some(DataAugmentdot::from(2u64)),
    };
    assert_eq!(att.dots.as_ref().unwrap().0, 2);
}

#[test]
fn test_att_octave_displacement() {
    let att = AttOctaveDisplacement {
        dis: Some(DataOctaveDis::from(8u64)),
        dis_place: Some(DataStaffrelBasic::Above),
    };
    assert!(att.dis.is_some());
    assert_eq!(att.dis_place, Some(DataStaffrelBasic::Above));
}

#[test]
fn test_att_tie_present() {
    let att = AttTiePresent {
        tie: vec![DataTie::from("i".to_string())],
    };
    assert_eq!(att.tie.len(), 1);
}

#[test]
fn test_att_beaming_log() {
    let att = AttBeamingLog {
        beam_group: None,
        beam_rests: None,
    };
    assert_eq!(att.beam_group, None);
}

#[test]
fn test_att_stems() {
    let att = AttStems {
        stem_with: None,
        stem_form: None,
        stem_dir: Some(DataStemdirection::DataStemdirectionBasic(
            DataStemdirectionBasic::Up,
        )),
        stem_len: None,
        stem_mod: None,
        stem_pos: None,
        stem_sameas: None,
        stem_visible: None,
        stem_x: None,
        stem_y: None,
    };
    assert!(att.stem_dir.is_some());
}

#[test]
fn test_att_slur_present() {
    let att = AttSlurPresent {
        slur: vec![DataSlur::from("1i".to_string())],
    };
    assert_eq!(att.slur.len(), 1);
}

#[test]
fn test_att_tuplet_present() {
    let att = AttTupletPresent {
        tuplet: vec![DataTuplet::from("1i".to_string())],
    };
    assert_eq!(att.tuplet.len(), 1);
}

#[test]
fn test_att_color() {
    let att = AttColor {
        color: Some(DataColor::DataColorvalues(DataColorvalues::from(
            "#FF0000".to_string(),
        ))),
    };
    assert!(att.color.is_some());
}

#[test]
fn test_att_visibility() {
    let att = AttVisibility {
        visible: Some(DataBoolean::True),
    };
    assert_eq!(att.visible, Some(DataBoolean::True));
}

// ============================================================================
// Serialization/Deserialization Round-Trip for Complex Structs
// ============================================================================

#[test]
fn test_att_stems_roundtrip() {
    let original = AttStems {
        stem_with: None,
        stem_form: None,
        stem_dir: Some(DataStemdirection::DataStemdirectionBasic(
            DataStemdirectionBasic::Down,
        )),
        stem_len: None,
        stem_mod: None,
        stem_pos: None,
        stem_sameas: None,
        stem_visible: Some(DataBoolean::True),
        stem_x: None,
        stem_y: None,
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AttStems = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_att_control_event_roundtrip() {
    let original = AttControlEvent {
        when: None,
        layer: vec![1, 2],
        part: vec![],
        partstaff: vec![],
        plist: vec![],
        staff: vec![1],
        evaluate: None,
        tstamp: Some(DataBeat::from(2.5f64)),
        tstamp_ges: None,
        tstamp_real: None,
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AttControlEvent = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn test_att_duration_ges_roundtrip() {
    let original = AttDurationGes {
        dur_ges: Some(DataDurationGestural::DataDurationCmn(DataDurationCmn::N8)),
        dots_ges: Some(DataAugmentdot::from(1u64)),
        dur_metrical: None,
        dur_ppq: Some(480),
        dur_real: None,
        dur_recip: None,
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AttDurationGes = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

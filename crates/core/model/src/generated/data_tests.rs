//! Unit tests for generated MEI data types.
//!
//! Tests verify:
//! - Enum serialization/deserialization matches MEI attribute values
//! - Newtype wrappers (Display, FromStr, From) work correctly
//! - Pattern validation catches invalid values
//! - Union types serialize/deserialize correctly

use super::data::*;
use super::validation::{Validate, ValidationContext};

// ============================================================================
// Simple Enum Tests
// ============================================================================

#[test]
fn test_data_duration_cmn_serialization() {
    // Test round-trip for all duration values
    let durations = [
        (DataDurationCmn::Long, "long"),
        (DataDurationCmn::Breve, "breve"),
        (DataDurationCmn::N1, "1"),
        (DataDurationCmn::N2, "2"),
        (DataDurationCmn::N4, "4"),
        (DataDurationCmn::N8, "8"),
        (DataDurationCmn::N16, "16"),
        (DataDurationCmn::N32, "32"),
        (DataDurationCmn::N64, "64"),
        (DataDurationCmn::N128, "128"),
        (DataDurationCmn::N256, "256"),
        (DataDurationCmn::N512, "512"),
        (DataDurationCmn::N1024, "1024"),
        (DataDurationCmn::N2048, "2048"),
    ];

    for (variant, expected_str) in durations {
        // Serialize
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, format!("\"{}\"", expected_str));

        // Deserialize
        let deserialized: DataDurationCmn =
            serde_json::from_str(&format!("\"{}\"", expected_str)).unwrap();
        assert_eq!(deserialized, variant);
    }
}

#[test]
fn test_data_clefshape_serialization() {
    let shapes = [
        (DataClefshape::G, "G"),
        (DataClefshape::Gg, "GG"),
        (DataClefshape::F, "F"),
        (DataClefshape::C, "C"),
        (DataClefshape::Perc, "perc"),
        (DataClefshape::Tab, "TAB"),
    ];

    for (variant, expected_str) in shapes {
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, format!("\"{}\"", expected_str));

        let deserialized: DataClefshape =
            serde_json::from_str(&format!("\"{}\"", expected_str)).unwrap();
        assert_eq!(deserialized, variant);
    }
}

#[test]
fn test_data_metersign_values() {
    assert_eq!(
        serde_json::to_string(&DataMetersign::Common).unwrap(),
        "\"common\""
    );
    assert_eq!(
        serde_json::to_string(&DataMetersign::Cut).unwrap(),
        "\"cut\""
    );
    assert_eq!(
        serde_json::to_string(&DataMetersign::Open).unwrap(),
        "\"open\""
    );
}

#[test]
fn test_data_grace_values() {
    let values = [
        (DataGrace::Acc, "acc"),
        (DataGrace::Unacc, "unacc"),
        (DataGrace::Unknown, "unknown"),
    ];

    for (variant, expected_str) in values {
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, format!("\"{}\"", expected_str));
    }
}

#[test]
fn test_data_harppedalposition_values() {
    assert_eq!(
        serde_json::to_string(&DataHarppedalposition::F).unwrap(),
        "\"f\""
    );
    assert_eq!(
        serde_json::to_string(&DataHarppedalposition::N).unwrap(),
        "\"n\""
    );
    assert_eq!(
        serde_json::to_string(&DataHarppedalposition::S).unwrap(),
        "\"s\""
    );
}

#[test]
fn test_data_fill_values() {
    let fills = [
        (DataFill::Void, "void"),
        (DataFill::Solid, "solid"),
        (DataFill::Top, "top"),
        (DataFill::Bottom, "bottom"),
        (DataFill::Left, "left"),
        (DataFill::Right, "right"),
    ];

    for (variant, expected_str) in fills {
        let serialized = serde_json::to_string(&variant).unwrap();
        assert_eq!(serialized, format!("\"{}\"", expected_str));
    }
}

// ============================================================================
// Newtype Wrapper Tests
// ============================================================================

#[test]
fn test_data_clefline_creation_and_display() {
    let line = DataClefline::from(3u64);
    assert_eq!(line.0, 3);
    assert_eq!(line.to_string(), "3");

    // FromStr
    let parsed: DataClefline = "4".parse().unwrap();
    assert_eq!(parsed.0, 4);
}

#[test]
fn test_data_pitchname_creation_and_validation() {
    let pitch = DataPitchname::from("c".to_string());
    assert_eq!(pitch.0, "c");
    assert_eq!(pitch.to_string(), "c");

    // Valid pitch names should pass validation
    let mut ctx = ValidationContext::new();
    for name in ["a", "b", "c", "d", "e", "f", "g"] {
        let p = DataPitchname::from(name.to_string());
        p.validate_with_context(&mut ctx);
    }
    assert!(
        !ctx.has_errors(),
        "Valid pitch names should not produce validation errors"
    );

    // Invalid pitch should fail validation
    let mut ctx = ValidationContext::new();
    let invalid = DataPitchname::from("h".to_string());
    invalid.validate_with_context(&mut ctx);
    assert!(ctx.has_errors(), "Invalid pitch 'h' should fail validation");
}

#[test]
fn test_data_octave_dis_validation() {
    // Valid values: 8, 15, 22
    let mut ctx = ValidationContext::new();
    for val in [8u64, 15, 22] {
        let octave = DataOctaveDis::from(val);
        octave.validate_with_context(&mut ctx);
    }
    assert!(!ctx.has_errors(), "Valid octave displacements should pass");

    // Invalid value
    let mut ctx = ValidationContext::new();
    let invalid = DataOctaveDis::from(7u64);
    invalid.validate_with_context(&mut ctx);
    assert!(
        ctx.has_errors(),
        "Invalid octave displacement 7 should fail"
    );
}

#[test]
fn test_data_tstampoffset_creation() {
    let offset = DataTstampoffset::from(1.5f64);
    assert_eq!(offset.0, 1.5);
    assert_eq!(offset.to_string(), "1.5");

    // FromStr
    let parsed: DataTstampoffset = "2.25".parse().unwrap();
    assert_eq!(parsed.0, 2.25);

    // Negative offsets
    let negative: DataTstampoffset = "-0.5".parse().unwrap();
    assert_eq!(negative.0, -0.5);
}

#[test]
fn test_data_interval_melodic_creation() {
    let interval = DataIntervalMelodic::from("u3".to_string());
    assert_eq!(interval.0, "u3");
    assert_eq!(interval.to_string(), "u3");

    let parsed: DataIntervalMelodic = "d5".parse().unwrap();
    assert_eq!(parsed.0, "d5");
}

// ============================================================================
// Enum Trait Derivations
// ============================================================================

#[test]
fn test_enum_debug_clone_eq() {
    // Test Debug
    let dur = DataDurationCmn::N4;
    let debug_str = format!("{:?}", dur);
    assert!(debug_str.contains("N4"));

    // Test Clone
    let cloned = dur.clone();
    assert_eq!(cloned, dur);

    // Test PartialEq / Eq
    assert_eq!(DataDurationCmn::N4, DataDurationCmn::N4);
    assert_ne!(DataDurationCmn::N4, DataDurationCmn::N8);

    // Test Hash (compiles = works)
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(DataDurationCmn::N4);
    set.insert(DataDurationCmn::N8);
    assert_eq!(set.len(), 2);
}

#[test]
fn test_newtype_debug_clone_eq() {
    let pitch1 = DataPitchname::from("c".to_string());
    let pitch2 = DataPitchname::from("c".to_string());
    let pitch3 = DataPitchname::from("d".to_string());

    // Debug
    let debug_str = format!("{:?}", pitch1);
    assert!(debug_str.contains("DataPitchname"));

    // Clone
    let cloned = pitch1.clone();
    assert_eq!(cloned, pitch1);

    // PartialEq / Eq
    assert_eq!(pitch1, pitch2);
    assert_ne!(pitch1, pitch3);

    // Hash
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(pitch1);
    set.insert(pitch2); // duplicate, shouldn't increase size
    assert_eq!(set.len(), 1);
}

// ============================================================================
// Serde Transparent Tests
// ============================================================================

#[test]
fn test_transparent_serde() {
    // Newtype wrappers should serialize as their inner value
    let line = DataClefline::from(2u64);
    let json = serde_json::to_string(&line).unwrap();
    assert_eq!(json, "2");

    let deserialized: DataClefline = serde_json::from_str("3").unwrap();
    assert_eq!(deserialized.0, 3);

    // String wrappers
    let pitch = DataPitchname::from("g".to_string());
    let json = serde_json::to_string(&pitch).unwrap();
    assert_eq!(json, "\"g\"");

    let deserialized: DataPitchname = serde_json::from_str("\"f\"").unwrap();
    assert_eq!(deserialized.0, "f");
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_invalid_enum_deserialization() {
    // Unknown duration value should fail
    let result: Result<DataDurationCmn, _> = serde_json::from_str("\"invalid\"");
    assert!(result.is_err());

    let result: Result<DataClefshape, _> = serde_json::from_str("\"X\"");
    assert!(result.is_err());
}

#[test]
fn test_invalid_numeric_parse() {
    let result: Result<DataClefline, _> = "abc".parse();
    assert!(result.is_err());

    let result: Result<DataTstampoffset, _> = "not_a_number".parse();
    assert!(result.is_err());
}

// ============================================================================
// Float Display Formatting Tests
// ============================================================================

#[test]
fn test_data_beat_display_formatting() {
    // Whole numbers should display without decimal point
    let beat1 = DataBeat::from(1.0);
    assert_eq!(beat1.to_string(), "1");

    let beat4 = DataBeat::from(4.0);
    assert_eq!(beat4.to_string(), "4");

    let beat100 = DataBeat::from(100.0);
    assert_eq!(beat100.to_string(), "100");

    // Fractional numbers should display with decimal point
    let beat_half = DataBeat::from(1.5);
    assert_eq!(beat_half.to_string(), "1.5");

    let beat_quarter = DataBeat::from(2.25);
    assert_eq!(beat_quarter.to_string(), "2.25");

    let beat_third = DataBeat::from(1.333);
    assert_eq!(beat_third.to_string(), "1.333");

    // Zero should display as "0"
    let beat_zero = DataBeat::from(0.0);
    assert_eq!(beat_zero.to_string(), "0");

    // Negative whole numbers
    let beat_neg = DataBeat::from(-2.0);
    assert_eq!(beat_neg.to_string(), "-2");

    // Negative fractional numbers
    let beat_neg_frac = DataBeat::from(-1.5);
    assert_eq!(beat_neg_frac.to_string(), "-1.5");
}

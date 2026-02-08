//! Utility functions for MEI to MusicXML conversion.
//!
//! Duration, pitch, and ID helper functions used across conversion modules.

/// Convert MEI duration string (e.g. "4", "quarter") to quarter note units.
pub fn duration_str_to_quarter_notes(s: &str) -> f64 {
    match s.trim().to_lowercase().as_str() {
        "long" | "0" => 16.0,
        "breve" => 8.0,
        "whole" | "1" => 4.0,
        "half" | "2" => 2.0,
        "quarter" | "4" => 1.0,
        "eighth" | "8" => 0.5,
        "16th" | "16" => 0.25,
        "32nd" | "32" => 0.125,
        "64th" | "64" => 0.0625,
        "128th" | "128" => 0.03125,
        "256th" | "256" => 0.015625,
        "512th" | "512" => 0.0078125,
        "1024th" | "1024" => 0.00390625,
        "2048th" | "2048" => 0.001953125,
        _ => 1.0,
    }
}

/// Convert MEI duration to quarter note units.
pub fn duration_to_quarter_notes(dur: &tusk_model::data::DataDuration) -> f64 {
    use tusk_model::data::{DataDuration, DataDurationCmn};

    match dur {
        DataDuration::MeiDataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => 16.0,    // Long = 4 whole notes
            DataDurationCmn::Breve => 8.0,    // Breve = 2 whole notes
            DataDurationCmn::N1 => 4.0,       // Whole = 4 quarters
            DataDurationCmn::N2 => 2.0,       // Half = 2 quarters
            DataDurationCmn::N4 => 1.0,       // Quarter
            DataDurationCmn::N8 => 0.5,       // Eighth
            DataDurationCmn::N16 => 0.25,     // 16th
            DataDurationCmn::N32 => 0.125,    // 32nd
            DataDurationCmn::N64 => 0.0625,   // 64th
            DataDurationCmn::N128 => 0.03125, // 128th
            DataDurationCmn::N256 => 0.015625,
            DataDurationCmn::N512 => 0.0078125,
            DataDurationCmn::N1024 => 0.00390625,
            DataDurationCmn::N2048 => 0.001953125, // 2048th note
        },
        // For mensural durations, return quarter note as fallback
        _ => 1.0,
    }
}

/// Convert MEI rest duration string to quarter note units.
pub fn duration_rests_str_to_quarter_notes(s: &str) -> f64 {
    duration_str_to_quarter_notes(s)
}

/// Convert MEI rest duration (DataDurationrests) to quarter note units.
pub fn duration_rests_to_quarter_notes(dur: &tusk_model::data::DataDurationrests) -> f64 {
    use tusk_model::data::{DataDurationCmn, DataDurationrests};

    match dur {
        DataDurationrests::MeiDataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => 16.0,    // Long = 4 whole notes
            DataDurationCmn::Breve => 8.0,    // Breve = 2 whole notes
            DataDurationCmn::N1 => 4.0,       // Whole = 4 quarters
            DataDurationCmn::N2 => 2.0,       // Half = 2 quarters
            DataDurationCmn::N4 => 1.0,       // Quarter
            DataDurationCmn::N8 => 0.5,       // Eighth
            DataDurationCmn::N16 => 0.25,     // 16th
            DataDurationCmn::N32 => 0.125,    // 32nd
            DataDurationCmn::N64 => 0.0625,   // 64th
            DataDurationCmn::N128 => 0.03125, // 128th
            DataDurationCmn::N256 => 0.015625,
            DataDurationCmn::N512 => 0.0078125,
            DataDurationCmn::N1024 => 0.00390625,
            DataDurationCmn::N2048 => 0.001953125,
        },
        // For mensural rest durations, return quarter note as fallback
        DataDurationrests::MeiDataDurationrestsMensural(_) => 1.0,
    }
}

/// Apply augmentation dots to a duration.
pub fn apply_dots(base_duration: f64, dots: u64) -> f64 {
    let mut duration = base_duration;
    let mut dot_value = base_duration / 2.0;
    for _ in 0..dots {
        duration += dot_value;
        dot_value /= 2.0;
    }
    duration
}

/// Convert MEI duration string to MusicXML NoteTypeValue.
pub fn convert_mei_duration_str_to_note_type(s: &str) -> crate::model::note::NoteTypeValue {
    use crate::model::note::NoteTypeValue;
    match s.trim().to_lowercase().as_str() {
        "long" | "0" => NoteTypeValue::Long,
        "breve" => NoteTypeValue::Breve,
        "whole" | "1" => NoteTypeValue::Whole,
        "half" | "2" => NoteTypeValue::Half,
        "quarter" | "4" => NoteTypeValue::Quarter,
        "eighth" | "8" => NoteTypeValue::Eighth,
        "16th" | "16" => NoteTypeValue::N16th,
        "32nd" | "32" => NoteTypeValue::N32nd,
        "64th" | "64" => NoteTypeValue::N64th,
        "128th" | "128" => NoteTypeValue::N128th,
        "256th" | "256" => NoteTypeValue::N256th,
        "512th" | "512" => NoteTypeValue::N512th,
        "1024th" | "1024" => NoteTypeValue::N1024th,
        "2048th" | "2048" => NoteTypeValue::N1024th,
        _ => NoteTypeValue::Quarter,
    }
}

/// Convert MEI duration to MusicXML NoteTypeValue.
pub fn convert_mei_duration_to_note_type(
    dur: &tusk_model::data::DataDuration,
) -> crate::model::note::NoteTypeValue {
    use crate::model::note::NoteTypeValue;
    use tusk_model::data::{DataDuration, DataDurationCmn};

    match dur {
        DataDuration::MeiDataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => NoteTypeValue::Long,
            DataDurationCmn::Breve => NoteTypeValue::Breve,
            DataDurationCmn::N1 => NoteTypeValue::Whole,
            DataDurationCmn::N2 => NoteTypeValue::Half,
            DataDurationCmn::N4 => NoteTypeValue::Quarter,
            DataDurationCmn::N8 => NoteTypeValue::Eighth,
            DataDurationCmn::N16 => NoteTypeValue::N16th,
            DataDurationCmn::N32 => NoteTypeValue::N32nd,
            DataDurationCmn::N64 => NoteTypeValue::N64th,
            DataDurationCmn::N128 => NoteTypeValue::N128th,
            DataDurationCmn::N256 => NoteTypeValue::N256th,
            DataDurationCmn::N512 => NoteTypeValue::N512th,
            DataDurationCmn::N1024 => NoteTypeValue::N1024th,
            DataDurationCmn::N2048 => NoteTypeValue::N1024th, // MusicXML doesn't have 2048th, use 1024th
        },
        // For mensural durations, return quarter as fallback
        _ => NoteTypeValue::Quarter,
    }
}

/// Convert MEI rest duration (DataDurationrests) to MusicXML NoteTypeValue.
pub fn convert_mei_duration_rests_to_note_type(
    dur: &tusk_model::data::DataDurationrests,
) -> Option<crate::model::note::NoteTypeValue> {
    use crate::model::note::NoteTypeValue;
    use tusk_model::data::{DataDurationCmn, DataDurationrests};

    match dur {
        DataDurationrests::MeiDataDurationCmn(cmn) => {
            let value = match cmn {
                DataDurationCmn::Long => NoteTypeValue::Long,
                DataDurationCmn::Breve => NoteTypeValue::Breve,
                DataDurationCmn::N1 => NoteTypeValue::Whole,
                DataDurationCmn::N2 => NoteTypeValue::Half,
                DataDurationCmn::N4 => NoteTypeValue::Quarter,
                DataDurationCmn::N8 => NoteTypeValue::Eighth,
                DataDurationCmn::N16 => NoteTypeValue::N16th,
                DataDurationCmn::N32 => NoteTypeValue::N32nd,
                DataDurationCmn::N64 => NoteTypeValue::N64th,
                DataDurationCmn::N128 => NoteTypeValue::N128th,
                DataDurationCmn::N256 => NoteTypeValue::N256th,
                DataDurationCmn::N512 => NoteTypeValue::N512th,
                DataDurationCmn::N1024 => NoteTypeValue::N1024th,
                DataDurationCmn::N2048 => NoteTypeValue::N1024th, // MusicXML doesn't have 2048th
            };
            Some(value)
        }
        // Mensural rest durations have no direct MusicXML equivalent
        DataDurationrests::MeiDataDurationrestsMensural(_) => None,
    }
}

/// Convert MEI @mm.unit string (e.g. "quarter", "4") to MusicXML beat unit string.
pub fn mei_mm_unit_str_to_beat_unit(s: &str) -> String {
    match s.trim().to_lowercase().as_str() {
        "long" | "0" => "long".to_string(),
        "breve" => "breve".to_string(),
        "whole" | "1" => "whole".to_string(),
        "half" | "2" => "half".to_string(),
        "quarter" | "4" => "quarter".to_string(),
        "eighth" | "8" => "eighth".to_string(),
        "16th" | "16" => "16th".to_string(),
        "32nd" | "32" => "32nd".to_string(),
        "64th" | "64" => "64th".to_string(),
        "128th" | "128" => "128th".to_string(),
        "256th" | "256" => "256th".to_string(),
        "512th" | "512" => "512th".to_string(),
        "1024th" | "1024" => "1024th".to_string(),
        "2048th" | "2048" => "2048th".to_string(),
        _ => "quarter".to_string(),
    }
}

/// Convert MEI duration to MusicXML beat unit string.
pub fn convert_mei_duration_to_beat_unit(dur: &tusk_model::data::DataDuration) -> String {
    use tusk_model::data::{DataDuration, DataDurationCmn};

    match dur {
        DataDuration::MeiDataDurationCmn(cmn) => match cmn {
            DataDurationCmn::Long => "long".to_string(),
            DataDurationCmn::Breve => "breve".to_string(),
            DataDurationCmn::N1 => "whole".to_string(),
            DataDurationCmn::N2 => "half".to_string(),
            DataDurationCmn::N4 => "quarter".to_string(),
            DataDurationCmn::N8 => "eighth".to_string(),
            DataDurationCmn::N16 => "16th".to_string(),
            DataDurationCmn::N32 => "32nd".to_string(),
            DataDurationCmn::N64 => "64th".to_string(),
            DataDurationCmn::N128 => "128th".to_string(),
            DataDurationCmn::N256 => "256th".to_string(),
            DataDurationCmn::N512 => "512th".to_string(),
            DataDurationCmn::N1024 => "1024th".to_string(),
            DataDurationCmn::N2048 => "2048th".to_string(),
        },
        _ => "quarter".to_string(), // Default for non-CMN durations
    }
}

/// Convert MEI stem direction string to MusicXML StemValue.
pub fn convert_mei_stem_direction_str(s: &str) -> crate::model::note::StemValue {
    use crate::model::note::StemValue;
    match s.trim().to_lowercase().as_str() {
        "down" => StemValue::Down,
        _ => StemValue::Up,
    }
}

/// Convert MEI stem direction to MusicXML StemValue.
pub fn convert_mei_stem_direction(
    stem_dir: &tusk_model::data::DataStemdirection,
) -> crate::model::note::StemValue {
    use crate::model::note::StemValue;
    use tusk_model::data::{DataStemdirection, DataStemdirectionBasic};

    match stem_dir {
        DataStemdirection::MeiDataStemdirectionBasic(basic) => match basic {
            DataStemdirectionBasic::Up => StemValue::Up,
            DataStemdirectionBasic::Down => StemValue::Down,
        },
        // For extended directions (left, right, ne, nw, se, sw), default to up
        DataStemdirection::MeiDataStemdirectionExtended(_) => StemValue::Up,
    }
}

/// Parse an MEI measurement value (e.g., "200vu", "100", "50.5vu") to f64.
///
/// MEI measurements can include units like "vu" (virtual units), "pt" (points),
/// etc. This function extracts the numeric value, discarding the unit suffix.
/// Parse MEI measurement string (e.g. "200vu", "100") to f64.
pub fn parse_mei_measurement_str(s: &str) -> Option<f64> {
    let s = s.trim();
    if let Ok(val) = s.parse::<f64>() {
        return Some(val);
    }
    let numeric_part: String = s
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    numeric_part.parse::<f64>().ok()
}

pub fn parse_mei_measurement(
    measurement: &tusk_model::data::DataMeasurementunsigned,
) -> Option<f64> {
    let s = measurement.to_string();

    // Try to parse as a simple number first
    if let Ok(val) = s.parse::<f64>() {
        return Some(val);
    }

    // Try to extract numeric prefix (handle "200vu", "100pt", etc.)
    let numeric_part: String = s
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    numeric_part.parse::<f64>().ok()
}

/// Extract label text from staffGrp children.
pub fn extract_label_text(staff_grp: &tusk_model::elements::StaffGrp) -> Option<String> {
    use tusk_model::elements::{LabelChild, StaffGrpChild};

    for child in &staff_grp.children {
        if let StaffGrpChild::Label(label) = child {
            let mut text = String::new();
            for label_child in &label.children {
                if let LabelChild::Text(t) = label_child {
                    text.push_str(t);
                }
            }
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Extract labelAbbr text from staffGrp children.
pub fn extract_label_abbr_text(staff_grp: &tusk_model::elements::StaffGrp) -> Option<String> {
    use tusk_model::elements::{LabelAbbrChild, StaffGrpChild};

    for child in &staff_grp.children {
        if let StaffGrpChild::LabelAbbr(label_abbr) = child {
            let mut text = String::new();
            for label_child in &label_abbr.children {
                if let LabelAbbrChild::Text(t) = label_child {
                    text.push_str(t);
                }
            }
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Extract label text from staffDef children.
pub fn extract_staff_def_label(staff_def: &tusk_model::elements::StaffDef) -> Option<String> {
    use tusk_model::elements::{LabelChild, StaffDefChild};

    for child in &staff_def.children {
        if let StaffDefChild::Label(label) = child {
            let mut text = String::new();
            for label_child in &label.children {
                if let LabelChild::Text(t) = label_child {
                    text.push_str(t);
                }
            }
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Extract labelAbbr text from staffDef children.
pub fn extract_staff_def_label_abbr(staff_def: &tusk_model::elements::StaffDef) -> Option<String> {
    use tusk_model::elements::{LabelAbbrChild, StaffDefChild};

    for child in &staff_def.children {
        if let StaffDefChild::LabelAbbr(label_abbr) = child {
            let mut text = String::new();
            for label_child in &label_abbr.children {
                if let LabelAbbrChild::Text(t) = label_child {
                    text.push_str(t);
                }
            }
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    None
}

/// Extract title text from MEI fileDesc.
pub fn extract_title_from_file_desc(file_desc: &tusk_model::elements::FileDesc) -> Option<String> {
    use tusk_model::elements::{FileDescChild, TitleChild, TitleStmtChild};

    for child in &file_desc.children {
        if let FileDescChild::TitleStmt(title_stmt) = child {
            for ts_child in &title_stmt.children {
                if let TitleStmtChild::Title(title) = ts_child {
                    // Collect text content from title children
                    let mut text = String::new();
                    for title_child in &title.children {
                        if let TitleChild::Text(t) = title_child {
                            text.push_str(t);
                        }
                    }
                    if !text.is_empty() {
                        return Some(text);
                    }
                }
            }
        }
    }
    None
}

/// Find the Body element in Music.
pub fn find_body_in_music(
    music: &tusk_model::elements::Music,
) -> Option<&tusk_model::elements::Body> {
    use tusk_model::elements::MusicChild;

    for child in &music.children {
        if let MusicChild::Body(body) = child {
            return Some(body);
        }
    }
    None
}

/// Find the first Mdiv in a Body.
pub fn find_first_mdiv_in_body(
    body: &tusk_model::elements::Body,
) -> Option<&tusk_model::elements::Mdiv> {
    use tusk_model::elements::BodyChild;

    for child in &body.children {
        if let BodyChild::Mdiv(mdiv) = child {
            return Some(mdiv);
        }
    }
    None
}

/// Find the Score element in an Mdiv.
pub fn find_score_in_mdiv(
    mdiv: &tusk_model::elements::Mdiv,
) -> Option<&tusk_model::elements::Score> {
    use tusk_model::elements::MdivChild;

    for child in &mdiv.children {
        if let MdivChild::Score(score) = child {
            return Some(score);
        }
    }
    None
}

/// Find the ScoreDef in a Score.
pub fn find_score_def(
    score: &tusk_model::elements::Score,
) -> Option<&tusk_model::elements::ScoreDef> {
    use tusk_model::elements::ScoreChild;

    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            return Some(score_def);
        }
    }
    None
}

/// Create empty Part elements matching the part-list.
#[cfg(test)]
pub fn create_empty_parts(
    part_list: &crate::model::elements::PartList,
) -> Vec<crate::model::elements::Part> {
    use crate::model::elements::{Part, PartListItem};

    let mut parts = Vec::new();

    for item in &part_list.items {
        if let PartListItem::ScorePart(score_part) = item {
            parts.push(Part::new(&score_part.id));
        }
    }

    parts
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // Duration to Quarter Notes Tests
    // ========================================================================

    #[test]
    fn test_duration_to_quarter_notes_all_cmn_values() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        let test_cases = [
            (DataDurationCmn::Long, 16.0),
            (DataDurationCmn::Breve, 8.0),
            (DataDurationCmn::N1, 4.0),
            (DataDurationCmn::N2, 2.0),
            (DataDurationCmn::N4, 1.0),
            (DataDurationCmn::N8, 0.5),
            (DataDurationCmn::N16, 0.25),
            (DataDurationCmn::N32, 0.125),
            (DataDurationCmn::N64, 0.0625),
            (DataDurationCmn::N128, 0.03125),
            (DataDurationCmn::N256, 0.015625),
            (DataDurationCmn::N512, 0.0078125),
            (DataDurationCmn::N1024, 0.00390625),
            (DataDurationCmn::N2048, 0.001953125),
        ];

        for (cmn_dur, expected) in test_cases {
            let dur = DataDuration::MeiDataDurationCmn(cmn_dur);
            let result = duration_to_quarter_notes(&dur);
            assert!(
                (result - expected).abs() < 1e-10,
                "Duration {:?} expected {} but got {}",
                cmn_dur,
                expected,
                result
            );
        }
    }

    #[test]
    fn test_duration_rests_to_quarter_notes_all_cmn_values() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};

        let test_cases = [
            (DataDurationCmn::Long, 16.0),
            (DataDurationCmn::Breve, 8.0),
            (DataDurationCmn::N1, 4.0),
            (DataDurationCmn::N2, 2.0),
            (DataDurationCmn::N4, 1.0),
            (DataDurationCmn::N8, 0.5),
            (DataDurationCmn::N16, 0.25),
            (DataDurationCmn::N32, 0.125),
        ];

        for (cmn_dur, expected) in test_cases {
            let dur = DataDurationrests::MeiDataDurationCmn(cmn_dur);
            let result = duration_rests_to_quarter_notes(&dur);
            assert!(
                (result - expected).abs() < 1e-10,
                "Rest duration {:?} expected {} but got {}",
                cmn_dur,
                expected,
                result
            );
        }
    }

    // ========================================================================
    // Apply Dots Tests
    // ========================================================================

    #[test]
    fn test_apply_dots_zero_dots() {
        // Quarter note with no dots = 1.0
        assert!((apply_dots(1.0, 0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_apply_dots_single_dot() {
        // Dotted quarter = 1.0 + 0.5 = 1.5
        assert!((apply_dots(1.0, 1) - 1.5).abs() < 1e-10);

        // Dotted half = 2.0 + 1.0 = 3.0
        assert!((apply_dots(2.0, 1) - 3.0).abs() < 1e-10);

        // Dotted whole = 4.0 + 2.0 = 6.0
        assert!((apply_dots(4.0, 1) - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_apply_dots_double_dot() {
        // Double-dotted quarter = 1.0 + 0.5 + 0.25 = 1.75
        assert!((apply_dots(1.0, 2) - 1.75).abs() < 1e-10);

        // Double-dotted half = 2.0 + 1.0 + 0.5 = 3.5
        assert!((apply_dots(2.0, 2) - 3.5).abs() < 1e-10);
    }

    #[test]
    fn test_apply_dots_triple_dot() {
        // Triple-dotted quarter = 1.0 + 0.5 + 0.25 + 0.125 = 1.875
        assert!((apply_dots(1.0, 3) - 1.875).abs() < 1e-10);
    }

    // ========================================================================
    // Duration to Note Type Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_duration_to_note_type_all_values() {
        use crate::model::note::NoteTypeValue;
        use tusk_model::data::{DataDuration, DataDurationCmn};

        let test_cases = [
            (DataDurationCmn::Long, NoteTypeValue::Long),
            (DataDurationCmn::Breve, NoteTypeValue::Breve),
            (DataDurationCmn::N1, NoteTypeValue::Whole),
            (DataDurationCmn::N2, NoteTypeValue::Half),
            (DataDurationCmn::N4, NoteTypeValue::Quarter),
            (DataDurationCmn::N8, NoteTypeValue::Eighth),
            (DataDurationCmn::N16, NoteTypeValue::N16th),
            (DataDurationCmn::N32, NoteTypeValue::N32nd),
            (DataDurationCmn::N64, NoteTypeValue::N64th),
            (DataDurationCmn::N128, NoteTypeValue::N128th),
            (DataDurationCmn::N256, NoteTypeValue::N256th),
            (DataDurationCmn::N512, NoteTypeValue::N512th),
            (DataDurationCmn::N1024, NoteTypeValue::N1024th),
            // 2048th maps to 1024th since MusicXML doesn't support 2048th
            (DataDurationCmn::N2048, NoteTypeValue::N1024th),
        ];

        for (cmn_dur, expected) in test_cases {
            let dur = DataDuration::MeiDataDurationCmn(cmn_dur);
            let result = convert_mei_duration_to_note_type(&dur);
            assert_eq!(
                result, expected,
                "Duration {:?} expected {:?} but got {:?}",
                cmn_dur, expected, result
            );
        }
    }

    #[test]
    fn test_convert_mei_duration_rests_to_note_type_all_values() {
        use crate::model::note::NoteTypeValue;
        use tusk_model::data::{DataDurationCmn, DataDurationrests};

        let test_cases = [
            (DataDurationCmn::Long, Some(NoteTypeValue::Long)),
            (DataDurationCmn::Breve, Some(NoteTypeValue::Breve)),
            (DataDurationCmn::N1, Some(NoteTypeValue::Whole)),
            (DataDurationCmn::N2, Some(NoteTypeValue::Half)),
            (DataDurationCmn::N4, Some(NoteTypeValue::Quarter)),
            (DataDurationCmn::N8, Some(NoteTypeValue::Eighth)),
            (DataDurationCmn::N16, Some(NoteTypeValue::N16th)),
            (DataDurationCmn::N32, Some(NoteTypeValue::N32nd)),
        ];

        for (cmn_dur, expected) in test_cases {
            let dur = DataDurationrests::MeiDataDurationCmn(cmn_dur);
            let result = convert_mei_duration_rests_to_note_type(&dur);
            assert_eq!(
                result, expected,
                "Rest duration {:?} expected {:?} but got {:?}",
                cmn_dur, expected, result
            );
        }
    }

    // ========================================================================
    // Duration to Beat Unit Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_duration_to_beat_unit() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        let test_cases = [
            (DataDurationCmn::Long, "long"),
            (DataDurationCmn::Breve, "breve"),
            (DataDurationCmn::N1, "whole"),
            (DataDurationCmn::N2, "half"),
            (DataDurationCmn::N4, "quarter"),
            (DataDurationCmn::N8, "eighth"),
            (DataDurationCmn::N16, "16th"),
            (DataDurationCmn::N32, "32nd"),
            (DataDurationCmn::N64, "64th"),
            (DataDurationCmn::N128, "128th"),
            (DataDurationCmn::N256, "256th"),
            (DataDurationCmn::N512, "512th"),
            (DataDurationCmn::N1024, "1024th"),
            (DataDurationCmn::N2048, "2048th"),
        ];

        for (cmn_dur, expected) in test_cases {
            let dur = DataDuration::MeiDataDurationCmn(cmn_dur);
            let result = convert_mei_duration_to_beat_unit(&dur);
            assert_eq!(
                result, expected,
                "Duration {:?} expected {} but got {}",
                cmn_dur, expected, result
            );
        }
    }

    // ========================================================================
    // Stem Direction Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_stem_direction_up() {
        use crate::model::note::StemValue;
        use tusk_model::data::{DataStemdirection, DataStemdirectionBasic};

        let stem_dir = DataStemdirection::MeiDataStemdirectionBasic(DataStemdirectionBasic::Up);
        let result = convert_mei_stem_direction(&stem_dir);
        assert_eq!(result, StemValue::Up);
    }

    #[test]
    fn test_convert_mei_stem_direction_down() {
        use crate::model::note::StemValue;
        use tusk_model::data::{DataStemdirection, DataStemdirectionBasic};

        let stem_dir = DataStemdirection::MeiDataStemdirectionBasic(DataStemdirectionBasic::Down);
        let result = convert_mei_stem_direction(&stem_dir);
        assert_eq!(result, StemValue::Down);
    }

    // ========================================================================
    // Duration Division Calculation Tests
    // ========================================================================

    #[test]
    fn test_duration_with_divisions_quarter_note() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        // Quarter note with divisions=4 should be 4 divisions
        let dur = DataDuration::MeiDataDurationCmn(DataDurationCmn::N4);
        let quarters = duration_to_quarter_notes(&dur);
        let divisions = 4.0;
        let result = quarters * divisions;
        assert!((result - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_duration_with_divisions_half_note() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        // Half note with divisions=4 should be 8 divisions
        let dur = DataDuration::MeiDataDurationCmn(DataDurationCmn::N2);
        let quarters = duration_to_quarter_notes(&dur);
        let divisions = 4.0;
        let result = quarters * divisions;
        assert!((result - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_duration_with_divisions_eighth_note() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        // Eighth note with divisions=4 should be 2 divisions
        let dur = DataDuration::MeiDataDurationCmn(DataDurationCmn::N8);
        let quarters = duration_to_quarter_notes(&dur);
        let divisions = 4.0;
        let result = quarters * divisions;
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_duration_with_divisions_whole_note() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        // Whole note with divisions=4 should be 16 divisions
        let dur = DataDuration::MeiDataDurationCmn(DataDurationCmn::N1);
        let quarters = duration_to_quarter_notes(&dur);
        let divisions = 4.0;
        let result = quarters * divisions;
        assert!((result - 16.0).abs() < 1e-10);
    }

    #[test]
    fn test_duration_with_divisions_dotted_quarter() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        // Dotted quarter with divisions=4 should be 6 divisions
        let dur = DataDuration::MeiDataDurationCmn(DataDurationCmn::N4);
        let quarters = duration_to_quarter_notes(&dur);
        let dotted = apply_dots(quarters, 1);
        let divisions = 4.0;
        let result = dotted * divisions;
        assert!((result - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_duration_high_precision_divisions() {
        use tusk_model::data::{DataDuration, DataDurationCmn};

        // Test with divisions=96 (common high precision)
        let dur = DataDuration::MeiDataDurationCmn(DataDurationCmn::N4);
        let quarters = duration_to_quarter_notes(&dur);
        let divisions = 96.0;
        let result = quarters * divisions;
        assert!((result - 96.0).abs() < 1e-10);

        // Eighth note with divisions=96 = 48
        let dur_eighth = DataDuration::MeiDataDurationCmn(DataDurationCmn::N8);
        let quarters_eighth = duration_to_quarter_notes(&dur_eighth);
        let result_eighth = quarters_eighth * divisions;
        assert!((result_eighth - 48.0).abs() < 1e-10);

        // Sixteenth note with divisions=96 = 24
        let dur_16th = DataDuration::MeiDataDurationCmn(DataDurationCmn::N16);
        let quarters_16th = duration_to_quarter_notes(&dur_16th);
        let result_16th = quarters_16th * divisions;
        assert!((result_16th - 24.0).abs() < 1e-10);
    }

    // ========================================================================
    // Create Empty Parts Tests
    // ========================================================================

    #[test]
    fn test_create_empty_parts_single_part() {
        use crate::model::elements::{PartList, PartListItem, ScorePart};

        let mut part_list = PartList::default();
        let score_part = ScorePart::new("P1", "Piano");
        part_list
            .items
            .push(PartListItem::ScorePart(Box::new(score_part)));

        let parts = create_empty_parts(&part_list);
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].id, "P1");
    }

    #[test]
    fn test_create_empty_parts_multiple_parts() {
        use crate::model::elements::{PartList, PartListItem, ScorePart};

        let mut part_list = PartList::default();
        for (id, name) in [("P1", "Violin I"), ("P2", "Violin II"), ("P3", "Cello")] {
            let score_part = ScorePart::new(id, name);
            part_list
                .items
                .push(PartListItem::ScorePart(Box::new(score_part)));
        }

        let parts = create_empty_parts(&part_list);
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0].id, "P1");
        assert_eq!(parts[1].id, "P2");
        assert_eq!(parts[2].id, "P3");
    }

    #[test]
    fn test_create_empty_parts_empty_list() {
        use crate::model::elements::PartList;

        let part_list = PartList::default();
        let parts = create_empty_parts(&part_list);
        assert!(parts.is_empty());
    }
}

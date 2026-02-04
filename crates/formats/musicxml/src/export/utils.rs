//! Utility functions for MEI to MusicXML conversion.
//!
//! Duration, pitch, and ID helper functions used across conversion modules.

/// Convert MEI duration to quarter note units.
pub fn duration_to_quarter_notes(dur: &tusk_model::data::DataDuration) -> f64 {
    use tusk_model::data::{DataDuration, DataDurationCmn};

    match dur {
        DataDuration::DataDurationCmn(cmn) => match cmn {
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

/// Convert MEI rest duration (DataDurationrests) to quarter note units.
pub fn duration_rests_to_quarter_notes(dur: &tusk_model::data::DataDurationrests) -> f64 {
    use tusk_model::data::{DataDurationCmn, DataDurationrests};

    match dur {
        DataDurationrests::DataDurationCmn(cmn) => match cmn {
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
        DataDurationrests::DataDurationrestsMensural(_) => 1.0,
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

/// Convert MEI duration to MusicXML NoteTypeValue.
pub fn convert_mei_duration_to_note_type(
    dur: &tusk_model::data::DataDuration,
) -> crate::model::note::NoteTypeValue {
    use crate::model::note::NoteTypeValue;
    use tusk_model::data::{DataDuration, DataDurationCmn};

    match dur {
        DataDuration::DataDurationCmn(cmn) => match cmn {
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
        DataDurationrests::DataDurationCmn(cmn) => {
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
        DataDurationrests::DataDurationrestsMensural(_) => None,
    }
}

/// Convert MEI duration to MusicXML beat unit string.
pub fn convert_mei_duration_to_beat_unit(dur: &tusk_model::data::DataDuration) -> String {
    use tusk_model::data::{DataDuration, DataDurationCmn};

    match dur {
        DataDuration::DataDurationCmn(cmn) => match cmn {
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

/// Convert MEI stem direction to MusicXML StemValue.
pub fn convert_mei_stem_direction(
    stem_dir: &tusk_model::data::DataStemdirection,
) -> crate::model::note::StemValue {
    use crate::model::note::StemValue;
    use tusk_model::data::{DataStemdirection, DataStemdirectionBasic};

    match stem_dir {
        DataStemdirection::DataStemdirectionBasic(basic) => match basic {
            DataStemdirectionBasic::Up => StemValue::Up,
            DataStemdirectionBasic::Down => StemValue::Down,
        },
        // For extended directions (left, right, ne, nw, se, sw), default to up
        DataStemdirection::DataStemdirectionExtended(_) => StemValue::Up,
    }
}

/// Parse an MEI measurement value (e.g., "200vu", "100", "50.5vu") to f64.
///
/// MEI measurements can include units like "vu" (virtual units), "pt" (points),
/// etc. This function extracts the numeric value, discarding the unit suffix.
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
///
/// Note: Due to code generation limitations, Body might not be directly
/// accessible as a child of Music. This function handles that case.
pub fn find_body_in_music(
    _music: &tusk_model::elements::Music,
) -> Option<&tusk_model::elements::Body> {
    // The generated Music type doesn't include Body as a direct child variant.
    // In actual MEI documents, body is a child of music, but the code generator
    // only included certain children. For now, return None and handle this
    // limitation - the full document structure will need special handling.
    //
    // TODO: Update code generator to include body as a Music child, or use
    // a separate parsing path for the complete document structure.
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

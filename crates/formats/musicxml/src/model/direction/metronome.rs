//! MusicXML 4.0 metronome types.

use serde::{Deserialize, Serialize};

use crate::model::data::{LeftCenterRight, Valign, YesNo};

/// Metronome/tempo marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metronome {
    /// Metronome content (beat-unit based or metric modulation)
    #[serde(flatten)]
    pub content: MetronomeContent,

    /// Whether to display parentheses
    #[serde(rename = "@parentheses", skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<YesNo>,

    /// Whether to print
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

    /// Text justification
    #[serde(rename = "@justify", skip_serializing_if = "Option::is_none")]
    pub justify: Option<LeftCenterRight>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<LeftCenterRight>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<Valign>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Content of a metronome marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetronomeContent {
    /// Standard beat-unit = per-minute format
    BeatUnit {
        /// The beat unit (e.g., "quarter", "eighth")
        #[serde(rename = "beat-unit")]
        beat_unit: String,
        /// Dots on the beat unit
        #[serde(
            rename = "beat-unit-dot",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        beat_unit_dots: Vec<()>,
        /// Per-minute value (number or text)
        #[serde(rename = "per-minute")]
        per_minute: String,
    },
    /// Beat-unit = beat-unit format (metric modulation)
    BeatUnitEquivalent(MetricModulation),
}

/// Metric modulation (beat-unit = beat-unit).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricModulation {
    /// The first beat unit
    #[serde(rename = "beat-unit")]
    pub beat_unit_1: String,
    /// Dots on the first beat unit
    #[serde(
        rename = "beat-unit-dot",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub beat_unit_dots_1: Vec<()>,
    /// The second beat unit
    #[serde(rename = "beat-unit-2")]
    pub beat_unit_2: String,
    /// Dots on the second beat unit
    #[serde(
        rename = "beat-unit-dot-2",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub beat_unit_dots_2: Vec<()>,
}

impl Metronome {
    /// Create a simple metronome marking (e.g., quarter = 120).
    pub fn simple(beat_unit: impl Into<String>, per_minute: u32) -> Self {
        Self {
            content: MetronomeContent::BeatUnit {
                beat_unit: beat_unit.into(),
                beat_unit_dots: Vec::new(),
                per_minute: per_minute.to_string(),
            },
            parentheses: None,
            print_object: None,
            justify: None,
            default_x: None,
            default_y: None,
            halign: None,
            valign: None,
            id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metronome_simple() {
        let metronome = Metronome::simple("quarter", 120);
        if let MetronomeContent::BeatUnit {
            beat_unit,
            per_minute,
            ..
        } = &metronome.content
        {
            assert_eq!(beat_unit, "quarter");
            assert_eq!(per_minute, "120");
        } else {
            panic!("Expected BeatUnit content");
        }
    }

    #[test]
    fn test_metronome_with_parentheses() {
        let mut metronome = Metronome::simple("half", 60);
        metronome.parentheses = Some(YesNo::Yes);
        assert_eq!(metronome.parentheses, Some(YesNo::Yes));
    }
}

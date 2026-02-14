//! MusicXML 4.1 metronome types.

use serde::{Deserialize, Serialize};

use crate::musicxml::data::{LeftCenterRight, StartStop, Valign, YesNo};

/// Metronome/tempo marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Metronome {
    /// Metronome content (beat-unit based, metric modulation, or metronome-note)
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
///
/// MusicXML metronome has two main forms:
/// 1. Beat-unit form: `beat-unit [beat-unit-dot*] [beat-unit-tied*] (per-minute | beat-unit2)`
/// 2. Metronome-note form: `[metronome-arrows] metronome-note+ [metronome-relation metronome-note+]`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MetronomeContent {
    /// Standard beat-unit = per-minute format (e.g., quarter = 120)
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
        /// Tied beat units after the main beat unit
        #[serde(
            rename = "beat-unit-tied",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        beat_unit_tied: Vec<BeatUnitTied>,
        /// Per-minute value (number or text like "132-144", "c. 108")
        #[serde(rename = "per-minute")]
        per_minute: String,
    },
    /// Beat-unit = beat-unit format (metric modulation)
    BeatUnitEquivalent(MetricModulation),
    /// Metronome-note format for complex metric relationships (e.g., swing)
    MetronomeNotes(MetronomeNoteContent),
}

/// A tied beat unit in a metronome marking.
///
/// Represents `<beat-unit-tied>` which contains a beat-unit group.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatUnitTied {
    /// The beat unit (e.g., "quarter", "eighth")
    #[serde(rename = "beat-unit")]
    pub beat_unit: String,
    /// Dots on the beat unit
    #[serde(
        rename = "beat-unit-dot",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub beat_unit_dots: Vec<()>,
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
    /// Tied beat units after the first beat unit
    #[serde(
        rename = "beat-unit-tied-1",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub beat_unit_tied_1: Vec<BeatUnitTied>,
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
    /// Tied beat units after the second beat unit
    #[serde(
        rename = "beat-unit-tied-2",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub beat_unit_tied_2: Vec<BeatUnitTied>,
}

/// Content for metronome-note based markings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetronomeNoteContent {
    /// Whether arrows are displayed
    #[serde(
        rename = "metronome-arrows",
        default,
        skip_serializing_if = "std::ops::Not::not"
    )]
    pub arrows: bool,
    /// First group of metronome notes
    #[serde(rename = "metronome-note")]
    pub notes_1: Vec<MetronomeNote>,
    /// Relationship symbol between note groups (usually "equals")
    #[serde(rename = "metronome-relation", skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    /// Second group of metronome notes (present when relation is set)
    #[serde(
        rename = "metronome-note-2",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub notes_2: Vec<MetronomeNote>,
}

/// A note within a metronome-note marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetronomeNote {
    /// Note type (e.g., "quarter", "eighth")
    #[serde(rename = "metronome-type")]
    pub note_type: String,
    /// Number of dots
    #[serde(
        rename = "metronome-dot",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub dots: Vec<()>,
    /// Beams on this note
    #[serde(
        rename = "metronome-beam",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub beams: Vec<MetronomeBeam>,
    /// Tied status
    #[serde(rename = "metronome-tied", skip_serializing_if = "Option::is_none")]
    pub tied: Option<MetronomeTied>,
    /// Tuplet grouping
    #[serde(rename = "metronome-tuplet", skip_serializing_if = "Option::is_none")]
    pub tuplet: Option<MetronomeTuplet>,
}

/// A beam within a metronome note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetronomeBeam {
    /// Beam number (1-based)
    #[serde(rename = "@number")]
    pub number: u32,
    /// Beam value (begin, continue, end, etc.)
    #[serde(rename = "$text")]
    pub value: String,
}

/// Tied status in a metronome note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetronomeTied {
    /// Start or stop
    #[serde(rename = "@type")]
    pub tied_type: StartStop,
}

/// Tuplet grouping in a metronome note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetronomeTuplet {
    /// Start or stop
    #[serde(rename = "@type")]
    pub tuplet_type: StartStop,
    /// Whether to show bracket
    #[serde(rename = "@bracket", skip_serializing_if = "Option::is_none")]
    pub bracket: Option<YesNo>,
    /// How to display the number
    #[serde(rename = "@show-number", skip_serializing_if = "Option::is_none")]
    pub show_number: Option<String>,
    /// Actual number of notes
    #[serde(rename = "actual-notes", skip_serializing_if = "Option::is_none")]
    pub actual_notes: Option<u32>,
    /// Normal number of notes
    #[serde(rename = "normal-notes", skip_serializing_if = "Option::is_none")]
    pub normal_notes: Option<u32>,
    /// Normal note type
    #[serde(rename = "normal-type", skip_serializing_if = "Option::is_none")]
    pub normal_type: Option<String>,
    /// Normal dots (for dotted normal types)
    #[serde(rename = "normal-dot", default, skip_serializing_if = "Vec::is_empty")]
    pub normal_dots: Vec<()>,
}

impl Metronome {
    /// Create a simple metronome marking (e.g., quarter = 120).
    pub fn simple(beat_unit: impl Into<String>, per_minute: u32) -> Self {
        Self {
            content: MetronomeContent::BeatUnit {
                beat_unit: beat_unit.into(),
                beat_unit_dots: Vec::new(),
                beat_unit_tied: Vec::new(),
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

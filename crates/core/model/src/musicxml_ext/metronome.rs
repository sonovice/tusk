//! Typed metronome data for lossless MusicXML roundtrip.

use serde::{Deserialize, Serialize};

/// Typed metronome data for lossless roundtrip of MusicXML `<metronome>`.
///
/// Captures the full metronome structure including beat-unit-tied, metric
/// modulation, and metronome-note forms that MEI cannot represent natively.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MetronomeData {
    /// Metronome content variant.
    pub content: MetronomeContentData,

    /// Whether to display parentheses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<bool>,

    /// Print-object control.
    #[serde(rename = "po", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<bool>,

    /// Text justification (left/center/right).
    #[serde(rename = "jst", skip_serializing_if = "Option::is_none")]
    pub justify: Option<String>,

    /// Default X position.
    #[serde(rename = "dx", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position.
    #[serde(rename = "dy", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Horizontal alignment.
    #[serde(rename = "ha", skip_serializing_if = "Option::is_none")]
    pub halign: Option<String>,

    /// Vertical alignment.
    #[serde(rename = "va", skip_serializing_if = "Option::is_none")]
    pub valign: Option<String>,

    /// Optional unique ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Content of a metronome marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetronomeContentData {
    /// Standard beat-unit = per-minute format (e.g., quarter = 120).
    BeatUnit {
        /// The beat unit (e.g., "quarter", "eighth").
        unit: String,
        /// Number of dots on the beat unit.
        #[serde(default, skip_serializing_if = "is_zero")]
        dots: u32,
        /// Tied beat units after the main beat unit.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        tied: Vec<BeatUnitTiedData>,
        /// Per-minute value (supports non-numeric like "132-144", "c. 108").
        pm: String,
    },
    /// Beat-unit = beat-unit format (metric modulation).
    Modulation(MetricModulationData),
    /// Metronome-note format for complex metric relationships.
    Notes(MetronomeNotesData),
}

impl Default for MetronomeContentData {
    fn default() -> Self {
        Self::BeatUnit {
            unit: String::new(),
            dots: 0,
            tied: Vec::new(),
            pm: String::new(),
        }
    }
}

/// A tied beat unit in a metronome marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeatUnitTiedData {
    /// The beat unit (e.g., "quarter", "eighth").
    pub unit: String,
    /// Number of dots on the beat unit.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub dots: u32,
}

/// Metric modulation (beat-unit = beat-unit).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetricModulationData {
    /// First beat unit.
    pub unit1: String,
    /// Dots on first beat unit.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub dots1: u32,
    /// Tied beat units after the first beat unit.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tied1: Vec<BeatUnitTiedData>,
    /// Second beat unit.
    pub unit2: String,
    /// Dots on second beat unit.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub dots2: u32,
    /// Tied beat units after the second beat unit.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tied2: Vec<BeatUnitTiedData>,
}

/// Metronome-note content for complex metric relationships.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetronomeNotesData {
    /// Whether arrows are displayed.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub arrows: bool,
    /// First group of metronome notes.
    pub notes1: Vec<MetronomeNoteData>,
    /// Relationship symbol between note groups (usually "equals").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    /// Second group of metronome notes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes2: Vec<MetronomeNoteData>,
}

/// A note within a metronome-note marking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetronomeNoteData {
    /// Note type (e.g., "quarter", "eighth").
    #[serde(rename = "ty")]
    pub note_type: String,
    /// Number of dots.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub dots: u32,
    /// Beams on this note.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beams: Vec<MetronomeBeamData>,
    /// Tied status (start/stop).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tied: Option<String>,
    /// Tuplet grouping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuplet: Option<MetronomeTupletData>,
}

/// A beam within a metronome note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MetronomeBeamData {
    /// Beam number (1-based).
    pub number: u32,
    /// Beam value (begin, continue, end, etc.).
    pub value: String,
}

/// Tuplet grouping in a metronome note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct MetronomeTupletData {
    /// Start or stop.
    #[serde(rename = "ty")]
    pub tuplet_type: String,
    /// Whether to show bracket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bracket: Option<bool>,
    /// How to display the number.
    #[serde(rename = "sn", skip_serializing_if = "Option::is_none")]
    pub show_number: Option<String>,
    /// Actual number of notes.
    #[serde(rename = "an", skip_serializing_if = "Option::is_none")]
    pub actual_notes: Option<u32>,
    /// Normal number of notes.
    #[serde(rename = "nn", skip_serializing_if = "Option::is_none")]
    pub normal_notes: Option<u32>,
    /// Normal note type.
    #[serde(rename = "nt", skip_serializing_if = "Option::is_none")]
    pub normal_type: Option<String>,
    /// Normal dots count.
    #[serde(rename = "nd", default, skip_serializing_if = "is_zero")]
    pub normal_dots: u32,
}

fn is_zero(v: &u32) -> bool {
    *v == 0
}

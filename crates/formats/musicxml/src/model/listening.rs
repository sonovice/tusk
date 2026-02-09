//! MusicXML 4.1 listening, listen, and grouping element types.
//!
//! - `<listening>` appears as a standalone measure-level element or as a child
//!   of `<direction>`. Contains `<sync>` and `<other-listening>` children.
//! - `<listen>` appears as a child of `<note>`. Contains `<assess>`, `<wait>`,
//!   and `<other-listen>` children.
//! - `<grouping>` is a standalone measure-level element for analytical grouping
//!   of musical content with `<feature>` children.
//!
//! `<link>` and `<bookmark>` are defined in `elements/score.rs` (shared with credits).

use serde::{Deserialize, Serialize};

use super::data::YesNo;

// ============================================================================
// Listening (direction-level and measure-level)
// ============================================================================

/// The `<listening>` element — used for interactive music performance synchronization.
///
/// Can appear as a child of `<direction>` or as a standalone element in `music-data`.
/// Contains sync/other-listening children and an optional offset.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Listening {
    /// Sync and other-listening children (at least one required per XSD)
    pub children: Vec<ListeningChild>,

    /// Optional offset from current position in divisions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<crate::model::direction::Offset>,
}

/// A child of the `<listening>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListeningChild {
    Sync(Sync),
    OtherListening(OtherListening),
}

/// The `<sync>` element — specifies the style of synchronization for interactive performance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sync {
    /// Required type attribute (none, tempo, mostly-tempo, mostly-event, event, always-event)
    #[serde(rename = "@type")]
    pub sync_type: String,

    /// Optional latency in milliseconds
    #[serde(rename = "@latency", skip_serializing_if = "Option::is_none")]
    pub latency: Option<u32>,

    /// Optional player reference
    #[serde(rename = "@player", skip_serializing_if = "Option::is_none")]
    pub player: Option<String>,

    /// Optional time-only attribute (comma-separated list of time-only numbers)
    #[serde(rename = "@time-only", skip_serializing_if = "Option::is_none")]
    pub time_only: Option<String>,
}

impl Default for Sync {
    fn default() -> Self {
        Self {
            sync_type: "none".to_string(),
            latency: None,
            player: None,
            time_only: None,
        }
    }
}

/// The `<other-listening>` element — for listening types not covered by sync.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct OtherListening {
    /// Required type attribute
    #[serde(rename = "@type")]
    pub other_type: String,

    /// Optional player reference
    #[serde(rename = "@player", skip_serializing_if = "Option::is_none")]
    pub player: Option<String>,

    /// Optional time-only attribute
    #[serde(rename = "@time-only", skip_serializing_if = "Option::is_none")]
    pub time_only: Option<String>,

    /// Text content
    #[serde(rename = "$value", default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

impl Default for OtherListening {
    fn default() -> Self {
        Self {
            other_type: String::new(),
            player: None,
            time_only: None,
            value: String::new(),
        }
    }
}

// ============================================================================
// Listen (note-level)
// ============================================================================

/// The `<listen>` element — specifies listening behavior for a note during interactive performance.
///
/// Appears as a child of `<note>`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Listen {
    /// Assess, wait, and other-listen children
    pub children: Vec<ListenChild>,
}

/// A child of the `<listen>` element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ListenChild {
    Assess(Assess),
    Wait(Wait),
    OtherListen(OtherListening),
}

/// The `<assess>` element — indicates whether a note should be assessed for intonation/timing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Assess {
    /// Required type attribute (yes/no)
    #[serde(rename = "@type")]
    pub assess_type: YesNo,

    /// Optional player reference
    #[serde(rename = "@player", skip_serializing_if = "Option::is_none")]
    pub player: Option<String>,

    /// Optional time-only attribute
    #[serde(rename = "@time-only", skip_serializing_if = "Option::is_none")]
    pub time_only: Option<String>,
}

impl Default for Assess {
    fn default() -> Self {
        Self {
            assess_type: YesNo::Yes,
            player: None,
            time_only: None,
        }
    }
}

/// The `<wait>` element — indicates to wait for a performer to begin the next note.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Wait {
    /// Optional player reference
    #[serde(rename = "@player", skip_serializing_if = "Option::is_none")]
    pub player: Option<String>,

    /// Optional time-only attribute
    #[serde(rename = "@time-only", skip_serializing_if = "Option::is_none")]
    pub time_only: Option<String>,
}

// ============================================================================
// Grouping
// ============================================================================

/// The `<grouping>` element — groups musical content for analysis purposes.
///
/// Appears as a standalone measure-level element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Grouping {
    /// Required type attribute (start/stop/single)
    #[serde(rename = "@type")]
    pub grouping_type: String,

    /// Optional number (default "1")
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Optional member-of reference
    #[serde(rename = "@member-of", skip_serializing_if = "Option::is_none")]
    pub member_of: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Feature children
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<Feature>,
}

impl Default for Grouping {
    fn default() -> Self {
        Self {
            grouping_type: "start".to_string(),
            number: None,
            member_of: None,
            id: None,
            features: Vec::new(),
        }
    }
}

/// The `<feature>` element — describes a musical feature for grouping analysis.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Feature {
    /// Optional type attribute
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,

    /// Text content
    #[serde(rename = "$value", default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

//! Measure types for MusicXML documents.
//!
//! This module contains the Measure type and its content enum.

use serde::{Deserialize, Serialize};

use crate::musicxml::data::YesNo;

use super::barline::Barline;
use super::score::{Bookmark, Link};
use crate::musicxml::direction::Sound;
use crate::musicxml::figured_bass::FiguredBass;
use crate::musicxml::harmony::Harmony;
use crate::musicxml::listening::{Grouping, Listening};
use crate::musicxml::print::Print;

/// A measure in a part.
///
/// Contains music data (notes, rests, directions, etc.).
/// Content will be expanded in Phase 4.2.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measure {
    /// Measure number
    #[serde(rename = "@number")]
    pub number: String,

    /// Displayed measure number (when different from @number)
    #[serde(rename = "@text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Implicit measure (pickup, etc.)
    #[serde(rename = "@implicit", skip_serializing_if = "Option::is_none")]
    pub implicit: Option<YesNo>,

    /// Non-controlling measure for multi-rest
    #[serde(rename = "@non-controlling", skip_serializing_if = "Option::is_none")]
    pub non_controlling: Option<YesNo>,

    /// Width of measure
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,

    /// Optional ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Measure content (will be expanded in Phase 4.2)
    #[serde(rename = "$value", default)]
    pub content: Vec<MeasureContent>,
}

impl Measure {
    /// Create a new measure with the given number.
    pub fn new(number: &str) -> Self {
        Self {
            number: number.to_string(),
            text: None,
            implicit: None,
            non_controlling: None,
            width: None,
            id: None,
            content: Vec::new(),
        }
    }
}

/// Measure content - notes, rests, directions, and other music data.
///
/// Represents the various elements that can appear within a measure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeasureContent {
    /// A note or rest.
    Note(Box<crate::musicxml::note::Note>),
    /// Backup - moves the cursor backward in time.
    Backup(Box<crate::musicxml::note::Backup>),
    /// Forward - moves the cursor forward in time.
    Forward(Box<crate::musicxml::note::Forward>),
    /// Attributes (key, time, clef, divisions, etc.).
    Attributes(Box<crate::musicxml::attributes::Attributes>),
    /// Direction (dynamics, tempo, pedals, wedges, etc.).
    Direction(Box<crate::musicxml::direction::Direction>),
    /// Harmony (chord symbols, Roman numerals, fretboard diagrams).
    Harmony(Box<Harmony>),
    /// Figured bass notation.
    FiguredBass(Box<FiguredBass>),
    /// Print (page/system breaks, layout overrides).
    Print(Box<Print>),
    /// Standalone sound (playback parameters not attached to a direction).
    Sound(Box<Sound>),
    /// Listening (interactive performance synchronization).
    Listening(Box<Listening>),
    /// Barline (location and style).
    Barline(Box<Barline>),
    /// Grouping (analytical grouping of musical content).
    Grouping(Box<Grouping>),
    /// Link (XLink reference to external resource).
    Link(Box<Link>),
    /// Bookmark (navigation point within the score).
    Bookmark(Box<Bookmark>),
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure() {
        let measure = Measure {
            number: "1".to_string(),
            text: None,
            implicit: Some(YesNo::Yes),
            non_controlling: None,
            width: Some(200.0),
            id: Some("m1".to_string()),
            content: Vec::new(),
        };

        assert_eq!(measure.number, "1");
        assert_eq!(measure.implicit, Some(YesNo::Yes));
        assert_eq!(measure.width, Some(200.0));
        assert_eq!(measure.id.as_deref(), Some("m1"));
    }

    #[test]
    fn test_measure_text_attribute() {
        let measure = Measure {
            number: "5".to_string(),
            text: Some("5a".to_string()),
            implicit: None,
            non_controlling: None,
            width: None,
            id: None,
            content: Vec::new(),
        };

        assert_eq!(measure.text.as_deref(), Some("5a"));

        // Serde roundtrip
        let json = serde_json::to_string(&measure).unwrap();
        let deserialized: Measure = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.text, measure.text);
        assert_eq!(deserialized.number, measure.number);
    }

    #[test]
    fn test_measure_text_none_not_serialized() {
        let measure = Measure::new("1");
        let json = serde_json::to_string(&measure).unwrap();
        assert!(!json.contains("@text"));
    }
}

//! Barline types for MusicXML documents.
//!
//! This module contains barline, repeat, and ending types.

use serde::{Deserialize, Serialize};

use super::super::data::{StartStopDiscontinue, YesNo};
use super::super::direction::{Coda, Segno};
use super::super::notations::{Fermata, WavyLine};

/// Location of a barline within a measure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BarlineLocation {
    /// First element in the measure (aside from print, bookmark, link).
    Left,
    /// Last element in the measure (default).
    #[default]
    Right,
    /// Middle of the measure (e.g. dotted barline in complex meters).
    Middle,
}

impl BarlineLocation {
    /// MusicXML location attribute value.
    pub fn to_musicxml_str(&self) -> &'static str {
        match self {
            BarlineLocation::Left => "left",
            BarlineLocation::Right => "right",
            BarlineLocation::Middle => "middle",
        }
    }
}

/// Barline style (MusicXML bar-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BarStyle {
    #[serde(rename = "regular")]
    Regular,
    #[serde(rename = "dotted")]
    Dotted,
    #[serde(rename = "dashed")]
    Dashed,
    #[serde(rename = "heavy")]
    Heavy,
    #[serde(rename = "light-light")]
    LightLight,
    #[serde(rename = "light-heavy")]
    LightHeavy,
    #[serde(rename = "heavy-light")]
    HeavyLight,
    #[serde(rename = "heavy-heavy")]
    HeavyHeavy,
    #[serde(rename = "tick")]
    Tick,
    #[serde(rename = "short")]
    Short,
    #[serde(rename = "none")]
    None,
}

impl BarStyle {
    /// Parse MusicXML bar-style text content.
    pub fn from_musicxml_str(s: &str) -> Option<Self> {
        let s = s.trim().to_lowercase();
        Some(match s.as_str() {
            "regular" => BarStyle::Regular,
            "dotted" => BarStyle::Dotted,
            "dashed" => BarStyle::Dashed,
            "heavy" => BarStyle::Heavy,
            "light-light" => BarStyle::LightLight,
            "light-heavy" => BarStyle::LightHeavy,
            "heavy-light" => BarStyle::HeavyLight,
            "heavy-heavy" => BarStyle::HeavyHeavy,
            "tick" => BarStyle::Tick,
            "short" => BarStyle::Short,
            "none" => BarStyle::None,
            _ => return None,
        })
    }

    /// MusicXML bar-style element text value.
    pub fn to_musicxml_str(&self) -> &'static str {
        match self {
            BarStyle::Regular => "regular",
            BarStyle::Dotted => "dotted",
            BarStyle::Dashed => "dashed",
            BarStyle::Heavy => "heavy",
            BarStyle::LightLight => "light-light",
            BarStyle::LightHeavy => "light-heavy",
            BarStyle::HeavyLight => "heavy-light",
            BarStyle::HeavyHeavy => "heavy-heavy",
            BarStyle::Tick => "tick",
            BarStyle::Short => "short",
            BarStyle::None => "none",
        }
    }
}

/// Repeat direction (backward or forward).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BackwardForward {
    #[serde(rename = "backward")]
    Backward,
    #[serde(rename = "forward")]
    Forward,
}

/// Winged repeat extensions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Winged {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "straight")]
    Straight,
    #[serde(rename = "curved")]
    Curved,
    #[serde(rename = "double-straight")]
    DoubleStraight,
    #[serde(rename = "double-curved")]
    DoubleCurved,
}

/// Repeat marks within a barline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Repeat {
    /// Forward or backward direction (required).
    #[serde(rename = "@direction")]
    pub direction: BackwardForward,

    /// Number of times the repeated section is played.
    #[serde(rename = "@times", default, skip_serializing_if = "Option::is_none")]
    pub times: Option<u32>,

    /// Whether repeats are played after a jump (da capo/dal segno).
    #[serde(
        rename = "@after-jump",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub after_jump: Option<YesNo>,

    /// Winged extensions above/below the barline.
    #[serde(rename = "@winged", default, skip_serializing_if = "Option::is_none")]
    pub winged: Option<Winged>,
}

/// Ending (volta bracket) within a barline.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ending {
    /// Ending number(s), comma-separated (required). E.g. "1", "1,2", "1,2,3".
    #[serde(rename = "@number")]
    pub number: String,

    /// Start, stop, or discontinue (required).
    #[serde(rename = "@type")]
    pub ending_type: StartStopDiscontinue,

    /// Text content displayed on the volta bracket.
    #[serde(rename = "$value", default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Default Y position.
    #[serde(
        rename = "@default-y",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_y: Option<f64>,

    /// End length in tenths.
    #[serde(
        rename = "@end-length",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_length: Option<f64>,

    /// Print-object attribute.
    #[serde(
        rename = "@print-object",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub print_object: Option<YesNo>,

    /// Default X position.
    #[serde(
        rename = "@default-x",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_x: Option<f64>,

    /// Text X position offset.
    #[serde(rename = "@text-x", default, skip_serializing_if = "Option::is_none")]
    pub text_x: Option<f64>,

    /// Text Y position offset.
    #[serde(rename = "@text-y", default, skip_serializing_if = "Option::is_none")]
    pub text_y: Option<f64>,
}

/// Barline element with all XSD children.
///
/// XSD sequence: bar-style, editorial, wavy-line, segno, coda,
/// fermata (0-2), ending, repeat.
///
/// Also has barline-level attributes: @location, @segno, @coda, @divisions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Barline {
    /// Where the barline appears (left, right, middle). Default is right.
    #[serde(rename = "@location", skip_serializing_if = "Option::is_none")]
    pub location: Option<BarlineLocation>,

    /// Barline-level segno attribute (token for sound linking).
    #[serde(rename = "@segno", skip_serializing_if = "Option::is_none")]
    pub segno_attr: Option<String>,

    /// Barline-level coda attribute (token for sound linking).
    #[serde(rename = "@coda", skip_serializing_if = "Option::is_none")]
    pub coda_attr: Option<String>,

    /// Barline-level divisions attribute.
    #[serde(rename = "@divisions", skip_serializing_if = "Option::is_none")]
    pub divisions: Option<f64>,

    /// Barline style (optional child element).
    #[serde(rename = "bar-style", skip_serializing_if = "Option::is_none")]
    pub bar_style: Option<BarStyle>,

    /// Editorial footnote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footnote: Option<crate::model::note::FormattedText>,

    /// Editorial level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<crate::model::note::Level>,

    /// Wavy-line (trill continuation at barline).
    #[serde(rename = "wavy-line", skip_serializing_if = "Option::is_none")]
    pub wavy_line: Option<WavyLine>,

    /// Segno sign child element.
    #[serde(rename = "segno", skip_serializing_if = "Option::is_none")]
    pub segno: Option<Segno>,

    /// Coda sign child element.
    #[serde(rename = "coda", skip_serializing_if = "Option::is_none")]
    pub coda: Option<Coda>,

    /// Fermata markings (up to 2).
    #[serde(rename = "fermata", default, skip_serializing_if = "Vec::is_empty")]
    pub fermatas: Vec<Fermata>,

    /// Ending (volta bracket).
    #[serde(rename = "ending", skip_serializing_if = "Option::is_none")]
    pub ending: Option<Ending>,

    /// Repeat marks.
    #[serde(rename = "repeat", skip_serializing_if = "Option::is_none")]
    pub repeat: Option<Repeat>,
}

impl Barline {
    /// Whether this barline has any children beyond bar-style.
    pub fn has_extra_children(&self) -> bool {
        self.wavy_line.is_some()
            || self.segno.is_some()
            || self.coda.is_some()
            || !self.fermatas.is_empty()
            || self.ending.is_some()
            || self.repeat.is_some()
    }

    /// Whether this barline has any attributes beyond location.
    pub fn has_extra_attrs(&self) -> bool {
        self.segno_attr.is_some() || self.coda_attr.is_some() || self.divisions.is_some()
    }
}

/// Legacy placeholder for deserialization compatibility when barline has no content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BarlinePlaceholder;

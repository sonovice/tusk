//! Barline types for MusicXML documents.
//!
//! This module contains barline and ending types.

use serde::{Deserialize, Serialize};

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

/// Barline element (replaces placeholder).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Barline {
    /// Where the barline appears (left, right, middle). Default is right.
    #[serde(rename = "@location", skip_serializing_if = "Option::is_none")]
    pub location: Option<BarlineLocation>,

    /// Barline style (optional child element in MusicXML; we capture as optional field).
    #[serde(rename = "bar-style", skip_serializing_if = "Option::is_none")]
    pub bar_style: Option<BarStyle>,
}

impl Default for Barline {
    fn default() -> Self {
        Self {
            location: None,
            bar_style: None,
        }
    }
}

/// Legacy placeholder for deserialization compatibility when barline has no content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct BarlinePlaceholder;

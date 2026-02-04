//! MusicXML 4.0 wedge/hairpin types.

use serde::{Deserialize, Serialize};

use crate::model::data::{LineType, YesNo};

/// Wedge type for crescendo/diminuendo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WedgeType {
    /// Crescendo (closed at left, open at right)
    Crescendo,
    /// Diminuendo (open at left, closed at right)
    Diminuendo,
    /// Stop the wedge
    Stop,
    /// Continue across system break
    Continue,
}

/// Crescendo or diminuendo wedge.
///
/// Spread is measured in tenths of staff line space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wedge {
    /// Type of wedge (required)
    #[serde(rename = "@type")]
    pub wedge_type: WedgeType,

    /// Number level for distinguishing overlapping wedges
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<u8>,

    /// Spread in tenths
    #[serde(rename = "@spread", skip_serializing_if = "Option::is_none")]
    pub spread: Option<f64>,

    /// Circle at point indicating crescendo from nothing or diminuendo to nothing
    #[serde(rename = "@niente", skip_serializing_if = "Option::is_none")]
    pub niente: Option<YesNo>,

    /// Line type (solid, dashed, dotted, wavy)
    #[serde(rename = "@line-type", skip_serializing_if = "Option::is_none")]
    pub line_type: Option<LineType>,

    /// Dash length in tenths
    #[serde(rename = "@dash-length", skip_serializing_if = "Option::is_none")]
    pub dash_length: Option<f64>,

    /// Space length in tenths
    #[serde(rename = "@space-length", skip_serializing_if = "Option::is_none")]
    pub space_length: Option<f64>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Relative X position
    #[serde(rename = "@relative-x", skip_serializing_if = "Option::is_none")]
    pub relative_x: Option<f64>,

    /// Relative Y position
    #[serde(rename = "@relative-y", skip_serializing_if = "Option::is_none")]
    pub relative_y: Option<f64>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl Wedge {
    /// Create a new wedge of the given type.
    pub fn new(wedge_type: WedgeType) -> Self {
        Self {
            wedge_type,
            number: None,
            spread: None,
            niente: None,
            line_type: None,
            dash_length: None,
            space_length: None,
            default_x: None,
            default_y: None,
            relative_x: None,
            relative_y: None,
            color: None,
            id: None,
        }
    }

    /// Create a crescendo wedge.
    pub fn crescendo() -> Self {
        Self::new(WedgeType::Crescendo)
    }

    /// Create a diminuendo wedge.
    pub fn diminuendo() -> Self {
        Self::new(WedgeType::Diminuendo)
    }

    /// Create a stop wedge.
    pub fn stop() -> Self {
        Self::new(WedgeType::Stop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wedge_creation() {
        let wedge = Wedge::crescendo();
        assert_eq!(wedge.wedge_type, WedgeType::Crescendo);
        assert!(wedge.number.is_none());
        assert!(wedge.spread.is_none());
        assert!(wedge.niente.is_none());
    }

    #[test]
    fn test_wedge_diminuendo() {
        let wedge = Wedge::diminuendo();
        assert_eq!(wedge.wedge_type, WedgeType::Diminuendo);
    }

    #[test]
    fn test_wedge_stop() {
        let wedge = Wedge::stop();
        assert_eq!(wedge.wedge_type, WedgeType::Stop);
    }

    #[test]
    fn test_wedge_with_attributes() {
        let mut wedge = Wedge::crescendo();
        wedge.number = Some(1);
        wedge.spread = Some(15.0);
        wedge.niente = Some(YesNo::Yes);
        wedge.line_type = Some(LineType::Dashed);

        assert_eq!(wedge.number, Some(1));
        assert_eq!(wedge.spread, Some(15.0));
        assert_eq!(wedge.niente, Some(YesNo::Yes));
        assert_eq!(wedge.line_type, Some(LineType::Dashed));
    }
}

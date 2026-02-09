//! MusicXML 4.0 figured bass types.
//!
//! This module contains types for the `<figured-bass>` element and its children,
//! used for Baroque continuo notation.

use serde::{Deserialize, Serialize};

use super::data::{AboveBelow, StartStopContinue, YesNo};
use super::direction::Offset;
use super::harmony::StyleText;

// ============================================================================
// FiguredBass Element
// ============================================================================

/// A figured-bass element representing figured bass notation.
///
/// Figured bass elements take their position from the first regular note
/// that follows in score order. The optional duration element indicates
/// changes of figures under a note.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiguredBass {
    /// One or more figure elements (ordered top to bottom)
    pub figures: Vec<Figure>,

    /// Optional duration in divisions
    pub duration: Option<f64>,

    /// Optional offset from current position
    pub offset: Option<Offset>,

    /// Staff number
    pub staff: Option<u32>,

    /// Whether figures are in parentheses
    #[serde(rename = "@parentheses", skip_serializing_if = "Option::is_none")]
    pub parentheses: Option<YesNo>,

    /// Placement above or below
    #[serde(rename = "@placement", skip_serializing_if = "Option::is_none")]
    pub placement: Option<AboveBelow>,

    /// Whether to print this object
    #[serde(rename = "@print-object", skip_serializing_if = "Option::is_none")]
    pub print_object: Option<YesNo>,

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

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,

    /// Color
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<String>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

// ============================================================================
// Figure
// ============================================================================

/// A single figure within a figured-bass element.
///
/// Each figure can have an optional prefix (accidental before the number),
/// a figure-number, an optional suffix (accidental after or overstrike),
/// and an optional extend line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Figure {
    /// Prefix (accidental before the figure number).
    /// Values include plus, sharp, flat, natural, double-sharp, flat-flat, sharp-sharp.
    pub prefix: Option<StyleText>,

    /// The figure number.
    pub figure_number: Option<StyleText>,

    /// Suffix (accidental after or overstrike of the figure number).
    /// Values include plus, sharp, flat, natural, double-sharp, flat-flat, sharp-sharp,
    /// slash, back-slash, vertical.
    pub suffix: Option<StyleText>,

    /// Extend line for continuation.
    pub extend: Option<FigureExtend>,
}

// ============================================================================
// FigureExtend
// ============================================================================

/// An extend line for a figured bass figure.
///
/// Used to show continuation of a figure across multiple notes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FigureExtend {
    /// Type of extend (start/stop/continue).
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub extend_type: Option<StartStopContinue>,

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
}

//! MusicXML 4.0 print element types.
//!
//! The `<print>` element contains general printing parameters, including
//! layout elements. Layout group elements in a print element only apply to
//! the current page, system, or staff.

use serde::{Deserialize, Serialize};

use super::data::YesNo;
use super::elements::{NameDisplay, PageLayout, StaffLayout, SystemLayout};

/// A print element containing general printing parameters.
///
/// Controls page/system breaks, measure-level layout overrides,
/// and display changes for part names.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Print {
    // --- Layout group (same as defaults) ---
    /// Page layout override for this page
    pub page_layout: Option<PageLayout>,

    /// System layout override for this system
    pub system_layout: Option<SystemLayout>,

    /// Staff layout overrides (one per staff)
    pub staff_layouts: Vec<StaffLayout>,

    // --- Print-specific children ---
    /// Measure layout (horizontal distance from previous measure)
    pub measure_layout: Option<MeasureLayout>,

    /// Measure numbering display
    pub measure_numbering: Option<MeasureNumbering>,

    /// Part name display override
    pub part_name_display: Option<NameDisplay>,

    /// Part abbreviation display override
    pub part_abbreviation_display: Option<NameDisplay>,

    // --- Print attributes ---
    /// Spacing between staves (deprecated; use staff-layout instead)
    #[serde(rename = "@staff-spacing", skip_serializing_if = "Option::is_none")]
    pub staff_spacing: Option<f64>,

    /// Force a new system
    #[serde(rename = "@new-system", skip_serializing_if = "Option::is_none")]
    pub new_system: Option<YesNo>,

    /// Force a new page
    #[serde(rename = "@new-page", skip_serializing_if = "Option::is_none")]
    pub new_page: Option<YesNo>,

    /// Number of blank pages to insert before this measure
    #[serde(rename = "@blank-page", skip_serializing_if = "Option::is_none")]
    pub blank_page: Option<u32>,

    /// New page number
    #[serde(rename = "@page-number", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,

    /// Optional unique ID
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Measure layout: horizontal distance from the previous measure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasureLayout {
    /// Horizontal distance from the previous measure (in tenths)
    pub measure_distance: Option<f64>,
}

/// Measure numbering display settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeasureNumbering {
    /// Display value (none, measure, system)
    pub value: MeasureNumberingValue,

    /// System relation
    #[serde(rename = "@system", skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    /// Staff number reference
    #[serde(rename = "@staff", skip_serializing_if = "Option::is_none")]
    pub staff: Option<u32>,

    /// Show on multiple rests even mid-system
    #[serde(
        rename = "@multiple-rest-always",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiple_rest_always: Option<YesNo>,

    /// Show range on multiple rests
    #[serde(
        rename = "@multiple-rest-range",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiple_rest_range: Option<YesNo>,

    /// Default X position
    #[serde(rename = "@default-x", skip_serializing_if = "Option::is_none")]
    pub default_x: Option<f64>,

    /// Default Y position
    #[serde(rename = "@default-y", skip_serializing_if = "Option::is_none")]
    pub default_y: Option<f64>,

    /// Font family
    #[serde(rename = "@font-family", skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,

    /// Font size
    #[serde(rename = "@font-size", skip_serializing_if = "Option::is_none")]
    pub font_size: Option<f64>,

    /// Font style
    #[serde(rename = "@font-style", skip_serializing_if = "Option::is_none")]
    pub font_style: Option<String>,

    /// Font weight
    #[serde(rename = "@font-weight", skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,

    /// Horizontal alignment
    #[serde(rename = "@halign", skip_serializing_if = "Option::is_none")]
    pub halign: Option<String>,

    /// Vertical alignment
    #[serde(rename = "@valign", skip_serializing_if = "Option::is_none")]
    pub valign: Option<String>,
}

/// Measure numbering value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MeasureNumberingValue {
    None,
    Measure,
    System,
}

impl MeasureNumberingValue {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "none" => Some(MeasureNumberingValue::None),
            "measure" => Some(MeasureNumberingValue::Measure),
            "system" => Some(MeasureNumberingValue::System),
            _ => Option::None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            MeasureNumberingValue::None => "none",
            MeasureNumberingValue::Measure => "measure",
            MeasureNumberingValue::System => "system",
        }
    }
}

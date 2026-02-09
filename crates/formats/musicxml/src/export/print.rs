//! Print element export from MEI to MusicXML.
//!
//! Converts MEI `<sb>` and `<pb>` measure children back to MusicXML `<print>`
//! elements. When the MEI element carries a `musicxml:print,` label prefix,
//! the full Print struct is deserialized from the JSON payload for lossless
//! roundtrip. Otherwise, a minimal print element is created.

use crate::context::ConversionContext;
use crate::import::print::print_from_label;
use crate::model::data::YesNo;
use crate::model::elements::MeasureContent;
use crate::model::print::Print;
use tusk_model::elements::{Pb, Sb};

/// Convert an MEI `<sb>` element to a MusicXML `<print>` measure content.
pub fn convert_mei_sb(
    sb: &Sb,
    _ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    if let Some(label) = sb.common.label.as_deref() {
        if let Some(print) = print_from_label(label) {
            return Some(MeasureContent::Print(Box::new(print)));
        }
    }

    // Fallback: minimal print with new-system=yes
    let print = Print {
        page_layout: None,
        system_layout: None,
        staff_layouts: Vec::new(),
        measure_layout: None,
        measure_numbering: None,
        part_name_display: None,
        part_abbreviation_display: None,
        staff_spacing: None,
        new_system: Some(YesNo::Yes),
        new_page: None,
        blank_page: None,
        page_number: None,
        id: None,
    };
    Some(MeasureContent::Print(Box::new(print)))
}

/// Convert an MEI `<pb>` element to a MusicXML `<print>` measure content.
pub fn convert_mei_pb(
    pb: &Pb,
    _ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    if let Some(label) = pb.common.label.as_deref() {
        if let Some(print) = print_from_label(label) {
            return Some(MeasureContent::Print(Box::new(print)));
        }
    }

    // Fallback: minimal print with new-page=yes
    let print = Print {
        page_layout: None,
        system_layout: None,
        staff_layouts: Vec::new(),
        measure_layout: None,
        measure_numbering: None,
        part_name_display: None,
        part_abbreviation_display: None,
        staff_spacing: None,
        new_system: None,
        new_page: Some(YesNo::Yes),
        blank_page: None,
        page_number: None,
        id: None,
    };
    Some(MeasureContent::Print(Box::new(print)))
}

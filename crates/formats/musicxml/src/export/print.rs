//! Print element export from MEI to MusicXML.
//!
//! Converts MEI `<sb>` and `<pb>` measure children back to MusicXML `<print>`
//! elements. When the MEI element has PrintData in ExtensionStore, the full
//! Print struct is reconstructed from typed data. Otherwise, a minimal print
//! element is created.

use crate::context::ConversionContext;
use crate::model::data::YesNo;
use crate::model::elements::MeasureContent;
use crate::model::print::Print;
use tusk_model::elements::{Pb, Sb};
use tusk_model::musicxml_ext::PrintData;

/// Convert an MEI `<sb>` element to a MusicXML `<print>` measure content.
pub fn convert_mei_sb(sb: &Sb, ctx: &mut ConversionContext) -> Option<MeasureContent> {
    // Preferred: reconstruct from ExtensionStore typed data
    if let Some(id) = &sb.common.xml_id {
        if let Some(data) = ctx.ext_store().print(id) {
            let print = build_print_from_data(data);
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
pub fn convert_mei_pb(pb: &Pb, ctx: &mut ConversionContext) -> Option<MeasureContent> {
    // Preferred: reconstruct from ExtensionStore typed data
    if let Some(id) = &pb.common.xml_id {
        if let Some(data) = ctx.ext_store().print(id) {
            let print = build_print_from_data(data);
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

/// Build a MusicXML `Print` from typed `PrintData`.
fn build_print_from_data(data: &PrintData) -> Print {
    Print {
        page_layout: data.page_layout.clone(),
        system_layout: data.system_layout.clone(),
        staff_layouts: data.staff_layouts.clone(),
        measure_layout: data.measure_layout.clone(),
        measure_numbering: data.measure_numbering.clone(),
        part_name_display: data.part_name_display.clone(),
        part_abbreviation_display: data.part_abbreviation_display.clone(),
        staff_spacing: data.staff_spacing,
        new_system: data
            .new_system
            .map(|b| if b { YesNo::Yes } else { YesNo::No }),
        new_page: data
            .new_page
            .map(|b| if b { YesNo::Yes } else { YesNo::No }),
        blank_page: data.blank_page,
        page_number: data.page_number.clone(),
        id: data.id.clone(),
    }
}

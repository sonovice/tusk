//! Print element conversion from MusicXML to MEI.
//!
//! Converts MusicXML `<print>` elements to MEI `<sb>` (system break) and/or
//! `<pb>` (page break) measure children. Full MusicXML data is stored in
//! ExtensionStore for lossless roundtrip.

use crate::context::ConversionContext;
use crate::model::data::YesNo;
use crate::model::print::Print;
use tusk_model::elements::{MeasureChild, Pb, Sb};
use tusk_model::musicxml_ext::PrintData;

/// Convert a MusicXML `<print>` element to MEI `<sb>` and/or `<pb>` measure children.
///
/// - `new-system="yes"` → `<sb>` with data in ExtensionStore
/// - `new-page="yes"` → `<pb>` with data in ExtensionStore
/// - Both → `<pb>` only (page break implies system break)
/// - Neither → `<sb>` as carrier for layout-only print data
pub fn convert_print(print: &Print, ctx: &mut ConversionContext) -> Vec<MeasureChild> {
    let has_new_page = print.new_page == Some(YesNo::Yes);
    let has_new_system = print.new_system == Some(YesNo::Yes);

    let mut children = Vec::new();

    let print_ext = build_print_data(print);

    if has_new_page {
        let mut pb = Pb::default();
        pb.common.xml_id = Some(ctx.generate_id_with_suffix("pb"));
        if let Some(ref id) = pb.common.xml_id {
            ctx.ext_store_mut()
                .insert_print(id.clone(), print_ext.clone());
        }
        children.push(MeasureChild::Pb(Box::new(pb)));
    } else if has_new_system {
        let mut sb = Sb::default();
        sb.common.xml_id = Some(ctx.generate_id_with_suffix("sb"));
        if let Some(ref id) = sb.common.xml_id {
            ctx.ext_store_mut()
                .insert_print(id.clone(), print_ext.clone());
        }
        children.push(MeasureChild::Sb(Box::new(sb)));
    } else {
        let mut sb = Sb::default();
        sb.common.xml_id = Some(ctx.generate_id_with_suffix("sb"));
        if let Some(ref id) = sb.common.xml_id {
            ctx.ext_store_mut().insert_print(id.clone(), print_ext);
        }
        children.push(MeasureChild::Sb(Box::new(sb)));
    }

    children
}

fn build_print_data(print: &Print) -> PrintData {
    PrintData {
        new_system: print.new_system.map(|v| matches!(v, YesNo::Yes)),
        new_page: print.new_page.map(|v| matches!(v, YesNo::Yes)),
        blank_page: print.blank_page,
        page_number: print.page_number.clone(),
        staff_spacing: print.staff_spacing,
        page_layout: print.page_layout.clone(),
        system_layout: print.system_layout.clone(),
        staff_layouts: print.staff_layouts.clone(),
        measure_layout: print.measure_layout.clone(),
        measure_numbering: print.measure_numbering.clone(),
        part_name_display: print.part_name_display.clone(),
        part_abbreviation_display: print.part_abbreviation_display.clone(),
        id: print.id.clone(),
    }
}

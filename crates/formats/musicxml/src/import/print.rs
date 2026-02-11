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

/// Label marker for MEI sb/pb elements carrying MusicXML print data (via ExtensionStore).
pub const PRINT_LABEL_PREFIX: &str = "musicxml:print";

/// Convert a MusicXML `<print>` element to MEI `<sb>` and/or `<pb>` measure children.
///
/// - `new-system="yes"` → `<sb>` with marker label + data in ExtensionStore
/// - `new-page="yes"` → `<pb>` with marker label + data in ExtensionStore
/// - Both → `<pb>` only (page break implies system break)
/// - Neither → `<sb>` as carrier for layout-only print data
pub fn convert_print(print: &Print, ctx: &mut ConversionContext) -> Vec<MeasureChild> {
    let has_new_page = print.new_page == Some(YesNo::Yes);
    let has_new_system = print.new_system == Some(YesNo::Yes);

    let mut children = Vec::new();

    let print_ext = build_print_data(print);
    let mxml_json = serde_json::to_value(print).ok();

    if has_new_page {
        let mut pb = Pb::default();
        pb.common.xml_id = Some(ctx.generate_id_with_suffix("pb"));
        pb.common.label = Some(PRINT_LABEL_PREFIX.to_string());
        if let Some(ref id) = pb.common.xml_id {
            let entry = ctx.ext_store_mut().entry(id.clone());
            entry.print_data = Some(print_ext.clone());
            entry.mxml_json = mxml_json.clone();
        }
        children.push(MeasureChild::Pb(Box::new(pb)));
    } else if has_new_system {
        let mut sb = Sb::default();
        sb.common.xml_id = Some(ctx.generate_id_with_suffix("sb"));
        sb.common.label = Some(PRINT_LABEL_PREFIX.to_string());
        if let Some(ref id) = sb.common.xml_id {
            let entry = ctx.ext_store_mut().entry(id.clone());
            entry.print_data = Some(print_ext.clone());
            entry.mxml_json = mxml_json.clone();
        }
        children.push(MeasureChild::Sb(Box::new(sb)));
    } else {
        let mut sb = Sb::default();
        sb.common.xml_id = Some(ctx.generate_id_with_suffix("sb"));
        sb.common.label = Some(PRINT_LABEL_PREFIX.to_string());
        if let Some(ref id) = sb.common.xml_id {
            let entry = ctx.ext_store_mut().entry(id.clone());
            entry.print_data = Some(print_ext);
            entry.mxml_json = mxml_json;
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
        page_layout: print
            .page_layout
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok()),
        system_layout: print
            .system_layout
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok()),
        staff_layouts: print
            .staff_layouts
            .iter()
            .filter_map(|v| serde_json::to_value(v).ok())
            .collect(),
        measure_layout: print
            .measure_layout
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok()),
        measure_numbering: print
            .measure_numbering
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok()),
        part_name_display: print
            .part_name_display
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok()),
        part_abbreviation_display: print
            .part_abbreviation_display
            .as_ref()
            .and_then(|v| serde_json::to_value(v).ok()),
        id: print.id.clone(),
    }
}

/// Deserialize a Print from a legacy JSON roundtrip label.
pub fn print_from_label(label: &str) -> Option<Print> {
    if label == PRINT_LABEL_PREFIX {
        return None;
    }
    let json = label.strip_prefix("musicxml:print,")?;
    serde_json::from_str(json).ok()
}

//! Print element conversion from MusicXML to MEI.
//!
//! Converts MusicXML `<print>` elements to MEI `<sb>` (system break) and/or
//! `<pb>` (page break) measure children. The full MusicXML Print struct is
//! serialized as JSON in the `@label` attribute for lossless roundtrip.

use crate::context::ConversionContext;
use crate::model::data::YesNo;
use crate::model::print::Print;
use tusk_model::elements::{MeasureChild, Pb, Sb};
use tusk_model::musicxml_ext::PrintData;

/// Label prefix for MEI sb/pb elements carrying roundtrip print JSON data.
pub const PRINT_LABEL_PREFIX: &str = "musicxml:print,";

/// Convert a MusicXML `<print>` element to MEI `<sb>` and/or `<pb>` measure children.
///
/// - `new-system="yes"` → `<sb>` with full Print JSON in @label
/// - `new-page="yes"` → `<pb>` with full Print JSON in @label
/// - Both → `<pb>` only (page break implies system break)
/// - Neither → `<sb>` as carrier for layout-only print data
pub fn convert_print(print: &Print, ctx: &mut ConversionContext) -> Vec<MeasureChild> {
    let json_label = serde_json::to_string(print)
        .ok()
        .map(|json| format!("{}{}", PRINT_LABEL_PREFIX, json));

    let has_new_page = print.new_page == Some(YesNo::Yes);
    let has_new_system = print.new_system == Some(YesNo::Yes);

    let mut children = Vec::new();

    let print_ext = build_print_data(print);

    if has_new_page {
        // Page break (implies system break too)
        let mut pb = Pb::default();
        pb.common.xml_id = Some(ctx.generate_id_with_suffix("pb"));
        if let Some(ref label) = json_label {
            pb.common.label = Some(label.clone());
        }
        if let Some(ref id) = pb.common.xml_id {
            ctx.ext_store_mut().entry(id.clone()).print_data = Some(print_ext.clone());
        }
        children.push(MeasureChild::Pb(Box::new(pb)));
    } else if has_new_system {
        // System break only
        let mut sb = Sb::default();
        sb.common.xml_id = Some(ctx.generate_id_with_suffix("sb"));
        if let Some(ref label) = json_label {
            sb.common.label = Some(label.clone());
        }
        if let Some(ref id) = sb.common.xml_id {
            ctx.ext_store_mut().entry(id.clone()).print_data = Some(print_ext.clone());
        }
        children.push(MeasureChild::Sb(Box::new(sb)));
    } else {
        // No break, but has layout data — use sb as carrier
        let mut sb = Sb::default();
        sb.common.xml_id = Some(ctx.generate_id_with_suffix("sb"));
        if let Some(ref label) = json_label {
            sb.common.label = Some(label.clone());
        }
        if let Some(ref id) = sb.common.xml_id {
            ctx.ext_store_mut().entry(id.clone()).print_data = Some(print_ext);
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

/// Deserialize a Print from a roundtrip label string.
pub fn print_from_label(label: &str) -> Option<Print> {
    let json = label.strip_prefix(PRINT_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

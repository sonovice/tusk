//! Barline children conversion from MusicXML to MEI.
//!
//! Barlines with extra children (repeat, ending, fermata, segno, coda,
//! wavy-line) are stored as JSON-in-label on MEI `<dir>` control events
//! for lossless roundtrip. The basic bar-style is still set on MEI
//! measure @left/@right via the existing bar_style_to_mei_barrendition
//! mapping. This module handles the extra children only.

use crate::context::ConversionContext;
use crate::model::elements::Barline;
use tusk_model::elements::{Dir, DirChild, MeasureChild};

/// Label prefix for MEI dir elements carrying barline JSON data.
pub const BARLINE_LABEL_PREFIX: &str = "musicxml:barline,";

/// Convert a MusicXML `<barline>` with extra children to an MEI `<dir>` measure child.
///
/// Only called for barlines that have children beyond bar-style (repeat, ending,
/// fermata, segno, coda, wavy-line) or extra attributes (segno, coda, divisions).
/// The full Barline struct is serialized as JSON in the dir's `@label` attribute.
pub fn convert_barline(barline: &Barline, ctx: &mut ConversionContext) -> MeasureChild {
    let json_label = serde_json::to_string(barline)
        .ok()
        .map(|json| format!("{}{}", BARLINE_LABEL_PREFIX, json));

    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("barline"));
    if let Some(label) = json_label {
        dir.common.label = Some(label);
    }

    // Human-readable summary
    let summary = barline_summary(barline);
    if !summary.is_empty() {
        dir.children.push(DirChild::Text(summary));
    }

    // Set tstamp to 1 (barlines are measure-level, not beat-positioned)
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));

    // Staff — barlines are measure-level, always use staff 1
    dir.dir_log.staff = Some("1".to_string());

    MeasureChild::Dir(Box::new(dir))
}

/// Generate a human-readable summary of a Barline's extra children.
fn barline_summary(barline: &Barline) -> String {
    let mut parts = Vec::new();

    if let Some(loc) = &barline.location {
        parts.push(format!("barline:{}", loc.to_musicxml_str()));
    }
    if let Some(style) = &barline.bar_style {
        parts.push(style.to_musicxml_str().to_string());
    }
    if let Some(ref repeat) = barline.repeat {
        let dir = match repeat.direction {
            crate::model::elements::BackwardForward::Forward => "forward",
            crate::model::elements::BackwardForward::Backward => "backward",
        };
        parts.push(format!("repeat:{dir}"));
    }
    if let Some(ref ending) = barline.ending {
        parts.push(format!("ending:{}", ending.number));
    }
    if !barline.fermatas.is_empty() {
        parts.push(format!("fermata({})", barline.fermatas.len()));
    }
    if barline.segno.is_some() {
        parts.push("segno".to_string());
    }
    if barline.coda.is_some() {
        parts.push("coda".to_string());
    }
    if barline.wavy_line.is_some() {
        parts.push("wavy-line".to_string());
    }

    parts.join("; ")
}

/// Deserialize a Barline from a roundtrip label string.
pub fn barline_from_label(label: &str) -> Option<Barline> {
    let json = label.strip_prefix(BARLINE_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

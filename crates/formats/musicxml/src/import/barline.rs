//! Barline children conversion from MusicXML to MEI.
//!
//! Barlines with extra children (repeat, ending, fermata, segno, coda,
//! wavy-line) are stored in ExtensionStore on MEI `<dir>` control events
//! for lossless roundtrip. The basic bar-style is still set on MEI
//! measure @left/@right via the existing bar_style_to_mei_barrendition
//! mapping. This module handles the extra children only.

use crate::context::ConversionContext;
use crate::model::elements::Barline;
use tusk_model::elements::{Dir, DirChild, MeasureChild};
use tusk_model::musicxml_ext::{BarlineData, EndingData, RepeatData};

/// Convert a MusicXML `<barline>` with extra children to an MEI `<dir>` measure child.
///
/// Only called for barlines that have children beyond bar-style (repeat, ending,
/// fermata, segno, coda, wavy-line) or extra attributes (segno, coda, divisions).
/// Data is stored in ExtensionStore; identified by map membership (no label).
pub fn convert_barline(barline: &Barline, ctx: &mut ConversionContext) -> MeasureChild {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("barline"));

    // Store typed BarlineData in ExtensionStore (no label, no mxml_json)
    if let Some(ref id) = dir.common.xml_id {
        ctx.ext_store_mut()
            .insert_barline(id.clone(), build_barline_data(barline));
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

fn build_barline_data(b: &Barline) -> BarlineData {
    use crate::model::data::YesNo;

    BarlineData {
        location: b.location.as_ref().map(|l| l.to_musicxml_str().to_string()),
        bar_style: b
            .bar_style
            .as_ref()
            .map(|s| s.to_musicxml_str().to_string()),
        repeat: b.repeat.as_ref().map(|r| RepeatData {
            direction: match r.direction {
                crate::model::elements::BackwardForward::Forward => "forward".to_string(),
                crate::model::elements::BackwardForward::Backward => "backward".to_string(),
            },
            times: r.times,
            after_jump: r.after_jump.map(|v| matches!(v, YesNo::Yes)),
            winged: r
                .winged
                .as_ref()
                .and_then(|w| serde_json::to_value(w).ok())
                .and_then(|v| v.as_str().map(|s| s.to_string())),
        }),
        ending: b.ending.as_ref().map(|e| EndingData {
            number: e.number.clone(),
            ending_type: e.ending_type.to_string(),
            text: e.text.clone(),
            visual: None,
        }),
        fermatas: b
            .fermatas
            .iter()
            .filter_map(|f| serde_json::to_value(f).ok())
            .collect(),
        segno: b.segno.as_ref().and_then(|s| serde_json::to_value(s).ok()),
        coda: b.coda.as_ref().and_then(|c| serde_json::to_value(c).ok()),
        wavy_line: b
            .wavy_line
            .as_ref()
            .and_then(|w| serde_json::to_value(w).ok()),
        segno_attr: b.segno_attr.clone(),
        coda_attr: b.coda_attr.clone(),
        divisions: b.divisions,
    }
}

//! Measure-style export from MEI to MusicXML.
//!
//! Converts MEI `<dir>` elements carrying `musicxml:measure-style,` labels back
//! to MusicXML `<measure-style>` inside `<attributes>`. Uses the same JSON-in-label
//! roundtrip pattern as sound, print, harmony, and figured-bass.

use crate::context::ConversionContext;
use crate::import::measure_style::measure_style_from_label;
use crate::model::attributes::{Attributes, MeasureStyle};
use crate::model::elements::MeasureContent;
use tusk_model::elements::Dir;

/// Convert an MEI `<dir>` with a measure-style label to MusicXML `<attributes>`.
///
/// Tries ExtensionStore mxml_json first, falls back to label parsing.
pub fn convert_mei_measure_style_dir(
    dir: &Dir,
    ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    // Preferred: reconstruct from ExtensionStore mxml_json
    if let Some(id) = &dir.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref val) = ext.mxml_json {
                if let Ok(ms) = serde_json::from_value::<MeasureStyle>(val.clone()) {
                    let attrs = Attributes {
                        measure_styles: vec![ms],
                        ..Default::default()
                    };
                    return Some(MeasureContent::Attributes(Box::new(attrs)));
                }
            }
        }
    }

    // Fallback: reconstruct from label
    let label = dir.common.label.as_deref()?;
    let ms = measure_style_from_label(label)?;
    let attrs = Attributes {
        measure_styles: vec![ms],
        ..Default::default()
    };
    Some(MeasureContent::Attributes(Box::new(attrs)))
}

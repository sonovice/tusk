//! Measure-style export from MEI to MusicXML.
//!
//! Converts MEI `<dir>` elements carrying `musicxml:measure-style,` labels back
//! to MusicXML `<measure-style>` inside `<attributes>`. Uses the same JSON-in-label
//! roundtrip pattern as sound, print, harmony, and figured-bass.

use crate::context::ConversionContext;
use crate::import::measure_style::measure_style_from_label;
use crate::model::attributes::Attributes;
use crate::model::elements::MeasureContent;
use tusk_model::elements::Dir;

/// Convert an MEI `<dir>` with a measure-style label to MusicXML `<attributes>`.
///
/// Returns `Some(MeasureContent::Attributes)` containing just the measure-style,
/// or `None` if the dir doesn't carry a measure-style label.
pub fn convert_mei_measure_style_dir(
    dir: &Dir,
    _ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    let label = dir.common.label.as_deref()?;
    let ms = measure_style_from_label(label)?;
    let attrs = Attributes {
        measure_styles: vec![ms],
        ..Default::default()
    };
    Some(MeasureContent::Attributes(Box::new(attrs)))
}

//! Sound element export from MEI to MusicXML.
//!
//! Converts MEI `<dir>` elements carrying `musicxml:sound,` labels back to
//! standalone MusicXML `<sound>` measure content elements. Uses the same
//! JSON-in-label roundtrip pattern as harmony, figured-bass, and print.

use crate::context::ConversionContext;
use crate::import::sound::sound_from_label;
use crate::model::direction::Sound;
use crate::model::elements::MeasureContent;
use tusk_model::elements::Dir;

/// Convert an MEI `<dir>` with a standalone sound label to a MusicXML `<sound>` element.
///
/// Tries ExtensionStore mxml_json first, falls back to label parsing.
pub fn convert_mei_sound_dir(dir: &Dir, ctx: &mut ConversionContext) -> Option<MeasureContent> {
    // Preferred: reconstruct from ExtensionStore mxml_json
    if let Some(id) = &dir.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref val) = ext.mxml_json {
                if let Ok(sound) = serde_json::from_value::<Sound>(val.clone()) {
                    return Some(MeasureContent::Sound(Box::new(sound)));
                }
            }
        }
    }

    // Fallback: reconstruct from label
    let label = dir.common.label.as_deref()?;
    let sound = sound_from_label(label)?;
    Some(MeasureContent::Sound(Box::new(sound)))
}

//! Sound element export from MEI to MusicXML.
//!
//! Converts MEI `<dir>` elements carrying `musicxml:sound,` labels back to
//! standalone MusicXML `<sound>` measure content elements. Uses the same
//! JSON-in-label roundtrip pattern as harmony, figured-bass, and print.

use crate::context::ConversionContext;
use crate::import::sound::sound_from_label;
use crate::model::elements::MeasureContent;
use tusk_model::elements::Dir;

/// Convert an MEI `<dir>` with a standalone sound label to a MusicXML `<sound>` element.
///
/// Returns `Some(MeasureContent::Sound)` if the dir carries a `musicxml:sound,` label,
/// `None` otherwise.
pub fn convert_mei_sound_dir(dir: &Dir, _ctx: &mut ConversionContext) -> Option<MeasureContent> {
    let label = dir.common.label.as_deref()?;
    let sound = sound_from_label(label)?;
    Some(MeasureContent::Sound(Box::new(sound)))
}

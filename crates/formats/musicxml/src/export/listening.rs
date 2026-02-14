//! Converts MEI `<dir>` elements carrying listening/grouping/link/bookmark
//! data back to MusicXML measure-level elements.

use crate::context::ConversionContext;
use crate::model::elements::MeasureContent;
use tusk_model::elements::Dir;
use tusk_model::musicxml_ext::ListeningData;

/// Try to convert an MEI `<dir>` to a listening/grouping/link/bookmark MusicXML element.
///
/// Returns `Some(MeasureContent)` if this dir carries listening data in the ExtensionStore.
pub fn convert_mei_listening_dir(dir: &Dir, ctx: &mut ConversionContext) -> Option<MeasureContent> {
    let id = dir.common.xml_id.as_ref()?;
    let data = ctx.ext_store().listening(id)?;
    match data {
        ListeningData::Listening(val) => {
            Some(MeasureContent::Listening(Box::new(val.clone())))
        }
        ListeningData::Grouping(val) => {
            Some(MeasureContent::Grouping(Box::new(val.clone())))
        }
        ListeningData::Link(val) => {
            Some(MeasureContent::Link(Box::new(val.clone())))
        }
        ListeningData::Bookmark(val) => {
            Some(MeasureContent::Bookmark(Box::new(val.clone())))
        }
    }
}

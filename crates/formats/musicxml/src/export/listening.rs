//! Converts MEI `<dir>` elements carrying listening/grouping/link/bookmark
//! data back to MusicXML measure-level elements.

use crate::context::ConversionContext;
use crate::model::elements::MeasureContent;
use crate::model::elements::score::{Bookmark, Link};
use crate::model::listening::{Grouping, Listening};
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
            let l: Listening = serde_json::from_value(val.clone()).ok()?;
            Some(MeasureContent::Listening(Box::new(l)))
        }
        ListeningData::Grouping(val) => {
            let g: Grouping = serde_json::from_value(val.clone()).ok()?;
            Some(MeasureContent::Grouping(Box::new(g)))
        }
        ListeningData::Link(val) => {
            let l: Link = serde_json::from_value(val.clone()).ok()?;
            Some(MeasureContent::Link(Box::new(l)))
        }
        ListeningData::Bookmark(val) => {
            let b: Bookmark = serde_json::from_value(val.clone()).ok()?;
            Some(MeasureContent::Bookmark(Box::new(b)))
        }
    }
}

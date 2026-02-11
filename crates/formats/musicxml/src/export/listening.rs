//! Converts MEI `<dir>` elements carrying listening/grouping/link/bookmark
//! labels back to MusicXML measure-level elements.

use crate::context::ConversionContext;
use crate::import::listening::{
    bookmark_from_label, grouping_from_label, link_from_label, listening_from_label,
};
use crate::model::elements::score::{Bookmark, Link};
use crate::model::elements::MeasureContent;
use crate::model::listening::{Grouping, Listening};
use tusk_model::elements::Dir;

/// Convert an MEI `<dir>` with a listening label to MusicXML `<listening>`.
pub fn convert_mei_listening_dir(
    dir: &Dir,
    ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    // Preferred: reconstruct from ExtensionStore mxml_json
    if let Some(id) = &dir.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref val) = ext.mxml_json {
                if let Ok(l) = serde_json::from_value::<Listening>(val.clone()) {
                    return Some(MeasureContent::Listening(Box::new(l)));
                }
            }
        }
    }
    let label = dir.common.label.as_deref()?;
    let listening = listening_from_label(label)?;
    Some(MeasureContent::Listening(Box::new(listening)))
}

/// Convert an MEI `<dir>` with a grouping label to MusicXML `<grouping>`.
pub fn convert_mei_grouping_dir(dir: &Dir, ctx: &mut ConversionContext) -> Option<MeasureContent> {
    if let Some(id) = &dir.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref val) = ext.mxml_json {
                if let Ok(g) = serde_json::from_value::<Grouping>(val.clone()) {
                    return Some(MeasureContent::Grouping(Box::new(g)));
                }
            }
        }
    }
    let label = dir.common.label.as_deref()?;
    let grouping = grouping_from_label(label)?;
    Some(MeasureContent::Grouping(Box::new(grouping)))
}

/// Convert an MEI `<dir>` with a link label to MusicXML `<link>`.
pub fn convert_mei_link_dir(dir: &Dir, ctx: &mut ConversionContext) -> Option<MeasureContent> {
    if let Some(id) = &dir.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref val) = ext.mxml_json {
                if let Ok(l) = serde_json::from_value::<Link>(val.clone()) {
                    return Some(MeasureContent::Link(Box::new(l)));
                }
            }
        }
    }
    let label = dir.common.label.as_deref()?;
    let link = link_from_label(label)?;
    Some(MeasureContent::Link(Box::new(link)))
}

/// Convert an MEI `<dir>` with a bookmark label to MusicXML `<bookmark>`.
pub fn convert_mei_bookmark_dir(dir: &Dir, ctx: &mut ConversionContext) -> Option<MeasureContent> {
    if let Some(id) = &dir.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref val) = ext.mxml_json {
                if let Ok(b) = serde_json::from_value::<Bookmark>(val.clone()) {
                    return Some(MeasureContent::Bookmark(Box::new(b)));
                }
            }
        }
    }
    let label = dir.common.label.as_deref()?;
    let bookmark = bookmark_from_label(label)?;
    Some(MeasureContent::Bookmark(Box::new(bookmark)))
}

//! Converts MEI `<dir>` elements carrying listening/grouping/link/bookmark
//! labels back to MusicXML measure-level elements.

use crate::context::ConversionContext;
use crate::import::listening::{
    bookmark_from_label, grouping_from_label, link_from_label, listening_from_label,
};
use crate::model::elements::MeasureContent;
use tusk_model::elements::Dir;

/// Convert an MEI `<dir>` with a listening label to MusicXML `<listening>`.
pub fn convert_mei_listening_dir(
    dir: &Dir,
    _ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    let label = dir.common.label.as_deref()?;
    let listening = listening_from_label(label)?;
    Some(MeasureContent::Listening(Box::new(listening)))
}

/// Convert an MEI `<dir>` with a grouping label to MusicXML `<grouping>`.
pub fn convert_mei_grouping_dir(dir: &Dir, _ctx: &mut ConversionContext) -> Option<MeasureContent> {
    let label = dir.common.label.as_deref()?;
    let grouping = grouping_from_label(label)?;
    Some(MeasureContent::Grouping(Box::new(grouping)))
}

/// Convert an MEI `<dir>` with a link label to MusicXML `<link>`.
pub fn convert_mei_link_dir(dir: &Dir, _ctx: &mut ConversionContext) -> Option<MeasureContent> {
    let label = dir.common.label.as_deref()?;
    let link = link_from_label(label)?;
    Some(MeasureContent::Link(Box::new(link)))
}

/// Convert an MEI `<dir>` with a bookmark label to MusicXML `<bookmark>`.
pub fn convert_mei_bookmark_dir(dir: &Dir, _ctx: &mut ConversionContext) -> Option<MeasureContent> {
    let label = dir.common.label.as_deref()?;
    let bookmark = bookmark_from_label(label)?;
    Some(MeasureContent::Bookmark(Box::new(bookmark)))
}

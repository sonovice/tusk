//! Listening, grouping, link, and bookmark conversion from MusicXML to MEI.
//!
//! These measure-level elements are stored as JSON-in-label on MEI `<dir>`
//! control events for lossless roundtrip, following the same pattern as
//! barline, sound, print, and measure-style conversions.

use crate::context::ConversionContext;
use crate::model::elements::score::{Bookmark, Link};
use crate::model::listening::{Grouping, Listening};
use tusk_model::elements::{Dir, DirChild, MeasureChild};
use tusk_model::musicxml_ext::ListeningData;

pub const LISTENING_LABEL_PREFIX: &str = "musicxml:listening,";
pub const GROUPING_LABEL_PREFIX: &str = "musicxml:grouping,";
pub const LINK_LABEL_PREFIX: &str = "musicxml:link,";
pub const BOOKMARK_LABEL_PREFIX: &str = "musicxml:bookmark,";

/// Convert a standalone MusicXML `<listening>` element to an MEI `<dir>`.
pub fn convert_listening(listening: &Listening, ctx: &mut ConversionContext) -> MeasureChild {
    let json_label = serde_json::to_string(listening)
        .ok()
        .map(|json| format!("{}{}", LISTENING_LABEL_PREFIX, json));

    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("listening"));
    if let Some(label) = json_label {
        dir.common.label = Some(label);
    }
    dir.children.push(DirChild::Text("listening".to_string()));
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
    dir.dir_log.staff = Some("1".to_string());

    if let Some(ref id) = dir.common.xml_id {
        if let Ok(val) = serde_json::to_value(listening) {
            let entry = ctx.ext_store_mut().entry(id.clone());
            entry.listening = Some(ListeningData::Listening(val.clone()));
            entry.mxml_json = Some(val);
        }
    }

    MeasureChild::Dir(Box::new(dir))
}

/// Convert a MusicXML `<grouping>` element to an MEI `<dir>`.
pub fn convert_grouping(grouping: &Grouping, ctx: &mut ConversionContext) -> MeasureChild {
    let json_label = serde_json::to_string(grouping)
        .ok()
        .map(|json| format!("{}{}", GROUPING_LABEL_PREFIX, json));

    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("grouping"));
    if let Some(label) = json_label {
        dir.common.label = Some(label);
    }
    let summary = format!("grouping:{}", grouping.grouping_type);
    dir.children.push(DirChild::Text(summary));
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
    dir.dir_log.staff = Some("1".to_string());

    if let Some(ref id) = dir.common.xml_id {
        if let Ok(val) = serde_json::to_value(grouping) {
            let entry = ctx.ext_store_mut().entry(id.clone());
            entry.listening = Some(ListeningData::Grouping(val.clone()));
            entry.mxml_json = Some(val);
        }
    }

    MeasureChild::Dir(Box::new(dir))
}

/// Convert a MusicXML `<link>` element to an MEI `<dir>`.
pub fn convert_link(link: &Link, ctx: &mut ConversionContext) -> MeasureChild {
    let json_label = serde_json::to_string(link)
        .ok()
        .map(|json| format!("{}{}", LINK_LABEL_PREFIX, json));

    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("link"));
    if let Some(label) = json_label {
        dir.common.label = Some(label);
    }
    dir.children
        .push(DirChild::Text(format!("link:{}", link.href)));
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
    dir.dir_log.staff = Some("1".to_string());

    if let Some(ref id) = dir.common.xml_id {
        if let Ok(val) = serde_json::to_value(link) {
            let entry = ctx.ext_store_mut().entry(id.clone());
            entry.listening = Some(ListeningData::Link(val.clone()));
            entry.mxml_json = Some(val);
        }
    }

    MeasureChild::Dir(Box::new(dir))
}

/// Convert a MusicXML `<bookmark>` element to an MEI `<dir>`.
pub fn convert_bookmark(bookmark: &Bookmark, ctx: &mut ConversionContext) -> MeasureChild {
    let json_label = serde_json::to_string(bookmark)
        .ok()
        .map(|json| format!("{}{}", BOOKMARK_LABEL_PREFIX, json));

    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("bookmark"));
    if let Some(label) = json_label {
        dir.common.label = Some(label);
    }
    dir.children
        .push(DirChild::Text(format!("bookmark:{}", bookmark.id)));
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
    dir.dir_log.staff = Some("1".to_string());

    if let Some(ref id) = dir.common.xml_id {
        if let Ok(val) = serde_json::to_value(bookmark) {
            let entry = ctx.ext_store_mut().entry(id.clone());
            entry.listening = Some(ListeningData::Bookmark(val.clone()));
            entry.mxml_json = Some(val);
        }
    }

    MeasureChild::Dir(Box::new(dir))
}

/// Deserialize a Listening from a roundtrip label.
pub fn listening_from_label(label: &str) -> Option<Listening> {
    let json = label.strip_prefix(LISTENING_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

/// Deserialize a Grouping from a roundtrip label.
pub fn grouping_from_label(label: &str) -> Option<Grouping> {
    let json = label.strip_prefix(GROUPING_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

/// Deserialize a Link from a roundtrip label.
pub fn link_from_label(label: &str) -> Option<Link> {
    let json = label.strip_prefix(LINK_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

/// Deserialize a Bookmark from a roundtrip label.
pub fn bookmark_from_label(label: &str) -> Option<Bookmark> {
    let json = label.strip_prefix(BOOKMARK_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

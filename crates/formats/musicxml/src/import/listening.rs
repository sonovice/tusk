//! Listening, grouping, link, and bookmark conversion from MusicXML to MEI.
//!
//! These measure-level elements are stored in ExtensionStore on MEI `<dir>`
//! control events for lossless roundtrip.

use crate::context::ConversionContext;
use crate::model::elements::score::{Bookmark, Link};
use crate::model::listening::{Grouping, Listening};
use tusk_model::elements::{Dir, DirChild, MeasureChild};
use tusk_model::musicxml_ext::ListeningData;

pub const LISTENING_LABEL_PREFIX: &str = "musicxml:listening";
pub const GROUPING_LABEL_PREFIX: &str = "musicxml:grouping";
pub const LINK_LABEL_PREFIX: &str = "musicxml:link";
pub const BOOKMARK_LABEL_PREFIX: &str = "musicxml:bookmark";

/// Convert a standalone MusicXML `<listening>` element to an MEI `<dir>`.
pub fn convert_listening(listening: &Listening, ctx: &mut ConversionContext) -> MeasureChild {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("listening"));
    dir.common.label = Some(LISTENING_LABEL_PREFIX.to_string());
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
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("grouping"));
    dir.common.label = Some(GROUPING_LABEL_PREFIX.to_string());
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
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("link"));
    dir.common.label = Some(LINK_LABEL_PREFIX.to_string());
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
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("bookmark"));
    dir.common.label = Some(BOOKMARK_LABEL_PREFIX.to_string());
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

/// Deserialize a Listening from a legacy JSON roundtrip label.
pub fn listening_from_label(label: &str) -> Option<Listening> {
    if label == LISTENING_LABEL_PREFIX {
        return None;
    }
    let json = label.strip_prefix("musicxml:listening,")?;
    serde_json::from_str(json).ok()
}

/// Deserialize a Grouping from a legacy JSON roundtrip label.
pub fn grouping_from_label(label: &str) -> Option<Grouping> {
    if label == GROUPING_LABEL_PREFIX {
        return None;
    }
    let json = label.strip_prefix("musicxml:grouping,")?;
    serde_json::from_str(json).ok()
}

/// Deserialize a Link from a legacy JSON roundtrip label.
pub fn link_from_label(label: &str) -> Option<Link> {
    if label == LINK_LABEL_PREFIX {
        return None;
    }
    let json = label.strip_prefix("musicxml:link,")?;
    serde_json::from_str(json).ok()
}

/// Deserialize a Bookmark from a legacy JSON roundtrip label.
pub fn bookmark_from_label(label: &str) -> Option<Bookmark> {
    if label == BOOKMARK_LABEL_PREFIX {
        return None;
    }
    let json = label.strip_prefix("musicxml:bookmark,")?;
    serde_json::from_str(json).ok()
}

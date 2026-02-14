//! Listening, grouping, link, and bookmark conversion from MusicXML to MEI.
//!
//! These measure-level elements are stored in ExtensionStore on MEI `<dir>`
//! control events for lossless roundtrip.

use crate::context::ConversionContext;
use crate::model::elements::score::{Bookmark, Link};
use crate::model::listening::{Grouping, Listening};
use tusk_model::elements::{Dir, DirChild, MeasureChild};
use tusk_model::musicxml_ext::ListeningData;

/// Convert a standalone MusicXML `<listening>` element to an MEI `<dir>`.
pub fn convert_listening(listening: &Listening, ctx: &mut ConversionContext) -> MeasureChild {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("listening"));
    dir.children.push(DirChild::Text("listening".to_string()));
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
    dir.dir_log.staff = Some("1".to_string());

    if let Some(ref id) = dir.common.xml_id {
        if let Ok(val) = serde_json::to_value(listening) {
            ctx.ext_store_mut()
                .insert_listening(id.clone(), ListeningData::Listening(val));
        }
    }

    MeasureChild::Dir(Box::new(dir))
}

/// Convert a MusicXML `<grouping>` element to an MEI `<dir>`.
pub fn convert_grouping(grouping: &Grouping, ctx: &mut ConversionContext) -> MeasureChild {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("grouping"));
    let summary = format!("grouping:{}", grouping.grouping_type);
    dir.children.push(DirChild::Text(summary));
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
    dir.dir_log.staff = Some("1".to_string());

    if let Some(ref id) = dir.common.xml_id {
        if let Ok(val) = serde_json::to_value(grouping) {
            ctx.ext_store_mut()
                .insert_listening(id.clone(), ListeningData::Grouping(val));
        }
    }

    MeasureChild::Dir(Box::new(dir))
}

/// Convert a MusicXML `<link>` element to an MEI `<dir>`.
pub fn convert_link(link: &Link, ctx: &mut ConversionContext) -> MeasureChild {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("link"));
    dir.children
        .push(DirChild::Text(format!("link:{}", link.href)));
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
    dir.dir_log.staff = Some("1".to_string());

    if let Some(ref id) = dir.common.xml_id {
        if let Ok(val) = serde_json::to_value(link) {
            ctx.ext_store_mut()
                .insert_listening(id.clone(), ListeningData::Link(val));
        }
    }

    MeasureChild::Dir(Box::new(dir))
}

/// Convert a MusicXML `<bookmark>` element to an MEI `<dir>`.
pub fn convert_bookmark(bookmark: &Bookmark, ctx: &mut ConversionContext) -> MeasureChild {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("bookmark"));
    dir.children
        .push(DirChild::Text(format!("bookmark:{}", bookmark.id)));
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));
    dir.dir_log.staff = Some("1".to_string());

    if let Some(ref id) = dir.common.xml_id {
        if let Ok(val) = serde_json::to_value(bookmark) {
            ctx.ext_store_mut()
                .insert_listening(id.clone(), ListeningData::Bookmark(val));
        }
    }

    MeasureChild::Dir(Box::new(dir))
}

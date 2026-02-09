//! Serializer implementations for listening, listen, grouping, link, and bookmark types.

use std::io::Write;

use crate::model::elements::score::{Bookmark, Link};
use crate::model::listening::*;
use crate::serializer::{
    MusicXmlSerialize, MusicXmlWriter, SerializeResult, push_opt_attr, push_opt_str_attr,
};

use super::score::yes_no_str;

// ============================================================================
// Listening
// ============================================================================

impl MusicXmlSerialize for Listening {
    fn element_name(&self) -> &'static str {
        "listening"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty() || self.offset.is_some()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                ListeningChild::Sync(sync) => sync.serialize(w)?,
                ListeningChild::OtherListening(other) => {
                    serialize_other_listening(w, other, "other-listening")?;
                }
            }
        }
        if let Some(ref offset) = self.offset {
            let mut start = w.start_element("offset");
            if let Some(ref sound) = offset.sound {
                start.push_attribute(("sound", yes_no_str(sound)));
            }
            w.write_start(start)?;
            w.write_text(&offset.value.to_string())?;
            w.write_end("offset")?;
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Sync {
    fn element_name(&self) -> &'static str {
        "sync"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("type", self.sync_type.clone())];
        push_opt_attr!(attrs, "latency", self.latency);
        push_opt_str_attr!(attrs, "player", self.player);
        push_opt_str_attr!(attrs, "time-only", self.time_only);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

fn serialize_other_listening<W: Write>(
    w: &mut MusicXmlWriter<W>,
    other: &OtherListening,
    element_name: &str,
) -> SerializeResult<()> {
    let mut start = w.start_element(element_name);
    start.push_attribute(("type", other.other_type.as_str()));
    if let Some(ref player) = other.player {
        start.push_attribute(("player", player.as_str()));
    }
    if let Some(ref time_only) = other.time_only {
        start.push_attribute(("time-only", time_only.as_str()));
    }
    if other.value.is_empty() {
        w.write_empty(start)?;
    } else {
        w.write_start(start)?;
        w.write_text(&other.value)?;
        w.write_end(element_name)?;
    }
    Ok(())
}

// ============================================================================
// Listen (note-level)
// ============================================================================

impl MusicXmlSerialize for Listen {
    fn element_name(&self) -> &'static str {
        "listen"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                ListenChild::Assess(assess) => assess.serialize(w)?,
                ListenChild::Wait(wait) => wait.serialize(w)?,
                ListenChild::OtherListen(other) => {
                    serialize_other_listening(w, other, "other-listen")?;
                }
            }
        }
        Ok(())
    }
}

impl MusicXmlSerialize for Assess {
    fn element_name(&self) -> &'static str {
        "assess"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("type", yes_no_str(&self.assess_type).to_string())];
        push_opt_str_attr!(attrs, "player", self.player);
        push_opt_str_attr!(attrs, "time-only", self.time_only);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MusicXmlSerialize for Wait {
    fn element_name(&self) -> &'static str {
        "wait"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_opt_str_attr!(attrs, "player", self.player);
        push_opt_str_attr!(attrs, "time-only", self.time_only);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Grouping
// ============================================================================

impl MusicXmlSerialize for Grouping {
    fn element_name(&self) -> &'static str {
        "grouping"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("type", self.grouping_type.clone())];
        push_opt_str_attr!(attrs, "number", self.number);
        push_opt_str_attr!(attrs, "member-of", self.member_of);
        push_opt_str_attr!(attrs, "id", self.id);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.features.is_empty()
    }

    fn serialize_children<W: Write>(&self, w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        for feature in &self.features {
            let mut start = w.start_element("feature");
            if let Some(ref ft) = feature.feature_type {
                start.push_attribute(("type", ft.as_str()));
            }
            if feature.value.is_empty() {
                w.write_empty(start)?;
            } else {
                w.write_start(start)?;
                w.write_text(&feature.value)?;
                w.write_end("feature")?;
            }
        }
        Ok(())
    }
}

// ============================================================================
// Link
// ============================================================================

impl MusicXmlSerialize for Link {
    fn element_name(&self) -> &'static str {
        "link"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("xmlns:xlink", "http://www.w3.org/1999/xlink".to_string())];
        attrs.push(("xlink:href", self.href.clone()));
        push_opt_str_attr!(attrs, "xlink:type", self.xlink_type);
        push_opt_str_attr!(attrs, "xlink:role", self.xlink_role);
        push_opt_str_attr!(attrs, "xlink:title", self.xlink_title);
        push_opt_str_attr!(attrs, "xlink:show", self.xlink_show);
        push_opt_str_attr!(attrs, "xlink:actuate", self.xlink_actuate);
        push_opt_str_attr!(attrs, "name", self.name);
        push_opt_str_attr!(attrs, "element", self.element);
        push_opt_attr!(attrs, "position", self.position);
        push_opt_attr!(attrs, "default-x", self.default_x);
        push_opt_attr!(attrs, "default-y", self.default_y);
        push_opt_attr!(attrs, "relative-x", self.relative_x);
        push_opt_attr!(attrs, "relative-y", self.relative_y);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Bookmark
// ============================================================================

impl MusicXmlSerialize for Bookmark {
    fn element_name(&self) -> &'static str {
        "bookmark"
    }

    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = vec![("id", self.id.clone())];
        push_opt_str_attr!(attrs, "name", self.name);
        push_opt_str_attr!(attrs, "element", self.element);
        push_opt_attr!(attrs, "position", self.position);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _w: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

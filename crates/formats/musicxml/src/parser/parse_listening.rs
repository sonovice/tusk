//! Parsing for listening, listen, grouping, link, and bookmark elements.

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::{ParseError, Result, get_attr, read_text, skip_element};
use crate::model::data::*;
use crate::model::elements::score::{Bookmark, Link};
use crate::model::listening::*;

// ============================================================================
// Listening (direction-level and measure-level)
// ============================================================================

/// Parse a `<listening>` element with children.
pub fn parse_listening<R: BufRead>(reader: &mut Reader<R>) -> Result<Listening> {
    let mut buf = Vec::new();
    let mut children = Vec::new();
    let mut offset = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"other-listening" => {
                    let other_type = get_attr(&e, "type")?.unwrap_or_default();
                    let player = get_attr(&e, "player")?;
                    let time_only = get_attr(&e, "time-only")?;
                    let value = read_text(reader, b"other-listening")?;
                    children.push(ListeningChild::OtherListening(OtherListening {
                        other_type,
                        player,
                        time_only,
                        value,
                    }));
                }
                b"offset" => {
                    let sound = get_attr(&e, "sound")?.and_then(|s| parse_yes_no_opt(&s));
                    let value = read_text(reader, b"offset")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("offset".to_string()))?;
                    offset = Some(crate::model::direction::Offset { value, sound });
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"sync" => {
                    children.push(ListeningChild::Sync(parse_sync_attrs(&e)?));
                }
                b"other-listening" => {
                    let other_type = get_attr(&e, "type")?.unwrap_or_default();
                    let player = get_attr(&e, "player")?;
                    let time_only = get_attr(&e, "time-only")?;
                    children.push(ListeningChild::OtherListening(OtherListening {
                        other_type,
                        player,
                        time_only,
                        value: String::new(),
                    }));
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"listening" => break,
            Event::Eof => return Err(ParseError::MissingElement("listening end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Listening { children, offset })
}

/// Parse `<sync>` attributes from an empty element.
fn parse_sync_attrs(e: &BytesStart) -> Result<Sync> {
    Ok(Sync {
        sync_type: get_attr(e, "type")?.unwrap_or_else(|| "none".to_string()),
        latency: get_attr(e, "latency")?.and_then(|s| s.parse().ok()),
        player: get_attr(e, "player")?,
        time_only: get_attr(e, "time-only")?,
    })
}

// ============================================================================
// Listen (note-level)
// ============================================================================

/// Parse a `<listen>` element with children.
pub fn parse_listen<R: BufRead>(reader: &mut Reader<R>) -> Result<Listen> {
    let mut buf = Vec::new();
    let mut children = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"other-listen" => {
                    let other_type = get_attr(&e, "type")?.unwrap_or_default();
                    let player = get_attr(&e, "player")?;
                    let time_only = get_attr(&e, "time-only")?;
                    let value = read_text(reader, b"other-listen")?;
                    children.push(ListenChild::OtherListen(OtherListening {
                        other_type,
                        player,
                        time_only,
                        value,
                    }));
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"assess" => {
                    let assess_type = get_attr(&e, "type")?
                        .and_then(|s| parse_yes_no_opt(&s))
                        .unwrap_or(YesNo::Yes);
                    let player = get_attr(&e, "player")?;
                    let time_only = get_attr(&e, "time-only")?;
                    children.push(ListenChild::Assess(Assess {
                        assess_type,
                        player,
                        time_only,
                    }));
                }
                b"wait" => {
                    let player = get_attr(&e, "player")?;
                    let time_only = get_attr(&e, "time-only")?;
                    children.push(ListenChild::Wait(Wait { player, time_only }));
                }
                b"other-listen" => {
                    let other_type = get_attr(&e, "type")?.unwrap_or_default();
                    let player = get_attr(&e, "player")?;
                    let time_only = get_attr(&e, "time-only")?;
                    children.push(ListenChild::OtherListen(OtherListening {
                        other_type,
                        player,
                        time_only,
                        value: String::new(),
                    }));
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"listen" => break,
            Event::Eof => return Err(ParseError::MissingElement("listen end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Listen { children })
}

// ============================================================================
// Grouping
// ============================================================================

/// Parse a `<grouping>` element with children.
pub fn parse_grouping<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Grouping> {
    let mut buf = Vec::new();
    let grouping_type = get_attr(start, "type")?.unwrap_or_else(|| "start".to_string());
    let number = get_attr(start, "number")?;
    let member_of = get_attr(start, "member-of")?;
    let id = get_attr(start, "id")?;
    let mut features = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"feature" => {
                    let feature_type = get_attr(&e, "type")?;
                    let value = read_text(reader, b"feature")?;
                    features.push(Feature {
                        feature_type,
                        value,
                    });
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => {
                if e.name().as_ref() == b"feature" {
                    let feature_type = get_attr(&e, "type")?;
                    features.push(Feature {
                        feature_type,
                        value: String::new(),
                    });
                }
            }
            Event::End(e) if e.name().as_ref() == b"grouping" => break,
            Event::Eof => return Err(ParseError::MissingElement("grouping end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Grouping {
        grouping_type,
        number,
        member_of,
        id,
        features,
    })
}

/// Parse a `<grouping>` empty element (e.g., `<grouping type="stop"/>`).
pub fn parse_grouping_empty(e: &BytesStart) -> Result<Grouping> {
    Ok(Grouping {
        grouping_type: get_attr(e, "type")?.unwrap_or_else(|| "start".to_string()),
        number: get_attr(e, "number")?,
        member_of: get_attr(e, "member-of")?,
        id: get_attr(e, "id")?,
        features: Vec::new(),
    })
}

// ============================================================================
// Link
// ============================================================================

/// Parse a `<link>` empty element (always empty per XSD).
pub fn parse_link_empty(e: &BytesStart) -> Result<Link> {
    Ok(Link {
        href: get_attr(e, "xlink:href")?.unwrap_or_default(),
        xlink_type: get_attr(e, "xlink:type")?,
        xlink_role: get_attr(e, "xlink:role")?,
        xlink_title: get_attr(e, "xlink:title")?,
        xlink_show: get_attr(e, "xlink:show")?,
        xlink_actuate: get_attr(e, "xlink:actuate")?,
        name: get_attr(e, "name")?,
        element: get_attr(e, "element")?,
        position: get_attr(e, "position")?.and_then(|s| s.parse().ok()),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        relative_x: get_attr(e, "relative-x")?.and_then(|s| s.parse().ok()),
        relative_y: get_attr(e, "relative-y")?.and_then(|s| s.parse().ok()),
    })
}

// ============================================================================
// Bookmark
// ============================================================================

/// Parse a `<bookmark>` empty element (always empty per XSD).
pub fn parse_bookmark_empty(e: &BytesStart) -> Result<Bookmark> {
    Ok(Bookmark {
        id: get_attr(e, "id")?.unwrap_or_default(),
        name: get_attr(e, "name")?,
        element: get_attr(e, "element")?,
        position: get_attr(e, "position")?.and_then(|s| s.parse().ok()),
    })
}

fn parse_yes_no_opt(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

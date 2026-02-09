//! Parser for MusicXML `<score-part>` child elements: `<player>` and `<part-link>`.

use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::{ParseError, Reader, Result, get_attr, get_attr_required, read_text, skip_element};
use crate::model::elements::{InstrumentLink, PartLink, Player};

/// Parse a `<player>` element (Start event).
pub(super) fn parse_player<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<Player> {
    let mut buf = Vec::new();
    let id = get_attr_required(start, "id")?;
    let mut player_name = String::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"player-name" => player_name = read_text(reader, b"player-name")?,
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"player" => break,
            Event::Eof => return Err(ParseError::MissingElement("player end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Player { id, player_name })
}

/// Parse a `<part-link>` element (Start event).
pub(super) fn parse_part_link<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<PartLink> {
    let mut buf = Vec::new();
    let href = get_attr_required(start, "xlink:href")?;
    let mut part_link = PartLink {
        href,
        xlink_type: get_attr(start, "xlink:type")?,
        xlink_role: get_attr(start, "xlink:role")?,
        xlink_title: get_attr(start, "xlink:title")?,
        xlink_show: get_attr(start, "xlink:show")?,
        xlink_actuate: get_attr(start, "xlink:actuate")?,
        instrument_links: Vec::new(),
        group_links: Vec::new(),
    };

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"group-link" => part_link
                    .group_links
                    .push(read_text(reader, b"group-link")?),
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => {
                if e.name().as_ref() == b"instrument-link" {
                    if let Some(id) = get_attr(&e, "id")? {
                        part_link.instrument_links.push(InstrumentLink { id });
                    }
                }
            }
            Event::End(e) if e.name().as_ref() == b"part-link" => break,
            Event::Eof => return Err(ParseError::MissingElement("part-link end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(part_link)
}

//! Parser for MusicXML `<figured-bass>` elements.

use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::{ParseError, Reader, Result, get_attr, read_text, skip_element};
use crate::model::data::{AboveBelow, StartStopContinue, YesNo};
use crate::model::direction::Offset;
use crate::model::figured_bass::{Figure, FigureExtend, FiguredBass};
use crate::model::harmony::StyleText;

/// Parse a `<figured-bass>` element.
pub fn parse_figured_bass<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<FiguredBass> {
    let mut buf = Vec::new();

    // Parse attributes
    let parentheses = get_attr(start, "parentheses")?.and_then(|s| parse_yes_no(&s));
    let placement = get_attr(start, "placement")?.and_then(|s| match s.as_str() {
        "above" => Some(AboveBelow::Above),
        "below" => Some(AboveBelow::Below),
        _ => None,
    });
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no(&s));
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let relative_x = get_attr(start, "relative-x")?.and_then(|s| s.parse().ok());
    let relative_y = get_attr(start, "relative-y")?.and_then(|s| s.parse().ok());
    let font_family = get_attr(start, "font-family")?;
    let font_style = get_attr(start, "font-style")?;
    let font_size = get_attr(start, "font-size")?.and_then(|s| s.parse().ok());
    let font_weight = get_attr(start, "font-weight")?;
    let color = get_attr(start, "color")?;
    let halign = get_attr(start, "halign")?;
    let valign = get_attr(start, "valign")?;
    let id = get_attr(start, "id")?;

    let mut figures: Vec<Figure> = Vec::new();
    let mut duration: Option<f64> = None;
    let mut footnote = None;
    let mut level = None;
    let mut offset: Option<Offset> = None;
    let mut staff: Option<u32> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"figure" => figures.push(parse_figure(reader)?),
                b"duration" => {
                    duration = Some(
                        read_text(reader, b"duration")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("duration".to_string()))?,
                    );
                }
                b"offset" => {
                    let sound = get_attr(&e, "sound")?.and_then(|s| parse_yes_no(&s));
                    let value: f64 = read_text(reader, b"offset")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("offset".to_string()))?;
                    offset = Some(Offset { value, sound });
                }
                b"staff" => {
                    staff = Some(
                        read_text(reader, b"staff")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staff".to_string()))?,
                    );
                }
                b"footnote" => {
                    footnote = Some(super::parse_note::parse_formatted_text(
                        reader,
                        &e,
                        b"footnote",
                    )?)
                }
                b"level" => level = Some(super::parse_note::parse_level(reader, &e)?),
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"figure" => figures.push(Figure {
                    prefix: None,
                    figure_number: None,
                    suffix: None,
                    extend: None,
                }),
                b"offset" => {
                    let sound = get_attr(&e, "sound")?.and_then(|s| parse_yes_no(&s));
                    offset = Some(Offset { value: 0.0, sound });
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"figured-bass" => break,
            Event::Eof => return Err(ParseError::MissingElement("figured-bass end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(FiguredBass {
        figures,
        duration,
        footnote,
        level,
        offset,
        staff,
        parentheses,
        placement,
        print_object,
        default_x,
        default_y,
        relative_x,
        relative_y,
        font_family,
        font_style,
        font_size,
        font_weight,
        color,
        halign,
        valign,
        id,
    })
}

/// Parse a `<figure>` element.
fn parse_figure<R: BufRead>(reader: &mut Reader<R>) -> Result<Figure> {
    let mut buf = Vec::new();
    let mut prefix: Option<StyleText> = None;
    let mut figure_number: Option<StyleText> = None;
    let mut suffix: Option<StyleText> = None;
    let mut extend: Option<FigureExtend> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"prefix" => prefix = Some(parse_style_text(reader, &e, b"prefix")?),
                b"figure-number" => {
                    figure_number = Some(parse_style_text(reader, &e, b"figure-number")?)
                }
                b"suffix" => suffix = Some(parse_style_text(reader, &e, b"suffix")?),
                b"extend" => {
                    let extend_type =
                        get_attr(&e, "type")?.and_then(|s| parse_start_stop_continue(&s));
                    let ext_default_x = get_attr(&e, "default-x")?.and_then(|s| s.parse().ok());
                    let ext_default_y = get_attr(&e, "default-y")?.and_then(|s| s.parse().ok());
                    let ext_relative_x = get_attr(&e, "relative-x")?.and_then(|s| s.parse().ok());
                    let ext_relative_y = get_attr(&e, "relative-y")?.and_then(|s| s.parse().ok());
                    let ext_color = get_attr(&e, "color")?;
                    // Consume until end tag
                    let mut inner_buf = Vec::new();
                    loop {
                        match reader.read_event_into(&mut inner_buf)? {
                            Event::End(end) if end.name().as_ref() == b"extend" => break,
                            Event::Eof => {
                                return Err(ParseError::MissingElement("extend end".to_string()));
                            }
                            _ => {}
                        }
                        inner_buf.clear();
                    }
                    extend = Some(FigureExtend {
                        extend_type,
                        default_x: ext_default_x,
                        default_y: ext_default_y,
                        relative_x: ext_relative_x,
                        relative_y: ext_relative_y,
                        color: ext_color,
                    });
                }
                b"footnote" | b"level" => skip_element(reader, &e)?,
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"extend" => {
                let extend_type = get_attr(&e, "type")?.and_then(|s| parse_start_stop_continue(&s));
                let ext_default_x = get_attr(&e, "default-x")?.and_then(|s| s.parse().ok());
                let ext_default_y = get_attr(&e, "default-y")?.and_then(|s| s.parse().ok());
                let ext_relative_x = get_attr(&e, "relative-x")?.and_then(|s| s.parse().ok());
                let ext_relative_y = get_attr(&e, "relative-y")?.and_then(|s| s.parse().ok());
                let ext_color = get_attr(&e, "color")?;
                extend = Some(FigureExtend {
                    extend_type,
                    default_x: ext_default_x,
                    default_y: ext_default_y,
                    relative_x: ext_relative_x,
                    relative_y: ext_relative_y,
                    color: ext_color,
                });
            }
            Event::End(e) if e.name().as_ref() == b"figure" => break,
            Event::Eof => return Err(ParseError::MissingElement("figure end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Figure {
        prefix,
        figure_number,
        suffix,
        extend,
    })
}

/// Parse a style-text element (text content with print-style attributes).
fn parse_style_text<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    end_tag: &[u8],
) -> Result<StyleText> {
    let font_family = get_attr(start, "font-family")?;
    let font_style = get_attr(start, "font-style")?;
    let font_size = get_attr(start, "font-size")?.and_then(|s| s.parse().ok());
    let font_weight = get_attr(start, "font-weight")?;
    let color = get_attr(start, "color")?;
    let value = read_text(reader, end_tag)?;
    Ok(StyleText {
        value,
        font_family,
        font_style,
        font_size,
        font_weight,
        color,
    })
}

fn parse_yes_no(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

fn parse_start_stop_continue(s: &str) -> Option<StartStopContinue> {
    match s {
        "start" => Some(StartStopContinue::Start),
        "stop" => Some(StartStopContinue::Stop),
        "continue" => Some(StartStopContinue::Continue),
        _ => None,
    }
}

//! Parser for MusicXML `<print>` elements.

use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::parse_defaults::{parse_page_layout, parse_staff_layout, parse_system_layout};
use super::{ParseError, Reader, Result, get_attr, read_text, skip_element};
use crate::model::data::YesNo;
use crate::model::elements::{
    AccidentalText, FormattedTextId, NameDisplay, NameDisplayContent, StaffLayout,
};
use crate::model::print::{MeasureLayout, MeasureNumbering, MeasureNumberingValue, Print};

/// Parse a `<print>` element (Start event).
pub fn parse_print<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Print> {
    let mut buf = Vec::new();

    let staff_spacing = get_attr(start, "staff-spacing")?.and_then(|s| s.parse().ok());
    let new_system = get_attr(start, "new-system")?.and_then(|s| parse_yes_no(&s));
    let new_page = get_attr(start, "new-page")?.and_then(|s| parse_yes_no(&s));
    let blank_page = get_attr(start, "blank-page")?.and_then(|s| s.parse().ok());
    let page_number = get_attr(start, "page-number")?;
    let id = get_attr(start, "id")?;

    let mut page_layout = None;
    let mut system_layout = None;
    let mut staff_layouts: Vec<StaffLayout> = Vec::new();
    let mut measure_layout = None;
    let mut measure_numbering = None;
    let mut part_name_display = None;
    let mut part_abbreviation_display = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"page-layout" => page_layout = Some(parse_page_layout(reader)?),
                b"system-layout" => system_layout = Some(parse_system_layout(reader)?),
                b"staff-layout" => staff_layouts.push(parse_staff_layout(reader, &e)?),
                b"measure-layout" => measure_layout = Some(parse_measure_layout(reader)?),
                b"measure-numbering" => {
                    measure_numbering = Some(parse_measure_numbering(reader, &e)?)
                }
                b"part-name-display" => {
                    part_name_display =
                        Some(parse_name_display(reader, &e, b"part-name-display")?)
                }
                b"part-abbreviation-display" => {
                    part_abbreviation_display = Some(parse_name_display(
                        reader,
                        &e,
                        b"part-abbreviation-display",
                    )?)
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"staff-layout" => {
                    let number = get_attr(&e, "number")?.and_then(|s| s.parse().ok());
                    staff_layouts.push(StaffLayout {
                        number,
                        staff_distance: None,
                    });
                }
                b"measure-layout" => {
                    measure_layout = Some(MeasureLayout {
                        measure_distance: None,
                    });
                }
                b"part-name-display" => {
                    let print_object =
                        get_attr(&e, "print-object")?.and_then(|s| parse_yes_no(&s));
                    part_name_display = Some(NameDisplay {
                        content: Vec::new(),
                        print_object,
                    });
                }
                b"part-abbreviation-display" => {
                    let print_object =
                        get_attr(&e, "print-object")?.and_then(|s| parse_yes_no(&s));
                    part_abbreviation_display = Some(NameDisplay {
                        content: Vec::new(),
                        print_object,
                    });
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"print" => break,
            Event::Eof => return Err(ParseError::MissingElement("print end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Print {
        page_layout,
        system_layout,
        staff_layouts,
        measure_layout,
        measure_numbering,
        part_name_display,
        part_abbreviation_display,
        staff_spacing,
        new_system,
        new_page,
        blank_page,
        page_number,
        id,
    })
}

/// Parse an empty `<print>` element (Empty event — attributes only).
pub fn parse_print_empty(start: &BytesStart) -> Result<Print> {
    let staff_spacing = get_attr(start, "staff-spacing")?.and_then(|s| s.parse().ok());
    let new_system = get_attr(start, "new-system")?.and_then(|s| parse_yes_no(&s));
    let new_page = get_attr(start, "new-page")?.and_then(|s| parse_yes_no(&s));
    let blank_page = get_attr(start, "blank-page")?.and_then(|s| s.parse().ok());
    let page_number = get_attr(start, "page-number")?;
    let id = get_attr(start, "id")?;

    Ok(Print {
        page_layout: None,
        system_layout: None,
        staff_layouts: Vec::new(),
        measure_layout: None,
        measure_numbering: None,
        part_name_display: None,
        part_abbreviation_display: None,
        staff_spacing,
        new_system,
        new_page,
        blank_page,
        page_number,
        id,
    })
}

/// Parse a `<measure-layout>` element.
fn parse_measure_layout<R: BufRead>(reader: &mut Reader<R>) -> Result<MeasureLayout> {
    let mut buf = Vec::new();
    let mut measure_distance = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"measure-distance" => {
                    measure_distance = Some(
                        read_text(reader, b"measure-distance")?
                            .parse()
                            .map_err(|_| {
                                ParseError::ParseNumber("measure-distance".to_string())
                            })?,
                    );
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"measure-layout" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement("measure-layout end".to_string()))
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(MeasureLayout { measure_distance })
}

/// Parse a `<measure-numbering>` element.
fn parse_measure_numbering<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<MeasureNumbering> {
    let system = get_attr(start, "system")?;
    let staff = get_attr(start, "staff")?.and_then(|s| s.parse().ok());
    let multiple_rest_always =
        get_attr(start, "multiple-rest-always")?.and_then(|s| parse_yes_no(&s));
    let multiple_rest_range =
        get_attr(start, "multiple-rest-range")?.and_then(|s| parse_yes_no(&s));
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let font_family = get_attr(start, "font-family")?;
    let font_size = get_attr(start, "font-size")?.and_then(|s| s.parse().ok());
    let font_style = get_attr(start, "font-style")?;
    let font_weight = get_attr(start, "font-weight")?;
    let halign = get_attr(start, "halign")?;
    let valign = get_attr(start, "valign")?;

    let text = read_text(reader, b"measure-numbering")?;
    let value = MeasureNumberingValue::from_str(&text)
        .ok_or_else(|| ParseError::InvalidContent("measure-numbering".to_string(), text))?;

    Ok(MeasureNumbering {
        value,
        system,
        staff,
        multiple_rest_always,
        multiple_rest_range,
        default_x,
        default_y,
        font_family,
        font_size,
        font_style,
        font_weight,
        halign,
        valign,
    })
}

/// Parse a `<part-name-display>` or `<part-abbreviation-display>` element.
fn parse_name_display<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    end_tag: &[u8],
) -> Result<NameDisplay> {
    let mut buf = Vec::new();
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no(&s));
    let mut content = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"display-text" => {
                    let font_family = get_attr(&e, "font-family")?;
                    let font_size = get_attr(&e, "font-size")?.and_then(|s| s.parse().ok());
                    let font_style = get_attr(&e, "font-style")?.and_then(|s| s.parse().ok());
                    let font_weight = get_attr(&e, "font-weight")?.and_then(|s| s.parse().ok());
                    let default_x = get_attr(&e, "default-x")?.and_then(|s| s.parse().ok());
                    let default_y = get_attr(&e, "default-y")?.and_then(|s| s.parse().ok());
                    let halign = get_attr(&e, "halign")?.and_then(|s| s.parse().ok());
                    let valign = get_attr(&e, "valign")?.and_then(|s| s.parse().ok());
                    let justify = get_attr(&e, "justify")?.and_then(|s| s.parse().ok());
                    let id = get_attr(&e, "id")?;
                    let value = read_text(reader, b"display-text")?;
                    content.push(NameDisplayContent::DisplayText(FormattedTextId {
                        value,
                        id,
                        default_x,
                        default_y,
                        font_family,
                        font_size,
                        font_style,
                        font_weight,
                        justify,
                        halign,
                        valign,
                    }));
                }
                b"accidental-text" => {
                    let smufl = get_attr(&e, "smufl")?;
                    let value = read_text(reader, b"accidental-text")?;
                    content.push(NameDisplayContent::AccidentalText(AccidentalText {
                        value,
                        smufl,
                    }));
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == end_tag => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    String::from_utf8_lossy(end_tag).to_string() + " end",
                ))
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(NameDisplay {
        content,
        print_object,
    })
}

fn parse_yes_no(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

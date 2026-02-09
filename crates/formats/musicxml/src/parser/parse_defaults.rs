//! Parsing for `<defaults>` and its children: scaling, layout, appearance, fonts.

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use crate::model::*;
use crate::parser::{ParseError, Result, get_attr, read_text, skip_element};

pub(crate) fn parse_defaults<R: BufRead>(reader: &mut Reader<R>) -> Result<Defaults> {
    let mut buf = Vec::new();
    let mut defaults = Defaults::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"scaling" => defaults.scaling = Some(parse_scaling(reader)?),
                b"page-layout" => defaults.page_layout = Some(parse_page_layout(reader)?),
                b"system-layout" => defaults.system_layout = Some(parse_system_layout(reader)?),
                b"staff-layout" => defaults.staff_layouts.push(parse_staff_layout(reader, &e)?),
                b"appearance" => defaults.appearance = Some(parse_appearance(reader)?),
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"music-font" => defaults.music_font = Some(parse_empty_font_attrs(&e)?),
                b"word-font" => defaults.word_font = Some(parse_empty_font_attrs(&e)?),
                b"lyric-font" => defaults.lyric_fonts.push(parse_lyric_font_attrs(&e)?),
                b"lyric-language" => {
                    defaults
                        .lyric_languages
                        .push(parse_lyric_language_attrs(&e)?);
                }
                b"concert-score" => { /* Phase 24 */ }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"defaults" => break,
            Event::Eof => return Err(ParseError::MissingElement("defaults end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(defaults)
}

fn parse_scaling<R: BufRead>(reader: &mut Reader<R>) -> Result<Scaling> {
    let mut buf = Vec::new();
    let mut millimeters = 0.0;
    let mut tenths = 0.0;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"millimeters" => {
                    millimeters = read_text(reader, b"millimeters")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("millimeters".to_string()))?
                }
                b"tenths" => {
                    tenths = read_text(reader, b"tenths")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("tenths".to_string()))?
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"scaling" => break,
            Event::Eof => return Err(ParseError::MissingElement("scaling end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Scaling {
        millimeters,
        tenths,
    })
}

fn parse_page_layout<R: BufRead>(reader: &mut Reader<R>) -> Result<PageLayout> {
    let mut buf = Vec::new();
    let mut layout = PageLayout::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"page-height" => {
                    layout.page_height = Some(
                        read_text(reader, b"page-height")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("page-height".to_string()))?,
                    )
                }
                b"page-width" => {
                    layout.page_width = Some(
                        read_text(reader, b"page-width")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("page-width".to_string()))?,
                    )
                }
                b"page-margins" => layout.page_margins.push(parse_page_margins(reader, &e)?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"page-layout" => break,
            Event::Eof => return Err(ParseError::MissingElement("page-layout end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(layout)
}

fn parse_page_margins<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<PageMargins> {
    let mut buf = Vec::new();
    let margin_type = get_attr(start, "type")?.map(|s| match s.as_str() {
        "odd" => MarginType::Odd,
        "even" => MarginType::Even,
        _ => MarginType::Both,
    });
    let mut left = 0.0;
    let mut right = 0.0;
    let mut top = 0.0;
    let mut bottom = 0.0;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"left-margin" => {
                    left = read_text(reader, b"left-margin")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("left-margin".to_string()))?
                }
                b"right-margin" => {
                    right = read_text(reader, b"right-margin")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("right-margin".to_string()))?
                }
                b"top-margin" => {
                    top = read_text(reader, b"top-margin")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("top-margin".to_string()))?
                }
                b"bottom-margin" => {
                    bottom = read_text(reader, b"bottom-margin")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("bottom-margin".to_string()))?
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"page-margins" => break,
            Event::Eof => return Err(ParseError::MissingElement("page-margins end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(PageMargins {
        margin_type,
        left_margin: left,
        right_margin: right,
        top_margin: top,
        bottom_margin: bottom,
    })
}

fn parse_system_layout<R: BufRead>(reader: &mut Reader<R>) -> Result<SystemLayout> {
    let mut buf = Vec::new();
    let mut layout = SystemLayout::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"system-margins" => layout.system_margins = Some(parse_system_margins(reader)?),
                b"system-distance" => {
                    layout.system_distance = Some(
                        read_text(reader, b"system-distance")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("system-distance".to_string()))?,
                    )
                }
                b"top-system-distance" => {
                    layout.top_system_distance = Some(
                        read_text(reader, b"top-system-distance")?
                            .parse()
                            .map_err(|_| {
                                ParseError::ParseNumber("top-system-distance".to_string())
                            })?,
                    )
                }
                b"system-dividers" => layout.system_dividers = Some(parse_system_dividers(reader)?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"system-layout" => break,
            Event::Eof => return Err(ParseError::MissingElement("system-layout end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(layout)
}

fn parse_system_margins<R: BufRead>(reader: &mut Reader<R>) -> Result<SystemMargins> {
    let mut buf = Vec::new();
    let mut left = 0.0;
    let mut right = 0.0;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"left-margin" => {
                    left = read_text(reader, b"left-margin")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("left-margin".to_string()))?
                }
                b"right-margin" => {
                    right = read_text(reader, b"right-margin")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("right-margin".to_string()))?
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"system-margins" => break,
            Event::Eof => return Err(ParseError::MissingElement("system-margins end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(SystemMargins {
        left_margin: left,
        right_margin: right,
    })
}

pub(crate) fn parse_staff_layout<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<StaffLayout> {
    let mut buf = Vec::new();
    let number = get_attr(start, "number")?.map(|s| s.parse().unwrap_or(1));
    let mut staff_distance = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"staff-distance" {
                    staff_distance = Some(
                        read_text(reader, b"staff-distance")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staff-distance".to_string()))?,
                    );
                } else {
                    skip_element(reader, &e)?;
                }
            }
            Event::End(e) if e.name().as_ref() == b"staff-layout" => break,
            Event::Eof => return Err(ParseError::MissingElement("staff-layout end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(StaffLayout {
        number,
        staff_distance,
    })
}

fn parse_system_dividers<R: BufRead>(reader: &mut Reader<R>) -> Result<SystemDividers> {
    let mut buf = Vec::new();
    let mut dividers = SystemDividers::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Empty(e) => match e.name().as_ref() {
                b"left-divider" => {
                    dividers.left_divider = Some(parse_empty_print_style_align(&e)?);
                }
                b"right-divider" => {
                    dividers.right_divider = Some(parse_empty_print_style_align(&e)?);
                }
                _ => {}
            },
            Event::Start(e) => {
                skip_element(reader, &e)?;
            }
            Event::End(e) if e.name().as_ref() == b"system-dividers" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    "system-dividers end".to_string(),
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(dividers)
}

fn parse_empty_print_style_align(start: &BytesStart) -> Result<EmptyPrintStyleAlign> {
    let print_object = get_attr(start, "print-object")?.and_then(|s| match s.as_str() {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    });
    Ok(EmptyPrintStyleAlign { print_object })
}

fn parse_appearance<R: BufRead>(reader: &mut Reader<R>) -> Result<Appearance> {
    let mut buf = Vec::new();
    let mut appearance = Appearance::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"line-width" => {
                    let lw_type = get_attr(&e, "type")?.unwrap_or_default();
                    let value: f64 = read_text(reader, b"line-width")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("line-width".to_string()))?;
                    appearance.line_widths.push(LineWidth {
                        line_width_type: lw_type,
                        value,
                    });
                }
                b"note-size" => {
                    let ns_type = get_attr(&e, "type")?.unwrap_or_default();
                    let value: f64 = read_text(reader, b"note-size")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("note-size".to_string()))?;
                    let note_size_type = match ns_type.as_str() {
                        "grace" => NoteSizeType::Grace,
                        "grace-cue" => NoteSizeType::GraceCue,
                        "large" => NoteSizeType::Large,
                        _ => NoteSizeType::Cue,
                    };
                    appearance.note_sizes.push(NoteSize {
                        note_size_type,
                        value,
                    });
                }
                b"distance" => {
                    let d_type = get_attr(&e, "type")?.unwrap_or_default();
                    let value: f64 = read_text(reader, b"distance")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("distance".to_string()))?;
                    appearance.distances.push(Distance {
                        distance_type: d_type,
                        value,
                    });
                }
                b"glyph" => {
                    let g_type = get_attr(&e, "type")?.unwrap_or_default();
                    let value = read_text(reader, b"glyph")?;
                    appearance.glyphs.push(Glyph {
                        glyph_type: g_type,
                        value,
                    });
                }
                b"other-appearance" => {
                    let oa_type = get_attr(&e, "type")?.unwrap_or_default();
                    let value = read_text(reader, b"other-appearance")?;
                    appearance.other_appearances.push(OtherAppearance {
                        appearance_type: oa_type,
                        value,
                    });
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"appearance" => break,
            Event::Eof => return Err(ParseError::MissingElement("appearance end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(appearance)
}

fn parse_empty_font_attrs(start: &BytesStart) -> Result<EmptyFont> {
    Ok(EmptyFont {
        font_family: get_attr(start, "font-family")?,
        font_style: get_attr(start, "font-style")?.and_then(|s| match s.as_str() {
            "italic" => Some(FontStyle::Italic),
            "normal" => Some(FontStyle::Normal),
            _ => None,
        }),
        font_size: get_attr(start, "font-size")?.and_then(|s| parse_font_size_value(&s)),
        font_weight: get_attr(start, "font-weight")?.and_then(|s| match s.as_str() {
            "bold" => Some(FontWeight::Bold),
            "normal" => Some(FontWeight::Normal),
            _ => None,
        }),
    })
}

fn parse_lyric_font_attrs(start: &BytesStart) -> Result<LyricFont> {
    Ok(LyricFont {
        number: get_attr(start, "number")?,
        name: get_attr(start, "name")?,
        font_family: get_attr(start, "font-family")?,
        font_style: get_attr(start, "font-style")?.and_then(|s| match s.as_str() {
            "italic" => Some(FontStyle::Italic),
            "normal" => Some(FontStyle::Normal),
            _ => None,
        }),
        font_size: get_attr(start, "font-size")?.and_then(|s| parse_font_size_value(&s)),
        font_weight: get_attr(start, "font-weight")?.and_then(|s| match s.as_str() {
            "bold" => Some(FontWeight::Bold),
            "normal" => Some(FontWeight::Normal),
            _ => None,
        }),
    })
}

fn parse_lyric_language_attrs(start: &BytesStart) -> Result<LyricLanguage> {
    Ok(LyricLanguage {
        number: get_attr(start, "number")?,
        name: get_attr(start, "name")?,
        lang: get_attr(start, "xml:lang")?.unwrap_or_default(),
    })
}

/// Parse a font-size value that can be either numeric points or a CSS size name.
fn parse_font_size_value(s: &str) -> Option<FontSize> {
    if let Ok(pts) = s.parse::<f64>() {
        Some(FontSize::Points(pts))
    } else if let Ok(css) = s.parse::<CssFontSize>() {
        Some(FontSize::Css(css))
    } else {
        None
    }
}

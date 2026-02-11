//! MusicXML parser for score-partwise and score-timewise documents.
//!
//! This module provides parsing functionality for MusicXML files,
//! converting XML into the intermediate MusicXML model types.
//!
//! Both document formats are supported:
//! - `score-partwise`: Parts contain measures (most common)
//! - `score-timewise`: Measures contain parts (converted to partwise internally)

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::collections::HashMap;
use std::io::BufRead;
use thiserror::Error;

use crate::model::*;

/// Errors that can occur during MusicXML parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    /// XML parsing error from quick-xml.
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),

    /// UTF-8 decoding error.
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// Attribute error from quick-xml.
    #[error("Attribute error: {0}")]
    Attr(#[from] quick_xml::events::attributes::AttrError),

    /// Escape error from quick-xml.
    #[error("Escape error: {0}")]
    Escape(#[from] quick_xml::escape::EscapeError),

    /// Missing required element.
    #[error("Missing required element: {0}")]
    MissingElement(String),

    /// Missing required attribute.
    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),

    /// Invalid attribute value.
    #[error("Invalid attribute value for {0}: {1}")]
    InvalidAttribute(String, String),

    /// Unexpected element.
    #[error("Unexpected element: {0}")]
    UnexpectedElement(String),

    /// Invalid element content.
    #[error("Invalid content in {0}: {1}")]
    InvalidContent(String, String),

    /// Parse error for numeric values.
    #[error("Failed to parse number: {0}")]
    ParseNumber(String),
}

/// Result type for MusicXML parsing operations.
pub type Result<T> = std::result::Result<T, ParseError>;

/// Parse a MusicXML score-partwise document from a string.
pub fn parse_score_partwise(xml: &str) -> Result<ScorePartwise> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    parse_score_partwise_from_reader(&mut reader)
}

/// Parse a MusicXML score-timewise document from a string.
///
/// The timewise document is converted to partwise format internally,
/// as ScorePartwise is the canonical representation.
pub fn parse_score_timewise(xml: &str) -> Result<ScorePartwise> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    parse_score_timewise_from_reader(&mut reader)
}

/// Parse a MusicXML score-partwise document from a reader.
pub fn parse_score_partwise_from_reader<R: BufRead>(
    reader: &mut Reader<R>,
) -> Result<ScorePartwise> {
    let mut buf = Vec::new();
    let mut score = ScorePartwise::default();

    // Find the root element
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) if e.name().as_ref() == b"score-partwise" => {
                // Parse version attribute
                score.version = get_attr(&e, "version")?;
                // Parse content
                parse_score_partwise_content(reader, &mut score)?;
                break;
            }
            Event::Eof => {
                return Err(ParseError::MissingElement("score-partwise".to_string()));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(score)
}

/// Parse a MusicXML score-timewise document from a reader.
///
/// Converts to partwise format during parsing.
pub fn parse_score_timewise_from_reader<R: BufRead>(
    reader: &mut Reader<R>,
) -> Result<ScorePartwise> {
    let mut buf = Vec::new();
    let mut score = ScorePartwise::default();

    // Find the root element
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) if e.name().as_ref() == b"score-timewise" => {
                // Parse version attribute
                score.version = get_attr(&e, "version")?;
                // Parse content and convert to partwise
                parse_score_timewise_content(reader, &mut score)?;
                break;
            }
            Event::Eof => {
                return Err(ParseError::MissingElement("score-timewise".to_string()));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(score)
}

fn parse_score_partwise_content<R: BufRead>(
    reader: &mut Reader<R>,
    score: &mut ScorePartwise,
) -> Result<()> {
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name();
                match name.as_ref() {
                    b"work" => score.work = Some(parse_work(reader)?),
                    b"movement-number" => {
                        score.movement_number = Some(read_text(reader, b"movement-number")?)
                    }
                    b"movement-title" => {
                        score.movement_title = Some(read_text(reader, b"movement-title")?)
                    }
                    b"identification" => score.identification = Some(parse_identification(reader)?),
                    b"defaults" => score.defaults = Some(parse_defaults(reader)?),
                    b"credit" => score.credits.push(parse_credit(reader, &e)?),
                    b"part-list" => score.part_list = parse_part_list(reader)?,
                    b"part" => score.parts.push(parse_part(reader, &e)?),
                    _ => skip_element(reader, &e)?,
                }
            }
            Event::Empty(e) => {
                // Handle empty elements if needed
                let name = e.name();
                if name.as_ref() == b"credit" {
                    score.credits.push(parse_credit_empty(&e)?)
                }
            }
            Event::End(e) if e.name().as_ref() == b"score-partwise" => break,
            Event::Eof => return Err(ParseError::MissingElement("score-partwise end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

/// Parse score-timewise content and convert to partwise format.
///
/// In timewise format, the structure is:
/// ```xml
/// <score-timewise>
///   <part-list>...</part-list>
///   <measure number="1">
///     <part id="P1">...</part>
///     <part id="P2">...</part>
///   </measure>
///   <measure number="2">
///     ...
///   </measure>
/// </score-timewise>
/// ```
///
/// This is converted to partwise format where each part contains its measures.
fn parse_score_timewise_content<R: BufRead>(
    reader: &mut Reader<R>,
    score: &mut ScorePartwise,
) -> Result<()> {
    let mut buf = Vec::new();

    // Collect measures in timewise format, then reorganize to partwise
    // Key: part_id, Value: list of measures for that part
    let mut part_measures: HashMap<String, Vec<Measure>> = HashMap::new();
    // Track part order from part-list
    let mut part_order: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name();
                match name.as_ref() {
                    b"work" => score.work = Some(parse_work(reader)?),
                    b"movement-number" => {
                        score.movement_number = Some(read_text(reader, b"movement-number")?)
                    }
                    b"movement-title" => {
                        score.movement_title = Some(read_text(reader, b"movement-title")?)
                    }
                    b"identification" => score.identification = Some(parse_identification(reader)?),
                    b"defaults" => score.defaults = Some(parse_defaults(reader)?),
                    b"credit" => score.credits.push(parse_credit(reader, &e)?),
                    b"part-list" => {
                        score.part_list = parse_part_list(reader)?;
                        // Extract part IDs in order
                        for item in &score.part_list.items {
                            if let PartListItem::ScorePart(sp) = item {
                                part_order.push(sp.id.clone());
                                part_measures.insert(sp.id.clone(), Vec::new());
                            }
                        }
                    }
                    b"measure" => {
                        parse_timewise_measure(reader, &e, &mut part_measures)?;
                    }
                    _ => skip_element(reader, &e)?,
                }
            }
            Event::Empty(e) => {
                let name = e.name();
                if name.as_ref() == b"credit" {
                    score.credits.push(parse_credit_empty(&e)?)
                }
            }
            Event::End(e) if e.name().as_ref() == b"score-timewise" => break,
            Event::Eof => return Err(ParseError::MissingElement("score-timewise end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    // Convert collected timewise data to partwise format
    for part_id in part_order {
        let measures = part_measures.remove(&part_id).unwrap_or_default();
        score.parts.push(Part {
            id: part_id,
            measures,
        });
    }

    Ok(())
}

/// Parse a single measure in timewise format.
///
/// A timewise measure contains parts as children:
/// ```xml
/// <measure number="1" width="200">
///   <part id="P1">
///     <note>...</note>
///   </part>
///   <part id="P2">
///     <note>...</note>
///   </part>
/// </measure>
/// ```
fn parse_timewise_measure<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    part_measures: &mut HashMap<String, Vec<Measure>>,
) -> Result<()> {
    let mut buf = Vec::new();

    // Parse measure attributes
    let measure_number = get_attr_required(start, "number")?;
    let measure_text = get_attr(start, "text")?;
    let implicit = get_attr(start, "implicit")?.and_then(|s| match s.as_str() {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    });
    let non_controlling = get_attr(start, "non-controlling")?.and_then(|s| match s.as_str() {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    });
    let width = get_attr(start, "width")?.and_then(|s| s.parse().ok());
    let measure_id = get_attr(start, "id")?;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"part" {
                    let part_id = get_attr_required(&e, "id")?;
                    let content = parse_timewise_part_content(reader)?;

                    // Create measure for this part
                    let measure = Measure {
                        number: measure_number.clone(),
                        text: measure_text.clone(),
                        implicit,
                        non_controlling,
                        width,
                        id: measure_id.clone(),
                        content,
                    };

                    // Add to part's measures
                    if let Some(measures) = part_measures.get_mut(&part_id) {
                        measures.push(measure);
                    } else {
                        // Part not in part-list, create entry
                        part_measures.insert(part_id, vec![measure]);
                    }
                } else {
                    skip_element(reader, &e)?;
                }
            }
            Event::Empty(e) => {
                // Handle empty <part id="..."/> elements
                if e.name().as_ref() == b"part" {
                    let part_id = get_attr_required(&e, "id")?;
                    let measure = Measure {
                        number: measure_number.clone(),
                        text: measure_text.clone(),
                        implicit,
                        non_controlling,
                        width,
                        id: measure_id.clone(),
                        content: Vec::new(),
                    };

                    if let Some(measures) = part_measures.get_mut(&part_id) {
                        measures.push(measure);
                    } else {
                        part_measures.insert(part_id, vec![measure]);
                    }
                }
            }
            Event::End(e) if e.name().as_ref() == b"measure" => break,
            Event::Eof => return Err(ParseError::MissingElement("measure end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

/// Parse the content of a part element within a timewise measure.
///
/// This is the same content as in partwise measure elements:
/// notes, rests, attributes, directions, etc.
fn parse_timewise_part_content<R: BufRead>(reader: &mut Reader<R>) -> Result<Vec<MeasureContent>> {
    let mut buf = Vec::new();
    let mut content = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"note" => content.push(MeasureContent::Note(Box::new(parse_note(reader, &e)?))),
                b"backup" => content.push(MeasureContent::Backup(Box::new(parse_backup(reader)?))),
                b"forward" => {
                    content.push(MeasureContent::Forward(Box::new(parse_forward(reader)?)))
                }
                b"attributes" => content.push(MeasureContent::Attributes(Box::new(
                    parse_attributes(reader)?,
                ))),
                b"direction" => content.push(MeasureContent::Direction(Box::new(parse_direction(
                    reader, &e,
                )?))),
                b"harmony" => content.push(MeasureContent::Harmony(Box::new(parse_harmony(
                    reader, &e,
                )?))),
                b"figured-bass" => content.push(MeasureContent::FiguredBass(Box::new(
                    parse_figured_bass(reader, &e)?,
                ))),
                b"sound" => content.push(MeasureContent::Sound(Box::new(parse_sound_full(
                    reader, &e,
                )?))),
                b"listening" => content.push(MeasureContent::Listening(Box::new(parse_listening(
                    reader,
                )?))),
                b"barline" => {
                    content.push(MeasureContent::Barline(Box::new(parse_barline(
                        reader, &e,
                    )?)));
                }
                b"grouping" => content.push(MeasureContent::Grouping(Box::new(parse_grouping(
                    reader, &e,
                )?))),
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"barline" => {
                    content.push(MeasureContent::Barline(Box::new(parse_barline_empty(&e)?)));
                }
                b"sound" => {
                    content.push(MeasureContent::Sound(Box::new(parse_sound_attrs(&e)?)));
                }
                b"grouping" => {
                    content.push(MeasureContent::Grouping(Box::new(parse_grouping_empty(
                        &e,
                    )?)));
                }
                b"link" => {
                    content.push(MeasureContent::Link(Box::new(parse_link_empty(&e)?)));
                }
                b"bookmark" => {
                    content.push(MeasureContent::Bookmark(Box::new(parse_bookmark_empty(
                        &e,
                    )?)));
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"part" => break,
            Event::Eof => return Err(ParseError::MissingElement("part end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(content)
}

fn parse_work<R: BufRead>(reader: &mut Reader<R>) -> Result<Work> {
    let mut buf = Vec::new();
    let mut work = Work::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"work-number" => work.work_number = Some(read_text(reader, b"work-number")?),
                b"work-title" => work.work_title = Some(read_text(reader, b"work-title")?),
                b"opus" => {
                    work.opus = Some(parse_opus(&e)?);
                    skip_element(reader, &e)?;
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => {
                if e.name().as_ref() == b"opus" {
                    work.opus = Some(parse_opus(&e)?);
                }
            }
            Event::End(e) if e.name().as_ref() == b"work" => break,
            Event::Eof => return Err(ParseError::MissingElement("work end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(work)
}

fn parse_opus(e: &BytesStart) -> Result<Opus> {
    Ok(Opus {
        href: get_attr_required(e, "xlink:href")?,
        xlink_type: get_attr(e, "xlink:type")?,
    })
}

fn parse_identification<R: BufRead>(reader: &mut Reader<R>) -> Result<Identification> {
    let mut buf = Vec::new();
    let mut ident = Identification::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"creator" => ident
                    .creators
                    .push(parse_typed_text(reader, &e, b"creator")?),
                b"rights" => ident.rights.push(parse_typed_text(reader, &e, b"rights")?),
                b"encoding" => ident.encoding = Some(parse_encoding(reader)?),
                b"source" => ident.source = Some(read_text(reader, b"source")?),
                b"relation" => ident
                    .relations
                    .push(parse_typed_text(reader, &e, b"relation")?),
                b"miscellaneous" => ident.miscellaneous = Some(parse_miscellaneous(reader)?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"identification" => break,
            Event::Eof => return Err(ParseError::MissingElement("identification end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(ident)
}

fn parse_typed_text<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    end_tag: &[u8],
) -> Result<TypedText> {
    let text_type = get_attr(start, "type")?;
    let value = read_text(reader, end_tag)?;
    Ok(TypedText { text_type, value })
}

fn parse_encoding<R: BufRead>(reader: &mut Reader<R>) -> Result<Encoding> {
    let mut buf = Vec::new();
    let mut encoding = Encoding::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"encoding-date" => {
                    encoding.encoding_date = Some(read_text(reader, b"encoding-date")?)
                }
                b"encoder" => encoding
                    .encoders
                    .push(parse_typed_text(reader, &e, b"encoder")?),
                b"software" => encoding.software.push(read_text(reader, b"software")?),
                b"encoding-description" => encoding
                    .encoding_descriptions
                    .push(read_text(reader, b"encoding-description")?),
                b"supports" => {
                    encoding.supports.push(parse_supports(&e)?);
                    skip_element(reader, &e)?;
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => {
                if e.name().as_ref() == b"supports" {
                    encoding.supports.push(parse_supports(&e)?);
                }
            }
            Event::End(e) if e.name().as_ref() == b"encoding" => break,
            Event::Eof => return Err(ParseError::MissingElement("encoding end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(encoding)
}

fn parse_supports(e: &BytesStart) -> Result<Supports> {
    Ok(Supports {
        element: get_attr_required(e, "element")?,
        support_type: parse_yes_no(&get_attr_required(e, "type")?)?,
        attribute: get_attr(e, "attribute")?,
        value: get_attr(e, "value")?,
    })
}

fn parse_miscellaneous<R: BufRead>(reader: &mut Reader<R>) -> Result<Miscellaneous> {
    let mut buf = Vec::new();
    let mut misc = Miscellaneous::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"miscellaneous-field" {
                    let name = get_attr_required(&e, "name")?;
                    let value = read_text(reader, b"miscellaneous-field")?;
                    misc.fields.push(MiscellaneousField { name, value });
                } else {
                    skip_element(reader, &e)?;
                }
            }
            Event::End(e) if e.name().as_ref() == b"miscellaneous" => break,
            Event::Eof => return Err(ParseError::MissingElement("miscellaneous end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(misc)
}

fn parse_credit<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Credit> {
    let mut buf = Vec::new();
    let mut credit = Credit {
        page: get_attr(start, "page")?.map(|s| s.parse().unwrap_or(1)),
        ..Default::default()
    };

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"credit-type" => credit.credit_types.push(read_text(reader, b"credit-type")?),
                b"credit-words" => {
                    let words = parse_credit_words(reader, &e)?;
                    if let Some(CreditContent::Words(ref mut cw)) = credit.content {
                        cw.words.push(words);
                    } else {
                        credit.content =
                            Some(CreditContent::Words(CreditWords { words: vec![words] }));
                    }
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"credit" => break,
            Event::Eof => return Err(ParseError::MissingElement("credit end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(credit)
}

fn parse_credit_empty(start: &BytesStart) -> Result<Credit> {
    Ok(Credit {
        page: get_attr(start, "page")?.map(|s| s.parse().unwrap_or(1)),
        ..Default::default()
    })
}

fn parse_credit_words<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<FormattedTextId> {
    let value = read_text(reader, b"credit-words")?;
    Ok(FormattedTextId {
        value,
        id: get_attr(start, "id")?,
        default_x: get_attr(start, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(start, "default-y")?.and_then(|s| s.parse().ok()),
        font_family: get_attr(start, "font-family")?,
        font_size: get_attr(start, "font-size")?.and_then(|s| s.parse().ok().map(FontSize::Points)),
        font_style: get_attr(start, "font-style")?.and_then(|s| match s.as_str() {
            "italic" => Some(FontStyle::Italic),
            "normal" => Some(FontStyle::Normal),
            _ => None,
        }),
        font_weight: get_attr(start, "font-weight")?.and_then(|s| match s.as_str() {
            "bold" => Some(FontWeight::Bold),
            "normal" => Some(FontWeight::Normal),
            _ => None,
        }),
        justify: get_attr(start, "justify")?.and_then(|s| match s.as_str() {
            "left" => Some(LeftCenterRight::Left),
            "center" => Some(LeftCenterRight::Center),
            "right" => Some(LeftCenterRight::Right),
            _ => None,
        }),
        halign: get_attr(start, "halign")?.and_then(|s| match s.as_str() {
            "left" => Some(LeftCenterRight::Left),
            "center" => Some(LeftCenterRight::Center),
            "right" => Some(LeftCenterRight::Right),
            _ => None,
        }),
        valign: get_attr(start, "valign")?.and_then(|s| match s.as_str() {
            "top" => Some(Valign::Top),
            "middle" => Some(Valign::Middle),
            "bottom" => Some(Valign::Bottom),
            "baseline" => Some(Valign::Baseline),
            _ => None,
        }),
    })
}

fn parse_part_list<R: BufRead>(reader: &mut Reader<R>) -> Result<PartList> {
    let mut buf = Vec::new();
    let mut items = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"part-group" => items.push(PartListItem::PartGroup(Box::new(parse_part_group(
                    reader, &e,
                )?))),
                b"score-part" => items.push(PartListItem::ScorePart(Box::new(parse_score_part(
                    reader, &e,
                )?))),
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => {
                if e.name().as_ref() == b"part-group" {
                    items.push(PartListItem::PartGroup(Box::new(parse_part_group_empty(
                        &e,
                    )?)));
                }
            }
            Event::End(e) if e.name().as_ref() == b"part-list" => break,
            Event::Eof => return Err(ParseError::MissingElement("part-list end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(PartList { items })
}

fn parse_part_group<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<PartGroup> {
    let mut buf = Vec::new();
    let mut group = PartGroup {
        group_type: parse_start_stop(&get_attr_required(start, "type")?)?,
        number: get_attr(start, "number")?,
        group_name: None,
        group_name_display: None,
        group_abbreviation: None,
        group_abbreviation_display: None,
        group_symbol: None,
        group_barline: None,
        group_time: None,
    };

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"group-name" => group.group_name = Some(read_text(reader, b"group-name")?),
                b"group-name-display" => {
                    group.group_name_display = Some(parse_print::parse_name_display(
                        reader,
                        &e,
                        b"group-name-display",
                    )?);
                }
                b"group-abbreviation" => {
                    group.group_abbreviation = Some(read_text(reader, b"group-abbreviation")?)
                }
                b"group-abbreviation-display" => {
                    group.group_abbreviation_display = Some(parse_print::parse_name_display(
                        reader,
                        &e,
                        b"group-abbreviation-display",
                    )?);
                }
                b"group-symbol" => {
                    let value = read_text(reader, b"group-symbol")?;
                    group.group_symbol = Some(GroupSymbolValue {
                        value: match value.as_str() {
                            "brace" => GroupSymbol::Brace,
                            "bracket" => GroupSymbol::Bracket,
                            "line" => GroupSymbol::Line,
                            "square" => GroupSymbol::Square,
                            _ => GroupSymbol::None,
                        },
                        default_x: None,
                        relative_x: None,
                        color: None,
                    });
                }
                b"group-barline" => {
                    let value = read_text(reader, b"group-barline")?;
                    group.group_barline = Some(GroupBarlineValue {
                        value: match value.as_str() {
                            "yes" => GroupBarline::Yes,
                            "no" => GroupBarline::No,
                            "Mensurstrich" => GroupBarline::Mensurstrich,
                            _ => GroupBarline::Yes,
                        },
                        color: None,
                    });
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => {
                if e.name().as_ref() == b"group-time" {
                    group.group_time = Some(Empty);
                }
            }
            Event::End(e) if e.name().as_ref() == b"part-group" => break,
            Event::Eof => return Err(ParseError::MissingElement("part-group end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(group)
}

fn parse_part_group_empty(start: &BytesStart) -> Result<PartGroup> {
    Ok(PartGroup {
        group_type: parse_start_stop(&get_attr_required(start, "type")?)?,
        number: get_attr(start, "number")?,
        group_name: None,
        group_name_display: None,
        group_abbreviation: None,
        group_abbreviation_display: None,
        group_symbol: None,
        group_barline: None,
        group_time: None,
    })
}

fn parse_score_part<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<ScorePart> {
    let mut buf = Vec::new();
    let id = get_attr_required(start, "id")?;
    let mut part = ScorePart::new(&id, "");

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"part-name" => part.part_name = parse_part_name(reader, &e)?,
                b"part-abbreviation" => part.part_abbreviation = Some(parse_part_name(reader, &e)?),
                b"score-instrument" => part
                    .score_instruments
                    .push(parse_score_instrument(reader, &e)?),
                b"midi-device" => part
                    .midi_assignments
                    .push(MidiAssignment::MidiDevice(parse_midi_device(reader, &e)?)),
                b"midi-instrument" => part.midi_assignments.push(MidiAssignment::MidiInstrument(
                    parse_midi_instrument(reader, &e)?,
                )),
                b"part-name-display" => {
                    part.part_name_display = Some(parse_print::parse_name_display(
                        reader,
                        &e,
                        b"part-name-display",
                    )?);
                }
                b"part-abbreviation-display" => {
                    part.part_abbreviation_display = Some(parse_print::parse_name_display(
                        reader,
                        &e,
                        b"part-abbreviation-display",
                    )?);
                }
                b"player" => part
                    .players
                    .push(parse_part_list::parse_player(reader, &e)?),
                b"part-link" => part
                    .part_links
                    .push(parse_part_list::parse_part_link(reader, &e)?),
                b"group" => part.groups.push(read_text(reader, b"group")?),
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"midi-device" => part
                    .midi_assignments
                    .push(MidiAssignment::MidiDevice(parse_midi_device_empty(&e)?)),
                b"midi-instrument" => part.midi_assignments.push(MidiAssignment::MidiInstrument(
                    parse_midi_instrument_empty(&e)?,
                )),
                b"part-name-display" => {
                    part.part_name_display = Some(NameDisplay {
                        print_object: get_attr(&e, "print-object")?.and_then(|s| {
                            match s.as_str() {
                                "yes" => Some(YesNo::Yes),
                                "no" => Some(YesNo::No),
                                _ => None,
                            }
                        }),
                        content: Vec::new(),
                    });
                }
                b"part-abbreviation-display" => {
                    part.part_abbreviation_display = Some(NameDisplay {
                        print_object: get_attr(&e, "print-object")?.and_then(|s| {
                            match s.as_str() {
                                "yes" => Some(YesNo::Yes),
                                "no" => Some(YesNo::No),
                                _ => None,
                            }
                        }),
                        content: Vec::new(),
                    });
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"score-part" => break,
            Event::Eof => return Err(ParseError::MissingElement("score-part end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(part)
}

fn parse_part_name<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<PartName> {
    let value = read_text_for_tag(reader, start.name().as_ref())?;
    Ok(PartName {
        value,
        print_object: get_attr(start, "print-object")?.and_then(|s| match s.as_str() {
            "yes" => Some(YesNo::Yes),
            "no" => Some(YesNo::No),
            _ => None,
        }),
        ..Default::default()
    })
}

fn parse_score_instrument<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<ScoreInstrument> {
    let mut buf = Vec::new();
    let id = get_attr_required(start, "id")?;
    let mut instrument = ScoreInstrument {
        id,
        instrument_name: String::new(),
        instrument_abbreviation: None,
        instrument_sound: None,
        solo: None,
        ensemble: None,
        virtual_instrument: None,
    };

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"instrument-name" => {
                    instrument.instrument_name = read_text(reader, b"instrument-name")?
                }
                b"instrument-abbreviation" => {
                    instrument.instrument_abbreviation =
                        Some(read_text(reader, b"instrument-abbreviation")?)
                }
                b"instrument-sound" => {
                    instrument.instrument_sound = Some(read_text(reader, b"instrument-sound")?)
                }
                b"virtual-instrument" => {
                    instrument.virtual_instrument = Some(parse_virtual_instrument(reader)?)
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"solo" => instrument.solo = Some(true),
                b"ensemble" => instrument.ensemble = Some(Ensemble { value: None }),
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"score-instrument" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    "score-instrument end".to_string(),
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(instrument)
}

fn parse_virtual_instrument<R: BufRead>(reader: &mut Reader<R>) -> Result<VirtualInstrument> {
    let mut buf = Vec::new();
    let mut vi = VirtualInstrument::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"virtual-library" => {
                    vi.virtual_library = Some(read_text(reader, b"virtual-library")?)
                }
                b"virtual-name" => vi.virtual_name = Some(read_text(reader, b"virtual-name")?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"virtual-instrument" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    "virtual-instrument end".to_string(),
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(vi)
}

fn parse_midi_device<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<MidiDevice> {
    let value = read_text_for_tag(reader, start.name().as_ref()).ok();
    Ok(MidiDevice {
        value: if value.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
            None
        } else {
            value
        },
        port: get_attr(start, "port")?.and_then(|s| s.parse().ok()),
        id: get_attr(start, "id")?,
    })
}

fn parse_midi_device_empty(start: &BytesStart) -> Result<MidiDevice> {
    Ok(MidiDevice {
        value: None,
        port: get_attr(start, "port")?.and_then(|s| s.parse().ok()),
        id: get_attr(start, "id")?,
    })
}

fn parse_midi_instrument<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<MidiInstrument> {
    let mut buf = Vec::new();
    let id = get_attr_required(start, "id")?;
    let mut midi = MidiInstrument {
        id,
        midi_channel: None,
        midi_name: None,
        midi_bank: None,
        midi_program: None,
        midi_unpitched: None,
        volume: None,
        pan: None,
        elevation: None,
    };

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"midi-channel" => {
                    midi.midi_channel = Some(
                        read_text(reader, b"midi-channel")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("midi-channel".to_string()))?,
                    )
                }
                b"midi-name" => midi.midi_name = Some(read_text(reader, b"midi-name")?),
                b"midi-bank" => {
                    midi.midi_bank = Some(
                        read_text(reader, b"midi-bank")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("midi-bank".to_string()))?,
                    )
                }
                b"midi-program" => {
                    midi.midi_program = Some(
                        read_text(reader, b"midi-program")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("midi-program".to_string()))?,
                    )
                }
                b"midi-unpitched" => {
                    midi.midi_unpitched = Some(
                        read_text(reader, b"midi-unpitched")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("midi-unpitched".to_string()))?,
                    )
                }
                b"volume" => {
                    midi.volume = Some(
                        read_text(reader, b"volume")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("volume".to_string()))?,
                    )
                }
                b"pan" => {
                    midi.pan = Some(
                        read_text(reader, b"pan")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("pan".to_string()))?,
                    )
                }
                b"elevation" => {
                    midi.elevation = Some(
                        read_text(reader, b"elevation")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("elevation".to_string()))?,
                    )
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"midi-instrument" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    "midi-instrument end".to_string(),
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(midi)
}

fn parse_midi_instrument_empty(start: &BytesStart) -> Result<MidiInstrument> {
    Ok(MidiInstrument {
        id: get_attr_required(start, "id")?,
        midi_channel: None,
        midi_name: None,
        midi_bank: None,
        midi_program: None,
        midi_unpitched: None,
        volume: None,
        pan: None,
        elevation: None,
    })
}

fn parse_part<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Part> {
    let mut buf = Vec::new();
    let id = get_attr_required(start, "id")?;
    let mut part = Part::new(&id);

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"measure" {
                    part.measures.push(parse_measure(reader, &e)?);
                } else {
                    skip_element(reader, &e)?;
                }
            }
            Event::End(e) if e.name().as_ref() == b"part" => break,
            Event::Eof => return Err(ParseError::MissingElement("part end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(part)
}

fn parse_measure<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Measure> {
    let mut buf = Vec::new();
    let mut measure = Measure {
        number: get_attr_required(start, "number")?,
        text: get_attr(start, "text")?,
        implicit: get_attr(start, "implicit")?.and_then(|s| match s.as_str() {
            "yes" => Some(YesNo::Yes),
            "no" => Some(YesNo::No),
            _ => None,
        }),
        non_controlling: get_attr(start, "non-controlling")?.and_then(|s| match s.as_str() {
            "yes" => Some(YesNo::Yes),
            "no" => Some(YesNo::No),
            _ => None,
        }),
        width: get_attr(start, "width")?.and_then(|s| s.parse().ok()),
        id: get_attr(start, "id")?,
        content: Vec::new(),
    };

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"note" => measure
                        .content
                        .push(MeasureContent::Note(Box::new(parse_note(reader, &e)?))),
                    b"backup" => measure
                        .content
                        .push(MeasureContent::Backup(Box::new(parse_backup(reader)?))),
                    b"forward" => measure
                        .content
                        .push(MeasureContent::Forward(Box::new(parse_forward(reader)?))),
                    b"attributes" => measure.content.push(MeasureContent::Attributes(Box::new(
                        parse_attributes(reader)?,
                    ))),
                    b"direction" => {
                        measure
                            .content
                            .push(MeasureContent::Direction(Box::new(parse_direction(
                                reader, &e,
                            )?)))
                    }
                    b"harmony" => {
                        measure
                            .content
                            .push(MeasureContent::Harmony(Box::new(parse_harmony(
                                reader, &e,
                            )?)))
                    }
                    b"figured-bass" => measure.content.push(MeasureContent::FiguredBass(Box::new(
                        parse_figured_bass(reader, &e)?,
                    ))),
                    b"print" => measure
                        .content
                        .push(MeasureContent::Print(Box::new(parse_print(reader, &e)?))),
                    b"sound" => {
                        measure
                            .content
                            .push(MeasureContent::Sound(Box::new(parse_sound_full(
                                reader, &e,
                            )?)))
                    }
                    b"listening" => {
                        measure
                            .content
                            .push(MeasureContent::Listening(Box::new(parse_listening(
                                reader,
                            )?)))
                    }
                    b"barline" => {
                        measure
                            .content
                            .push(MeasureContent::Barline(Box::new(parse_barline(
                                reader, &e,
                            )?)));
                    }
                    b"grouping" => {
                        measure
                            .content
                            .push(MeasureContent::Grouping(Box::new(parse_grouping(
                                reader, &e,
                            )?)))
                    }
                    _ => skip_element(reader, &e)?,
                }
            }
            Event::Empty(e) => {
                match e.name().as_ref() {
                    b"barline" => measure
                        .content
                        .push(MeasureContent::Barline(Box::new(parse_barline_empty(&e)?))),
                    b"print" => measure
                        .content
                        .push(MeasureContent::Print(Box::new(parse_print_empty(&e)?))),
                    b"sound" => measure
                        .content
                        .push(MeasureContent::Sound(Box::new(parse_sound_attrs(&e)?))),
                    b"grouping" => measure.content.push(MeasureContent::Grouping(Box::new(
                        parse_grouping_empty(&e)?,
                    ))),
                    b"link" => measure
                        .content
                        .push(MeasureContent::Link(Box::new(parse_link_empty(&e)?))),
                    b"bookmark" => measure.content.push(MeasureContent::Bookmark(Box::new(
                        parse_bookmark_empty(&e)?,
                    ))),
                    _ => {}
                }
            }
            Event::End(e) if e.name().as_ref() == b"measure" => break,
            Event::Eof => return Err(ParseError::MissingElement("measure end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(measure)
}

// Include submodule parsers
mod parse_attributes;
mod parse_defaults;
mod parse_direction;
mod parse_figured_bass;
mod parse_harmony;
mod parse_listening;
mod parse_metronome;
mod parse_notations;
mod parse_note;
mod parse_part_list;
mod parse_print;
mod parse_technical;

use parse_attributes::parse_attributes;
use parse_defaults::parse_defaults;
use parse_direction::{parse_direction, parse_sound_attrs, parse_sound_full};
use parse_figured_bass::parse_figured_bass;
use parse_harmony::parse_harmony;
use parse_listening::{
    parse_bookmark_empty, parse_grouping, parse_grouping_empty, parse_link_empty, parse_listening,
};
use parse_note::{parse_backup, parse_forward, parse_note};
use parse_print::{parse_print, parse_print_empty};

// ============================================================================
// Helper Functions
// ============================================================================

/// Get an optional attribute value.
fn get_attr(e: &BytesStart, name: &str) -> Result<Option<String>> {
    for attr in e.attributes() {
        let attr = attr?;
        if attr.key.as_ref() == name.as_bytes() {
            return Ok(Some(std::str::from_utf8(&attr.value)?.to_string()));
        }
    }
    Ok(None)
}

/// Get a required attribute value.
fn get_attr_required(e: &BytesStart, name: &str) -> Result<String> {
    get_attr(e, name)?.ok_or_else(|| ParseError::MissingAttribute(name.to_string()))
}

/// Read text content until the end tag.
fn read_text<R: BufRead>(reader: &mut Reader<R>, end_tag: &[u8]) -> Result<String> {
    let mut buf = Vec::new();
    let mut text = String::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Text(e) => {
                let raw = std::str::from_utf8(&e)?;
                text.push_str(&quick_xml::escape::unescape(raw)?);
            }
            Event::CData(e) => text.push_str(std::str::from_utf8(&e)?),
            Event::End(e) if e.name().as_ref() == end_tag => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(format!(
                    "{} end",
                    String::from_utf8_lossy(end_tag)
                )));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(text)
}

/// Read text content for a dynamically named tag.
fn read_text_for_tag<R: BufRead>(reader: &mut Reader<R>, end_tag: &[u8]) -> Result<String> {
    read_text(reader, end_tag)
}

fn parse_yes_no_opt(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

/// Read past all remaining content until a matching end tag.
fn skip_to_end_tag<R: BufRead>(reader: &mut Reader<R>, tag: &[u8]) -> Result<()> {
    let mut buf = Vec::new();
    let mut depth = 1u32;
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) if e.name().as_ref() == tag => depth += 1,
            Event::End(e) if e.name().as_ref() == tag => {
                depth -= 1;
                if depth == 0 {
                    return Ok(());
                }
            }
            Event::Eof => {
                return Err(ParseError::MissingElement(format!(
                    "{} end",
                    String::from_utf8_lossy(tag)
                )));
            }
            _ => {}
        }
        buf.clear();
    }
}

/// Parse barline attributes from an element start/empty tag.
fn parse_barline_attrs(e: &BytesStart) -> Result<crate::model::elements::Barline> {
    use crate::model::elements::{Barline, BarlineLocation};
    let location = get_attr(e, "location")?.and_then(|s| match s.to_lowercase().as_str() {
        "left" => Some(BarlineLocation::Left),
        "right" => Some(BarlineLocation::Right),
        "middle" => Some(BarlineLocation::Middle),
        _ => None,
    });
    Ok(Barline {
        location,
        segno_attr: get_attr(e, "segno")?,
        coda_attr: get_attr(e, "coda")?,
        divisions: get_attr(e, "divisions")?.and_then(|s| s.parse().ok()),
        ..Barline::default()
    })
}

/// Parse barline from an empty element (attributes only).
fn parse_barline_empty(e: &BytesStart) -> Result<crate::model::elements::Barline> {
    parse_barline_attrs(e)
}

/// Parse barline element with all XSD children.
///
/// XSD sequence: bar-style, editorial, wavy-line, segno, coda,
/// fermata (0-2), ending, repeat.
fn parse_barline<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<crate::model::elements::Barline> {
    use crate::model::StartStopDiscontinue;
    use crate::model::elements::{BackwardForward, BarStyle, Ending, Repeat, Winged};
    let mut barline = parse_barline_attrs(start)?;
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"bar-style" => {
                    let text = read_text(reader, b"bar-style")?;
                    if let Some(style) = BarStyle::from_musicxml_str(&text) {
                        barline.bar_style = Some(style);
                    }
                }
                b"footnote" => {
                    barline.footnote =
                        Some(parse_note::parse_formatted_text(reader, &e, b"footnote")?);
                }
                b"level" => {
                    barline.level = Some(parse_note::parse_level(reader, &e)?);
                }
                b"fermata" => {
                    barline
                        .fermatas
                        .push(parse_notations::parse_fermata_start(reader, &e)?);
                }
                b"ending" => {
                    let text = read_text(reader, b"ending")?;
                    barline.ending = Some(Ending {
                        number: get_attr_required(&e, "number")?,
                        ending_type: match get_attr_required(&e, "type")?.as_str() {
                            "start" => StartStopDiscontinue::Start,
                            "stop" => StartStopDiscontinue::Stop,
                            "discontinue" => StartStopDiscontinue::Discontinue,
                            s => {
                                return Err(ParseError::InvalidAttribute(
                                    "type".to_string(),
                                    s.to_string(),
                                ));
                            }
                        },
                        text: if text.is_empty() { None } else { Some(text) },
                        default_y: get_attr(&e, "default-y")?.and_then(|s| s.parse().ok()),
                        end_length: get_attr(&e, "end-length")?.and_then(|s| s.parse().ok()),
                        print_object: get_attr(&e, "print-object")?
                            .and_then(|s| parse_yes_no_opt(&s)),
                        default_x: get_attr(&e, "default-x")?.and_then(|s| s.parse().ok()),
                        text_x: get_attr(&e, "text-x")?.and_then(|s| s.parse().ok()),
                        text_y: get_attr(&e, "text-y")?.and_then(|s| s.parse().ok()),
                    });
                }
                b"segno" => {
                    barline.segno = Some(parse_direction::parse_segno(&e)?);
                    skip_to_end_tag(reader, b"segno")?;
                }
                b"coda" => {
                    barline.coda = Some(parse_direction::parse_coda(&e)?);
                    skip_to_end_tag(reader, b"coda")?;
                }
                b"wavy-line" => {
                    barline.wavy_line = Some(parse_note::parse_wavy_line(&e)?);
                    skip_to_end_tag(reader, b"wavy-line")?;
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"repeat" => {
                    barline.repeat = Some(Repeat {
                        direction: match get_attr_required(&e, "direction")?.as_str() {
                            "forward" => BackwardForward::Forward,
                            "backward" => BackwardForward::Backward,
                            s => {
                                return Err(ParseError::InvalidAttribute(
                                    "direction".to_string(),
                                    s.to_string(),
                                ));
                            }
                        },
                        times: get_attr(&e, "times")?.and_then(|s| s.parse().ok()),
                        after_jump: get_attr(&e, "after-jump")?.and_then(|s| parse_yes_no_opt(&s)),
                        winged: get_attr(&e, "winged")?.and_then(|s| match s.as_str() {
                            "none" => Some(Winged::None),
                            "straight" => Some(Winged::Straight),
                            "curved" => Some(Winged::Curved),
                            "double-straight" => Some(Winged::DoubleStraight),
                            "double-curved" => Some(Winged::DoubleCurved),
                            _ => None,
                        }),
                    });
                }
                b"fermata" => {
                    barline
                        .fermatas
                        .push(parse_notations::parse_fermata_empty(&e)?);
                }
                b"ending" => {
                    barline.ending = Some(Ending {
                        number: get_attr_required(&e, "number")?,
                        ending_type: match get_attr_required(&e, "type")?.as_str() {
                            "start" => StartStopDiscontinue::Start,
                            "stop" => StartStopDiscontinue::Stop,
                            "discontinue" => StartStopDiscontinue::Discontinue,
                            s => {
                                return Err(ParseError::InvalidAttribute(
                                    "type".to_string(),
                                    s.to_string(),
                                ));
                            }
                        },
                        text: None,
                        default_y: get_attr(&e, "default-y")?.and_then(|s| s.parse().ok()),
                        end_length: get_attr(&e, "end-length")?.and_then(|s| s.parse().ok()),
                        print_object: get_attr(&e, "print-object")?
                            .and_then(|s| parse_yes_no_opt(&s)),
                        default_x: get_attr(&e, "default-x")?.and_then(|s| s.parse().ok()),
                        text_x: get_attr(&e, "text-x")?.and_then(|s| s.parse().ok()),
                        text_y: get_attr(&e, "text-y")?.and_then(|s| s.parse().ok()),
                    });
                }
                b"segno" => {
                    barline.segno = Some(parse_direction::parse_segno(&e)?);
                }
                b"coda" => {
                    barline.coda = Some(parse_direction::parse_coda(&e)?);
                }
                b"wavy-line" => {
                    barline.wavy_line = Some(parse_note::parse_wavy_line(&e)?);
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"barline" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement("barline end".to_string()));
            }
            _ => {}
        }
        buf.clear();
    }
    Ok(barline)
}

/// Skip an element and all its children.
fn skip_element<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<()> {
    let mut buf = Vec::new();
    let mut depth = 1;
    let tag_name = start.name().as_ref().to_vec();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == tag_name {
                    depth += 1;
                }
            }
            Event::End(e) => {
                if e.name().as_ref() == tag_name {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
            }
            Event::Eof => {
                return Err(ParseError::MissingElement(format!(
                    "{} end",
                    String::from_utf8_lossy(&tag_name)
                )));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(())
}

/// Parse a yes/no value.
fn parse_yes_no(s: &str) -> Result<YesNo> {
    match s {
        "yes" => Ok(YesNo::Yes),
        "no" => Ok(YesNo::No),
        _ => Err(ParseError::InvalidAttribute(
            "yes-no".to_string(),
            s.to_string(),
        )),
    }
}

/// Parse a start/stop value.
fn parse_start_stop(s: &str) -> Result<StartStop> {
    match s {
        "start" => Ok(StartStop::Start),
        "stop" => Ok(StartStop::Stop),
        _ => Err(ParseError::InvalidAttribute(
            "start-stop".to_string(),
            s.to_string(),
        )),
    }
}

#[cfg(test)]
mod tests;

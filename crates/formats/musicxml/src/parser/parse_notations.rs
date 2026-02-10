//! Parsing for fermata, arpeggiate, glissando, slide, and other notation types
//! within the <notations> element.

use quick_xml::Reader;
use quick_xml::events::BytesStart;
use std::io::BufRead;

use super::{ParseError, Result, get_attr, get_attr_required, read_text};
use crate::model::data::*;
use crate::model::notations::*;

/// Helper: parse placement attribute from element.
pub(crate) fn parse_placement_attr(e: &BytesStart) -> Option<AboveBelow> {
    get_attr(e, "placement")
        .ok()
        .flatten()
        .and_then(|s| match s.as_str() {
            "above" => Some(AboveBelow::Above),
            "below" => Some(AboveBelow::Below),
            _ => None,
        })
}

/// Parse fermata shape from text content.
fn parse_fermata_shape(text: &str) -> Option<FermataShape> {
    match text.trim() {
        "" => Some(FermataShape::Empty),
        "normal" => Some(FermataShape::Normal),
        "angled" => Some(FermataShape::Angled),
        "square" => Some(FermataShape::Square),
        "double-angled" => Some(FermataShape::DoubleAngled),
        "double-square" => Some(FermataShape::DoubleSquare),
        "double-dot" => Some(FermataShape::DoubleDot),
        "half-curve" => Some(FermataShape::HalfCurve),
        "curlew" => Some(FermataShape::Curlew),
        _ => None,
    }
}

/// Parse fermata attributes common to both Start and Empty events.
fn parse_fermata_attrs(e: &BytesStart) -> Result<Fermata> {
    Ok(Fermata {
        shape: None, // filled in by caller for Start events
        fermata_type: get_attr(e, "type")?.and_then(|s| match s.as_str() {
            "upright" => Some(UprightInverted::Upright),
            "inverted" => Some(UprightInverted::Inverted),
            _ => None,
        }),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        relative_x: get_attr(e, "relative-x")?.and_then(|s| s.parse().ok()),
        relative_y: get_attr(e, "relative-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
    })
}

/// Parse fermata from Start event (may have text content).
pub(crate) fn parse_fermata_start<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<Fermata> {
    let mut f = parse_fermata_attrs(e)?;
    let text = read_text(reader, b"fermata")?;
    f.shape = parse_fermata_shape(&text);
    Ok(f)
}

/// Parse fermata from Empty event (no text content).
pub(crate) fn parse_fermata_empty(e: &BytesStart) -> Result<Fermata> {
    parse_fermata_attrs(e)
}

fn parse_yes_no_opt(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

/// Parse arpeggiate from Empty event.
pub(crate) fn parse_arpeggiate(e: &BytesStart) -> Result<Arpeggiate> {
    Ok(Arpeggiate {
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        direction: get_attr(e, "direction")?.and_then(|s| match s.as_str() {
            "up" => Some(UpDown::Up),
            "down" => Some(UpDown::Down),
            _ => None,
        }),
        unbroken: get_attr(e, "unbroken")?.and_then(|s| parse_yes_no_opt(&s)),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        placement: parse_placement_attr(e),
        color: get_attr(e, "color")?,
    })
}

/// Parse non-arpeggiate from Empty event.
pub(crate) fn parse_non_arpeggiate(e: &BytesStart) -> Result<NonArpeggiate> {
    let non_arpeggiate_type = match get_attr_required(e, "type")?.as_str() {
        "top" => TopBottom::Top,
        "bottom" => TopBottom::Bottom,
        s => {
            return Err(ParseError::InvalidAttribute(
                "type".to_string(),
                s.to_string(),
            ));
        }
    };
    Ok(NonArpeggiate {
        non_arpeggiate_type,
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        placement: parse_placement_attr(e),
        color: get_attr(e, "color")?,
    })
}

/// Parse glissando/slide shared attributes.
#[allow(clippy::type_complexity)]
fn parse_start_stop_line_attrs(
    e: &BytesStart,
) -> Result<(
    StartStop,
    Option<u8>,
    Option<LineType>,
    Option<f64>,
    Option<f64>,
    Option<String>,
)> {
    let ss_type = match get_attr_required(e, "type")?.as_str() {
        "start" => StartStop::Start,
        "stop" => StartStop::Stop,
        s => {
            return Err(ParseError::InvalidAttribute(
                "type".to_string(),
                s.to_string(),
            ));
        }
    };
    let number = get_attr(e, "number")?.and_then(|s| s.parse().ok());
    let line_type = get_attr(e, "line-type")?.and_then(|s| match s.as_str() {
        "solid" => Some(LineType::Solid),
        "dashed" => Some(LineType::Dashed),
        "dotted" => Some(LineType::Dotted),
        "wavy" => Some(LineType::Wavy),
        _ => None,
    });
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;
    Ok((ss_type, number, line_type, default_x, default_y, color))
}

/// Parse glissando from Start event (has text content).
pub(crate) fn parse_glissando<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<Glissando> {
    let (glissando_type, number, line_type, default_x, default_y, color) =
        parse_start_stop_line_attrs(e)?;
    let text = read_text(reader, b"glissando")?;
    Ok(Glissando {
        glissando_type,
        number,
        line_type,
        default_x,
        default_y,
        color,
        text,
    })
}

/// Parse glissando from Empty event.
pub(crate) fn parse_glissando_empty(e: &BytesStart) -> Result<Glissando> {
    let (glissando_type, number, line_type, default_x, default_y, color) =
        parse_start_stop_line_attrs(e)?;
    Ok(Glissando {
        glissando_type,
        number,
        line_type,
        default_x,
        default_y,
        color,
        text: String::new(),
    })
}

/// Parse slide from Start event (has text content).
pub(crate) fn parse_slide<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Slide> {
    let (slide_type, number, line_type, default_x, default_y, color) =
        parse_start_stop_line_attrs(e)?;
    let text = read_text(reader, b"slide")?;
    Ok(Slide {
        slide_type,
        number,
        line_type,
        default_x,
        default_y,
        color,
        text,
    })
}

/// Parse slide from Empty event.
pub(crate) fn parse_slide_empty(e: &BytesStart) -> Result<Slide> {
    let (slide_type, number, line_type, default_x, default_y, color) =
        parse_start_stop_line_attrs(e)?;
    Ok(Slide {
        slide_type,
        number,
        line_type,
        default_x,
        default_y,
        color,
        text: String::new(),
    })
}

/// Parse other-notation from Start event (has text content).
pub(crate) fn parse_other_notation_start<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<OtherNotation> {
    let notation_type = match get_attr_required(e, "type")?.as_str() {
        "start" => StartStopSingle::Start,
        "stop" => StartStopSingle::Stop,
        "single" => StartStopSingle::Single,
        s => {
            return Err(ParseError::InvalidAttribute(
                "type".to_string(),
                s.to_string(),
            ));
        }
    };
    let number = get_attr(e, "number")?.and_then(|s| s.parse().ok());
    let placement = parse_placement_attr(e);
    let smufl = get_attr(e, "smufl")?;
    let text = read_text(reader, b"other-notation")?;
    Ok(OtherNotation {
        notation_type,
        number,
        placement,
        smufl,
        text,
    })
}

/// Parse other-notation from Empty event.
pub(crate) fn parse_other_notation_empty(e: &BytesStart) -> Result<OtherNotation> {
    let notation_type = match get_attr_required(e, "type")?.as_str() {
        "start" => StartStopSingle::Start,
        "stop" => StartStopSingle::Stop,
        "single" => StartStopSingle::Single,
        s => {
            return Err(ParseError::InvalidAttribute(
                "type".to_string(),
                s.to_string(),
            ));
        }
    };
    Ok(OtherNotation {
        notation_type,
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        placement: parse_placement_attr(e),
        smufl: get_attr(e, "smufl")?,
        text: String::new(),
    })
}

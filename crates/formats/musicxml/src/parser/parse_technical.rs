//! Parsing for technical notation types within `<notations><technical>`.

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::parse_notations::parse_placement_attr;
use super::{ParseError, Result, get_attr, get_attr_required, read_text, skip_element};
use crate::model::data::*;
use crate::model::notations::EmptyPlacement;
use crate::model::technical::*;

/// Parse the `<technical>` container element.
pub(crate) fn parse_technical<R: BufRead>(reader: &mut Reader<R>) -> Result<Technical> {
    let mut buf = Vec::new();
    let mut tech = Technical::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"up-bow" => {
                    tech.up_bow.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"up-bow")?;
                }
                b"down-bow" => {
                    tech.down_bow.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"down-bow")?;
                }
                b"harmonic" => {
                    tech.harmonic.push(parse_harmonic(reader, &e)?);
                }
                b"open-string" => {
                    tech.open_string.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"open-string")?;
                }
                b"thumb-position" => {
                    tech.thumb_position.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"thumb-position")?;
                }
                b"fingering" => {
                    tech.fingering.push(parse_fingering(reader, &e)?);
                }
                b"pluck" => {
                    tech.pluck.push(parse_placement_text(reader, &e, b"pluck")?);
                }
                b"double-tongue" => {
                    tech.double_tongue.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"double-tongue")?;
                }
                b"triple-tongue" => {
                    tech.triple_tongue.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"triple-tongue")?;
                }
                b"stopped" => {
                    tech.stopped.push(parse_empty_placement_smufl(&e));
                    skip_to_end(reader, b"stopped")?;
                }
                b"snap-pizzicato" => {
                    tech.snap_pizzicato.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"snap-pizzicato")?;
                }
                b"fret" => {
                    tech.fret.push(parse_fret(reader, &e)?);
                }
                b"string" => {
                    tech.string.push(parse_tech_string(reader, &e)?);
                }
                b"hammer-on" => {
                    tech.hammer_on
                        .push(parse_hammer_on_pull_off(reader, &e, b"hammer-on")?);
                }
                b"pull-off" => {
                    tech.pull_off
                        .push(parse_hammer_on_pull_off(reader, &e, b"pull-off")?);
                }
                b"bend" => {
                    tech.bend.push(parse_bend(reader, &e)?);
                }
                b"tap" => {
                    tech.tap.push(parse_tap(reader, &e)?);
                }
                b"heel" => {
                    tech.heel.push(parse_heel_toe(&e));
                    skip_to_end(reader, b"heel")?;
                }
                b"toe" => {
                    tech.toe.push(parse_heel_toe(&e));
                    skip_to_end(reader, b"toe")?;
                }
                b"fingernails" => {
                    tech.fingernails.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"fingernails")?;
                }
                b"hole" => {
                    tech.hole.push(parse_hole(reader, &e)?);
                }
                b"arrow" => {
                    tech.arrow.push(parse_arrow(reader, &e)?);
                }
                b"handbell" => {
                    tech.handbell.push(parse_handbell(reader, &e)?);
                }
                b"brass-bend" => {
                    tech.brass_bend.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"brass-bend")?;
                }
                b"flip" => {
                    tech.flip.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"flip")?;
                }
                b"smear" => {
                    tech.smear.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"smear")?;
                }
                b"open" => {
                    tech.open.push(parse_empty_placement_smufl(&e));
                    skip_to_end(reader, b"open")?;
                }
                b"half-muted" => {
                    tech.half_muted.push(parse_empty_placement_smufl(&e));
                    skip_to_end(reader, b"half-muted")?;
                }
                b"harmon-mute" => {
                    tech.harmon_mute.push(parse_harmon_mute(reader, &e)?);
                }
                b"golpe" => {
                    tech.golpe.push(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"golpe")?;
                }
                b"other-technical" => {
                    tech.other_technical
                        .push(parse_other_technical(reader, &e)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"up-bow" => tech.up_bow.push(parse_empty_placement_from(&e)),
                b"down-bow" => tech.down_bow.push(parse_empty_placement_from(&e)),
                b"harmonic" => tech.harmonic.push(parse_harmonic_attrs(&e)?),
                b"open-string" => tech.open_string.push(parse_empty_placement_from(&e)),
                b"thumb-position" => tech.thumb_position.push(parse_empty_placement_from(&e)),
                b"fingering" => tech.fingering.push(parse_fingering_empty(&e)?),
                b"double-tongue" => tech.double_tongue.push(parse_empty_placement_from(&e)),
                b"triple-tongue" => tech.triple_tongue.push(parse_empty_placement_from(&e)),
                b"stopped" => tech.stopped.push(parse_empty_placement_smufl(&e)),
                b"snap-pizzicato" => tech.snap_pizzicato.push(parse_empty_placement_from(&e)),
                b"tap" => tech.tap.push(parse_tap_empty(&e)?),
                b"heel" => tech.heel.push(parse_heel_toe(&e)),
                b"toe" => tech.toe.push(parse_heel_toe(&e)),
                b"fingernails" => tech.fingernails.push(parse_empty_placement_from(&e)),
                b"brass-bend" => tech.brass_bend.push(parse_empty_placement_from(&e)),
                b"flip" => tech.flip.push(parse_empty_placement_from(&e)),
                b"smear" => tech.smear.push(parse_empty_placement_from(&e)),
                b"open" => tech.open.push(parse_empty_placement_smufl(&e)),
                b"half-muted" => tech.half_muted.push(parse_empty_placement_smufl(&e)),
                b"golpe" => tech.golpe.push(parse_empty_placement_from(&e)),
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"technical" => break,
            Event::Eof => return Err(ParseError::MissingElement("technical end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(tech)
}

// ============================================================================
// Helper parsing functions
// ============================================================================

fn skip_to_end<R: BufRead>(reader: &mut Reader<R>, tag: &[u8]) -> Result<()> {
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf)? {
            Event::End(e) if e.name().as_ref() == tag => break,
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
    Ok(())
}

fn parse_yes_no_opt(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

/// Parse an empty-placement element from attributes.
fn parse_empty_placement_from(e: &BytesStart) -> EmptyPlacement {
    EmptyPlacement {
        placement: parse_placement_attr(e),
        default_x: get_attr(e, "default-x")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        color: get_attr(e, "color").ok().flatten(),
    }
}

/// Parse an empty-placement-smufl element from attributes.
fn parse_empty_placement_smufl(e: &BytesStart) -> EmptyPlacementSmufl {
    EmptyPlacementSmufl {
        placement: parse_placement_attr(e),
        default_x: get_attr(e, "default-x")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        color: get_attr(e, "color").ok().flatten(),
        smufl: get_attr(e, "smufl").ok().flatten(),
    }
}

/// Parse heel-toe from attributes.
fn parse_heel_toe(e: &BytesStart) -> HeelToe {
    HeelToe {
        substitution: get_attr(e, "substitution")
            .ok()
            .flatten()
            .and_then(|s| parse_yes_no_opt(&s)),
        placement: parse_placement_attr(e),
        default_x: get_attr(e, "default-x")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        color: get_attr(e, "color").ok().flatten(),
    }
}

/// Parse fingering from Start event (has text content).
fn parse_fingering<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Fingering> {
    let substitution = get_attr(e, "substitution")?.and_then(|s| parse_yes_no_opt(&s));
    let alternate = get_attr(e, "alternate")?.and_then(|s| parse_yes_no_opt(&s));
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;
    let value = read_text(reader, b"fingering")?;
    Ok(Fingering {
        value,
        substitution,
        alternate,
        placement,
        default_x,
        default_y,
        color,
    })
}

/// Parse fingering from Empty event.
fn parse_fingering_empty(e: &BytesStart) -> Result<Fingering> {
    Ok(Fingering {
        value: String::new(),
        substitution: get_attr(e, "substitution")?.and_then(|s| parse_yes_no_opt(&s)),
        alternate: get_attr(e, "alternate")?.and_then(|s| parse_yes_no_opt(&s)),
        placement: parse_placement_attr(e),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
    })
}

/// Parse placement-text (pluck, with-bar, etc.) from Start event.
fn parse_placement_text<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
    end_tag: &[u8],
) -> Result<PlacementText> {
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let font_style = get_attr(e, "font-style")?;
    let color = get_attr(e, "color")?;
    let value = read_text(reader, end_tag)?;
    Ok(PlacementText {
        value,
        placement,
        default_x,
        default_y,
        font_style,
        color,
    })
}

/// Parse fret from Start event (has text content).
fn parse_fret<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Fret> {
    let color = get_attr(e, "color")?;
    let text = read_text(reader, b"fret")?;
    let value = text
        .parse()
        .map_err(|_| ParseError::ParseNumber("fret".to_string()))?;
    Ok(Fret { value, color })
}

/// Parse string from Start event (has text content).
fn parse_tech_string<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<TechString> {
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;
    let text = read_text(reader, b"string")?;
    let value = text
        .parse()
        .map_err(|_| ParseError::ParseNumber("string".to_string()))?;
    Ok(TechString {
        value,
        placement,
        default_x,
        default_y,
        color,
    })
}

/// Parse hammer-on or pull-off from Start event.
fn parse_hammer_on_pull_off<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
    end_tag: &[u8],
) -> Result<HammerOnPullOff> {
    let ho_type = match get_attr_required(e, "type")?.as_str() {
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
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;
    let text = read_text(reader, end_tag)?;
    Ok(HammerOnPullOff {
        ho_type,
        number,
        placement,
        default_x,
        default_y,
        color,
        text,
    })
}

/// Parse bend element.
fn parse_bend<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Bend> {
    let shape = get_attr(e, "shape")?.and_then(|s| match s.as_str() {
        "straight" => Some(BendShape::Straight),
        "curved" => Some(BendShape::Curved),
        _ => None,
    });
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;

    let mut buf = Vec::new();
    let mut bend_alter: f64 = 0.0;
    let mut pre_bend = None;
    let mut release = None;
    let mut with_bar = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"bend-alter" => {
                    let text = read_text(reader, b"bend-alter")?;
                    bend_alter = text
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("bend-alter".to_string()))?;
                }
                b"with-bar" => {
                    with_bar = Some(parse_placement_text(reader, &e, b"with-bar")?);
                }
                b"release" => {
                    let offset = get_attr(&e, "offset")?.and_then(|s| s.parse().ok());
                    release = Some(BendRelease { offset });
                    skip_to_end(reader, b"release")?;
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"pre-bend" => pre_bend = Some(true),
                b"release" => {
                    let offset = get_attr(&e, "offset")
                        .ok()
                        .flatten()
                        .and_then(|s| s.parse().ok());
                    release = Some(BendRelease { offset });
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"bend" => break,
            Event::Eof => return Err(ParseError::MissingElement("bend end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Bend {
        bend_alter,
        pre_bend,
        release,
        with_bar,
        shape,
        default_x,
        default_y,
        color,
    })
}

/// Parse tap from Start event (may have text content).
fn parse_tap<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Tap> {
    let hand = get_attr(e, "hand")?.and_then(|s| match s.as_str() {
        "left" => Some(TapHand::Left),
        "right" => Some(TapHand::Right),
        _ => None,
    });
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;
    let value = read_text(reader, b"tap")?;
    Ok(Tap {
        value,
        hand,
        placement,
        default_x,
        default_y,
        color,
    })
}

/// Parse tap from Empty event.
fn parse_tap_empty(e: &BytesStart) -> Result<Tap> {
    Ok(Tap {
        value: String::new(),
        hand: get_attr(e, "hand")?.and_then(|s| match s.as_str() {
            "left" => Some(TapHand::Left),
            "right" => Some(TapHand::Right),
            _ => None,
        }),
        placement: parse_placement_attr(e),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
    })
}

/// Parse hole element.
fn parse_hole<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Hole> {
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;

    let mut buf = Vec::new();
    let mut hole_type = None;
    let mut hole_closed = None;
    let mut hole_shape = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"hole-type" => {
                    hole_type = Some(read_text(reader, b"hole-type")?);
                }
                b"hole-closed" => {
                    let location =
                        get_attr(&e, "location")?.and_then(|s| parse_hole_closed_location(&s));
                    let text = read_text(reader, b"hole-closed")?;
                    let value = parse_hole_closed_value(&text)?;
                    hole_closed = Some(HoleClosed { value, location });
                }
                b"hole-shape" => {
                    hole_shape = Some(read_text(reader, b"hole-shape")?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"hole-closed" => {
                let location = get_attr(&e, "location")
                    .ok()
                    .flatten()
                    .and_then(|s| parse_hole_closed_location(&s));
                // Empty hole-closed defaults to "no" per convention
                hole_closed = Some(HoleClosed {
                    value: HoleClosedValue::No,
                    location,
                });
            }
            Event::End(e) if e.name().as_ref() == b"hole" => break,
            Event::Eof => return Err(ParseError::MissingElement("hole end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    let hole_closed =
        hole_closed.ok_or_else(|| ParseError::MissingElement("hole-closed".to_string()))?;

    Ok(Hole {
        hole_type,
        hole_closed,
        hole_shape,
        placement,
        default_x,
        default_y,
        color,
    })
}

fn parse_hole_closed_value(s: &str) -> Result<HoleClosedValue> {
    match s.trim() {
        "yes" => Ok(HoleClosedValue::Yes),
        "no" => Ok(HoleClosedValue::No),
        "half" => Ok(HoleClosedValue::Half),
        _ => Err(ParseError::InvalidContent(
            "hole-closed".to_string(),
            s.to_string(),
        )),
    }
}

fn parse_hole_closed_location(s: &str) -> Option<HoleClosedLocation> {
    match s {
        "right" => Some(HoleClosedLocation::Right),
        "bottom" => Some(HoleClosedLocation::Bottom),
        "left" => Some(HoleClosedLocation::Left),
        "top" => Some(HoleClosedLocation::Top),
        _ => None,
    }
}

/// Parse arrow element.
fn parse_arrow<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Arrow> {
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;
    let smufl = get_attr(e, "smufl")?;

    let mut buf = Vec::new();
    let mut arrow_direction = None;
    let mut arrow_style = None;
    let mut arrowhead = false;
    let mut circular_arrow = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"arrow-direction" => {
                    arrow_direction = Some(read_text(reader, b"arrow-direction")?);
                }
                b"arrow-style" => {
                    arrow_style = Some(read_text(reader, b"arrow-style")?);
                }
                b"circular-arrow" => {
                    circular_arrow = Some(read_text(reader, b"circular-arrow")?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"arrowhead" => {
                arrowhead = true;
            }
            Event::Empty(_) => {}
            Event::End(e) if e.name().as_ref() == b"arrow" => break,
            Event::Eof => return Err(ParseError::MissingElement("arrow end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    let content = if let Some(dir) = arrow_direction {
        ArrowContent::Directional {
            direction: dir,
            style: arrow_style,
            arrowhead,
        }
    } else if let Some(ca) = circular_arrow {
        ArrowContent::Circular(ca)
    } else {
        return Err(ParseError::MissingElement(
            "arrow-direction or circular-arrow".to_string(),
        ));
    };

    Ok(Arrow {
        content,
        placement,
        default_x,
        default_y,
        color,
        smufl,
    })
}

/// Parse handbell from Start event (has text content).
fn parse_handbell<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Handbell> {
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;
    let value = read_text(reader, b"handbell")?;
    Ok(Handbell {
        value,
        placement,
        default_x,
        default_y,
        color,
    })
}

/// Parse harmon-mute element.
fn parse_harmon_mute<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<HarmonMute> {
    let placement = parse_placement_attr(e);
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;

    let mut buf = Vec::new();
    let mut harmon_closed = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"harmon-closed" => {
                    let location =
                        get_attr(&e, "location")?.and_then(|s| parse_harmon_closed_location(&s));
                    let text = read_text(reader, b"harmon-closed")?;
                    let value = parse_harmon_closed_value(&text)?;
                    harmon_closed = Some(HarmonClosed { value, location });
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"harmon-closed" => {
                let location = get_attr(&e, "location")
                    .ok()
                    .flatten()
                    .and_then(|s| parse_harmon_closed_location(&s));
                harmon_closed = Some(HarmonClosed {
                    value: HarmonClosedValue::No,
                    location,
                });
            }
            Event::Empty(_) => {}
            Event::End(e) if e.name().as_ref() == b"harmon-mute" => break,
            Event::Eof => return Err(ParseError::MissingElement("harmon-mute end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    let harmon_closed =
        harmon_closed.ok_or_else(|| ParseError::MissingElement("harmon-closed".to_string()))?;

    Ok(HarmonMute {
        harmon_closed,
        placement,
        default_x,
        default_y,
        color,
    })
}

fn parse_harmon_closed_value(s: &str) -> Result<HarmonClosedValue> {
    match s.trim() {
        "yes" => Ok(HarmonClosedValue::Yes),
        "no" => Ok(HarmonClosedValue::No),
        "half" => Ok(HarmonClosedValue::Half),
        _ => Err(ParseError::InvalidContent(
            "harmon-closed".to_string(),
            s.to_string(),
        )),
    }
}

fn parse_harmon_closed_location(s: &str) -> Option<HarmonClosedLocation> {
    match s {
        "right" => Some(HarmonClosedLocation::Right),
        "bottom" => Some(HarmonClosedLocation::Bottom),
        "left" => Some(HarmonClosedLocation::Left),
        "top" => Some(HarmonClosedLocation::Top),
        _ => None,
    }
}

/// Parse harmonic element (has child elements: natural/artificial, base/touching/sounding-pitch).
fn parse_harmonic<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Harmonic> {
    let mut h = parse_harmonic_attrs(e)?;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"natural" => {
                    h.natural = Some(true);
                    skip_to_end(reader, b"natural")?;
                }
                b"artificial" => {
                    h.artificial = Some(true);
                    skip_to_end(reader, b"artificial")?;
                }
                b"base-pitch" => {
                    h.base_pitch = Some(true);
                    skip_to_end(reader, b"base-pitch")?;
                }
                b"touching-pitch" => {
                    h.touching_pitch = Some(true);
                    skip_to_end(reader, b"touching-pitch")?;
                }
                b"sounding-pitch" => {
                    h.sounding_pitch = Some(true);
                    skip_to_end(reader, b"sounding-pitch")?;
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"natural" => h.natural = Some(true),
                b"artificial" => h.artificial = Some(true),
                b"base-pitch" => h.base_pitch = Some(true),
                b"touching-pitch" => h.touching_pitch = Some(true),
                b"sounding-pitch" => h.sounding_pitch = Some(true),
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"harmonic" => break,
            Event::Eof => return Err(ParseError::MissingElement("harmonic end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(h)
}

/// Parse harmonic attributes only (for Empty events).
fn parse_harmonic_attrs(e: &BytesStart) -> Result<Harmonic> {
    Ok(Harmonic {
        placement: parse_placement_attr(e),
        print_object: get_attr(e, "print-object")?.and_then(|s| parse_yes_no_opt(&s)),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        ..Default::default()
    })
}

/// Parse other-technical from Start event.
fn parse_other_technical<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<OtherTechnical> {
    let placement = parse_placement_attr(e);
    let smufl = get_attr(e, "smufl")?;
    let default_x = get_attr(e, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(e, "color")?;
    let value = read_text(reader, b"other-technical")?;
    Ok(OtherTechnical {
        value,
        placement,
        smufl,
        default_x,
        default_y,
        color,
    })
}

//! Direction parsing.

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::parse_listening::parse_listening;
use super::{ParseError, Result, get_attr, read_text, skip_element};
use crate::model::data::*;
use crate::model::direction::*;
use crate::model::elements::score::{MidiDevice, MidiInstrument};
use crate::model::listening::Listening;

pub fn parse_direction<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<Direction> {
    let mut buf = Vec::new();
    let placement = get_attr(start, "placement")?.and_then(|s| match s.as_str() {
        "above" => Some(AboveBelow::Above),
        "below" => Some(AboveBelow::Below),
        _ => None,
    });
    let directive = get_attr(start, "directive")?.and_then(|s| parse_yes_no_opt(&s));

    let mut direction_types: Vec<DirectionType> = Vec::new();
    let mut offset: Option<Offset> = None;
    let mut staff: Option<u32> = None;
    let mut sound: Option<Sound> = None;
    let mut listening: Option<Listening> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"direction-type" => direction_types.push(parse_direction_type(reader)?),
                b"offset" => offset = Some(parse_offset(reader, &e)?),
                b"staff" => {
                    staff = Some(
                        read_text(reader, b"staff")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staff".to_string()))?,
                    )
                }
                b"sound" => {
                    sound = Some(parse_sound_full(reader, &e)?);
                }
                b"listening" => {
                    listening = Some(parse_listening(reader)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => {
                if e.name().as_ref() == b"sound" {
                    sound = Some(parse_sound_attrs(&e)?);
                }
            }
            Event::End(e) if e.name().as_ref() == b"direction" => break,
            Event::Eof => return Err(ParseError::MissingElement("direction end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Direction {
        direction_types,
        offset,
        staff,
        sound,
        listening,
        placement,
        directive,
        id: None,
    })
}

fn parse_direction_type<R: BufRead>(reader: &mut Reader<R>) -> Result<DirectionType> {
    let mut buf = Vec::new();
    let mut content: Option<DirectionTypeContent> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"wedge" => {
                    content = Some(DirectionTypeContent::Wedge(parse_wedge(&e)?));
                    skip_to_end(reader, b"wedge")?;
                }
                b"dynamics" => {
                    content = Some(DirectionTypeContent::Dynamics(parse_dynamics(reader)?))
                }
                b"words" => {
                    let words = parse_words(reader, &e)?;
                    if let Some(DirectionTypeContent::Words(ref mut w)) = content {
                        w.push(words);
                    } else {
                        content = Some(DirectionTypeContent::Words(vec![words]));
                    }
                }
                b"metronome" => {
                    content = Some(DirectionTypeContent::Metronome(parse_metronome(
                        reader, &e,
                    )?))
                }
                b"rehearsal" => {
                    let rehearsal = parse_rehearsal(reader, &e)?;
                    content = Some(DirectionTypeContent::Rehearsal(vec![rehearsal]));
                }
                b"pedal" => {
                    content = Some(DirectionTypeContent::Pedal(parse_pedal(&e)?));
                    skip_to_end(reader, b"pedal")?;
                }
                b"octave-shift" => {
                    content = Some(DirectionTypeContent::OctaveShift(parse_octave_shift(&e)?));
                    skip_to_end(reader, b"octave-shift")?;
                }
                b"dashes" => {
                    content = Some(DirectionTypeContent::Dashes(parse_dashes(&e)?));
                    skip_to_end(reader, b"dashes")?;
                }
                b"bracket" => {
                    content = Some(DirectionTypeContent::Bracket(parse_bracket(&e)?));
                    skip_to_end(reader, b"bracket")?;
                }
                b"segno" => {
                    content = Some(DirectionTypeContent::Segno(vec![parse_segno(&e)?]));
                    skip_to_end(reader, b"segno")?;
                }
                b"coda" => {
                    content = Some(DirectionTypeContent::Coda(vec![parse_coda(&e)?]));
                    skip_to_end(reader, b"coda")?;
                }
                b"symbol" => {
                    let sym = parse_symbol(reader, &e)?;
                    if let Some(DirectionTypeContent::Symbol(ref mut v)) = content {
                        v.push(sym);
                    } else {
                        content = Some(DirectionTypeContent::Symbol(vec![sym]));
                    }
                }
                b"harp-pedals" => {
                    content = Some(DirectionTypeContent::HarpPedals(parse_harp_pedals(
                        reader, &e,
                    )?))
                }
                b"damp" => {
                    content = Some(DirectionTypeContent::Damp(parse_empty_direction(&e)?));
                    skip_to_end(reader, b"damp")?;
                }
                b"damp-all" => {
                    content = Some(DirectionTypeContent::DampAll(parse_empty_direction(&e)?));
                    skip_to_end(reader, b"damp-all")?;
                }
                b"eyeglasses" => {
                    content = Some(DirectionTypeContent::Eyeglasses(parse_empty_direction(&e)?));
                    skip_to_end(reader, b"eyeglasses")?;
                }
                b"string-mute" => {
                    content = Some(DirectionTypeContent::StringMute(parse_string_mute(&e)?));
                    skip_to_end(reader, b"string-mute")?;
                }
                b"scordatura" => {
                    content = Some(DirectionTypeContent::Scordatura(parse_scordatura(
                        reader, &e,
                    )?))
                }
                b"image" => {
                    content = Some(DirectionTypeContent::Image(parse_image(&e)?));
                    skip_to_end(reader, b"image")?;
                }
                b"principal-voice" => {
                    content = Some(DirectionTypeContent::PrincipalVoice(parse_principal_voice(
                        reader, &e,
                    )?))
                }
                b"percussion" => {
                    let perc = parse_percussion(reader, &e)?;
                    if let Some(DirectionTypeContent::Percussion(ref mut v)) = content {
                        v.push(perc);
                    } else {
                        content = Some(DirectionTypeContent::Percussion(vec![perc]));
                    }
                }
                b"accordion-registration" => {
                    content = Some(DirectionTypeContent::AccordionRegistration(
                        parse_accordion_registration(reader, &e)?,
                    ))
                }
                b"staff-divide" => {
                    content = Some(DirectionTypeContent::StaffDivide(parse_staff_divide(&e)?));
                    skip_to_end(reader, b"staff-divide")?;
                }
                b"other-direction" => {
                    content = Some(DirectionTypeContent::OtherDirection(parse_other_direction(
                        reader, &e,
                    )?))
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"wedge" => content = Some(DirectionTypeContent::Wedge(parse_wedge(&e)?)),
                b"pedal" => content = Some(DirectionTypeContent::Pedal(parse_pedal(&e)?)),
                b"octave-shift" => {
                    content = Some(DirectionTypeContent::OctaveShift(parse_octave_shift(&e)?))
                }
                b"dashes" => content = Some(DirectionTypeContent::Dashes(parse_dashes(&e)?)),
                b"bracket" => content = Some(DirectionTypeContent::Bracket(parse_bracket(&e)?)),
                b"segno" => content = Some(DirectionTypeContent::Segno(vec![parse_segno(&e)?])),
                b"coda" => content = Some(DirectionTypeContent::Coda(vec![parse_coda(&e)?])),
                b"damp" => content = Some(DirectionTypeContent::Damp(parse_empty_direction(&e)?)),
                b"damp-all" => {
                    content = Some(DirectionTypeContent::DampAll(parse_empty_direction(&e)?))
                }
                b"eyeglasses" => {
                    content = Some(DirectionTypeContent::Eyeglasses(parse_empty_direction(&e)?))
                }
                b"string-mute" => {
                    content = Some(DirectionTypeContent::StringMute(parse_string_mute(&e)?))
                }
                b"image" => content = Some(DirectionTypeContent::Image(parse_image(&e)?)),
                b"staff-divide" => {
                    content = Some(DirectionTypeContent::StaffDivide(parse_staff_divide(&e)?))
                }
                b"accordion-registration" => {
                    content = Some(DirectionTypeContent::AccordionRegistration(
                        AccordionRegistration::default(),
                    ))
                }
                b"other-direction" => {
                    content = Some(DirectionTypeContent::OtherDirection(OtherDirection {
                        value: None,
                        print_object: get_attr(&e, "print-object")?
                            .and_then(|s| parse_yes_no_opt(&s)),
                        smufl: get_attr(&e, "smufl")?,
                        default_x: get_attr(&e, "default-x")?.and_then(|s| s.parse().ok()),
                        default_y: get_attr(&e, "default-y")?.and_then(|s| s.parse().ok()),
                        halign: get_attr(&e, "halign")?.and_then(|s| parse_lcr(&s)),
                        valign: get_attr(&e, "valign")?.and_then(|s| parse_valign(&s)),
                        id: get_attr(&e, "id")?,
                    }));
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"direction-type" => break,
            Event::Eof => return Err(ParseError::MissingElement("direction-type end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(DirectionType {
        content: content.unwrap_or(DirectionTypeContent::Words(vec![])),
        id: None,
    })
}

fn parse_wedge(e: &BytesStart) -> Result<Wedge> {
    let type_str = get_attr(e, "type")?.unwrap_or_default();
    let wedge_type = match type_str.as_str() {
        "crescendo" => WedgeType::Crescendo,
        "diminuendo" => WedgeType::Diminuendo,
        "stop" => WedgeType::Stop,
        "continue" => WedgeType::Continue,
        _ => WedgeType::Crescendo,
    };

    Ok(Wedge {
        wedge_type,
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        spread: get_attr(e, "spread")?.and_then(|s| s.parse().ok()),
        niente: get_attr(e, "niente")?.and_then(|s| parse_yes_no_opt(&s)),
        line_type: get_attr(e, "line-type")?.and_then(|s| match s.as_str() {
            "solid" => Some(LineType::Solid),
            "dashed" => Some(LineType::Dashed),
            "dotted" => Some(LineType::Dotted),
            "wavy" => Some(LineType::Wavy),
            _ => None,
        }),
        dash_length: get_attr(e, "dash-length")?.and_then(|s| s.parse().ok()),
        space_length: get_attr(e, "space-length")?.and_then(|s| s.parse().ok()),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        relative_x: get_attr(e, "relative-x")?.and_then(|s| s.parse().ok()),
        relative_y: get_attr(e, "relative-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        id: get_attr(e, "id")?,
    })
}

pub(crate) fn parse_dynamics<R: BufRead>(reader: &mut Reader<R>) -> Result<Dynamics> {
    let mut buf = Vec::new();
    let mut values: Vec<DynamicsValue> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let dyn_value = parse_dynamics_element(e.name().as_ref());
                if let Some(v) = dyn_value {
                    values.push(v);
                }
                // Skip to end of the dynamics element
                skip_to_end(reader, e.name().as_ref())?;
            }
            Event::Empty(e) => {
                let dyn_value = parse_dynamics_element(e.name().as_ref());
                if let Some(v) = dyn_value {
                    values.push(v);
                }
            }
            Event::End(e) if e.name().as_ref() == b"dynamics" => break,
            Event::Eof => return Err(ParseError::MissingElement("dynamics end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Dynamics {
        values,
        placement: None,
    })
}

fn parse_dynamics_element(name: &[u8]) -> Option<DynamicsValue> {
    match name {
        b"ppp" => Some(DynamicsValue::Ppp),
        b"pp" => Some(DynamicsValue::Pp),
        b"p" => Some(DynamicsValue::P),
        b"mp" => Some(DynamicsValue::Mp),
        b"mf" => Some(DynamicsValue::Mf),
        b"f" => Some(DynamicsValue::F),
        b"ff" => Some(DynamicsValue::Ff),
        b"fff" => Some(DynamicsValue::Fff),
        b"fp" => Some(DynamicsValue::Fp),
        b"sf" => Some(DynamicsValue::Sf),
        b"sfz" => Some(DynamicsValue::Sfz),
        b"sfp" => Some(DynamicsValue::Sfp),
        b"sfpp" => Some(DynamicsValue::Sfpp),
        b"sffz" => Some(DynamicsValue::Sffz),
        b"sfzp" => Some(DynamicsValue::Sfzp),
        b"pf" => Some(DynamicsValue::Pf),
        b"rf" => Some(DynamicsValue::Rf),
        b"rfz" => Some(DynamicsValue::Rfz),
        b"fz" => Some(DynamicsValue::Fz),
        b"n" => Some(DynamicsValue::N),
        b"pppp" => Some(DynamicsValue::Pppp),
        b"ffff" => Some(DynamicsValue::Ffff),
        b"ppppp" => Some(DynamicsValue::Ppppp),
        b"fffff" => Some(DynamicsValue::Fffff),
        b"pppppp" => Some(DynamicsValue::Pppppp),
        b"ffffff" => Some(DynamicsValue::Ffffff),
        _ => None,
    }
}

fn parse_words<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Words> {
    let value = read_text(reader, b"words")?;
    Ok(Words {
        value,
        enclosure: get_attr(start, "enclosure")?.and_then(|s| parse_enclosure(&s)),
        font_family: get_attr(start, "font-family")?,
        font_style: get_attr(start, "font-style")?.and_then(|s| match s.as_str() {
            "italic" => Some(FontStyle::Italic),
            "normal" => Some(FontStyle::Normal),
            _ => None,
        }),
        font_size: get_attr(start, "font-size")?.and_then(|s| s.parse().ok().map(FontSize::Points)),
        font_weight: get_attr(start, "font-weight")?.and_then(|s| match s.as_str() {
            "bold" => Some(FontWeight::Bold),
            "normal" => Some(FontWeight::Normal),
            _ => None,
        }),
        color: get_attr(start, "color")?,
        halign: get_attr(start, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(start, "valign")?.and_then(|s| parse_valign(&s)),
        justify: get_attr(start, "justify")?.and_then(|s| parse_lcr(&s)),
        default_x: get_attr(start, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(start, "default-y")?.and_then(|s| s.parse().ok()),
        relative_x: get_attr(start, "relative-x")?.and_then(|s| s.parse().ok()),
        relative_y: get_attr(start, "relative-y")?.and_then(|s| s.parse().ok()),
        id: get_attr(start, "id")?,
    })
}

fn parse_rehearsal<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Rehearsal> {
    let value = read_text(reader, b"rehearsal")?;
    Ok(Rehearsal {
        value,
        enclosure: get_attr(start, "enclosure")?.and_then(|s| parse_enclosure(&s)),
        font_family: get_attr(start, "font-family")?,
        font_style: get_attr(start, "font-style")?.and_then(|s| match s.as_str() {
            "italic" => Some(FontStyle::Italic),
            "normal" => Some(FontStyle::Normal),
            _ => None,
        }),
        font_size: get_attr(start, "font-size")?.and_then(|s| s.parse().ok().map(FontSize::Points)),
        font_weight: get_attr(start, "font-weight")?.and_then(|s| match s.as_str() {
            "bold" => Some(FontWeight::Bold),
            "normal" => Some(FontWeight::Normal),
            _ => None,
        }),
        color: get_attr(start, "color")?,
        halign: get_attr(start, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(start, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(start, "id")?,
    })
}

fn parse_metronome<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Metronome> {
    let mut buf = Vec::new();
    let parentheses = get_attr(start, "parentheses")?.and_then(|s| parse_yes_no_opt(&s));

    let mut beat_unit: Option<String> = None;
    let mut per_minute: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"beat-unit" => beat_unit = Some(read_text(reader, b"beat-unit")?),
                b"per-minute" => per_minute = Some(read_text(reader, b"per-minute")?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"metronome" => break,
            Event::Eof => return Err(ParseError::MissingElement("metronome end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Metronome {
        content: MetronomeContent::BeatUnit {
            beat_unit: beat_unit.unwrap_or_else(|| "quarter".to_string()),
            beat_unit_dots: Vec::new(),
            per_minute: per_minute.unwrap_or_else(|| "120".to_string()),
        },
        parentheses,
        print_object: None,
        justify: None,
        default_x: None,
        default_y: None,
        halign: None,
        valign: None,
        id: None,
    })
}

fn parse_pedal(e: &BytesStart) -> Result<Pedal> {
    let type_str = get_attr(e, "type")?.unwrap_or_default();
    let pedal_type = match type_str.as_str() {
        "start" => PedalType::Start,
        "stop" => PedalType::Stop,
        "sostenuto" => PedalType::Sostenuto,
        "change" => PedalType::Change,
        "continue" => PedalType::Continue,
        "discontinue" => PedalType::Discontinue,
        "resume" => PedalType::Resume,
        _ => PedalType::Start,
    };

    Ok(Pedal {
        pedal_type,
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        line: get_attr(e, "line")?.and_then(|s| parse_yes_no_opt(&s)),
        sign: get_attr(e, "sign")?.and_then(|s| parse_yes_no_opt(&s)),
        abbreviated: get_attr(e, "abbreviated")?.and_then(|s| parse_yes_no_opt(&s)),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        relative_x: get_attr(e, "relative-x")?.and_then(|s| s.parse().ok()),
        relative_y: get_attr(e, "relative-y")?.and_then(|s| s.parse().ok()),
        halign: get_attr(e, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(e, "valign")?.and_then(|s| parse_valign(&s)),
        color: get_attr(e, "color")?,
        id: get_attr(e, "id")?,
    })
}

fn parse_octave_shift(e: &BytesStart) -> Result<OctaveShift> {
    let type_str = get_attr(e, "type")?.unwrap_or_default();
    let shift_type = match type_str.as_str() {
        "up" => OctaveShiftType::Up,
        "down" => OctaveShiftType::Down,
        "stop" => OctaveShiftType::Stop,
        "continue" => OctaveShiftType::Continue,
        _ => OctaveShiftType::Down,
    };

    Ok(OctaveShift {
        shift_type,
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        size: get_attr(e, "size")?.and_then(|s| s.parse().ok()),
        dash_length: get_attr(e, "dash-length")?.and_then(|s| s.parse().ok()),
        space_length: get_attr(e, "space-length")?.and_then(|s| s.parse().ok()),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        font_family: get_attr(e, "font-family")?,
        font_size: get_attr(e, "font-size")?.and_then(|s| s.parse().ok().map(FontSize::Points)),
        color: get_attr(e, "color")?,
        id: get_attr(e, "id")?,
    })
}

fn parse_dashes(e: &BytesStart) -> Result<Dashes> {
    let type_str = get_attr(e, "type")?.unwrap_or_default();
    let dash_type = match type_str.as_str() {
        "start" => StartStopContinue::Start,
        "stop" => StartStopContinue::Stop,
        "continue" => StartStopContinue::Continue,
        _ => StartStopContinue::Start,
    };

    Ok(Dashes {
        dash_type,
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        dash_length: get_attr(e, "dash-length")?.and_then(|s| s.parse().ok()),
        space_length: get_attr(e, "space-length")?.and_then(|s| s.parse().ok()),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        id: get_attr(e, "id")?,
    })
}

fn parse_bracket(e: &BytesStart) -> Result<Bracket> {
    let type_str = get_attr(e, "type")?.unwrap_or_default();
    let bracket_type = match type_str.as_str() {
        "start" => StartStopContinue::Start,
        "stop" => StartStopContinue::Stop,
        "continue" => StartStopContinue::Continue,
        _ => StartStopContinue::Start,
    };

    let line_end_str = get_attr(e, "line-end")?.unwrap_or_default();
    let line_end = match line_end_str.as_str() {
        "up" => LineEnd::Up,
        "down" => LineEnd::Down,
        "both" => LineEnd::Both,
        "arrow" => LineEnd::Arrow,
        "none" => LineEnd::None,
        _ => LineEnd::None,
    };

    Ok(Bracket {
        bracket_type,
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        line_end,
        end_length: get_attr(e, "end-length")?.and_then(|s| s.parse().ok()),
        line_type: get_attr(e, "line-type")?.and_then(|s| match s.as_str() {
            "solid" => Some(LineType::Solid),
            "dashed" => Some(LineType::Dashed),
            "dotted" => Some(LineType::Dotted),
            "wavy" => Some(LineType::Wavy),
            _ => None,
        }),
        dash_length: get_attr(e, "dash-length")?.and_then(|s| s.parse().ok()),
        space_length: get_attr(e, "space-length")?.and_then(|s| s.parse().ok()),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        id: get_attr(e, "id")?,
    })
}

pub(crate) fn parse_segno(e: &BytesStart) -> Result<Segno> {
    Ok(Segno {
        smufl: get_attr(e, "smufl")?,
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        halign: get_attr(e, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(e, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(e, "id")?,
    })
}

pub(crate) fn parse_coda(e: &BytesStart) -> Result<Coda> {
    Ok(Coda {
        smufl: get_attr(e, "smufl")?,
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        halign: get_attr(e, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(e, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(e, "id")?,
    })
}

fn parse_symbol<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Symbol> {
    let value = read_text(reader, b"symbol")?;
    Ok(Symbol {
        value,
        font_family: get_attr(start, "font-family")?,
        font_size: get_attr(start, "font-size")?.and_then(|s| s.parse().ok().map(FontSize::Points)),
        color: get_attr(start, "color")?,
        halign: get_attr(start, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(start, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(start, "id")?,
    })
}

fn parse_harp_pedals<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<HarpPedals> {
    let mut pedal_tunings = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"pedal-tuning" => pedal_tunings.push(parse_pedal_tuning(reader)?),
                _ => skip_element(reader, e)?,
            },
            Event::End(ref e) if e.name().as_ref() == b"harp-pedals" => break,
            Event::Eof => return Err(ParseError::MissingElement("harp-pedals end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(HarpPedals {
        pedal_tunings,
        default_x: get_attr(start, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(start, "default-y")?.and_then(|s| s.parse().ok()),
        halign: get_attr(start, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(start, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(start, "id")?,
    })
}

fn parse_pedal_tuning<R: BufRead>(reader: &mut Reader<R>) -> Result<PedalTuning> {
    let mut pedal_step = None;
    let mut pedal_alter = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"pedal-step" => pedal_step = Some(read_text(reader, b"pedal-step")?),
                b"pedal-alter" => {
                    pedal_alter = Some(
                        read_text(reader, b"pedal-alter")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("pedal-alter".to_string()))?,
                    )
                }
                _ => skip_element(reader, e)?,
            },
            Event::End(ref e) if e.name().as_ref() == b"pedal-tuning" => break,
            Event::Eof => return Err(ParseError::MissingElement("pedal-tuning end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(PedalTuning {
        pedal_step: pedal_step.unwrap_or_default(),
        pedal_alter: pedal_alter.unwrap_or(0.0),
    })
}

/// Parse an empty direction element that has only position/alignment attrs.
/// Returns fields needed for Damp/DampAll/Eyeglasses (they share the same shape).
fn parse_empty_direction_attrs(e: &BytesStart) -> Result<EmptyDirectionAttrs> {
    Ok(EmptyDirectionAttrs {
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        halign: get_attr(e, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(e, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(e, "id")?,
    })
}

struct EmptyDirectionAttrs {
    default_x: Option<f64>,
    default_y: Option<f64>,
    halign: Option<LeftCenterRight>,
    valign: Option<Valign>,
    id: Option<String>,
}

/// Generic parse for Damp/DampAll/Eyeglasses — they have identical attribute sets.
/// Returns a type that impls Into<Damp>, Into<DampAll>, Into<Eyeglasses> via the
/// identical field layout — we use a helper to avoid code duplication.
fn parse_empty_direction<T: From<EmptyDirectionAttrs>>(e: &BytesStart) -> Result<T> {
    Ok(T::from(parse_empty_direction_attrs(e)?))
}

impl From<EmptyDirectionAttrs> for Damp {
    fn from(a: EmptyDirectionAttrs) -> Self {
        Damp {
            default_x: a.default_x,
            default_y: a.default_y,
            halign: a.halign,
            valign: a.valign,
            id: a.id,
        }
    }
}

impl From<EmptyDirectionAttrs> for DampAll {
    fn from(a: EmptyDirectionAttrs) -> Self {
        DampAll {
            default_x: a.default_x,
            default_y: a.default_y,
            halign: a.halign,
            valign: a.valign,
            id: a.id,
        }
    }
}

impl From<EmptyDirectionAttrs> for Eyeglasses {
    fn from(a: EmptyDirectionAttrs) -> Self {
        Eyeglasses {
            default_x: a.default_x,
            default_y: a.default_y,
            halign: a.halign,
            valign: a.valign,
            id: a.id,
        }
    }
}

fn parse_string_mute(e: &BytesStart) -> Result<StringMute> {
    let mute_type = match get_attr(e, "type")?.unwrap_or_default().as_str() {
        "off" => StringMuteType::Off,
        _ => StringMuteType::On,
    };
    Ok(StringMute {
        mute_type,
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        halign: get_attr(e, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(e, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(e, "id")?,
    })
}

fn parse_scordatura<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Scordatura> {
    let mut accords = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"accord" => accords.push(parse_accord(reader, e)?),
                _ => skip_element(reader, e)?,
            },
            Event::End(ref e) if e.name().as_ref() == b"scordatura" => break,
            Event::Eof => return Err(ParseError::MissingElement("scordatura end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Scordatura {
        accords,
        id: get_attr(start, "id")?,
    })
}

fn parse_accord<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Accord> {
    let string: u32 = get_attr(start, "string")?
        .unwrap_or_default()
        .parse()
        .map_err(|_| ParseError::ParseNumber("accord string".to_string()))?;
    let mut tuning_step = None;
    let mut tuning_alter = None;
    let mut tuning_octave = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"tuning-step" => tuning_step = Some(read_text(reader, b"tuning-step")?),
                b"tuning-alter" => tuning_alter = read_text(reader, b"tuning-alter")?.parse().ok(),
                b"tuning-octave" => {
                    tuning_octave = Some(
                        read_text(reader, b"tuning-octave")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("tuning-octave".to_string()))?,
                    )
                }
                _ => skip_element(reader, e)?,
            },
            Event::End(ref e) if e.name().as_ref() == b"accord" => break,
            Event::Eof => return Err(ParseError::MissingElement("accord end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Accord {
        string,
        tuning_step: tuning_step.unwrap_or_default(),
        tuning_alter,
        tuning_octave: tuning_octave.unwrap_or(4),
    })
}

fn parse_image(e: &BytesStart) -> Result<DirectionImage> {
    Ok(DirectionImage {
        source: get_attr(e, "source")?.unwrap_or_default(),
        image_type: get_attr(e, "type")?.unwrap_or_default(),
        height: get_attr(e, "height")?.and_then(|s| s.parse().ok()),
        width: get_attr(e, "width")?.and_then(|s| s.parse().ok()),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        halign: get_attr(e, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(e, "valign")?.and_then(|s| parse_valign_image(&s)),
        id: get_attr(e, "id")?,
    })
}

fn parse_principal_voice<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<PrincipalVoice> {
    let voice_type = match get_attr(start, "type")?.unwrap_or_default().as_str() {
        "stop" => StartStop::Stop,
        _ => StartStop::Start,
    };
    let symbol = match get_attr(start, "symbol")?.unwrap_or_default().as_str() {
        "Nebenstimme" => PrincipalVoiceSymbol::Nebenstimme,
        "plain" => PrincipalVoiceSymbol::Plain,
        "none" => PrincipalVoiceSymbol::None,
        _ => PrincipalVoiceSymbol::Hauptstimme,
    };
    let text = read_text(reader, b"principal-voice")?;
    let value = if text.is_empty() { None } else { Some(text) };
    Ok(PrincipalVoice {
        value,
        voice_type,
        symbol,
        default_x: get_attr(start, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(start, "default-y")?.and_then(|s| s.parse().ok()),
        halign: get_attr(start, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(start, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(start, "id")?,
    })
}

fn parse_percussion<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Percussion> {
    let enclosure = get_attr(start, "enclosure")?.and_then(|s| parse_enclosure(&s));
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let halign = get_attr(start, "halign")?.and_then(|s| parse_lcr(&s));
    let valign = get_attr(start, "valign")?.and_then(|s| parse_valign(&s));
    let id = get_attr(start, "id")?;

    let mut content: Option<PercussionContent> = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"glass" => content = Some(PercussionContent::Glass(read_text(reader, b"glass")?)),
                b"metal" => content = Some(PercussionContent::Metal(read_text(reader, b"metal")?)),
                b"wood" => content = Some(PercussionContent::Wood(read_text(reader, b"wood")?)),
                b"pitched" => {
                    content = Some(PercussionContent::Pitched(read_text(reader, b"pitched")?))
                }
                b"membrane" => {
                    content = Some(PercussionContent::Membrane(read_text(reader, b"membrane")?))
                }
                b"effect" => {
                    content = Some(PercussionContent::Effect(read_text(reader, b"effect")?))
                }
                b"timpani" => {
                    content = Some(PercussionContent::Timpani);
                    skip_to_end(reader, b"timpani")?;
                }
                b"beater" => {
                    let tip = parse_tip_direction(e)?;
                    let value = read_text(reader, b"beater")?;
                    content = Some(PercussionContent::Beater(Beater { value, tip }));
                }
                b"stick" => {
                    content = Some(PercussionContent::Stick(parse_stick(reader, e)?));
                }
                b"stick-location" => {
                    content = Some(PercussionContent::StickLocation(read_text(
                        reader,
                        b"stick-location",
                    )?))
                }
                b"other-percussion" => {
                    content = Some(PercussionContent::OtherPercussion(read_text(
                        reader,
                        b"other-percussion",
                    )?))
                }
                _ => skip_element(reader, e)?,
            },
            Event::Empty(ref e) => {
                if e.name().as_ref() == b"timpani" {
                    content = Some(PercussionContent::Timpani);
                }
            }
            Event::End(ref e) if e.name().as_ref() == b"percussion" => break,
            Event::Eof => return Err(ParseError::MissingElement("percussion end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Percussion {
        content: content.unwrap_or(PercussionContent::OtherPercussion(String::new())),
        enclosure,
        default_x,
        default_y,
        halign,
        valign,
        id,
    })
}

fn parse_tip_direction(e: &BytesStart) -> Result<Option<TipDirection>> {
    Ok(get_attr(e, "tip")?.and_then(|s| match s.as_str() {
        "up" => Some(TipDirection::Up),
        "down" => Some(TipDirection::Down),
        "left" => Some(TipDirection::Left),
        "right" => Some(TipDirection::Right),
        "northwest" => Some(TipDirection::Northwest),
        "northeast" => Some(TipDirection::Northeast),
        "southeast" => Some(TipDirection::Southeast),
        "southwest" => Some(TipDirection::Southwest),
        _ => None,
    }))
}

fn parse_stick<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Stick> {
    let tip = parse_tip_direction(start)?;
    let parentheses = get_attr(start, "parentheses")?.and_then(|s| parse_yes_no_opt(&s));
    let dashed_circle = get_attr(start, "dashed-circle")?.and_then(|s| parse_yes_no_opt(&s));
    let mut stick_type = None;
    let mut stick_material = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"stick-type" => stick_type = Some(read_text(reader, b"stick-type")?),
                b"stick-material" => stick_material = Some(read_text(reader, b"stick-material")?),
                _ => skip_element(reader, e)?,
            },
            Event::End(ref e) if e.name().as_ref() == b"stick" => break,
            Event::Eof => return Err(ParseError::MissingElement("stick end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Stick {
        stick_type: stick_type.unwrap_or_default(),
        stick_material: stick_material.unwrap_or_default(),
        tip,
        parentheses,
        dashed_circle,
    })
}

fn parse_accordion_registration<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<AccordionRegistration> {
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let halign = get_attr(start, "halign")?.and_then(|s| parse_lcr(&s));
    let valign = get_attr(start, "valign")?.and_then(|s| parse_valign(&s));
    let id = get_attr(start, "id")?;

    let mut accordion_high = None;
    let mut accordion_middle = None;
    let mut accordion_low = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"accordion-high" => {
                    accordion_high = Some(true);
                    skip_to_end(reader, b"accordion-high")?;
                }
                b"accordion-middle" => {
                    accordion_middle = read_text(reader, b"accordion-middle")?.parse().ok();
                }
                b"accordion-low" => {
                    accordion_low = Some(true);
                    skip_to_end(reader, b"accordion-low")?;
                }
                _ => skip_element(reader, e)?,
            },
            Event::Empty(ref e) => match e.name().as_ref() {
                b"accordion-high" => accordion_high = Some(true),
                b"accordion-low" => accordion_low = Some(true),
                _ => {}
            },
            Event::End(ref e) if e.name().as_ref() == b"accordion-registration" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    "accordion-registration end".to_string(),
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(AccordionRegistration {
        accordion_high,
        accordion_middle,
        accordion_low,
        default_x,
        default_y,
        halign,
        valign,
        id,
    })
}

fn parse_staff_divide(e: &BytesStart) -> Result<StaffDivide> {
    let divide_type = match get_attr(e, "type")?.unwrap_or_default().as_str() {
        "up" => StaffDivideType::Up,
        "up-down" => StaffDivideType::UpDown,
        _ => StaffDivideType::Down,
    };
    Ok(StaffDivide {
        divide_type,
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        halign: get_attr(e, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(e, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(e, "id")?,
    })
}

fn parse_other_direction<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<OtherDirection> {
    let text = read_text(reader, b"other-direction")?;
    let value = if text.is_empty() { None } else { Some(text) };
    Ok(OtherDirection {
        value,
        print_object: get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s)),
        smufl: get_attr(start, "smufl")?,
        default_x: get_attr(start, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(start, "default-y")?.and_then(|s| s.parse().ok()),
        halign: get_attr(start, "halign")?.and_then(|s| parse_lcr(&s)),
        valign: get_attr(start, "valign")?.and_then(|s| parse_valign(&s)),
        id: get_attr(start, "id")?,
    })
}

fn parse_valign_image(s: &str) -> Option<ValignImage> {
    match s {
        "top" => Some(ValignImage::Top),
        "middle" => Some(ValignImage::Middle),
        "bottom" => Some(ValignImage::Bottom),
        _ => None,
    }
}

fn parse_offset<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Offset> {
    let sound = get_attr(start, "sound")?.and_then(|s| parse_yes_no_opt(&s));
    let value = read_text(reader, b"offset")?
        .parse()
        .map_err(|_| ParseError::ParseNumber("offset".to_string()))?;
    Ok(Offset { value, sound })
}

/// Parse sound attributes from a `<sound>` element tag.
pub(crate) fn parse_sound_attrs(e: &BytesStart) -> Result<Sound> {
    Ok(Sound {
        tempo: get_attr(e, "tempo")?.and_then(|s| s.parse().ok()),
        dynamics: get_attr(e, "dynamics")?.and_then(|s| s.parse().ok()),
        dacapo: get_attr(e, "dacapo")?.and_then(|s| parse_yes_no_opt(&s)),
        segno: get_attr(e, "segno")?,
        dalsegno: get_attr(e, "dalsegno")?,
        coda: get_attr(e, "coda")?,
        tocoda: get_attr(e, "tocoda")?,
        divisions: get_attr(e, "divisions")?.and_then(|s| s.parse().ok()),
        forward_repeat: get_attr(e, "forward-repeat")?.and_then(|s| parse_yes_no_opt(&s)),
        fine: get_attr(e, "fine")?,
        time_only: get_attr(e, "time-only")?,
        pizzicato: get_attr(e, "pizzicato")?.and_then(|s| parse_yes_no_opt(&s)),
        pan: get_attr(e, "pan")?.and_then(|s| s.parse().ok()),
        elevation: get_attr(e, "elevation")?.and_then(|s| s.parse().ok()),
        damper_pedal: get_attr(e, "damper-pedal")?,
        soft_pedal: get_attr(e, "soft-pedal")?,
        sostenuto_pedal: get_attr(e, "sostenuto-pedal")?,
        id: get_attr(e, "id")?,
        ..Default::default()
    })
}

/// Parse a `<sound>` element with children from a Start event.
pub(crate) fn parse_sound_full<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<Sound> {
    let mut sound = parse_sound_attrs(e)?;
    let mut buf = Vec::new();
    let mut current_group: Option<SoundMidiGroup> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref start) => match start.name().as_ref() {
                b"instrument-change" => {
                    // Flush any pending group if it has content
                    flush_midi_group(&mut sound, &mut current_group);
                    let ic = parse_instrument_change(reader, start)?;
                    current_group = Some(SoundMidiGroup {
                        instrument_change: Some(ic),
                        midi_device: None,
                        midi_instrument: None,
                        play: None,
                    });
                }
                b"midi-device" => {
                    let md = parse_midi_device_child(reader, start)?;
                    let group = current_group.get_or_insert(SoundMidiGroup {
                        instrument_change: None,
                        midi_device: None,
                        midi_instrument: None,
                        play: None,
                    });
                    group.midi_device = Some(md);
                }
                b"midi-instrument" => {
                    let mi = parse_midi_instrument_child(reader, start)?;
                    let group = current_group.get_or_insert(SoundMidiGroup {
                        instrument_change: None,
                        midi_device: None,
                        midi_instrument: None,
                        play: None,
                    });
                    group.midi_instrument = Some(mi);
                }
                b"play" => {
                    let play = parse_play(reader, start)?;
                    let group = current_group.get_or_insert(SoundMidiGroup {
                        instrument_change: None,
                        midi_device: None,
                        midi_instrument: None,
                        play: None,
                    });
                    group.play = Some(play);
                }
                b"swing" => {
                    sound.swing = Some(parse_swing(reader)?);
                }
                b"offset" => {
                    sound.offset = Some(parse_offset(reader, start)?);
                }
                _ => skip_element(reader, start)?,
            },
            Event::Empty(ref emp) => match emp.name().as_ref() {
                b"midi-device" => {
                    let md = parse_midi_device_empty(emp)?;
                    let group = current_group.get_or_insert(SoundMidiGroup {
                        instrument_change: None,
                        midi_device: None,
                        midi_instrument: None,
                        play: None,
                    });
                    group.midi_device = Some(md);
                }
                b"instrument-change" => {
                    flush_midi_group(&mut sound, &mut current_group);
                    let ic = InstrumentChange {
                        id: get_attr(emp, "id")?.unwrap_or_default(),
                        instrument_sound: None,
                        solo: None,
                        ensemble: None,
                        virtual_library: None,
                        virtual_name: None,
                    };
                    current_group = Some(SoundMidiGroup {
                        instrument_change: Some(ic),
                        midi_device: None,
                        midi_instrument: None,
                        play: None,
                    });
                }
                _ => {}
            },
            Event::End(ref end) if end.name().as_ref() == b"sound" => break,
            Event::Eof => return Err(ParseError::MissingElement("sound end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    // Flush final group
    flush_midi_group(&mut sound, &mut current_group);
    Ok(sound)
}

fn flush_midi_group(sound: &mut Sound, group: &mut Option<SoundMidiGroup>) {
    if let Some(g) = group.take() {
        sound.midi_instrument_changes.push(g);
    }
}

fn parse_instrument_change<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<InstrumentChange> {
    let id = get_attr(start, "id")?.unwrap_or_default();
    let mut instrument_sound = None;
    let mut solo = None;
    let mut ensemble = None;
    let mut virtual_library = None;
    let mut virtual_name = None;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"instrument-sound" => {
                    instrument_sound = Some(read_text(reader, b"instrument-sound")?)
                }
                b"solo" => {
                    read_text(reader, b"solo").ok();
                    solo = Some(true);
                }
                b"ensemble" => ensemble = Some(read_text(reader, b"ensemble")?),
                b"virtual-library" => {
                    virtual_library = Some(read_text(reader, b"virtual-library")?)
                }
                b"virtual-name" => virtual_name = Some(read_text(reader, b"virtual-name")?),
                _ => skip_element(reader, e)?,
            },
            Event::Empty(ref e) => {
                if e.name().as_ref() == b"solo" {
                    solo = Some(true);
                }
            }
            Event::End(ref e) if e.name().as_ref() == b"instrument-change" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    "instrument-change end".to_string(),
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(InstrumentChange {
        id,
        instrument_sound,
        solo,
        ensemble,
        virtual_library,
        virtual_name,
    })
}

fn parse_midi_device_child<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<MidiDevice> {
    let port = get_attr(start, "port")?.and_then(|s| s.parse().ok());
    let id = get_attr(start, "id")?;
    let value_text = read_text(reader, b"midi-device")?;
    let value = if value_text.is_empty() {
        None
    } else {
        Some(value_text)
    };
    Ok(MidiDevice { value, port, id })
}

fn parse_midi_device_empty(e: &BytesStart) -> Result<MidiDevice> {
    Ok(MidiDevice {
        value: None,
        port: get_attr(e, "port")?.and_then(|s| s.parse().ok()),
        id: get_attr(e, "id")?,
    })
}

fn parse_midi_instrument_child<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<MidiInstrument> {
    let id = get_attr(start, "id")?.unwrap_or_default();
    let mut mi = MidiInstrument {
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
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"midi-channel" => {
                    mi.midi_channel = read_text(reader, b"midi-channel")?.parse().ok()
                }
                b"midi-name" => mi.midi_name = Some(read_text(reader, b"midi-name")?),
                b"midi-bank" => mi.midi_bank = read_text(reader, b"midi-bank")?.parse().ok(),
                b"midi-program" => {
                    mi.midi_program = read_text(reader, b"midi-program")?.parse().ok()
                }
                b"midi-unpitched" => {
                    mi.midi_unpitched = read_text(reader, b"midi-unpitched")?.parse().ok()
                }
                b"volume" => mi.volume = read_text(reader, b"volume")?.parse().ok(),
                b"pan" => mi.pan = read_text(reader, b"pan")?.parse().ok(),
                b"elevation" => mi.elevation = read_text(reader, b"elevation")?.parse().ok(),
                _ => skip_element(reader, e)?,
            },
            Event::End(ref e) if e.name().as_ref() == b"midi-instrument" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    "midi-instrument end".to_string(),
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(mi)
}

fn parse_play<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Play> {
    let id = get_attr(start, "id")?;
    let mut entries = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"ipa" => entries.push(PlayEntry::Ipa(read_text(reader, b"ipa")?)),
                b"mute" => entries.push(PlayEntry::Mute(read_text(reader, b"mute")?)),
                b"semi-pitched" => {
                    entries.push(PlayEntry::SemiPitched(read_text(reader, b"semi-pitched")?))
                }
                b"other-play" => {
                    let play_type = get_attr(e, "type")?.unwrap_or_default();
                    let value = read_text(reader, b"other-play")?;
                    entries.push(PlayEntry::OtherPlay(OtherPlay { play_type, value }));
                }
                _ => skip_element(reader, e)?,
            },
            Event::End(ref e) if e.name().as_ref() == b"play" => break,
            Event::Eof => return Err(ParseError::MissingElement("play end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Play { id, entries })
}

fn parse_swing<R: BufRead>(reader: &mut Reader<R>) -> Result<Swing> {
    let mut buf = Vec::new();
    let mut is_straight = false;
    let mut first: Option<u32> = None;
    let mut second: Option<u32> = None;
    let mut swing_type: Option<String> = None;
    let mut swing_style: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(ref e) => match e.name().as_ref() {
                b"first" => first = read_text(reader, b"first")?.parse().ok(),
                b"second" => second = read_text(reader, b"second")?.parse().ok(),
                b"swing-type" => swing_type = Some(read_text(reader, b"swing-type")?),
                b"swing-style" => swing_style = Some(read_text(reader, b"swing-style")?),
                b"straight" => {
                    read_text(reader, b"straight").ok();
                    is_straight = true;
                }
                _ => skip_element(reader, e)?,
            },
            Event::Empty(ref e) => {
                if e.name().as_ref() == b"straight" {
                    is_straight = true;
                }
            }
            Event::End(ref e) if e.name().as_ref() == b"swing" => break,
            Event::Eof => return Err(ParseError::MissingElement("swing end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    let content = if is_straight {
        SwingContent::Straight
    } else {
        SwingContent::Ratio(SwingRatio {
            first: first.unwrap_or(1),
            second: second.unwrap_or(1),
            swing_type,
        })
    };

    Ok(Swing {
        content,
        swing_style,
    })
}

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

fn parse_lcr(s: &str) -> Option<LeftCenterRight> {
    match s {
        "left" => Some(LeftCenterRight::Left),
        "center" => Some(LeftCenterRight::Center),
        "right" => Some(LeftCenterRight::Right),
        _ => None,
    }
}

fn parse_valign(s: &str) -> Option<Valign> {
    match s {
        "top" => Some(Valign::Top),
        "middle" => Some(Valign::Middle),
        "bottom" => Some(Valign::Bottom),
        "baseline" => Some(Valign::Baseline),
        _ => None,
    }
}

fn parse_enclosure(s: &str) -> Option<EnclosureShape> {
    match s {
        "rectangle" => Some(EnclosureShape::Rectangle),
        "square" => Some(EnclosureShape::Square),
        "oval" => Some(EnclosureShape::Oval),
        "circle" => Some(EnclosureShape::Circle),
        "bracket" => Some(EnclosureShape::Bracket),
        "triangle" => Some(EnclosureShape::Triangle),
        "diamond" => Some(EnclosureShape::Diamond),
        "none" => Some(EnclosureShape::None),
        _ => None,
    }
}

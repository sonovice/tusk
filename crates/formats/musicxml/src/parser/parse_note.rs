//! Note, backup, and forward parsing.

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::parse_direction::parse_dynamics;
use super::parse_listening::parse_listen;
use super::parse_notations::{
    parse_arpeggiate, parse_fermata_empty, parse_fermata_start, parse_glissando,
    parse_glissando_empty, parse_non_arpeggiate, parse_other_notation_empty,
    parse_other_notation_start, parse_placement_attr, parse_slide, parse_slide_empty,
};
use super::parse_technical::parse_technical;
use super::{ParseError, Result, get_attr, get_attr_required, read_text, skip_element};
use crate::model::data::*;
use crate::model::direction::Play;
use crate::model::elements::Empty;
use crate::model::listening::Listen;
use crate::model::lyric::*;
use crate::model::notations::*;
use crate::model::note::*;

pub fn parse_note<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Note> {
    let mut buf = Vec::new();

    // Parse attributes
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let relative_x = get_attr(start, "relative-x")?.and_then(|s| s.parse().ok());
    let relative_y = get_attr(start, "relative-y")?.and_then(|s| s.parse().ok());
    let dynamics = get_attr(start, "dynamics")?.and_then(|s| s.parse().ok());
    let end_dynamics = get_attr(start, "end-dynamics")?.and_then(|s| s.parse().ok());
    let attack = get_attr(start, "attack")?.and_then(|s| s.parse().ok());
    let release = get_attr(start, "release")?.and_then(|s| s.parse().ok());
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
    let print_leger = get_attr(start, "print-leger")?.and_then(|s| parse_yes_no_opt(&s));
    let print_spacing = get_attr(start, "print-spacing")?.and_then(|s| parse_yes_no_opt(&s));
    let pizzicato = get_attr(start, "pizzicato")?.and_then(|s| parse_yes_no_opt(&s));
    let color = get_attr(start, "color")?;
    let note_id = get_attr(start, "id")?;

    let mut grace: Option<Grace> = None;
    let mut cue: Option<Empty> = None;
    let mut chord: Option<Empty> = None;
    let mut pitch: Option<Pitch> = None;
    let mut unpitched: Option<Unpitched> = None;
    let mut rest: Option<Rest> = None;
    let mut duration: Option<f64> = None;
    let mut ties: Vec<Tie> = Vec::new();
    let mut voice: Option<String> = None;
    let mut instruments: Vec<Instrument> = Vec::new();
    let mut note_type: Option<NoteType> = None;
    let mut dots: Vec<Dot> = Vec::new();
    let mut accidental: Option<Accidental> = None;
    let mut time_modification: Option<TimeModification> = None;
    let mut stem: Option<Stem> = None;
    let mut staff: Option<u32> = None;
    let mut beams: Vec<Beam> = Vec::new();
    let mut notehead: Option<Notehead> = None;
    let mut notehead_text: Option<NoteheadText> = None;
    let mut notations: Option<Notations> = None;
    let mut lyrics: Vec<Lyric> = Vec::new();
    let mut play: Option<Play> = None;
    let mut listen: Option<Listen> = None;
    let mut footnote: Option<FormattedText> = None;
    let mut level: Option<Level> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"grace" => {
                    grace = Some(parse_grace(&e)?);
                    skip_to_end(reader, b"grace")?;
                }
                b"pitch" => pitch = Some(parse_pitch(reader)?),
                b"unpitched" => unpitched = Some(parse_unpitched(reader)?),
                b"rest" => rest = Some(parse_rest(reader, &e)?),
                b"duration" => {
                    duration = Some(
                        read_text(reader, b"duration")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("duration".to_string()))?,
                    )
                }
                b"tie" => {
                    ties.push(parse_tie(&e)?);
                    skip_to_end(reader, b"tie")?;
                }
                b"voice" => voice = Some(read_text(reader, b"voice")?),
                b"instrument" => {
                    if let Some(id) = get_attr(&e, "id")? {
                        instruments.push(Instrument::new(&id));
                    }
                    skip_to_end(reader, b"instrument")?;
                }
                b"type" => note_type = Some(parse_note_type(reader, &e)?),
                b"dot" => {
                    dots.push(parse_dot(&e)?);
                    skip_to_end(reader, b"dot")?;
                }
                b"accidental" => accidental = Some(parse_accidental(reader, &e)?),
                b"time-modification" => time_modification = Some(parse_time_modification(reader)?),
                b"stem" => stem = Some(parse_stem(reader, &e)?),
                b"notehead" => notehead = Some(parse_notehead(reader, &e)?),
                b"notehead-text" => notehead_text = Some(parse_notehead_text(reader)?),
                b"staff" => {
                    staff = Some(
                        read_text(reader, b"staff")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staff".to_string()))?,
                    )
                }
                b"beam" => beams.push(parse_beam(reader, &e)?),
                b"notations" => notations = Some(parse_notations(reader)?),
                b"lyric" => lyrics.push(parse_lyric(reader, &e)?),
                b"play" => play = Some(parse_note_play(reader, &e)?),
                b"listen" => listen = Some(parse_listen(reader)?),
                b"footnote" => footnote = Some(parse_formatted_text(reader, &e, b"footnote")?),
                b"level" => level = Some(parse_level(reader, &e)?),
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"grace" => grace = Some(parse_grace(&e)?),
                b"cue" => cue = Some(Empty),
                b"chord" => chord = Some(Empty),
                b"rest" => rest = Some(parse_rest_empty(&e)?),
                b"tie" => ties.push(parse_tie(&e)?),
                b"dot" => dots.push(parse_dot(&e)?),
                b"instrument" => {
                    if let Some(id) = get_attr(&e, "id")? {
                        instruments.push(Instrument::new(&id));
                    }
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"note" => break,
            Event::Eof => return Err(ParseError::MissingElement("note end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    // Determine content type
    let content = if let Some(p) = pitch {
        FullNoteContent::Pitch(p)
    } else if let Some(u) = unpitched {
        FullNoteContent::Unpitched(u)
    } else if let Some(r) = rest {
        FullNoteContent::Rest(r)
    } else {
        // Default to a rest if nothing specified
        FullNoteContent::Rest(Rest::default())
    };

    Ok(Note {
        grace,
        cue,
        chord,
        content,
        duration,
        ties,
        footnote,
        level,
        voice,
        instruments,
        note_type,
        dots,
        accidental,
        time_modification,
        stem,
        notehead,
        notehead_text,
        staff,
        beams,
        notations,
        lyrics,
        play,
        listen,
        default_x,
        default_y,
        relative_x,
        relative_y,
        print_object,
        print_leger,
        print_spacing,
        dynamics,
        end_dynamics,
        attack,
        release,
        pizzicato,
        color,
        id: note_id,
    })
}

fn parse_grace(e: &BytesStart) -> Result<Grace> {
    Ok(Grace {
        steal_time_previous: get_attr(e, "steal-time-previous")?.and_then(|s| s.parse().ok()),
        steal_time_following: get_attr(e, "steal-time-following")?.and_then(|s| s.parse().ok()),
        make_time: get_attr(e, "make-time")?.and_then(|s| s.parse().ok()),
        slash: get_attr(e, "slash")?.and_then(|s| parse_yes_no_opt(&s)),
    })
}

fn parse_pitch<R: BufRead>(reader: &mut Reader<R>) -> Result<Pitch> {
    let mut buf = Vec::new();
    let mut step = Step::C;
    let mut alter: Option<f64> = None;
    let mut octave: u8 = 4;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"step" => {
                    let s = read_text(reader, b"step")?;
                    step = match s.as_str() {
                        "A" => Step::A,
                        "B" => Step::B,
                        "C" => Step::C,
                        "D" => Step::D,
                        "E" => Step::E,
                        "F" => Step::F,
                        "G" => Step::G,
                        _ => return Err(ParseError::InvalidContent("step".to_string(), s)),
                    };
                }
                b"alter" => {
                    alter = Some(
                        read_text(reader, b"alter")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("alter".to_string()))?,
                    )
                }
                b"octave" => {
                    octave = read_text(reader, b"octave")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("octave".to_string()))?
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"pitch" => break,
            Event::Eof => return Err(ParseError::MissingElement("pitch end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Pitch {
        step,
        alter,
        octave,
    })
}

fn parse_unpitched<R: BufRead>(reader: &mut Reader<R>) -> Result<Unpitched> {
    let mut buf = Vec::new();
    let mut unpitched = Unpitched::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"display-step" => {
                    let s = read_text(reader, b"display-step")?;
                    unpitched.display_step = Some(match s.as_str() {
                        "A" => Step::A,
                        "B" => Step::B,
                        "C" => Step::C,
                        "D" => Step::D,
                        "E" => Step::E,
                        "F" => Step::F,
                        "G" => Step::G,
                        _ => return Err(ParseError::InvalidContent("display-step".to_string(), s)),
                    });
                }
                b"display-octave" => {
                    unpitched.display_octave = Some(
                        read_text(reader, b"display-octave")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("display-octave".to_string()))?,
                    )
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"unpitched" => break,
            Event::Eof => return Err(ParseError::MissingElement("unpitched end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(unpitched)
}

fn parse_rest<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Rest> {
    let mut buf = Vec::new();
    let mut rest = Rest {
        measure: get_attr(start, "measure")?.and_then(|s| parse_yes_no_opt(&s)),
        ..Default::default()
    };

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"display-step" => {
                    let s = read_text(reader, b"display-step")?;
                    rest.display_step = Some(match s.as_str() {
                        "A" => Step::A,
                        "B" => Step::B,
                        "C" => Step::C,
                        "D" => Step::D,
                        "E" => Step::E,
                        "F" => Step::F,
                        "G" => Step::G,
                        _ => return Err(ParseError::InvalidContent("display-step".to_string(), s)),
                    });
                }
                b"display-octave" => {
                    rest.display_octave = Some(
                        read_text(reader, b"display-octave")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("display-octave".to_string()))?,
                    )
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"rest" => break,
            Event::Eof => return Err(ParseError::MissingElement("rest end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(rest)
}

fn parse_rest_empty(start: &BytesStart) -> Result<Rest> {
    Ok(Rest {
        measure: get_attr(start, "measure")?.and_then(|s| parse_yes_no_opt(&s)),
        ..Default::default()
    })
}

fn parse_tie(e: &BytesStart) -> Result<Tie> {
    let tie_type = match get_attr_required(e, "type")?.as_str() {
        "start" => StartStop::Start,
        "stop" => StartStop::Stop,
        s => {
            return Err(ParseError::InvalidAttribute(
                "type".to_string(),
                s.to_string(),
            ));
        }
    };
    Ok(Tie {
        tie_type,
        time_only: get_attr(e, "time-only")?,
    })
}

fn parse_note_type<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<NoteType> {
    let size = get_attr(start, "size")?.and_then(|s| match s.as_str() {
        "cue" => Some(SymbolSize::Cue),
        "full" => Some(SymbolSize::Full),
        "grace-cue" => Some(SymbolSize::GraceCue),
        "large" => Some(SymbolSize::Large),
        _ => None,
    });
    let value_str = read_text(reader, b"type")?;
    let value = match value_str.as_str() {
        "1024th" => NoteTypeValue::N1024th,
        "512th" => NoteTypeValue::N512th,
        "256th" => NoteTypeValue::N256th,
        "128th" => NoteTypeValue::N128th,
        "64th" => NoteTypeValue::N64th,
        "32nd" => NoteTypeValue::N32nd,
        "16th" => NoteTypeValue::N16th,
        "eighth" => NoteTypeValue::Eighth,
        "quarter" => NoteTypeValue::Quarter,
        "half" => NoteTypeValue::Half,
        "whole" => NoteTypeValue::Whole,
        "breve" => NoteTypeValue::Breve,
        "long" => NoteTypeValue::Long,
        "maxima" => NoteTypeValue::Maxima,
        _ => return Err(ParseError::InvalidContent("type".to_string(), value_str)),
    };
    Ok(NoteType { value, size })
}

fn parse_dot(e: &BytesStart) -> Result<Dot> {
    Ok(Dot {
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        relative_x: get_attr(e, "relative-x")?.and_then(|s| s.parse().ok()),
        relative_y: get_attr(e, "relative-y")?.and_then(|s| s.parse().ok()),
        placement: get_attr(e, "placement")?.and_then(|s| match s.as_str() {
            "above" => Some(AboveBelow::Above),
            "below" => Some(AboveBelow::Below),
            _ => None,
        }),
        color: get_attr(e, "color")?,
    })
}

/// Parse an accidental value string into AccidentalValue enum.
pub(crate) fn parse_accidental_value(s: &str) -> Result<AccidentalValue> {
    Ok(match s {
        "sharp" => AccidentalValue::Sharp,
        "natural" => AccidentalValue::Natural,
        "flat" => AccidentalValue::Flat,
        "double-sharp" => AccidentalValue::DoubleSharp,
        "sharp-sharp" => AccidentalValue::SharpSharp,
        "flat-flat" => AccidentalValue::FlatFlat,
        "natural-sharp" => AccidentalValue::NaturalSharp,
        "natural-flat" => AccidentalValue::NaturalFlat,
        "quarter-flat" => AccidentalValue::QuarterFlat,
        "quarter-sharp" => AccidentalValue::QuarterSharp,
        "three-quarters-flat" => AccidentalValue::ThreeQuartersFlat,
        "three-quarters-sharp" => AccidentalValue::ThreeQuartersSharp,
        "sharp-down" => AccidentalValue::SharpDown,
        "sharp-up" => AccidentalValue::SharpUp,
        "natural-down" => AccidentalValue::NaturalDown,
        "natural-up" => AccidentalValue::NaturalUp,
        "flat-down" => AccidentalValue::FlatDown,
        "flat-up" => AccidentalValue::FlatUp,
        "double-sharp-down" => AccidentalValue::DoubleSharpDown,
        "double-sharp-up" => AccidentalValue::DoubleSharpUp,
        "flat-flat-down" => AccidentalValue::FlatFlatDown,
        "flat-flat-up" => AccidentalValue::FlatFlatUp,
        "arrow-down" => AccidentalValue::ArrowDown,
        "arrow-up" => AccidentalValue::ArrowUp,
        "triple-sharp" => AccidentalValue::TripleSharp,
        "triple-flat" => AccidentalValue::TripleFlat,
        "slash-quarter-sharp" => AccidentalValue::SlashQuarterSharp,
        "slash-sharp" => AccidentalValue::SlashSharp,
        "slash-flat" => AccidentalValue::SlashFlat,
        "double-slash-flat" => AccidentalValue::DoubleSlashFlat,
        "sharp-1" => AccidentalValue::Sharp1,
        "sharp-2" => AccidentalValue::Sharp2,
        "sharp-3" => AccidentalValue::Sharp3,
        "sharp-5" => AccidentalValue::Sharp5,
        "flat-1" => AccidentalValue::Flat1,
        "flat-2" => AccidentalValue::Flat2,
        "flat-3" => AccidentalValue::Flat3,
        "flat-4" => AccidentalValue::Flat4,
        "sori" => AccidentalValue::Sori,
        "koron" => AccidentalValue::Koron,
        _ => AccidentalValue::Other,
    })
}

fn parse_accidental<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Accidental> {
    let cautionary = get_attr(start, "cautionary")?.and_then(|s| parse_yes_no_opt(&s));
    let editorial = get_attr(start, "editorial")?.and_then(|s| parse_yes_no_opt(&s));
    let parentheses = get_attr(start, "parentheses")?.and_then(|s| parse_yes_no_opt(&s));
    let bracket = get_attr(start, "bracket")?.and_then(|s| parse_yes_no_opt(&s));

    let value_str = read_text(reader, b"accidental")?;
    let value = parse_accidental_value(&value_str)?;

    Ok(Accidental {
        value,
        cautionary,
        editorial,
        parentheses,
        bracket,
        size: None,
        smufl: None,
    })
}

fn parse_time_modification<R: BufRead>(reader: &mut Reader<R>) -> Result<TimeModification> {
    let mut buf = Vec::new();
    let mut actual_notes = 1;
    let mut normal_notes = 1;
    let mut normal_type: Option<NoteTypeValue> = None;
    let mut normal_dots: Vec<Empty> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"actual-notes" => {
                    actual_notes = read_text(reader, b"actual-notes")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("actual-notes".to_string()))?
                }
                b"normal-notes" => {
                    normal_notes = read_text(reader, b"normal-notes")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("normal-notes".to_string()))?
                }
                b"normal-type" => {
                    let s = read_text(reader, b"normal-type")?;
                    normal_type = Some(parse_note_type_value(&s)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"normal-dot" => {
                normal_dots.push(Empty);
            }
            Event::End(e) if e.name().as_ref() == b"time-modification" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement(
                    "time-modification end".to_string(),
                ));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(TimeModification {
        actual_notes,
        normal_notes,
        normal_type,
        normal_dots,
    })
}

fn parse_stem<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Stem> {
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let value_str = read_text(reader, b"stem")?;
    let value = match value_str.as_str() {
        "up" => StemValue::Up,
        "down" => StemValue::Down,
        "double" => StemValue::Double,
        "none" => StemValue::None,
        _ => StemValue::Up,
    };
    Ok(Stem {
        value,
        default_y,
        relative_y: None,
        color: None,
    })
}

fn parse_beam<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Beam> {
    let number = get_attr(start, "number")?.and_then(|s| s.parse().ok());
    let repeater = get_attr(start, "repeater")?.and_then(|s| parse_yes_no_opt(&s));
    let fan = get_attr(start, "fan")?.and_then(|s| match s.as_str() {
        "accel" => Some(Fan::Accel),
        "rit" => Some(Fan::Rit),
        "none" => Some(Fan::None),
        _ => Option::None,
    });
    let color = get_attr(start, "color")?;
    let id = get_attr(start, "id")?;
    let value_str = read_text(reader, b"beam")?;
    let value = match value_str.as_str() {
        "begin" => BeamValue::Begin,
        "continue" => BeamValue::Continue,
        "end" => BeamValue::End,
        "forward hook" => BeamValue::ForwardHook,
        "backward hook" => BeamValue::BackwardHook,
        _ => BeamValue::Begin,
    };
    Ok(Beam {
        value,
        number,
        repeater,
        fan,
        color,
        id,
    })
}

pub fn parse_backup<R: BufRead>(reader: &mut Reader<R>) -> Result<Backup> {
    let mut buf = Vec::new();
    let mut duration = 0.0;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                if e.name().as_ref() == b"duration" {
                    duration = read_text(reader, b"duration")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("duration".to_string()))?;
                } else {
                    skip_element(reader, &e)?;
                }
            }
            Event::End(e) if e.name().as_ref() == b"backup" => break,
            Event::Eof => return Err(ParseError::MissingElement("backup end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Backup {
        duration,
        footnote: None,
        level: None,
    })
}

pub fn parse_forward<R: BufRead>(reader: &mut Reader<R>) -> Result<Forward> {
    let mut buf = Vec::new();
    let mut duration = 0.0;
    let mut voice: Option<String> = None;
    let mut staff: Option<u32> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"duration" => {
                    duration = read_text(reader, b"duration")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("duration".to_string()))?
                }
                b"voice" => voice = Some(read_text(reader, b"voice")?),
                b"staff" => {
                    staff = Some(
                        read_text(reader, b"staff")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staff".to_string()))?,
                    )
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"forward" => break,
            Event::Eof => return Err(ParseError::MissingElement("forward end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Forward {
        duration,
        footnote: None,
        level: None,
        voice,
        staff,
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

// ============================================================================
// Notehead Parsing
// ============================================================================

fn parse_notehead<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Notehead> {
    let filled = get_attr(start, "filled")?.and_then(|s| parse_yes_no_opt(&s));
    let parentheses = get_attr(start, "parentheses")?.and_then(|s| parse_yes_no_opt(&s));
    let font_family = get_attr(start, "font-family")?;
    let font_style = get_attr(start, "font-style")?.and_then(|s| match s.as_str() {
        "normal" => Some(FontStyle::Normal),
        "italic" => Some(FontStyle::Italic),
        _ => None,
    });
    let font_size =
        get_attr(start, "font-size")?.and_then(|s| s.parse::<f64>().ok().map(FontSize::Points));
    let font_weight = get_attr(start, "font-weight")?.and_then(|s| match s.as_str() {
        "normal" => Some(FontWeight::Normal),
        "bold" => Some(FontWeight::Bold),
        _ => None,
    });
    let color = get_attr(start, "color")?;
    let smufl = get_attr(start, "smufl")?;

    let value_str = read_text(reader, b"notehead")?;
    let value = parse_notehead_value(&value_str);

    Ok(Notehead {
        value,
        filled,
        parentheses,
        font_family,
        font_style,
        font_size,
        font_weight,
        color,
        smufl,
    })
}

fn parse_notehead_value(s: &str) -> NoteheadValue {
    match s {
        "slash" => NoteheadValue::Slash,
        "triangle" => NoteheadValue::Triangle,
        "diamond" => NoteheadValue::Diamond,
        "square" => NoteheadValue::Square,
        "cross" => NoteheadValue::Cross,
        "x" => NoteheadValue::X,
        "circle-x" => NoteheadValue::CircleX,
        "inverted triangle" => NoteheadValue::InvertedTriangle,
        "arrow down" => NoteheadValue::ArrowDown,
        "arrow up" => NoteheadValue::ArrowUp,
        "circled" => NoteheadValue::Circled,
        "slashed" => NoteheadValue::Slashed,
        "back slashed" => NoteheadValue::BackSlashed,
        "normal" => NoteheadValue::Normal,
        "cluster" => NoteheadValue::Cluster,
        "circle dot" => NoteheadValue::CircleDot,
        "left triangle" => NoteheadValue::LeftTriangle,
        "rectangle" => NoteheadValue::Rectangle,
        "none" => NoteheadValue::None,
        "do" => NoteheadValue::Do,
        "re" => NoteheadValue::Re,
        "mi" => NoteheadValue::Mi,
        "fa" => NoteheadValue::Fa,
        "fa up" => NoteheadValue::FaUp,
        "so" => NoteheadValue::So,
        "la" => NoteheadValue::La,
        "ti" => NoteheadValue::Ti,
        _ => NoteheadValue::Other,
    }
}

// ============================================================================
// Notehead Text Parsing
// ============================================================================

fn parse_notehead_text<R: BufRead>(reader: &mut Reader<R>) -> Result<NoteheadText> {
    let mut buf = Vec::new();
    let mut children = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"display-text" => {
                    let ft = parse_formatted_text(reader, &e, b"display-text")?;
                    children.push(NoteheadTextChild::DisplayText(ft));
                }
                b"accidental-text" => {
                    let value_str = read_text(reader, b"accidental-text")?;
                    let value = parse_accidental_value(&value_str)?;
                    let smufl = get_attr(&e, "smufl")?;
                    children.push(NoteheadTextChild::AccidentalText(AccidentalText {
                        value,
                        smufl,
                    }));
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"notehead-text" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement("notehead-text end".to_string()));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(NoteheadText { children })
}

// ============================================================================
// Play Parsing (note-level)
// ============================================================================

fn parse_note_play<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Play> {
    use crate::model::direction::{OtherPlay, PlayEntry};

    let id = get_attr(start, "id")?;
    let mut entries = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"ipa" => {
                    entries.push(PlayEntry::Ipa(read_text(reader, b"ipa")?));
                }
                b"mute" => {
                    entries.push(PlayEntry::Mute(read_text(reader, b"mute")?));
                }
                b"semi-pitched" => {
                    entries.push(PlayEntry::SemiPitched(read_text(reader, b"semi-pitched")?));
                }
                b"other-play" => {
                    let play_type = get_attr(&e, "type")?.unwrap_or_default();
                    let value = read_text(reader, b"other-play")?;
                    entries.push(PlayEntry::OtherPlay(OtherPlay { play_type, value }));
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"play" => break,
            Event::Eof => return Err(ParseError::MissingElement("play end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Play { id, entries })
}

// ============================================================================
// Editorial Parsing (footnote, level)
// ============================================================================

fn parse_formatted_text<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    end_tag: &[u8],
) -> Result<FormattedText> {
    let lang = get_attr(start, "xml:lang")?;
    let dir = get_attr(start, "dir")?.and_then(|s| match s.as_str() {
        "ltr" => Some(TextDirection::Ltr),
        "rtl" => Some(TextDirection::Rtl),
        "lro" => Some(TextDirection::Lro),
        "rlo" => Some(TextDirection::Rlo),
        _ => None,
    });
    let enclosure = get_attr(start, "enclosure")?.and_then(|s| match s.as_str() {
        "rectangle" => Some(EnclosureShape::Rectangle),
        "square" => Some(EnclosureShape::Square),
        "oval" => Some(EnclosureShape::Oval),
        "circle" => Some(EnclosureShape::Circle),
        "bracket" => Some(EnclosureShape::Bracket),
        "inverted-bracket" => Some(EnclosureShape::InvertedBracket),
        "triangle" => Some(EnclosureShape::Triangle),
        "diamond" => Some(EnclosureShape::Diamond),
        "pentagon" => Some(EnclosureShape::Pentagon),
        "hexagon" => Some(EnclosureShape::Hexagon),
        "heptagon" => Some(EnclosureShape::Heptagon),
        "octagon" => Some(EnclosureShape::Octagon),
        "nonagon" => Some(EnclosureShape::Nonagon),
        "decagon" => Some(EnclosureShape::Decagon),
        "none" => Some(EnclosureShape::None),
        _ => None,
    });

    let value = read_text(reader, end_tag)?;

    Ok(FormattedText {
        value,
        lang,
        dir,
        enclosure,
    })
}

fn parse_level<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Level> {
    let parentheses = get_attr(start, "parentheses")?.and_then(|s| parse_yes_no_opt(&s));
    let bracket = get_attr(start, "bracket")?.and_then(|s| parse_yes_no_opt(&s));
    let reference = get_attr(start, "reference")?.and_then(|s| parse_yes_no_opt(&s));
    let value = read_text(reader, b"level")?;

    Ok(Level {
        value,
        parentheses,
        bracket,
        reference,
    })
}

fn parse_yes_no_opt(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

// ============================================================================
// Notations Parsing
// ============================================================================

fn parse_notations<R: BufRead>(reader: &mut Reader<R>) -> Result<Notations> {
    let mut buf = Vec::new();
    let mut notations = Notations::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"footnote" => {
                    notations.footnote = Some(parse_formatted_text(reader, &e, b"footnote")?);
                }
                b"level" => {
                    notations.level = Some(parse_level(reader, &e)?);
                }
                b"slur" => {
                    notations.slurs.push(parse_slur(&e)?);
                    skip_to_end(reader, b"slur")?;
                }
                b"tied" => {
                    notations.tied.push(parse_tied(&e)?);
                    skip_to_end(reader, b"tied")?;
                }
                b"tuplet" => {
                    notations.tuplets.push(parse_tuplet(reader, &e)?);
                }
                b"articulations" => {
                    notations.articulations = Some(parse_articulations(reader)?);
                }
                b"ornaments" => {
                    notations.ornaments = Some(parse_ornaments(reader)?);
                }
                b"technical" => {
                    notations.technical = Some(parse_technical(reader)?);
                }
                b"dynamics" => {
                    let placement = parse_placement_attr(&e);
                    let mut dyn_elem = parse_dynamics(reader)?;
                    dyn_elem.placement = placement;
                    notations.dynamics.push(dyn_elem);
                }
                b"fermata" => {
                    notations.fermatas.push(parse_fermata_start(reader, &e)?);
                }
                b"glissando" => {
                    notations.glissandos.push(parse_glissando(reader, &e)?);
                }
                b"slide" => {
                    notations.slides.push(parse_slide(reader, &e)?);
                }
                b"accidental-mark" => {
                    notations
                        .accidental_marks
                        .push(parse_accidental_mark(reader, &e)?);
                }
                b"other-notation" => {
                    notations
                        .other_notations
                        .push(parse_other_notation_start(reader, &e)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"slur" => notations.slurs.push(parse_slur(&e)?),
                b"tied" => notations.tied.push(parse_tied(&e)?),
                b"tuplet" => notations.tuplets.push(parse_tuplet_empty(&e)?),
                b"fermata" => notations.fermatas.push(parse_fermata_empty(&e)?),
                b"dynamics" => {
                    notations.dynamics.push(crate::model::direction::Dynamics {
                        values: Vec::new(),
                        placement: parse_placement_attr(&e),
                    });
                }
                b"arpeggiate" => {
                    notations.arpeggiate = Some(parse_arpeggiate(&e)?);
                }
                b"non-arpeggiate" => {
                    notations.non_arpeggiate = Some(parse_non_arpeggiate(&e)?);
                }
                b"glissando" => {
                    notations.glissandos.push(parse_glissando_empty(&e)?);
                }
                b"slide" => {
                    notations.slides.push(parse_slide_empty(&e)?);
                }
                b"accidental-mark" => {
                    notations.accidental_marks.push(AccidentalMark {
                        value: String::new(),
                        placement: parse_placement_attr(&e),
                    });
                }
                b"other-notation" => {
                    notations
                        .other_notations
                        .push(parse_other_notation_empty(&e)?);
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"notations" => break,
            Event::Eof => return Err(ParseError::MissingElement("notations end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(notations)
}

fn parse_tuplet_attrs(e: &BytesStart) -> Result<Tuplet> {
    let tuplet_type = match get_attr_required(e, "type")?.as_str() {
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
    let bracket = get_attr(e, "bracket")?.and_then(|s| parse_yes_no_opt(&s));
    let show_number = get_attr(e, "show-number")?.and_then(|s| match s.as_str() {
        "actual" => Some(ShowTuplet::Actual),
        "both" => Some(ShowTuplet::Both),
        "none" => Some(ShowTuplet::None),
        _ => Option::None,
    });
    let show_type = get_attr(e, "show-type")?.and_then(|s| match s.as_str() {
        "actual" => Some(ShowTuplet::Actual),
        "both" => Some(ShowTuplet::Both),
        "none" => Some(ShowTuplet::None),
        _ => Option::None,
    });
    let line_shape = get_attr(e, "line-shape")?.and_then(|s| match s.as_str() {
        "straight" => Some(LineShape::Straight),
        "curved" => Some(LineShape::Curved),
        _ => Option::None,
    });
    let placement = get_attr(e, "placement")?.and_then(|s| match s.as_str() {
        "above" => Some(AboveBelow::Above),
        "below" => Some(AboveBelow::Below),
        _ => Option::None,
    });

    Ok(Tuplet {
        tuplet_type,
        number,
        bracket,
        show_number,
        show_type,
        line_shape,
        placement,
        tuplet_actual: None,
        tuplet_normal: None,
    })
}

/// Parse a <tuplet> element that is self-closing (empty, attributes only).
fn parse_tuplet_empty(e: &BytesStart) -> Result<Tuplet> {
    parse_tuplet_attrs(e)
}

/// Parse a <tuplet> element with children (tuplet-actual, tuplet-normal).
fn parse_tuplet<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Tuplet> {
    let mut tuplet = parse_tuplet_attrs(start)?;
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"tuplet-actual" => {
                    tuplet.tuplet_actual = Some(parse_tuplet_portion(reader, b"tuplet-actual")?);
                }
                b"tuplet-normal" => {
                    tuplet.tuplet_normal = Some(parse_tuplet_portion(reader, b"tuplet-normal")?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(_) => {}
            Event::End(e) if e.name().as_ref() == b"tuplet" => break,
            Event::Eof => return Err(ParseError::MissingElement("tuplet end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(tuplet)
}

/// Parse a tuplet-portion (tuplet-actual or tuplet-normal).
fn parse_tuplet_portion<R: BufRead>(
    reader: &mut Reader<R>,
    end_tag: &[u8],
) -> Result<TupletPortion> {
    let mut buf = Vec::new();
    let mut tuplet_number: Option<TupletNumber> = None;
    let mut tuplet_type: Option<TupletType> = None;
    let mut tuplet_dots: Vec<TupletDot> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"tuplet-number" => {
                    let text = read_text(reader, b"tuplet-number")?;
                    let value = text
                        .parse::<u32>()
                        .map_err(|_| ParseError::ParseNumber("tuplet-number".to_string()))?;
                    tuplet_number = Some(TupletNumber { value });
                }
                b"tuplet-type" => {
                    let text = read_text(reader, b"tuplet-type")?;
                    let value = parse_note_type_value(&text)?;
                    tuplet_type = Some(TupletType { value });
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"tuplet-dot" => {
                tuplet_dots.push(TupletDot);
            }
            Event::Empty(_) => {}
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

    Ok(TupletPortion {
        tuplet_number,
        tuplet_type,
        tuplet_dots,
    })
}

/// Parse a NoteTypeValue from a string.
pub(crate) fn parse_note_type_value(s: &str) -> Result<NoteTypeValue> {
    match s {
        "1024th" => Ok(NoteTypeValue::N1024th),
        "512th" => Ok(NoteTypeValue::N512th),
        "256th" => Ok(NoteTypeValue::N256th),
        "128th" => Ok(NoteTypeValue::N128th),
        "64th" => Ok(NoteTypeValue::N64th),
        "32nd" => Ok(NoteTypeValue::N32nd),
        "16th" => Ok(NoteTypeValue::N16th),
        "eighth" => Ok(NoteTypeValue::Eighth),
        "quarter" => Ok(NoteTypeValue::Quarter),
        "half" => Ok(NoteTypeValue::Half),
        "whole" => Ok(NoteTypeValue::Whole),
        "breve" => Ok(NoteTypeValue::Breve),
        "long" => Ok(NoteTypeValue::Long),
        "maxima" => Ok(NoteTypeValue::Maxima),
        _ => Err(ParseError::InvalidContent(
            "note-type".to_string(),
            s.to_string(),
        )),
    }
}

fn parse_slur(e: &BytesStart) -> Result<Slur> {
    let slur_type = match get_attr_required(e, "type")?.as_str() {
        "start" => StartStopContinue::Start,
        "stop" => StartStopContinue::Stop,
        "continue" => StartStopContinue::Continue,
        s => {
            return Err(ParseError::InvalidAttribute(
                "type".to_string(),
                s.to_string(),
            ));
        }
    };

    let number = get_attr(e, "number")?.and_then(|s| s.parse().ok());
    let placement = get_attr(e, "placement")?.and_then(|s| match s.as_str() {
        "above" => Some(AboveBelow::Above),
        "below" => Some(AboveBelow::Below),
        _ => None,
    });
    let orientation = get_attr(e, "orientation")?.and_then(|s| match s.as_str() {
        "over" => Some(OverUnder::Over),
        "under" => Some(OverUnder::Under),
        _ => None,
    });

    Ok(Slur {
        slur_type,
        number,
        placement,
        orientation,
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        bezier_x: get_attr(e, "bezier-x")?.and_then(|s| s.parse().ok()),
        bezier_y: get_attr(e, "bezier-y")?.and_then(|s| s.parse().ok()),
        bezier_x2: get_attr(e, "bezier-x2")?.and_then(|s| s.parse().ok()),
        bezier_y2: get_attr(e, "bezier-y2")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        id: get_attr(e, "id")?,
    })
}

fn parse_tied(e: &BytesStart) -> Result<Tied> {
    let tied_type = match get_attr_required(e, "type")?.as_str() {
        "start" => TiedType::Start,
        "stop" => TiedType::Stop,
        "continue" => TiedType::Continue,
        "let-ring" => TiedType::LetRing,
        s => {
            return Err(ParseError::InvalidAttribute(
                "type".to_string(),
                s.to_string(),
            ));
        }
    };

    let number = get_attr(e, "number")?.and_then(|s| s.parse().ok());
    let orientation = get_attr(e, "orientation")?.and_then(|s| match s.as_str() {
        "over" => Some(OverUnder::Over),
        "under" => Some(OverUnder::Under),
        _ => None,
    });

    Ok(Tied {
        tied_type,
        number,
        orientation,
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        bezier_x: get_attr(e, "bezier-x")?.and_then(|s| s.parse().ok()),
        bezier_y: get_attr(e, "bezier-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        id: get_attr(e, "id")?,
    })
}

fn parse_articulations<R: BufRead>(reader: &mut Reader<R>) -> Result<Articulations> {
    let mut buf = Vec::new();
    let mut artics = Articulations::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                parse_articulation_element(&e, &mut artics);
                skip_element(reader, &e)?;
            }
            Event::Empty(e) => {
                parse_articulation_element(&e, &mut artics);
            }
            Event::End(e) if e.name().as_ref() == b"articulations" => break,
            Event::Eof => return Err(ParseError::MissingElement("articulations end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(artics)
}

fn parse_articulation_element(e: &BytesStart, artics: &mut Articulations) {
    let placement = get_attr(e, "placement")
        .ok()
        .flatten()
        .and_then(|s| match s.as_str() {
            "above" => Some(AboveBelow::Above),
            "below" => Some(AboveBelow::Below),
            _ => None,
        });
    let default_x = get_attr(e, "default-x")
        .ok()
        .flatten()
        .and_then(|s| s.parse().ok());
    let default_y = get_attr(e, "default-y")
        .ok()
        .flatten()
        .and_then(|s| s.parse().ok());
    let color = get_attr(e, "color").ok().flatten();

    let empty = EmptyPlacement {
        placement,
        default_x,
        default_y,
        color,
    };

    match e.name().as_ref() {
        b"accent" => artics.accent = Some(empty),
        b"strong-accent" => {
            let accent_type = get_attr(e, "type")
                .ok()
                .flatten()
                .and_then(|s| match s.as_str() {
                    "up" => Some(UpDown::Up),
                    "down" => Some(UpDown::Down),
                    _ => None,
                });
            artics.strong_accent = Some(StrongAccent {
                accent_type,
                placement,
                default_x,
                default_y,
            });
        }
        b"staccato" => artics.staccato = Some(empty),
        b"tenuto" => artics.tenuto = Some(empty),
        b"detached-legato" => artics.detached_legato = Some(empty),
        b"staccatissimo" => artics.staccatissimo = Some(empty),
        b"spiccato" => artics.spiccato = Some(empty),
        b"scoop" => artics.scoop = Some(empty),
        b"plop" => artics.plop = Some(empty),
        b"doit" => artics.doit = Some(empty),
        b"falloff" => artics.falloff = Some(empty),
        b"stress" => artics.stress = Some(empty),
        b"unstress" => artics.unstress = Some(empty),
        b"soft-accent" => artics.soft_accent = Some(empty),
        b"breath-mark" => {
            artics.breath_mark = Some(BreathMark {
                value: None,
                placement,
            });
        }
        b"caesura" => {
            artics.caesura = Some(Caesura {
                value: None,
                placement,
            });
        }
        _ => {}
    }
}

// ============================================================================
// Ornaments Parsing
// ============================================================================

fn parse_ornaments<R: BufRead>(reader: &mut Reader<R>) -> Result<Ornaments> {
    let mut buf = Vec::new();
    let mut ornaments = Ornaments::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"trill-mark" => {
                    ornaments.trill_mark = Some(parse_empty_trill_sound(&e));
                    skip_to_end(reader, b"trill-mark")?;
                }
                b"turn" => {
                    ornaments.turn = Some(parse_horizontal_turn(&e));
                    skip_to_end(reader, b"turn")?;
                }
                b"delayed-turn" => {
                    ornaments.delayed_turn = Some(parse_horizontal_turn(&e));
                    skip_to_end(reader, b"delayed-turn")?;
                }
                b"inverted-turn" => {
                    ornaments.inverted_turn = Some(parse_horizontal_turn(&e));
                    skip_to_end(reader, b"inverted-turn")?;
                }
                b"delayed-inverted-turn" => {
                    ornaments.delayed_inverted_turn = Some(parse_horizontal_turn(&e));
                    skip_to_end(reader, b"delayed-inverted-turn")?;
                }
                b"vertical-turn" => {
                    ornaments.vertical_turn = Some(parse_empty_trill_sound(&e));
                    skip_to_end(reader, b"vertical-turn")?;
                }
                b"inverted-vertical-turn" => {
                    ornaments.inverted_vertical_turn = Some(parse_empty_trill_sound(&e));
                    skip_to_end(reader, b"inverted-vertical-turn")?;
                }
                b"shake" => {
                    ornaments.shake = Some(parse_empty_trill_sound(&e));
                    skip_to_end(reader, b"shake")?;
                }
                b"wavy-line" => {
                    ornaments.wavy_line = Some(parse_wavy_line(&e)?);
                    skip_to_end(reader, b"wavy-line")?;
                }
                b"mordent" => {
                    ornaments.mordent = Some(parse_mordent(&e));
                    skip_to_end(reader, b"mordent")?;
                }
                b"inverted-mordent" => {
                    ornaments.inverted_mordent = Some(parse_mordent(&e));
                    skip_to_end(reader, b"inverted-mordent")?;
                }
                b"schleifer" => {
                    ornaments.schleifer = Some(parse_empty_placement_from(&e));
                    skip_to_end(reader, b"schleifer")?;
                }
                b"tremolo" => {
                    ornaments.tremolo = Some(parse_tremolo(reader, &e)?);
                }
                b"haydn" => {
                    ornaments.haydn = Some(parse_empty_trill_sound(&e));
                    skip_to_end(reader, b"haydn")?;
                }
                b"other-ornament" => {
                    ornaments.other_ornament = Some(parse_other_ornament(reader, &e)?);
                }
                b"accidental-mark" => {
                    ornaments
                        .accidental_marks
                        .push(parse_accidental_mark(reader, &e)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"trill-mark" => ornaments.trill_mark = Some(parse_empty_trill_sound(&e)),
                b"turn" => ornaments.turn = Some(parse_horizontal_turn(&e)),
                b"delayed-turn" => ornaments.delayed_turn = Some(parse_horizontal_turn(&e)),
                b"inverted-turn" => ornaments.inverted_turn = Some(parse_horizontal_turn(&e)),
                b"delayed-inverted-turn" => {
                    ornaments.delayed_inverted_turn = Some(parse_horizontal_turn(&e));
                }
                b"vertical-turn" => {
                    ornaments.vertical_turn = Some(parse_empty_trill_sound(&e));
                }
                b"inverted-vertical-turn" => {
                    ornaments.inverted_vertical_turn = Some(parse_empty_trill_sound(&e));
                }
                b"shake" => ornaments.shake = Some(parse_empty_trill_sound(&e)),
                b"wavy-line" => ornaments.wavy_line = Some(parse_wavy_line(&e)?),
                b"mordent" => ornaments.mordent = Some(parse_mordent(&e)),
                b"inverted-mordent" => ornaments.inverted_mordent = Some(parse_mordent(&e)),
                b"schleifer" => ornaments.schleifer = Some(parse_empty_placement_from(&e)),
                b"tremolo" => {
                    ornaments.tremolo = Some(parse_tremolo_empty(&e)?);
                }
                b"haydn" => ornaments.haydn = Some(parse_empty_trill_sound(&e)),
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"ornaments" => break,
            Event::Eof => return Err(ParseError::MissingElement("ornaments end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(ornaments)
}

/// Parse trill-sound attribute group from an element.
fn parse_trill_sound(e: &BytesStart) -> TrillSound {
    TrillSound {
        start_note: get_attr(e, "start-note")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        trill_step: get_attr(e, "trill-step")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        two_note_turn: get_attr(e, "two-note-turn")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        accelerate: get_attr(e, "accelerate")
            .ok()
            .flatten()
            .and_then(|s| parse_yes_no_opt(&s)),
        beats: get_attr(e, "beats")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        second_beat: get_attr(e, "second-beat")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        last_beat: get_attr(e, "last-beat")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
    }
}

/// Parse placement + print-style + trill-sound from an element.
fn parse_empty_trill_sound(e: &BytesStart) -> EmptyTrillSound {
    EmptyTrillSound {
        placement: get_attr(e, "placement")
            .ok()
            .flatten()
            .and_then(|s| match s.as_str() {
                "above" => Some(AboveBelow::Above),
                "below" => Some(AboveBelow::Below),
                _ => None,
            }),
        default_x: get_attr(e, "default-x")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        color: get_attr(e, "color").ok().flatten(),
        trill_sound: parse_trill_sound(e),
    }
}

/// Parse horizontal-turn type (extends empty-trill-sound with slash).
fn parse_horizontal_turn(e: &BytesStart) -> HorizontalTurn {
    HorizontalTurn {
        placement: get_attr(e, "placement")
            .ok()
            .flatten()
            .and_then(|s| match s.as_str() {
                "above" => Some(AboveBelow::Above),
                "below" => Some(AboveBelow::Below),
                _ => None,
            }),
        default_x: get_attr(e, "default-x")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        color: get_attr(e, "color").ok().flatten(),
        slash: get_attr(e, "slash")
            .ok()
            .flatten()
            .and_then(|s| parse_yes_no_opt(&s)),
        trill_sound: parse_trill_sound(e),
    }
}

/// Parse mordent type (extends empty-trill-sound with long, approach, departure).
fn parse_mordent(e: &BytesStart) -> Mordent {
    Mordent {
        placement: get_attr(e, "placement")
            .ok()
            .flatten()
            .and_then(|s| match s.as_str() {
                "above" => Some(AboveBelow::Above),
                "below" => Some(AboveBelow::Below),
                _ => None,
            }),
        default_x: get_attr(e, "default-x")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")
            .ok()
            .flatten()
            .and_then(|s| s.parse().ok()),
        color: get_attr(e, "color").ok().flatten(),
        long: get_attr(e, "long")
            .ok()
            .flatten()
            .and_then(|s| parse_yes_no_opt(&s)),
        approach: get_attr(e, "approach")
            .ok()
            .flatten()
            .and_then(|s| match s.as_str() {
                "above" => Some(AboveBelow::Above),
                "below" => Some(AboveBelow::Below),
                _ => None,
            }),
        departure: get_attr(e, "departure")
            .ok()
            .flatten()
            .and_then(|s| match s.as_str() {
                "above" => Some(AboveBelow::Above),
                "below" => Some(AboveBelow::Below),
                _ => None,
            }),
        trill_sound: parse_trill_sound(e),
    }
}

/// Parse wavy-line element attributes.
pub(crate) fn parse_wavy_line(e: &BytesStart) -> Result<WavyLine> {
    let wavy_line_type = match get_attr_required(e, "type")?.as_str() {
        "start" => StartStopContinue::Start,
        "stop" => StartStopContinue::Stop,
        "continue" => StartStopContinue::Continue,
        s => {
            return Err(ParseError::InvalidAttribute(
                "type".to_string(),
                s.to_string(),
            ));
        }
    };

    Ok(WavyLine {
        wavy_line_type,
        number: get_attr(e, "number")?.and_then(|s| s.parse().ok()),
        placement: get_attr(e, "placement")?.and_then(|s| match s.as_str() {
            "above" => Some(AboveBelow::Above),
            "below" => Some(AboveBelow::Below),
            _ => None,
        }),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        smufl: get_attr(e, "smufl")?,
        trill_sound: parse_trill_sound(e),
    })
}

/// Parse tremolo element (has text content = marks count, type attribute).
fn parse_tremolo<R: BufRead>(reader: &mut Reader<R>, e: &BytesStart) -> Result<Tremolo> {
    let tremolo_type = get_attr(e, "type")?
        .and_then(|s| s.parse().ok())
        .unwrap_or(TremoloType::Single);

    let text = read_text(reader, b"tremolo")?;
    let value = if text.is_empty() {
        None
    } else {
        text.parse().ok()
    };

    Ok(Tremolo {
        tremolo_type,
        value,
        placement: get_attr(e, "placement")?.and_then(|s| match s.as_str() {
            "above" => Some(AboveBelow::Above),
            "below" => Some(AboveBelow::Below),
            _ => None,
        }),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        smufl: get_attr(e, "smufl")?,
    })
}

/// Parse a self-closing tremolo element (empty, no text content).
fn parse_tremolo_empty(e: &BytesStart) -> Result<Tremolo> {
    let tremolo_type = get_attr(e, "type")?
        .and_then(|s| s.parse().ok())
        .unwrap_or(TremoloType::Single);

    Ok(Tremolo {
        tremolo_type,
        value: None,
        placement: get_attr(e, "placement")?.and_then(|s| match s.as_str() {
            "above" => Some(AboveBelow::Above),
            "below" => Some(AboveBelow::Below),
            _ => None,
        }),
        default_x: get_attr(e, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(e, "default-y")?.and_then(|s| s.parse().ok()),
        color: get_attr(e, "color")?,
        smufl: get_attr(e, "smufl")?,
    })
}

/// Parse other-ornament element (has text content, placement attribute).
fn parse_other_ornament<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<OtherOrnament> {
    let placement = get_attr(e, "placement")?.and_then(|s| match s.as_str() {
        "above" => Some(AboveBelow::Above),
        "below" => Some(AboveBelow::Below),
        _ => None,
    });
    let value = read_text(reader, b"other-ornament")?;
    Ok(OtherOrnament { value, placement })
}

/// Parse accidental-mark element within ornaments.
fn parse_accidental_mark<R: BufRead>(
    reader: &mut Reader<R>,
    e: &BytesStart,
) -> Result<AccidentalMark> {
    let placement = get_attr(e, "placement")?.and_then(|s| match s.as_str() {
        "above" => Some(AboveBelow::Above),
        "below" => Some(AboveBelow::Below),
        _ => None,
    });
    let value = read_text(reader, b"accidental-mark")?;
    Ok(AccidentalMark { value, placement })
}

/// Parse an empty-placement from element attributes.
fn parse_empty_placement_from(e: &BytesStart) -> EmptyPlacement {
    EmptyPlacement {
        placement: get_attr(e, "placement")
            .ok()
            .flatten()
            .and_then(|s| match s.as_str() {
                "above" => Some(AboveBelow::Above),
                "below" => Some(AboveBelow::Below),
                _ => None,
            }),
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

// ============================================================================
// Lyric Parsing
// ============================================================================

fn parse_lyric_attrs(start: &BytesStart) -> Result<Lyric> {
    Ok(Lyric {
        number: get_attr(start, "number")?,
        name: get_attr(start, "name")?,
        justify: get_attr(start, "justify")?.and_then(|s| match s.as_str() {
            "left" => Some(LeftCenterRight::Left),
            "center" => Some(LeftCenterRight::Center),
            "right" => Some(LeftCenterRight::Right),
            _ => None,
        }),
        default_x: get_attr(start, "default-x")?.and_then(|s| s.parse().ok()),
        default_y: get_attr(start, "default-y")?.and_then(|s| s.parse().ok()),
        relative_x: get_attr(start, "relative-x")?.and_then(|s| s.parse().ok()),
        relative_y: get_attr(start, "relative-y")?.and_then(|s| s.parse().ok()),
        placement: get_attr(start, "placement")?.and_then(|s| match s.as_str() {
            "above" => Some(AboveBelow::Above),
            "below" => Some(AboveBelow::Below),
            _ => None,
        }),
        color: get_attr(start, "color")?,
        print_object: get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s)),
        time_only: get_attr(start, "time-only")?,
        id: get_attr(start, "id")?,
        content: LyricContent::Laughing, // placeholder, will be replaced
        end_line: false,
        end_paragraph: false,
    })
}

fn parse_syllabic_value(s: &str) -> Option<Syllabic> {
    match s {
        "single" => Some(Syllabic::Single),
        "begin" => Some(Syllabic::Begin),
        "middle" => Some(Syllabic::Middle),
        "end" => Some(Syllabic::End),
        _ => None,
    }
}

/// Parse a `<lyric>` element with children.
fn parse_lyric<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Lyric> {
    let mut lyric = parse_lyric_attrs(start)?;
    let mut buf = Vec::new();

    // State for parsing the choice content
    let mut syllable_groups: Vec<SyllableGroup> = Vec::new();
    let mut current_syllabic: Option<Syllabic> = None;
    let mut current_elision: Option<Elision> = None;
    let mut extend: Option<Extend> = None;
    let mut is_laughing = false;
    let mut is_humming = false;
    let mut has_text = false;
    let mut end_line = false;
    let mut end_paragraph = false;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"syllabic" => {
                    let s = read_text(reader, b"syllabic")?;
                    current_syllabic = parse_syllabic_value(&s);
                }
                b"text" => {
                    let font_family = get_attr(&e, "font-family")?;
                    let font_size = get_attr(&e, "font-size")?
                        .and_then(|s| s.parse::<f64>().ok().map(FontSize::Points));
                    let font_style = get_attr(&e, "font-style")?.and_then(|s| match s.as_str() {
                        "normal" => Some(FontStyle::Normal),
                        "italic" => Some(FontStyle::Italic),
                        _ => None,
                    });
                    let font_weight = get_attr(&e, "font-weight")?.and_then(|s| match s.as_str() {
                        "normal" => Some(FontWeight::Normal),
                        "bold" => Some(FontWeight::Bold),
                        _ => None,
                    });
                    let color = get_attr(&e, "color")?;
                    let value = read_text(reader, b"text")?;
                    syllable_groups.push(SyllableGroup {
                        elision: current_elision.take(),
                        syllabic: current_syllabic.take(),
                        text: LyricText {
                            value,
                            font_family,
                            font_size,
                            font_style,
                            font_weight,
                            color,
                        },
                    });
                    has_text = true;
                }
                b"elision" => {
                    let font_family = get_attr(&e, "font-family")?;
                    let font_size = get_attr(&e, "font-size")?
                        .and_then(|s| s.parse::<f64>().ok().map(FontSize::Points));
                    let font_style = get_attr(&e, "font-style")?.and_then(|s| match s.as_str() {
                        "normal" => Some(FontStyle::Normal),
                        "italic" => Some(FontStyle::Italic),
                        _ => None,
                    });
                    let font_weight = get_attr(&e, "font-weight")?.and_then(|s| match s.as_str() {
                        "normal" => Some(FontWeight::Normal),
                        "bold" => Some(FontWeight::Bold),
                        _ => None,
                    });
                    let color = get_attr(&e, "color")?;
                    let value = read_text(reader, b"elision")?;
                    current_elision = Some(Elision {
                        value,
                        font_family,
                        font_size,
                        font_style,
                        font_weight,
                        color,
                    });
                }
                b"extend" => {
                    let ext_type = get_attr(&e, "type")?.and_then(|s| match s.as_str() {
                        "start" => Some(StartStopContinue::Start),
                        "stop" => Some(StartStopContinue::Stop),
                        "continue" => Some(StartStopContinue::Continue),
                        _ => None,
                    });
                    extend = Some(Extend {
                        extend_type: ext_type,
                    });
                    skip_to_end(reader, b"extend")?;
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"laughing" => is_laughing = true,
                b"humming" => is_humming = true,
                b"end-line" => end_line = true,
                b"end-paragraph" => end_paragraph = true,
                b"extend" => {
                    let ext_type = get_attr(&e, "type")?.and_then(|s| match s.as_str() {
                        "start" => Some(StartStopContinue::Start),
                        "stop" => Some(StartStopContinue::Stop),
                        "continue" => Some(StartStopContinue::Continue),
                        _ => None,
                    });
                    extend = Some(Extend {
                        extend_type: ext_type,
                    });
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"lyric" => break,
            Event::Eof => return Err(ParseError::MissingElement("lyric end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    // Determine content type
    lyric.content = if is_laughing {
        LyricContent::Laughing
    } else if is_humming {
        LyricContent::Humming
    } else if has_text {
        LyricContent::Text {
            syllable_groups,
            extend,
        }
    } else if let Some(ext) = extend {
        LyricContent::ExtendOnly(ext)
    } else {
        // Default to empty text
        LyricContent::Text {
            syllable_groups: Vec::new(),
            extend: None,
        }
    };
    lyric.end_line = end_line;
    lyric.end_paragraph = end_paragraph;

    Ok(lyric)
}

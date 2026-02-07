//! Note, backup, and forward parsing.

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::{ParseError, Result, get_attr, get_attr_required, read_text, skip_element};
use crate::model::data::*;
use crate::model::elements::Empty;
use crate::model::notations::*;
use crate::model::note::*;

pub fn parse_note<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Note> {
    let mut buf = Vec::new();

    // Parse attributes
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let dynamics = get_attr(start, "dynamics")?.and_then(|s| s.parse().ok());
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
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
    let mut note_type: Option<NoteType> = None;
    let mut dots: Vec<Dot> = Vec::new();
    let mut accidental: Option<Accidental> = None;
    let mut time_modification: Option<TimeModification> = None;
    let mut stem: Option<Stem> = None;
    let mut staff: Option<u32> = None;
    let mut beams: Vec<Beam> = Vec::new();
    let mut notations: Option<Notations> = None;

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
                b"type" => note_type = Some(parse_note_type(reader, &e)?),
                b"dot" => {
                    dots.push(parse_dot(&e)?);
                    skip_to_end(reader, b"dot")?;
                }
                b"accidental" => accidental = Some(parse_accidental(reader, &e)?),
                b"time-modification" => time_modification = Some(parse_time_modification(reader)?),
                b"stem" => stem = Some(parse_stem(reader, &e)?),
                b"staff" => {
                    staff = Some(
                        read_text(reader, b"staff")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staff".to_string()))?,
                    )
                }
                b"beam" => beams.push(parse_beam(reader, &e)?),
                b"notations" => notations = Some(parse_notations(reader)?),
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"grace" => grace = Some(parse_grace(&e)?),
                b"cue" => cue = Some(Empty),
                b"chord" => chord = Some(Empty),
                b"rest" => rest = Some(parse_rest_empty(&e)?),
                b"tie" => ties.push(parse_tie(&e)?),
                b"dot" => dots.push(parse_dot(&e)?),
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
        footnote: None,
        level: None,
        voice,
        instruments: Vec::new(),
        note_type,
        dots,
        accidental,
        time_modification,
        stem,
        notehead: None,
        staff,
        beams,
        notations,
        default_x,
        default_y,
        relative_x: None,
        relative_y: None,
        print_object,
        print_leger: None,
        print_spacing: None,
        dynamics,
        end_dynamics: None,
        attack: None,
        release: None,
        pizzicato: None,
        color: None,
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

fn parse_accidental<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Accidental> {
    let cautionary = get_attr(start, "cautionary")?.and_then(|s| parse_yes_no_opt(&s));
    let editorial = get_attr(start, "editorial")?.and_then(|s| parse_yes_no_opt(&s));
    let parentheses = get_attr(start, "parentheses")?.and_then(|s| parse_yes_no_opt(&s));
    let bracket = get_attr(start, "bracket")?.and_then(|s| parse_yes_no_opt(&s));

    let value_str = read_text(reader, b"accidental")?;
    let value = match value_str.as_str() {
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
        _ => AccidentalValue::Other,
    };

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
                    normal_type = Some(match s.as_str() {
                        "eighth" => NoteTypeValue::Eighth,
                        "quarter" => NoteTypeValue::Quarter,
                        "half" => NoteTypeValue::Half,
                        "whole" => NoteTypeValue::Whole,
                        "16th" => NoteTypeValue::N16th,
                        "32nd" => NoteTypeValue::N32nd,
                        _ => NoteTypeValue::Quarter,
                    });
                }
                _ => skip_element(reader, &e)?,
            },
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
        normal_dots: Vec::new(),
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
        repeater: None,
        fan: None,
        color: None,
        id: None,
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
                b"slur" => {
                    notations.slurs.push(parse_slur(&e)?);
                    skip_to_end(reader, b"slur")?;
                }
                b"tied" => {
                    notations.tied.push(parse_tied(&e)?);
                    skip_to_end(reader, b"tied")?;
                }
                b"articulations" => {
                    notations.articulations = Some(parse_articulations(reader)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"slur" => notations.slurs.push(parse_slur(&e)?),
                b"tied" => notations.tied.push(parse_tied(&e)?),
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

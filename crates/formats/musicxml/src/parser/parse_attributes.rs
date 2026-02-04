//! Attributes parsing.

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::{get_attr, read_text, skip_element, ParseError, Result};
use crate::model::attributes::*;
use crate::model::data::*;

pub fn parse_attributes<R: BufRead>(reader: &mut Reader<R>) -> Result<Attributes> {
    let mut buf = Vec::new();
    let mut attrs = Attributes::default();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"divisions" => {
                    attrs.divisions = Some(
                        read_text(reader, b"divisions")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("divisions".to_string()))?,
                    )
                }
                b"key" => attrs.keys.push(parse_key(reader, &e)?),
                b"time" => attrs.times.push(parse_time(reader, &e)?),
                b"staves" => {
                    attrs.staves = Some(
                        read_text(reader, b"staves")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staves".to_string()))?,
                    )
                }
                b"instruments" => {
                    attrs.instruments = Some(
                        read_text(reader, b"instruments")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("instruments".to_string()))?,
                    )
                }
                b"clef" => attrs.clefs.push(parse_clef(reader, &e)?),
                b"transpose" => attrs.transposes.push(parse_transpose(reader, &e)?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"attributes" => break,
            Event::Eof => return Err(ParseError::MissingElement("attributes end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(attrs)
}

fn parse_key<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Key> {
    let mut buf = Vec::new();
    let number = get_attr(start, "number")?.and_then(|s| s.parse().ok());
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
    let id = get_attr(start, "id")?;

    let mut fifths: Option<i8> = None;
    let mut mode: Option<Mode> = None;
    let mut cancel: Option<Cancel> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"fifths" => {
                    fifths = Some(
                        read_text(reader, b"fifths")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("fifths".to_string()))?,
                    )
                }
                b"mode" => {
                    let s = read_text(reader, b"mode")?;
                    mode = Some(match s.as_str() {
                        "major" => Mode::Major,
                        "minor" => Mode::Minor,
                        "dorian" => Mode::Dorian,
                        "phrygian" => Mode::Phrygian,
                        "lydian" => Mode::Lydian,
                        "mixolydian" => Mode::Mixolydian,
                        "aeolian" => Mode::Aeolian,
                        "ionian" => Mode::Ionian,
                        "locrian" => Mode::Locrian,
                        "none" => Mode::None,
                        other => Mode::Other(other.to_string()),
                    });
                }
                b"cancel" => cancel = Some(parse_cancel(reader, &e)?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"key" => break,
            Event::Eof => return Err(ParseError::MissingElement("key end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    let content = KeyContent::Traditional(TraditionalKey {
        cancel,
        fifths: fifths.unwrap_or(0),
        mode,
    });

    Ok(Key {
        number,
        print_object,
        id,
        content,
        key_octaves: Vec::new(),
    })
}

fn parse_cancel<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Cancel> {
    let location = get_attr(start, "location")?.and_then(|s| match s.as_str() {
        "left" => Some(CancelLocation::Left),
        "right" => Some(CancelLocation::Right),
        "before-barline" => Some(CancelLocation::BeforeBarline),
        _ => None,
    });
    let fifths = read_text(reader, b"cancel")?
        .parse()
        .map_err(|_| ParseError::ParseNumber("cancel".to_string()))?;
    Ok(Cancel { fifths, location })
}

fn parse_time<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Time> {
    let mut buf = Vec::new();
    let number = get_attr(start, "number")?.and_then(|s| s.parse().ok());
    let symbol = get_attr(start, "symbol")?.and_then(|s| match s.as_str() {
        "common" => Some(TimeSymbol::Common),
        "cut" => Some(TimeSymbol::Cut),
        "single-number" => Some(TimeSymbol::SingleNumber),
        "note" => Some(TimeSymbol::Note),
        "dotted-note" => Some(TimeSymbol::DottedNote),
        "normal" => Some(TimeSymbol::Normal),
        _ => None,
    });
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));

    let mut beats: Option<String> = None;
    let mut beat_type: Option<String> = None;
    let mut senza_misura: Option<String> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"beats" => beats = Some(read_text(reader, b"beats")?),
                b"beat-type" => beat_type = Some(read_text(reader, b"beat-type")?),
                b"senza-misura" => senza_misura = Some(read_text(reader, b"senza-misura")?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"time" => break,
            Event::Eof => return Err(ParseError::MissingElement("time end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    let content = if senza_misura.is_some() {
        TimeContent::SenzaMisura(SenzaMisura {
            symbol: senza_misura,
        })
    } else {
        TimeContent::Standard(StandardTime {
            signatures: vec![TimeSignature {
                beats: beats.unwrap_or_else(|| "4".to_string()),
                beat_type: beat_type.unwrap_or_else(|| "4".to_string()),
            }],
            interchangeable: None,
        })
    };

    Ok(Time {
        number,
        symbol,
        separator: None,
        print_object,
        id: None,
        content,
    })
}

fn parse_clef<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Clef> {
    let mut buf = Vec::new();
    let number = get_attr(start, "number")?.and_then(|s| s.parse().ok());
    let additional = get_attr(start, "additional")?.and_then(|s| parse_yes_no_opt(&s));
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));

    let mut sign = ClefSign::G;
    let mut line: Option<u32> = None;
    let mut clef_octave_change: Option<i32> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                match e.name().as_ref() {
                    b"sign" => {
                        let s = read_text(reader, b"sign")?;
                        sign = match s.as_str() {
                            "G" => ClefSign::G,
                            "F" => ClefSign::F,
                            "C" => ClefSign::C,
                            "percussion" => ClefSign::Percussion,
                            "TAB" => ClefSign::Tab,
                            "jianpu" => ClefSign::Jianpu,
                            "none" => ClefSign::None,
                            _ => ClefSign::G,
                        };
                    }
                    b"line" => {
                        line = Some(
                            read_text(reader, b"line")?
                                .parse()
                                .map_err(|_| ParseError::ParseNumber("line".to_string()))?,
                        )
                    }
                    b"clef-octave-change" => {
                        clef_octave_change =
                            Some(read_text(reader, b"clef-octave-change")?.parse().map_err(
                                |_| ParseError::ParseNumber("clef-octave-change".to_string()),
                            )?)
                    }
                    _ => skip_element(reader, &e)?,
                }
            }
            Event::End(e) if e.name().as_ref() == b"clef" => break,
            Event::Eof => return Err(ParseError::MissingElement("clef end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Clef {
        number,
        additional,
        size: None,
        after_barline: None,
        print_object,
        id: None,
        sign,
        line,
        clef_octave_change,
    })
}

fn parse_transpose<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Transpose> {
    let mut buf = Vec::new();
    let number = get_attr(start, "number")?.and_then(|s| s.parse().ok());

    let mut diatonic: Option<i32> = None;
    let mut chromatic: f64 = 0.0;
    let mut octave_change: Option<i32> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"diatonic" => {
                    diatonic = Some(
                        read_text(reader, b"diatonic")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("diatonic".to_string()))?,
                    )
                }
                b"chromatic" => {
                    chromatic = read_text(reader, b"chromatic")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("chromatic".to_string()))?
                }
                b"octave-change" => {
                    octave_change = Some(
                        read_text(reader, b"octave-change")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("octave-change".to_string()))?,
                    )
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"transpose" => break,
            Event::Eof => return Err(ParseError::MissingElement("transpose end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Transpose {
        number,
        id: None,
        diatonic,
        chromatic,
        octave_change,
        double: None,
    })
}

fn parse_yes_no_opt(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

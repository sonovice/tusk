//! Attributes parsing.

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::{ParseError, Result, get_attr, read_text, skip_element};
use crate::model::attributes::*;
use crate::model::data::*;
use crate::model::note::NoteTypeValue;

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
                b"part-symbol" => attrs.part_symbol = Some(parse_part_symbol(reader, &e)?),
                b"staff-details" => attrs.staff_details.push(parse_staff_details(reader, &e)?),
                b"transpose" => attrs.transposes.push(parse_transpose(reader, &e)?),
                b"measure-style" => attrs.measure_styles.push(parse_measure_style(reader, &e)?),
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

fn parse_staff_details<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<StaffDetails> {
    let mut buf = Vec::new();
    let number = get_attr(start, "number")?.and_then(|s| s.parse().ok());
    let show_frets = get_attr(start, "show-frets")?.and_then(|s| match s.as_str() {
        "numbers" => Some(ShowFrets::Numbers),
        "letters" => Some(ShowFrets::Letters),
        _ => None,
    });
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
    let print_spacing = get_attr(start, "print-spacing")?.and_then(|s| parse_yes_no_opt(&s));

    let mut staff_type: Option<StaffType> = None;
    let mut staff_lines: Option<u32> = None;
    let mut line_details: Vec<LineDetail> = Vec::new();
    let mut staff_tunings: Vec<StaffTuning> = Vec::new();
    let mut capo: Option<u32> = None;
    let mut staff_size: Option<StaffSize> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"staff-type" => {
                    let s = read_text(reader, b"staff-type")?;
                    staff_type = Some(match s.as_str() {
                        "ossia" => StaffType::Ossia,
                        "editorial" => StaffType::Editorial,
                        "cue" => StaffType::Cue,
                        "regular" => StaffType::Regular,
                        "alternate" => StaffType::Alternate,
                        _ => StaffType::Regular,
                    });
                }
                b"staff-lines" => {
                    staff_lines = Some(
                        read_text(reader, b"staff-lines")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staff-lines".to_string()))?,
                    );
                }
                b"line-detail" => line_details.push(parse_line_detail(&e)?),
                b"staff-tuning" => staff_tunings.push(parse_staff_tuning(reader, &e)?),
                b"capo" => {
                    capo = Some(
                        read_text(reader, b"capo")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("capo".to_string()))?,
                    );
                }
                b"staff-size" => {
                    let scaling = get_attr(&e, "scaling")?.and_then(|s| s.parse().ok());
                    let value: f64 = read_text(reader, b"staff-size")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("staff-size".to_string()))?;
                    staff_size = Some(StaffSize { value, scaling });
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"line-detail" => {
                line_details.push(parse_line_detail(&e)?);
            }
            Event::End(e) if e.name().as_ref() == b"staff-details" => break,
            Event::Eof => return Err(ParseError::MissingElement("staff-details end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(StaffDetails {
        number,
        show_frets,
        print_object,
        print_spacing,
        staff_type,
        staff_lines,
        line_details,
        staff_tunings,
        capo,
        staff_size,
    })
}

fn parse_line_detail(start: &BytesStart) -> Result<LineDetail> {
    let line: u32 = get_attr(start, "line")?
        .ok_or_else(|| ParseError::MissingAttribute("line-detail @line".to_string()))?
        .parse()
        .map_err(|_| ParseError::ParseNumber("line-detail line".to_string()))?;
    let width = get_attr(start, "width")?.and_then(|s| s.parse().ok());
    let color = get_attr(start, "color")?;
    let line_type = get_attr(start, "line-type")?.and_then(|s| match s.as_str() {
        "solid" => Some(LineType::Solid),
        "dashed" => Some(LineType::Dashed),
        "dotted" => Some(LineType::Dotted),
        "wavy" => Some(LineType::Wavy),
        _ => None,
    });
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));

    Ok(LineDetail {
        line,
        width,
        color,
        line_type,
        print_object,
    })
}

fn parse_staff_tuning<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<StaffTuning> {
    let mut buf = Vec::new();
    let line: u32 = get_attr(start, "line")?
        .ok_or_else(|| ParseError::MissingAttribute("staff-tuning @line".to_string()))?
        .parse()
        .map_err(|_| ParseError::ParseNumber("staff-tuning line".to_string()))?;

    let mut tuning_step = Step::C;
    let mut tuning_alter: Option<f64> = None;
    let mut tuning_octave: u8 = 4;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"tuning-step" => {
                    let s = read_text(reader, b"tuning-step")?;
                    tuning_step = super::parse_harmony::parse_step_value(&s)?;
                }
                b"tuning-alter" => {
                    tuning_alter = Some(
                        read_text(reader, b"tuning-alter")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("tuning-alter".to_string()))?,
                    );
                }
                b"tuning-octave" => {
                    tuning_octave = read_text(reader, b"tuning-octave")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("tuning-octave".to_string()))?;
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"staff-tuning" => break,
            Event::Eof => return Err(ParseError::MissingElement("staff-tuning end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(StaffTuning {
        line,
        tuning_step,
        tuning_alter,
        tuning_octave,
    })
}

fn parse_part_symbol<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<PartSymbol> {
    let top_staff = get_attr(start, "top-staff")?.and_then(|s| s.parse().ok());
    let bottom_staff = get_attr(start, "bottom-staff")?.and_then(|s| s.parse().ok());
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let color = get_attr(start, "color")?;

    let text = read_text(reader, b"part-symbol")?;
    let value = match text.as_str() {
        "none" => PartSymbolValue::None,
        "brace" => PartSymbolValue::Brace,
        "line" => PartSymbolValue::Line,
        "bracket" => PartSymbolValue::Bracket,
        "square" => PartSymbolValue::Square,
        _ => PartSymbolValue::Brace, // default per spec
    };

    Ok(PartSymbol {
        value,
        top_staff,
        bottom_staff,
        default_x,
        color,
    })
}

fn parse_measure_style<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<MeasureStyle> {
    let mut buf = Vec::new();
    let number = get_attr(start, "number")?.and_then(|s| s.parse().ok());
    let mut content: Option<MeasureStyleContent> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"multiple-rest" => {
                    let use_symbols =
                        get_attr(&e, "use-symbols")?.and_then(|s| parse_yes_no_opt(&s));
                    let value: u32 = read_text(reader, b"multiple-rest")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("multiple-rest".to_string()))?;
                    content = Some(MeasureStyleContent::MultipleRest(MultipleRest {
                        value,
                        use_symbols,
                    }));
                }
                b"measure-repeat" => {
                    content = Some(MeasureStyleContent::MeasureRepeat(parse_measure_repeat(
                        reader, &e,
                    )?));
                }
                b"beat-repeat" => {
                    content = Some(MeasureStyleContent::BeatRepeat(parse_beat_repeat(
                        reader, &e,
                    )?));
                }
                b"slash" => {
                    content = Some(MeasureStyleContent::Slash(parse_slash(reader, &e)?));
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"measure-repeat" => {
                    let repeat_type = match get_attr(&e, "type")?.as_deref() {
                        Some("stop") => StartStop::Stop,
                        _ => StartStop::Start,
                    };
                    let slashes = get_attr(&e, "slashes")?.and_then(|s| s.parse().ok());
                    content = Some(MeasureStyleContent::MeasureRepeat(MeasureRepeat {
                        value: None,
                        repeat_type,
                        slashes,
                    }));
                }
                b"beat-repeat" => {
                    let repeat_type = match get_attr(&e, "type")?.as_deref() {
                        Some("stop") => StartStopContinue::Stop,
                        Some("continue") => StartStopContinue::Continue,
                        _ => StartStopContinue::Start,
                    };
                    let slashes = get_attr(&e, "slashes")?.and_then(|s| s.parse().ok());
                    let use_dots = get_attr(&e, "use-dots")?.and_then(|s| parse_yes_no_opt(&s));
                    content = Some(MeasureStyleContent::BeatRepeat(BeatRepeat {
                        repeat_type,
                        slashes,
                        use_dots,
                        slash_type: None,
                        slash_dots: Vec::new(),
                        except_voices: Vec::new(),
                    }));
                }
                b"slash" => {
                    let slash_type = match get_attr(&e, "type")?.as_deref() {
                        Some("stop") => StartStop::Stop,
                        _ => StartStop::Start,
                    };
                    let use_stems = get_attr(&e, "use-stems")?.and_then(|s| parse_yes_no_opt(&s));
                    let use_dots = get_attr(&e, "use-dots")?.and_then(|s| parse_yes_no_opt(&s));
                    content = Some(MeasureStyleContent::Slash(Slash {
                        slash_type,
                        use_stems,
                        use_dots,
                        slash_type_element: None,
                        slash_dots: Vec::new(),
                        except_voices: Vec::new(),
                    }));
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"measure-style" => break,
            Event::Eof => return Err(ParseError::MissingElement("measure-style end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    let content =
        content.ok_or_else(|| ParseError::MissingElement("measure-style content".to_string()))?;

    Ok(MeasureStyle { number, content })
}

fn parse_measure_repeat<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<MeasureRepeat> {
    let repeat_type = match get_attr(start, "type")?.as_deref() {
        Some("stop") => StartStop::Stop,
        _ => StartStop::Start,
    };
    let slashes = get_attr(start, "slashes")?.and_then(|s| s.parse().ok());

    let text = read_text(reader, b"measure-repeat")?;
    let value = if text.is_empty() {
        None
    } else {
        Some(
            text.parse()
                .map_err(|_| ParseError::ParseNumber("measure-repeat".to_string()))?,
        )
    };

    Ok(MeasureRepeat {
        value,
        repeat_type,
        slashes,
    })
}

fn parse_beat_repeat<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<BeatRepeat> {
    let mut buf = Vec::new();
    let repeat_type = match get_attr(start, "type")?.as_deref() {
        Some("stop") => StartStopContinue::Stop,
        Some("continue") => StartStopContinue::Continue,
        _ => StartStopContinue::Start,
    };
    let slashes = get_attr(start, "slashes")?.and_then(|s| s.parse().ok());
    let use_dots = get_attr(start, "use-dots")?.and_then(|s| parse_yes_no_opt(&s));

    let mut slash_type: Option<NoteTypeValue> = None;
    let mut slash_dots: Vec<EmptyElement> = Vec::new();
    let mut except_voices: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"slash-type" => {
                    let s = read_text(reader, b"slash-type")?;
                    slash_type = Some(super::parse_note::parse_note_type_value(&s)?);
                }
                b"except-voice" => {
                    except_voices.push(read_text(reader, b"except-voice")?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"slash-dot" => {
                slash_dots.push(EmptyElement);
            }
            Event::End(e) if e.name().as_ref() == b"beat-repeat" => break,
            Event::Eof => return Err(ParseError::MissingElement("beat-repeat end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(BeatRepeat {
        repeat_type,
        slashes,
        use_dots,
        slash_type,
        slash_dots,
        except_voices,
    })
}

fn parse_slash<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Slash> {
    let mut buf = Vec::new();
    let slash_type = match get_attr(start, "type")?.as_deref() {
        Some("stop") => StartStop::Stop,
        _ => StartStop::Start,
    };
    let use_stems = get_attr(start, "use-stems")?.and_then(|s| parse_yes_no_opt(&s));
    let use_dots = get_attr(start, "use-dots")?.and_then(|s| parse_yes_no_opt(&s));

    let mut slash_type_element: Option<NoteTypeValue> = None;
    let mut slash_dots: Vec<EmptyElement> = Vec::new();
    let mut except_voices: Vec<String> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"slash-type" => {
                    let s = read_text(reader, b"slash-type")?;
                    slash_type_element = Some(super::parse_note::parse_note_type_value(&s)?);
                }
                b"except-voice" => {
                    except_voices.push(read_text(reader, b"except-voice")?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"slash-dot" => {
                slash_dots.push(EmptyElement);
            }
            Event::End(e) if e.name().as_ref() == b"slash" => break,
            Event::Eof => return Err(ParseError::MissingElement("slash end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Slash {
        slash_type,
        use_stems,
        use_dots,
        slash_type_element,
        slash_dots,
        except_voices,
    })
}

fn parse_yes_no_opt(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

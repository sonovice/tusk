//! Parser for MusicXML `<harmony>` elements.

use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::{ParseError, Reader, Result, get_attr, get_attr_required, read_text, skip_element};
use crate::model::data::{LeftRight, StartStop, Step, YesNo};
use crate::model::direction::Offset;
use crate::model::harmony::*;

/// Parse a `<harmony>` element.
pub fn parse_harmony<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Harmony> {
    let mut buf = Vec::new();

    let harmony_type = get_attr(start, "type")?.and_then(|s| match s.as_str() {
        "explicit" => Some(HarmonyType::Explicit),
        "implied" => Some(HarmonyType::Implied),
        "alternate" => Some(HarmonyType::Alternate),
        _ => None,
    });
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
    let print_frame = get_attr(start, "print-frame")?.and_then(|s| parse_yes_no_opt(&s));
    let arrangement = get_attr(start, "arrangement")?.and_then(|s| parse_arrangement(&s));
    let placement = get_attr(start, "placement")?.and_then(|s| match s.as_str() {
        "above" => Some(crate::model::data::AboveBelow::Above),
        "below" => Some(crate::model::data::AboveBelow::Below),
        _ => None,
    });
    let font_family = get_attr(start, "font-family")?;
    let font_size = get_attr(start, "font-size")?.and_then(|s| s.parse().ok());
    let font_style = get_attr(start, "font-style")?;
    let font_weight = get_attr(start, "font-weight")?;
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let color = get_attr(start, "color")?;
    let id = get_attr(start, "id")?;

    let mut chords: Vec<HarmonyChord> = Vec::new();
    let mut frame: Option<Frame> = None;
    let mut offset: Option<Offset> = None;
    let mut footnote = None;
    let mut level = None;
    let mut staff: Option<u32> = None;

    // Temporary state for building a chord
    let mut current_root_type: Option<HarmonyChordRoot> = None;
    let mut current_kind: Option<Kind> = None;
    let mut current_inversion: Option<Inversion> = None;
    let mut current_bass: Option<Bass> = None;
    let mut current_degrees: Vec<Degree> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"root" => {
                    // If we have a pending chord, flush it
                    flush_chord(
                        &mut chords,
                        &mut current_root_type,
                        &mut current_kind,
                        &mut current_inversion,
                        &mut current_bass,
                        &mut current_degrees,
                    );
                    current_root_type = Some(HarmonyChordRoot::Root(parse_root(reader)?));
                }
                b"numeral" => {
                    flush_chord(
                        &mut chords,
                        &mut current_root_type,
                        &mut current_kind,
                        &mut current_inversion,
                        &mut current_bass,
                        &mut current_degrees,
                    );
                    current_root_type = Some(HarmonyChordRoot::Numeral(parse_numeral(reader)?));
                }
                b"function" => {
                    flush_chord(
                        &mut chords,
                        &mut current_root_type,
                        &mut current_kind,
                        &mut current_inversion,
                        &mut current_bass,
                        &mut current_degrees,
                    );
                    current_root_type = Some(HarmonyChordRoot::Function(parse_style_text(
                        reader,
                        &e,
                        b"function",
                    )?));
                }
                b"kind" => {
                    current_kind = Some(parse_kind(reader, &e)?);
                }
                b"inversion" => {
                    current_inversion = Some(parse_inversion(reader, &e)?);
                }
                b"bass" => {
                    current_bass = Some(parse_bass(reader, &e)?);
                }
                b"degree" => {
                    current_degrees.push(parse_degree(reader, &e)?);
                }
                b"frame" => {
                    frame = Some(parse_frame(reader, &e)?);
                }
                b"offset" => {
                    offset = Some(parse_offset(reader, &e)?);
                }
                b"footnote" => {
                    footnote = Some(super::parse_note::parse_formatted_text(
                        reader,
                        &e,
                        b"footnote",
                    )?)
                }
                b"level" => level = Some(super::parse_note::parse_level(reader, &e)?),
                b"staff" => {
                    staff = Some(
                        read_text(reader, b"staff")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("staff".to_string()))?,
                    );
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"kind" => {
                    current_kind = Some(parse_kind_empty(&e)?);
                }
                b"offset" => {
                    offset = Some(parse_offset_empty(&e)?);
                }
                b"frame" => {
                    // Empty frame — no notes, unusual but possible
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"harmony" => break,
            Event::Eof => return Err(ParseError::MissingElement("harmony end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    // Flush the last chord
    flush_chord(
        &mut chords,
        &mut current_root_type,
        &mut current_kind,
        &mut current_inversion,
        &mut current_bass,
        &mut current_degrees,
    );

    Ok(Harmony {
        chords,
        frame,
        offset,
        footnote,
        level,
        staff,
        harmony_type,
        print_object,
        print_frame,
        arrangement,
        placement,
        font_family,
        font_size,
        font_style,
        font_weight,
        default_x,
        default_y,
        color,
        id,
    })
}

/// Flush accumulated chord fields into a completed HarmonyChord.
fn flush_chord(
    chords: &mut Vec<HarmonyChord>,
    root_type: &mut Option<HarmonyChordRoot>,
    kind: &mut Option<Kind>,
    inversion: &mut Option<Inversion>,
    bass: &mut Option<Bass>,
    degrees: &mut Vec<Degree>,
) {
    if let (Some(rt), Some(k)) = (root_type.take(), kind.take()) {
        chords.push(HarmonyChord {
            root_type: rt,
            kind: k,
            inversion: inversion.take(),
            bass: bass.take(),
            degrees: std::mem::take(degrees),
        });
    }
}

// ============================================================================
// Root parsing
// ============================================================================

fn parse_root<R: BufRead>(reader: &mut Reader<R>) -> Result<Root> {
    let mut buf = Vec::new();
    let mut root_step: Option<RootStep> = None;
    let mut root_alter: Option<HarmonyAlter> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"root-step" => root_step = Some(parse_step_element(reader, &e, b"root-step")?),
                b"root-alter" => root_alter = Some(parse_harmony_alter(reader, &e, b"root-alter")?),
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"root" => break,
            Event::Eof => return Err(ParseError::MissingElement("root end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Root {
        root_step: root_step.ok_or_else(|| ParseError::MissingElement("root-step".to_string()))?,
        root_alter,
    })
}

// ============================================================================
// Step element parsing (shared for root-step and bass-step)
// ============================================================================

fn parse_step_element<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    end_tag: &[u8],
) -> Result<RootStep> {
    let text_attr = get_attr(start, "text")?;
    let step_text = read_text(reader, end_tag)?;
    let value = parse_step_value(&step_text)?;
    Ok(RootStep {
        value,
        text: text_attr,
    })
}

pub(crate) fn parse_step_value(s: &str) -> Result<Step> {
    match s.trim() {
        "A" => Ok(Step::A),
        "B" => Ok(Step::B),
        "C" => Ok(Step::C),
        "D" => Ok(Step::D),
        "E" => Ok(Step::E),
        "F" => Ok(Step::F),
        "G" => Ok(Step::G),
        _ => Err(ParseError::InvalidContent(
            "step".to_string(),
            s.to_string(),
        )),
    }
}

// ============================================================================
// HarmonyAlter parsing (shared for root-alter, bass-alter, numeral-alter)
// ============================================================================

fn parse_harmony_alter<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    end_tag: &[u8],
) -> Result<HarmonyAlter> {
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
    let location = get_attr(start, "location")?.and_then(|s| match s.as_str() {
        "left" => Some(LeftRight::Left),
        "right" => Some(LeftRight::Right),
        _ => None,
    });
    let value: f64 = read_text(reader, end_tag)?
        .parse()
        .map_err(|_| ParseError::ParseNumber("harmony-alter".to_string()))?;

    Ok(HarmonyAlter {
        value,
        print_object,
        location,
    })
}

// ============================================================================
// Kind parsing
// ============================================================================

fn parse_kind<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Kind> {
    let text = get_attr(start, "text")?;
    let use_symbols = get_attr(start, "use-symbols")?.and_then(|s| parse_yes_no_opt(&s));
    let stack_degrees = get_attr(start, "stack-degrees")?.and_then(|s| parse_yes_no_opt(&s));
    let parentheses_degrees =
        get_attr(start, "parentheses-degrees")?.and_then(|s| parse_yes_no_opt(&s));
    let bracket_degrees = get_attr(start, "bracket-degrees")?.and_then(|s| parse_yes_no_opt(&s));
    let halign = get_attr(start, "halign")?.and_then(|s| match s.as_str() {
        "left" => Some(crate::model::data::LeftCenterRight::Left),
        "center" => Some(crate::model::data::LeftCenterRight::Center),
        "right" => Some(crate::model::data::LeftCenterRight::Right),
        _ => None,
    });
    let valign = get_attr(start, "valign")?.and_then(|s| match s.as_str() {
        "top" => Some(crate::model::data::Valign::Top),
        "middle" => Some(crate::model::data::Valign::Middle),
        "bottom" => Some(crate::model::data::Valign::Bottom),
        "baseline" => Some(crate::model::data::Valign::Baseline),
        _ => None,
    });

    let kind_text = read_text(reader, b"kind")?;
    let value = KindValue::from_str(&kind_text)
        .ok_or_else(|| ParseError::InvalidContent("kind".to_string(), kind_text))?;

    Ok(Kind {
        value,
        text,
        use_symbols,
        stack_degrees,
        parentheses_degrees,
        bracket_degrees,
        halign,
        valign,
    })
}

fn parse_kind_empty(start: &BytesStart) -> Result<Kind> {
    let text = get_attr(start, "text")?;
    let use_symbols = get_attr(start, "use-symbols")?.and_then(|s| parse_yes_no_opt(&s));
    let stack_degrees = get_attr(start, "stack-degrees")?.and_then(|s| parse_yes_no_opt(&s));
    let parentheses_degrees =
        get_attr(start, "parentheses-degrees")?.and_then(|s| parse_yes_no_opt(&s));
    let bracket_degrees = get_attr(start, "bracket-degrees")?.and_then(|s| parse_yes_no_opt(&s));
    let halign = get_attr(start, "halign")?.and_then(|s| match s.as_str() {
        "left" => Some(crate::model::data::LeftCenterRight::Left),
        "center" => Some(crate::model::data::LeftCenterRight::Center),
        "right" => Some(crate::model::data::LeftCenterRight::Right),
        _ => None,
    });
    let valign = get_attr(start, "valign")?.and_then(|s| match s.as_str() {
        "top" => Some(crate::model::data::Valign::Top),
        "middle" => Some(crate::model::data::Valign::Middle),
        "bottom" => Some(crate::model::data::Valign::Bottom),
        "baseline" => Some(crate::model::data::Valign::Baseline),
        _ => None,
    });

    // Empty <kind/> — unusual but treat as "other"
    Ok(Kind {
        value: KindValue::Other,
        text,
        use_symbols,
        stack_degrees,
        parentheses_degrees,
        bracket_degrees,
        halign,
        valign,
    })
}

// ============================================================================
// Inversion parsing
// ============================================================================

fn parse_inversion<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Inversion> {
    let text = get_attr(start, "text")?;
    let value: u32 = read_text(reader, b"inversion")?
        .parse()
        .map_err(|_| ParseError::ParseNumber("inversion".to_string()))?;
    Ok(Inversion { value, text })
}

// ============================================================================
// Bass parsing
// ============================================================================

fn parse_bass<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Bass> {
    let mut buf = Vec::new();
    let arrangement = get_attr(start, "arrangement")?.and_then(|s| parse_arrangement(&s));
    let mut bass_separator: Option<StyleText> = None;
    let mut bass_step: Option<BassStep> = None;
    let mut bass_alter: Option<HarmonyAlter> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"bass-separator" => {
                    bass_separator = Some(parse_style_text(reader, &e, b"bass-separator")?);
                }
                b"bass-step" => {
                    let text_attr = get_attr(&e, "text")?;
                    let step_text = read_text(reader, b"bass-step")?;
                    let value = parse_step_value(&step_text)?;
                    bass_step = Some(BassStep {
                        value,
                        text: text_attr,
                    });
                }
                b"bass-alter" => {
                    bass_alter = Some(parse_harmony_alter(reader, &e, b"bass-alter")?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"bass" => break,
            Event::Eof => return Err(ParseError::MissingElement("bass end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Bass {
        bass_separator,
        bass_step: bass_step.ok_or_else(|| ParseError::MissingElement("bass-step".to_string()))?,
        bass_alter,
        arrangement,
    })
}

// ============================================================================
// Degree parsing
// ============================================================================

fn parse_degree<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Degree> {
    let mut buf = Vec::new();
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
    let mut degree_value: Option<DegreeValue> = None;
    let mut degree_alter: Option<DegreeAlter> = None;
    let mut degree_type: Option<DegreeType> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"degree-value" => {
                    degree_value = Some(parse_degree_value(reader, &e)?);
                }
                b"degree-alter" => {
                    degree_alter = Some(parse_degree_alter(reader, &e)?);
                }
                b"degree-type" => {
                    degree_type = Some(parse_degree_type(reader, &e)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"degree" => break,
            Event::Eof => return Err(ParseError::MissingElement("degree end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Degree {
        degree_value: degree_value
            .ok_or_else(|| ParseError::MissingElement("degree-value".to_string()))?,
        degree_alter: degree_alter
            .ok_or_else(|| ParseError::MissingElement("degree-alter".to_string()))?,
        degree_type: degree_type
            .ok_or_else(|| ParseError::MissingElement("degree-type".to_string()))?,
        print_object,
    })
}

fn parse_degree_value<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<DegreeValue> {
    let symbol = get_attr(start, "symbol")?.and_then(|s| DegreeSymbolValue::from_str(&s));
    let text = get_attr(start, "text")?;
    let value: u32 = read_text(reader, b"degree-value")?
        .parse()
        .map_err(|_| ParseError::ParseNumber("degree-value".to_string()))?;
    Ok(DegreeValue {
        value,
        symbol,
        text,
    })
}

fn parse_degree_alter<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<DegreeAlter> {
    let plus_minus = get_attr(start, "plus-minus")?.and_then(|s| parse_yes_no_opt(&s));
    let value: f64 = read_text(reader, b"degree-alter")?
        .parse()
        .map_err(|_| ParseError::ParseNumber("degree-alter".to_string()))?;
    Ok(DegreeAlter { value, plus_minus })
}

fn parse_degree_type<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<DegreeType> {
    let text = get_attr(start, "text")?;
    let type_text = read_text(reader, b"degree-type")?;
    let value = DegreeTypeValue::from_str(&type_text)
        .ok_or_else(|| ParseError::InvalidContent("degree-type".to_string(), type_text))?;
    Ok(DegreeType { value, text })
}

// ============================================================================
// Numeral parsing
// ============================================================================

fn parse_numeral<R: BufRead>(reader: &mut Reader<R>) -> Result<Numeral> {
    let mut buf = Vec::new();
    let mut numeral_root: Option<NumeralRoot> = None;
    let mut numeral_alter: Option<HarmonyAlter> = None;
    let mut numeral_key: Option<NumeralKey> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"numeral-root" => {
                    let text = get_attr(&e, "text")?;
                    let value: u32 = read_text(reader, b"numeral-root")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("numeral-root".to_string()))?;
                    numeral_root = Some(NumeralRoot { value, text });
                }
                b"numeral-alter" => {
                    numeral_alter = Some(parse_harmony_alter(reader, &e, b"numeral-alter")?);
                }
                b"numeral-key" => {
                    numeral_key = Some(parse_numeral_key(reader, &e)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"numeral" => break,
            Event::Eof => return Err(ParseError::MissingElement("numeral end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Numeral {
        numeral_root: numeral_root
            .ok_or_else(|| ParseError::MissingElement("numeral-root".to_string()))?,
        numeral_alter,
        numeral_key,
    })
}

fn parse_numeral_key<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<NumeralKey> {
    let mut buf = Vec::new();
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
    let mut numeral_fifths: Option<i8> = None;
    let mut numeral_mode: Option<NumeralMode> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"numeral-fifths" => {
                    numeral_fifths = Some(
                        read_text(reader, b"numeral-fifths")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("numeral-fifths".to_string()))?,
                    );
                }
                b"numeral-mode" => {
                    let mode_text = read_text(reader, b"numeral-mode")?;
                    numeral_mode = Some(NumeralMode::from_str(&mode_text).ok_or_else(|| {
                        ParseError::InvalidContent("numeral-mode".to_string(), mode_text)
                    })?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"numeral-key" => break,
            Event::Eof => return Err(ParseError::MissingElement("numeral-key end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(NumeralKey {
        numeral_fifths: numeral_fifths
            .ok_or_else(|| ParseError::MissingElement("numeral-fifths".to_string()))?,
        numeral_mode: numeral_mode
            .ok_or_else(|| ParseError::MissingElement("numeral-mode".to_string()))?,
        print_object,
    })
}

// ============================================================================
// Frame parsing
// ============================================================================

fn parse_frame<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Frame> {
    let mut buf = Vec::new();
    let height = get_attr(start, "height")?.and_then(|s| s.parse().ok());
    let width = get_attr(start, "width")?.and_then(|s| s.parse().ok());
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let halign = get_attr(start, "halign")?.and_then(|s| match s.as_str() {
        "left" => Some(crate::model::data::LeftCenterRight::Left),
        "center" => Some(crate::model::data::LeftCenterRight::Center),
        "right" => Some(crate::model::data::LeftCenterRight::Right),
        _ => None,
    });
    let valign = get_attr(start, "valign")?.and_then(|s| match s.as_str() {
        "top" => Some(crate::model::data::Valign::Top),
        "middle" => Some(crate::model::data::Valign::Middle),
        "bottom" => Some(crate::model::data::Valign::Bottom),
        "baseline" => Some(crate::model::data::Valign::Baseline),
        _ => None,
    });
    let unplayed = get_attr(start, "unplayed")?;
    let color = get_attr(start, "color")?;
    let id = get_attr(start, "id")?;

    let mut frame_strings: Option<u32> = None;
    let mut frame_frets: Option<u32> = None;
    let mut first_fret: Option<FirstFret> = None;
    let mut frame_notes: Vec<FrameNote> = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"frame-strings" => {
                    frame_strings = Some(
                        read_text(reader, b"frame-strings")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("frame-strings".to_string()))?,
                    );
                }
                b"frame-frets" => {
                    frame_frets = Some(
                        read_text(reader, b"frame-frets")?
                            .parse()
                            .map_err(|_| ParseError::ParseNumber("frame-frets".to_string()))?,
                    );
                }
                b"first-fret" => {
                    first_fret = Some(parse_first_fret(reader, &e)?);
                }
                b"frame-note" => {
                    frame_notes.push(parse_frame_note(reader)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::End(e) if e.name().as_ref() == b"frame" => break,
            Event::Eof => return Err(ParseError::MissingElement("frame end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(Frame {
        frame_strings: frame_strings
            .ok_or_else(|| ParseError::MissingElement("frame-strings".to_string()))?,
        frame_frets: frame_frets
            .ok_or_else(|| ParseError::MissingElement("frame-frets".to_string()))?,
        first_fret,
        frame_notes,
        height,
        width,
        default_x,
        default_y,
        halign,
        valign,
        unplayed,
        color,
        id,
    })
}

fn parse_first_fret<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<FirstFret> {
    let text = get_attr(start, "text")?;
    let location = get_attr(start, "location")?.and_then(|s| match s.as_str() {
        "left" => Some(LeftRight::Left),
        "right" => Some(LeftRight::Right),
        _ => None,
    });
    let value: u32 = read_text(reader, b"first-fret")?
        .parse()
        .map_err(|_| ParseError::ParseNumber("first-fret".to_string()))?;
    Ok(FirstFret {
        value,
        text,
        location,
    })
}

fn parse_frame_note<R: BufRead>(reader: &mut Reader<R>) -> Result<FrameNote> {
    let mut buf = Vec::new();
    let mut string: Option<FrameString> = None;
    let mut fret: Option<Fret> = None;
    let mut fingering: Option<FrameFingering> = None;
    let mut barre: Option<Barre> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"string" => {
                    let placement = get_attr(&e, "placement")?.and_then(|s| match s.as_str() {
                        "above" => Some(crate::model::data::AboveBelow::Above),
                        "below" => Some(crate::model::data::AboveBelow::Below),
                        _ => None,
                    });
                    let value: u32 = read_text(reader, b"string")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("string".to_string()))?;
                    string = Some(FrameString { value, placement });
                }
                b"fret" => {
                    let value: u32 = read_text(reader, b"fret")?
                        .parse()
                        .map_err(|_| ParseError::ParseNumber("fret".to_string()))?;
                    fret = Some(Fret { value });
                }
                b"fingering" => {
                    let substitution =
                        get_attr(&e, "substitution")?.and_then(|s| parse_yes_no_opt(&s));
                    let alternate = get_attr(&e, "alternate")?.and_then(|s| parse_yes_no_opt(&s));
                    let value = read_text(reader, b"fingering")?;
                    fingering = Some(FrameFingering {
                        value,
                        substitution,
                        alternate,
                    });
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => {
                if e.name().as_ref() == b"barre" {
                    let barre_type = parse_start_stop_val(&get_attr_required(&e, "type")?)?;
                    let barre_color = get_attr(&e, "color")?;
                    barre = Some(Barre {
                        barre_type,
                        color: barre_color,
                    });
                }
            }
            Event::End(e) if e.name().as_ref() == b"frame-note" => break,
            Event::Eof => return Err(ParseError::MissingElement("frame-note end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    Ok(FrameNote {
        string: string.ok_or_else(|| ParseError::MissingElement("string".to_string()))?,
        fret: fret.ok_or_else(|| ParseError::MissingElement("fret".to_string()))?,
        fingering,
        barre,
    })
}

// ============================================================================
// Style text parsing (for function, bass-separator)
// ============================================================================

fn parse_style_text<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
    end_tag: &[u8],
) -> Result<StyleText> {
    let font_family = get_attr(start, "font-family")?;
    let font_style = get_attr(start, "font-style")?;
    let font_size = get_attr(start, "font-size")?.and_then(|s| s.parse().ok());
    let font_weight = get_attr(start, "font-weight")?;
    let color = get_attr(start, "color")?;
    let value = read_text(reader, end_tag)?;
    Ok(StyleText {
        value,
        font_family,
        font_style,
        font_size,
        font_weight,
        color,
    })
}

// ============================================================================
// Offset parsing
// ============================================================================

fn parse_offset<R: BufRead>(reader: &mut Reader<R>, start: &BytesStart) -> Result<Offset> {
    let sound = get_attr(start, "sound")?.and_then(|s| parse_yes_no_opt(&s));
    let value: f64 = read_text(reader, b"offset")?
        .parse()
        .map_err(|_| ParseError::ParseNumber("offset".to_string()))?;
    Ok(Offset { value, sound })
}

fn parse_offset_empty(start: &BytesStart) -> Result<Offset> {
    let sound = get_attr(start, "sound")?.and_then(|s| parse_yes_no_opt(&s));
    Ok(Offset { value: 0.0, sound })
}

// ============================================================================
// Helper functions
// ============================================================================

fn parse_yes_no_opt(s: &str) -> Option<YesNo> {
    match s {
        "yes" => Some(YesNo::Yes),
        "no" => Some(YesNo::No),
        _ => None,
    }
}

fn parse_arrangement(s: &str) -> Option<HarmonyArrangement> {
    match s {
        "vertical" => Some(HarmonyArrangement::Vertical),
        "horizontal" => Some(HarmonyArrangement::Horizontal),
        "diagonal" => Some(HarmonyArrangement::Diagonal),
        _ => None,
    }
}

fn parse_start_stop_val(s: &str) -> Result<StartStop> {
    match s {
        "start" => Ok(StartStop::Start),
        "stop" => Ok(StartStop::Stop),
        _ => Err(ParseError::InvalidAttribute(
            "start-stop".to_string(),
            s.to_string(),
        )),
    }
}

//! Metronome parsing for all MusicXML 4.1 metronome forms.
//!
//! Handles:
//! - Beat-unit + per-minute (standard tempo)
//! - Beat-unit + beat-unit (metric modulation)
//! - Beat-unit-tied (tied beat units)
//! - Metronome-note form (complex metric relationships, swing)
//! - Metronome-arrows, metronome-beam, metronome-tied, metronome-tuplet

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::io::BufRead;

use super::{ParseError, Result, get_attr, read_text, skip_element};
use crate::model::data::*;
use crate::model::direction::*;

pub(crate) fn parse_metronome<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<Metronome> {
    let parentheses = get_attr(start, "parentheses")?.and_then(|s| parse_yes_no_opt(&s));
    let print_object = get_attr(start, "print-object")?.and_then(|s| parse_yes_no_opt(&s));
    let justify = get_attr(start, "justify")?.and_then(|s| parse_lcr(&s));
    let default_x = get_attr(start, "default-x")?.and_then(|s| s.parse().ok());
    let default_y = get_attr(start, "default-y")?.and_then(|s| s.parse().ok());
    let halign = get_attr(start, "halign")?.and_then(|s| parse_lcr(&s));
    let valign = get_attr(start, "valign")?.and_then(|s| parse_valign(&s));
    let id = get_attr(start, "id")?;

    // Collect all child elements as tokens, then determine which form
    let mut tokens = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"beat-unit" => {
                    tokens.push(MetronomeToken::BeatUnit(read_text(reader, b"beat-unit")?));
                }
                b"per-minute" => {
                    tokens.push(MetronomeToken::PerMinute(read_text(reader, b"per-minute")?));
                }
                b"beat-unit-tied" => {
                    tokens.push(MetronomeToken::BeatUnitTied(parse_beat_unit_tied(reader)?));
                }
                b"metronome-note" => {
                    tokens.push(MetronomeToken::MetronomeNote(parse_metronome_note(reader)?));
                }
                b"metronome-relation" => {
                    tokens.push(MetronomeToken::MetronomeRelation(read_text(
                        reader,
                        b"metronome-relation",
                    )?));
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"beat-unit-dot" => tokens.push(MetronomeToken::BeatUnitDot),
                b"metronome-arrows" => tokens.push(MetronomeToken::MetronomeArrows),
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"metronome" => break,
            Event::Eof => return Err(ParseError::MissingElement("metronome end".to_string())),
            _ => {}
        }
        buf.clear();
    }

    let content = build_metronome_content(tokens);

    Ok(Metronome {
        content,
        parentheses,
        print_object,
        justify,
        default_x,
        default_y,
        halign,
        valign,
        id,
    })
}

/// Token collected while parsing metronome children.
enum MetronomeToken {
    BeatUnit(String),
    BeatUnitDot,
    BeatUnitTied(BeatUnitTied),
    PerMinute(String),
    MetronomeArrows,
    MetronomeNote(MetronomeNote),
    MetronomeRelation(String),
}

/// Build MetronomeContent from collected tokens.
fn build_metronome_content(tokens: Vec<MetronomeToken>) -> MetronomeContent {
    // Check if this is a metronome-note form
    let has_metronome_note = tokens
        .iter()
        .any(|t| matches!(t, MetronomeToken::MetronomeNote(_)));

    if has_metronome_note {
        return build_metronome_note_content(tokens);
    }

    // Beat-unit form: collect beat-units, dots, tied, and per-minute
    let mut beat_units: Vec<String> = Vec::new();
    let mut dot_counts: Vec<usize> = vec![0]; // dots for current beat-unit
    let mut tied_groups: Vec<Vec<BeatUnitTied>> = vec![Vec::new()]; // tied for current group
    let mut per_minute: Option<String> = None;

    for token in tokens {
        match token {
            MetronomeToken::BeatUnit(bu) => {
                if beat_units.is_empty() {
                    beat_units.push(bu);
                } else {
                    // Second beat-unit starts a new group
                    beat_units.push(bu);
                    dot_counts.push(0);
                    tied_groups.push(Vec::new());
                }
            }
            MetronomeToken::BeatUnitDot => {
                let idx = dot_counts.len().saturating_sub(1);
                dot_counts[idx] += 1;
            }
            MetronomeToken::BeatUnitTied(tied) => {
                let idx = tied_groups.len().saturating_sub(1);
                tied_groups[idx].push(tied);
            }
            MetronomeToken::PerMinute(pm) => per_minute = Some(pm),
            _ => {}
        }
    }

    if let Some(pm) = per_minute {
        // beat-unit = per-minute form
        MetronomeContent::BeatUnit {
            beat_unit: beat_units.into_iter().next().unwrap_or("quarter".into()),
            beat_unit_dots: vec![(); dot_counts.first().copied().unwrap_or(0)],
            beat_unit_tied: tied_groups.into_iter().next().unwrap_or_default(),
            per_minute: pm,
        }
    } else if beat_units.len() >= 2 {
        // beat-unit = beat-unit form (metric modulation)
        let mut bu_iter = beat_units.into_iter();
        let mut dc_iter = dot_counts.into_iter();
        let mut tg_iter = tied_groups.into_iter();
        MetronomeContent::BeatUnitEquivalent(MetricModulation {
            beat_unit_1: bu_iter.next().unwrap(),
            beat_unit_dots_1: vec![(); dc_iter.next().unwrap_or(0)],
            beat_unit_tied_1: tg_iter.next().unwrap_or_default(),
            beat_unit_2: bu_iter.next().unwrap(),
            beat_unit_dots_2: vec![(); dc_iter.next().unwrap_or(0)],
            beat_unit_tied_2: tg_iter.next().unwrap_or_default(),
        })
    } else {
        // Fallback
        MetronomeContent::BeatUnit {
            beat_unit: beat_units.into_iter().next().unwrap_or("quarter".into()),
            beat_unit_dots: vec![(); dot_counts.first().copied().unwrap_or(0)],
            beat_unit_tied: Vec::new(),
            per_minute: "120".to_string(),
        }
    }
}

/// Build MetronomeNoteContent from tokens.
fn build_metronome_note_content(tokens: Vec<MetronomeToken>) -> MetronomeContent {
    let mut arrows = false;
    let mut notes_1 = Vec::new();
    let mut relation: Option<String> = None;
    let mut notes_2 = Vec::new();
    let mut after_relation = false;

    for token in tokens {
        match token {
            MetronomeToken::MetronomeArrows => arrows = true,
            MetronomeToken::MetronomeNote(note) => {
                if after_relation {
                    notes_2.push(note);
                } else {
                    notes_1.push(note);
                }
            }
            MetronomeToken::MetronomeRelation(rel) => {
                relation = Some(if rel.is_empty() {
                    "equals".to_string()
                } else {
                    rel
                });
                after_relation = true;
            }
            _ => {}
        }
    }

    MetronomeContent::MetronomeNotes(MetronomeNoteContent {
        arrows,
        notes_1,
        relation,
        notes_2,
    })
}

/// Parse `<beat-unit-tied>` element containing a beat-unit group.
fn parse_beat_unit_tied<R: BufRead>(reader: &mut Reader<R>) -> Result<BeatUnitTied> {
    let mut buf = Vec::new();
    let mut beat_unit: Option<String> = None;
    let mut dots = 0usize;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) if e.name().as_ref() == b"beat-unit" => {
                beat_unit = Some(read_text(reader, b"beat-unit")?);
            }
            Event::Empty(e) if e.name().as_ref() == b"beat-unit-dot" => dots += 1,
            Event::Start(e) => skip_element(reader, &e)?,
            Event::End(e) if e.name().as_ref() == b"beat-unit-tied" => break,
            Event::Eof => return Err(ParseError::MissingElement("beat-unit-tied end".into())),
            _ => {}
        }
        buf.clear();
    }

    Ok(BeatUnitTied {
        beat_unit: beat_unit.unwrap_or("quarter".into()),
        beat_unit_dots: vec![(); dots],
    })
}

/// Parse `<metronome-note>` element.
fn parse_metronome_note<R: BufRead>(reader: &mut Reader<R>) -> Result<MetronomeNote> {
    let mut buf = Vec::new();
    let mut note_type: Option<String> = None;
    let mut dots = 0usize;
    let mut beams = Vec::new();
    let mut tied: Option<MetronomeTied> = None;
    let mut tuplet: Option<MetronomeTuplet> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"metronome-type" => {
                    note_type = Some(read_text(reader, b"metronome-type")?);
                }
                b"metronome-beam" => {
                    let number = get_attr(&e, "number")?
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(1);
                    let value = read_text(reader, b"metronome-beam")?;
                    beams.push(MetronomeBeam { number, value });
                }
                b"metronome-tuplet" => {
                    tuplet = Some(parse_metronome_tuplet(reader, &e)?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) => match e.name().as_ref() {
                b"metronome-dot" => dots += 1,
                b"metronome-tied" => {
                    let type_str = get_attr(&e, "type")?.unwrap_or_default();
                    tied = Some(MetronomeTied {
                        tied_type: if type_str == "stop" {
                            StartStop::Stop
                        } else {
                            StartStop::Start
                        },
                    });
                }
                _ => {}
            },
            Event::End(e) if e.name().as_ref() == b"metronome-note" => break,
            Event::Eof => return Err(ParseError::MissingElement("metronome-note end".into())),
            _ => {}
        }
        buf.clear();
    }

    Ok(MetronomeNote {
        note_type: note_type.unwrap_or("quarter".into()),
        dots: vec![(); dots],
        beams,
        tied,
        tuplet,
    })
}

/// Parse `<metronome-tuplet>` element.
fn parse_metronome_tuplet<R: BufRead>(
    reader: &mut Reader<R>,
    start: &BytesStart,
) -> Result<MetronomeTuplet> {
    let type_str = get_attr(start, "type")?.unwrap_or_default();
    let tuplet_type = if type_str == "stop" {
        StartStop::Stop
    } else {
        StartStop::Start
    };
    let bracket = get_attr(start, "bracket")?.and_then(|s| parse_yes_no_opt(&s));
    let show_number = get_attr(start, "show-number")?;

    let mut buf = Vec::new();
    let mut actual_notes: Option<u32> = None;
    let mut normal_notes: Option<u32> = None;
    let mut normal_type: Option<String> = None;
    let mut normal_dots = 0usize;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => match e.name().as_ref() {
                b"actual-notes" => {
                    actual_notes = read_text(reader, b"actual-notes")?.parse().ok();
                }
                b"normal-notes" => {
                    normal_notes = read_text(reader, b"normal-notes")?.parse().ok();
                }
                b"normal-type" => {
                    normal_type = Some(read_text(reader, b"normal-type")?);
                }
                _ => skip_element(reader, &e)?,
            },
            Event::Empty(e) if e.name().as_ref() == b"normal-dot" => normal_dots += 1,
            Event::End(e) if e.name().as_ref() == b"metronome-tuplet" => break,
            Event::Eof => {
                return Err(ParseError::MissingElement("metronome-tuplet end".into()));
            }
            _ => {}
        }
        buf.clear();
    }

    Ok(MetronomeTuplet {
        tuplet_type,
        bracket,
        show_number,
        actual_notes,
        normal_notes,
        normal_type,
        normal_dots: vec![(); normal_dots],
    })
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

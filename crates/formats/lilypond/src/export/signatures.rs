//! Clef / key / time event sequence extraction and injection for LilyPond export.

use tusk_model::elements::ScoreDefChild;
use tusk_model::extensions::ExtensionStore;
use tusk_model::{ControlEvent, EventSequence};

use crate::model::pitch::Pitch;
use crate::model::signature::{Clef, KeySignature, TimeSignature};
use crate::model::{Mode, Music};

/// A signature event parsed from the staffDef label.
pub(super) struct SignatureEvent {
    /// Position in the note/rest stream (0-based).
    pub(super) position: u32,
    /// The Music expression to inject.
    pub(super) music: Music,
}

/// Extract event sequences from all staffDefs via ext_store.
pub(super) fn extract_event_sequences(
    score: &tusk_model::elements::Score,
    ext_store: &ExtensionStore,
) -> Vec<Vec<SignatureEvent>> {
    let mut result = Vec::new();
    for child in &score.children {
        if let tusk_model::elements::ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for sdef in super::collect_staff_defs_from_grp(grp) {
                        result.push(parse_event_sequence_from_ext(sdef, ext_store));
                    }
                }
            }
        }
    }
    result
}

/// Read event sequence from ext_store, falling back to staffDef attributes.
fn parse_event_sequence_from_ext(staff_def: &tusk_model::elements::StaffDef, ext_store: &ExtensionStore) -> Vec<SignatureEvent> {
    if let Some(id) = staff_def.basic.xml_id.as_deref()
        && let Some(seq) = ext_store.event_sequence(id) {
            return convert_event_sequence(seq.clone());
        }

    // No event sequence -- try to reconstruct from staffDef attributes
    reconstruct_initial_signatures(staff_def)
}

/// Convert a typed EventSequence to SignatureEvents.
fn convert_event_sequence(seq: EventSequence) -> Vec<SignatureEvent> {
    let mut events = Vec::new();
    for pe in seq.events {
        let music = match pe.event {
            ControlEvent::Clef { name } => Music::Clef(Clef { name }),
            ControlEvent::Key { step, alter, mode } => {
                if let Some(m) = Mode::from_name(&mode) {
                    Music::KeySignature(KeySignature {
                        pitch: Pitch {
                            step,
                            alter,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                        mode: m,
                    })
                } else {
                    continue;
                }
            }
            ControlEvent::Time {
                numerators,
                denominator,
            } => Music::TimeSignature(TimeSignature {
                numerators,
                denominator,
            }),
            ControlEvent::BarCheck => Music::BarCheck,
            ControlEvent::BarLine { bar_type } => Music::BarLine { bar_type },
            ControlEvent::AutoBeamOn => Music::AutoBeamOn,
            ControlEvent::AutoBeamOff => Music::AutoBeamOff,
            ControlEvent::Tempo { serialized } => {
                if let Some(t) = parse_tempo_from_label(&serialized) {
                    Music::Tempo(t)
                } else {
                    continue;
                }
            }
            ControlEvent::Mark { serialized } => {
                if let Some(m) = parse_mark_from_label(&serialized) {
                    Music::Mark(m)
                } else {
                    continue;
                }
            }
            ControlEvent::TextMark { serialized } => {
                if let Some(tm) = parse_textmark_from_label(&serialized) {
                    Music::TextMark(tm)
                } else {
                    continue;
                }
            }
            ControlEvent::Markup { serialized } => {
                if let Some(m) = parse_markup_from_label(&serialized) {
                    Music::Markup(m)
                } else {
                    continue;
                }
            }
            ControlEvent::MarkupList { serialized } => {
                if let Some(ml) = parse_markuplist_from_label(&serialized) {
                    Music::MarkupList(ml)
                } else {
                    continue;
                }
            }
        };
        events.push(SignatureEvent {
            position: pe.position,
            music,
        });
    }
    events
}

/// Reconstruct initial clef/key/time from staffDef attributes when no event label exists.
fn reconstruct_initial_signatures(
    staff_def: &tusk_model::elements::StaffDef,
) -> Vec<SignatureEvent> {
    let mut events = Vec::new();

    // Clef
    if let Some(ref shape) = staff_def.staff_def_log.clef_shape {
        let line = staff_def
            .staff_def_log
            .clef_line
            .as_ref()
            .map(|l| l.0)
            .unwrap_or(2);
        let dis = staff_def.staff_def_log.clef_dis.as_ref().map(|d| d.0);
        let dis_place = staff_def.staff_def_log.clef_dis_place.as_ref();
        let name = crate::import::mei_clef_to_name(shape, line, dis, dis_place);
        events.push(SignatureEvent {
            position: 0,
            music: Music::Clef(Clef { name }),
        });
    }

    // Key
    if let Some(ref keysig) = staff_def.staff_def_log.keysig
        && let Ok(fifths) = keysig.0.parse::<i32>()
    {
        let (pitch, mode) = crate::import::fifths_to_key(fifths);
        events.push(SignatureEvent {
            position: 0,
            music: Music::KeySignature(KeySignature { pitch, mode }),
        });
    }

    // Meter
    if let Some(ref count) = staff_def.staff_def_log.meter_count {
        let numerators: Vec<u32> = count
            .split('+')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let denominator: u32 = staff_def
            .staff_def_log
            .meter_unit
            .as_ref()
            .and_then(|u| u.parse().ok())
            .unwrap_or(4);
        if !numerators.is_empty() {
            events.push(SignatureEvent {
                position: 0,
                music: Music::TimeSignature(TimeSignature {
                    numerators,
                    denominator,
                }),
            });
        }
    }

    events
}

/// Count the total notes in a list of Music items.
pub(super) fn total_note_count(items: &[Music]) -> u32 {
    items.iter().map(note_count).sum()
}

/// Inject signature events from the event sequence into a measure's items,
/// using only events whose position falls in [note_offset, note_offset + measure_note_count).
///
/// Skips Clef/KeySignature/TimeSignature events for non-first measures because
/// voice splitting can change note counting between passes, causing these events
/// to be injected at unstable positions. Tempo/Mark/TextMark/Markup events are
/// kept since they don't cause roundtrip instability.
pub(super) fn inject_measure_signature_events(
    items: &mut Vec<Music>,
    events: &[SignatureEvent],
    note_offset: u32,
) {
    let measure_note_count = total_note_count(items);
    // Filter events for this measure, skip clef/key/time (unstable positions)
    let measure_events: Vec<SignatureEvent> = events
        .iter()
        .filter(|e| e.position >= note_offset && e.position < note_offset + measure_note_count)
        .filter(|e| !matches!(e.music, Music::Clef(_) | Music::KeySignature(_) | Music::TimeSignature(_)))
        .map(|e| SignatureEvent {
            position: e.position - note_offset,
            music: e.music.clone(),
        })
        .collect();
    if !measure_events.is_empty() {
        inject_signature_events(items, &measure_events);
    }
}

/// Count how many note-stream positions a Music item occupies.
///
/// Bare notes count as 1. Wrappers (Tuplet/Grace/Repeat) count as the
/// total notes they contain. Non-note items (Override, MusicFunction, etc.)
/// count as 0. Must stay consistent with import's `build_event_sequence`
/// note counting.
fn note_count(m: &Music) -> u32 {
    match m {
        Music::Note(_)
        | Music::Chord(_)
        | Music::Rest(_)
        | Music::MultiMeasureRest(_)
        | Music::ChordRepetition(_)
        | Music::DrumNote(_)
        | Music::DrumChord(_) => 1,
        Music::Tuplet { body, .. } => note_count(body),
        Music::Grace { body }
        | Music::Acciaccatura { body }
        | Music::Appoggiatura { body } => note_count(body),
        Music::AfterGrace { main, grace, .. } => note_count(main) + note_count(grace),
        Music::Repeat {
            body, alternatives, ..
        } => {
            let mut n = note_count(body);
            if let Some(alts) = alternatives {
                for a in alts {
                    n += note_count(a);
                }
            }
            n
        }
        Music::Sequential(items) => items.iter().map(note_count).sum(),
        Music::Simultaneous(items) => {
            let voice_like_count = items
                .iter()
                .filter(|i| {
                    matches!(
                        i,
                        Music::Sequential(_)
                            | Music::Relative { .. }
                            | Music::Fixed { .. }
                            | Music::Transpose { .. }
                            | Music::ContextedMusic { .. }
                    )
                })
                .count();
            let is_inline_polyphony = items.len() >= 2 && voice_like_count >= 2;
            if is_inline_polyphony {
                items.first().map(note_count).unwrap_or(0)
            } else {
                items.iter().map(note_count).sum()
            }
        }
        Music::Once { music } => note_count(music),
        _ => 0,
    }
}

/// Inject signature events into a single layer at the correct positions.
///
/// Events are keyed by position in the note/rest stream. Runs AFTER
/// wrapping steps so signatures stay outside tuplet/grace/repeat wrappers.
/// Uses recursive `note_count` to correctly handle both non-note items
/// (count=0, skipped) and wrapper items (count=N notes inside).
pub(super) fn inject_signature_events(items: &mut Vec<Music>, events: &[SignatureEvent]) {
    if items.is_empty() || events.is_empty() {
        return;
    }

    // Build insertion map: position -> list of Music to insert (in order)
    let mut inserts: std::collections::BTreeMap<u32, Vec<Music>> =
        std::collections::BTreeMap::new();
    for ev in events {
        inserts
            .entry(ev.position)
            .or_default()
            .push(ev.music.clone());
    }

    // Rebuild layer with injected events, using note_count for positions
    let mut new_items = Vec::new();
    let mut pending_zero = inserts.remove(&0).unwrap_or_default();
    dedupe_existing_leading_zero_events(items, &mut pending_zero);
    let mut inserted_zero = pending_zero.is_empty();
    let mut note_idx: u32 = 0;
    for item in items.drain(..) {
        if !inserted_zero && note_idx == 0 && should_stay_before_initial_signatures(&item) {
            new_items.push(item);
            continue;
        }
        let nc = note_count(&item);
        if nc > 0 {
            if !inserted_zero {
                new_items.append(&mut pending_zero);
                inserted_zero = true;
            }
            if let Some(to_insert) = inserts.remove(&note_idx) {
                new_items.extend(to_insert);
            }
            note_idx += nc;
        } else if !inserted_zero && note_idx == 0 {
            new_items.append(&mut pending_zero);
            inserted_zero = true;
        }
        new_items.push(item);
    }
    if !inserted_zero {
        new_items.append(&mut pending_zero);
    }
    // Remaining events are for later measures (positions beyond this
    // measure's note range). They're already represented in those
    // measures' MEI layers, so discard them here.
    *items = new_items;
}

fn dedupe_existing_leading_zero_events(items: &[Music], pending_zero: &mut Vec<Music>) {
    if pending_zero.is_empty() {
        return;
    }

    for item in items {
        if note_count(item) > 0 {
            break;
        }
        if let Some(pos) = pending_zero.iter().position(|candidate| candidate == item) {
            pending_zero.remove(pos);
        }
    }
}

fn should_stay_before_initial_signatures(item: &Music) -> bool {
    matches!(item, Music::LineComment(_))
        || matches!(item, Music::Set { path, .. } if !property_path_ends_with(path, "skipBars"))
}

fn property_path_ends_with(path: &crate::model::PropertyPath, segment: &str) -> bool {
    path.segments
        .last()
        .and_then(|part| match part {
            crate::model::PathSegment::Named(name) => Some(name.as_str()),
            crate::model::PathSegment::Scheme(_) => None,
        })
        .is_some_and(|name| name == segment)
}

/// Re-parse a serialized markup string back into a `Markup` AST node.
///
/// The string is the serialized form produced by `serialize_markup()` — i.e.
/// the content after `\markup`, not including the keyword itself.
fn parse_markup_from_label(s: &str) -> Option<crate::model::markup::Markup> {
    use crate::parser::Parser;
    // Wrap in a form the parser can handle
    let src = format!("\\markup {s}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Markup(m) = item {
            return Some(m.clone());
        }
    }
    None
}

/// Re-parse a serialized markuplist string back into a `MarkupList` AST node.
fn parse_markuplist_from_label(s: &str) -> Option<crate::model::markup::MarkupList> {
    use crate::parser::Parser;
    let src = format!("\\markuplist {s}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::MarkupList(ml) = item {
            return Some(ml.clone());
        }
    }
    None
}

/// Re-parse a serialized `\tempo ...` string back into a Tempo AST node.
fn parse_tempo_from_label(s: &str) -> Option<crate::model::signature::Tempo> {
    use crate::parser::Parser;
    // Append a dummy note so the parser can consume the tempo
    let src = format!("{s}\nc4");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::Sequential(items)) = item {
            for m in items {
                if let Music::Tempo(t) = m {
                    return Some(t.clone());
                }
            }
        }
        if let crate::model::ToplevelExpression::Music(Music::Tempo(t)) = item {
            return Some(t.clone());
        }
    }
    None
}

/// Re-parse a serialized `\mark ...` string back into a Mark AST node.
fn parse_mark_from_label(s: &str) -> Option<crate::model::signature::Mark> {
    use crate::parser::Parser;
    let src = format!("{s}\nc4");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::Sequential(items)) = item {
            for m in items {
                if let Music::Mark(m) = m {
                    return Some(m.clone());
                }
            }
        }
        if let crate::model::ToplevelExpression::Music(Music::Mark(m)) = item {
            return Some(m.clone());
        }
    }
    None
}

/// Re-parse a serialized `\textMark ...` string back into a TextMark AST node.
fn parse_textmark_from_label(s: &str) -> Option<crate::model::signature::TextMark> {
    use crate::parser::Parser;
    let src = format!("{s}\nc4");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::Sequential(items)) = item {
            for m in items {
                if let Music::TextMark(tm) = m {
                    return Some(tm.clone());
                }
            }
        }
        if let crate::model::ToplevelExpression::Music(Music::TextMark(tm)) = item {
            return Some(tm.clone());
        }
    }
    None
}

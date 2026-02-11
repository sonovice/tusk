//! Clef / key / time event sequence extraction and injection for LilyPond export.

use tusk_model::elements::ScoreDefChild;
use tusk_model::elements::StaffGrpChild;
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

/// Extract event sequences from all staffDefs.
pub(super) fn extract_event_sequences(
    score: &tusk_model::elements::Score,
) -> Vec<Vec<SignatureEvent>> {
    let mut result = Vec::new();
    for child in &score.children {
        if let tusk_model::elements::ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for grp_child in &grp.children {
                        if let StaffGrpChild::StaffDef(sdef) = grp_child {
                            result.push(parse_event_sequence_json(sdef));
                        }
                    }
                }
            }
        }
    }
    result
}

/// Parse the `tusk:events,{json}` segment from a staffDef label.
fn parse_event_sequence_json(staff_def: &tusk_model::elements::StaffDef) -> Vec<SignatureEvent> {
    let label = match &staff_def.labelled.label {
        Some(l) => l.as_str(),
        None => return Vec::new(),
    };

    // Find the tusk:events JSON segment
    for segment in label.split('|') {
        if let Some(json) = segment.strip_prefix("tusk:events,")
            && let Ok(seq) = serde_json::from_str::<EventSequence>(json)
        {
            return convert_event_sequence(seq);
        }
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

/// Inject signature events into layer items at the correct positions.
///
/// Events are keyed by position in the note/rest stream. We insert them
/// before the note/rest at that position. Only injected into the first layer
/// (voice 1) since clef/key/time apply to the whole staff.
pub(super) fn inject_signature_events(layers: &mut [Vec<Music>], events: &[SignatureEvent]) {
    if layers.is_empty() || events.is_empty() {
        return;
    }
    // Only inject into first layer
    let layer = &mut layers[0];

    // Build insertion map: position -> list of Music to insert (in order)
    let mut inserts: std::collections::BTreeMap<u32, Vec<Music>> =
        std::collections::BTreeMap::new();
    for ev in events {
        inserts
            .entry(ev.position)
            .or_default()
            .push(ev.music.clone());
    }

    // Rebuild layer with injected events
    let mut new_items = Vec::new();
    for (note_idx, item) in layer.drain(..).enumerate() {
        if let Some(to_insert) = inserts.remove(&(note_idx as u32)) {
            new_items.extend(to_insert);
        }
        new_items.push(item);
    }
    // Any remaining events at end of stream
    for (_pos, to_insert) in inserts {
        new_items.extend(to_insert);
    }
    *layer = new_items;
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

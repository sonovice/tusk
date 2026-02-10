//! Clef / key / time event sequence extraction and injection for LilyPond export.

use tusk_model::elements::ScoreDefChild;
use tusk_model::elements::StaffGrpChild;

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
                            result.push(parse_event_sequence_label(sdef));
                        }
                    }
                }
            }
        }
    }
    result
}

/// Parse the `lilypond:events,...` segment from a staffDef label.
fn parse_event_sequence_label(staff_def: &tusk_model::elements::StaffDef) -> Vec<SignatureEvent> {
    let label = match &staff_def.labelled.label {
        Some(l) => l.as_str(),
        None => return Vec::new(),
    };

    // Find the lilypond:events segment (label may have multiple | separated segments)
    let events_str = label
        .split('|')
        .find_map(|seg| seg.strip_prefix("lilypond:events,"));

    let events_str = match events_str {
        Some(s) => s,
        None => {
            // No event sequence -- try to reconstruct from staffDef attributes
            return reconstruct_initial_signatures(staff_def);
        }
    };

    let mut events = Vec::new();
    for entry in events_str.split(';') {
        if entry.is_empty() {
            continue;
        }
        let (type_str, pos_str) = match entry.rsplit_once('@') {
            Some(pair) => pair,
            None => continue,
        };
        let position: u32 = match pos_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Some(name) = type_str.strip_prefix("clef:") {
            events.push(SignatureEvent {
                position,
                music: Music::Clef(Clef {
                    name: name.to_string(),
                }),
            });
        } else if let Some(key_str) = type_str.strip_prefix("key:") {
            if let Some(ks) = parse_key_label(key_str) {
                events.push(SignatureEvent {
                    position,
                    music: Music::KeySignature(ks),
                });
            }
        } else if let Some(time_str) = type_str.strip_prefix("time:")
            && let Some(ts) = parse_time_label(time_str)
        {
            events.push(SignatureEvent {
                position,
                music: Music::TimeSignature(ts),
            });
        } else if type_str == "autobeamon" {
            events.push(SignatureEvent {
                position,
                music: Music::AutoBeamOn,
            });
        } else if type_str == "autobeamoff" {
            events.push(SignatureEvent {
                position,
                music: Music::AutoBeamOff,
            });
        } else if type_str == "barcheck" {
            events.push(SignatureEvent {
                position,
                music: Music::BarCheck,
            });
        } else if let Some(bar_type) = type_str.strip_prefix("barline:") {
            // Unescape pipe characters
            let bar_type = bar_type.replace("\\u007c", "|");
            events.push(SignatureEvent {
                position,
                music: Music::BarLine { bar_type },
            });
        } else if let Some(markup_str) = type_str.strip_prefix("markup:") {
            let unescaped = crate::import::signatures::unescape_label_value(markup_str);
            if let Some(m) = parse_markup_from_label(&unescaped) {
                events.push(SignatureEvent {
                    position,
                    music: Music::Markup(m),
                });
            }
        } else if let Some(markuplist_str) = type_str.strip_prefix("markuplist:") {
            let unescaped = crate::import::signatures::unescape_label_value(markuplist_str);
            if let Some(ml) = parse_markuplist_from_label(&unescaped) {
                events.push(SignatureEvent {
                    position,
                    music: Music::MarkupList(ml),
                });
            }
        }
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

/// Parse a key signature label: `STEP.ALTER.MODE`
fn parse_key_label(s: &str) -> Option<KeySignature> {
    let mut parts = s.splitn(3, '.');
    let step_str = parts.next()?;
    let alter_str = parts.next()?;
    let mode_str = parts.next()?;

    let step = step_str.chars().next()?;
    let alter: f32 = alter_str.parse().ok()?;
    let mode = Mode::from_name(mode_str)?;

    Some(KeySignature {
        pitch: Pitch {
            step,
            alter,
            octave: 0,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        },
        mode,
    })
}

/// Parse a time signature label: `N+M/D`
fn parse_time_label(s: &str) -> Option<TimeSignature> {
    let (num_str, den_str) = s.split_once('/')?;
    let numerators: Vec<u32> = num_str
        .split('+')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    let denominator: u32 = den_str.trim().parse().ok()?;
    if numerators.is_empty() {
        return None;
    }
    Some(TimeSignature {
        numerators,
        denominator,
    })
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

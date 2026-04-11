//! Measure splitting for LilyPond import.
//!
//! Splits a flat LyEvent stream into measure groups based on time signatures,
//! explicit measure markers, and accumulated note durations.

use super::events::LyEvent;
use crate::model::signature::TimeSignature;

/// A group of events belonging to a single measure.
pub(super) struct MeasureGroup {
    pub events: Vec<LyEvent>,
    /// Explicit barline type at the end of this measure, if any.
    pub end_barline: Option<String>,
}

/// Split a stream of LyEvents into measure groups.
///
/// Hybrid strategy: always tracks accumulated note durations against the
/// current time signature AND treats `BarCheck` / `BarLine` as forced measure
/// boundaries when they are encountered. Duration-based splitting remains
/// active unless explicit `BarCheck`s show that the source is already
/// authoritative about every measure boundary.
pub(super) fn split_events_into_measures(events: Vec<LyEvent>) -> Vec<MeasureGroup> {
    split_events_impl(events, false)
}

/// Like `split_events_into_measures`, but resolves implicit durations
/// (LilyPond `c4 d e f` = c4 d4 e4 f4) for correct beat accumulation.
/// Used for polyphonic voices where events may have `None` durations.
pub(super) fn split_events_into_measures_resolved(events: Vec<LyEvent>) -> Vec<MeasureGroup> {
    split_events_impl(events, true)
}

fn split_events_impl(events: Vec<LyEvent>, resolve_durations: bool) -> Vec<MeasureGroup> {
    // Pre-scan: detect if BarChecks separate multi-measure content. If so,
    // trust them as the authoritative measure boundaries and disable
    // duration-based splitting (which can create spurious splits when note
    // durations don't exactly fill the time signature). Plain BarLines
    // still force local splits, but they do not imply that every missing
    // measure boundary should be suppressed.
    let bar_checks_authoritative = has_authoritative_bar_checks(&events);

    let mut measures: Vec<MeasureGroup> = Vec::new();
    let mut current_events: Vec<LyEvent> = Vec::new();
    let mut current_barline: Option<String> = None;

    // Current time signature: default 4/4
    let mut measure_quarters = 4.0f64;
    let mut accumulated = 0.0f64;

    // Implicit duration inheritance (only when resolve_durations is true).
    let mut last_duration = crate::model::Duration { base: 4, dots: 0, multipliers: vec![] };

    // Tuplet ratio stack
    let mut tuplet_stack: Vec<(u32, u32)> = Vec::new();

    // Grace note mode: grace notes have zero duration
    let mut in_grace = false;

    // Alternative tracking: alternatives after the first don't add duration
    // because they're parallel paths (played instead of, not after, the first).
    let mut skip_alt_duration = false;
    // A measure rest finalizes its measure immediately; the exported trailing
    // `\bar "|"` should attach to that measure, not create a phantom empty one.
    let mut just_closed_measure_rest = false;

    for event in events {
        match &event {
            LyEvent::TimeSig(ts) => {
                just_closed_measure_rest = false;
                measure_quarters = time_sig_quarters(ts);
                current_events.push(event);
            }
            LyEvent::TupletStart { numerator, denominator, .. } => {
                just_closed_measure_rest = false;
                tuplet_stack.push((*numerator, *denominator));
                current_events.push(event);
            }
            LyEvent::TupletEnd => {
                just_closed_measure_rest = false;
                tuplet_stack.pop();
                // Suffix event: if measure just closed, keep with previous measure
                if accumulated < 0.001 && !measures.is_empty() && current_events.is_empty() {
                    measures.last_mut().unwrap().events.push(event);
                } else {
                    current_events.push(event);
                }
            }
            LyEvent::GraceStart(_) => {
                just_closed_measure_rest = false;
                in_grace = true;
                current_events.push(event);
            }
            LyEvent::GraceEnd => {
                just_closed_measure_rest = false;
                in_grace = false;
                current_events.push(event);
            }
            LyEvent::AlternativeStart { index } => {
                just_closed_measure_rest = false;
                // Alternatives after the first are parallel (same time slot),
                // so their notes shouldn't accumulate duration.
                skip_alt_duration = *index > 0;
                current_events.push(event);
            }
            LyEvent::BarCheck => {
                just_closed_measure_rest = false;
                // Bar check = author-confirmed measure boundary.
                // Force split regardless of accumulated duration.
                if !current_events.is_empty() {
                    // If current events are only control events (no notes/rests),
                    // merge into previous measure instead of creating a tiny one
                    // (e.g. trailing \clef after a MeasureRest)
                    let has_notes = current_events.iter().any(has_duration);
                    if !has_notes && !measures.is_empty() {
                        measures.last_mut().unwrap().events.extend(std::mem::take(&mut current_events));
                    } else {
                        measures.push(MeasureGroup {
                            events: std::mem::take(&mut current_events),
                            end_barline: current_barline.take(),
                        });
                    }
                } else if measures.is_empty() {
                    // Preserve empty first measure (e.g. { | notes... }).
                    // All voices need this — when voice splitting distributes
                    // BarChecks to all voices, secondary voices also need an
                    // empty first measure to stay aligned with the primary.
                    measures.push(MeasureGroup {
                        events: Vec::new(),
                        end_barline: current_barline.take(),
                    });
                } else if resolve_durations {
                    // Split polyphonic exporter output can have flat measures
                    // carried only by voice 0 between later secondary-voice
                    // entries. Preserve those empty resolved measures here so
                    // dormant voices do not slide earlier on re-import.
                    measures.push(MeasureGroup {
                        events: Vec::new(),
                        end_barline: current_barline.take(),
                    });
                }
                accumulated = 0.0;
            }
            LyEvent::BarLine(bar_type) => {
                current_barline = Some(bar_type.clone());
                if !current_events.is_empty() {
                    just_closed_measure_rest = false;
                    let has_notes = current_events.iter().any(has_duration);
                    if !has_notes && !measures.is_empty() {
                        measures.last_mut().unwrap().events.extend(std::mem::take(&mut current_events));
                    } else {
                        measures.push(MeasureGroup {
                            events: std::mem::take(&mut current_events),
                            end_barline: current_barline.take(),
                        });
                    }
                } else if just_closed_measure_rest {
                    if !measures.is_empty() {
                        measures.last_mut().unwrap().end_barline = current_barline.take();
                    }
                    just_closed_measure_rest = false;
                } else if resolve_durations && bar_type == "|" {
                    // Split polyphonic exporter output must preserve empty
                    // measures in dormant voices, or later voice content
                    // slides earlier on re-import.
                    measures.push(MeasureGroup {
                        events: Vec::new(),
                        end_barline: current_barline.take(),
                    });
                } else if !measures.is_empty() {
                    // Trailing barline after a bar check — attach to previous measure
                    measures.last_mut().unwrap().end_barline = current_barline.take();
                }
                accumulated = 0.0;
            }
            LyEvent::Note(n) | LyEvent::PitchedRest(n) => {
                just_closed_measure_rest = false;
                if !in_grace && !skip_alt_duration {
                    if let Some(dur) = &n.duration {
                        accumulated += event_quarters(dur, &tuplet_stack);
                        last_duration = dur.clone();
                    } else if resolve_durations {
                        accumulated += event_quarters(&last_duration, &tuplet_stack);
                    }
                }
                current_events.push(event);
                if !skip_alt_duration && !bar_checks_authoritative {
                    check_measure_boundary(&mut measures, &mut current_events, &mut current_barline, &mut accumulated, measure_quarters);
                }
            }
            LyEvent::Rest(r) => {
                just_closed_measure_rest = false;
                if !skip_alt_duration {
                    if let Some(dur) = &r.duration {
                        accumulated += event_quarters(dur, &tuplet_stack);
                        last_duration = dur.clone();
                    } else if resolve_durations {
                        accumulated += event_quarters(&last_duration, &tuplet_stack);
                    }
                }
                current_events.push(event);
                if !skip_alt_duration && !bar_checks_authoritative {
                    check_measure_boundary(&mut measures, &mut current_events, &mut current_barline, &mut accumulated, measure_quarters);
                }
            }
            LyEvent::Skip(s) => {
                if just_closed_measure_rest
                    && current_events.is_empty()
                    && is_export_measure_spacer(s)
                {
                    if let Some(last) = measures.last_mut() {
                        last.events.push(event);
                    }
                    continue;
                }
                just_closed_measure_rest = false;
                if !in_grace && !skip_alt_duration {
                    if let Some(dur) = &s.duration {
                        accumulated += event_quarters(dur, &tuplet_stack);
                        last_duration = dur.clone();
                    } else if resolve_durations {
                        accumulated += event_quarters(&last_duration, &tuplet_stack);
                    }
                }
                current_events.push(event);
                if !skip_alt_duration && !bar_checks_authoritative {
                    check_measure_boundary(&mut measures, &mut current_events, &mut current_barline, &mut accumulated, measure_quarters);
                }
            }
            LyEvent::Chord { duration, .. } => {
                just_closed_measure_rest = false;
                if !in_grace && !skip_alt_duration {
                    if let Some(dur) = duration {
                        accumulated += event_quarters(dur, &tuplet_stack);
                        last_duration = dur.clone();
                    } else if resolve_durations {
                        accumulated += event_quarters(&last_duration, &tuplet_stack);
                    }
                }
                current_events.push(event);
                if !skip_alt_duration && !bar_checks_authoritative {
                    check_measure_boundary(&mut measures, &mut current_events, &mut current_barline, &mut accumulated, measure_quarters);
                }
            }
            LyEvent::MeasureRest(_) => {
                current_events.push(event);
                if !skip_alt_duration {
                    measures.push(MeasureGroup {
                        events: std::mem::take(&mut current_events),
                        end_barline: current_barline.take(),
                    });
                    accumulated = 0.0;
                    just_closed_measure_rest = true;
                }
            }
            LyEvent::DrumEvent(dn) => {
                just_closed_measure_rest = false;
                if !in_grace && !skip_alt_duration {
                    if let Some(dur) = &dn.duration {
                        accumulated += event_quarters(dur, &tuplet_stack);
                    }
                }
                current_events.push(event);
                if !skip_alt_duration && !bar_checks_authoritative {
                    check_measure_boundary(&mut measures, &mut current_events, &mut current_barline, &mut accumulated, measure_quarters);
                }
            }
            LyEvent::DrumChordEvent(dc) => {
                just_closed_measure_rest = false;
                if !in_grace && !skip_alt_duration {
                    if let Some(dur) = &dc.duration {
                        accumulated += event_quarters(dur, &tuplet_stack);
                    }
                }
                current_events.push(event);
                if !skip_alt_duration && !bar_checks_authoritative {
                    check_measure_boundary(&mut measures, &mut current_events, &mut current_barline, &mut accumulated, measure_quarters);
                }
            }
            LyEvent::ChordName(ce) => {
                just_closed_measure_rest = false;
                if !skip_alt_duration {
                    if let Some(dur) = &ce.duration {
                        accumulated += event_quarters(dur, &tuplet_stack);
                    }
                }
                current_events.push(event);
                if !skip_alt_duration && !bar_checks_authoritative {
                    check_measure_boundary(&mut measures, &mut current_events, &mut current_barline, &mut accumulated, measure_quarters);
                }
            }
            LyEvent::FigureEvent(fe) => {
                just_closed_measure_rest = false;
                if !skip_alt_duration {
                    if let Some(dur) = &fe.duration {
                        accumulated += event_quarters(dur, &tuplet_stack);
                    }
                }
                current_events.push(event);
                if !skip_alt_duration && !bar_checks_authoritative {
                    check_measure_boundary(&mut measures, &mut current_events, &mut current_barline, &mut accumulated, measure_quarters);
                }
            }
            // Suffix events
            LyEvent::RepeatEnd | LyEvent::AlternativeEnd => {
                // Clear alternative skip on AlternativeEnd
                if matches!(&event, LyEvent::AlternativeEnd) {
                    skip_alt_duration = false;
                }
                if accumulated < 0.001 && !measures.is_empty() && current_events.is_empty() {
                    measures.last_mut().unwrap().events.push(event);
                } else {
                    current_events.push(event);
                }
            }
            _ => {
                current_events.push(event);
            }
        }
    }

    // Flush remaining events
    if !current_events.is_empty() {
        // If remaining events have no note-producing content, merge them
        // into the last measure (trailing control events like \clef).
        let has_notes = current_events.iter().any(has_duration);
        if !has_notes && !measures.is_empty() {
            let last = measures.last_mut().unwrap();
            last.events.extend(current_events);
            if current_barline.is_some() {
                last.end_barline = current_barline;
            }
        } else {
            measures.push(MeasureGroup {
                events: current_events,
                end_barline: current_barline,
            });
        }
    }

    if measures.is_empty() {
        measures.push(MeasureGroup {
            events: Vec::new(),
            end_barline: None,
        });
    }

    measures
}

fn is_export_measure_spacer(skip: &crate::model::note::SkipEvent) -> bool {
    skip.post_events.is_empty()
        && skip.duration.as_ref().is_some_and(|dur| {
            dur.dots == 0
                && dur.multipliers.len() == 1
                && dur.multipliers[0].1 == 1
                && dur.multipliers[0].0 >= 2
        })
}

/// Does this event produce note duration (notes, rests, chords)?
fn has_duration(event: &LyEvent) -> bool {
    matches!(
        event,
        LyEvent::Note(_)
            | LyEvent::PitchedRest(_)
            | LyEvent::Rest(_)
            | LyEvent::Skip(_)
            | LyEvent::Chord { .. }
            | LyEvent::MeasureRest(_)
            | LyEvent::DrumEvent(_)
            | LyEvent::DrumChordEvent(_)
            | LyEvent::ChordName(_)
            | LyEvent::FigureEvent(_)
    )
}

/// Check if accumulated duration >= measure length; if so, close the measure.
fn check_measure_boundary(
    measures: &mut Vec<MeasureGroup>,
    current_events: &mut Vec<LyEvent>,
    current_barline: &mut Option<String>,
    accumulated: &mut f64,
    measure_quarters: f64,
) {
    // Use small epsilon for float comparison
    if *accumulated >= measure_quarters - 0.001 {
        measures.push(MeasureGroup {
            events: std::mem::take(current_events),
            end_barline: current_barline.take(),
        });
        // Keep any overshoot (for pickup measures or intentional overflow)
        *accumulated -= measure_quarters;
        if *accumulated < 0.001 {
            *accumulated = 0.0;
        }
    }
}

/// Calculate quarter-note duration of an event, accounting for tuplet scaling.
fn event_quarters(dur: &crate::model::Duration, tuplet_stack: &[(u32, u32)]) -> f64 {
    let mut q = dur.quarters();
    // Apply tuplet scaling: \tuplet 3/2 means 3 notes in time of 2
    // so each note's actual duration = written_dur * denominator / numerator
    for &(num, den) in tuplet_stack {
        q *= den as f64 / num as f64;
    }
    q
}

/// Detect if explicit exported measure boundaries separate multi-measure content.
///
/// Returns true if at least one `BarCheck` or plain `BarLine("|")` has
/// note-producing content both before and after it — meaning the source
/// already carries explicit per-measure boundaries. When true, duration-based
/// splitting is disabled to avoid spurious splits from accumulation
/// mismatches in exported or polyphonic material.
///
/// Returns false for trailing boundaries (notes before but nothing after),
/// which appear in single-measure files and shouldn't disable duration splitting.
fn has_authoritative_bar_checks(events: &[LyEvent]) -> bool {
    let mut seen_note = false;
    let mut seen_bar_after_note = false;

    for event in events {
        match event {
            LyEvent::BarCheck => {
                if seen_note {
                    seen_bar_after_note = true;
                }
            }
            LyEvent::BarLine(bar_type) if bar_type == "|" => {
                if seen_note {
                    seen_bar_after_note = true;
                }
            }
            _ if has_duration(event) => {
                if seen_bar_after_note {
                    return true;
                }
                seen_note = true;
            }
            _ => {}
        }
    }
    false
}

/// Calculate the length of a measure in quarter notes from a time signature.
fn time_sig_quarters(ts: &TimeSignature) -> f64 {
    let num_sum: u32 = ts.numerators.iter().sum();
    num_sum as f64 * 4.0 / ts.denominator as f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Duration, NoteEvent, RestEvent, Pitch};

    fn make_note(base: u32) -> LyEvent {
        LyEvent::Note(NoteEvent {
            pitch: Pitch { step: 'c', alter: 0.0, octave: 1, force_accidental: false, cautionary: false, octave_check: None },
            duration: Some(Duration { base, dots: 0, multipliers: vec![] }),
            pitched_rest: false,
            post_events: vec![],
        })
    }

    #[test]
    fn four_quarters_makes_one_measure() {
        let events = vec![make_note(4), make_note(4), make_note(4), make_note(4)];
        let measures = split_events_into_measures(events);
        assert_eq!(measures.len(), 1);
        assert_eq!(measures[0].events.len(), 4);
    }

    #[test]
    fn eight_quarters_makes_two_measures() {
        let events = vec![
            make_note(4), make_note(4), make_note(4), make_note(4),
            make_note(4), make_note(4), make_note(4), make_note(4),
        ];
        let measures = split_events_into_measures(events);
        assert_eq!(measures.len(), 2);
        assert_eq!(measures[0].events.len(), 4);
        assert_eq!(measures[1].events.len(), 4);
    }

    #[test]
    fn bar_check_splits() {
        let events = vec![
            make_note(2), make_note(2),
            LyEvent::BarCheck,
            make_note(4), make_note(4), make_note(4), make_note(4),
        ];
        let measures = split_events_into_measures(events);
        assert_eq!(measures.len(), 2);
    }

    #[test]
    fn time_sig_change() {
        let events = vec![
            LyEvent::TimeSig(TimeSignature { numerators: vec![3], denominator: 4 }),
            make_note(4), make_note(4), make_note(4),
            make_note(4), make_note(4), make_note(4),
        ];
        let measures = split_events_into_measures(events);
        assert_eq!(measures.len(), 2);
        // First measure has timesig + 3 quarter notes
        assert_eq!(measures[0].events.len(), 4);
    }

    #[test]
    fn barline_between_duration_regions() {
        // 4 quarters (1 measure) + \bar "||" + 4 quarters (1 measure) = 2 measures
        // Even without bar checks, the barline forces a split
        let events = vec![
            make_note(4), make_note(4), make_note(4), make_note(4),
            LyEvent::BarLine("||".to_string()),
            make_note(4), make_note(4), make_note(4), make_note(4),
        ];
        let measures = split_events_into_measures(events);
        assert_eq!(measures.len(), 2);
        assert_eq!(measures[0].end_barline, Some("||".to_string()));
    }

    #[test]
    fn duration_splits_between_barlines() {
        // 8 quarters + \bar "||" + 4 quarters = 3 measures
        // Duration splits the first 8 into 2 measures, barline starts measure 3
        let events = vec![
            make_note(4), make_note(4), make_note(4), make_note(4),
            make_note(4), make_note(4), make_note(4), make_note(4),
            LyEvent::BarLine("||".to_string()),
            make_note(4), make_note(4), make_note(4), make_note(4),
        ];
        let measures = split_events_into_measures(events);
        assert_eq!(measures.len(), 3);
    }

    #[test]
    fn rest_with_multiplier() {
        // r32*16 = 2 quarters; four of them make 8 quarters = 2 measures
        let events = vec![
            LyEvent::Rest(RestEvent {
                duration: Some(Duration { base: 32, dots: 0, multipliers: vec![(16, 1)] }),
                post_events: vec![],
            }),
            LyEvent::Rest(RestEvent {
                duration: Some(Duration { base: 32, dots: 0, multipliers: vec![(16, 1)] }),
                post_events: vec![],
            }),
            LyEvent::Rest(RestEvent {
                duration: Some(Duration { base: 32, dots: 0, multipliers: vec![(16, 1)] }),
                post_events: vec![],
            }),
            LyEvent::Rest(RestEvent {
                duration: Some(Duration { base: 32, dots: 0, multipliers: vec![(16, 1)] }),
                post_events: vec![],
            }),
        ];
        let measures = split_events_into_measures(events);
        assert_eq!(measures.len(), 2);
    }

    #[test]
    fn resolved_bar_checks_preserve_empty_dormant_voice_measures() {
        let events = vec![
            make_note(4),
            LyEvent::BarCheck,
            LyEvent::BarCheck,
            LyEvent::BarCheck,
            make_note(4),
            LyEvent::BarCheck,
        ];
        let measures = split_events_into_measures_resolved(events);
        assert_eq!(measures.len(), 4);
        assert_eq!(measures[0].events.len(), 1);
        assert!(measures[1].events.is_empty());
        assert!(measures[2].events.is_empty());
        assert_eq!(measures[3].events.len(), 1);
    }
}

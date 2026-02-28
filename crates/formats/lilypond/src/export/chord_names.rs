//! Chord-mode / Harm handling for LilyPond export.
//!
//! Extracts chord-mode events from MEI `<harm>` control events and
//! reconstructs `\new ChordNames \chordmode { ... }` context structure.
//!
//! Supports two sources:
//! 1. LilyPond roundtrip: `ext_store.chord_mode_info(id)` → re-parse serialized string
//! 2. MusicXML-originated: `ext_store.harmony(id)` → convert HarmonyData to ChordModeEvent

use tusk_model::elements::{MeasureChild, ScoreChild, ScoreDefChild, SectionChild, StaffChild, LayerChild};
use tusk_model::extensions::ExtensionStore;
use tusk_model::musicxml_ext::{HarmonyData, DegreeData, BassData};

use crate::model::duration::Duration;
use crate::model::note::{
    ChordModeEvent, ChordModifier, ChordQualityItem, ChordStep, StepAlteration,
};
use crate::model::pitch::Pitch;
use crate::model::Music;

/// Metadata for a ChordNames context, extracted from staffGrp label.
pub(super) struct ChordNamesMeta {
    pub(super) name: Option<String>,
    pub(super) with_block_str: Option<String>,
}

/// Collect chord-mode events from Harm control events in the score via ext_store.
///
/// Walks measures in order, computing durations from tstamp gaps and time signature.
/// Inserts bar checks between measures and skips for gaps without chords.
pub(super) fn collect_chord_mode_harms(
    score: &tusk_model::elements::Score,
    ext_store: &ExtensionStore,
) -> Vec<Music> {
    let (mut meter_count, mut meter_unit) = extract_initial_time_sig(score);
    let mut events: Vec<Music> = Vec::new();
    let mut measure_idx = 0usize;

    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            collect_from_section(
                &section.children,
                ext_store,
                &mut meter_count,
                &mut meter_unit,
                &mut events,
                &mut measure_idx,
            );
        }
    }
    events
}

/// Recursively collect from section children (handles nested sections/endings).
fn collect_from_section(
    children: &[SectionChild],
    ext_store: &ExtensionStore,
    meter_count: &mut u32,
    meter_unit: &mut u32,
    events: &mut Vec<Music>,
    measure_idx: &mut usize,
) {
    for sc in children {
        match sc {
            SectionChild::Measure(measure) => {
                // Check for time sig changes in this measure's layers
                if let Some((mc, mu)) = extract_measure_time_sig(measure) {
                    *meter_count = mc;
                    *meter_unit = mu;
                }

                // Collect all harms with tstamps
                // `from_lilypond` = true means durations already embedded, don't override
                let mut harms: Vec<(f64, Option<crate::model::note::ChordModeEvent>, bool)> = Vec::new();
                for mc in &measure.children {
                    if let MeasureChild::Harm(harm) = mc {
                        let tstamp = harm
                            .harm_log
                            .tstamp
                            .as_ref()
                            .map(|t| t.0)
                            .unwrap_or(1.0);
                        let id = harm.common.xml_id.as_deref();
                        // Try LilyPond roundtrip path first
                        let ly_ce = id.and_then(|id| {
                            let info = ext_store.chord_mode_info(id)?;
                            parse_chord_mode_event_str(&info.serialized)
                        });
                        if let Some(ce) = ly_ce {
                            harms.push((tstamp, Some(ce), true));
                        } else {
                            // Fallback: MusicXML HarmonyData
                            let ce = id
                                .and_then(|id| ext_store.harmony(id))
                                .and_then(harmony_data_to_chord_mode_event);
                            harms.push((tstamp, ce, false));
                        }
                    }
                }

                // Check if any harms come from LilyPond roundtrip (durations pre-embedded)
                let all_from_lilypond = !harms.is_empty() && harms.iter().all(|h| h.2);

                if all_from_lilypond {
                    // LilyPond roundtrip: emit events as-is, no duration computation
                    for (_tstamp, ce, _) in &harms {
                        if let Some(ce) = ce {
                            events.push(Music::ChordModeEntry(ce.clone()));
                        }
                    }
                } else if harms.is_empty() {
                    // No harms — emit skip if we've already started
                    if !events.is_empty() {
                        if *measure_idx > 0 {
                            events.push(Music::BarCheck);
                        }
                        let dur = beats_to_duration(*meter_count as f64, *meter_unit);
                        events.push(Music::Skip(crate::model::note::SkipEvent {
                            duration: Some(dur),
                            post_events: Vec::new(),
                        }));
                    }
                } else {
                    // MusicXML path: compute durations from tstamp gaps
                    harms.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

                    if *measure_idx > 0 && !events.is_empty() {
                        events.push(Music::BarCheck);
                    }

                    let total_beats = *meter_count as f64;
                    let tstamps: Vec<f64> = harms.iter().map(|h| h.0).collect();

                    // Gap before first harm
                    if tstamps[0] > 1.0 + 0.01 {
                        let gap = tstamps[0] - 1.0;
                        let dur = beats_to_duration(gap, *meter_unit);
                        events.push(Music::Skip(crate::model::note::SkipEvent {
                            duration: Some(dur),
                            post_events: Vec::new(),
                        }));
                    }

                    for (i, (_, ce, _)) in harms.iter().enumerate() {
                        let dur_beats = if i + 1 < tstamps.len() {
                            tstamps[i + 1] - tstamps[i]
                        } else {
                            (total_beats + 1.0) - tstamps[i]
                        };
                        let dur = beats_to_duration(dur_beats.max(0.0), *meter_unit);

                        if let Some(mut ce) = ce.clone() {
                            ce.duration = Some(dur);
                            events.push(Music::ChordModeEntry(ce));
                        } else {
                            events.push(Music::Skip(crate::model::note::SkipEvent {
                                duration: Some(dur),
                                post_events: Vec::new(),
                            }));
                        }
                    }
                }

                *measure_idx += 1;
            }
            SectionChild::Ending(ending) => {
                // Recurse into ending measures
                for ec in &ending.children {
                    if let tusk_model::elements::EndingChild::Measure(m) = ec {
                        // Wrap in a slice-compatible call via temporary SectionChild
                        let temp = [SectionChild::Measure(m.clone())];
                        collect_from_section(&temp, ext_store, meter_count, meter_unit, events, measure_idx);
                    }
                }
            }
            SectionChild::Section(nested) => {
                collect_from_section(&nested.children, ext_store, meter_count, meter_unit, events, measure_idx);
            }
            _ => {}
        }
    }
}


/// Convert a beat count to a LilyPond Duration.
///
/// `num_beats` is the number of beats; `beat_unit` is the denominator (4=quarter, 8=eighth).
fn beats_to_duration(num_beats: f64, beat_unit: u32) -> Duration {
    let whole_fraction = num_beats / beat_unit as f64;

    // Try standard durations with 0-2 dots
    for base in [1u32, 2, 4, 8, 16, 32] {
        let base_val = 1.0 / base as f64;
        if (whole_fraction - base_val).abs() < 0.001 {
            return Duration { base, dots: 0, multipliers: Vec::new() };
        }
        let dotted = base_val * 1.5;
        if (whole_fraction - dotted).abs() < 0.001 {
            return Duration { base, dots: 1, multipliers: Vec::new() };
        }
        let double_dot = base_val * 1.75;
        if (whole_fraction - double_dot).abs() < 0.001 {
            return Duration { base, dots: 2, multipliers: Vec::new() };
        }
    }

    // Fallback: beat_unit with rational multiplier
    // Convert num_beats to a fraction: try common denominators
    let (num, den) = approximate_fraction(num_beats);
    if num > 0 && den > 0 {
        Duration {
            base: beat_unit,
            dots: 0,
            multipliers: vec![(num, den)],
        }
    } else {
        Duration { base: beat_unit, dots: 0, multipliers: Vec::new() }
    }
}

/// Approximate a float as a simple fraction (numerator, denominator).
fn approximate_fraction(val: f64) -> (u32, u32) {
    // Try denominators 1..8 to find a clean fraction
    for den in 1u32..=8 {
        let num_f = val * den as f64;
        let num_r = num_f.round();
        if (num_f - num_r).abs() < 0.01 && num_r > 0.0 {
            let num = num_r as u32;
            // Simplify
            let g = gcd(num, den);
            return (num / g, den / g);
        }
    }
    // Last resort: round to integer
    let r = val.round().max(1.0) as u32;
    (r, 1)
}

/// Greatest common divisor.
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// Extract initial time signature from scoreDef/staffDef.
fn extract_initial_time_sig(score: &tusk_model::elements::Score) -> (u32, u32) {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            // Check scoreDef itself
            if let Some(mc) = &score_def.score_def_log.meter_count {
                let count: u32 = mc.split('+').filter_map(|s| s.trim().parse::<u32>().ok()).sum();
                let unit = score_def.score_def_log.meter_unit
                    .as_ref()
                    .and_then(|u| u.parse().ok())
                    .unwrap_or(4);
                if count > 0 {
                    return (count, unit);
                }
            }
            // Check first staffDef
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    if let Some((c, u)) = extract_staffgrp_time_sig(grp) {
                        return (c, u);
                    }
                }
            }
        }
    }
    (4, 4) // default
}

/// Recursively check staffGrp for time sig on any staffDef.
fn extract_staffgrp_time_sig(grp: &tusk_model::elements::StaffGrp) -> Option<(u32, u32)> {
    for child in &grp.children {
        match child {
            tusk_model::elements::StaffGrpChild::StaffDef(sd) => {
                if let Some(mc) = &sd.staff_def_log.meter_count {
                    let count: u32 = mc.split('+').filter_map(|s| s.trim().parse::<u32>().ok()).sum();
                    let unit = sd.staff_def_log.meter_unit
                        .as_ref()
                        .and_then(|u| u.parse().ok())
                        .unwrap_or(4);
                    if count > 0 {
                        return Some((count, unit));
                    }
                }
            }
            tusk_model::elements::StaffGrpChild::StaffGrp(nested) => {
                if let Some(ts) = extract_staffgrp_time_sig(nested) {
                    return Some(ts);
                }
            }
            _ => {}
        }
    }
    None
}

/// Check if a measure contains an inline time sig change.
fn extract_measure_time_sig(measure: &tusk_model::elements::Measure) -> Option<(u32, u32)> {
    for mc in &measure.children {
        if let MeasureChild::Staff(staff) = mc {
            for sc in &staff.children {
                let StaffChild::Layer(layer) = sc;
                for lc in &layer.children {
                    if let LayerChild::MeterSig(ms) = lc {
                        let count: u32 = ms.meter_sig_log.count
                            .as_ref()?
                            .split('+')
                            .filter_map(|s| s.trim().parse::<u32>().ok())
                            .sum();
                        let unit = ms.meter_sig_log.unit
                            .as_ref()
                            .and_then(|u| u.parse().ok())
                            .unwrap_or(4);
                        if count > 0 {
                            return Some((count, unit));
                        }
                    }
                }
            }
            // Only check first staff
            return None;
        }
    }
    None
}

// ============================================================================
// HarmonyData → ChordModeEvent conversion
// ============================================================================

/// Convert HarmonyData to a ChordModeEvent (for MusicXML-originated harmony).
fn harmony_data_to_chord_mode_event(data: &HarmonyData) -> Option<ChordModeEvent> {
    let chord = data.chords.first()?;
    if chord.root_type != "root" {
        // Only support standard root-based chords for now
        return None;
    }
    let root = step_alter_to_pitch(chord.root_step.as_deref()?, chord.root_alter)?;

    let (mut quality, mut removals) = kind_to_quality(&chord.kind.value);

    // Apply degree modifications
    for deg in &chord.degrees {
        if let Some((item, is_removal)) = convert_degree(deg) {
            if is_removal {
                if let ChordQualityItem::Step(s) = item {
                    removals.push(s);
                }
            } else {
                quality.push(item);
            }
        }
    }

    let inversion = chord.bass.as_ref().and_then(bass_to_pitch);

    Some(ChordModeEvent {
        root,
        duration: None, // filled in by caller
        quality,
        removals,
        inversion,
        bass: None,
        post_events: Vec::new(),
    })
}

/// Convert step letter + alter to LilyPond Pitch.
fn step_alter_to_pitch(step: &str, alter: Option<f64>) -> Option<Pitch> {
    let step_char = step.chars().next()?.to_ascii_lowercase();
    if !('a'..='g').contains(&step_char) {
        return None;
    }
    Some(Pitch {
        step: step_char,
        alter: alter.unwrap_or(0.0) as f32,
        octave: 0,
        force_accidental: false,
        cautionary: false,
        octave_check: None,
    })
}

/// Convert BassData to a Pitch for chord inversion.
fn bass_to_pitch(bass: &BassData) -> Option<Pitch> {
    step_alter_to_pitch(&bass.step, bass.alter)
}

/// Map MusicXML kind value to LilyPond chord quality items.
fn kind_to_quality(kind_value: &str) -> (Vec<ChordQualityItem>, Vec<ChordStep>) {
    use ChordModifier::*;
    use ChordQualityItem::*;

    let q = |items: &[(bool, u8, StepAlteration)]| -> Vec<ChordQualityItem> {
        items
            .iter()
            .map(|(is_mod, val, alt)| {
                if *is_mod {
                    // val encodes modifier: 0=minor, 1=aug, 2=dim, 3=maj, 4=sus
                    Modifier(match val {
                        0 => Minor,
                        1 => Augmented,
                        2 => Diminished,
                        3 => Major,
                        4 => Suspended,
                        _ => Minor,
                    })
                } else {
                    Step(ChordStep { number: *val, alteration: *alt })
                }
            })
            .collect()
    };

    let n = StepAlteration::Natural;
    let s = StepAlteration::Sharp;
    let f = StepAlteration::Flat;

    let quality = match kind_value {
        "major" => vec![],
        "minor" => q(&[(true, 0, n)]),
        "augmented" => q(&[(true, 1, n)]),
        "diminished" => q(&[(true, 2, n)]),
        "dominant" => q(&[(false, 7, n)]),
        "major-seventh" => q(&[(true, 3, n), (false, 7, n)]),
        "minor-seventh" => q(&[(true, 0, n), (false, 7, n)]),
        "diminished-seventh" => q(&[(true, 2, n), (false, 7, n)]),
        "augmented-seventh" => q(&[(true, 1, n), (false, 7, n)]),
        "half-diminished" => q(&[(true, 0, n), (false, 7, n), (false, 5, f)]),
        "major-minor" => q(&[(true, 0, n), (false, 7, s)]),
        "major-sixth" => q(&[(false, 6, n)]),
        "minor-sixth" => q(&[(true, 0, n), (false, 6, n)]),
        "dominant-ninth" => q(&[(false, 9, n)]),
        "major-ninth" => q(&[(true, 3, n), (false, 9, n)]),
        "minor-ninth" => q(&[(true, 0, n), (false, 9, n)]),
        "dominant-11th" => q(&[(false, 11, n)]),
        "major-11th" => q(&[(true, 3, n), (false, 11, n)]),
        "minor-11th" => q(&[(true, 0, n), (false, 11, n)]),
        "dominant-13th" => q(&[(false, 13, n)]),
        "major-13th" => q(&[(true, 3, n), (false, 13, n)]),
        "minor-13th" => q(&[(true, 0, n), (false, 13, n)]),
        "suspended-second" => q(&[(true, 4, n), (false, 2, n)]),
        "suspended-fourth" => q(&[(true, 4, n), (false, 4, n)]),
        "power" => q(&[(false, 1, n), (false, 5, n)]),
        "none" | "other" | _ => vec![],
    };
    (quality, Vec::new())
}

/// Convert a MusicXML degree modification to a chord quality/removal item.
fn convert_degree(deg: &DegreeData) -> Option<(ChordQualityItem, bool)> {
    let alt = if deg.alter > 0.0 {
        StepAlteration::Sharp
    } else if deg.alter < 0.0 {
        StepAlteration::Flat
    } else {
        StepAlteration::Natural
    };
    let step = ChordStep {
        number: deg.value as u8,
        alteration: alt,
    };
    match deg.degree_type.as_str() {
        "add" | "alter" => Some((ChordQualityItem::Step(step), false)),
        "subtract" => Some((ChordQualityItem::Step(step), true)),
        _ => None,
    }
}

// ============================================================================
// LilyPond roundtrip path (unchanged)
// ============================================================================

/// Parse a ChordModeEvent from a Harm element via ext_store (LilyPond roundtrip).
#[allow(dead_code)]
fn parse_chord_mode_from_ext(
    harm: &tusk_model::elements::Harm,
    ext_store: &ExtensionStore,
) -> Option<crate::model::note::ChordModeEvent> {
    let id = harm.common.xml_id.as_deref()?;
    let info = ext_store.chord_mode_info(id)?;
    parse_chord_mode_event_str(&info.serialized)
}

/// Parse a chord-mode event string back into a ChordModeEvent.
///
/// Re-parses through the LilyPond parser by wrapping in `\chordmode { ... }`.
fn parse_chord_mode_event_str(s: &str) -> Option<crate::model::note::ChordModeEvent> {
    use crate::parser::Parser;
    let src = format!("\\chordmode {{ {s} }}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::ChordMode { body }) = item {
            if let Music::Sequential(items) = body.as_ref() {
                for m in items {
                    if let Music::ChordModeEntry(ce) = m {
                        return Some(ce.clone());
                    }
                }
            }
            if let Music::ChordModeEntry(ce) = body.as_ref() {
                return Some(ce.clone());
            }
        }
    }
    None
}

/// Extract ChordNames context metadata from the staffGrp via ext_store.
///
/// The import stores chord-names context under key `"{grp_id}-chordnames"`.
pub(super) fn extract_chord_names_meta(
    score: &tusk_model::elements::Score,
    ext_store: &ExtensionStore,
) -> Option<ChordNamesMeta> {
    for child in &score.children {
        if let ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child
                    && let Some(id) = grp.common.xml_id.as_deref() {
                        let cn_key = format!("{id}-chordnames");
                        if let Some(ctx) = ext_store.staff_context(&cn_key) {
                            return Some(ChordNamesMeta {
                                name: ctx.name.clone(),
                                with_block_str: ctx.with_block.clone(),
                            });
                        }
                    }
            }
        }
    }
    None
}

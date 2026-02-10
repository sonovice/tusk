//! Pitch context (relative / transpose) extraction and application for LilyPond export.

use tusk_model::elements::{ScoreDefChild, StaffGrpChild};

use crate::model::pitch::Pitch;
use crate::model::{Music, NoteEvent};

use super::build_layers_music;

/// Parsed pitch context for a staff.
pub(super) enum PitchCtx {
    /// `\relative [pitch] { ... }` -- reference pitch in marks format.
    Relative {
        step: char,
        alter: f32,
        octave: i8,
        has_pitch: bool,
    },
    /// `\transpose from to { ... }`.
    Transpose { from: Pitch, to: Pitch },
}

/// Extract pitch context labels from all staffDefs.
pub(super) fn extract_pitch_contexts(score: &tusk_model::elements::Score) -> Vec<Option<PitchCtx>> {
    let mut result = Vec::new();
    for child in &score.children {
        if let tusk_model::elements::ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for grp_child in &grp.children {
                        if let StaffGrpChild::StaffDef(sdef) = grp_child {
                            result.push(parse_pitch_context_label(sdef));
                        }
                    }
                }
            }
        }
    }
    result
}

/// Parse the `lilypond:relative,...` or `lilypond:transpose,...` segment from a staffDef label.
fn parse_pitch_context_label(staff_def: &tusk_model::elements::StaffDef) -> Option<PitchCtx> {
    let label = staff_def.labelled.label.as_deref()?;

    for segment in label.split('|') {
        if segment == "lilypond:relative" {
            // No reference pitch
            return Some(PitchCtx::Relative {
                step: 'f',
                alter: 0.0,
                octave: 0,
                has_pitch: false,
            });
        }
        if let Some(rest) = segment.strip_prefix("lilypond:relative,") {
            let parts: Vec<&str> = rest.splitn(3, '.').collect();
            if parts.len() == 3 {
                let step = parts[0].chars().next().unwrap_or('c');
                let alter: f32 = parts[1].parse().unwrap_or(0.0);
                let octave: i8 = parts[2].parse().unwrap_or(0);
                return Some(PitchCtx::Relative {
                    step,
                    alter,
                    octave,
                    has_pitch: true,
                });
            }
        }
        if let Some(rest) = segment.strip_prefix("lilypond:transpose,") {
            let pitches: Vec<&str> = rest.splitn(2, ',').collect();
            if pitches.len() == 2 {
                let from = parse_pitch_label(pitches[0]);
                let to = parse_pitch_label(pitches[1]);
                if let (Some(f), Some(t)) = (from, to) {
                    return Some(PitchCtx::Transpose { from: f, to: t });
                }
            }
        }
    }
    None
}

/// Parse a pitch label `STEP.ALTER.OCT` into a Pitch.
fn parse_pitch_label(s: &str) -> Option<Pitch> {
    let parts: Vec<&str> = s.splitn(3, '.').collect();
    if parts.len() == 3 {
        let step = parts[0].chars().next()?;
        let alter: f32 = parts[1].parse().ok()?;
        let octave: i8 = parts[2].parse().ok()?;
        Some(Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        })
    } else {
        None
    }
}

/// Apply pitch context wrappers to each staff's music.
///
/// For relative mode: convert absolute pitches to relative and wrap in `\relative`.
/// For transpose: un-transpose pitches and wrap in `\transpose`.
pub(super) fn apply_pitch_contexts(
    staff_music: &mut [Vec<Vec<Music>>],
    pitch_contexts: &[Option<PitchCtx>],
) {
    for (staff_idx, layers) in staff_music.iter_mut().enumerate() {
        if let Some(Some(ctx)) = pitch_contexts.get(staff_idx) {
            match ctx {
                PitchCtx::Relative {
                    step,
                    alter,
                    octave,
                    has_pitch,
                } => {
                    // Convert absolute pitches to relative in all layers
                    for layer_items in layers.iter_mut() {
                        convert_to_relative(layer_items, *step, *octave);
                    }

                    // Build the music to wrap -- we'll do this after layers
                    // are processed into final music, but we need to mark
                    // for wrapping. Store as a sentinel at the beginning.
                    // Actually, it's easier to wrap the flattened music.
                    // Let's take a different approach: collect all layer items,
                    // build the music, wrap, then put back.
                    let all_items: Vec<Vec<Music>> = std::mem::take(layers);
                    let inner = build_layers_music(all_items);

                    let ref_pitch = if *has_pitch {
                        Some(Box::new(Music::Note(NoteEvent {
                            pitch: Pitch {
                                step: *step,
                                alter: *alter,
                                octave: *octave,
                                force_accidental: false,
                                cautionary: false,
                                octave_check: None,
                            },
                            duration: None,
                            pitched_rest: false,
                            post_events: vec![],
                        })))
                    } else {
                        None
                    };

                    let wrapped = Music::Relative {
                        pitch: ref_pitch,
                        body: Box::new(inner),
                    };

                    // Put back as a single layer with the wrapped music
                    layers.push(vec![wrapped]);
                }
                PitchCtx::Transpose { from, to } => {
                    // Un-transpose pitches in all layers
                    for layer_items in layers.iter_mut() {
                        untranspose_items(layer_items, from, to);
                    }

                    let all_items: Vec<Vec<Music>> = std::mem::take(layers);
                    let inner = build_layers_music(all_items);

                    let wrapped = Music::Transpose {
                        from: Box::new(Music::Note(NoteEvent {
                            pitch: from.clone(),
                            duration: None,
                            pitched_rest: false,
                            post_events: vec![],
                        })),
                        to: Box::new(Music::Note(NoteEvent {
                            pitch: to.clone(),
                            duration: None,
                            pitched_rest: false,
                            post_events: vec![],
                        })),
                        body: Box::new(inner),
                    };

                    layers.push(vec![wrapped]);
                }
            }
        }
    }
}

/// Convert a list of Music items from absolute to relative octave marks.
fn convert_to_relative(items: &mut [Music], ref_step: char, ref_oct: i8) {
    let mut current_step = ref_step;
    let mut current_oct = ref_oct;

    for item in items.iter_mut() {
        match item {
            Music::Note(note) => {
                let rel_marks = note.pitch.to_relative_marks(current_step, current_oct);
                current_step = note.pitch.step;
                current_oct = note.pitch.octave;
                note.pitch.octave = rel_marks;
            }
            Music::Chord(chord) => {
                // In relative mode, the first pitch in a chord is relative to
                // the previous note; subsequent pitches are relative to the first.
                // Capture first pitch's absolute position before mutating.
                let first_step = chord.pitches[0].step;
                let first_oct = chord.pitches[0].octave;
                for (i, pitch) in chord.pitches.iter_mut().enumerate() {
                    let (rs, ro) = if i == 0 {
                        (current_step, current_oct)
                    } else {
                        (first_step, first_oct)
                    };
                    let rel_marks = pitch.to_relative_marks(rs, ro);
                    pitch.octave = rel_marks;
                }
                // Update reference for next event to the first chord pitch
                current_step = first_step;
                current_oct = first_oct;
            }
            _ => {}
        }
    }
}

/// Un-transpose pitches in a list of Music items.
fn untranspose_items(items: &mut [Music], from: &Pitch, to: &Pitch) {
    for item in items.iter_mut() {
        match item {
            Music::Note(note) => {
                note.pitch = note.pitch.untranspose(from, to);
            }
            Music::Chord(chord) => {
                for pitch in &mut chord.pitches {
                    *pitch = pitch.untranspose(from, to);
                }
            }
            _ => {}
        }
    }
}

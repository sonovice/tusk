//! Pitch context (relative / transpose) extraction and application for LilyPond export.

use tusk_model::PitchContext as ExtPitchContext;
use tusk_model::elements::{ScoreDefChild, StaffGrpChild};
use tusk_model::extensions::ExtensionStore;

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
    /// `\fixed pitch { ... }` -- reference pitch in marks format.
    Fixed { step: char, alter: f32, octave: i8 },
    /// `\transpose from to { ... }`.
    Transpose { from: Pitch, to: Pitch },
}

/// Extract pitch context from all staffDefs via ext_store.
pub(super) fn extract_pitch_contexts(score: &tusk_model::elements::Score, ext_store: &ExtensionStore) -> Vec<Option<PitchCtx>> {
    let mut result = Vec::new();
    for child in &score.children {
        if let tusk_model::elements::ScoreChild::ScoreDef(score_def) = child {
            for sd_child in &score_def.children {
                if let ScoreDefChild::StaffGrp(grp) = sd_child {
                    for grp_child in &grp.children {
                        if let StaffGrpChild::StaffDef(sdef) = grp_child {
                            let ctx = sdef.basic.xml_id.as_deref().and_then(|id| {
                                let ext = ext_store.pitch_context(id)?;
                                Some(ext_pitch_context_to_pitch_ctx(ext.clone()))
                            });
                            result.push(ctx);
                        }
                    }
                }
            }
        }
    }
    result
}

/// Convert a typed ExtPitchContext to the export PitchCtx.
fn ext_pitch_context_to_pitch_ctx(ext: ExtPitchContext) -> PitchCtx {
    match ext {
        ExtPitchContext::Relative { ref_pitch: None } => PitchCtx::Relative {
            step: 'f',
            alter: 0.0,
            octave: 0,
            has_pitch: false,
        },
        ExtPitchContext::Relative { ref_pitch: Some(p) } => PitchCtx::Relative {
            step: p.step,
            alter: p.alter,
            octave: p.octave,
            has_pitch: true,
        },
        ExtPitchContext::Transpose { from, to } => PitchCtx::Transpose {
            from: Pitch {
                step: from.step,
                alter: from.alter,
                octave: from.octave,
                force_accidental: false,
                cautionary: false,
                octave_check: None,
            },
            to: Pitch {
                step: to.step,
                alter: to.alter,
                octave: to.octave,
                force_accidental: false,
                cautionary: false,
                octave_check: None,
            },
        },
        ExtPitchContext::Fixed { ref_pitch } => PitchCtx::Fixed {
            step: ref_pitch.step,
            alter: ref_pitch.alter,
            octave: ref_pitch.octave,
        },
        ExtPitchContext::Absolute => PitchCtx::Relative {
            step: 'f',
            alter: 0.0,
            octave: 0,
            has_pitch: false,
        },
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
                PitchCtx::Fixed {
                    step,
                    alter,
                    octave,
                } => {
                    // Convert absolute pitches to fixed offsets (independent per note)
                    for layer_items in layers.iter_mut() {
                        convert_to_fixed(layer_items, *octave);
                    }

                    let all_items: Vec<Vec<Music>> = std::mem::take(layers);
                    let inner = build_layers_music(all_items);

                    let ref_pitch = Box::new(Music::Note(NoteEvent {
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
                    }));

                    let wrapped = Music::Fixed {
                        pitch: ref_pitch,
                        body: Box::new(inner),
                    };

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

/// Convert a list of Music items from absolute to fixed octave offsets.
///
/// In `\fixed` mode each pitch's octave marks are independent offsets from the
/// reference octave (no sequential dependency).
fn convert_to_fixed(items: &mut [Music], ref_oct: i8) {
    for item in items.iter_mut() {
        match item {
            Music::Note(note) => {
                note.pitch.octave -= ref_oct;
            }
            Music::Chord(chord) => {
                for pitch in &mut chord.pitches {
                    pitch.octave -= ref_oct;
                }
            }
            Music::Tuplet { body, .. }
            | Music::Grace { body, .. }
            | Music::Acciaccatura { body, .. }
            | Music::Appoggiatura { body, .. } => {
                convert_to_fixed_music(body, ref_oct);
            }
            Music::AfterGrace { main, grace, .. } => {
                convert_to_fixed_music(main, ref_oct);
                convert_to_fixed_music(grace, ref_oct);
            }
            Music::Repeat {
                body, alternatives, ..
            } => {
                convert_to_fixed_music(body, ref_oct);
                if let Some(alts) = alternatives {
                    for alt in alts {
                        convert_to_fixed_music(alt, ref_oct);
                    }
                }
            }
            Music::Sequential(inner) | Music::Simultaneous(inner) => {
                convert_to_fixed(inner, ref_oct);
            }
            _ => {}
        }
    }
}

/// Convert a single Music node from absolute to fixed offsets.
fn convert_to_fixed_music(music: &mut Music, ref_oct: i8) {
    match music {
        Music::Sequential(items) | Music::Simultaneous(items) => {
            convert_to_fixed(items, ref_oct);
        }
        _ => convert_to_fixed(std::slice::from_mut(music), ref_oct),
    }
}

/// Convert a list of Music items from absolute to relative octave marks.
///
/// Returns the final reference pitch (step, octave in marks) after processing.
fn convert_to_relative(items: &mut [Music], ref_step: char, ref_oct: i8) -> (char, i8) {
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
                // In relative mode, the first pitch is relative to the previous
                // note; each subsequent pitch is relative to the preceding pitch
                // within the chord. Capture absolute values before mutating.
                let first_step = chord.pitches[0].step;
                let first_oct = chord.pitches[0].octave;
                let mut chord_ref_step = current_step;
                let mut chord_ref_oct = current_oct;
                for pitch in chord.pitches.iter_mut() {
                    let abs_step = pitch.step;
                    let abs_oct = pitch.octave;
                    let rel_marks = pitch.to_relative_marks(chord_ref_step, chord_ref_oct);
                    pitch.octave = rel_marks;
                    chord_ref_step = abs_step;
                    chord_ref_oct = abs_oct;
                }
                // Update reference for next event to the first chord pitch
                current_step = first_step;
                current_oct = first_oct;
            }
            // Recurse into nested structures
            Music::Tuplet { body, .. }
            | Music::Grace { body, .. }
            | Music::Acciaccatura { body, .. }
            | Music::Appoggiatura { body, .. } => {
                let (s, o) = convert_to_relative_music(body, current_step, current_oct);
                current_step = s;
                current_oct = o;
            }
            Music::AfterGrace { main, grace, .. } => {
                let (s, o) = convert_to_relative_music(main, current_step, current_oct);
                let (s2, o2) = convert_to_relative_music(grace, s, o);
                current_step = s2;
                current_oct = o2;
            }
            Music::Repeat {
                body, alternatives, ..
            } => {
                let (s, o) = convert_to_relative_music(body, current_step, current_oct);
                current_step = s;
                current_oct = o;
                if let Some(alts) = alternatives {
                    for alt in alts {
                        let (s2, o2) = convert_to_relative_music(alt, current_step, current_oct);
                        current_step = s2;
                        current_oct = o2;
                    }
                }
            }
            Music::Sequential(inner) | Music::Simultaneous(inner) => {
                let (s, o) = convert_to_relative(inner, current_step, current_oct);
                current_step = s;
                current_oct = o;
            }
            _ => {}
        }
    }
    (current_step, current_oct)
}

/// Convert a single Music node from absolute to relative.
fn convert_to_relative_music(music: &mut Music, ref_step: char, ref_oct: i8) -> (char, i8) {
    match music {
        Music::Sequential(items) | Music::Simultaneous(items) => {
            convert_to_relative(items, ref_step, ref_oct)
        }
        _ => convert_to_relative(std::slice::from_mut(music), ref_step, ref_oct),
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
            Music::Tuplet { body, .. }
            | Music::Grace { body, .. }
            | Music::Acciaccatura { body, .. }
            | Music::Appoggiatura { body, .. } => {
                untranspose_music(body, from, to);
            }
            Music::AfterGrace { main, grace, .. } => {
                untranspose_music(main, from, to);
                untranspose_music(grace, from, to);
            }
            Music::Repeat {
                body, alternatives, ..
            } => {
                untranspose_music(body, from, to);
                if let Some(alts) = alternatives {
                    for alt in alts {
                        untranspose_music(alt, from, to);
                    }
                }
            }
            Music::Sequential(inner) | Music::Simultaneous(inner) => {
                untranspose_items(inner, from, to);
            }
            _ => {}
        }
    }
}

/// Un-transpose a single Music node.
fn untranspose_music(music: &mut Music, from: &Pitch, to: &Pitch) {
    match music {
        Music::Sequential(items) | Music::Simultaneous(items) => {
            untranspose_items(items, from, to);
        }
        _ => untranspose_items(std::slice::from_mut(music), from, to),
    }
}

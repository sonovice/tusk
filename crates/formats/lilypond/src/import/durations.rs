//! Resolve implicit durations in the LilyPond AST.
//!
//! In LilyPond, `c4 d e f` means c4 d4 e4 f4 — bare notes inherit the
//! previous event's duration. This module fills in `None` durations with
//! explicit values so the measure splitter can correctly count beats.
//!
//! Duration state is tracked per-voice: each branch of a `Simultaneous`
//! block gets its own duration context.

use crate::model::{Duration, FunctionArg, Music};

/// Resolve implicit durations only within Simultaneous (polyphonic) blocks.
///
/// Traverses the tree looking for `<< >>` nodes. When found, resolves
/// implicit durations within each voice independently. Non-polyphonic
/// content is left unchanged to preserve existing measure-splitting behavior.
pub(super) fn resolve_in_simultaneous(music: &mut Music) {
    walk_for_simultaneous(music);
}

fn walk_for_simultaneous(music: &mut Music) {
    match music {
        Music::Simultaneous(items) => {
            // Resolve each voice's implicit durations independently
            let default = Duration { base: 4, dots: 0, multipliers: vec![] };
            for item in items.iter_mut() {
                let mut last = default.clone();
                resolve(item, &mut last);
            }
        }
        // Recurse into containers to find nested Simultaneous nodes
        Music::Sequential(items) => {
            for m in items.iter_mut() {
                walk_for_simultaneous(m);
            }
        }
        Music::Relative { body, .. }
        | Music::Fixed { body, .. }
        | Music::Transpose { body, .. }
        | Music::Grace { body }
        | Music::Acciaccatura { body }
        | Music::Appoggiatura { body }
        | Music::Once { music: body }
        | Music::ChordMode { body }
        | Music::DrumMode { body }
        | Music::LyricMode { body }
        | Music::FigureMode { body }
        | Music::ContextedMusic { music: body, .. } => walk_for_simultaneous(body),
        Music::Tuplet { body, .. } => walk_for_simultaneous(body),
        Music::Repeat { body, alternatives, .. } => {
            walk_for_simultaneous(body);
            if let Some(alts) = alternatives {
                for a in alts.iter_mut() {
                    walk_for_simultaneous(a);
                }
            }
        }
        Music::AfterGrace { main, grace, .. } => {
            walk_for_simultaneous(main);
            walk_for_simultaneous(grace);
        }
        Music::AddLyrics { music, lyrics } => {
            walk_for_simultaneous(music);
            for l in lyrics.iter_mut() {
                walk_for_simultaneous(l);
            }
        }
        Music::LyricsTo { lyrics, .. } => walk_for_simultaneous(lyrics),
        Music::MusicFunction { args, .. } | Music::PartialFunction { args, .. } => {
            for arg in args.iter_mut() {
                if let FunctionArg::Music(m) = arg {
                    walk_for_simultaneous(m);
                }
            }
        }
        _ => {}
    }
}

/// Resolve implicit durations in a music tree (in-place).
///
/// Every Note/Rest/Skip/Chord with `duration: None` gets the last
/// explicit duration (default: quarter note). Each voice in `<< >>`
/// blocks gets independent duration tracking.
pub(super) fn resolve_implicit_durations(music: &mut Music) {
    let mut last = Duration { base: 4, dots: 0, multipliers: vec![] };
    resolve(music, &mut last);
}

fn resolve(music: &mut Music, last: &mut Duration) {
    match music {
        Music::Note(n) => {
            if let Some(d) = &n.duration {
                *last = d.clone();
            } else {
                n.duration = Some(last.clone());
            }
        }
        Music::Rest(r) => {
            if let Some(d) = &r.duration {
                *last = d.clone();
            } else {
                r.duration = Some(last.clone());
            }
        }
        Music::Skip(s) => {
            if let Some(d) = &s.duration {
                *last = d.clone();
            } else {
                s.duration = Some(last.clone());
            }
        }
        Music::Chord(c) => {
            if let Some(d) = &c.duration {
                *last = d.clone();
            } else {
                c.duration = Some(last.clone());
            }
        }
        Music::ChordRepetition(cr) => {
            if let Some(d) = &cr.duration {
                *last = d.clone();
            } else {
                cr.duration = Some(last.clone());
            }
        }
        Music::MultiMeasureRest(r) => {
            if let Some(d) = &r.duration {
                *last = d.clone();
            } else {
                r.duration = Some(last.clone());
            }
        }

        // Sequential: resolve in order, sharing duration state
        Music::Sequential(items) => {
            for m in items.iter_mut() {
                resolve(m, last);
            }
        }

        // Simultaneous: each voice gets its own duration state
        Music::Simultaneous(items) => {
            let saved = last.clone();
            for m in items.iter_mut() {
                let mut voice_last = saved.clone();
                resolve(m, &mut voice_last);
            }
            // Don't update last from Simultaneous children
        }

        // Wrappers: recurse into body
        Music::Relative { body, .. }
        | Music::Fixed { body, .. }
        | Music::Transpose { body, .. }
        | Music::Grace { body }
        | Music::Acciaccatura { body }
        | Music::Appoggiatura { body }
        | Music::Once { music: body }
        | Music::ChordMode { body }
        | Music::DrumMode { body }
        | Music::LyricMode { body }
        | Music::FigureMode { body } => resolve(body, last),

        Music::Tuplet { body, .. } => resolve(body, last),

        Music::ContextedMusic { music, .. } => resolve(music, last),

        Music::Repeat { body, alternatives, .. } => {
            resolve(body, last);
            if let Some(alts) = alternatives {
                for alt in alts.iter_mut() {
                    let mut alt_last = last.clone();
                    resolve(alt, &mut alt_last);
                }
            }
        }

        Music::AfterGrace { main, grace, .. } => {
            resolve(main, last);
            resolve(grace, last);
        }

        Music::AddLyrics { music, lyrics } => {
            resolve(music, last);
            for l in lyrics.iter_mut() {
                resolve(l, last);
            }
        }

        Music::LyricsTo { lyrics, .. } => resolve(lyrics, last),

        Music::MusicFunction { args, .. } | Music::PartialFunction { args, .. } => {
            for arg in args.iter_mut() {
                if let FunctionArg::Music(m) = arg {
                    resolve(m, last);
                }
            }
        }

        // Leaf nodes: nothing to do
        _ => {}
    }
}

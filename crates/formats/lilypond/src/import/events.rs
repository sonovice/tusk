//! Event collection from LilyPond AST for import.
//!
//! Collects note/rest/skip/control events from the music tree, resolving
//! relative pitches and transpositions to absolute values.

use tusk_model::generated::data::DataGrace;
use tusk_model::ExtensionStore;

use crate::model::{self, Music, NoteEvent, PostEvent, RestEvent};

/// Internal event representation for collecting from the AST.
///
/// Events own resolved copies of notes (not references) because relative/transpose
/// resolution produces new Pitch values.
pub(super) enum LyEvent {
    Note(NoteEvent),
    Chord {
        pitches: Vec<crate::model::Pitch>,
        duration: Option<crate::model::Duration>,
        post_events: Vec<PostEvent>,
        /// True if this chord was expanded from a `q` (chord repetition).
        is_chord_repetition: bool,
    },
    Rest(RestEvent),
    PitchedRest(NoteEvent),
    MeasureRest(model::MultiMeasureRestEvent),
    Skip(model::SkipEvent),
    Clef(model::Clef),
    KeySig(model::KeySignature),
    TimeSig(model::TimeSignature),
    AutoBeamOn,
    AutoBeamOff,
    /// Marks the beginning of a tuplet group.
    TupletStart {
        numerator: u32,
        denominator: u32,
        span_duration: Option<crate::model::Duration>,
    },
    /// Marks the end of a tuplet group.
    TupletEnd,
    /// Marks the beginning of a grace note group.
    GraceStart(GraceType),
    /// Marks the end of a grace note group.
    GraceEnd,
    /// Marks the beginning of a repeat body.
    RepeatStart {
        repeat_type: model::RepeatType,
        count: u32,
        num_alternatives: u32,
    },
    /// Marks the end of a repeat body.
    RepeatEnd,
    /// Marks the beginning of an alternative ending (0-indexed).
    AlternativeStart {
        index: u32,
    },
    /// Marks the end of an alternative ending.
    AlternativeEnd,
    /// A bar check `|` (timing assertion).
    BarCheck,
    /// An explicit bar line `\bar "TYPE"`.
    BarLine(String),
    /// A serialized `\markup { ... }` expression.
    Markup(String),
    /// A serialized `\markuplist { ... }` expression.
    MarkupList(String),
    /// A serialized `\tempo ...` expression.
    Tempo(String),
    /// A serialized `\mark ...` expression.
    Mark(String),
    /// A serialized `\textMark ...` expression.
    TextMark(String),
    /// A chord-mode event (chord name symbol).
    ChordName(model::note::ChordModeEvent),
    /// A figured bass event.
    FigureEvent(model::note::FigureEvent),
    /// A drum note event.
    DrumEvent(model::note::DrumNoteEvent),
    /// A drum chord event.
    DrumChordEvent(model::note::DrumChordEvent),
    /// A serialized property operation (`\override`, `\set`, `\revert`, `\unset`, `\once`).
    PropertyOp(String),
    /// A structured music function call (`\functionName args...`).
    MusicFunction(tusk_model::FunctionCall),
    /// A Scheme expression in music position (`#expr`), serialized.
    SchemeMusic(String),
    /// A context change (`\change Staff = "name"`).
    ContextChange {
        context_type: String,
        name: String,
    },
}

/// Type of grace note construct for import/export roundtrip.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum GraceType {
    /// `\grace { ... }`
    Grace,
    /// `\acciaccatura { ... }`
    Acciaccatura,
    /// `\appoggiatura { ... }`
    Appoggiatura,
    /// `\afterGrace [fraction] main { grace }` — grace notes only (main note is separate).
    AfterGrace { fraction: Option<(u32, u32)> },
}

/// Pitch context tracking for relative/fixed mode and transposition.
#[derive(Clone)]
pub(super) struct PitchContext {
    /// If in relative mode, (ref_step, ref_oct in marks format).
    pub(super) relative: Option<(char, i8)>,
    /// If in fixed mode, the reference octave (marks format).
    /// Pitches are absolute with octave marks offset from this reference.
    pub(super) fixed: Option<i8>,
    /// Stack of transpositions to apply: (from, to) pairs.
    transpositions: Vec<(crate::model::Pitch, crate::model::Pitch)>,
    /// Last chord pitches for `q` (chord repetition) expansion.
    pub(super) last_chord_pitches: Vec<crate::model::Pitch>,
}

impl PitchContext {
    pub(super) fn new() -> Self {
        PitchContext {
            relative: None,
            fixed: None,
            transpositions: Vec::new(),
            last_chord_pitches: Vec::new(),
        }
    }

    /// Resolve a pitch through the current context (relative/fixed -> absolute, then transpose).
    pub(super) fn resolve(&mut self, pitch: &crate::model::Pitch) -> crate::model::Pitch {
        let mut resolved = if let Some((ref_step, ref_oct)) = self.relative {
            let abs = pitch.resolve_relative(ref_step, ref_oct);
            // Update reference for next note
            self.relative = Some((abs.step, abs.octave));
            abs
        } else if let Some(ref_oct) = self.fixed {
            // Fixed mode: octave marks are offsets from the reference octave.
            // Each pitch is resolved independently (no sequential dependency).
            crate::model::Pitch {
                step: pitch.step,
                alter: pitch.alter,
                octave: ref_oct + pitch.octave,
                force_accidental: pitch.force_accidental,
                cautionary: pitch.cautionary,
                octave_check: pitch.octave_check,
            }
        } else {
            pitch.clone()
        };

        // Apply transpositions (innermost first)
        for (from, to) in &self.transpositions {
            resolved = resolved.transpose(from, to);
        }

        resolved
    }
}

/// Recursively collect note/rest/skip events from LilyPond music,
/// resolving relative pitches and transpositions to absolute.
pub(super) fn collect_events(music: &Music, events: &mut Vec<LyEvent>, ctx: &mut PitchContext) {
    match music {
        Music::Note(note) => {
            let mut resolved = note.clone();
            resolved.pitch = ctx.resolve(&note.pitch);
            if note.pitched_rest {
                events.push(LyEvent::PitchedRest(resolved));
            } else {
                events.push(LyEvent::Note(resolved));
            }
        }
        Music::Chord(chord) => {
            let resolved_pitches: Vec<_> = chord.pitches.iter().map(|p| ctx.resolve(p)).collect();
            // In relative mode, the reference for the NEXT event is the FIRST
            // chord pitch (per LilyPond spec), not the last.
            if ctx.relative.is_some()
                && let Some(first) = resolved_pitches.first()
            {
                ctx.relative = Some((first.step, first.octave));
            }
            ctx.last_chord_pitches = resolved_pitches.clone();
            events.push(LyEvent::Chord {
                pitches: resolved_pitches,
                duration: chord.duration.clone(),
                post_events: chord.post_events.clone(),
                is_chord_repetition: false,
            });
        }
        Music::ChordRepetition(cr) => {
            // Expand `q` to the most recent chord's pitches
            if !ctx.last_chord_pitches.is_empty() {
                events.push(LyEvent::Chord {
                    pitches: ctx.last_chord_pitches.clone(),
                    duration: cr.duration.clone(),
                    post_events: cr.post_events.clone(),
                    is_chord_repetition: true,
                });
            }
        }
        Music::Rest(rest) => events.push(LyEvent::Rest(rest.clone())),
        Music::Skip(skip) => events.push(LyEvent::Skip(skip.clone())),
        Music::MultiMeasureRest(mrest) => events.push(LyEvent::MeasureRest(mrest.clone())),
        Music::Sequential(items) | Music::Simultaneous(items) => {
            for item in items {
                collect_events(item, events, ctx);
            }
        }
        Music::Relative { pitch, body } => {
            let mut inner_ctx = ctx.clone();
            // Set the reference pitch for relative mode
            let (ref_step, ref_oct) = if let Some(ref_pitch_music) = pitch {
                extract_pitch_from_music(ref_pitch_music)
                    .map(|p| (p.step, p.octave))
                    .unwrap_or(('f', 0)) // default: f (middle of keyboard)
            } else {
                ('f', 0) // LilyPond default: f (below middle C)
            };
            inner_ctx.relative = Some((ref_step, ref_oct));
            collect_events(body, events, &mut inner_ctx);
        }
        Music::Fixed { pitch, body } => {
            // Fixed mode: pitches have octave marks relative to the reference pitch's octave.
            // Each pitch is resolved independently (no sequential dependency like \relative).
            let mut inner_ctx = ctx.clone();
            let ref_oct = extract_pitch_from_music(pitch)
                .map(|p| p.octave)
                .unwrap_or(1); // default: c' (octave 4)
            inner_ctx.fixed = Some(ref_oct);
            collect_events(body, events, &mut inner_ctx);
        }
        Music::Tuplet {
            numerator,
            denominator,
            span_duration,
            body,
        } => {
            events.push(LyEvent::TupletStart {
                numerator: *numerator,
                denominator: *denominator,
                span_duration: span_duration.clone(),
            });
            collect_events(body, events, ctx);
            events.push(LyEvent::TupletEnd);
        }
        Music::Transpose { from, to, body } => {
            let from_pitch = extract_pitch_from_music(from);
            let to_pitch = extract_pitch_from_music(to);
            if let (Some(fp), Some(tp)) = (from_pitch, to_pitch) {
                let mut inner_ctx = ctx.clone();
                inner_ctx.transpositions.push((fp, tp));
                collect_events(body, events, &mut inner_ctx);
            } else {
                // Can't extract pitches -- collect without transposing
                collect_events(body, events, ctx);
            }
        }
        Music::ContextedMusic { music, .. } => {
            collect_events(music, events, ctx);
        }
        Music::ContextChange { context_type, name } => {
            events.push(LyEvent::ContextChange {
                context_type: context_type.clone(),
                name: name.clone(),
            });
        }
        Music::Clef(c) => events.push(LyEvent::Clef(c.clone())),
        Music::KeySignature(ks) => events.push(LyEvent::KeySig(ks.clone())),
        Music::TimeSignature(ts) => events.push(LyEvent::TimeSig(ts.clone())),
        Music::AutoBeamOn => events.push(LyEvent::AutoBeamOn),
        Music::AutoBeamOff => events.push(LyEvent::AutoBeamOff),
        Music::Grace { body } => {
            events.push(LyEvent::GraceStart(GraceType::Grace));
            collect_events(body, events, ctx);
            events.push(LyEvent::GraceEnd);
        }
        Music::Acciaccatura { body } => {
            events.push(LyEvent::GraceStart(GraceType::Acciaccatura));
            collect_events(body, events, ctx);
            events.push(LyEvent::GraceEnd);
        }
        Music::Appoggiatura { body } => {
            events.push(LyEvent::GraceStart(GraceType::Appoggiatura));
            collect_events(body, events, ctx);
            events.push(LyEvent::GraceEnd);
        }
        Music::AfterGrace {
            fraction,
            main,
            grace,
        } => {
            collect_events(main, events, ctx);
            events.push(LyEvent::GraceStart(GraceType::AfterGrace {
                fraction: *fraction,
            }));
            collect_events(grace, events, ctx);
            events.push(LyEvent::GraceEnd);
        }
        Music::Repeat {
            repeat_type,
            count,
            body,
            alternatives,
        } => {
            let num_alts = alternatives.as_ref().map_or(0, |a| a.len() as u32);
            events.push(LyEvent::RepeatStart {
                repeat_type: *repeat_type,
                count: *count,
                num_alternatives: num_alts,
            });
            collect_events(body, events, ctx);
            events.push(LyEvent::RepeatEnd);
            if let Some(alts) = alternatives {
                for (i, alt) in alts.iter().enumerate() {
                    events.push(LyEvent::AlternativeStart { index: i as u32 });
                    collect_events(alt, events, ctx);
                    events.push(LyEvent::AlternativeEnd);
                }
            }
        }
        Music::BarCheck => events.push(LyEvent::BarCheck),
        Music::BarLine { bar_type } => events.push(LyEvent::BarLine(bar_type.clone())),
        Music::LyricMode { .. } => {
            // Lyric mode content — lyrics handled via lyrics::collect_lyric_syllables
        }
        Music::AddLyrics { music, .. } => {
            // Collect only the music part; lyrics handled via lyrics::extract_addlyrics
            collect_events(music, events, ctx);
        }
        Music::LyricsTo { .. } => {
            // Lyrics handled via lyrics::extract_lyricsto in analyze_staves
        }
        Music::Lyric(_) => {
            // Lyric events handled in lyrics module
        }
        Music::Markup(m) => {
            let serialized = crate::serializer::serialize_markup(m);
            events.push(LyEvent::Markup(serialized));
        }
        Music::MarkupList(ml) => {
            let serialized = crate::serializer::serialize_markuplist(ml);
            events.push(LyEvent::MarkupList(serialized));
        }
        Music::Tempo(t) => {
            let serialized = crate::serializer::serialize_tempo(t);
            events.push(LyEvent::Tempo(serialized));
        }
        Music::Mark(m) => {
            let serialized = crate::serializer::serialize_mark(m);
            events.push(LyEvent::Mark(serialized));
        }
        Music::TextMark(tm) => {
            let serialized = crate::serializer::serialize_text_mark(tm);
            events.push(LyEvent::TextMark(serialized));
        }
        // Chord mode: recurse into body to collect chord-mode entries
        Music::ChordMode { body } => collect_events(body, events, ctx),
        Music::ChordModeEntry(ce) => {
            // Resolve root pitch through pitch context
            let mut resolved = ce.clone();
            resolved.root = ctx.resolve(&ce.root);
            if let Some(ref mut inv) = resolved.inversion {
                *inv = ctx.resolve(inv);
            }
            if let Some(ref mut bass) = resolved.bass {
                *bass = ctx.resolve(bass);
            }
            events.push(LyEvent::ChordName(resolved));
        }
        Music::DrumMode { body } => collect_events(body, events, ctx),
        Music::DrumNote(dn) => {
            events.push(LyEvent::DrumEvent(dn.clone()));
        }
        Music::DrumChord(dc) => {
            events.push(LyEvent::DrumChordEvent(dc.clone()));
        }
        Music::FigureMode { body } => collect_events(body, events, ctx),
        Music::Figure(fe) => {
            events.push(LyEvent::FigureEvent(fe.clone()));
        }
        Music::Once { music: inner } => {
            // If wrapping a property op, serialize the entire \once expression
            if matches!(
                inner.as_ref(),
                Music::Override { .. }
                    | Music::Revert { .. }
                    | Music::Set { .. }
                    | Music::Unset { .. }
            ) {
                let serialized = crate::serializer::serialize_property_op(music);
                events.push(LyEvent::PropertyOp(serialized));
            } else {
                collect_events(inner, events, ctx);
            }
        }
        Music::Override { .. } | Music::Revert { .. } | Music::Set { .. } | Music::Unset { .. } => {
            let serialized = crate::serializer::serialize_property_op(music);
            events.push(LyEvent::PropertyOp(serialized));
        }
        Music::MusicFunction { name, args } => {
            let fc = tusk_model::FunctionCall {
                name: name.clone(),
                args: args.iter().map(function_arg_to_ext_value).collect(),
                is_partial: false,
            };
            events.push(LyEvent::MusicFunction(fc));
        }
        Music::PartialFunction { name, args } => {
            let fc = tusk_model::FunctionCall {
                name: name.clone(),
                args: args.iter().map(function_arg_to_ext_value).collect(),
                is_partial: true,
            };
            events.push(LyEvent::MusicFunction(fc));
        }
        Music::SchemeMusic(expr) => {
            let serialized = crate::serializer::serialize_scheme_expr(expr);
            events.push(LyEvent::SchemeMusic(serialized));
        }
        Music::Event(_) | Music::Identifier(_) | Music::Unparsed(_) | Music::LineComment(_) => {}
    }
}

/// Convert a GraceType to GraceInfo for ext_store.
fn grace_type_to_info(gt: &GraceType) -> tusk_model::GraceInfo {
    match gt {
        GraceType::Grace => tusk_model::GraceInfo::Grace,
        GraceType::Acciaccatura => tusk_model::GraceInfo::Acciaccatura,
        GraceType::Appoggiatura => tusk_model::GraceInfo::Appoggiatura,
        GraceType::AfterGrace { fraction } => tusk_model::GraceInfo::AfterGrace {
            fraction: *fraction,
        },
    }
}

/// Map a `GraceType` to the corresponding MEI `DataGrace` value.
fn grace_type_to_data_grace(gt: &GraceType) -> DataGrace {
    match gt {
        GraceType::Appoggiatura => DataGrace::Acc,
        GraceType::Grace | GraceType::Acciaccatura | GraceType::AfterGrace { .. } => {
            DataGrace::Unacc
        }
    }
}

/// Set `@grace` on an MEI note and store grace info in ext_store.
pub(super) fn apply_grace_to_note(note: &mut tusk_model::elements::Note, gt: &GraceType, ext_store: &mut ExtensionStore) {
    note.note_log.grace = Some(grace_type_to_data_grace(gt));
    if let Some(ref id) = note.common.xml_id {
        ext_store.insert_grace_info(id.clone(), grace_type_to_info(gt));
    }
}

/// Set `@grace` on an MEI chord and store grace info in ext_store.
pub(super) fn apply_grace_to_chord(chord: &mut tusk_model::elements::Chord, gt: &GraceType, ext_store: &mut ExtensionStore) {
    chord.chord_log.grace = Some(grace_type_to_data_grace(gt));
    if let Some(ref id) = chord.common.xml_id {
        ext_store.insert_grace_info(id.clone(), grace_type_to_info(gt));
    }
}

/// Extract a Pitch from a Music node (for \relative and \transpose arguments).
pub(super) fn extract_pitch_from_music(music: &Music) -> Option<crate::model::Pitch> {
    match music {
        Music::Note(n) => Some(n.pitch.clone()),
        _ => None,
    }
}

/// Convert a LilyPond `FunctionArg` to a typed `ExtValue` for MEI storage.
fn function_arg_to_ext_value(arg: &crate::model::FunctionArg) -> tusk_model::ExtValue {
    use crate::model::FunctionArg;
    match arg {
        FunctionArg::Music(m) => tusk_model::ExtValue::Music(crate::serializer::serialize_music(m)),
        FunctionArg::String(s) => tusk_model::ExtValue::String(s.clone()),
        FunctionArg::Number(n) => tusk_model::ExtValue::Number(*n),
        FunctionArg::SchemeExpr(expr) => {
            tusk_model::ExtValue::Scheme(crate::serializer::serialize_scheme_expr(expr))
        }
        FunctionArg::Duration(dur) => tusk_model::ExtValue::Duration(dur.base, dur.dots),
        FunctionArg::Identifier(name) => tusk_model::ExtValue::Identifier(name.clone()),
        FunctionArg::Default => tusk_model::ExtValue::Default,
        FunctionArg::SymbolList(segments) => tusk_model::ExtValue::SymbolList(segments.clone()),
    }
}

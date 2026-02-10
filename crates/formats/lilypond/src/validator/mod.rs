//! Structural validation of the LilyPond AST.
//!
//! Checks consistency (e.g. brace matching, slur start/stop, context references).
//! Validation is run after parsing and before import to MEI.

use thiserror::Error;

use crate::model::*;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("score block has no music")]
    ScoreNoMusic,

    #[error("empty sequential music block")]
    EmptySequential,

    #[error("invalid duration base {base}: must be a power of 2 (1..128)")]
    InvalidDurationBase { base: u32 },

    #[error("excessive dots ({dots}): maximum recommended is 4")]
    ExcessiveDots { dots: u8 },

    #[error("duration multiplier denominator is zero")]
    ZeroMultiplierDenominator,

    #[error("unknown context type '{name}'")]
    UnknownContextType { name: String },

    #[error("unknown clef name '{name}'")]
    UnknownClefName { name: String },

    #[error("unknown key mode '{mode}'")]
    UnknownKeyMode { mode: String },

    #[error("invalid time signature: numerator must be positive")]
    InvalidTimeNumerator,

    #[error("invalid time signature: denominator must be positive")]
    InvalidTimeDenominator,

    #[error("chord must contain at least one pitch")]
    EmptyChord,

    #[error("unmatched slur: {0} open, {1} close")]
    UnmatchedSlur(usize, usize),

    #[error("unmatched phrasing slur: {0} open, {1} close")]
    UnmatchedPhrasingSlur(usize, usize),

    #[error("{0}")]
    Other(String),
}

/// Well-known LilyPond context types.
const KNOWN_CONTEXT_TYPES: &[&str] = &[
    "Score",
    "StaffGroup",
    "ChoirStaff",
    "GrandStaff",
    "PianoStaff",
    "Staff",
    "RhythmicStaff",
    "TabStaff",
    "DrumStaff",
    "Voice",
    "TabVoice",
    "DrumVoice",
    "Lyrics",
    "ChordNames",
    "FiguredBass",
    "Devnull",
    "NullVoice",
    "CueVoice",
    "Global",
    "MensuralStaff",
    "MensuralVoice",
    "VaticanaStaff",
    "VaticanaVoice",
    "GregorianTranscriptionStaff",
    "GregorianTranscriptionVoice",
    "KievanStaff",
    "KievanVoice",
    "PetrucciStaff",
    "PetrucciVoice",
];

// ---------------------------------------------------------------------------
// Validator
// ---------------------------------------------------------------------------

/// Validate a parsed [`LilyPondFile`] AST.
///
/// Returns `Ok(())` if the AST is structurally valid, or a list of errors.
pub fn validate(file: &LilyPondFile) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    for item in &file.items {
        validate_toplevel(item, &mut errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_toplevel(expr: &ToplevelExpression, errors: &mut Vec<ValidationError>) {
    match expr {
        ToplevelExpression::Score(sb) => validate_score(sb, errors),
        ToplevelExpression::Book(bb) => validate_book(bb, errors),
        ToplevelExpression::BookPart(bp) => validate_bookpart(bp, errors),
        ToplevelExpression::Header(hb) => validate_header(hb, errors),
        ToplevelExpression::Assignment(_) => {}
        ToplevelExpression::Music(m) => {
            validate_music(m, errors);
            validate_slur_balance(m, errors);
        }
    }
}

fn validate_score(sb: &ScoreBlock, errors: &mut Vec<ValidationError>) {
    // A score should have at least one music item
    let has_music = sb.items.iter().any(|i| matches!(i, ScoreItem::Music(_)));
    if !has_music {
        errors.push(ValidationError::ScoreNoMusic);
    }

    for item in &sb.items {
        match item {
            ScoreItem::Music(m) => {
                validate_music(m, errors);
                validate_slur_balance(m, errors);
            }
            ScoreItem::Header(hb) => validate_header(hb, errors),
            ScoreItem::Layout(_) | ScoreItem::Midi(_) => {}
        }
    }
}

fn validate_book(bb: &BookBlock, errors: &mut Vec<ValidationError>) {
    for item in &bb.items {
        match item {
            BookItem::Score(sb) => validate_score(sb, errors),
            BookItem::BookPart(bp) => validate_bookpart(bp, errors),
            BookItem::Header(hb) => validate_header(hb, errors),
            BookItem::Music(m) => validate_music(m, errors),
            BookItem::Paper(_) | BookItem::Assignment(_) => {}
        }
    }
}

fn validate_bookpart(bp: &BookPartBlock, errors: &mut Vec<ValidationError>) {
    for item in &bp.items {
        match item {
            BookPartItem::Score(sb) => validate_score(sb, errors),
            BookPartItem::Header(hb) => validate_header(hb, errors),
            BookPartItem::Music(m) => validate_music(m, errors),
            BookPartItem::Paper(_) | BookPartItem::Assignment(_) => {}
        }
    }
}

fn validate_header(_hb: &HeaderBlock, _errors: &mut Vec<ValidationError>) {
    // Header field validation can be extended later
}

fn validate_post_events(events: &[note::PostEvent], _errors: &mut Vec<ValidationError>) {
    // Individual post-event validation (extensible for future checks).
    // Slur balance is checked at the sequential-music level.
    let _ = events;
}

fn count_slurs(
    m: &Music,
    slur_opens: &mut usize,
    slur_closes: &mut usize,
    phr_opens: &mut usize,
    phr_closes: &mut usize,
) {
    let check_post = |events: &[note::PostEvent],
                      so: &mut usize,
                      sc: &mut usize,
                      po: &mut usize,
                      pc: &mut usize| {
        for ev in events {
            match ev {
                note::PostEvent::SlurStart => *so += 1,
                note::PostEvent::SlurEnd => *sc += 1,
                note::PostEvent::PhrasingSlurStart => *po += 1,
                note::PostEvent::PhrasingSlurEnd => *pc += 1,
                note::PostEvent::Tie => {}
            }
        }
    };

    match m {
        Music::Note(n) => check_post(
            &n.post_events,
            slur_opens,
            slur_closes,
            phr_opens,
            phr_closes,
        ),
        Music::Chord(c) => check_post(
            &c.post_events,
            slur_opens,
            slur_closes,
            phr_opens,
            phr_closes,
        ),
        Music::Rest(r) => check_post(
            &r.post_events,
            slur_opens,
            slur_closes,
            phr_opens,
            phr_closes,
        ),
        Music::Skip(s) => check_post(
            &s.post_events,
            slur_opens,
            slur_closes,
            phr_opens,
            phr_closes,
        ),
        Music::MultiMeasureRest(r) => check_post(
            &r.post_events,
            slur_opens,
            slur_closes,
            phr_opens,
            phr_closes,
        ),
        Music::Sequential(items) | Music::Simultaneous(items) => {
            for item in items {
                count_slurs(item, slur_opens, slur_closes, phr_opens, phr_closes);
            }
        }
        Music::Relative { body, .. } | Music::Fixed { body, .. } => {
            count_slurs(body, slur_opens, slur_closes, phr_opens, phr_closes);
        }
        Music::Transpose { body, .. } => {
            count_slurs(body, slur_opens, slur_closes, phr_opens, phr_closes);
        }
        Music::ContextedMusic { music, .. } => {
            count_slurs(music, slur_opens, slur_closes, phr_opens, phr_closes);
        }
        _ => {}
    }
}

fn validate_slur_balance(m: &Music, errors: &mut Vec<ValidationError>) {
    let (mut so, mut sc, mut po, mut pc) = (0, 0, 0, 0);
    count_slurs(m, &mut so, &mut sc, &mut po, &mut pc);
    if so != sc {
        errors.push(ValidationError::UnmatchedSlur(so, sc));
    }
    if po != pc {
        errors.push(ValidationError::UnmatchedPhrasingSlur(po, pc));
    }
}

fn validate_music(m: &Music, errors: &mut Vec<ValidationError>) {
    match m {
        Music::Sequential(items) => {
            for item in items {
                validate_music(item, errors);
            }
        }
        Music::Simultaneous(items) => {
            for item in items {
                validate_music(item, errors);
            }
        }
        Music::Relative { pitch, body } => {
            if let Some(p) = pitch {
                validate_music(p, errors);
            }
            validate_music(body, errors);
        }
        Music::Fixed { pitch, body } => {
            validate_music(pitch, errors);
            validate_music(body, errors);
        }
        Music::Transpose { from, to, body } => {
            validate_music(from, errors);
            validate_music(to, errors);
            validate_music(body, errors);
        }
        Music::ContextedMusic {
            context_type,
            music,
            ..
        } => {
            if !KNOWN_CONTEXT_TYPES.contains(&context_type.as_str()) {
                errors.push(ValidationError::UnknownContextType {
                    name: context_type.clone(),
                });
            }
            validate_music(music, errors);
        }
        Music::ContextChange { context_type, .. } => {
            if !KNOWN_CONTEXT_TYPES.contains(&context_type.as_str()) {
                errors.push(ValidationError::UnknownContextType {
                    name: context_type.clone(),
                });
            }
        }
        Music::Clef(c) => {
            if !c.is_known() {
                errors.push(ValidationError::UnknownClefName {
                    name: c.name.clone(),
                });
            }
        }
        Music::KeySignature(_) => {
            // Pitch and mode are structurally valid by construction
        }
        Music::TimeSignature(ts) => {
            if ts.numerators.is_empty() || ts.numerators.contains(&0) {
                errors.push(ValidationError::InvalidTimeNumerator);
            }
            if ts.denominator == 0 {
                errors.push(ValidationError::InvalidTimeDenominator);
            }
        }
        Music::Note(n) => {
            if let Some(dur) = &n.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&n.post_events, errors);
        }
        Music::Chord(c) => {
            if c.pitches.is_empty() {
                errors.push(ValidationError::EmptyChord);
            }
            if let Some(dur) = &c.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&c.post_events, errors);
        }
        Music::Rest(r) => {
            if let Some(dur) = &r.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&r.post_events, errors);
        }
        Music::Skip(s) => {
            if let Some(dur) = &s.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&s.post_events, errors);
        }
        Music::MultiMeasureRest(r) => {
            if let Some(dur) = &r.duration {
                validate_duration(dur, errors);
            }
            validate_post_events(&r.post_events, errors);
        }
        Music::Event(_) | Music::Identifier(_) | Music::Unparsed(_) => {}
    }
}

fn validate_duration(dur: &Duration, errors: &mut Vec<ValidationError>) {
    if !Duration::is_valid_base(dur.base) {
        errors.push(ValidationError::InvalidDurationBase { base: dur.base });
    }
    if dur.dots > 4 {
        errors.push(ValidationError::ExcessiveDots { dots: dur.dots });
    }
    for &(_, den) in &dur.multipliers {
        if den == 0 {
            errors.push(ValidationError::ZeroMultiplierDenominator);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_score_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![ToplevelExpression::Score(ScoreBlock {
                items: vec![ScoreItem::Music(Music::Sequential(vec![Music::Note(
                    NoteEvent {
                        pitch: Pitch {
                            step: 'c',
                            alter: 0.0,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                        duration: Some(Duration {
                            base: 4,
                            dots: 0,
                            multipliers: vec![],
                        }),
                        pitched_rest: false,
                        post_events: vec![],
                    },
                )]))],
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_book_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![ToplevelExpression::Book(BookBlock {
                items: vec![BookItem::Score(ScoreBlock {
                    items: vec![ScoreItem::Music(Music::Sequential(vec![Music::Note(
                        NoteEvent {
                            pitch: Pitch {
                                step: 'c',
                                alter: 0.0,
                                octave: 0,
                                force_accidental: false,
                                cautionary: false,
                                octave_check: None,
                            },
                            duration: Some(Duration {
                                base: 4,
                                dots: 0,
                                multipliers: vec![],
                            }),
                            pitched_rest: false,
                            post_events: vec![],
                        },
                    )]))],
                })],
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn score_without_music_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Score(ScoreBlock {
                items: vec![ScoreItem::Layout(LayoutBlock { body: vec![] })],
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::ScoreNoMusic))
        );
    }

    #[test]
    fn empty_file_passes() {
        let file = LilyPondFile {
            version: Some(Version {
                version: "2.24.0".into(),
            }),
            items: vec![],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn header_only_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Header(HeaderBlock {
                fields: vec![Assignment {
                    name: "title".into(),
                    value: AssignmentValue::String("Test".into()),
                }],
            })],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 3 validator tests ─────────────────────────────────────

    #[test]
    fn invalid_duration_base() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 3, // invalid
                        dots: 0,
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
        );
    }

    #[test]
    fn excessive_dots() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 5, // excessive
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::ExcessiveDots { dots: 5 }))
        );
    }

    #[test]
    fn zero_multiplier_denominator() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Rest(RestEvent {
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![(2, 0)],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::ZeroMultiplierDenominator))
        );
    }

    #[test]
    fn unknown_context_type() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::ContextedMusic {
                keyword: ContextKeyword::New,
                context_type: "Bogus".into(),
                name: None,
                with_block: None,
                music: Box::new(Music::Sequential(vec![])),
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter().any(
                |e| matches!(e, ValidationError::UnknownContextType { name } if name == "Bogus")
            )
        );
    }

    #[test]
    fn known_context_type_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::ContextedMusic {
                keyword: ContextKeyword::New,
                context_type: "Staff".into(),
                name: None,
                with_block: None,
                music: Box::new(Music::Sequential(vec![Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                })])),
            })],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn context_change_unknown_type() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::ContextChange {
                context_type: "FooBar".into(),
                name: "x".into(),
            })],
        };
        let errs = validate(&file).unwrap_err();
        assert!(errs.iter().any(
            |e| matches!(e, ValidationError::UnknownContextType { name } if name == "FooBar")
        ));
    }

    #[test]
    fn valid_duration_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 2,
                        multipliers: vec![(3, 2)],
                    }),
                    pitched_rest: false,
                    post_events: vec![],
                }),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 6 validator tests ─────────────────────────────────────

    #[test]
    fn valid_clef_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Clef(Clef {
                name: "treble".into(),
            }))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn unknown_clef_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Clef(Clef {
                name: "bogus".into(),
            }))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnknownClefName { name } if name == "bogus"))
        );
    }

    #[test]
    fn transposed_clef_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Clef(Clef {
                name: "G_8".into(),
            }))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_key_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::KeySignature(
                KeySignature {
                    pitch: Pitch {
                        step: 'd',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    mode: Mode::Major,
                },
            ))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn valid_time_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::TimeSignature(
                TimeSignature {
                    numerators: vec![4],
                    denominator: 4,
                },
            ))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn zero_time_denominator_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::TimeSignature(
                TimeSignature {
                    numerators: vec![4],
                    denominator: 0,
                },
            ))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidTimeDenominator))
        );
    }

    #[test]
    fn zero_time_numerator_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::TimeSignature(
                TimeSignature {
                    numerators: vec![0],
                    denominator: 4,
                },
            ))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidTimeNumerator))
        );
    }

    #[test]
    fn additive_time_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::TimeSignature(
                TimeSignature {
                    numerators: vec![3, 3, 2],
                    denominator: 8,
                },
            ))],
        };
        assert!(validate(&file).is_ok());
    }

    // ── Phase 8 validator tests ─────────────────────────────────────

    #[test]
    fn valid_chord_passes() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Chord(ChordEvent {
                    pitches: vec![
                        Pitch {
                            step: 'c',
                            alter: 0.0,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                        Pitch {
                            step: 'e',
                            alter: 0.0,
                            octave: 0,
                            force_accidental: false,
                            cautionary: false,
                            octave_check: None,
                        },
                    ],
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn empty_chord_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Chord(ChordEvent {
                    pitches: vec![],
                    duration: Some(Duration {
                        base: 4,
                        dots: 0,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::EmptyChord))
        );
    }

    #[test]
    fn chord_invalid_duration_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                Music::Chord(ChordEvent {
                    pitches: vec![Pitch {
                        step: 'c',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    }],
                    duration: Some(Duration {
                        base: 3, // invalid
                        dots: 0,
                        multipliers: vec![],
                    }),
                    post_events: vec![],
                }),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
        );
    }

    // ── Phase 9 validator tests ─────────────────────────────────────

    fn make_note(post_events: Vec<note::PostEvent>) -> Music {
        Music::Note(NoteEvent {
            pitch: Pitch {
                step: 'c',
                alter: 0.0,
                octave: 0,
                force_accidental: false,
                cautionary: false,
                octave_check: None,
            },
            duration: Some(Duration {
                base: 4,
                dots: 0,
                multipliers: vec![],
            }),
            pitched_rest: false,
            post_events,
        })
    }

    #[test]
    fn balanced_slurs_pass() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::SlurStart]),
                make_note(vec![]),
                make_note(vec![PostEvent::SlurEnd]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn unmatched_slur_start_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::SlurStart]),
                make_note(vec![]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnmatchedSlur(1, 0)))
        );
    }

    #[test]
    fn unmatched_phrasing_slur_fails() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::PhrasingSlurStart]),
                make_note(vec![]),
            ]))],
        };
        let errs = validate(&file).unwrap_err();
        assert!(
            errs.iter()
                .any(|e| matches!(e, ValidationError::UnmatchedPhrasingSlur(1, 0)))
        );
    }

    #[test]
    fn balanced_phrasing_slurs_pass() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::PhrasingSlurStart]),
                make_note(vec![PostEvent::PhrasingSlurEnd]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }

    #[test]
    fn tie_does_not_affect_slur_balance() {
        let file = LilyPondFile {
            version: None,
            items: vec![ToplevelExpression::Music(Music::Sequential(vec![
                make_note(vec![PostEvent::Tie]),
                make_note(vec![]),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }
}

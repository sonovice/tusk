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

    #[error("{0}")]
    Other(String),
}

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
        ToplevelExpression::Music(m) => validate_music(m, errors),
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
            ScoreItem::Music(m) => validate_music(m, errors),
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
        Music::ContextedMusic { music, .. } => {
            validate_music(music, errors);
        }
        Music::Note(n) => {
            if let Some(dur) = &n.duration {
                validate_duration(dur, errors);
            }
        }
        Music::Rest(r) => {
            if let Some(dur) = &r.duration {
                validate_duration(dur, errors);
            }
        }
        Music::Skip(s) => {
            if let Some(dur) = &s.duration {
                validate_duration(dur, errors);
            }
        }
        Music::MultiMeasureRest(r) => {
            if let Some(dur) = &r.duration {
                validate_duration(dur, errors);
            }
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
                        },
                        duration: Some(Duration {
                            base: 4,
                            dots: 0,
                            multipliers: vec![],
                        }),
                        pitched_rest: false,
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
                            },
                            duration: Some(Duration {
                                base: 4,
                                dots: 0,
                                multipliers: vec![],
                            }),
                            pitched_rest: false,
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
                    },
                    duration: Some(Duration {
                        base: 3, // invalid
                        dots: 0,
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
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
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 5, // excessive
                        multipliers: vec![],
                    }),
                    pitched_rest: false,
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
                    },
                    duration: Some(Duration {
                        base: 4,
                        dots: 2,
                        multipliers: vec![(3, 2)],
                    }),
                    pitched_rest: false,
                }),
            ]))],
        };
        assert!(validate(&file).is_ok());
    }
}

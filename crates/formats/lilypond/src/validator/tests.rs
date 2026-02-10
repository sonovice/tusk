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
        errs.iter()
            .any(|e| matches!(e, ValidationError::UnknownContextType { name } if name == "Bogus"))
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
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::UnknownContextType { name } if name == "FooBar"))
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

// ── Phase 10 validator tests ────────────────────────────────────

#[test]
fn balanced_beams_pass() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::BeamStart]),
            make_note(vec![]),
            make_note(vec![PostEvent::BeamEnd]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn unmatched_beam_start_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::BeamStart]),
            make_note(vec![]),
        ]))],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::UnmatchedBeam(1, 0)))
    );
}

#[test]
fn unmatched_beam_end_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![]),
            make_note(vec![PostEvent::BeamEnd]),
        ]))],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::UnmatchedBeam(0, 1)))
    );
}

// ── Phase 11 validator tests ───────────────────────────────────

#[test]
fn balanced_hairpin_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Crescendo]),
            make_note(vec![]),
            make_note(vec![PostEvent::HairpinEnd]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn unmatched_hairpin_start_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Crescendo]),
            make_note(vec![]),
        ]))],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::UnmatchedHairpin(1, 0)))
    );
}

#[test]
fn unmatched_hairpin_end_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![]),
            make_note(vec![PostEvent::HairpinEnd]),
        ]))],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::UnmatchedHairpin(0, 1)))
    );
}

#[test]
fn decrescendo_hairpin_balanced() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Decrescendo]),
            make_note(vec![PostEvent::HairpinEnd]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn dynamic_does_not_affect_hairpin_balance() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Dynamic("f".into())]),
            make_note(vec![PostEvent::Dynamic("p".into())]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn known_dynamic_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Dynamic("sfz".into())]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

// ── Phase 12 validator tests ──────────────────────────────────

#[test]
fn valid_articulation_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Articulation {
                direction: Direction::Neutral,
                script: ScriptAbbreviation::Dot,
            }]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_fingering_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Fingering {
                direction: Direction::Up,
                digit: 3,
            }]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_string_number_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::StringNumber {
                direction: Direction::Neutral,
                number: 1,
            }]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

// ── Phase 13 validator tests ──────────────────────────────────

#[test]
fn valid_tremolo_32_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Tremolo(32)]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_tremolo_bare_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Tremolo(0)]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn invalid_tremolo_type_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Tremolo(12)]),
        ]))],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidTremoloType { value: 12 }))
    );
}

#[test]
fn invalid_tremolo_4_fails() {
    // 4 is a power of 2 but < 8, not valid for tremolo
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Tremolo(4)]),
        ]))],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidTremoloType { value: 4 }))
    );
}

#[test]
fn tremolo_does_not_affect_span_balance() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![PostEvent::Tremolo(32)]),
            make_note(vec![PostEvent::Tremolo(16)]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

// ── Phase 12 validator tests (continued) ──────────────────────

#[test]
fn articulation_does_not_affect_span_balance() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![
                PostEvent::Articulation {
                    direction: Direction::Neutral,
                    script: ScriptAbbreviation::Accent,
                },
                PostEvent::Fingering {
                    direction: Direction::Neutral,
                    digit: 1,
                },
            ]),
            make_note(vec![PostEvent::NamedArticulation {
                direction: Direction::Down,
                name: "staccato".into(),
            }]),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

// ── Phase 15 validator tests ──────────────────────────────────

#[test]
fn valid_tuplet_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Tuplet {
            numerator: 3,
            denominator: 2,
            span_duration: None,
            body: Box::new(Music::Sequential(vec![
                make_note(vec![]),
                make_note(vec![]),
                make_note(vec![]),
            ])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn tuplet_zero_numerator_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Tuplet {
            numerator: 0,
            denominator: 2,
            span_duration: None,
            body: Box::new(Music::Sequential(vec![])),
        })],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidTupletFraction))
    );
}

#[test]
fn tuplet_zero_denominator_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Tuplet {
            numerator: 3,
            denominator: 0,
            span_duration: None,
            body: Box::new(Music::Sequential(vec![])),
        })],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidTupletFraction))
    );
}

#[test]
fn tuplet_invalid_span_duration_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Tuplet {
            numerator: 3,
            denominator: 2,
            span_duration: Some(Duration {
                base: 3, // invalid
                dots: 0,
                multipliers: vec![],
            }),
            body: Box::new(Music::Sequential(vec![])),
        })],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
    );
}

#[test]
fn tuplet_with_slurs_span_balance() {
    // Slurs inside a tuplet should be tracked for balance
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Tuplet {
            numerator: 3,
            denominator: 2,
            span_duration: None,
            body: Box::new(Music::Sequential(vec![
                make_note(vec![PostEvent::SlurStart]),
                make_note(vec![]),
                make_note(vec![PostEvent::SlurEnd]),
            ])),
        })],
    };
    assert!(validate(&file).is_ok());
}

// ── Phase 16 validator tests ──────────────────────────────────

#[test]
fn valid_grace_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Grace {
            body: Box::new(make_note(vec![])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_acciaccatura_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Acciaccatura {
            body: Box::new(make_note(vec![])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_appoggiatura_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Appoggiatura {
            body: Box::new(make_note(vec![])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_after_grace_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::AfterGrace {
            fraction: Some((3, 4)),
            main: Box::new(make_note(vec![])),
            grace: Box::new(Music::Sequential(vec![make_note(vec![])])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn after_grace_no_fraction_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::AfterGrace {
            fraction: None,
            main: Box::new(make_note(vec![])),
            grace: Box::new(Music::Sequential(vec![make_note(vec![])])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn after_grace_zero_fraction_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::AfterGrace {
            fraction: Some((0, 4)),
            main: Box::new(make_note(vec![])),
            grace: Box::new(Music::Sequential(vec![make_note(vec![])])),
        })],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidAfterGraceFraction))
    );
}

#[test]
fn grace_with_slurs_span_balance() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Grace {
            body: Box::new(Music::Sequential(vec![
                make_note(vec![PostEvent::SlurStart]),
                make_note(vec![PostEvent::SlurEnd]),
            ])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn nested_tuplet_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Tuplet {
            numerator: 3,
            denominator: 2,
            span_duration: None,
            body: Box::new(Music::Sequential(vec![
                Music::Tuplet {
                    numerator: 3,
                    denominator: 2,
                    span_duration: None,
                    body: Box::new(Music::Sequential(vec![
                        make_note(vec![]),
                        make_note(vec![]),
                        make_note(vec![]),
                    ])),
                },
                make_note(vec![]),
            ])),
        })],
    };
    assert!(validate(&file).is_ok());
}

// ── Repeat validation (Phase 17) ────────────────────────────────────────

#[test]
fn valid_repeat_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Repeat {
            repeat_type: RepeatType::Volta,
            count: 2,
            body: Box::new(Music::Sequential(vec![make_note(vec![])])),
            alternatives: None,
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn repeat_zero_count_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Repeat {
            repeat_type: RepeatType::Volta,
            count: 0,
            body: Box::new(Music::Sequential(vec![make_note(vec![])])),
            alternatives: None,
        })],
    };
    let errors = validate(&file).unwrap_err();
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::InvalidRepeatCount))
    );
}

#[test]
fn valid_repeat_with_alternatives_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Repeat {
            repeat_type: RepeatType::Volta,
            count: 2,
            body: Box::new(Music::Sequential(vec![make_note(vec![])])),
            alternatives: Some(vec![
                Music::Sequential(vec![make_note(vec![])]),
                Music::Sequential(vec![make_note(vec![])]),
            ]),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn repeat_span_balance_in_body() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Repeat {
            repeat_type: RepeatType::Volta,
            count: 2,
            body: Box::new(Music::Sequential(vec![
                make_note(vec![PostEvent::SlurStart]),
                make_note(vec![PostEvent::SlurEnd]),
            ])),
            alternatives: None,
        })],
    };
    assert!(validate(&file).is_ok());
}

// ── Bar check & bar line (Phase 18) ─────────────────────────────────────

#[test]
fn bar_check_passes() {
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
                    dots: 0,
                    multipliers: vec![],
                }),
                pitched_rest: false,
                post_events: vec![],
            }),
            Music::BarCheck,
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn bar_line_valid_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::BarLine {
            bar_type: "|.".to_string(),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn bar_line_empty_type_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::BarLine {
            bar_type: String::new(),
        })],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::EmptyBarLineType))
    );
}

// ── Chord repetition (Phase 19) ───────────────────────────────────────

#[test]
fn chord_rep_valid_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::ChordRepetition(
            ChordRepetitionEvent {
                duration: Some(Duration {
                    base: 4,
                    dots: 0,
                    multipliers: vec![],
                }),
                post_events: vec![],
            },
        ))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn chord_rep_invalid_duration_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::ChordRepetition(
            ChordRepetitionEvent {
                duration: Some(Duration {
                    base: 3, // invalid
                    dots: 0,
                    multipliers: vec![],
                }),
                post_events: vec![],
            },
        ))],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
    );
}

#[test]
fn chord_rep_span_balance() {
    // ChordRepetition with slur open should be counted in span balance
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::ChordRepetition(ChordRepetitionEvent {
                duration: Some(Duration {
                    base: 4,
                    dots: 0,
                    multipliers: vec![],
                }),
                post_events: vec![PostEvent::SlurStart],
            }),
            Music::ChordRepetition(ChordRepetitionEvent {
                duration: None,
                post_events: vec![PostEvent::SlurEnd],
            }),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

// ── Lyric validation (Phase 20) ─────────────────────────────────

#[test]
fn lyricmode_valid_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::LyricMode {
            body: Box::new(Music::Sequential(vec![
                Music::Lyric(LyricEvent {
                    text: "hello".into(),
                    duration: None,
                    post_events: vec![],
                }),
                Music::Lyric(LyricEvent {
                    text: "world".into(),
                    duration: None,
                    post_events: vec![],
                }),
            ])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn lyric_with_invalid_duration_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::LyricMode {
            body: Box::new(Music::Sequential(vec![Music::Lyric(LyricEvent {
                text: "bad".into(),
                duration: Some(Duration {
                    base: 3, // invalid — not a power of 2
                    dots: 0,
                    multipliers: vec![],
                }),
                post_events: vec![],
            })])),
        })],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
    );
}

#[test]
fn addlyrics_valid_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::AddLyrics {
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
            lyrics: vec![Music::Sequential(vec![Music::Lyric(LyricEvent {
                text: "do".into(),
                duration: None,
                post_events: vec![],
            })])],
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn lyricsto_valid_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::LyricsTo {
            voice_id: "melody".into(),
            lyrics: Box::new(Music::Sequential(vec![Music::Lyric(LyricEvent {
                text: "la".into(),
                duration: None,
                post_events: vec![],
            })])),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn lyric_hyphen_extender_dont_affect_span_balance() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::LyricMode {
            body: Box::new(Music::Sequential(vec![
                Music::Lyric(LyricEvent {
                    text: "hel".into(),
                    duration: None,
                    post_events: vec![PostEvent::LyricHyphen],
                }),
                Music::Lyric(LyricEvent {
                    text: "lo".into(),
                    duration: None,
                    post_events: vec![PostEvent::LyricExtender],
                }),
            ])),
        })],
    };
    assert!(validate(&file).is_ok());
}

// ──────────────────────────────────────────────────────────────────
// Markup validation (Phase 21)
// ──────────────────────────────────────────────────────────────────

#[test]
fn valid_markup_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Markup(markup::Markup::String(
            "Hello".into(),
        ))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_markup_command_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Markup(markup::Markup::Command {
            name: "bold".into(),
            args: vec![markup::Markup::String("text".into())],
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_markup_list_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Markup(markup::Markup::List(vec![
            markup::Markup::Word("Hello".into()),
            markup::Markup::Word("World".into()),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn valid_markuplist_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::MarkupList(markup::MarkupList {
            items: vec![
                markup::Markup::String("one".into()),
                markup::Markup::String("two".into()),
            ],
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn markup_score_validates_inner() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Markup(markup::Markup::Score(
            ScoreBlock { items: vec![] },
        ))],
    };
    // Score with no music should produce ScoreNoMusic error
    let result = validate(&file);
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::ScoreNoMusic))
    );
}

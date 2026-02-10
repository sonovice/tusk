//! Extended validator tests (Phase 17+: repeats, bar checks, chord repetition, lyrics, markup, tempo, marks).

use super::*;

fn make_note(post_events: Vec<PostEvent>) -> Music {
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

fn wrap_music(m: Music) -> LilyPondFile {
    LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Score(ScoreBlock {
            items: vec![ScoreItem::Music(Music::Sequential(vec![
                m,
                make_note(vec![]),
            ]))],
        })],
    }
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

// ── Tempo / Mark / TextMark validation (Phase 22) ───────────────────────

#[test]
fn valid_tempo_passes() {
    let file = wrap_music(Music::Tempo(Tempo {
        text: Some(markup::Markup::Word("Allegro".into())),
        duration: Some(Duration {
            base: 4,
            dots: 0,
            multipliers: vec![],
        }),
        bpm: Some(TempoRange::Single(120)),
    }));
    assert!(validate(&file).is_ok());
}

#[test]
fn tempo_text_only_passes() {
    let file = wrap_music(Music::Tempo(Tempo {
        text: Some(markup::Markup::Word("Adagio".into())),
        duration: None,
        bpm: None,
    }));
    assert!(validate(&file).is_ok());
}

#[test]
fn tempo_range_invalid_order() {
    let file = wrap_music(Music::Tempo(Tempo {
        text: None,
        duration: Some(Duration {
            base: 4,
            dots: 0,
            multipliers: vec![],
        }),
        bpm: Some(TempoRange::Range(144, 120)),
    }));
    let result = validate(&file);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .iter()
            .any(|e| matches!(e, ValidationError::InvalidTempoRange { .. }))
    );
}

#[test]
fn tempo_zero_bpm_fails() {
    let file = wrap_music(Music::Tempo(Tempo {
        text: None,
        duration: Some(Duration {
            base: 4,
            dots: 0,
            multipliers: vec![],
        }),
        bpm: Some(TempoRange::Single(0)),
    }));
    let result = validate(&file);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .iter()
            .any(|e| matches!(e, ValidationError::InvalidTempoBpm))
    );
}

#[test]
fn mark_default_passes() {
    let file = wrap_music(Music::Mark(Mark {
        label: MarkLabel::Default,
    }));
    assert!(validate(&file).is_ok());
}

#[test]
fn text_mark_passes() {
    let file = wrap_music(Music::TextMark(TextMark {
        text: markup::Markup::Word("Fine".into()),
    }));
    assert!(validate(&file).is_ok());
}

#[test]
fn figure_mode_valid_passes() {
    let file = wrap_music(Music::FigureMode {
        body: Box::new(Music::Sequential(vec![Music::Figure(FigureEvent {
            figures: vec![
                BassFigure {
                    number: Some(6),
                    alteration: FigureAlteration::Natural,
                    modifications: vec![],
                    bracket_start: false,
                    bracket_stop: false,
                },
                BassFigure {
                    number: Some(4),
                    alteration: FigureAlteration::Sharp,
                    modifications: vec![FiguredBassModification::Augmented],
                    bracket_start: true,
                    bracket_stop: true,
                },
            ],
            duration: Some(Duration {
                base: 4,
                dots: 0,
                multipliers: vec![],
            }),
        })])),
    });
    assert!(validate(&file).is_ok());
}

#[test]
fn figure_invalid_number_fails() {
    let file = wrap_music(Music::Figure(FigureEvent {
        figures: vec![BassFigure {
            number: Some(0),
            alteration: FigureAlteration::Natural,
            modifications: vec![],
            bracket_start: false,
            bracket_stop: false,
        }],
        duration: None,
    }));
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidFigureNumber { number: 0 }))
    );
}

#[test]
fn figure_space_valid() {
    let file = wrap_music(Music::Figure(FigureEvent {
        figures: vec![BassFigure {
            number: None,
            alteration: FigureAlteration::Natural,
            modifications: vec![],
            bracket_start: false,
            bracket_stop: false,
        }],
        duration: None,
    }));
    assert!(validate(&file).is_ok());
}

// ── Assignment / variable validation (Phase 28) ────────────────────────

#[test]
fn valid_assignment_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Assignment(Assignment {
            name: "melody".into(),
            value: AssignmentValue::Music(Box::new(Music::Sequential(vec![make_note(vec![])]))),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn assignment_string_value_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Assignment(Assignment {
            name: "myTitle".into(),
            value: AssignmentValue::String("Hello".into()),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn assignment_number_value_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Assignment(Assignment {
            name: "myNum".into(),
            value: AssignmentValue::Number(42.0),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn assignment_identifier_value_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Assignment(Assignment {
            name: "soprano".into(),
            value: AssignmentValue::Identifier("melody".into()),
        })],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn empty_assignment_name_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Assignment(Assignment {
            name: String::new(),
            value: AssignmentValue::Number(1.0),
        })],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::EmptyAssignmentName))
    );
}

#[test]
fn assignment_with_invalid_music_value_fails() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Assignment(Assignment {
            name: "broken".into(),
            value: AssignmentValue::Music(Box::new(Music::Note(NoteEvent {
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
            }))),
        })],
    };
    let errs = validate(&file).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::InvalidDurationBase { base: 3 }))
    );
}

#[test]
fn assignment_then_score_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![
            ToplevelExpression::Assignment(Assignment {
                name: "melody".into(),
                value: AssignmentValue::Music(Box::new(Music::Sequential(vec![make_note(vec![])]))),
            }),
            ToplevelExpression::Score(ScoreBlock {
                items: vec![ScoreItem::Music(Music::ContextedMusic {
                    keyword: ContextKeyword::New,
                    context_type: "Staff".into(),
                    name: None,
                    with_block: None,
                    music: Box::new(Music::Identifier("melody".into())),
                })],
            }),
        ],
    };
    assert!(validate(&file).is_ok());
}

#[test]
fn music_identifier_ref_passes() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note(vec![]),
            Music::Identifier("someVar".into()),
        ]))],
    };
    assert!(validate(&file).is_ok());
}

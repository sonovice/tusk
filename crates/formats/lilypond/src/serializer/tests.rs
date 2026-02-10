use super::*;

#[test]
fn serialize_version_only() {
    let file = LilyPondFile {
        version: Some(Version {
            version: "2.24.0".into(),
        }),
        items: vec![],
    };
    let output = serialize(&file);
    assert_eq!(output, "\\version \"2.24.0\"\n");
}

#[test]
fn serialize_minimal_score() {
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
    let output = serialize(&file);
    assert!(output.contains("\\version \"2.24.0\""));
    assert!(output.contains("\\score {"));
    assert!(output.contains("{ c4 }"));
    assert!(output.contains("}"));
}

#[test]
fn serialize_header_block() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Header(HeaderBlock {
            fields: vec![
                Assignment {
                    name: "title".into(),
                    value: AssignmentValue::String("My Piece".into()),
                },
                Assignment {
                    name: "composer".into(),
                    value: AssignmentValue::String("JS Bach".into()),
                },
            ],
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\header {"));
    assert!(output.contains("title = \"My Piece\""));
    assert!(output.contains("composer = \"JS Bach\""));
}

#[test]
fn serialize_score_with_layout_midi() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Score(ScoreBlock {
            items: vec![
                ScoreItem::Music(Music::Sequential(vec![Music::Note(NoteEvent {
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
                ScoreItem::Layout(LayoutBlock { body: vec![] }),
                ScoreItem::Midi(MidiBlock { body: vec![] }),
            ],
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\layout { }"));
    assert!(output.contains("\\midi { }"));
}

#[test]
fn serialize_assignment() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Assignment(Assignment {
            name: "melody".into(),
            value: AssignmentValue::Music(Box::new(Music::Sequential(vec![
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
                Music::Note(NoteEvent {
                    pitch: Pitch {
                        step: 'd',
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
            ]))),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("melody = { c4 d4 }"));
}

#[test]
fn roundtrip_parse_serialize() {
    let input = "\\version \"2.24.0\"\n\\score {\n  { c4 }\n}\n";
    let ast = crate::parser::parse(input).unwrap();
    let output = serialize(&ast);
    // Re-parse the serialized output
    let ast2 = crate::parser::parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

// ── Phase 3 serializer tests ────────────────────────────────────

#[test]
fn serialize_note_with_accidental_octave() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Note(NoteEvent {
                pitch: Pitch {
                    step: 'c',
                    alter: 1.0,
                    octave: 2,
                    force_accidental: true,
                    cautionary: false,
                    octave_check: None,
                },
                duration: Some(Duration {
                    base: 4,
                    dots: 1,
                    multipliers: vec![],
                }),
                pitched_rest: false,
                post_events: vec![],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("cis''!4."));
}

#[test]
fn serialize_rest() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Rest(RestEvent {
                duration: Some(Duration {
                    base: 2,
                    dots: 0,
                    multipliers: vec![],
                }),
                post_events: vec![],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("r2"));
}

#[test]
fn serialize_skip() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Skip(SkipEvent {
                duration: Some(Duration {
                    base: 4,
                    dots: 0,
                    multipliers: vec![],
                }),
                post_events: vec![],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("s4"));
}

#[test]
fn serialize_multi_measure_rest_with_multiplier() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::MultiMeasureRest(MultiMeasureRestEvent {
                duration: Some(Duration {
                    base: 1,
                    dots: 0,
                    multipliers: vec![(4, 1)],
                }),
                post_events: vec![],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("R1*4"));
}

#[test]
fn serialize_duration_fraction_multiplier() {
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
                    multipliers: vec![(2, 3)],
                }),
                pitched_rest: false,
                post_events: vec![],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4*2/3"));
}

#[test]
fn serialize_pitched_rest() {
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
                pitched_rest: true,
                post_events: vec![],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4\\rest"));
}

// ── Phase 6 serializer tests ────────────────────────────────────

#[test]
fn serialize_clef() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Clef(Clef {
                name: "bass".into(),
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\clef \"bass\""));
}

#[test]
fn serialize_key_signature() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::KeySignature(KeySignature {
                pitch: Pitch {
                    step: 'b',
                    alter: -1.0,
                    octave: 0,
                    force_accidental: false,
                    cautionary: false,
                    octave_check: None,
                },
                mode: Mode::Minor,
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\key bes \\minor"));
}

#[test]
fn serialize_time_signature() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::TimeSignature(TimeSignature {
                numerators: vec![4],
                denominator: 4,
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\time 4/4"));
}

#[test]
fn serialize_time_signature_additive() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::TimeSignature(TimeSignature {
                numerators: vec![3, 3, 2],
                denominator: 8,
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\time 3+3+2/8"));
}

// ── Phase 8 serializer tests ────────────────────────────────────

#[test]
fn serialize_chord() {
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
                    Pitch {
                        step: 'g',
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
    let output = serialize(&file);
    assert!(output.contains("<c e g>4"));
}

#[test]
fn serialize_chord_accidentals() {
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
                        alter: -1.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                    Pitch {
                        step: 'g',
                        alter: 0.0,
                        octave: 0,
                        force_accidental: false,
                        cautionary: false,
                        octave_check: None,
                    },
                ],
                duration: Some(Duration {
                    base: 2,
                    dots: 1,
                    multipliers: vec![],
                }),
                post_events: vec![],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("<c ees g>2."));
}

// ── Phase 12 serializer tests ────────────────────────────────

#[test]
fn serialize_articulation_staccato() {
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
                post_events: vec![PostEvent::Articulation {
                    direction: Direction::Neutral,
                    script: ScriptAbbreviation::Dot,
                }],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4-."));
}

#[test]
fn serialize_articulation_direction_up() {
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
                post_events: vec![PostEvent::Articulation {
                    direction: Direction::Up,
                    script: ScriptAbbreviation::Accent,
                }],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4^>"));
}

#[test]
fn serialize_fingering() {
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
                post_events: vec![PostEvent::Fingering {
                    direction: Direction::Neutral,
                    digit: 3,
                }],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4-3"));
}

#[test]
fn serialize_named_articulation() {
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
                post_events: vec![PostEvent::NamedArticulation {
                    direction: Direction::Down,
                    name: "staccato".into(),
                }],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4_\\staccato"));
}

// ── Phase 13 serializer tests ────────────────────────────

#[test]
fn serialize_tremolo() {
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
                post_events: vec![PostEvent::Tremolo(32)],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4:32"));
}

#[test]
fn serialize_tremolo_bare() {
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
                post_events: vec![PostEvent::Tremolo(0)],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4:"));
    // Should NOT contain c4:0
    assert!(!output.contains("c4:0"));
}

// ── Phase 14 serializer tests ────────────────────────────

#[test]
fn serialize_string_number() {
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
                post_events: vec![PostEvent::StringNumber {
                    direction: Direction::Neutral,
                    number: 2,
                }],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4-\\2"));
}

#[test]
fn serialize_string_number_direction() {
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
                post_events: vec![PostEvent::StringNumber {
                    direction: Direction::Up,
                    number: 3,
                }],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4^\\3"));
}

// ── Phase 15 serializer tests ────────────────────────────

#[test]
fn serialize_tuplet_basic() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Tuplet {
            numerator: 3,
            denominator: 2,
            span_duration: None,
            body: Box::new(Music::Sequential(vec![Music::Note(NoteEvent {
                pitch: Pitch {
                    step: 'c',
                    alter: 0.0,
                    octave: 0,
                    force_accidental: false,
                    cautionary: false,
                    octave_check: None,
                },
                duration: Some(Duration {
                    base: 8,
                    dots: 0,
                    multipliers: vec![],
                }),
                pitched_rest: false,
                post_events: vec![],
            })])),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\tuplet 3/2 { c8 }"));
}

#[test]
fn serialize_tuplet_with_span() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Tuplet {
            numerator: 3,
            denominator: 2,
            span_duration: Some(Duration {
                base: 4,
                dots: 0,
                multipliers: vec![],
            }),
            body: Box::new(Music::Sequential(vec![])),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\tuplet 3/2 4 {  }"));
}

#[test]
fn serialize_chord_no_duration() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Chord(ChordEvent {
                pitches: vec![Pitch {
                    step: 'c',
                    alter: 0.0,
                    octave: 1,
                    force_accidental: false,
                    cautionary: false,
                    octave_check: None,
                }],
                duration: None,
                post_events: vec![],
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("<c'>"));
}

// ── Phase 16 serializer tests ────────────────────────────

fn make_note(step: char, base: u32) -> Music {
    Music::Note(NoteEvent {
        pitch: Pitch {
            step,
            alter: 0.0,
            octave: 0,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        },
        duration: Some(Duration {
            base,
            dots: 0,
            multipliers: vec![],
        }),
        pitched_rest: false,
        post_events: vec![],
    })
}

#[test]
fn serialize_grace() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Grace {
            body: Box::new(make_note('c', 16)),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\grace c16"));
}

#[test]
fn serialize_acciaccatura() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Acciaccatura {
            body: Box::new(make_note('d', 8)),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\acciaccatura d8"));
}

#[test]
fn serialize_appoggiatura() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Appoggiatura {
            body: Box::new(make_note('d', 8)),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\appoggiatura d8"));
}

#[test]
fn serialize_after_grace_no_fraction() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::AfterGrace {
            fraction: None,
            main: Box::new(make_note('c', 2)),
            grace: Box::new(Music::Sequential(vec![make_note('d', 16)])),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\afterGrace c2 { d16 }"));
}

#[test]
fn serialize_after_grace_with_fraction() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::AfterGrace {
            fraction: Some((3, 4)),
            main: Box::new(make_note('c', 2)),
            grace: Box::new(Music::Sequential(vec![make_note('d', 16)])),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\afterGrace 3/4 c2 { d16 }"));
}

// ── Repeat serialization (Phase 17) ─────────────────────────────────────

#[test]
fn serialize_repeat_volta_basic() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Repeat {
            repeat_type: RepeatType::Volta,
            count: 2,
            body: Box::new(Music::Sequential(vec![
                make_note('c', 4),
                make_note('d', 4),
            ])),
            alternatives: None,
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\repeat volta 2 { c4 d4 }"));
}

#[test]
fn serialize_repeat_with_alternatives() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Repeat {
            repeat_type: RepeatType::Volta,
            count: 2,
            body: Box::new(Music::Sequential(vec![make_note('c', 4)])),
            alternatives: Some(vec![
                Music::Sequential(vec![make_note('g', 2)]),
                Music::Sequential(vec![make_note('a', 2)]),
            ]),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\repeat volta 2 { c4 } \\alternative { { g2 } { a2 } }"));
}

#[test]
fn serialize_repeat_unfold() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Repeat {
            repeat_type: RepeatType::Unfold,
            count: 4,
            body: Box::new(Music::Sequential(vec![make_note('c', 8)])),
            alternatives: None,
        })],
    };
    let output = serialize(&file);
    assert!(output.contains("\\repeat unfold 4 { c8 }"));
}

// ── Bar check & bar line (Phase 18) ─────────────────────────────────────

#[test]
fn serialize_bar_check() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            make_note('c', 4),
            Music::BarCheck,
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("c4 |"), "got: {output}");
}

#[test]
fn serialize_bar_line_final() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::BarLine {
            bar_type: "|.".to_string(),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains(r#"\bar "|.""#), "got: {output}");
}

#[test]
fn serialize_bar_line_double() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::BarLine {
            bar_type: "||".to_string(),
        })],
    };
    let output = serialize(&file);
    assert!(output.contains(r#"\bar "||""#), "got: {output}");
}

// ── Chord repetition (Phase 19) ───────────────────────────────────────

#[test]
fn serialize_chord_rep_no_duration() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::ChordRepetition(
            ChordRepetitionEvent {
                duration: None,
                post_events: vec![],
            },
        ))],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "q");
}

#[test]
fn serialize_chord_rep_with_duration() {
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
    let output = serialize(&file);
    assert_eq!(output.trim(), "q4");
}

#[test]
fn serialize_chord_rep_with_post_events() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::ChordRepetition(
            ChordRepetitionEvent {
                duration: Some(Duration {
                    base: 4,
                    dots: 0,
                    multipliers: vec![],
                }),
                post_events: vec![PostEvent::Tie],
            },
        ))],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "q4~");
}

// ── Lyric serialization (Phase 20) ─────────────────────────────────

#[test]
fn serialize_lyricmode() {
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
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\lyricmode { hello world }");
}

#[test]
fn serialize_lyric_with_hyphen() {
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
                    post_events: vec![],
                }),
            ])),
        })],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\lyricmode { hel -- lo }");
}

#[test]
fn serialize_lyric_with_extender() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::LyricMode {
            body: Box::new(Music::Sequential(vec![Music::Lyric(LyricEvent {
                text: "hold".into(),
                duration: None,
                post_events: vec![PostEvent::LyricExtender],
            })])),
        })],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\lyricmode { hold __ }");
}

#[test]
fn serialize_lyricsto() {
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
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\lyricsto \"melody\" { la }");
}

#[test]
fn serialize_addlyrics() {
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
    let output = serialize(&file);
    assert_eq!(output.trim(), "{ c4 } \\addlyrics { do }");
}

#[test]
fn serialize_lyric_with_duration() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::LyricMode {
            body: Box::new(Music::Sequential(vec![Music::Lyric(LyricEvent {
                text: "word".into(),
                duration: Some(Duration {
                    base: 4,
                    dots: 1,
                    multipliers: vec![],
                }),
                post_events: vec![],
            })])),
        })],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\lyricmode { word4. }");
}

// ──────────────────────────────────────────────────────────────────
// Markup serialization (Phase 21)
// ──────────────────────────────────────────────────────────────────

#[test]
fn serialize_markup_string() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Markup(markup::Markup::String(
            "Hello".into(),
        ))],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\markup \"Hello\"");
}

#[test]
fn serialize_markup_word() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Markup(markup::Markup::Word(
            "Hello".into(),
        ))],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\markup Hello");
}

#[test]
fn serialize_markup_command() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Markup(markup::Markup::Command {
            name: "bold".into(),
            args: vec![markup::Markup::String("text".into())],
        })],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\markup \\bold \"text\"");
}

#[test]
fn serialize_markup_list() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Markup(markup::Markup::List(vec![
            markup::Markup::Word("Hello".into()),
            markup::Markup::Word("World".into()),
        ]))],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\markup { Hello World }");
}

#[test]
fn serialize_markuplist() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::MarkupList(markup::MarkupList {
            items: vec![
                markup::Markup::String("one".into()),
                markup::Markup::String("two".into()),
            ],
        })],
    };
    let output = serialize(&file);
    assert_eq!(output.trim(), "\\markuplist { \"one\" \"two\" }");
}

#[test]
fn serialize_markup_assignment() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Header(HeaderBlock {
            fields: vec![Assignment {
                name: "title".into(),
                value: AssignmentValue::Markup(markup::Markup::String("My Title".into())),
            }],
        })],
    };
    let output = serialize(&file);
    assert!(
        output.contains("title = \\markup \"My Title\""),
        "got: {output}"
    );
}

// ── Tempo ────────────────────────────────────────────────────────────────

#[test]
fn serialize_tempo_text_and_metronome() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Tempo(Tempo {
                text: Some(markup::Markup::Word("Allegro".into())),
                duration: Some(Duration {
                    base: 4,
                    dots: 0,
                    multipliers: vec![],
                }),
                bpm: Some(TempoRange::Single(120)),
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(
        output.contains("\\tempo \"Allegro\" 4 = 120"),
        "got: {output}"
    );
}

#[test]
fn serialize_tempo_metronome_only() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Tempo(Tempo {
                text: None,
                duration: Some(Duration {
                    base: 2,
                    dots: 0,
                    multipliers: vec![],
                }),
                bpm: Some(TempoRange::Single(60)),
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\tempo 2 = 60"), "got: {output}");
}

#[test]
fn serialize_tempo_text_only() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Tempo(Tempo {
                text: Some(markup::Markup::Word("Andante".into())),
                duration: None,
                bpm: None,
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\tempo \"Andante\""), "got: {output}");
}

#[test]
fn serialize_tempo_range() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Tempo(Tempo {
                text: Some(markup::Markup::Word("Vivace".into())),
                duration: Some(Duration {
                    base: 4,
                    dots: 1,
                    multipliers: vec![],
                }),
                bpm: Some(TempoRange::Range(132, 144)),
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(
        output.contains("\\tempo \"Vivace\" 4. = 132-144"),
        "got: {output}"
    );
}

// ── Mark ─────────────────────────────────────────────────────────────────

#[test]
fn serialize_mark_default() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Mark(Mark {
                label: MarkLabel::Default,
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\mark \\default"), "got: {output}");
}

#[test]
fn serialize_mark_number() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Mark(Mark {
                label: MarkLabel::Number(5),
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\mark 5"), "got: {output}");
}

#[test]
fn serialize_mark_string() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::Mark(Mark {
                label: MarkLabel::Markup(markup::Markup::Word("A".into())),
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\mark \"A\""), "got: {output}");
}

// ── TextMark ─────────────────────────────────────────────────────────────

#[test]
fn serialize_text_mark() {
    let file = LilyPondFile {
        version: None,
        items: vec![ToplevelExpression::Music(Music::Sequential(vec![
            Music::TextMark(TextMark {
                text: markup::Markup::Word("Fine".into()),
            }),
        ]))],
    };
    let output = serialize(&file);
    assert!(output.contains("\\textMark \"Fine\""), "got: {output}");
}

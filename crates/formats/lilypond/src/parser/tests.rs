use super::*;

#[test]
fn parse_version_only() {
    let ast = parse("\\version \"2.24.0\"").unwrap();
    assert_eq!(
        ast.version,
        Some(Version {
            version: "2.24.0".into()
        })
    );
    assert!(ast.items.is_empty());
}

#[test]
fn parse_minimal_score() {
    let ast = parse("\\version \"2.24.0\"\n\\score {\n  { c4 }\n}").unwrap();
    assert!(ast.version.is_some());
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Score(sb) => {
            assert_eq!(sb.items.len(), 1);
            match &sb.items[0] {
                ScoreItem::Music(Music::Sequential(items)) => {
                    assert_eq!(items.len(), 1);
                    match &items[0] {
                        Music::Note(n) => {
                            assert_eq!(n.pitch.step, 'c');
                            assert_eq!(n.pitch.alter, 0.0);
                            assert_eq!(n.duration.as_ref().unwrap().base, 4);
                        }
                        other => panic!("expected Note, got {other:?}"),
                    }
                }
                other => panic!("expected sequential music, got {other:?}"),
            }
        }
        other => panic!("expected score, got {other:?}"),
    }
}

#[test]
fn parse_score_with_layout_midi() {
    let input = r#"\version "2.24.0"
\score {
  { c4 }
  \layout { }
  \midi { }
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Score(sb) => {
            assert_eq!(sb.items.len(), 3);
            assert!(matches!(&sb.items[0], ScoreItem::Music(_)));
            assert!(matches!(&sb.items[1], ScoreItem::Layout(_)));
            assert!(matches!(&sb.items[2], ScoreItem::Midi(_)));
        }
        other => panic!("expected score, got {other:?}"),
    }
}

#[test]
fn parse_header_block() {
    let input = r#"\header {
  title = "My Piece"
  composer = "JS Bach"
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Header(hb) => {
            assert_eq!(hb.fields.len(), 2);
            assert_eq!(hb.fields[0].name, "title");
            assert_eq!(
                hb.fields[0].value,
                AssignmentValue::String("My Piece".into())
            );
            assert_eq!(hb.fields[1].name, "composer");
        }
        other => panic!("expected header, got {other:?}"),
    }
}

#[test]
fn parse_book_block() {
    let input = r#"\book {
  \header { title = "Sonata" }
  \score { { c4 } }
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Book(bb) => {
            assert_eq!(bb.items.len(), 2);
            assert!(matches!(&bb.items[0], BookItem::Header(_)));
            assert!(matches!(&bb.items[1], BookItem::Score(_)));
        }
        other => panic!("expected book, got {other:?}"),
    }
}

#[test]
fn parse_bookpart_block() {
    let input = r#"\bookpart {
  \header { title = "Movement 1" }
  \score { { c4 } }
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::BookPart(bp) => {
            assert_eq!(bp.items.len(), 2);
            assert!(matches!(&bp.items[0], BookPartItem::Header(_)));
            assert!(matches!(&bp.items[1], BookPartItem::Score(_)));
        }
        other => panic!("expected bookpart, got {other:?}"),
    }
}

#[test]
fn parse_toplevel_assignment() {
    let input = "melody = { c4 d4 e4 f4 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Assignment(a) => {
            assert_eq!(a.name, "melody");
            assert!(matches!(&a.value, AssignmentValue::Music(_)));
        }
        other => panic!("expected assignment, got {other:?}"),
    }
}

#[test]
fn parse_relative_music() {
    let input = "\\relative c' { c4 d e f }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Relative { pitch, body }) => {
            assert!(pitch.is_some());
            match pitch.as_deref() {
                Some(Music::Note(n)) => {
                    assert_eq!(n.pitch.step, 'c');
                    assert_eq!(n.pitch.octave, 1);
                }
                other => panic!("expected Note, got {other:?}"),
            }
            assert!(matches!(body.as_ref(), Music::Sequential(_)));
        }
        other => panic!("expected relative music, got {other:?}"),
    }
}

#[test]
fn parse_new_staff() {
    let input = "\\new Staff { c4 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ContextedMusic {
            keyword,
            context_type,
            name,
            ..
        }) => {
            assert_eq!(*keyword, ContextKeyword::New);
            assert_eq!(context_type, "Staff");
            assert!(name.is_none());
        }
        other => panic!("expected contexted music, got {other:?}"),
    }
}

#[test]
fn parse_fragment_score_minimal() {
    let input = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_score_minimal.ly"
    ))
    .expect("fixture file");
    let ast = parse(&input).unwrap();
    assert!(ast.version.is_some());
    assert_eq!(ast.items.len(), 1);
    assert!(matches!(&ast.items[0], ToplevelExpression::Score(_)));
}

#[test]
fn parse_simple_ly() {
    let input = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/simple.ly"
    ))
    .expect("fixture file");
    let ast = parse(&input).unwrap();
    assert_eq!(
        ast.version,
        Some(Version {
            version: "2.19.21".into()
        })
    );
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Relative { pitch, body }) => {
            assert!(pitch.is_none());
            match body.as_ref() {
                Music::Sequential(items) => assert_eq!(items.len(), 8),
                other => panic!("expected sequential, got {other:?}"),
            }
        }
        other => panic!("expected relative music, got {other:?}"),
    }
}

#[test]
fn roundtrip_simple_ly() {
    let input = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/simple.ly"
    ))
    .expect("fixture file");
    let ast = parse(&input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_score_minimal() {
    let input = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_score_minimal.ly"
    ))
    .expect("fixture file");
    let ast = parse(&input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

// ── Phase 3 tests ───────────────────────────────────────────────

#[test]
fn parse_note_with_pitch() {
    let ast = parse("{ c }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.pitch.step, 'c');
                assert_eq!(n.pitch.alter, 0.0);
                assert_eq!(n.pitch.octave, 0);
                assert!(!n.pitch.force_accidental);
                assert!(!n.pitch.cautionary);
                assert!(n.duration.is_none());
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_note_with_accidental_octave_duration() {
    let ast = parse("{ cis''4. }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.pitch.step, 'c');
                assert_eq!(n.pitch.alter, 1.0);
                assert_eq!(n.pitch.octave, 2);
                let dur = n.duration.as_ref().unwrap();
                assert_eq!(dur.base, 4);
                assert_eq!(dur.dots, 1);
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_note_force_accidental() {
    let ast = parse("{ cis! }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert!(n.pitch.force_accidental);
                assert!(!n.pitch.cautionary);
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_note_cautionary_accidental() {
    let ast = parse("{ bes? }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert!(!n.pitch.force_accidental);
                assert!(n.pitch.cautionary);
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_rest() {
    let ast = parse("{ r4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Rest(r) => {
                let dur = r.duration.as_ref().unwrap();
                assert_eq!(dur.base, 4);
                assert_eq!(dur.dots, 0);
            }
            other => panic!("expected Rest, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_skip() {
    let ast = parse("{ s2. }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Skip(s) => {
                let dur = s.duration.as_ref().unwrap();
                assert_eq!(dur.base, 2);
                assert_eq!(dur.dots, 1);
            }
            other => panic!("expected Skip, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_multi_measure_rest() {
    let ast = parse("{ R1 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::MultiMeasureRest(r) => {
                let dur = r.duration.as_ref().unwrap();
                assert_eq!(dur.base, 1);
            }
            other => panic!("expected MultiMeasureRest, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_duration_multiplier() {
    let ast = parse("{ R1*4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::MultiMeasureRest(r) => {
                let dur = r.duration.as_ref().unwrap();
                assert_eq!(dur.base, 1);
                assert_eq!(dur.multipliers, vec![(4, 1)]);
            }
            other => panic!("expected MultiMeasureRest, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_duration_fraction_multiplier() {
    let ast = parse("{ c4*2/3 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                let dur = n.duration.as_ref().unwrap();
                assert_eq!(dur.base, 4);
                assert_eq!(dur.multipliers, vec![(2, 3)]);
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_pitched_rest() {
    let ast = parse("{ c4\\rest }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert!(n.pitched_rest);
                assert_eq!(n.pitch.step, 'c');
            }
            other => panic!("expected Note (pitched rest), got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_rest_no_duration() {
    let ast = parse("{ r }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Rest(r) => {
                assert!(r.duration.is_none());
            }
            other => panic!("expected Rest, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_note_no_duration() {
    let ast = parse("{ c }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert!(n.duration.is_none());
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_multiple_notes() {
    let ast = parse("{ c4 d8 e16 f2 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 4);
            match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'c');
                    assert_eq!(n.duration.as_ref().unwrap().base, 4);
                }
                other => panic!("expected Note, got {other:?}"),
            }
            match &items[1] {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'd');
                    assert_eq!(n.duration.as_ref().unwrap().base, 8);
                }
                other => panic!("expected Note, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_octave_down() {
    let ast = parse("{ c,, }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.pitch.octave, -2);
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

// ── Phase 3 fixture roundtrip tests ──────────────────────────────

fn roundtrip_fixture(name: &str) {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/"
    )
    .to_string()
        + name;
    let input = std::fs::read_to_string(path).expect("fixture file");
    let ast = parse(&input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2, "AST mismatch after roundtrip of {name}");
}

#[test]
fn roundtrip_fragment_pitches() {
    roundtrip_fixture("fragment_pitches.ly");
}

#[test]
fn roundtrip_fragment_durations() {
    roundtrip_fixture("fragment_durations.ly");
}

#[test]
fn roundtrip_fragment_rests() {
    roundtrip_fixture("fragment_rests.ly");
}

// ── Phase 4 tests ───────────────────────────────────────────────

#[test]
fn parse_nested_sequential_simultaneous() {
    let ast = parse("{ << { c4 d4 } { e4 f4 } >> }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(outer)) => {
            assert_eq!(outer.len(), 1);
            match &outer[0] {
                Music::Simultaneous(voices) => {
                    assert_eq!(voices.len(), 2);
                    match &voices[0] {
                        Music::Sequential(items) => {
                            assert_eq!(items.len(), 2);
                            assert!(matches!(&items[0], Music::Note(n) if n.pitch.step == 'c'));
                            assert!(matches!(&items[1], Music::Note(n) if n.pitch.step == 'd'));
                        }
                        other => panic!("expected sequential, got {other:?}"),
                    }
                    match &voices[1] {
                        Music::Sequential(items) => {
                            assert_eq!(items.len(), 2);
                            assert!(matches!(&items[0], Music::Note(n) if n.pitch.step == 'e'));
                            assert!(matches!(&items[1], Music::Note(n) if n.pitch.step == 'f'));
                        }
                        other => panic!("expected sequential, got {other:?}"),
                    }
                }
                other => panic!("expected simultaneous, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_explicit_sequential_keyword() {
    let ast = parse("\\sequential { c4 d4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 2);
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_explicit_simultaneous_keyword() {
    let ast = parse("\\simultaneous { { c4 } { d4 } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Simultaneous(items)) => {
            assert_eq!(items.len(), 2);
        }
        other => panic!("expected simultaneous, got {other:?}"),
    }
}

#[test]
fn parse_voice_separator_backslash() {
    let ast = parse("<< { c4 d4 } \\\\ { e4 f4 } >>").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Simultaneous(voices)) => {
            assert_eq!(voices.len(), 2);
            assert!(matches!(&voices[0], Music::Sequential(_)));
            assert!(matches!(&voices[1], Music::Sequential(_)));
        }
        other => panic!("expected simultaneous, got {other:?}"),
    }
}

#[test]
fn parse_deeply_nested_music() {
    let ast = parse("{ { { c4 } } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(l1)) => {
            assert_eq!(l1.len(), 1);
            match &l1[0] {
                Music::Sequential(l2) => {
                    assert_eq!(l2.len(), 1);
                    match &l2[0] {
                        Music::Sequential(l3) => {
                            assert_eq!(l3.len(), 1);
                            assert!(matches!(&l3[0], Music::Note(_)));
                        }
                        other => panic!("expected sequential, got {other:?}"),
                    }
                }
                other => panic!("expected sequential, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_empty_sequential() {
    let ast = parse("{ }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert!(items.is_empty());
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_empty_simultaneous() {
    let ast = parse("<< >>").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Simultaneous(items)) => {
            assert!(items.is_empty());
        }
        other => panic!("expected simultaneous, got {other:?}"),
    }
}

#[test]
fn parse_simultaneous_with_notes() {
    // Notes directly inside << >> (no inner braces)
    let ast = parse("<< c4 d4 e4 >>").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Simultaneous(items)) => {
            assert_eq!(items.len(), 3);
        }
        other => panic!("expected simultaneous, got {other:?}"),
    }
}

#[test]
fn roundtrip_fragment_sequential_simultaneous() {
    roundtrip_fixture("fragment_sequential_simultaneous.ly");
}

// ── Phase 5 tests ───────────────────────────────────────────────

#[test]
fn parse_new_staff_with_name() {
    let ast = parse("\\new Staff = \"violin\" { c4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ContextedMusic {
            keyword,
            context_type,
            name,
            ..
        }) => {
            assert_eq!(*keyword, ContextKeyword::New);
            assert_eq!(context_type, "Staff");
            assert_eq!(name.as_deref(), Some("violin"));
        }
        other => panic!("expected contexted music, got {other:?}"),
    }
}

#[test]
fn parse_context_staff() {
    let ast = parse("\\context Staff = \"main\" { c4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ContextedMusic {
            keyword,
            context_type,
            name,
            ..
        }) => {
            assert_eq!(*keyword, ContextKeyword::Context);
            assert_eq!(context_type, "Staff");
            assert_eq!(name.as_deref(), Some("main"));
        }
        other => panic!("expected contexted music, got {other:?}"),
    }
}

#[test]
fn parse_new_with_block() {
    let input = r#"\new Staff \with {
  \consists "Span_arpeggio_engraver"
  instrumentName = "Piano"
} { c4 }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ContextedMusic {
            keyword,
            context_type,
            with_block,
            ..
        }) => {
            assert_eq!(*keyword, ContextKeyword::New);
            assert_eq!(context_type, "Staff");
            let items = with_block.as_ref().unwrap();
            assert_eq!(items.len(), 2);
            assert!(
                matches!(&items[0], ContextModItem::Consists(s) if s == "Span_arpeggio_engraver")
            );
            assert!(
                matches!(&items[1], ContextModItem::Assignment(a) if a.name == "instrumentName")
            );
        }
        other => panic!("expected contexted music, got {other:?}"),
    }
}

#[test]
fn parse_context_change() {
    let ast = parse("{ c4 \\change Staff = \"other\" d4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 3);
            match &items[1] {
                Music::ContextChange { context_type, name } => {
                    assert_eq!(context_type, "Staff");
                    assert_eq!(name, "other");
                }
                other => panic!("expected ContextChange, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_nested_new_staff() {
    let input = r#"\new StaffGroup << \new Staff { c4 } \new Staff { d4 } >>"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ContextedMusic {
            keyword,
            context_type,
            music,
            ..
        }) => {
            assert_eq!(*keyword, ContextKeyword::New);
            assert_eq!(context_type, "StaffGroup");
            match music.as_ref() {
                Music::Simultaneous(items) => {
                    assert_eq!(items.len(), 2);
                    assert!(
                        matches!(&items[0], Music::ContextedMusic { context_type, .. } if context_type == "Staff")
                    );
                    assert!(
                        matches!(&items[1], Music::ContextedMusic { context_type, .. } if context_type == "Staff")
                    );
                }
                other => panic!("expected simultaneous, got {other:?}"),
            }
        }
        other => panic!("expected contexted music, got {other:?}"),
    }
}

#[test]
fn parse_piano_staff() {
    let input = r#"\new PianoStaff <<
  \new Staff = "right" { c'4 }
  \new Staff = "left" { c4 }
>>"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ContextedMusic {
            keyword,
            context_type,
            ..
        }) => {
            assert_eq!(*keyword, ContextKeyword::New);
            assert_eq!(context_type, "PianoStaff");
        }
        other => panic!("expected PianoStaff, got {other:?}"),
    }
}

#[test]
fn roundtrip_context_fixture() {
    roundtrip_fixture("fragment_contexts.ly");
}

// ── Phase 6 tests ───────────────────────────────────────────────

#[test]
fn parse_clef_string() {
    let ast = parse("{ \\clef \"treble\" }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Clef(c) => assert_eq!(c.name, "treble"),
            other => panic!("expected Clef, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_clef_bare_symbol() {
    let ast = parse("{ \\clef bass }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Clef(c) => assert_eq!(c.name, "bass"),
            other => panic!("expected Clef, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_clef_transposed() {
    let ast = parse("{ \\clef \"G_8\" }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Clef(c) => assert_eq!(c.name, "G_8"),
            other => panic!("expected Clef, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_key_signature() {
    let ast = parse("{ \\key d \\major }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::KeySignature(ks) => {
                assert_eq!(ks.pitch.step, 'd');
                assert_eq!(ks.pitch.alter, 0.0);
                assert_eq!(ks.mode, Mode::Major);
            }
            other => panic!("expected KeySignature, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_key_signature_flat() {
    let ast = parse("{ \\key bes \\minor }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::KeySignature(ks) => {
                assert_eq!(ks.pitch.step, 'b');
                assert_eq!(ks.pitch.alter, -1.0);
                assert_eq!(ks.mode, Mode::Minor);
            }
            other => panic!("expected KeySignature, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_key_signature_modes() {
    for (mode_str, mode) in [
        ("major", Mode::Major),
        ("minor", Mode::Minor),
        ("dorian", Mode::Dorian),
        ("phrygian", Mode::Phrygian),
        ("lydian", Mode::Lydian),
        ("mixolydian", Mode::Mixolydian),
        ("aeolian", Mode::Aeolian),
        ("locrian", Mode::Locrian),
        ("ionian", Mode::Ionian),
    ] {
        let input = format!("{{ \\key c \\{mode_str} }}");
        let ast = parse(&input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::KeySignature(ks) => assert_eq!(ks.mode, mode, "mode: {mode_str}"),
                other => panic!("expected KeySignature for {mode_str}, got {other:?}"),
            },
            other => panic!("expected sequential for {mode_str}, got {other:?}"),
        }
    }
}

#[test]
fn parse_time_signature_simple() {
    let ast = parse("{ \\time 4/4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::TimeSignature(ts) => {
                assert_eq!(ts.numerators, vec![4]);
                assert_eq!(ts.denominator, 4);
            }
            other => panic!("expected TimeSignature, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_time_signature_compound() {
    let ast = parse("{ \\time 3/4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::TimeSignature(ts) => {
                assert_eq!(ts.numerators, vec![3]);
                assert_eq!(ts.denominator, 4);
            }
            other => panic!("expected TimeSignature, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_time_signature_additive() {
    let ast = parse("{ \\time 2+3/8 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::TimeSignature(ts) => {
                assert_eq!(ts.numerators, vec![2, 3]);
                assert_eq!(ts.denominator, 8);
            }
            other => panic!("expected TimeSignature, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_time_signature_triple_additive() {
    let ast = parse("{ \\time 3+3+2/8 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::TimeSignature(ts) => {
                assert_eq!(ts.numerators, vec![3, 3, 2]);
                assert_eq!(ts.denominator, 8);
            }
            other => panic!("expected TimeSignature, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_clef_key_time_sequence() {
    let input = "{ \\clef \"treble\" \\key d \\major \\time 4/4 c4 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 4);
            assert!(matches!(&items[0], Music::Clef(_)));
            assert!(matches!(&items[1], Music::KeySignature(_)));
            assert!(matches!(&items[2], Music::TimeSignature(_)));
            assert!(matches!(&items[3], Music::Note(_)));
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn roundtrip_fragment_clef_key_time() {
    roundtrip_fixture("fragment_clef_key_time.ly");
}

// ── Phase 7 tests ───────────────────────────────────────────────

#[test]
fn parse_transpose() {
    let ast = parse("\\transpose c d { c4 d e f }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Transpose { from, to, body }) => {
            match from.as_ref() {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'c');
                    assert_eq!(n.pitch.octave, 0);
                }
                other => panic!("expected Note for from, got {other:?}"),
            }
            match to.as_ref() {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'd');
                    assert_eq!(n.pitch.octave, 0);
                }
                other => panic!("expected Note for to, got {other:?}"),
            }
            assert!(matches!(body.as_ref(), Music::Sequential(_)));
        }
        other => panic!("expected Transpose, got {other:?}"),
    }
}

#[test]
fn parse_transpose_with_octave() {
    let ast = parse("\\transpose c' d'' { c4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Transpose { from, to, .. }) => {
            match from.as_ref() {
                Music::Note(n) => assert_eq!(n.pitch.octave, 1),
                other => panic!("expected Note, got {other:?}"),
            }
            match to.as_ref() {
                Music::Note(n) => assert_eq!(n.pitch.octave, 2),
                other => panic!("expected Note, got {other:?}"),
            }
        }
        other => panic!("expected Transpose, got {other:?}"),
    }
}

#[test]
fn parse_transpose_with_accidentals() {
    let ast = parse("\\transpose bes ees' { c4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Transpose { from, to, .. }) => {
            match from.as_ref() {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'b');
                    assert_eq!(n.pitch.alter, -1.0);
                }
                other => panic!("expected Note, got {other:?}"),
            }
            match to.as_ref() {
                Music::Note(n) => {
                    assert_eq!(n.pitch.step, 'e');
                    assert_eq!(n.pitch.alter, -1.0);
                    assert_eq!(n.pitch.octave, 1);
                }
                other => panic!("expected Note, got {other:?}"),
            }
        }
        other => panic!("expected Transpose, got {other:?}"),
    }
}

#[test]
fn parse_octave_check() {
    let ast = parse("{ c='4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.pitch.step, 'c');
                assert_eq!(n.pitch.octave_check, Some(1));
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_octave_check_down() {
    let ast = parse("{ f=,,4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.pitch.step, 'f');
                assert_eq!(n.pitch.octave_check, Some(-2));
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_octave_check_no_marks() {
    // `=` with no following octave marks means octave 0
    let ast = parse("{ c=4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.pitch.octave_check, Some(0));
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_no_octave_check() {
    let ast = parse("{ c'4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert!(n.pitch.octave_check.is_none());
            }
            other => panic!("expected Note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_relative_no_pitch() {
    let ast = parse("\\relative { c'4 d e f }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Relative { pitch, body }) => {
            assert!(pitch.is_none());
            assert!(matches!(body.as_ref(), Music::Sequential(_)));
        }
        other => panic!("expected relative, got {other:?}"),
    }
}

#[test]
fn roundtrip_fragment_relative_transpose() {
    roundtrip_fixture("fragment_relative_transpose.ly");
}

// ── Phase 8 chord parser tests ────────────────────────────────────

#[test]
fn parse_chord_basic() {
    let ast = parse("{ <c e g>4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                Music::Chord(chord) => {
                    assert_eq!(chord.pitches.len(), 3);
                    assert_eq!(chord.pitches[0].step, 'c');
                    assert_eq!(chord.pitches[1].step, 'e');
                    assert_eq!(chord.pitches[2].step, 'g');
                    assert_eq!(chord.duration.as_ref().unwrap().base, 4);
                }
                other => panic!("expected Chord, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_accidentals() {
    let ast = parse("{ <c es g>2. }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(chord) => {
                assert_eq!(chord.pitches.len(), 3);
                assert_eq!(chord.pitches[1].step, 'e');
                assert_eq!(chord.pitches[1].alter, -1.0); // es = E-flat
                assert_eq!(chord.duration.as_ref().unwrap().base, 2);
                assert_eq!(chord.duration.as_ref().unwrap().dots, 1);
            }
            other => panic!("expected Chord, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_octave_marks() {
    let ast = parse("{ <d' fis' a'>8 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(chord) => {
                assert_eq!(chord.pitches.len(), 3);
                assert_eq!(chord.pitches[0].step, 'd');
                assert_eq!(chord.pitches[0].octave, 1);
                assert_eq!(chord.pitches[1].step, 'f');
                assert_eq!(chord.pitches[1].alter, 1.0); // fis = F-sharp
                assert_eq!(chord.pitches[1].octave, 1);
                assert_eq!(chord.pitches[2].step, 'a');
                assert_eq!(chord.pitches[2].octave, 1);
                assert_eq!(chord.duration.as_ref().unwrap().base, 8);
            }
            other => panic!("expected Chord, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_force_cautionary() {
    let ast = parse("{ <cis''! e''?>4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(chord) => {
                assert_eq!(chord.pitches.len(), 2);
                assert!(chord.pitches[0].force_accidental);
                assert!(!chord.pitches[0].cautionary);
                assert!(!chord.pitches[1].force_accidental);
                assert!(chord.pitches[1].cautionary);
            }
            other => panic!("expected Chord, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_no_duration() {
    let ast = parse("{ <c e g> }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(chord) => {
                assert_eq!(chord.pitches.len(), 3);
                assert!(chord.duration.is_none());
            }
            other => panic!("expected Chord, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_single_pitch() {
    let ast = parse("{ <c>4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(chord) => {
                assert_eq!(chord.pitches.len(), 1);
                assert_eq!(chord.pitches[0].step, 'c');
            }
            other => panic!("expected Chord, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_mixed_with_notes() {
    let ast = parse("{ c4 <c e g>4 d4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 3);
            assert!(matches!(&items[0], Music::Note(_)));
            assert!(matches!(&items[1], Music::Chord(_)));
            assert!(matches!(&items[2], Music::Note(_)));
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn roundtrip_chord_basic() {
    let input = "{ <c e g>4 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_chord_complex() {
    let input = "{ <c es g>2. <d' fis' a'>8 <bes, d f>1 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_chords() {
    roundtrip_fixture("fragment_chords.ly");
}

// ── Phase 15 tests: tuplets ──────────────────────────────────────

#[test]
fn parse_tuplet_basic() {
    let ast = parse("{ \\tuplet 3/2 { c8 d e } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 1);
            match &items[0] {
                Music::Tuplet {
                    numerator,
                    denominator,
                    span_duration,
                    body,
                } => {
                    assert_eq!(*numerator, 3);
                    assert_eq!(*denominator, 2);
                    assert!(span_duration.is_none());
                    match body.as_ref() {
                        Music::Sequential(inner) => assert_eq!(inner.len(), 3),
                        other => panic!("expected sequential, got {other:?}"),
                    }
                }
                other => panic!("expected tuplet, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tuplet_with_span_duration() {
    let ast = parse("{ \\tuplet 3/2 4 { c8 d e f g a } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Tuplet {
                numerator,
                denominator,
                span_duration,
                body,
            } => {
                assert_eq!(*numerator, 3);
                assert_eq!(*denominator, 2);
                let dur = span_duration.as_ref().unwrap();
                assert_eq!(dur.base, 4);
                assert_eq!(dur.dots, 0);
                match body.as_ref() {
                    Music::Sequential(inner) => assert_eq!(inner.len(), 6),
                    other => panic!("expected sequential, got {other:?}"),
                }
            }
            other => panic!("expected tuplet, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tuplet_5_4() {
    let ast = parse("{ \\tuplet 5/4 { c16 d e f g } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Tuplet {
                numerator,
                denominator,
                ..
            } => {
                assert_eq!(*numerator, 5);
                assert_eq!(*denominator, 4);
            }
            other => panic!("expected tuplet, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tuplet_nested() {
    let ast = parse("{ \\tuplet 3/2 { \\tuplet 3/2 { c32 d e } f16 g } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            match &items[0] {
                Music::Tuplet { body, .. } => match body.as_ref() {
                    Music::Sequential(inner) => {
                        assert_eq!(inner.len(), 3); // inner tuplet, f, g
                        assert!(matches!(&inner[0], Music::Tuplet { .. }));
                    }
                    other => panic!("expected sequential, got {other:?}"),
                },
                other => panic!("expected tuplet, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_times_basic() {
    // \times 2/3 is equivalent to \tuplet 3/2
    let ast = parse("{ \\times 2/3 { c8 d e } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            match &items[0] {
                Music::Tuplet {
                    numerator,
                    denominator,
                    span_duration,
                    body,
                } => {
                    // \times 2/3 → stored as \tuplet 3/2 (inverted)
                    assert_eq!(*numerator, 3);
                    assert_eq!(*denominator, 2);
                    assert!(span_duration.is_none());
                    match body.as_ref() {
                        Music::Sequential(inner) => assert_eq!(inner.len(), 3),
                        other => panic!("expected sequential, got {other:?}"),
                    }
                }
                other => panic!("expected tuplet, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tuplet_with_dotted_span() {
    let ast = parse("{ \\tuplet 3/2 4. { c8 d e } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Tuplet { span_duration, .. } => {
                let dur = span_duration.as_ref().unwrap();
                assert_eq!(dur.base, 4);
                assert_eq!(dur.dots, 1);
            }
            other => panic!("expected tuplet, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tuplet_in_context() {
    // Tuplet inside \new Staff
    let ast = parse("\\new Staff { \\tuplet 3/2 { c8 d e } }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ContextedMusic { music, .. }) => match music.as_ref() {
            Music::Sequential(items) => {
                assert_eq!(items.len(), 1);
                assert!(matches!(&items[0], Music::Tuplet { .. }));
            }
            other => panic!("expected sequential, got {other:?}"),
        },
        other => panic!("expected context, got {other:?}"),
    }
}

#[test]
fn roundtrip_tuplet_basic() {
    let input = "{ \\tuplet 3/2 { c8 d e } }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_tuplet_with_span() {
    let input = "{ \\tuplet 3/2 4 { c8 d e f g a } }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_tuplet_nested() {
    let input = "{ \\tuplet 3/2 { \\tuplet 3/2 { c32 d e } f16 g } }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_tuplets() {
    roundtrip_fixture("fragment_tuplets.ly");
}

//! Parser tests for post-events: ties, slurs, beams, dynamics, hairpins (Phases 9–11).

use super::*;

fn roundtrip_fixture(name: &str) {
    let path = format!(
        "{}/tests/fixtures/lilypond/{name}",
        env!("CARGO_MANIFEST_DIR").replace("/crates/formats/lilypond", "")
    );
    let input = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let ast = parse(&input).unwrap_or_else(|e| panic!("parse {name}: {e}"));
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap_or_else(|e| panic!("re-parse {name}: {e}"));
    assert_eq!(ast, ast2, "roundtrip mismatch for {name}");
}

// ── Phase 9 parser tests ──────────────────────────────────────

#[test]
fn parse_tie() {
    let ast = parse("{ c4~ c4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 2);
            match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::Tie]);
                }
                other => panic!("expected note, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_slur() {
    let ast = parse("{ c4( d4 e4) }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 3);
            match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::SlurStart]);
                }
                other => panic!("expected note, got {other:?}"),
            }
            match &items[2] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::SlurEnd]);
                }
                other => panic!("expected note, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_phrasing_slur() {
    let ast = parse("{ c4\\( d4 e4\\) }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 3);
            match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::PhrasingSlurStart]);
                }
                other => panic!("expected note, got {other:?}"),
            }
            match &items[2] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::PhrasingSlurEnd]);
                }
                other => panic!("expected note, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_multiple_post_events() {
    let ast = parse("{ c4~( d4) }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.post_events, vec![PostEvent::Tie, PostEvent::SlurStart]);
            }
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_with_tie() {
    let ast = parse("{ <c e g>4~ <c e g>4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(c) => {
                assert_eq!(c.post_events, vec![PostEvent::Tie]);
            }
            other => panic!("expected chord, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_rest_with_slur() {
    // Unusual but valid in LilyPond grammar
    let ast = parse("{ r4( c4) }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Rest(r) => {
                assert_eq!(r.post_events, vec![PostEvent::SlurStart]);
            }
            other => panic!("expected rest, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn roundtrip_tie() {
    let input = "{ c4~ c4 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_slur() {
    let input = "{ c4( d4 e4) }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_phrasing_slur() {
    let input = "{ c4\\( d4 e4\\) }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_ties_slurs() {
    roundtrip_fixture("fragment_ties_slurs.ly");
}

// ── Phase 10 beam parser tests ──────────────────────────────────

#[test]
fn parse_beam_start_end() {
    let ast = parse("{ c8[ d e f] }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 4);
            match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::BeamStart]);
                }
                other => panic!("expected note, got {other:?}"),
            }
            match &items[3] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::BeamEnd]);
                }
                other => panic!("expected note, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_beam_with_slur() {
    let ast = parse("{ c8[( d e] f) }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 4);
            match &items[0] {
                Music::Note(n) => {
                    assert_eq!(
                        n.post_events,
                        vec![PostEvent::BeamStart, PostEvent::SlurStart]
                    );
                }
                other => panic!("expected note, got {other:?}"),
            }
            match &items[2] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::BeamEnd]);
                }
                other => panic!("expected note, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_autobeam_on() {
    let ast = parse("{ \\autoBeamOn c8 d e f }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert!(matches!(&items[0], Music::AutoBeamOn));
            assert!(matches!(&items[1], Music::Note(_)));
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_autobeam_off() {
    let ast = parse("{ \\autoBeamOff c8[ d e f] }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert!(matches!(&items[0], Music::AutoBeamOff));
            assert!(matches!(&items[1], Music::Note(_)));
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_with_beam() {
    let ast = parse("{ <c e>8[ <d f>] }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 2);
            match &items[0] {
                Music::Chord(c) => {
                    assert_eq!(c.post_events, vec![PostEvent::BeamStart]);
                }
                other => panic!("expected chord, got {other:?}"),
            }
            match &items[1] {
                Music::Chord(c) => {
                    assert_eq!(c.post_events, vec![PostEvent::BeamEnd]);
                }
                other => panic!("expected chord, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn roundtrip_beam() {
    let input = "{ c8[ d e f] }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_autobeam() {
    let input = "{ \\autoBeamOff c8[ d e f] \\autoBeamOn }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_beams() {
    roundtrip_fixture("fragment_beams.ly");
}

// ── Phase 11 (dynamics & hairpins) ─────────────────────────────────

#[test]
fn parse_dynamic_f() {
    let ast = parse("{ c4\\f }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => assert_eq!(n.post_events, vec![PostEvent::Dynamic("f".into())]),
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_crescendo_hairpin() {
    let ast = parse("{ c4\\< d e\\! }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            match &items[0] {
                Music::Note(n) => assert_eq!(n.post_events, vec![PostEvent::Crescendo]),
                o => panic!("expected note, got {o:?}"),
            }
            match &items[2] {
                Music::Note(n) => assert_eq!(n.post_events, vec![PostEvent::HairpinEnd]),
                o => panic!("expected note, got {o:?}"),
            }
        }
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_decrescendo_hairpin() {
    let ast = parse("{ c4\\> d\\! }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            match &items[0] {
                Music::Note(n) => assert_eq!(n.post_events, vec![PostEvent::Decrescendo]),
                o => panic!("expected note, got {o:?}"),
            }
            match &items[1] {
                Music::Note(n) => assert_eq!(n.post_events, vec![PostEvent::HairpinEnd]),
                o => panic!("expected note, got {o:?}"),
            }
        }
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_multiple_dynamics() {
    let ast = parse("{ c4\\f\\< d e\\!\\ff }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            match &items[0] {
                Music::Note(n) => {
                    assert_eq!(
                        n.post_events,
                        vec![PostEvent::Dynamic("f".into()), PostEvent::Crescendo]
                    );
                }
                o => panic!("expected note, got {o:?}"),
            }
            match &items[2] {
                Music::Note(n) => {
                    assert_eq!(
                        n.post_events,
                        vec![PostEvent::HairpinEnd, PostEvent::Dynamic("ff".into())]
                    );
                }
                o => panic!("expected note, got {o:?}"),
            }
        }
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_all_standard_dynamics() {
    for &dyn_name in note::KNOWN_DYNAMICS {
        let input = format!("{{ c4\\{dyn_name} }}");
        let ast = parse(&input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
                Music::Note(n) => {
                    assert_eq!(n.post_events, vec![PostEvent::Dynamic(dyn_name.into())]);
                }
                o => panic!("expected note for \\{dyn_name}, got {o:?}"),
            },
            o => panic!("expected sequential for \\{dyn_name}, got {o:?}"),
        }
    }
}

#[test]
fn parse_dynamics_on_chord() {
    let ast = parse("{ <c e g>4\\sfz }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(c) => {
                assert_eq!(c.post_events, vec![PostEvent::Dynamic("sfz".into())]);
            }
            o => panic!("expected chord, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_dynamics_on_rest() {
    let ast = parse("{ r4\\p }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Rest(r) => {
                assert_eq!(r.post_events, vec![PostEvent::Dynamic("p".into())]);
            }
            o => panic!("expected rest, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn roundtrip_dynamics() {
    let input = "{ c4\\f d\\p e\\< f g\\! a\\ff }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_hairpins() {
    let input = "{ c4\\< d e\\!\\ff b\\> c' d'\\! }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_dynamics() {
    roundtrip_fixture("fragment_dynamics.ly");
}

// ── Phase 12 (articulations & script abbreviations) ─────────────

#[test]
fn parse_staccato_abbreviation() {
    let ast = parse("{ c4-. }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Dot,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_accent_abbreviation() {
    let ast = parse("{ c4-> }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Accent,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_tenuto_abbreviation() {
    let ast = parse("{ c4-- }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Dash,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_marcato_abbreviation() {
    let ast = parse("{ c4-^ }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Marcato,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_stopped_abbreviation() {
    let ast = parse("{ c4-+ }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Stopped,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_staccatissimo_abbreviation() {
    let ast = parse("{ c4-! }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Staccatissimo,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_portato_abbreviation() {
    let ast = parse("{ c4-_ }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Portato,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_direction_up_staccato() {
    let ast = parse("{ c4^. }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Up,
                        script: ScriptAbbreviation::Dot,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_direction_down_tenuto() {
    let ast = parse("{ c4_- }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Down,
                        script: ScriptAbbreviation::Dash,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_fingering_neutral() {
    let ast = parse("{ c4-1 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Fingering {
                        direction: Direction::Neutral,
                        digit: 1,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_fingering_up() {
    let ast = parse("{ c4^3 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Fingering {
                        direction: Direction::Up,
                        digit: 3,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_fingering_down() {
    let ast = parse("{ c4_4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::Fingering {
                        direction: Direction::Down,
                        digit: 4,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_named_articulation() {
    let ast = parse("{ c4-\\staccato }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "staccato".into(),
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_multiple_articulations() {
    let ast = parse("{ c4-. -> }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![
                        PostEvent::Articulation {
                            direction: Direction::Neutral,
                            script: ScriptAbbreviation::Dot,
                        },
                        PostEvent::Articulation {
                            direction: Direction::Neutral,
                            script: ScriptAbbreviation::Accent,
                        },
                    ]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_articulation_with_fingering() {
    let ast = parse("{ c4-. -3 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![
                        PostEvent::Articulation {
                            direction: Direction::Neutral,
                            script: ScriptAbbreviation::Dot,
                        },
                        PostEvent::Fingering {
                            direction: Direction::Neutral,
                            digit: 3,
                        },
                    ]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_chord_with_articulation() {
    let ast = parse("{ <c e g>4-. }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(c) => {
                assert_eq!(
                    c.post_events,
                    vec![PostEvent::Articulation {
                        direction: Direction::Neutral,
                        script: ScriptAbbreviation::Dot,
                    }]
                );
            }
            o => panic!("expected chord, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_articulation_with_slur() {
    let ast = parse("{ c4-.(  d4) }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![
                        PostEvent::Articulation {
                            direction: Direction::Neutral,
                            script: ScriptAbbreviation::Dot,
                        },
                        PostEvent::SlurStart,
                    ]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn roundtrip_articulations() {
    let input = "{ c4-. d4-> e4-+ f4-- g4-! a4-_ b4-^ }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_directed_articulations() {
    let input = "{ c4^. d4^> e4_. f4_- }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fingerings() {
    let input = "{ c4-1 d4-2 e4^3 f4_4 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_named_articulation() {
    let input = "{ c4-\\staccato d4^\\accent }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_articulations() {
    roundtrip_fixture("fragment_articulations.ly");
}

// ── Phase 13 parser tests: Ornaments & Tremolos ─────────────

#[test]
fn parse_trill() {
    let ast = parse("{ c4\\trill }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "trill".into(),
                    }]
                );
            }
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_mordent() {
    let ast = parse("{ c4\\mordent }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "mordent".into(),
                    }]
                );
            }
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_turn_and_prall() {
    let ast = parse("{ c4\\turn d4\\prall }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            match &items[0] {
                Music::Note(n) => assert_eq!(
                    n.post_events[0],
                    PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "turn".into(),
                    }
                ),
                other => panic!("expected note, got {other:?}"),
            }
            match &items[1] {
                Music::Note(n) => assert_eq!(
                    n.post_events[0],
                    PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "prall".into(),
                    }
                ),
                other => panic!("expected note, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_fermata() {
    let ast = parse("{ c4\\fermata }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => assert_eq!(
                n.post_events[0],
                PostEvent::NamedArticulation {
                    direction: Direction::Neutral,
                    name: "fermata".into(),
                }
            ),
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_directed_trill() {
    // With direction prefix
    let ast = parse("{ c4-\\trill }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => assert_eq!(
                n.post_events[0],
                PostEvent::NamedArticulation {
                    direction: Direction::Neutral,
                    name: "trill".into(),
                }
            ),
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_upbow_downbow() {
    let ast = parse("{ c4\\upbow d4\\downbow }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            match &items[0] {
                Music::Note(n) => assert_eq!(
                    n.post_events[0],
                    PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "upbow".into(),
                    }
                ),
                other => panic!("expected note, got {other:?}"),
            }
            match &items[1] {
                Music::Note(n) => assert_eq!(
                    n.post_events[0],
                    PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "downbow".into(),
                    }
                ),
                other => panic!("expected note, got {other:?}"),
            }
        }
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tremolo_32() {
    let ast = parse("{ c4:32 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.post_events, vec![PostEvent::Tremolo(32)]);
            }
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tremolo_16() {
    let ast = parse("{ f8:16 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.duration.as_ref().unwrap().base, 8);
                assert_eq!(n.post_events, vec![PostEvent::Tremolo(16)]);
            }
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tremolo_bare_colon() {
    let ast = parse("{ c4: }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.post_events, vec![PostEvent::Tremolo(0)]);
            }
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_chord_tremolo() {
    let ast = parse("{ <c e g>4:32 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Chord(c) => {
                assert_eq!(c.pitches.len(), 3);
                assert_eq!(c.post_events, vec![PostEvent::Tremolo(32)]);
            }
            other => panic!("expected chord, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_tremolo_with_ornament() {
    let ast = parse("{ c4:32\\trill }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.post_events.len(), 2);
                assert_eq!(n.post_events[0], PostEvent::Tremolo(32));
                assert_eq!(
                    n.post_events[1],
                    PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "trill".into(),
                    }
                );
            }
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn parse_multiple_ornaments() {
    let ast = parse("{ c4\\trill\\fermata }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.post_events.len(), 2);
                assert_eq!(
                    n.post_events[0],
                    PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "trill".into(),
                    }
                );
                assert_eq!(
                    n.post_events[1],
                    PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "fermata".into(),
                    }
                );
            }
            other => panic!("expected note, got {other:?}"),
        },
        other => panic!("expected sequential, got {other:?}"),
    }
}

#[test]
fn roundtrip_tremolo() {
    let input = "{ c4:32 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_ornaments() {
    let input = "{ c4\\trill d4\\mordent e4\\turn f4\\fermata }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_ornaments_tremolo() {
    roundtrip_fixture("fragment_ornaments_tremolo.ly");
}

// ── Phase 14 (technical notations: string numbers, open, harmonic) ──

#[test]
fn parse_string_number_neutral() {
    let ast = parse("{ c4-\\1 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::StringNumber {
                        direction: Direction::Neutral,
                        number: 1,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_string_number_up() {
    let ast = parse("{ c4^\\2 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::StringNumber {
                        direction: Direction::Up,
                        number: 2,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_string_number_down() {
    let ast = parse("{ c4_\\3 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::StringNumber {
                        direction: Direction::Down,
                        number: 3,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_string_number_undirected() {
    // \1 without direction prefix
    let ast = parse("{ c4\\1 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::StringNumber {
                        direction: Direction::Neutral,
                        number: 1,
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_open_string() {
    let ast = parse("{ c4\\open }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "open".into(),
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_harmonic() {
    let ast = parse("{ c4\\harmonic }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(
                    n.post_events,
                    vec![PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "harmonic".into(),
                    }]
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn parse_string_number_with_open() {
    let ast = parse("{ c4-\\1 -\\open }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => match &items[0] {
            Music::Note(n) => {
                assert_eq!(n.post_events.len(), 2);
                assert_eq!(
                    n.post_events[0],
                    PostEvent::StringNumber {
                        direction: Direction::Neutral,
                        number: 1,
                    }
                );
                assert_eq!(
                    n.post_events[1],
                    PostEvent::NamedArticulation {
                        direction: Direction::Neutral,
                        name: "open".into(),
                    }
                );
            }
            o => panic!("expected note, got {o:?}"),
        },
        o => panic!("expected sequential, got {o:?}"),
    }
}

#[test]
fn roundtrip_string_numbers() {
    let input = "{ c4-\\1 d4-\\2 e4^\\3 f4_\\4 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_technical_notations() {
    let input = "{ c4\\open d4\\harmonic e4\\upbow f4\\downbow g4\\flageolet }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_technical() {
    roundtrip_fixture("fragment_technical.ly");
}

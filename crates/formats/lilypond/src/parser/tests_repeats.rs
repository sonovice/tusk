//! Parser tests for repeats and alternatives (Phase 17).

use crate::model::*;
use crate::parser::parse;

// ── Helpers ──────────────────────────────────────────────────────────────

fn roundtrip_fixture(name: &str) {
    let path = format!(
        "{}/tests/fixtures/lilypond/{}",
        env!("CARGO_MANIFEST_DIR").replace("/crates/formats/lilypond", ""),
        name
    );
    let input = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let ast = parse(&input).unwrap_or_else(|e| panic!("parse {name}: {e}"));
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap_or_else(|e| panic!("re-parse {name}: {e}"));
    assert_eq!(ast, ast2, "roundtrip mismatch for {name}");
}

// ── \repeat volta ────────────────────────────────────────────────────────

#[test]
fn parse_repeat_volta_basic() {
    let input = "\\repeat volta 2 { c4 d e f }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Repeat {
            repeat_type,
            count,
            body,
            alternatives,
        }) => {
            assert_eq!(*repeat_type, RepeatType::Volta);
            assert_eq!(*count, 2);
            assert!(matches!(body.as_ref(), Music::Sequential(items) if items.len() == 4));
            assert!(alternatives.is_none());
        }
        other => panic!("expected Repeat, got {other:?}"),
    }
}

#[test]
fn parse_repeat_volta_with_alternatives() {
    let input = "\\repeat volta 2 { c4 d e f } \\alternative { { g2 } { a2 } }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Repeat {
            repeat_type,
            count,
            body,
            alternatives,
        }) => {
            assert_eq!(*repeat_type, RepeatType::Volta);
            assert_eq!(*count, 2);
            assert!(matches!(body.as_ref(), Music::Sequential(_)));
            let alts = alternatives.as_ref().expect("should have alternatives");
            assert_eq!(alts.len(), 2);
            // Each alternative is a sequential block
            assert!(matches!(&alts[0], Music::Sequential(items) if items.len() == 1));
            assert!(matches!(&alts[1], Music::Sequential(items) if items.len() == 1));
        }
        other => panic!("expected Repeat, got {other:?}"),
    }
}

#[test]
fn parse_repeat_volta_three_alternatives() {
    let input = "\\repeat volta 3 { c4 } \\alternative { { g2 } { a2 } { b2 } }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Repeat {
            count,
            alternatives,
            ..
        }) => {
            assert_eq!(*count, 3);
            assert_eq!(alternatives.as_ref().unwrap().len(), 3);
        }
        other => panic!("expected Repeat, got {other:?}"),
    }
}

// ── \repeat unfold ───────────────────────────────────────────────────────

#[test]
fn parse_repeat_unfold() {
    let input = "\\repeat unfold 4 { c8 d }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Repeat {
            repeat_type,
            count,
            alternatives,
            ..
        }) => {
            assert_eq!(*repeat_type, RepeatType::Unfold);
            assert_eq!(*count, 4);
            assert!(alternatives.is_none());
        }
        other => panic!("expected Repeat, got {other:?}"),
    }
}

// ── \repeat percent ──────────────────────────────────────────────────────

#[test]
fn parse_repeat_percent() {
    let input = "\\repeat percent 4 { c4 d e f }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Repeat {
            repeat_type, count, ..
        }) => {
            assert_eq!(*repeat_type, RepeatType::Percent);
            assert_eq!(*count, 4);
        }
        other => panic!("expected Repeat, got {other:?}"),
    }
}

// ── \repeat tremolo ──────────────────────────────────────────────────────

#[test]
fn parse_repeat_tremolo() {
    let input = "\\repeat tremolo 8 { c16 d }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Repeat {
            repeat_type, count, ..
        }) => {
            assert_eq!(*repeat_type, RepeatType::Tremolo);
            assert_eq!(*count, 8);
        }
        other => panic!("expected Repeat, got {other:?}"),
    }
}

// ── \repeat segno ────────────────────────────────────────────────────────

#[test]
fn parse_repeat_segno() {
    let input = "\\repeat segno 2 { c4 d e f }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Repeat { repeat_type, .. }) => {
            assert_eq!(*repeat_type, RepeatType::Segno);
        }
        other => panic!("expected Repeat, got {other:?}"),
    }
}

// ── Nested repeats ───────────────────────────────────────────────────────

#[test]
fn parse_nested_repeat() {
    let input = "\\repeat volta 2 { \\repeat unfold 3 { c8 d } e4 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Repeat {
            repeat_type, body, ..
        }) => {
            assert_eq!(*repeat_type, RepeatType::Volta);
            match body.as_ref() {
                Music::Sequential(items) => {
                    assert_eq!(items.len(), 2); // inner repeat + e4
                    assert!(matches!(
                        &items[0],
                        Music::Repeat {
                            repeat_type: RepeatType::Unfold,
                            ..
                        }
                    ));
                }
                other => panic!("expected Sequential, got {other:?}"),
            }
        }
        other => panic!("expected Repeat, got {other:?}"),
    }
}

// ── Repeat in score ──────────────────────────────────────────────────────

#[test]
fn parse_repeat_in_score() {
    let input = "\\score { \\repeat volta 2 { c4 d e f } }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Score(sb) => {
            assert_eq!(sb.items.len(), 1);
            match &sb.items[0] {
                ScoreItem::Music(Music::Repeat { repeat_type, .. }) => {
                    assert_eq!(*repeat_type, RepeatType::Volta);
                }
                other => panic!("expected Repeat, got {other:?}"),
            }
        }
        other => panic!("expected Score, got {other:?}"),
    }
}

// ── Serializer roundtrip ─────────────────────────────────────────────────

#[test]
fn roundtrip_repeat_volta_basic() {
    let input = "\\repeat volta 2 { c4 d e f }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_repeat_with_alternatives() {
    let input = "\\repeat volta 2 { c4 d e f } \\alternative { { g2 } { a2 } }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_repeat_unfold() {
    let input = "\\repeat unfold 4 { c8 d }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_repeat_nested() {
    let input = "\\repeat volta 2 { \\repeat unfold 3 { c8 d } e4 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

// ── Fixture roundtrip ────────────────────────────────────────────────────

#[test]
fn roundtrip_repeats_fixture() {
    roundtrip_fixture("fragment_repeats.ly");
}

//! Parser tests for bar checks, bar lines, and multi-measure rests (Phase 18).

use crate::model::*;
use crate::parser::parse;

// ── Helpers ──────────────────────────────────────────────────────────────

fn roundtrip(input: &str) {
    let ast = parse(input).unwrap_or_else(|e| panic!("parse: {e}"));
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap_or_else(|e| panic!("re-parse: {e}"));
    assert_eq!(
        ast, ast2,
        "roundtrip mismatch:\ninput:  {input}\noutput: {output}"
    );
}

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

// ── Bar check ────────────────────────────────────────────────────────────

#[test]
fn parse_bar_check_standalone() {
    let input = "{ c4 d e f | }";
    let ast = parse(input).unwrap();
    let items = match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => items,
        other => panic!("expected sequential, got {other:?}"),
    };
    assert_eq!(items.len(), 5);
    assert_eq!(items[4], Music::BarCheck);
}

#[test]
fn parse_bar_check_between_notes() {
    let input = "{ c4 d e f | g a b c' | }";
    let ast = parse(input).unwrap();
    let items = match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => items,
        other => panic!("expected sequential, got {other:?}"),
    };
    // c d e f | g a b c' | = 10 items
    assert_eq!(items.len(), 10);
    assert_eq!(items[4], Music::BarCheck);
    assert_eq!(items[9], Music::BarCheck);
}

#[test]
fn parse_bar_check_at_start() {
    let input = "{ | c4 d e f }";
    let ast = parse(input).unwrap();
    let items = match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => items,
        other => panic!("expected sequential, got {other:?}"),
    };
    assert_eq!(items[0], Music::BarCheck);
}

// ── Bar line ─────────────────────────────────────────────────────────────

#[test]
fn parse_bar_line_final() {
    let input = r#"{ c4 d e f \bar "|." }"#;
    let ast = parse(input).unwrap();
    let items = match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => items,
        other => panic!("expected sequential, got {other:?}"),
    };
    assert_eq!(items.len(), 5);
    assert_eq!(
        items[4],
        Music::BarLine {
            bar_type: "|.".to_string()
        }
    );
}

#[test]
fn parse_bar_line_double() {
    let input = r#"\bar "||""#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::BarLine { bar_type }) => {
            assert_eq!(bar_type, "||");
        }
        other => panic!("expected BarLine, got {other:?}"),
    }
}

#[test]
fn parse_bar_line_repeat() {
    let input = r#"\bar ":|.""#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::BarLine { bar_type }) => {
            assert_eq!(bar_type, ":|.");
        }
        other => panic!("expected BarLine, got {other:?}"),
    }
}

#[test]
fn parse_bar_line_empty_string() {
    let input = r#"\bar """#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::BarLine { bar_type }) => {
            assert_eq!(bar_type, "");
        }
        other => panic!("expected BarLine, got {other:?}"),
    }
}

// ── Multi-measure rest (R) — already from Phase 3, confirm still works ───

#[test]
fn parse_multi_measure_rest_with_multiplier() {
    let input = "R1*4";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::MultiMeasureRest(r)) => {
            let dur = r.duration.as_ref().unwrap();
            assert_eq!(dur.base, 1);
            assert_eq!(dur.multipliers, vec![(4, 1)]);
        }
        other => panic!("expected MultiMeasureRest, got {other:?}"),
    }
}

#[test]
fn parse_multi_measure_rest_with_fraction_multiplier() {
    let input = "R2.*3/4";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::MultiMeasureRest(r)) => {
            let dur = r.duration.as_ref().unwrap();
            assert_eq!(dur.base, 2);
            assert_eq!(dur.dots, 1);
            assert_eq!(dur.multipliers, vec![(3, 4)]);
        }
        other => panic!("expected MultiMeasureRest, got {other:?}"),
    }
}

// ── Roundtrips ───────────────────────────────────────────────────────────

#[test]
fn roundtrip_bar_check() {
    roundtrip("{ c4 d e f | g a b c' | }");
}

#[test]
fn roundtrip_bar_line() {
    roundtrip(r#"{ c4 d e f \bar "|." }"#);
}

#[test]
fn roundtrip_bar_line_double() {
    roundtrip(r#"\bar "||""#);
}

#[test]
fn roundtrip_bar_line_repeat() {
    roundtrip(r#"\bar ":|.""#);
}

#[test]
fn roundtrip_multi_measure_rest_multiplier() {
    roundtrip("R1*4");
}

#[test]
fn roundtrip_bar_check_and_bar_line_combined() {
    roundtrip(r#"{ c4 d e f | g a b c' \bar "|." }"#);
}

#[test]
fn roundtrip_barcheck_barline_fixture() {
    roundtrip_fixture("fragment_barcheck_barline.ly");
}

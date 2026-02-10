//! Parser tests for chord repetition `q` (Phase 19).

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

// ── Parse chord repetition ─────────────────────────────────────────────

#[test]
fn parse_chord_rep_basic() {
    let input = "{ <c e g>4 q q q }";
    let ast = parse(input).unwrap();
    let items = match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => items,
        other => panic!("expected sequential, got {other:?}"),
    };
    assert_eq!(items.len(), 4);
    assert!(matches!(items[0], Music::Chord(_)));
    for (idx, item) in items.iter().enumerate().skip(1) {
        match item {
            Music::ChordRepetition(cr) => {
                assert!(cr.duration.is_none());
                assert!(cr.post_events.is_empty());
            }
            other => panic!("expected ChordRepetition at {idx}, got {other:?}"),
        }
    }
}

#[test]
fn parse_chord_rep_with_duration() {
    let input = "q4";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ChordRepetition(cr)) => {
            let dur = cr.duration.as_ref().unwrap();
            assert_eq!(dur.base, 4);
            assert_eq!(dur.dots, 0);
        }
        other => panic!("expected ChordRepetition, got {other:?}"),
    }
}

#[test]
fn parse_chord_rep_with_dotted_duration() {
    let input = "q2.";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ChordRepetition(cr)) => {
            let dur = cr.duration.as_ref().unwrap();
            assert_eq!(dur.base, 2);
            assert_eq!(dur.dots, 1);
        }
        other => panic!("expected ChordRepetition, got {other:?}"),
    }
}

#[test]
fn parse_chord_rep_with_post_events() {
    let input = "q4(";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ChordRepetition(cr)) => {
            assert_eq!(cr.post_events.len(), 1);
            assert_eq!(cr.post_events[0], PostEvent::SlurStart);
        }
        other => panic!("expected ChordRepetition, got {other:?}"),
    }
}

#[test]
fn parse_chord_rep_with_dynamics() {
    let input = r"q4\f";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ChordRepetition(cr)) => {
            assert_eq!(cr.post_events.len(), 1);
            assert_eq!(cr.post_events[0], PostEvent::Dynamic("f".to_string()));
        }
        other => panic!("expected ChordRepetition, got {other:?}"),
    }
}

#[test]
fn parse_chord_rep_with_tie() {
    let input = "q4~";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ChordRepetition(cr)) => {
            assert_eq!(cr.post_events.len(), 1);
            assert_eq!(cr.post_events[0], PostEvent::Tie);
        }
        other => panic!("expected ChordRepetition, got {other:?}"),
    }
}

#[test]
fn parse_chord_rep_with_tremolo() {
    let input = "q8:32";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ChordRepetition(cr)) => {
            let dur = cr.duration.as_ref().unwrap();
            assert_eq!(dur.base, 8);
            assert_eq!(cr.post_events.len(), 1);
            assert_eq!(cr.post_events[0], PostEvent::Tremolo(32));
        }
        other => panic!("expected ChordRepetition, got {other:?}"),
    }
}

#[test]
fn parse_chord_rep_no_duration() {
    let input = "q";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::ChordRepetition(cr)) => {
            assert!(cr.duration.is_none());
            assert!(cr.post_events.is_empty());
        }
        other => panic!("expected ChordRepetition, got {other:?}"),
    }
}

#[test]
fn parse_chord_rep_mixed_with_notes() {
    let input = "{ <c e g>4 q c4 q }";
    let ast = parse(input).unwrap();
    let items = match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => items,
        other => panic!("expected sequential, got {other:?}"),
    };
    assert_eq!(items.len(), 4);
    assert!(matches!(items[0], Music::Chord(_)));
    assert!(matches!(items[1], Music::ChordRepetition(_)));
    assert!(matches!(items[2], Music::Note(_)));
    assert!(matches!(items[3], Music::ChordRepetition(_)));
}

// ── Roundtrips ───────────────────────────────────────────────────────────

#[test]
fn roundtrip_chord_rep_basic() {
    roundtrip("{ <c e g>4 q q q }");
}

#[test]
fn roundtrip_chord_rep_with_duration() {
    roundtrip("q4");
}

#[test]
fn roundtrip_chord_rep_dotted() {
    roundtrip("q2.");
}

#[test]
fn roundtrip_chord_rep_with_post_events() {
    roundtrip("{ <c e g>4( q q q) }");
}

#[test]
fn roundtrip_chord_rep_with_dynamics() {
    roundtrip(r"{ <c e g>4\f q\p }");
}

#[test]
fn roundtrip_chord_rep_with_tie() {
    roundtrip("{ <c e g>4~ q }");
}

#[test]
fn roundtrip_chord_rep_fixture() {
    roundtrip_fixture("fragment_chord_repetition.ly");
}

//! Parser tests for grace notes (Phase 16).

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

// ── \grace ───────────────────────────────────────────────────────────────

#[test]
fn parse_grace_single_note() {
    let input = "\\grace c16 d4";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 2);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Grace { body }) => {
            assert!(matches!(body.as_ref(), Music::Note(n) if n.pitch.step == 'c'));
        }
        other => panic!("expected Grace, got {other:?}"),
    }
    match &ast.items[1] {
        ToplevelExpression::Music(Music::Note(n)) => {
            assert_eq!(n.pitch.step, 'd');
        }
        other => panic!("expected Note, got {other:?}"),
    }
}

#[test]
fn parse_grace_braced() {
    let input = "\\grace { c16 d16 }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Grace { body }) => match body.as_ref() {
            Music::Sequential(items) => assert_eq!(items.len(), 2),
            other => panic!("expected Sequential, got {other:?}"),
        },
        other => panic!("expected Grace, got {other:?}"),
    }
}

// ── \acciaccatura ────────────────────────────────────────────────────────

#[test]
fn parse_acciaccatura_single() {
    let input = "\\acciaccatura d8 c4";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 2);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Acciaccatura { body }) => {
            assert!(matches!(body.as_ref(), Music::Note(n) if n.pitch.step == 'd'));
        }
        other => panic!("expected Acciaccatura, got {other:?}"),
    }
}

#[test]
fn parse_acciaccatura_braced() {
    let input = "\\acciaccatura { c16 d16 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Acciaccatura { body }) => {
            assert!(matches!(body.as_ref(), Music::Sequential(_)));
        }
        other => panic!("expected Acciaccatura, got {other:?}"),
    }
}

// ── \appoggiatura ────────────────────────────────────────────────────────

#[test]
fn parse_appoggiatura_single() {
    let input = "\\appoggiatura d8 c2";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 2);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Appoggiatura { body }) => {
            assert!(matches!(body.as_ref(), Music::Note(n) if n.pitch.step == 'd'));
        }
        other => panic!("expected Appoggiatura, got {other:?}"),
    }
}

// ── \afterGrace ──────────────────────────────────────────────────────────

#[test]
fn parse_after_grace_no_fraction() {
    let input = "\\afterGrace c2 { d16 e16 }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::AfterGrace {
            fraction,
            main,
            grace,
        }) => {
            assert_eq!(*fraction, None);
            assert!(matches!(main.as_ref(), Music::Note(n) if n.pitch.step == 'c'));
            match grace.as_ref() {
                Music::Sequential(items) => assert_eq!(items.len(), 2),
                other => panic!("expected Sequential grace, got {other:?}"),
            }
        }
        other => panic!("expected AfterGrace, got {other:?}"),
    }
}

#[test]
fn parse_after_grace_with_fraction() {
    let input = "\\afterGrace 3/4 c2 { d16 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::AfterGrace {
            fraction,
            main,
            grace,
        }) => {
            assert_eq!(*fraction, Some((3, 4)));
            assert!(matches!(main.as_ref(), Music::Note(n) if n.pitch.step == 'c'));
            assert!(matches!(grace.as_ref(), Music::Sequential(_)));
        }
        other => panic!("expected AfterGrace, got {other:?}"),
    }
}

#[test]
fn parse_after_grace_fraction_7_8() {
    let input = "\\afterGrace 7/8 f1 { g8 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::AfterGrace { fraction, .. }) => {
            assert_eq!(*fraction, Some((7, 8)));
        }
        other => panic!("expected AfterGrace, got {other:?}"),
    }
}

// ── Grace inside sequential ──────────────────────────────────────────────

#[test]
fn parse_grace_in_sequential() {
    let input = "{ \\grace c16 d4 e4 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 3);
            assert!(matches!(&items[0], Music::Grace { .. }));
            assert!(matches!(&items[1], Music::Note(_)));
            assert!(matches!(&items[2], Music::Note(_)));
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
}

#[test]
fn parse_after_grace_in_sequential() {
    let input = "{ \\afterGrace c2 { d16 e16 } f4 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert_eq!(items.len(), 2); // afterGrace + f4
            assert!(matches!(&items[0], Music::AfterGrace { .. }));
            assert!(matches!(&items[1], Music::Note(_)));
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
}

// ── Roundtrip ────────────────────────────────────────────────────────────

#[test]
fn roundtrip_grace() {
    let input = "\\grace c16";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_acciaccatura() {
    let input = "\\acciaccatura d8";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_appoggiatura() {
    let input = "\\appoggiatura d8";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_after_grace_no_fraction() {
    let input = "\\afterGrace c2 { d16 e16 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_after_grace_with_fraction() {
    let input = "\\afterGrace 3/4 c2 { d16 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_grace_braced() {
    let input = "\\grace { c16 d16 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_grace() {
    roundtrip_fixture("fragment_grace.ly");
}

//! Parser tests for music functions (Phase 29).

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

// ── Generic music function with braced music ─────────────────────────────

#[test]
fn parse_function_with_music_arg() {
    let input = "\\someFunction { c4 d e f }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::MusicFunction { name, args }) => {
            assert_eq!(name, "someFunction");
            assert_eq!(args.len(), 1);
            assert!(matches!(&args[0], FunctionArg::Music(Music::Sequential(_))));
        }
        other => panic!("expected MusicFunction, got {other:?}"),
    }
}

// ── Function with string argument ────────────────────────────────────────

#[test]
fn parse_function_with_string_arg() {
    let input = "\\tag \"part\" { c4 d e f }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::MusicFunction { name, args }) => {
            assert_eq!(name, "tag");
            assert_eq!(args.len(), 2);
            assert!(matches!(&args[0], FunctionArg::String(s) if s == "part"));
            assert!(matches!(&args[1], FunctionArg::Music(Music::Sequential(_))));
        }
        other => panic!("expected MusicFunction, got {other:?}"),
    }
}

// ── Function with scheme argument ────────────────────────────────────────

#[test]
fn parse_function_with_scheme_arg() {
    let input = "\\keepWithTag #'print { c4 d e f }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::MusicFunction { name, args }) => {
            assert_eq!(name, "keepWithTag");
            assert_eq!(args.len(), 2);
            assert!(matches!(&args[0], FunctionArg::SchemeExpr(_)));
            assert!(matches!(&args[1], FunctionArg::Music(Music::Sequential(_))));
        }
        other => panic!("expected MusicFunction, got {other:?}"),
    }
}

// ── Function with two music arguments ────────────────────────────────────

#[test]
fn parse_function_with_two_music_args() {
    let input = "\\partCombine { c4 e g } { e4 g c' }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::MusicFunction { name, args }) => {
            assert_eq!(name, "partCombine");
            assert_eq!(args.len(), 2);
            assert!(matches!(&args[0], FunctionArg::Music(Music::Sequential(_))));
            assert!(matches!(&args[1], FunctionArg::Music(Music::Sequential(_))));
        }
        other => panic!("expected MusicFunction, got {other:?}"),
    }
}

// ── Function with numeric argument ───────────────────────────────────────

#[test]
fn parse_function_with_number_arg() {
    let input = "\\magnifyMusic 2 { c4 d e f }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::MusicFunction { name, args }) => {
            assert_eq!(name, "magnifyMusic");
            assert_eq!(args.len(), 2);
            assert!(matches!(&args[0], FunctionArg::Number(n) if *n == 2.0));
            assert!(matches!(&args[1], FunctionArg::Music(Music::Sequential(_))));
        }
        other => panic!("expected MusicFunction, got {other:?}"),
    }
}

// ── Partial function with \etc ───────────────────────────────────────────

#[test]
fn parse_partial_function() {
    let input = "\\tag #'score \\etc";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::PartialFunction { name, args }) => {
            assert_eq!(name, "tag");
            assert_eq!(args.len(), 1);
            assert!(matches!(&args[0], FunctionArg::SchemeExpr(_)));
        }
        other => panic!("expected PartialFunction, got {other:?}"),
    }
}

// ── Bare identifier (no args) stays as Identifier ────────────────────────

#[test]
fn parse_bare_identifier_no_args() {
    let input = "\\myMelody";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    assert!(matches!(
        &ast.items[0],
        ToplevelExpression::Music(Music::Identifier(s)) if s == "myMelody"
    ));
}

// ── Partial function with no args ────────────────────────────────────────

#[test]
fn parse_partial_function_no_args() {
    let input = "\\someFunc \\etc";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::PartialFunction { name, args }) => {
            assert_eq!(name, "someFunc");
            assert!(args.is_empty());
        }
        other => panic!("expected PartialFunction, got {other:?}"),
    }
}

// ── Function with \default argument ──────────────────────────────────────

#[test]
fn parse_function_with_default_arg() {
    let input = "\\someFunc \\default { c4 d e f }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        ToplevelExpression::Music(Music::MusicFunction { name, args }) => {
            assert_eq!(name, "someFunc");
            assert_eq!(args.len(), 2);
            assert!(matches!(&args[0], FunctionArg::Default));
            assert!(matches!(&args[1], FunctionArg::Music(Music::Sequential(_))));
        }
        other => panic!("expected MusicFunction, got {other:?}"),
    }
}

// ── Serialization roundtrip ──────────────────────────────────────────────

#[test]
fn serialize_function_call_roundtrip() {
    let input = "\\someFunction \"arg1\" { c4 d e f }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn serialize_partial_function_roundtrip() {
    let input = "\\tag #'score \\etc";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

// ── Fixture roundtrip ────────────────────────────────────────────────────

#[test]
fn roundtrip_fragment_music_functions() {
    roundtrip_fixture("fragment_music_functions.ly");
}

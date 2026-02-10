//! Tests for Scheme expression parsing and serialization roundtrip.

use crate::model::scheme::SchemeExpr;
use crate::model::*;
use crate::parser::parse;
use crate::serializer::serialize;

/// Helper: parse and return the first music item.
fn parse_first_music(src: &str) -> Music {
    let file = parse(src).unwrap();
    match &file.items[0] {
        ToplevelExpression::Music(m) => m.clone(),
        _ => panic!("expected Music"),
    }
}

/// Helper: roundtrip serialize.
fn roundtrip(src: &str) -> String {
    let file = parse(src).unwrap();
    serialize(&file)
}

// ── Boolean ────────────────────────────────────────────────────────

#[test]
fn scheme_bool_true() {
    let m = parse_first_music("\\set Staff.x = ##t");
    match m {
        Music::Set { value, .. } => {
            assert_eq!(value, PropertyValue::SchemeExpr(SchemeExpr::Bool(true)));
        }
        _ => panic!("expected Set"),
    }
}

#[test]
fn scheme_bool_false() {
    let m = parse_first_music("\\set Staff.x = ##f");
    match m {
        Music::Set { value, .. } => {
            assert_eq!(value, PropertyValue::SchemeExpr(SchemeExpr::Bool(false)));
        }
        _ => panic!("expected Set"),
    }
}

// ── Numbers ────────────────────────────────────────────────────────

#[test]
fn scheme_integer() {
    let m = parse_first_music("\\override Staff.fontSize = #-2");
    match m {
        Music::Override { value, .. } => {
            assert_eq!(value, PropertyValue::SchemeExpr(SchemeExpr::Integer(-2)));
        }
        _ => panic!("expected Override"),
    }
}

#[test]
fn scheme_positive_integer() {
    let m = parse_first_music("\\override Staff.fontSize = #3");
    match m {
        Music::Override { value, .. } => {
            assert_eq!(value, PropertyValue::SchemeExpr(SchemeExpr::Integer(3)));
        }
        _ => panic!("expected Override"),
    }
}

#[test]
fn scheme_float() {
    let m = parse_first_music("\\override Staff.fontSize = #2.5");
    match m {
        Music::Override { value, .. } => {
            assert_eq!(value, PropertyValue::SchemeExpr(SchemeExpr::Float(2.5)));
        }
        _ => panic!("expected Override"),
    }
}

#[test]
fn scheme_negative_float() {
    let m = parse_first_music("\\override Staff.fontSize = #-1.5");
    match m {
        Music::Override { value, .. } => {
            assert_eq!(value, PropertyValue::SchemeExpr(SchemeExpr::Float(-1.5)));
        }
        _ => panic!("expected Override"),
    }
}

// ── String ─────────────────────────────────────────────────────────

#[test]
fn scheme_string() {
    let m = parse_first_music("\\override Staff.name = #\"hello\"");
    match m {
        Music::Override { value, .. } => {
            assert_eq!(
                value,
                PropertyValue::SchemeExpr(SchemeExpr::String("hello".into()))
            );
        }
        _ => panic!("expected Override"),
    }
}

// ── Symbol (quoted) ────────────────────────────────────────────────

#[test]
fn scheme_symbol() {
    let file = parse("\\keepWithTag #'print { c4 }").unwrap();
    let m = match &file.items[0] {
        ToplevelExpression::Music(m) => m,
        _ => panic!("expected Music"),
    };
    match m {
        Music::MusicFunction { args, .. } => {
            assert!(
                matches!(&args[0], FunctionArg::SchemeExpr(SchemeExpr::Symbol(s)) if s == "print")
            );
        }
        _ => panic!("expected MusicFunction"),
    }
}

// ── Identifier ─────────────────────────────────────────────────────

#[test]
fn scheme_identifier() {
    let m = parse_first_music("\\override NoteHead.color = #red");
    match m {
        Music::Override { value, .. } => {
            assert_eq!(
                value,
                PropertyValue::SchemeExpr(SchemeExpr::Identifier("red".into()))
            );
        }
        _ => panic!("expected Override"),
    }
}

// ── List (S-expression) ───────────────────────────────────────────

#[test]
fn scheme_list() {
    let m = parse_first_music("\\override Glissando.color = #(rgb-color 1 0 0)");
    match m {
        Music::Override { value, .. } => match value {
            PropertyValue::SchemeExpr(SchemeExpr::List(raw)) => {
                assert!(raw.starts_with('('));
                assert!(raw.ends_with(')'));
                assert!(raw.contains("rgb-color"));
            }
            other => panic!("expected SchemeExpr::List, got {other:?}"),
        },
        _ => panic!("expected Override"),
    }
}

#[test]
fn scheme_nested_list() {
    let m = parse_first_music("\\override X.y = #(list (cons 1 2) (cons 3 4))");
    match m {
        Music::Override { value, .. } => match value {
            PropertyValue::SchemeExpr(SchemeExpr::List(raw)) => {
                assert!(raw.contains("list"));
                assert!(raw.contains("cons"));
            }
            other => panic!("expected SchemeExpr::List, got {other:?}"),
        },
        _ => panic!("expected Override"),
    }
}

// ── Roundtrips ─────────────────────────────────────────────────────

#[test]
fn roundtrip_bool_true() {
    let out = roundtrip("\\set Staff.x = ##t\n");
    assert!(out.contains("##t"));
}

#[test]
fn roundtrip_bool_false() {
    let out = roundtrip("\\set Staff.x = ##f\n");
    assert!(out.contains("##f"));
}

#[test]
fn roundtrip_identifier() {
    let out = roundtrip("\\override NoteHead.color = #red\n");
    assert!(out.contains("#red"));
}

#[test]
fn roundtrip_integer() {
    let out = roundtrip("\\override Staff.fontSize = #-2\n");
    assert!(out.contains("#-2"));
}

#[test]
fn roundtrip_symbol() {
    let out = roundtrip("\\keepWithTag #'print { c4 }\n");
    assert!(out.contains("#'print"));
}

#[test]
fn roundtrip_list() {
    let out = roundtrip("\\override Glissando.color = #(rgb-color 1 0 0)\n");
    assert!(out.contains("#(rgb-color 1 0 0)"));
}

#[test]
fn roundtrip_scheme_in_markup() {
    let out = roundtrip("\\markup \\with-color #red \"text\"\n");
    assert!(out.contains("#red"));
}

// ── Fixture parse ──────────────────────────────────────────────────

#[test]
fn parse_fragment_scheme() {
    let src = include_str!("../../../../../tests/fixtures/lilypond/fragment_scheme.ly");
    let file = parse(src).unwrap();
    // Validate
    crate::validator::validate(&file).unwrap();
    // Roundtrip
    let out = serialize(&file);
    let file2 = parse(&out).unwrap();
    crate::validator::validate(&file2).unwrap();
}

// ── Validator ──────────────────────────────────────────────────────

#[test]
fn validator_scheme_in_override() {
    let file = parse("{ \\override NoteHead.color = #red }").unwrap();
    crate::validator::validate(&file).unwrap();
}

#[test]
fn validator_scheme_bool_in_set() {
    let file = parse("{ \\set Staff.useBassFigureExtenders = ##t }").unwrap();
    crate::validator::validate(&file).unwrap();
}

#[test]
fn validator_scheme_list() {
    let file = parse("{ \\override X.y = #(list 1 2 3) }").unwrap();
    crate::validator::validate(&file).unwrap();
}

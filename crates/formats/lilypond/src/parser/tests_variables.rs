//! Parser tests for variable assignments and identifier references (Phase 28).

use super::parse;
use crate::model::*;

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
fn parse_assignment_string_value() {
    let input = r#"myTitle = "Hello World""#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Assignment(a) => {
            assert_eq!(a.name, "myTitle");
            assert!(matches!(&a.value, AssignmentValue::String(s) if s == "Hello World"));
        }
        other => panic!("expected assignment, got {other:?}"),
    }
}

#[test]
fn parse_assignment_number_value() {
    let input = "myNum = 42";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Assignment(a) => {
            assert_eq!(a.name, "myNum");
            assert!(matches!(&a.value, AssignmentValue::Number(n) if *n == 42.0));
        }
        other => panic!("expected assignment, got {other:?}"),
    }
}

#[test]
fn parse_assignment_music_value() {
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
fn parse_assignment_identifier_value() {
    let input = "soprano = \\melody";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Assignment(a) => {
            assert_eq!(a.name, "soprano");
            assert!(matches!(&a.value, AssignmentValue::Identifier(s) if s == "melody"));
        }
        other => panic!("expected assignment, got {other:?}"),
    }
}

#[test]
fn parse_assignment_markup_value() {
    let input = "title = \\markup { Hello }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Assignment(a) => {
            assert_eq!(a.name, "title");
            assert!(matches!(&a.value, AssignmentValue::Markup(_)));
        }
        other => panic!("expected assignment, got {other:?}"),
    }
}

#[test]
fn parse_assignment_scheme_value() {
    let input = "num = #3";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Assignment(a) => {
            assert_eq!(a.name, "num");
            assert!(matches!(&a.value, AssignmentValue::SchemeExpr(_)));
        }
        other => panic!("expected assignment, got {other:?}"),
    }
}

#[test]
fn parse_identifier_in_music() {
    let input = "{ c4 \\melody d4 }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert!(matches!(&items[1], Music::Identifier(s) if s == "melody"));
        }
        other => panic!("expected sequential music, got {other:?}"),
    }
}

#[test]
fn parse_assignment_then_score_with_ref() {
    let input = "melody = { c4 d4 e4 f4 }\n\\score { \\new Staff \\melody }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 2);
    assert!(matches!(&ast.items[0], ToplevelExpression::Assignment(_)));
    match &ast.items[1] {
        ToplevelExpression::Score(sb) => match &sb.items[0] {
            ScoreItem::Music(Music::ContextedMusic { music, .. }) => {
                assert!(matches!(music.as_ref(), Music::Identifier(s) if s == "melody"));
            }
            other => panic!("expected contexted music, got {other:?}"),
        },
        other => panic!("expected score, got {other:?}"),
    }
}

#[test]
fn parse_multiple_assignments() {
    let input = "melody = { c4 d4 }\nharmony = { e4 f4 }";
    let ast = parse(input).unwrap();
    assert_eq!(ast.items.len(), 2);
    match &ast.items[0] {
        ToplevelExpression::Assignment(a) => assert_eq!(a.name, "melody"),
        other => panic!("expected assignment, got {other:?}"),
    }
    match &ast.items[1] {
        ToplevelExpression::Assignment(a) => assert_eq!(a.name, "harmony"),
        other => panic!("expected assignment, got {other:?}"),
    }
}

#[test]
fn parse_assignment_relative_music() {
    let input = "melody = \\relative c' { c4 d e f }";
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Assignment(a) => {
            assert_eq!(a.name, "melody");
            assert!(
                matches!(&a.value, AssignmentValue::Music(m) if matches!(m.as_ref(), Music::Relative { .. }))
            );
        }
        other => panic!("expected assignment, got {other:?}"),
    }
}

#[test]
fn roundtrip_assignment_string() {
    let input = r#"myTitle = "Hello""#;
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_assignment_music() {
    let input = "melody = { c4 d4 e4 f4 }";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_assignment_identifier() {
    let input = "soprano = \\melody";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_assignment_with_score() {
    let input = "melody = { c4 d4 e4 f4 }\n\\score {\n  \\new Staff \\melody\n}";
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_fragment_variables() {
    roundtrip_fixture("fragment_variables.ly");
}

use super::*;
use crate::import;
use crate::parser::Parser;
use crate::serializer;

/// Parse LilyPond -> import to MEI -> export to LilyPond AST -> serialize.
fn roundtrip(src: &str) -> String {
    let file = Parser::new(src).unwrap().parse().unwrap();
    let mei = import::import(&file).unwrap();
    let exported = export(&mei).unwrap();
    serializer::serialize(&exported)
}

/// Parse LilyPond -> import to MEI -> export to LilyPond AST.
fn roundtrip_ast(src: &str) -> LilyPondFile {
    let file = Parser::new(src).unwrap().parse().unwrap();
    let mei = import::import(&file).unwrap();
    export(&mei).unwrap()
}

// ---------------------------------------------------------------------------
// Export / roundtrip tests for variables
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_variable_assignment_music() {
    let src = "melody = { c'4 d'4 }\n\\score {\n  \\new Staff \\melody\n}";
    let output = roundtrip(src);
    // The assignment should be preserved in the output
    assert!(
        output.contains("melody = "),
        "output should contain assignment: {output}"
    );
    // Music content should be present
    assert!(output.contains("c'4"), "output: {output}");
    assert!(output.contains("d'4"), "output: {output}");
}

#[test]
fn roundtrip_multiple_assignments() {
    let src = "melody = { c'4 d'4 }\nharmony = { e'4 f'4 }\n\\score {\n  \\new Staff \\melody\n}";
    let output = roundtrip(src);
    assert!(
        output.contains("melody = "),
        "output should contain melody assignment: {output}"
    );
    assert!(
        output.contains("harmony = "),
        "output should contain harmony assignment: {output}"
    );
}

#[test]
fn roundtrip_string_assignment() {
    let src = "myTitle = \"Test Title\"\n{ c'4 }";
    let output = roundtrip(src);
    assert!(
        output.contains("myTitle = \"Test Title\""),
        "output: {output}"
    );
}

#[test]
fn roundtrip_number_assignment() {
    let src = "myNum = 42\n{ c'4 }";
    let output = roundtrip(src);
    assert!(output.contains("myNum = 42"), "output: {output}");
}

#[test]
fn roundtrip_identifier_assignment() {
    let src = "melody = { c'4 d'4 }\nsoprano = \\melody\n\\score {\n  \\new Staff \\soprano\n}";
    let output = roundtrip(src);
    assert!(
        output.contains("melody = "),
        "output should contain melody: {output}"
    );
    assert!(
        output.contains("soprano = \\melody"),
        "output should contain soprano ref: {output}"
    );
}

#[test]
fn roundtrip_assignments_appear_before_score() {
    let src = "melody = { c'4 d'4 }\n\\score {\n  \\new Staff \\melody\n}";
    let output = roundtrip(src);
    let assignment_pos = output.find("melody = ").expect("should have assignment");
    let score_pos = output.find("\\score").expect("should have score");
    assert!(
        assignment_pos < score_pos,
        "assignments should appear before score: {output}"
    );
}

#[test]
fn roundtrip_ast_contains_assignments() {
    let src = "melody = { c'4 d'4 }\n\\score {\n  \\new Staff \\melody\n}";
    let ast = roundtrip_ast(src);
    let assignment_count = ast
        .items
        .iter()
        .filter(|item| matches!(item, ToplevelExpression::Assignment(_)))
        .count();
    assert_eq!(assignment_count, 1, "expected 1 assignment in exported AST");
}

#[test]
fn roundtrip_no_assignments_produces_no_assignments() {
    let src = "{ c'4 d'4 }";
    let ast = roundtrip_ast(src);
    let assignment_count = ast
        .items
        .iter()
        .filter(|item| matches!(item, ToplevelExpression::Assignment(_)))
        .count();
    assert_eq!(
        assignment_count, 0,
        "no assignments expected when input has none"
    );
}

#[test]
fn roundtrip_fixture_variables() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_variables.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    // Should preserve assignments
    assert!(
        output.contains("melody = "),
        "output should contain melody: {output}"
    );
    assert!(
        output.contains("harmony = "),
        "output should contain harmony: {output}"
    );
    assert!(
        output.contains("myNum = 42"),
        "output should contain myNum: {output}"
    );
    assert!(
        output.contains("myTitle = \"A Title\""),
        "output should contain myTitle: {output}"
    );
    assert!(
        output.contains("soprano = \\melody"),
        "output should contain soprano: {output}"
    );
}

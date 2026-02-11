//! Roundtrip tests for top-level markup/markuplist export.

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
// Top-level markup roundtrip tests
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_toplevel_markup() {
    let src = r#"\markup { "Title Page" }
\score { \new Staff { c'4 } }"#;
    let output = roundtrip(src);
    assert!(
        output.contains("\\markup"),
        "should contain \\markup: {output}"
    );
    assert!(
        output.contains("Title Page"),
        "should contain Title Page: {output}"
    );
}

#[test]
fn roundtrip_toplevel_markuplist() {
    let src = r#"\markuplist { "First" "Second" }
\score { \new Staff { c'4 } }"#;
    let output = roundtrip(src);
    assert!(
        output.contains("\\markuplist"),
        "should contain \\markuplist: {output}"
    );
    assert!(output.contains("First"), "should contain First: {output}");
    assert!(output.contains("Second"), "should contain Second: {output}");
}

#[test]
fn roundtrip_toplevel_markup_ordering() {
    let src = r#"\markup { "Before" }
\score { \new Staff { c'4 } }
\markup { "After" }"#;
    let output = roundtrip(src);
    let before_pos = output.find("Before").expect("should contain Before");
    let after_pos = output.find("After").expect("should contain After");
    let score_pos = output.find("\\score").expect("should contain score");
    assert!(
        before_pos < score_pos,
        "Before should appear before score: {output}"
    );
    assert!(
        after_pos > score_pos,
        "After should appear after score: {output}"
    );
}

#[test]
fn roundtrip_toplevel_mixed_markup_and_markuplist() {
    let src = r#"\markup { "Title" }
\markuplist { "A" "B" }
\score { \new Staff { c'4 } }"#;
    let output = roundtrip(src);
    assert!(
        output.contains("\\markup"),
        "should contain \\markup: {output}"
    );
    assert!(
        output.contains("\\markuplist"),
        "should contain \\markuplist: {output}"
    );
    assert!(output.contains("Title"), "should contain Title: {output}");
    let title_pos = output.find("Title").unwrap();
    let score_pos = output.find("\\score").unwrap();
    assert!(
        title_pos < score_pos,
        "Title should appear before score: {output}"
    );
}

#[test]
fn roundtrip_no_toplevel_markup_unchanged() {
    let src = r#"\score { \new Staff { c'4 d'4 } }"#;
    let ast = roundtrip_ast(src);
    let markup_count = ast
        .items
        .iter()
        .filter(|item| {
            matches!(
                item,
                ToplevelExpression::Markup(_) | ToplevelExpression::MarkupList(_)
            )
        })
        .count();
    assert_eq!(markup_count, 0, "no markups when input has none");
}

#[test]
fn roundtrip_toplevel_markup_ast_variants() {
    let src = r#"\markup \bold "Hello"
\markuplist { "Item" }
\score { \new Staff { c'4 } }"#;
    let ast = roundtrip_ast(src);
    let mut has_markup = false;
    let mut has_markuplist = false;
    for item in &ast.items {
        match item {
            ToplevelExpression::Markup(_) => has_markup = true,
            ToplevelExpression::MarkupList(_) => has_markuplist = true,
            _ => {}
        }
    }
    assert!(has_markup, "should have Markup variant in AST");
    assert!(has_markuplist, "should have MarkupList variant in AST");
}

#[test]
fn roundtrip_toplevel_markup_with_assignments() {
    let src = r#"myVar = "hello"
\markup { "Title" }
\score { \new Staff { c'4 } }"#;
    let output = roundtrip(src);
    assert!(
        output.contains("myVar"),
        "should contain variable: {output}"
    );
    assert!(output.contains("Title"), "should contain Title: {output}");
    assert!(
        output.contains("\\markup"),
        "should contain \\markup: {output}"
    );
}

#[test]
fn roundtrip_toplevel_markup_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_toplevel_markup.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(
        output.contains("\\markup"),
        "should contain \\markup: {output}"
    );
    assert!(
        output.contains("Title Page"),
        "should contain Title Page: {output}"
    );
    assert!(output.contains("\\score"), "should contain score: {output}");
}

//! Roundtrip tests for markup and lyrics import/export.

use super::*;
use crate::import;
use crate::parser::Parser;
use crate::serializer;

/// Parse LilyPond -> import to MEI -> export to LilyPond AST -> serialize.
fn roundtrip(src: &str) -> String {
    let file = Parser::new(src).unwrap().parse().unwrap();
    let (mei, ext_store) = import::import(&file).unwrap();
    let exported = export(&mei, &ext_store).unwrap();
    serializer::serialize(&exported)
}

// ---------------------------------------------------------------------------
// Lyrics roundtrip tests
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_lyrics_addlyrics() {
    let output = roundtrip(r#"\score { { c'4 d'4 e'4 f'4 } \addlyrics { do re mi fa } }"#);
    assert!(output.contains("do"), "lyric 'do': {output}");
    assert!(output.contains("re"), "lyric 're': {output}");
    assert!(output.contains("\\addlyrics"), "addlyrics: {output}");
}

#[test]
fn roundtrip_lyrics_hyphen() {
    let output = roundtrip(r#"\score { { c'4 d'4 e'4 f'4 } \addlyrics { hel -- lo world } }"#);
    assert!(output.contains("hel"), "syllable 'hel': {output}");
    assert!(output.contains("--"), "hyphen: {output}");
    assert!(output.contains("lo"), "syllable 'lo': {output}");
}

#[test]
fn roundtrip_lyrics_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_lyrics.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    // Should contain lyric text
    assert!(output.contains("do"), "lyric 'do': {output}");
    assert!(output.contains("re"), "lyric 're': {output}");
    assert!(output.contains("mi"), "lyric 'mi': {output}");
    assert!(output.contains("fa"), "lyric 'fa': {output}");
}

// ---------------------------------------------------------------------------
// Markup roundtrip tests
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_markup_simple() {
    let output = roundtrip(r#"{ c'4 \markup { Hello World } d'4 }"#);
    assert!(
        output.contains("\\markup"),
        "should contain \\markup: {output}"
    );
    assert!(output.contains("Hello"), "should contain Hello: {output}");
    assert!(output.contains("World"), "should contain World: {output}");
}

#[test]
fn roundtrip_markup_bold() {
    let output = roundtrip(r#"{ c'4 \markup \bold "Hello" d'4 }"#);
    assert!(output.contains("\\bold"), "should contain \\bold: {output}");
    assert!(
        output.contains("\"Hello\""),
        "should contain quoted Hello: {output}"
    );
}

#[test]
fn roundtrip_markup_nested_commands() {
    let output = roundtrip(r#"{ c'4 \markup \bold \italic "styled text" d'4 }"#);
    assert!(output.contains("\\bold"), "should contain \\bold: {output}");
    assert!(
        output.contains("\\italic"),
        "should contain \\italic: {output}"
    );
    assert!(
        output.contains("styled text"),
        "should contain text: {output}"
    );
}

#[test]
fn roundtrip_markup_column() {
    let output = roundtrip(r#"{ c'4 \markup \column { "line one" "line two" "line three" } d'4 }"#);
    assert!(
        output.contains("\\column"),
        "should contain \\column: {output}"
    );
    assert!(
        output.contains("line one"),
        "should contain line one: {output}"
    );
    assert!(
        output.contains("line three"),
        "should contain line three: {output}"
    );
}

#[test]
fn roundtrip_markup_at_start() {
    let output = roundtrip(r#"{ \markup "before" c'4 d'4 }"#);
    assert!(
        output.contains("\\markup"),
        "should contain \\markup: {output}"
    );
    assert!(
        output.contains("\"before\""),
        "should contain before: {output}"
    );
}

#[test]
fn roundtrip_markuplist() {
    let output = roundtrip(r#"{ c'4 \markuplist { "item one" "item two" } d'4 }"#);
    assert!(
        output.contains("\\markuplist"),
        "should contain \\markuplist: {output}"
    );
    assert!(
        output.contains("item one"),
        "should contain item one: {output}"
    );
    assert!(
        output.contains("item two"),
        "should contain item two: {output}"
    );
}

#[test]
fn roundtrip_multiple_markups() {
    let output = roundtrip(r#"{ c'4 \markup "one" d'4 \markup "two" e'4 }"#);
    assert!(output.contains("\"one\""), "should contain one: {output}");
    assert!(output.contains("\"two\""), "should contain two: {output}");
}

#[test]
fn roundtrip_markup_fixture() {
    // The fixture is top-level markups, not in music context
    // Test with music-embedded markup subset
    let src = r#"\score {
  {
    c'4
    \markup { Hello World }
    d'4
    \markup \bold "Hello"
    e'4
    \markup \bold \italic "styled text"
    f'4
  }
}"#;
    let output = roundtrip(src);
    assert!(output.contains("Hello"), "should contain Hello: {output}");
    assert!(output.contains("World"), "should contain World: {output}");
    assert!(output.contains("\\bold"), "should contain \\bold: {output}");
    assert!(
        output.contains("\\italic"),
        "should contain \\italic: {output}"
    );
    assert!(
        output.contains("styled text"),
        "should contain styled text: {output}"
    );
}

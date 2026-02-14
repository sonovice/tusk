//! Roundtrip tests for tempo, mark, and textMark import/export.

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
// Tempo roundtrip
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_tempo_text_and_metronome() {
    let output = roundtrip(r#"{ \tempo "Allegro" 4 = 120 c'4 d'4 }"#);
    assert!(
        output.contains("\\tempo"),
        "should contain \\tempo: {output}"
    );
    assert!(
        output.contains("\"Allegro\""),
        "should contain Allegro: {output}"
    );
    assert!(output.contains("= 120"), "should contain = 120: {output}");
}

#[test]
fn roundtrip_tempo_metronome_only() {
    let output = roundtrip(r#"{ \tempo 2 = 60 c'4 d'4 }"#);
    assert!(
        output.contains("\\tempo"),
        "should contain \\tempo: {output}"
    );
    assert!(output.contains("2 = 60"), "should contain 2 = 60: {output}");
}

#[test]
fn roundtrip_tempo_text_only() {
    let output = roundtrip(r#"{ \tempo "Andante" c'4 d'4 }"#);
    assert!(
        output.contains("\\tempo"),
        "should contain \\tempo: {output}"
    );
    assert!(
        output.contains("\"Andante\""),
        "should contain Andante: {output}"
    );
}

#[test]
fn roundtrip_tempo_range() {
    let output = roundtrip(r#"{ \tempo "Vivace" 4. = 132-144 c'4 d'4 }"#);
    assert!(
        output.contains("\\tempo"),
        "should contain \\tempo: {output}"
    );
    assert!(
        output.contains("132-144"),
        "should contain 132-144: {output}"
    );
    assert!(output.contains("4."), "should contain 4.: {output}");
}

#[test]
fn roundtrip_tempo_between_notes() {
    let output = roundtrip(r#"{ c'4 d'4 \tempo 4 = 100 e'4 f'4 }"#);
    assert!(
        output.contains("\\tempo"),
        "should contain \\tempo: {output}"
    );
    assert!(output.contains("= 100"), "should contain = 100: {output}");
    assert!(output.contains("c'4"), "should contain c'4: {output}");
    assert!(output.contains("e'4"), "should contain e'4: {output}");
}

// ---------------------------------------------------------------------------
// Mark roundtrip
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_mark_default() {
    let output = roundtrip(r#"{ \mark \default c'4 d'4 }"#);
    assert!(output.contains("\\mark"), "should contain \\mark: {output}");
    assert!(
        output.contains("\\default"),
        "should contain \\default: {output}"
    );
}

#[test]
fn roundtrip_mark_string() {
    let output = roundtrip(r#"{ \mark "A" c'4 d'4 }"#);
    assert!(output.contains("\\mark"), "should contain \\mark: {output}");
    assert!(output.contains("\"A\""), "should contain A: {output}");
}

#[test]
fn roundtrip_mark_number() {
    let output = roundtrip(r#"{ \mark 5 c'4 d'4 }"#);
    assert!(output.contains("\\mark"), "should contain \\mark: {output}");
    assert!(output.contains("5"), "should contain 5: {output}");
}

// ---------------------------------------------------------------------------
// TextMark roundtrip
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_textmark() {
    let output = roundtrip(r#"{ \textMark "Fine" c'4 d'4 }"#);
    assert!(
        output.contains("\\textMark"),
        "should contain \\textMark: {output}"
    );
    assert!(output.contains("\"Fine\""), "should contain Fine: {output}");
}

// ---------------------------------------------------------------------------
// Combined roundtrip
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_multiple_tempo_marks() {
    let output = roundtrip(
        r#"{
  \tempo "Allegro" 4 = 120
  c'4 d'4
  \mark \default
  e'4 f'4
  \textMark "Fine"
  g'4 a'4
}"#,
    );
    assert!(
        output.contains("\\tempo"),
        "should contain \\tempo: {output}"
    );
    assert!(
        output.contains("\"Allegro\""),
        "should contain Allegro: {output}"
    );
    assert!(output.contains("\\mark"), "should contain \\mark: {output}");
    assert!(
        output.contains("\\default"),
        "should contain \\default: {output}"
    );
    assert!(
        output.contains("\\textMark"),
        "should contain \\textMark: {output}"
    );
    assert!(output.contains("\"Fine\""), "should contain Fine: {output}");
}

#[test]
fn roundtrip_tempo_marks_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_tempo_marks.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    // Check all tempo/mark/textMark constructs survived
    assert!(
        output.contains("\"Allegro\""),
        "should contain Allegro: {output}"
    );
    assert!(output.contains("= 120"), "should contain = 120: {output}");
    assert!(output.contains("2 = 60"), "should contain 2 = 60: {output}");
    assert!(
        output.contains("\"Andante\""),
        "should contain Andante: {output}"
    );
    assert!(
        output.contains("132-144"),
        "should contain 132-144: {output}"
    );
    assert!(output.contains("\\mark"), "should contain \\mark: {output}");
    assert!(
        output.contains("\\default"),
        "should contain \\default: {output}"
    );
    assert!(output.contains("\"A\""), "should contain mark A: {output}");
    assert!(
        output.contains("\\textMark"),
        "should contain \\textMark: {output}"
    );
    assert!(output.contains("\"Fine\""), "should contain Fine: {output}");
}

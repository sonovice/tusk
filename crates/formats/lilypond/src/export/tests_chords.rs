//! Roundtrip tests for chord-mode import/export.

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

#[test]
fn roundtrip_chords_with_staff() {
    let src = r#"\score { << \chords { c1 d:m } \new Staff { c'1 d'1 } >> }"#;
    let output = roundtrip(src);
    assert!(
        output.contains("\\chordmode"),
        "should contain \\chordmode: {output}"
    );
    assert!(
        output.contains("ChordNames"),
        "should contain ChordNames context: {output}"
    );
    assert!(output.contains("c1"), "should contain c1: {output}");
    assert!(output.contains("d:m"), "should contain d:m: {output}");
}

#[test]
fn roundtrip_chord_quality_preserved() {
    let src =
        r#"\score { << \chords { c:7 c:dim7 c:maj c:aug } \new Staff { c'1 c'1 c'1 c'1 } >> }"#;
    let output = roundtrip(src);
    assert!(output.contains(":7"), "should contain :7: {output}");
    assert!(output.contains(":dim"), "should contain :dim: {output}");
    assert!(output.contains(":maj"), "should contain :maj: {output}");
    assert!(output.contains(":aug"), "should contain :aug: {output}");
}

#[test]
fn roundtrip_chord_inversion_preserved() {
    let src = r#"\score { << \chords { c:dim7/f } \new Staff { c'1 } >> }"#;
    let output = roundtrip(src);
    assert!(
        output.contains("/f"),
        "should contain /f for inversion: {output}"
    );
}

#[test]
fn roundtrip_bare_chordmode() {
    // Bare chord mode without a staff context — roundtrips as chordmode
    let src = "\\chordmode { c1 c:m c:7 c:dim7/f }";
    let output = roundtrip(src);
    assert!(
        output.contains("\\chordmode"),
        "should contain \\chordmode: {output}"
    );
    assert!(output.contains("c1"), "should contain c1: {output}");
    assert!(output.contains(":m"), "should contain :m: {output}");
    assert!(output.contains(":7"), "should contain :7: {output}");
    assert!(output.contains(":dim"), "should contain :dim: {output}");
}

#[test]
fn roundtrip_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_chordmode.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(
        output.contains("\\chordmode"),
        "should contain \\chordmode: {output}"
    );
    assert!(output.contains("c1"), "should contain c1: {output}");
    assert!(output.contains(":m"), "should contain :m: {output}");
    assert!(output.contains(":7"), "should contain :7: {output}");
}

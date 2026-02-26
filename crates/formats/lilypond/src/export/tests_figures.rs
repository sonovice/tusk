//! Roundtrip tests for figured bass import/export.

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
fn roundtrip_figures_basic() {
    let src = "\\figures { <6 4>4 <7 5>4 <3>2 }";
    let output = roundtrip(src);
    assert!(
        output.contains("\\figuremode"),
        "should contain \\figuremode: {output}"
    );
    assert!(
        output.contains("FiguredBass"),
        "should contain FiguredBass context: {output}"
    );
    assert!(
        output.contains("<6 4>4"),
        "should contain <6 4>4: {output}"
    );
    assert!(
        output.contains("<7 5>4"),
        "should contain <7 5>4: {output}"
    );
    assert!(
        output.contains("<3>2"),
        "should contain <3>2: {output}"
    );
}

#[test]
fn roundtrip_figuremode_alterations() {
    let src = "\\figuremode { <6+ 4>4 <7- 5!>2 }";
    let output = roundtrip(src);
    assert!(
        output.contains("<6+ 4>4"),
        "should preserve sharp alteration: {output}"
    );
    assert!(
        output.contains("<7- 5!>2"),
        "should preserve flat and forced natural: {output}"
    );
}

#[test]
fn roundtrip_figuremode_modifications() {
    let src = "\\figuremode { \\<5\\+ 3\\>4 \\<7 5/\\>4 \\<6! _\\>2 }";
    let output = roundtrip(src);
    assert!(
        output.contains("5\\+"),
        "should preserve augmented mod: {output}"
    );
    assert!(
        output.contains("5/"),
        "should preserve diminished: {output}"
    );
    assert!(
        output.contains("6!"),
        "should preserve forced natural: {output}"
    );
    assert!(
        output.contains("_"),
        "should preserve figure space: {output}"
    );
}

#[test]
fn roundtrip_figuremode_brackets() {
    let src = "\\figuremode { \\<[6 4]\\>4 \\<7 [5 3]\\>2 }";
    let output = roundtrip(src);
    assert!(
        output.contains("[6"),
        "should preserve bracket start: {output}"
    );
    assert!(
        output.contains("4]"),
        "should preserve bracket stop: {output}"
    );
}

#[test]
fn roundtrip_fixture_basic() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/figured-bass-basic.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(
        output.contains("\\figuremode"),
        "should contain \\figuremode: {output}"
    );
    assert!(
        output.contains("<6 4>4"),
        "should preserve basic figures: {output}"
    );
}

#[test]
fn roundtrip_fixture_alterations() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/figured-bass-alterations.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(
        output.contains("6+"),
        "should preserve sharp alteration: {output}"
    );
    assert!(
        output.contains("7-"),
        "should preserve flat alteration: {output}"
    );
    assert!(
        output.contains("5!"),
        "should preserve forced natural: {output}"
    );
}

#[test]
fn roundtrip_fixture_modifications() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/figured-bass-modifications.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(
        output.contains("5\\+"),
        "should preserve augmented mod: {output}"
    );
    assert!(
        output.contains("5/"),
        "should preserve diminished mod: {output}"
    );
}

#[test]
fn roundtrip_fixture_brackets() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/figured-bass-brackets.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(
        output.contains("[6"),
        "should preserve bracket start: {output}"
    );
    assert!(
        output.contains("4]"),
        "should preserve bracket end: {output}"
    );
}

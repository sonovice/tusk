//! Roundtrip tests for drum mode import/export.

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

#[test]
fn roundtrip_drummode_basic() {
    let src = "\\new DrumStaff \\drummode { bd4 sn4 hh4 }";
    let output = roundtrip(src);
    assert!(
        output.contains("\\drummode"),
        "should contain \\drummode: {output}"
    );
    assert!(
        output.contains("DrumStaff"),
        "should contain DrumStaff context: {output}"
    );
    assert!(output.contains("bd4"), "should contain bd4: {output}");
    assert!(output.contains("sn4"), "should contain sn4: {output}");
    assert!(output.contains("hh4"), "should contain hh4: {output}");
}

#[test]
fn roundtrip_drummode_long_names() {
    let src = "\\new DrumStaff \\drummode { bassdrum4 snare4 hihat4 }";
    let output = roundtrip(src);
    assert!(
        output.contains("bassdrum4"),
        "should preserve long name bassdrum: {output}"
    );
    assert!(
        output.contains("snare4"),
        "should preserve long name snare: {output}"
    );
    assert!(
        output.contains("hihat4"),
        "should preserve long name hihat: {output}"
    );
}

#[test]
fn roundtrip_drummode_chord() {
    let src = "\\new DrumStaff \\drummode { <bd sn>4 <hh cymr>8 }";
    let output = roundtrip(src);
    assert!(
        output.contains("<bd sn>4"),
        "should preserve drum chord: {output}"
    );
    assert!(
        output.contains("<hh cymr>8"),
        "should preserve drum chord with cymbal: {output}"
    );
}

#[test]
fn roundtrip_drummode_mixed() {
    let src = "\\new DrumStaff \\drummode { bd4 <bd sn>4 hh8 sn8 }";
    let output = roundtrip(src);
    assert!(output.contains("bd4"), "should contain bd4: {output}");
    assert!(
        output.contains("<bd sn>4"),
        "should contain drum chord: {output}"
    );
    assert!(output.contains("hh8"), "should contain hh8: {output}");
    assert!(output.contains("sn8"), "should contain sn8: {output}");
}

#[test]
fn roundtrip_drum_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_drummode.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(
        output.contains("\\drummode"),
        "should contain \\drummode: {output}"
    );
    assert!(output.contains("bd4"), "should contain bd4: {output}");
    assert!(output.contains("sn4"), "should contain sn4: {output}");
    assert!(output.contains("hh4"), "should contain hh4: {output}");
}

#[test]
fn roundtrip_drums_shorthand() {
    // \drums is shorthand for \new DrumStaff \drummode
    let src = "\\drums { bd4 sn4 hh4 }";
    let output = roundtrip(src);
    assert!(
        output.contains("DrumStaff"),
        "should contain DrumStaff context: {output}"
    );
    assert!(
        output.contains("\\drummode"),
        "should contain \\drummode: {output}"
    );
    assert!(output.contains("bd4"), "should contain bd4: {output}");
}

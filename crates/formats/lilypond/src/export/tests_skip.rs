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
fn roundtrip_skip_basic() {
    let output = roundtrip("{ c'4 s4 d'4 }");
    assert!(output.contains("s4"), "skip: {output}");
    assert!(output.contains("c'4"), "note before: {output}");
    assert!(output.contains("d'4"), "note after: {output}");
}

#[test]
fn roundtrip_skip_with_duration() {
    let output = roundtrip("{ s2 }");
    assert!(output.contains("s2"), "skip half: {output}");
}

#[test]
fn roundtrip_skip_dotted() {
    let output = roundtrip("{ s4. }");
    assert!(output.contains("s4."), "dotted skip: {output}");
}

#[test]
fn roundtrip_skip_in_voice() {
    let output = roundtrip("<< { c'4 d'4 } \\\\ { s4 e'4 } >>");
    assert!(output.contains("s4"), "skip in voice: {output}");
    assert!(output.contains("e'4"), "note in voice: {output}");
}

#[test]
fn roundtrip_skip_with_lyrics() {
    // Skip in music context alongside lyrics
    let output = roundtrip("{ c'4 s4 d'4 e'4 } \\addlyrics { one two three }");
    assert!(output.contains("s4"), "skip preserved: {output}");
    assert!(output.contains("\\addlyrics"), "lyrics preserved: {output}");
}

//! Roundtrip tests for music function calls.

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
fn roundtrip_music_function_with_music_arg() {
    let output = roundtrip("{ \\someFunction { c4 d e f } g4 }");
    assert!(
        output.contains("\\someFunction"),
        "output should contain function name: {output}"
    );
}

#[test]
fn roundtrip_music_function_with_string_and_music() {
    let output = roundtrip("{ \\tag \"part\" { c4 d e f } g4 }");
    assert!(
        output.contains("\\tag \"part\""),
        "output should contain tag with string: {output}"
    );
}

#[test]
fn roundtrip_music_function_with_numeric_arg() {
    let output = roundtrip("{ \\magnifyMusic 0.63 { c4 d e f } g4 }");
    assert!(
        output.contains("\\magnifyMusic 0.63"),
        "output should contain magnifyMusic with number: {output}"
    );
}

#[test]
fn roundtrip_partial_function() {
    let output = roundtrip("{ \\tag #'score \\etc c4 d e f }");
    assert!(
        output.contains("\\tag #'score \\etc"),
        "output should contain partial function: {output}"
    );
}

#[test]
fn roundtrip_multiple_function_calls() {
    let output = roundtrip("{ \\someFunc { c4 } \\otherFunc { d4 } e4 }");
    assert!(
        output.contains("\\someFunc"),
        "output should contain someFunc: {output}"
    );
    assert!(
        output.contains("\\otherFunc"),
        "output should contain otherFunc: {output}"
    );
}

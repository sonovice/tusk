//! Roundtrip tests for property operations (override/set/revert/unset/once/tweak).

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
fn roundtrip_override() {
    let output = roundtrip("{ \\override NoteHead.color = #red c4 d e f }");
    assert!(
        output.contains("\\override NoteHead.color = #red"),
        "output: {output}"
    );
    assert!(output.contains("c4"), "output: {output}");
}

#[test]
fn roundtrip_override_with_context() {
    let output = roundtrip("{ \\override Staff.TimeSignature.color = #green c4 d e f }");
    assert!(
        output.contains("\\override Staff.TimeSignature.color = #green"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_override_numeric_value() {
    let output = roundtrip("{ \\override Beam.gap-count = 5 c4 d e f }");
    assert!(
        output.contains("\\override Beam.gap-count = 5"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_revert() {
    let output = roundtrip("{ \\override NoteHead.color = #red c4 d \\revert NoteHead.color e f }");
    assert!(
        output.contains("\\override NoteHead.color = #red"),
        "output: {output}"
    );
    assert!(
        output.contains("\\revert NoteHead.color"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_set() {
    let output = roundtrip("{ \\set Staff.instrumentName = \"Piano\" c4 d e f }");
    assert!(
        output.contains("\\set Staff.instrumentName = \"Piano\""),
        "output: {output}"
    );
}

#[test]
fn roundtrip_set_boolean() {
    let output = roundtrip("{ \\set Staff.useBassFigureExtenders = ##t c4 d e f }");
    assert!(
        output.contains("\\set Staff.useBassFigureExtenders = ##t"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_unset() {
    let output = roundtrip("{ \\unset Staff.keyAlterations c4 d e f }");
    assert!(
        output.contains("\\unset Staff.keyAlterations"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_once_override() {
    let output = roundtrip("{ \\once \\override NoteHead.color = #red c4 d e f }");
    assert!(
        output.contains("\\once \\override NoteHead.color = #red"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_multiple_ops() {
    let output = roundtrip(
        "{ \\override NoteHead.color = #red \\set Staff.instrumentName = \"Violin\" c4 d e f \\revert NoteHead.color g a b c }",
    );
    assert!(
        output.contains("\\override NoteHead.color = #red"),
        "output: {output}"
    );
    assert!(
        output.contains("\\set Staff.instrumentName = \"Violin\""),
        "output: {output}"
    );
    assert!(
        output.contains("\\revert NoteHead.color"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_tweak_on_note() {
    let output = roundtrip("{ c4\\tweak color #red -. }");
    assert!(output.contains("\\tweak color #red"), "output: {output}");
}

#[test]
fn roundtrip_tweak_with_path() {
    let output = roundtrip("{ c4\\tweak NoteHead.color #blue -. }");
    assert!(
        output.contains("\\tweak NoteHead.color #blue"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_override_compound_scheme() {
    let output = roundtrip("{ \\override NoteHead.color = #(rgb-color 1 0 0) c4 d e f }");
    assert!(
        output.contains("\\override NoteHead.color = #(rgb-color 1 0 0)"),
        "output: {output}"
    );
}

// --- Scheme property path roundtrips (Phase 42.2) ---

#[test]
fn roundtrip_override_scheme_symbol_path() {
    let output = roundtrip("{ \\override #'font-size = #3 c4 d e f }");
    assert!(
        output.contains("\\override #'font-size = #3"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_override_mixed_dot_scheme_path() {
    let output = roundtrip("{ \\override Staff.NoteHead #'font-size = #3 c4 d e f }");
    assert!(
        output.contains("\\override Staff.NoteHead #'font-size = #3"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_revert_scheme_quoted_list() {
    let output = roundtrip(
        "{ \\override NoteHead.color = #red c4 \\revert #'(bound-details left text) d e f }",
    );
    assert!(
        output.contains("\\revert #'(bound-details left text)"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_revert_scheme_symbol() {
    let output = roundtrip("{ \\override NoteHead.color = #red c4 \\revert #'font-size d e f }");
    assert!(output.contains("\\revert #'font-size"), "output: {output}");
}

#[test]
fn roundtrip_revert_context_scheme_path() {
    let output =
        roundtrip("{ \\override NoteHead.color = #red c4 \\revert Staff #'fontSize d e f }");
    assert!(
        output.contains("\\revert Staff #'fontSize"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_once_override_scheme_path() {
    let output = roundtrip("{ \\once \\override #'font-size = #5 c4 d e f }");
    assert!(
        output.contains("\\once \\override #'font-size = #5"),
        "output: {output}"
    );
}

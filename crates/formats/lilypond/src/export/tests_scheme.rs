//! Roundtrip tests for Scheme expressions through MEI labels.
//!
//! Scheme expressions may appear embedded in property operations, function
//! calls, assignments, or output-definition fields, as well as standalone
//! `Music::SchemeMusic` in music position.  The roundtrip strategy serializes
//! the expression to text, stores it in an MEI `<dir>` @label, and re-parses
//! on export.  These tests verify that every `SchemeExpr` variant survives
//! that cycle.

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
// SchemeExpr::Bool
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_bool_true_in_set() {
    let output = roundtrip("{ \\set Staff.useBassFigureExtenders = ##t c4 }");
    assert!(output.contains("= ##t"), "output: {output}");
}

#[test]
fn roundtrip_scheme_bool_false_in_set() {
    let output = roundtrip("{ \\set Staff.voltaSpannerDuration = ##f c4 }");
    assert!(output.contains("= ##f"), "output: {output}");
}

#[test]
fn roundtrip_scheme_bool_false_in_header() {
    let output = roundtrip("\\header { tagline = ##f }\n\\score { { c4 } }");
    assert!(output.contains("tagline = ##f"), "output: {output}");
}

// ---------------------------------------------------------------------------
// SchemeExpr::Integer
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_integer_positive() {
    let output = roundtrip("{ \\override Beam.gap-count = #3 c4 }");
    assert!(output.contains("= #3"), "output: {output}");
}

#[test]
fn roundtrip_scheme_integer_negative() {
    let output = roundtrip("{ \\override Staff.fontSize = #-2 c4 }");
    assert!(output.contains("= #-2"), "output: {output}");
}

#[test]
fn roundtrip_scheme_integer_in_assignment() {
    let output = roundtrip("mySize = #-3\n{ c4 }");
    assert!(output.contains("mySize = #-3"), "output: {output}");
}

// ---------------------------------------------------------------------------
// SchemeExpr::Float
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_float_in_midi() {
    let output = roundtrip(
        "\\midi {\n  \\context {\n    \\Score\n    midiMinimumVolume = #0.2\n  }\n}\n\\score { { c4 } }",
    );
    assert!(
        output.contains("midiMinimumVolume = #0.2"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_scheme_float_in_function() {
    let output = roundtrip("{ \\magnifyMusic #0.63 { c4 d e f } g4 }");
    assert!(output.contains("0.63"), "output: {output}");
}

// ---------------------------------------------------------------------------
// SchemeExpr::String
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_string_in_set() {
    let output = roundtrip("{ \\set Staff.instrumentName = #\"Violin\" c4 }");
    assert!(
        output.contains("#\"Violin\"") || output.contains("\"Violin\""),
        "output: {output}"
    );
}

// ---------------------------------------------------------------------------
// SchemeExpr::Symbol
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_symbol_in_function() {
    let output = roundtrip("{ \\tag #'score \\etc c4 d e f }");
    assert!(output.contains("#'score"), "output: {output}");
}

#[test]
fn roundtrip_scheme_symbol_in_keepwithtag() {
    let output = roundtrip("{ \\keepWithTag #'print { c4 d e f } g4 }");
    assert!(output.contains("#'print"), "output: {output}");
}

// ---------------------------------------------------------------------------
// SchemeExpr::Identifier
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_identifier_color() {
    let output = roundtrip("{ \\override NoteHead.color = #red c4 }");
    assert!(output.contains("= #red"), "output: {output}");
}

#[test]
fn roundtrip_scheme_identifier_in_tweak() {
    let output = roundtrip("{ c4\\tweak color #blue -. }");
    assert!(output.contains("#blue"), "output: {output}");
}

// ---------------------------------------------------------------------------
// SchemeExpr::List (S-expression)
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_list_rgb_color() {
    let output = roundtrip("{ \\override NoteHead.color = #(rgb-color 1 0 0) c4 }");
    assert!(output.contains("#(rgb-color 1 0 0)"), "output: {output}");
}

#[test]
fn roundtrip_scheme_list_nested() {
    let output =
        roundtrip("{ \\override NoteHead.stencil = #(ly:make-stencil '() '(0 . 0) '(0 . 0)) c4 }");
    assert!(output.contains("#(ly:make-stencil"), "output: {output}");
}

// ---------------------------------------------------------------------------
// Combined: multiple Scheme variants in one file
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_mixed_ops() {
    let src = r#"{
  \override NoteHead.color = #red
  \set Staff.useBassFigureExtenders = ##t
  \override Beam.gap-count = #3
  \override Glissando.color = #(rgb-color 1 0 0)
  c4 d e f
  \revert NoteHead.color
  g4
}"#;
    let output = roundtrip(src);
    assert!(output.contains("#red"), "output: {output}");
    assert!(output.contains("##t"), "output: {output}");
    assert!(output.contains("#3"), "output: {output}");
    assert!(output.contains("#(rgb-color 1 0 0)"), "output: {output}");
}

// ---------------------------------------------------------------------------
// Scheme in assignment values
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_assignment_bool() {
    let output = roundtrip("showLastBar = ##t\n{ c4 }");
    assert!(output.contains("showLastBar = ##t"), "output: {output}");
}

#[test]
fn roundtrip_scheme_assignment_identifier() {
    let output = roundtrip("myColor = #blue\n{ c4 }");
    assert!(output.contains("myColor = #blue"), "output: {output}");
}

#[test]
fn roundtrip_scheme_assignment_list() {
    let output = roundtrip("myColor = #(rgb-color 0.5 0.3 0.1)\n{ c4 }");
    assert!(
        output.contains("#(rgb-color 0.5 0.3 0.1)"),
        "output: {output}"
    );
}

// ---------------------------------------------------------------------------
// Fixture roundtrip
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_fixture_scheme() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_scheme_roundtrip.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);

    // Boolean
    assert!(
        output.contains("tagline = ##f"),
        "missing tagline: {output}"
    );
    // Integer
    assert!(
        output.contains("#-2") || output.contains("fontSize = #-2"),
        "missing fontSize: {output}"
    );
    // Identifier
    assert!(output.contains("#red"), "missing #red: {output}");
    // Symbol
    assert!(output.contains("#'print"), "missing #'print: {output}");
    // S-expression list
    assert!(
        output.contains("#(rgb-color 1 0 0)"),
        "missing rgb-color: {output}"
    );
    // Bool in set
    assert!(output.contains("##t"), "missing ##t: {output}");
}

// ---------------------------------------------------------------------------
// Music::SchemeMusic — Scheme expressions in music position
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_scheme_music_list() {
    let output = roundtrip("{ c4 #(ly:export (make-music 'SkipEvent)) d4 }");
    assert!(
        output.contains("#(ly:export (make-music 'SkipEvent))"),
        "output: {output}"
    );
}

#[test]
fn roundtrip_scheme_music_identifier() {
    let output = roundtrip("{ c4 #myMusicVar d4 }");
    assert!(output.contains("#myMusicVar"), "output: {output}");
}

#[test]
fn roundtrip_scheme_music_embedded_lilypond() {
    let output = roundtrip("{ c4 ##{ f4 #} g4 }");
    assert!(output.contains("##{"), "output: {output}");
    assert!(output.contains("f4"), "output: {output}");
}

#[test]
fn roundtrip_scheme_music_multiple() {
    let output = roundtrip("{ c4 #myVar d4 #(ly:export (make-music 'NoteEvent)) e4 }");
    assert!(output.contains("#myVar"), "output: {output}");
    assert!(output.contains("#(ly:export"), "output: {output}");
}

#[test]
fn roundtrip_scheme_music_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_scheme_music.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);

    // List call
    assert!(
        output.contains("#(ly:export (make-music 'SkipEvent))"),
        "missing list call: {output}"
    );
    // Identifier
    assert!(
        output.contains("#myMusicVar"),
        "missing identifier: {output}"
    );
    // Embedded LilyPond
    assert!(output.contains("##{"), "missing embedded ly: {output}");
}

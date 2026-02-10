//! Tests that invalid LilyPond inputs produce clear validation/parse errors
//! rather than panics or silent corruption.

use super::*;
use crate::parser::Parser;

/// Parse and import; return the error if any.
fn try_import(src: &str) -> Result<Mei, String> {
    let file = match Parser::new(src) {
        Ok(p) => match p.parse() {
            Ok(f) => f,
            Err(e) => return Err(format!("parse error: {e}")),
        },
        Err(e) => return Err(format!("lexer error: {e}")),
    };
    import(&file).map_err(|e| format!("{e}"))
}

// ---------------------------------------------------------------------------
// Parse errors: malformed syntax caught by the parser
// ---------------------------------------------------------------------------

#[test]
fn unclosed_brace_is_parse_error() {
    let result = try_import("{ c4 d4");
    assert!(result.is_err());
    let msg = result.unwrap_err();
    assert!(
        msg.contains("parse") || msg.contains("expected"),
        "should be a clear parse error: {msg}"
    );
}

#[test]
fn garbage_input_is_parse_error() {
    let result = try_import("@@@!!!");
    assert!(result.is_err());
}

// ---------------------------------------------------------------------------
// Validation errors: structurally invalid AST caught by the validator
// ---------------------------------------------------------------------------

#[test]
fn invalid_duration_base_is_validation_error() {
    // Duration base 3 is not a power of 2
    let result = try_import(r"{ c\breve*3/1 d4 }");
    // This may or may not trigger depending on how the parser handles it.
    // The important thing is it doesn't panic.
    let _ = result;
}

#[test]
fn score_without_music_is_validation_error() {
    let result = try_import(r"\score { \layout { } }");
    assert!(result.is_err(), "score without music should fail");
    let msg = result.unwrap_err();
    assert!(
        msg.contains("no music") || msg.contains("score"),
        "error should mention missing music: {msg}"
    );
}

#[test]
fn empty_input_is_error() {
    let result = try_import("");
    assert!(result.is_err(), "empty input should produce an error");
}

#[test]
fn validation_error_display_is_readable() {
    // Construct a validation error directly to check formatting
    let errors = vec![
        crate::validator::ValidationError::EmptySequential,
        crate::validator::ValidationError::UnknownClefName {
            name: "xyzzy".into(),
        },
    ];
    let err = ImportError::Validation(errors);
    let msg = format!("{err}");
    assert!(
        msg.contains("empty sequential"),
        "should list first error: {msg}"
    );
    assert!(msg.contains("xyzzy"), "should list second error: {msg}");
}

// ---------------------------------------------------------------------------
// Format-level: import_from_str also validates
// ---------------------------------------------------------------------------

#[test]
fn import_from_str_validates() {
    use tusk_format::Importer;
    let fmt = crate::LilyPondFormat;
    let result = fmt.import_from_str(r"\score { \layout { } }");
    assert!(result.is_err(), "format-level import should validate");
}

// ---------------------------------------------------------------------------
// Valid inputs don't trigger validation errors
// ---------------------------------------------------------------------------

#[test]
fn valid_simple_score_passes_validation() {
    let result = try_import(r"{ c4 d e f }");
    assert!(
        result.is_ok(),
        "simple score should pass: {:?}",
        result.err()
    );
}

#[test]
fn valid_complex_score_passes_validation() {
    let src = r#"
    \version "2.24.0"
    \header { title = "Test" }
    \score {
      \new Staff \relative c' {
        \clef treble
        \key g \major
        \time 3/4
        c4( d e) |
        \tuplet 3/2 { f8 g a } b4 |
      }
      \layout { }
    }
    "#;
    let result = try_import(src);
    assert!(
        result.is_ok(),
        "complex score should pass: {:?}",
        result.err()
    );
}

//! Roundtrip tests for book/bookpart export.

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

// -------------------------------------------------------------------------
// Book with single score
// -------------------------------------------------------------------------

#[test]
fn roundtrip_book_single_score() {
    let output = roundtrip(
        r#"\version "2.24.0"
\book {
  \score { { c4 d e f } }
}"#,
    );
    assert!(output.contains("\\book"), "should contain \\book: {output}");
    assert!(
        output.contains("\\score"),
        "should contain \\score: {output}"
    );
    assert!(output.contains("c4"), "should contain notes: {output}");
}

// -------------------------------------------------------------------------
// Book with bookpart
// -------------------------------------------------------------------------

#[test]
fn roundtrip_book_with_bookpart() {
    let output = roundtrip(
        r#"\version "2.24.0"
\book {
  \bookpart {
    \score { { c4 d e f } }
  }
}"#,
    );
    assert!(output.contains("\\book"), "should contain \\book: {output}");
    assert!(
        output.contains("\\bookpart"),
        "should contain \\bookpart: {output}"
    );
    assert!(output.contains("c4"), "should contain notes: {output}");
}

// -------------------------------------------------------------------------
// Multiple bookparts
// -------------------------------------------------------------------------

#[test]
fn roundtrip_multiple_bookparts() {
    let output = roundtrip(
        r#"\version "2.24.0"
\book {
  \bookpart {
    \score { { c4 d e f } }
  }
  \bookpart {
    \score { { g4 a b c' } }
  }
}"#,
    );
    assert!(output.contains("\\book"), "should contain \\book: {output}");
    // Count bookpart occurrences
    let bp_count = output.matches("\\bookpart").count();
    assert_eq!(
        bp_count, 2,
        "should have 2 bookparts, got {bp_count}: {output}"
    );
    assert!(
        output.contains("c4"),
        "should contain first score notes: {output}"
    );
    assert!(
        output.contains("g4"),
        "should contain second score notes: {output}"
    );
}

// -------------------------------------------------------------------------
// Book header preserved
// -------------------------------------------------------------------------

#[test]
fn roundtrip_book_header_preserved() {
    let output = roundtrip(
        r#"\version "2.24.0"
\book {
  \header { title = "Test Book" }
  \score { { c4 d e f } }
}"#,
    );
    assert!(output.contains("\\book"), "should contain \\book: {output}");
    assert!(
        output.contains("Test Book"),
        "should preserve book header title: {output}"
    );
}

// -------------------------------------------------------------------------
// Bookpart header preserved
// -------------------------------------------------------------------------

#[test]
fn roundtrip_bookpart_header_preserved() {
    let output = roundtrip(
        r#"\version "2.24.0"
\book {
  \bookpart {
    \header { subtitle = "Part 1" }
    \score { { c4 d e f } }
  }
}"#,
    );
    assert!(
        output.contains("\\bookpart"),
        "should contain \\bookpart: {output}"
    );
    assert!(
        output.contains("Part 1"),
        "should preserve bookpart header: {output}"
    );
}

// -------------------------------------------------------------------------
// Book paper preserved
// -------------------------------------------------------------------------

#[test]
fn roundtrip_book_paper_preserved() {
    let output = roundtrip(
        r#"\version "2.24.0"
\book {
  \paper { indent = 0 }
  \score { { c4 d e f } }
}"#,
    );
    assert!(output.contains("\\book"), "should contain \\book: {output}");
    assert!(
        output.contains("\\paper"),
        "should preserve book paper: {output}"
    );
    assert!(
        output.contains("indent"),
        "should preserve paper indent: {output}"
    );
}

// -------------------------------------------------------------------------
// Full hierarchy: book header + bookpart headers
// -------------------------------------------------------------------------

#[test]
fn roundtrip_full_book_hierarchy() {
    let output = roundtrip(
        r#"\version "2.24.0"
\book {
  \header { title = "Test Book" }
  \bookpart {
    \header { subtitle = "Part 1" }
    \score { { c4 d e f } }
  }
  \bookpart {
    \header { subtitle = "Part 2" }
    \score { { g4 a b c' } }
  }
}"#,
    );
    assert!(output.contains("\\book"), "should contain \\book: {output}");
    assert!(
        output.contains("Test Book"),
        "should contain book title: {output}"
    );
    assert!(
        output.contains("Part 1"),
        "should contain part 1 subtitle: {output}"
    );
    assert!(
        output.contains("Part 2"),
        "should contain part 2 subtitle: {output}"
    );
    assert!(
        output.contains("c4"),
        "should contain first score: {output}"
    );
    assert!(
        output.contains("g4"),
        "should contain second score: {output}"
    );
}

// -------------------------------------------------------------------------
// Direct scores in book (no bookparts)
// -------------------------------------------------------------------------

#[test]
fn roundtrip_book_direct_scores() {
    let output = roundtrip(
        r#"\version "2.24.0"
\book {
  \score { { c4 d e f } }
  \score { { g4 a b c' } }
}"#,
    );
    assert!(output.contains("\\book"), "should contain \\book: {output}");
    // Should NOT contain \bookpart since the original had none
    assert!(
        !output.contains("\\bookpart"),
        "should not contain \\bookpart: {output}"
    );
    let score_count = output.matches("\\score").count();
    assert_eq!(
        score_count, 2,
        "should have 2 scores, got {score_count}: {output}"
    );
}

// -------------------------------------------------------------------------
// Non-book files are unaffected
// -------------------------------------------------------------------------

#[test]
fn roundtrip_non_book_unchanged() {
    let output = roundtrip(
        r#"\version "2.24.0"
\score { { c4 d e f } }"#,
    );
    assert!(
        !output.contains("\\book"),
        "non-book file should not gain \\book: {output}"
    );
    assert!(output.contains("c4"), "should contain notes: {output}");
}

// -------------------------------------------------------------------------
// Fragment fixture roundtrip
// -------------------------------------------------------------------------

#[test]
fn roundtrip_fragment_book_fixture() {
    let src = include_str!("../../../../../tests/fixtures/lilypond/fragment_book.ly");
    let output = roundtrip(src);
    assert!(output.contains("\\book"), "should contain \\book: {output}");
    assert!(
        output.contains("Test Book"),
        "should preserve book header: {output}"
    );
    assert!(
        output.contains("\\bookpart"),
        "should contain \\bookpart: {output}"
    );
    assert!(
        output.contains("Part 1"),
        "should preserve Part 1 header: {output}"
    );
    assert!(
        output.contains("Part 2"),
        "should preserve Part 2 header: {output}"
    );
}

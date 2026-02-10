//! Roundtrip tests for header, paper, layout, and midi blocks.

use crate::export::export;
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
fn roundtrip_header_basic() {
    let output = roundtrip("\\header { title = \"Test\" }\n\\score { { c4 } }");
    assert!(
        output.contains("\\header"),
        "missing \\header, got: {output}"
    );
    assert!(
        output.contains("title = \"Test\""),
        "missing title, got: {output}"
    );
}

#[test]
fn roundtrip_header_multiple_fields() {
    let output = roundtrip(
        "\\header { title = \"Suite\" composer = \"Bach\" opus = \"BWV 1\" }\n\\score { { c4 } }",
    );
    assert!(output.contains("title = \"Suite\""), "got: {output}");
    assert!(output.contains("composer = \"Bach\""), "got: {output}");
    assert!(output.contains("opus = \"BWV 1\""), "got: {output}");
}

#[test]
fn roundtrip_header_scheme_value() {
    let output = roundtrip("\\header { tagline = ##f }\n\\score { { c4 } }");
    assert!(output.contains("\\header"), "got: {output}");
    assert!(output.contains("tagline = ##f"), "got: {output}");
}

#[test]
fn roundtrip_paper_basic() {
    let output = roundtrip("\\paper { indent = 0 }\n\\score { { c4 } }");
    assert!(output.contains("\\paper"), "missing \\paper, got: {output}");
    assert!(output.contains("indent = 0"), "got: {output}");
}

#[test]
fn roundtrip_paper_multiple_fields() {
    let output = roundtrip("\\paper { indent = 0 ragged-right = ##t }\n\\score { { c4 } }");
    assert!(output.contains("indent = 0"), "got: {output}");
    assert!(output.contains("ragged-right = ##t"), "got: {output}");
}

#[test]
fn roundtrip_layout_empty() {
    let output = roundtrip("\\score { { c4 } \\layout { } }");
    assert!(output.contains("\\layout { }"), "got: {output}");
}

#[test]
fn roundtrip_layout_with_context() {
    let src = "\\layout {\n  \\context {\n    \\Score\n    \\remove \"Bar_number_engraver\"\n  }\n}\n\\score { { c4 } }";
    let output = roundtrip(src);
    assert!(output.contains("\\layout"), "got: {output}");
    assert!(output.contains("\\context"), "got: {output}");
    assert!(output.contains("\\Score"), "got: {output}");
    assert!(
        output.contains("\\remove \"Bar_number_engraver\"")
            || output.contains("\\remove Bar_number_engraver"),
        "got: {output}"
    );
}

#[test]
fn roundtrip_midi_empty() {
    let output = roundtrip("\\score { { c4 } \\midi { } }");
    assert!(output.contains("\\midi { }"), "got: {output}");
}

#[test]
fn roundtrip_score_header() {
    let output = roundtrip("\\score { { c4 } \\header { piece = \"Intro\" } }");
    assert!(output.contains("\\header"), "got: {output}");
    assert!(output.contains("piece = \"Intro\""), "got: {output}");
}

#[test]
fn roundtrip_score_layout_and_midi() {
    let output = roundtrip("\\score { { c4 } \\layout { ragged-right = ##t } \\midi { } }");
    assert!(output.contains("\\layout"), "got: {output}");
    assert!(output.contains("ragged-right = ##t"), "got: {output}");
    assert!(output.contains("\\midi"), "got: {output}");
}

#[test]
fn roundtrip_header_fixture() {
    let src = r#"\version "2.24.0"

\header {
  title = "Test Suite"
  subtitle = "For Header Parsing"
  composer = "J.S. Bach"
  arranger = "Claude"
  poet = "Anonymous"
  opus = "BWV 1"
  piece = "Prelude"
  dedication = "For testing"
  copyright = "Public Domain"
  tagline = ##f
}

\score {
  \new Staff { c4 d e f }
  \header {
    piece = "Nested Header"
  }
  \layout { }
  \midi { }
}
"#;
    let output = roundtrip(src);

    // Top-level header fields
    assert!(output.contains("title = \"Test Suite\""), "got: {output}");
    assert!(output.contains("composer = \"J.S. Bach\""), "got: {output}");
    assert!(output.contains("tagline = ##f"), "got: {output}");

    // Score-level header
    assert!(
        output.contains("piece = \"Nested Header\""),
        "got: {output}"
    );

    // Score-level layout and midi
    assert!(output.contains("\\layout"), "got: {output}");
    assert!(output.contains("\\midi"), "got: {output}");
}

#[test]
fn roundtrip_paper_layout_midi_fixture() {
    let src = r#"\version "2.24.0"

\paper {
  indent = 0
  ragged-right = ##t
  ragged-last = ##f
}

\layout {
  \context {
    \Score
    \remove "Bar_number_engraver"
  }
  \context {
    \Staff
    \consists "Span_arpeggio_engraver"
  }
}

\midi {
  \context {
    \Score
    midiMinimumVolume = #0.2
  }
}

\score {
  \new Staff { c4 d e f }
  \layout {
    ragged-right = ##t
  }
  \midi { }
}
"#;
    let output = roundtrip(src);

    // Top-level \paper
    assert!(output.contains("\\paper"), "got: {output}");
    assert!(output.contains("indent = 0"), "got: {output}");
    assert!(output.contains("ragged-right = ##t"), "got: {output}");

    // Top-level \layout with context
    assert!(output.contains("\\layout"), "got: {output}");
    assert!(output.contains("\\context"), "got: {output}");
    assert!(output.contains("\\Score"), "got: {output}");

    // Top-level \midi with context
    assert!(output.contains("\\midi"), "got: {output}");

    // Score-level blocks
    assert!(
        output.matches("\\layout").count() >= 2,
        "should have top-level + score layout, got: {output}"
    );
    assert!(
        output.matches("\\midi").count() >= 2,
        "should have top-level + score midi, got: {output}"
    );
}

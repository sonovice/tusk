use super::*;

// ── Header ───────────────────────────────────────────────────────────

#[test]
fn parse_full_header() {
    let input = r#"\header {
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
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Header(hb) => {
            assert_eq!(hb.fields.len(), 10);
            assert_eq!(hb.fields[0].name, "title");
            assert_eq!(
                hb.fields[0].value,
                AssignmentValue::String("Test Suite".into())
            );
            assert_eq!(hb.fields[9].name, "tagline");
            assert!(matches!(
                &hb.fields[9].value,
                AssignmentValue::SchemeExpr(_)
            ));
        }
        other => panic!("expected header, got {other:?}"),
    }
}

#[test]
fn parse_header_with_markup_value() {
    let input = r#"\header {
  title = \markup \bold "Title"
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Header(hb) => {
            assert_eq!(hb.fields.len(), 1);
            assert!(matches!(&hb.fields[0].value, AssignmentValue::Markup(_)));
        }
        other => panic!("expected header, got {other:?}"),
    }
}

#[test]
fn parse_score_nested_header() {
    let input = r#"\score {
  { c4 }
  \header {
    piece = "Nested"
  }
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Score(sb) => {
            assert_eq!(sb.items.len(), 2);
            match &sb.items[1] {
                ScoreItem::Header(hb) => {
                    assert_eq!(hb.fields[0].name, "piece");
                }
                other => panic!("expected header, got {other:?}"),
            }
        }
        other => panic!("expected score, got {other:?}"),
    }
}

// ── Paper ────────────────────────────────────────────────────────────

#[test]
fn parse_top_level_paper() {
    let input = r#"\paper {
  indent = 0
  ragged-right = ##t
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(pb.body.len(), 2);
            assert_eq!(pb.body[0].name, "indent");
            assert_eq!(pb.body[0].value, AssignmentValue::Number(0.0));
            assert_eq!(pb.body[1].name, "ragged-right");
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn parse_book_with_paper() {
    let input = r#"\book {
  \paper {
    indent = 0
  }
  \score {
    { c4 }
  }
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Book(bb) => {
            assert_eq!(bb.items.len(), 2);
            assert!(matches!(&bb.items[0], BookItem::Paper(_)));
            assert!(matches!(&bb.items[1], BookItem::Score(_)));
        }
        other => panic!("expected book, got {other:?}"),
    }
}

// ── Layout ───────────────────────────────────────────────────────────

#[test]
fn parse_top_level_layout() {
    let input = r#"\layout {
  ragged-right = ##t
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Layout(lb) => {
            assert_eq!(lb.body.len(), 1);
            match &lb.body[0] {
                LayoutItem::Assignment(a) => {
                    assert_eq!(a.name, "ragged-right");
                }
                other => panic!("expected assignment, got {other:?}"),
            }
        }
        other => panic!("expected layout, got {other:?}"),
    }
}

#[test]
fn parse_layout_with_context() {
    let input = r#"\layout {
  \context {
    \Score
    \remove "Bar_number_engraver"
  }
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Layout(lb) => {
            assert_eq!(lb.body.len(), 1);
            match &lb.body[0] {
                LayoutItem::ContextBlock(cb) => {
                    assert_eq!(cb.items.len(), 2);
                    assert!(matches!(&cb.items[0], ContextModItem::ContextRef(s) if s == "Score"));
                    assert!(
                        matches!(&cb.items[1], ContextModItem::Remove(s) if s == "Bar_number_engraver")
                    );
                }
                other => panic!("expected context block, got {other:?}"),
            }
        }
        other => panic!("expected layout, got {other:?}"),
    }
}

#[test]
fn parse_layout_mixed_assignments_and_contexts() {
    let input = r#"\layout {
  ragged-right = ##t
  \context {
    \Staff
    \consists "Span_arpeggio_engraver"
  }
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Layout(lb) => {
            assert_eq!(lb.body.len(), 2);
            assert!(matches!(&lb.body[0], LayoutItem::Assignment(_)));
            assert!(matches!(&lb.body[1], LayoutItem::ContextBlock(_)));
        }
        other => panic!("expected layout, got {other:?}"),
    }
}

// ── MIDI ─────────────────────────────────────────────────────────────

#[test]
fn parse_top_level_midi() {
    let input = r#"\midi { }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Midi(mb) => {
            assert!(mb.body.is_empty());
        }
        other => panic!("expected midi, got {other:?}"),
    }
}

#[test]
fn parse_midi_with_context() {
    let input = r#"\midi {
  \context {
    \Score
  }
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Midi(mb) => {
            assert_eq!(mb.body.len(), 1);
            assert!(matches!(&mb.body[0], MidiItem::ContextBlock(_)));
        }
        other => panic!("expected midi, got {other:?}"),
    }
}

#[test]
fn parse_midi_with_assignment() {
    let input = r#"\midi {
  midiMinimumVolume = #0.2
}"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Midi(mb) => {
            assert_eq!(mb.body.len(), 1);
            match &mb.body[0] {
                MidiItem::Assignment(a) => {
                    assert_eq!(a.name, "midiMinimumVolume");
                }
                other => panic!("expected assignment, got {other:?}"),
            }
        }
        other => panic!("expected midi, got {other:?}"),
    }
}

// ── Roundtrip ────────────────────────────────────────────────────────

#[test]
fn roundtrip_header_block() {
    let input = r#"\header {
  title = "Test"
  composer = "Bach"
}"#;
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_paper_block() {
    let input = r#"\paper {
  indent = 0
  ragged-right = ##t
}"#;
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_layout_with_context() {
    let input = r#"\layout {
  \context {
    \Score
    \remove "Bar_number_engraver"
  }
}"#;
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_midi_empty() {
    let input = r#"\midi { }"#;
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_score_with_all_blocks() {
    let input = r#"\score {
  { c4 d e f }
  \header {
    piece = "Test"
  }
  \layout {
    ragged-right = ##t
  }
  \midi { }
}"#;
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

// ── Fixture tests ────────────────────────────────────────────────────

#[test]
fn parse_fixture_header() {
    let input = include_str!("../../../../../tests/fixtures/lilypond/fragment_header.ly");
    let ast = parse(input).unwrap();
    // Should have: header, score
    assert_eq!(ast.items.len(), 2);
    assert!(matches!(&ast.items[0], ToplevelExpression::Header(_)));
    assert!(matches!(&ast.items[1], ToplevelExpression::Score(_)));

    // Validate the AST
    crate::validator::validate(&ast).unwrap();
}

#[test]
fn parse_fixture_paper_layout_midi() {
    let input =
        include_str!("../../../../../tests/fixtures/lilypond/fragment_paper_layout_midi.ly");
    let ast = parse(input).unwrap();
    // Should have: paper, layout, midi, score
    assert_eq!(ast.items.len(), 4);
    assert!(matches!(&ast.items[0], ToplevelExpression::Paper(_)));
    assert!(matches!(&ast.items[1], ToplevelExpression::Layout(_)));
    assert!(matches!(&ast.items[2], ToplevelExpression::Midi(_)));
    assert!(matches!(&ast.items[3], ToplevelExpression::Score(_)));

    // Validate
    crate::validator::validate(&ast).unwrap();
}

// ── Numeric expressions ─────────────────────────────────────────────

#[test]
fn parse_number_with_unit() {
    let input = r#"\paper { indent = 180\mm }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(pb.body[0].name, "indent");
            assert_eq!(
                pb.body[0].value,
                AssignmentValue::NumericExpression(NumericExpression::WithUnit(180.0, "mm".into()))
            );
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn parse_number_subtraction_with_units() {
    let input = r#"\paper { line-width = 180\mm - 2\cm }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(pb.body[0].name, "line-width");
            assert_eq!(
                pb.body[0].value,
                AssignmentValue::NumericExpression(NumericExpression::Sub(
                    Box::new(NumericExpression::WithUnit(180.0, "mm".into())),
                    Box::new(NumericExpression::WithUnit(2.0, "cm".into())),
                ))
            );
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn parse_number_addition() {
    let input = r#"\paper { indent = 3 + 4 }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(
                pb.body[0].value,
                AssignmentValue::NumericExpression(NumericExpression::Add(
                    Box::new(NumericExpression::Literal(3.0)),
                    Box::new(NumericExpression::Literal(4.0)),
                ))
            );
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn parse_number_multiplication() {
    let input = r#"\paper { indent = 10 * 2.5 }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(
                pb.body[0].value,
                AssignmentValue::NumericExpression(NumericExpression::Mul(
                    Box::new(NumericExpression::Literal(10.0)),
                    Box::new(NumericExpression::Literal(2.5)),
                ))
            );
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn parse_unary_minus_number() {
    let input = r#"\paper { indent = -5 }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(
                pb.body[0].value,
                AssignmentValue::NumericExpression(NumericExpression::Negate(Box::new(
                    NumericExpression::Literal(5.0)
                )))
            );
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn parse_unary_minus_with_unit() {
    let input = r#"\paper { indent = -5\mm }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(
                pb.body[0].value,
                AssignmentValue::NumericExpression(NumericExpression::Negate(Box::new(
                    NumericExpression::WithUnit(5.0, "mm".into())
                )))
            );
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn parse_all_units() {
    for unit in &["mm", "cm", "pt", "in", "bp", "dd", "cc", "sp"] {
        let input = format!("\\paper {{ indent = 10\\{unit} }}");
        let ast = parse(&input).unwrap();
        match &ast.items[0] {
            ToplevelExpression::Paper(pb) => {
                assert_eq!(
                    pb.body[0].value,
                    AssignmentValue::NumericExpression(NumericExpression::WithUnit(
                        10.0,
                        unit.to_string()
                    ))
                );
            }
            other => panic!("expected paper for unit {unit}, got {other:?}"),
        }
    }
}

#[test]
fn parse_complex_numeric_expression() {
    // 180\mm - 2\cm + 10\pt
    let input = r#"\paper { line-width = 180\mm - 2\cm + 10\pt }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            // Left-associative: (180\mm - 2\cm) + 10\pt
            assert_eq!(
                pb.body[0].value,
                AssignmentValue::NumericExpression(NumericExpression::Add(
                    Box::new(NumericExpression::Sub(
                        Box::new(NumericExpression::WithUnit(180.0, "mm".into())),
                        Box::new(NumericExpression::WithUnit(2.0, "cm".into())),
                    )),
                    Box::new(NumericExpression::WithUnit(10.0, "pt".into())),
                ))
            );
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn parse_numeric_division() {
    let input = r#"\paper { indent = 180\mm / 2 }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(
                pb.body[0].value,
                AssignmentValue::NumericExpression(NumericExpression::Div(
                    Box::new(NumericExpression::WithUnit(180.0, "mm".into())),
                    Box::new(NumericExpression::Literal(2.0)),
                ))
            );
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn plain_number_not_promoted_to_expression() {
    // A plain number without units or operators should stay as Number
    let input = r#"\paper { indent = 0 }"#;
    let ast = parse(input).unwrap();
    match &ast.items[0] {
        ToplevelExpression::Paper(pb) => {
            assert_eq!(pb.body[0].value, AssignmentValue::Number(0.0));
        }
        other => panic!("expected paper, got {other:?}"),
    }
}

#[test]
fn roundtrip_numeric_expression() {
    let input = r#"\paper {
  line-width = 180\mm - 2\cm
  indent = 0\mm
}"#;
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

#[test]
fn roundtrip_unary_minus() {
    let input = r#"\paper {
  indent = -5\mm
}"#;
    let ast = parse(input).unwrap();
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

// ── Fixture test ────────────────────────────────────────────────────

#[test]
fn parse_fixture_numeric_expr() {
    let input = include_str!("../../../../../tests/fixtures/lilypond/fragment_numeric_expr.ly");
    let ast = parse(input).unwrap();
    // Should have: paper, score
    assert!(!ast.items.is_empty());
    assert!(matches!(&ast.items[0], ToplevelExpression::Paper(_)));

    // Validate the AST
    crate::validator::validate(&ast).unwrap();

    // Serialization roundtrip
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap();
    assert_eq!(ast, ast2);
}

//! Tests for markup parsing (Phase 21).

use crate::model::markup::Markup;
use crate::model::*;
use crate::parser::parse;

fn parse_toplevel(input: &str) -> ToplevelExpression {
    let file = parse(input).expect("parse failed");
    file.items.into_iter().next().expect("no items")
}

fn parse_markup_expr(input: &str) -> Markup {
    match parse_toplevel(input) {
        ToplevelExpression::Markup(m) => m,
        other => panic!("expected Markup, got {other:?}"),
    }
}

fn parse_music_fragment(input: &str) -> Music {
    let file = parse(input).expect("parse failed");
    match &file.items[0] {
        ToplevelExpression::Music(m) => m.clone(),
        _ => panic!("expected Music"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Basic markup parsing
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_simple_string() {
    let m = parse_markup_expr("\\markup \"Hello\"");
    assert_eq!(m, Markup::String("Hello".into()));
}

#[test]
fn parse_markup_simple_word() {
    let m = parse_markup_expr("\\markup Hello");
    assert_eq!(m, Markup::Word("Hello".into()));
}

#[test]
fn parse_markup_braced_list() {
    let m = parse_markup_expr("\\markup { Hello World }");
    match &m {
        Markup::List(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], Markup::Word("Hello".into()));
            assert_eq!(items[1], Markup::Word("World".into()));
        }
        other => panic!("expected List, got {other:?}"),
    }
}

#[test]
fn parse_markup_braced_strings() {
    let m = parse_markup_expr("\\markup { \"one\" \"two\" }");
    match &m {
        Markup::List(items) => {
            assert_eq!(items.len(), 2);
            assert_eq!(items[0], Markup::String("one".into()));
            assert_eq!(items[1], Markup::String("two".into()));
        }
        other => panic!("expected List, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Prefix commands
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_bold() {
    let m = parse_markup_expr("\\markup \\bold \"Hello\"");
    match &m {
        Markup::Command { name, args } => {
            assert_eq!(name, "bold");
            assert_eq!(args.len(), 1);
            assert_eq!(args[0], Markup::String("Hello".into()));
        }
        other => panic!("expected Command, got {other:?}"),
    }
}

#[test]
fn parse_markup_chained_commands() {
    let m = parse_markup_expr("\\markup \\bold \\italic \"text\"");
    match &m {
        Markup::Command { name, args } => {
            assert_eq!(name, "bold");
            assert_eq!(args.len(), 1);
            match &args[0] {
                Markup::Command { name, args } => {
                    assert_eq!(name, "italic");
                    assert_eq!(args.len(), 1);
                    assert_eq!(args[0], Markup::String("text".into()));
                }
                other => panic!("expected inner Command, got {other:?}"),
            }
        }
        other => panic!("expected Command, got {other:?}"),
    }
}

#[test]
fn parse_markup_triple_chain() {
    let m = parse_markup_expr("\\markup \\bold \\italic \\larger \"big\"");
    // bold(italic(larger("big")))
    match &m {
        Markup::Command { name, args } => {
            assert_eq!(name, "bold");
            match &args[0] {
                Markup::Command { name, args } => {
                    assert_eq!(name, "italic");
                    match &args[0] {
                        Markup::Command { name, args } => {
                            assert_eq!(name, "larger");
                            assert_eq!(args[0], Markup::String("big".into()));
                        }
                        other => panic!("expected larger, got {other:?}"),
                    }
                }
                other => panic!("expected italic, got {other:?}"),
            }
        }
        other => panic!("expected bold, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// List commands (column, line, etc.)
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_column() {
    let m = parse_markup_expr("\\markup \\column { \"one\" \"two\" \"three\" }");
    match &m {
        Markup::Command { name, args } => {
            assert_eq!(name, "column");
            assert_eq!(args.len(), 3);
            assert_eq!(args[0], Markup::String("one".into()));
            assert_eq!(args[1], Markup::String("two".into()));
            assert_eq!(args[2], Markup::String("three".into()));
        }
        other => panic!("expected Command, got {other:?}"),
    }
}

#[test]
fn parse_markup_center_column() {
    let m = parse_markup_expr("\\markup \\center-column { \"a\" \"b\" }");
    match &m {
        Markup::Command { name, args } => {
            assert_eq!(name, "center-column");
            assert_eq!(args.len(), 2);
        }
        other => panic!("expected Command, got {other:?}"),
    }
}

#[test]
fn parse_markup_line() {
    let m = parse_markup_expr("\\markup \\line { one two three }");
    match &m {
        Markup::Command { name, args } => {
            assert_eq!(name, "line");
            assert_eq!(args.len(), 3);
            assert_eq!(args[0], Markup::Word("one".into()));
        }
        other => panic!("expected Command, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Commands in braced list
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_commands_in_list() {
    let m = parse_markup_expr("\\markup { \\bold \"one\" \"two\" \\italic \"three\" }");
    match &m {
        Markup::List(items) => {
            assert_eq!(items.len(), 3);
            match &items[0] {
                Markup::Command { name, .. } => assert_eq!(name, "bold"),
                other => panic!("expected Command, got {other:?}"),
            }
            assert_eq!(items[1], Markup::String("two".into()));
            match &items[2] {
                Markup::Command { name, .. } => assert_eq!(name, "italic"),
                other => panic!("expected Command, got {other:?}"),
            }
        }
        other => panic!("expected List, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Embedded score
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_score() {
    let m = parse_markup_expr("\\markup \\score { { c4 d e f } }");
    match &m {
        Markup::Score(sb) => {
            assert!(!sb.items.is_empty());
            assert!(matches!(&sb.items[0], ScoreItem::Music(_)));
        }
        other => panic!("expected Score, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Markuplist
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markuplist_braced() {
    let expr = parse_toplevel("\\markuplist { \"one\" \"two\" \"three\" }");
    match expr {
        ToplevelExpression::MarkupList(ml) => {
            assert_eq!(ml.items.len(), 3);
            assert_eq!(ml.items[0], Markup::String("one".into()));
        }
        other => panic!("expected MarkupList, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Markup in assignment
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_in_header() {
    let file = parse("\\header { title = \\markup \"My Title\" }").expect("parse failed");
    match &file.items[0] {
        ToplevelExpression::Header(hb) => {
            assert_eq!(hb.fields.len(), 1);
            assert_eq!(hb.fields[0].name, "title");
            match &hb.fields[0].value {
                AssignmentValue::Markup(m) => {
                    assert_eq!(*m, Markup::String("My Title".into()));
                }
                other => panic!("expected Markup, got {other:?}"),
            }
        }
        other => panic!("expected Header, got {other:?}"),
    }
}

#[test]
fn parse_markup_in_header_braced() {
    let file =
        parse("\\header { title = \\markup { \\bold \"My\" \"Title\" } }").expect("parse failed");
    match &file.items[0] {
        ToplevelExpression::Header(hb) => match &hb.fields[0].value {
            AssignmentValue::Markup(Markup::List(items)) => {
                assert_eq!(items.len(), 2);
            }
            other => panic!("expected Markup List, got {other:?}"),
        },
        other => panic!("expected Header, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Markup in music context
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_in_music() {
    let m = parse_music_fragment("{ c4 \\markup \"text\" d4 }");
    match &m {
        Music::Sequential(items) => {
            assert_eq!(items.len(), 3);
            assert!(matches!(&items[0], Music::Note(_)));
            match &items[1] {
                Music::Markup(Markup::String(s)) => assert_eq!(s, "text"),
                other => panic!("expected Markup, got {other:?}"),
            }
            assert!(matches!(&items[2], Music::Note(_)));
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Nested braced lists
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_nested_braces() {
    let m = parse_markup_expr("\\markup { { \"inner\" } \"outer\" }");
    match &m {
        Markup::List(items) => {
            assert_eq!(items.len(), 2);
            match &items[0] {
                Markup::List(inner) => {
                    assert_eq!(inner.len(), 1);
                    assert_eq!(inner[0], Markup::String("inner".into()));
                }
                other => panic!("expected inner List, got {other:?}"),
            }
            assert_eq!(items[1], Markup::String("outer".into()));
        }
        other => panic!("expected List, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Identifier in markup
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_identifier() {
    let m = parse_markup_expr("\\markup \\myVar");
    assert_eq!(m, Markup::Identifier("myVar".into()));
}

// ──────────────────────────────────────────────────────────────────
// Scheme in markup
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_with_scheme() {
    let m = parse_markup_expr("\\markup \\with-color #red \"text\"");
    match &m {
        Markup::Command { name, args } => {
            assert_eq!(name, "with-color");
            assert_eq!(args.len(), 2);
            match &args[0] {
                Markup::Scheme(s) => {
                    assert_eq!(
                        *s,
                        crate::model::scheme::SchemeExpr::Identifier("red".into())
                    )
                }
                other => panic!("expected Scheme, got {other:?}"),
            }
            assert_eq!(args[1], Markup::String("text".into()));
        }
        other => panic!("expected Command, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Number in markup
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_number() {
    let m = parse_markup_expr("\\markup \\abs-fontsize #16 \"text\"");
    match &m {
        Markup::Command { name, args } => {
            assert_eq!(name, "abs-fontsize");
            assert_eq!(args.len(), 2);
            match &args[0] {
                Markup::Scheme(s) => {
                    assert_eq!(*s, crate::model::scheme::SchemeExpr::Integer(16))
                }
                other => panic!("expected Scheme, got {other:?}"),
            }
        }
        other => panic!("expected Command, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Roundtrip: serialize and reparse
// ──────────────────────────────────────────────────────────────────

fn roundtrip_markup(input: &str) {
    let file = parse(input).expect("parse failed");
    let output = crate::serializer::serialize(&file);
    let file2 =
        parse(&output).unwrap_or_else(|e| panic!("reparse failed: {e}\nSerialized:\n{output}"));
    assert_eq!(
        file, file2,
        "roundtrip mismatch\nInput:  {input}\nOutput: {output}"
    );
}

#[test]
fn roundtrip_markup_string() {
    roundtrip_markup("\\markup \"Hello\"");
}

#[test]
fn roundtrip_markup_word() {
    roundtrip_markup("\\markup Hello");
}

#[test]
fn roundtrip_markup_bold() {
    roundtrip_markup("\\markup \\bold \"Hello\"");
}

#[test]
fn roundtrip_markup_chained() {
    roundtrip_markup("\\markup \\bold \\italic \"text\"");
}

#[test]
fn roundtrip_markup_braced() {
    roundtrip_markup("\\markup { Hello World }");
}

#[test]
fn roundtrip_markup_column() {
    roundtrip_markup("\\markup \\column { \"one\" \"two\" \"three\" }");
}

#[test]
fn roundtrip_markup_center_column() {
    roundtrip_markup("\\markup \\center-column { \"a\" \"b\" }");
}

#[test]
fn roundtrip_markup_score() {
    roundtrip_markup("\\markup \\score { { c4 d e f } }");
}

#[test]
fn roundtrip_markuplist() {
    roundtrip_markup("\\markuplist { \"one\" \"two\" \"three\" }");
}

#[test]
fn roundtrip_markup_nested() {
    roundtrip_markup("\\markup { { \"inner\" } \"outer\" }");
}

#[test]
fn roundtrip_markup_in_header() {
    roundtrip_markup("\\header { title = \\markup \"My Title\" }");
}

#[test]
fn roundtrip_markup_in_header_braced() {
    roundtrip_markup("\\header { title = \\markup { \\bold \"My\" \"Title\" } }");
}

// ──────────────────────────────────────────────────────────────────
// Partial markup (\etc)
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markup_partial_single() {
    let m = parse_markup_expr("\\markup \\bold \\etc");
    match &m {
        Markup::Partial { commands, args } => {
            assert_eq!(commands, &["bold"]);
            assert!(args.is_empty());
        }
        other => panic!("expected Partial, got {other:?}"),
    }
}

#[test]
fn parse_markup_partial_chained() {
    let m = parse_markup_expr("\\markup \\bold \\italic \\etc");
    match &m {
        Markup::Partial { commands, args } => {
            assert_eq!(commands, &["bold", "italic"]);
            assert!(args.is_empty());
        }
        other => panic!("expected Partial, got {other:?}"),
    }
}

#[test]
fn parse_markup_partial_triple() {
    let m = parse_markup_expr("\\markup \\bold \\italic \\larger \\etc");
    match &m {
        Markup::Partial { commands, args } => {
            assert_eq!(commands, &["bold", "italic", "larger"]);
            assert!(args.is_empty());
        }
        other => panic!("expected Partial, got {other:?}"),
    }
}

#[test]
fn roundtrip_markup_partial_single() {
    roundtrip_markup("\\markup \\bold \\etc");
}

#[test]
fn roundtrip_markup_partial_chained() {
    roundtrip_markup("\\markup \\bold \\italic \\etc");
}

// ──────────────────────────────────────────────────────────────────
// Markuplist with list-returning commands
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_markuplist_column_lines() {
    let expr = parse_toplevel("\\markuplist \\column-lines { \"one\" \"two\" }");
    match expr {
        ToplevelExpression::MarkupList(ml) => {
            // The markuplist wraps the list-returning command as a single item
            assert_eq!(ml.items.len(), 1);
            match &ml.items[0] {
                Markup::Command { name, args } => {
                    assert_eq!(name, "column-lines");
                    // Braced list items unpacked as individual args
                    assert_eq!(args.len(), 2);
                }
                other => panic!("expected Command, got {other:?}"),
            }
        }
        other => panic!("expected MarkupList, got {other:?}"),
    }
}

#[test]
fn parse_markuplist_wordwrap_lines() {
    let expr = parse_toplevel("\\markuplist \\wordwrap-lines { hello world }");
    match expr {
        ToplevelExpression::MarkupList(ml) => {
            assert_eq!(ml.items.len(), 1);
            match &ml.items[0] {
                Markup::Command { name, args } => {
                    assert_eq!(name, "wordwrap-lines");
                    assert_eq!(args.len(), 2);
                }
                other => panic!("expected Command, got {other:?}"),
            }
        }
        other => panic!("expected MarkupList, got {other:?}"),
    }
}

#[test]
fn roundtrip_markuplist_column_lines() {
    roundtrip_markup("\\markuplist \\column-lines { \"one\" \"two\" }");
}

// ──────────────────────────────────────────────────────────────────
// Partial markup in assignment
// ──────────────────────────────────────────────────────────────────

#[test]
fn parse_partial_markup_in_assignment() {
    let file = parse("myBold = \\markup \\bold \\etc").expect("parse failed");
    match &file.items[0] {
        ToplevelExpression::Assignment(a) => {
            assert_eq!(a.name, "myBold");
            match &a.value {
                AssignmentValue::Markup(Markup::Partial { commands, args }) => {
                    assert_eq!(commands, &["bold"]);
                    assert!(args.is_empty());
                }
                other => panic!("expected Markup Partial, got {other:?}"),
            }
        }
        other => panic!("expected Assignment, got {other:?}"),
    }
}

// ──────────────────────────────────────────────────────────────────
// Fixture roundtrip
// ──────────────────────────────────────────────────────────────────

#[test]
fn roundtrip_markup_fixture() {
    let input = include_str!("../../../../../tests/fixtures/lilypond/fragment_markup.ly");
    let file = parse(input).expect("parse failed");
    let output = crate::serializer::serialize(&file);
    let file2 =
        parse(&output).unwrap_or_else(|e| panic!("reparse failed: {e}\nSerialized:\n{output}"));
    assert_eq!(
        file, file2,
        "fixture roundtrip mismatch\nSerialized:\n{output}"
    );
}

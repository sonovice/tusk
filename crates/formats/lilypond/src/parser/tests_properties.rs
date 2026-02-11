//! Tests for property operation parsing: override, revert, set, unset, tweak, once.

use crate::model::scheme::SchemeExpr;
use crate::model::*;
use crate::parser::parse;
use crate::serializer::serialize;

/// Helper: parse a LilyPond string, extract the first Music from the AST.
fn parse_first_music(src: &str) -> Music {
    let file = parse(src).unwrap();
    match &file.items[0] {
        ToplevelExpression::Music(m) => m.clone(),
        _ => panic!("expected Music"),
    }
}

/// Helper: parse → serialize → parse roundtrip, compare serialized output.
fn roundtrip(src: &str) -> String {
    let file = parse(src).unwrap();
    serialize(&file)
}

// ── \override ────────────────────────────────────────────────────────

#[test]
fn override_simple() {
    let m = parse_first_music("\\override NoteHead.color = #red");
    match m {
        Music::Override { path, value } => {
            assert_eq!(path.segments, vec!["NoteHead", "color"]);
            assert_eq!(
                value,
                PropertyValue::SchemeExpr(SchemeExpr::Identifier("red".into()))
            );
        }
        _ => panic!("expected Override, got {m:?}"),
    }
}

#[test]
fn override_with_context() {
    let m = parse_first_music("\\override Staff.TimeSignature.color = #green");
    match m {
        Music::Override { path, value } => {
            assert_eq!(path.segments, vec!["Staff", "TimeSignature", "color"]);
            assert_eq!(
                value,
                PropertyValue::SchemeExpr(SchemeExpr::Identifier("green".into()))
            );
        }
        _ => panic!("expected Override, got {m:?}"),
    }
}

#[test]
fn override_number_value() {
    let m = parse_first_music("\\override Beam.gap-count = 5");
    match m {
        Music::Override { path, value } => {
            assert_eq!(path.segments, vec!["Beam", "gap-count"]);
            assert_eq!(value, PropertyValue::Number(5.0));
        }
        _ => panic!("expected Override, got {m:?}"),
    }
}

#[test]
fn override_scheme_compound() {
    let m = parse_first_music("\\override Glissando.color = #(rgb-color 1 0 0)");
    match m {
        Music::Override { path, value } => {
            assert_eq!(path.segments, vec!["Glissando", "color"]);
            match value {
                PropertyValue::SchemeExpr(SchemeExpr::List(s)) => {
                    assert!(s.starts_with("("))
                }
                other => panic!("expected SchemeExpr::List, got {other:?}"),
            }
        }
        _ => panic!("expected Override, got {m:?}"),
    }
}

// ── \revert ──────────────────────────────────────────────────────────

#[test]
fn revert_simple() {
    let m = parse_first_music("\\revert NoteHead.color");
    match m {
        Music::Revert { path } => {
            assert_eq!(path.segments, vec!["NoteHead", "color"]);
        }
        _ => panic!("expected Revert, got {m:?}"),
    }
}

#[test]
fn revert_with_context() {
    let m = parse_first_music("\\revert Staff.BarLine.color");
    match m {
        Music::Revert { path } => {
            assert_eq!(path.segments, vec!["Staff", "BarLine", "color"]);
        }
        _ => panic!("expected Revert, got {m:?}"),
    }
}

// ── \set ─────────────────────────────────────────────────────────────

#[test]
fn set_string_value() {
    let m = parse_first_music("\\set Staff.instrumentName = \"Piano\"");
    match m {
        Music::Set { path, value } => {
            assert_eq!(path.segments, vec!["Staff", "instrumentName"]);
            assert_eq!(value, PropertyValue::String("Piano".into()));
        }
        _ => panic!("expected Set, got {m:?}"),
    }
}

#[test]
fn set_scheme_value() {
    let m = parse_first_music("\\set Staff.useBassFigureExtenders = ##t");
    match m {
        Music::Set { path, value } => {
            assert_eq!(path.segments, vec!["Staff", "useBassFigureExtenders"]);
            assert_eq!(value, PropertyValue::SchemeExpr(SchemeExpr::Bool(true)));
        }
        _ => panic!("expected Set, got {m:?}"),
    }
}

#[test]
fn set_single_prop() {
    // No context prefix — just a bare property
    let m = parse_first_music("\\set stanza = \"verse\"");
    match m {
        Music::Set { path, value } => {
            assert_eq!(path.segments, vec!["stanza"]);
            assert_eq!(value, PropertyValue::String("verse".into()));
        }
        _ => panic!("expected Set, got {m:?}"),
    }
}

// ── \unset ───────────────────────────────────────────────────────────

#[test]
fn unset_simple() {
    let m = parse_first_music("\\unset Staff.keyAlterations");
    match m {
        Music::Unset { path } => {
            assert_eq!(path.segments, vec!["Staff", "keyAlterations"]);
        }
        _ => panic!("expected Unset, got {m:?}"),
    }
}

// ── \once ────────────────────────────────────────────────────────────

#[test]
fn once_override() {
    let m = parse_first_music("\\once \\override NoteHead.color = #red");
    match m {
        Music::Once { music } => match *music {
            Music::Override { path, value } => {
                assert_eq!(path.segments, vec!["NoteHead", "color"]);
                assert_eq!(
                    value,
                    PropertyValue::SchemeExpr(SchemeExpr::Identifier("red".into()))
                );
            }
            _ => panic!("expected Override inside Once"),
        },
        _ => panic!("expected Once, got {m:?}"),
    }
}

// ── \tweak (post-event) ─────────────────────────────────────────────

#[test]
fn tweak_post_event() {
    let file = parse("{ c4\\tweak color #red -. }").unwrap();
    let seq = match &file.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => items,
        other => panic!("expected Sequential, got {other:?}"),
    };
    let note = match &seq[0] {
        Music::Note(n) => n,
        other => panic!("expected Note, got {other:?}"),
    };
    // Find the tweak in post-events
    let tweaks: Vec<_> = note
        .post_events
        .iter()
        .filter(|e| matches!(e, note::PostEvent::Tweak { .. }))
        .collect();
    assert_eq!(tweaks.len(), 1);
    match &tweaks[0] {
        note::PostEvent::Tweak { path, value } => {
            assert_eq!(path.segments, vec!["color"]);
            assert_eq!(
                *value,
                PropertyValue::SchemeExpr(SchemeExpr::Identifier("red".into()))
            );
        }
        _ => unreachable!(),
    }
}

// ── Context mod items ────────────────────────────────────────────────

#[test]
fn context_mod_override() {
    let file = parse(
        r#"\new Staff \with {
  \override TimeSignature.color = #green
} { c4 }"#,
    )
    .unwrap();
    let m = match &file.items[0] {
        ToplevelExpression::Music(m) => m,
        _ => panic!("expected Music"),
    };
    match m {
        Music::ContextedMusic { with_block, .. } => {
            let items = with_block.as_ref().unwrap();
            match &items[0] {
                ContextModItem::Override { path, value } => {
                    assert_eq!(path.segments, vec!["TimeSignature", "color"]);
                    assert_eq!(
                        *value,
                        PropertyValue::SchemeExpr(SchemeExpr::Identifier("green".into()))
                    );
                }
                other => panic!("expected Override, got {other:?}"),
            }
        }
        _ => panic!("expected ContextedMusic"),
    }
}

#[test]
fn context_mod_set_unset() {
    let file = parse(
        r#"\new Staff \with {
  \set instrumentName = "Piano"
  \unset shortInstrumentName
} { c4 }"#,
    )
    .unwrap();
    let m = match &file.items[0] {
        ToplevelExpression::Music(m) => m,
        _ => panic!("expected Music"),
    };
    match m {
        Music::ContextedMusic { with_block, .. } => {
            let items = with_block.as_ref().unwrap();
            assert_eq!(items.len(), 2);
            match &items[0] {
                ContextModItem::Set { path, value } => {
                    assert_eq!(path.segments, vec!["instrumentName"]);
                    assert_eq!(*value, PropertyValue::String("Piano".into()));
                }
                other => panic!("expected Set, got {other:?}"),
            }
            match &items[1] {
                ContextModItem::Unset { path } => {
                    assert_eq!(path.segments, vec!["shortInstrumentName"]);
                }
                other => panic!("expected Unset, got {other:?}"),
            }
        }
        _ => panic!("expected ContextedMusic"),
    }
}

// ── Serialization roundtrip ──────────────────────────────────────────

#[test]
fn roundtrip_override() {
    let src = "\\override NoteHead.color = #red\n";
    let out = roundtrip(src);
    assert!(out.contains("\\override NoteHead.color = #red"));
}

#[test]
fn roundtrip_set() {
    let src = "\\set Staff.instrumentName = \"Piano\"\n";
    let out = roundtrip(src);
    assert!(out.contains("\\set Staff.instrumentName = \"Piano\""));
}

#[test]
fn roundtrip_revert() {
    let src = "\\revert NoteHead.color\n";
    let out = roundtrip(src);
    assert!(out.contains("\\revert NoteHead.color"));
}

#[test]
fn roundtrip_unset() {
    let src = "\\unset Staff.keyAlterations\n";
    let out = roundtrip(src);
    assert!(out.contains("\\unset Staff.keyAlterations"));
}

#[test]
fn roundtrip_once_override() {
    let src = "\\once \\override NoteHead.color = #red\n";
    let out = roundtrip(src);
    assert!(out.contains("\\once \\override NoteHead.color = #red"));
}

#[test]
fn roundtrip_tweak_in_music() {
    let src = "{ c4\\tweak color #red -. }\n";
    let out = roundtrip(src);
    assert!(out.contains("\\tweak color #red"));
}

#[test]
fn roundtrip_in_sequence() {
    let src = "{ \\override NoteHead.color = #red c4 d4 \\revert NoteHead.color e4 }\n";
    let out = roundtrip(src);
    assert!(out.contains("\\override NoteHead.color = #red"));
    assert!(out.contains("\\revert NoteHead.color"));
}

// ── Context def mod keywords ─────────────────────────────────────────

#[test]
fn context_def_denies() {
    let file = parse(
        r#"\layout {
  \context {
    \Staff
    \denies "Voice"
  }
}"#,
    )
    .unwrap();
    let layout = match &file.items[0] {
        ToplevelExpression::Layout(lb) => lb,
        other => panic!("expected Layout, got {other:?}"),
    };
    let cb = match &layout.body[0] {
        crate::model::LayoutItem::ContextBlock(cb) => cb,
        other => panic!("expected ContextBlock, got {other:?}"),
    };
    assert!(matches!(&cb.items[0], ContextModItem::ContextRef(n) if n == "Staff"));
    match &cb.items[1] {
        ContextModItem::Denies(name) => assert_eq!(name, "Voice"),
        other => panic!("expected Denies, got {other:?}"),
    }
}

#[test]
fn context_def_accepts() {
    let file = parse(
        r#"\layout {
  \context {
    \Staff
    \accepts "CueVoice"
  }
}"#,
    )
    .unwrap();
    let layout = match &file.items[0] {
        ToplevelExpression::Layout(lb) => lb,
        other => panic!("expected Layout, got {other:?}"),
    };
    let cb = match &layout.body[0] {
        crate::model::LayoutItem::ContextBlock(cb) => cb,
        other => panic!("expected ContextBlock, got {other:?}"),
    };
    match &cb.items[1] {
        ContextModItem::Accepts(name) => assert_eq!(name, "CueVoice"),
        other => panic!("expected Accepts, got {other:?}"),
    }
}

#[test]
fn context_def_alias() {
    let file = parse(
        r#"\layout {
  \context {
    \Staff
    \alias "Staff"
  }
}"#,
    )
    .unwrap();
    let layout = match &file.items[0] {
        ToplevelExpression::Layout(lb) => lb,
        other => panic!("expected Layout, got {other:?}"),
    };
    let cb = match &layout.body[0] {
        crate::model::LayoutItem::ContextBlock(cb) => cb,
        other => panic!("expected ContextBlock, got {other:?}"),
    };
    match &cb.items[1] {
        ContextModItem::Alias(name) => assert_eq!(name, "Staff"),
        other => panic!("expected Alias, got {other:?}"),
    }
}

#[test]
fn context_def_defaultchild() {
    let file = parse(
        r#"\layout {
  \context {
    \Score
    \defaultchild "Staff"
  }
}"#,
    )
    .unwrap();
    let layout = match &file.items[0] {
        ToplevelExpression::Layout(lb) => lb,
        other => panic!("expected Layout, got {other:?}"),
    };
    let cb = match &layout.body[0] {
        crate::model::LayoutItem::ContextBlock(cb) => cb,
        other => panic!("expected ContextBlock, got {other:?}"),
    };
    match &cb.items[1] {
        ContextModItem::DefaultChild(name) => assert_eq!(name, "Staff"),
        other => panic!("expected DefaultChild, got {other:?}"),
    }
}

#[test]
fn context_def_description() {
    let file = parse(
        r#"\layout {
  \context {
    \Staff
    \description "A custom staff"
  }
}"#,
    )
    .unwrap();
    let layout = match &file.items[0] {
        ToplevelExpression::Layout(lb) => lb,
        other => panic!("expected Layout, got {other:?}"),
    };
    let cb = match &layout.body[0] {
        crate::model::LayoutItem::ContextBlock(cb) => cb,
        other => panic!("expected ContextBlock, got {other:?}"),
    };
    match &cb.items[1] {
        ContextModItem::Description(text) => assert_eq!(text, "A custom staff"),
        other => panic!("expected Description, got {other:?}"),
    }
}

#[test]
fn context_def_name() {
    let file = parse(
        r#"\layout {
  \context {
    \Staff
    \name "CustomStaff"
  }
}"#,
    )
    .unwrap();
    let layout = match &file.items[0] {
        ToplevelExpression::Layout(lb) => lb,
        other => panic!("expected Layout, got {other:?}"),
    };
    let cb = match &layout.body[0] {
        crate::model::LayoutItem::ContextBlock(cb) => cb,
        other => panic!("expected ContextBlock, got {other:?}"),
    };
    match &cb.items[1] {
        ContextModItem::Name(name) => assert_eq!(name, "CustomStaff"),
        other => panic!("expected Name, got {other:?}"),
    }
}

#[test]
fn context_def_combined() {
    let file = parse(
        r#"\layout {
  \context {
    \Staff
    \accepts "CueVoice"
    \denies "Voice"
    \alias "Staff"
    \defaultchild "Voice"
    \description "Custom context"
    \name "MyStaff"
    \remove "Bar_number_engraver"
    \consists "Span_arpeggio_engraver"
  }
}"#,
    )
    .unwrap();
    let layout = match &file.items[0] {
        ToplevelExpression::Layout(lb) => lb,
        other => panic!("expected Layout, got {other:?}"),
    };
    let cb = match &layout.body[0] {
        crate::model::LayoutItem::ContextBlock(cb) => cb,
        other => panic!("expected ContextBlock, got {other:?}"),
    };
    assert_eq!(cb.items.len(), 9);
    assert!(matches!(&cb.items[0], ContextModItem::ContextRef(n) if n == "Staff"));
    assert!(matches!(&cb.items[1], ContextModItem::Accepts(n) if n == "CueVoice"));
    assert!(matches!(&cb.items[2], ContextModItem::Denies(n) if n == "Voice"));
    assert!(matches!(&cb.items[3], ContextModItem::Alias(n) if n == "Staff"));
    assert!(matches!(&cb.items[4], ContextModItem::DefaultChild(n) if n == "Voice"));
    assert!(matches!(&cb.items[5], ContextModItem::Description(n) if n == "Custom context"));
    assert!(matches!(&cb.items[6], ContextModItem::Name(n) if n == "MyStaff"));
    assert!(matches!(&cb.items[7], ContextModItem::Remove(n) if n == "Bar_number_engraver"));
    assert!(matches!(&cb.items[8], ContextModItem::Consists(n) if n == "Span_arpeggio_engraver"));
}

#[test]
fn roundtrip_context_def_keywords() {
    let src = r#"\layout {
  \context {
    \Staff
    \accepts "CueVoice"
    \denies "Voice"
    \alias "Staff"
    \defaultchild "Voice"
    \description "Custom context"
    \name "MyStaff"
  }
}
"#;
    let out = roundtrip(src);
    assert!(out.contains(r#"\accepts "CueVoice""#));
    assert!(out.contains(r#"\denies "Voice""#));
    assert!(out.contains(r#"\alias "Staff""#));
    assert!(out.contains(r#"\defaultchild "Voice""#));
    assert!(out.contains(r#"\description "Custom context""#));
    assert!(out.contains(r#"\name "MyStaff""#));
}

// ── Validation ───────────────────────────────────────────────────────

#[test]
fn validates_normal_property_ops() {
    let file = parse("{ \\override NoteHead.color = #red \\set Staff.instrumentName = \"Piano\" }")
        .unwrap();
    let result = crate::validator::validate(&file);
    assert!(result.is_ok());
}

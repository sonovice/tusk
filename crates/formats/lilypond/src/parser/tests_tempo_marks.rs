//! Parser tests for tempo, marks, and text marks (Phase 22).

use crate::model::*;
use crate::parser::parse;

// ── Helpers ──────────────────────────────────────────────────────────────

fn roundtrip(input: &str) {
    let ast = parse(input).unwrap_or_else(|e| panic!("parse: {e}"));
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap_or_else(|e| panic!("re-parse: {e}"));
    assert_eq!(
        ast, ast2,
        "roundtrip mismatch:\ninput:  {input}\noutput: {output}"
    );
}

fn roundtrip_fixture(name: &str) {
    let path = format!(
        "{}/tests/fixtures/lilypond/{}",
        env!("CARGO_MANIFEST_DIR").replace("/crates/formats/lilypond", ""),
        name
    );
    let input = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let ast = parse(&input).unwrap_or_else(|e| panic!("parse {name}: {e}"));
    let output = crate::serializer::serialize(&ast);
    let ast2 = parse(&output).unwrap_or_else(|e| panic!("re-parse {name}: {e}"));
    assert_eq!(ast, ast2, "roundtrip mismatch for {name}");
}

fn parse_first_music(input: &str) -> Music {
    let ast = parse(input).unwrap_or_else(|e| panic!("parse: {e}"));
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => items[0].clone(),
        ToplevelExpression::Music(m) => m.clone(),
        other => panic!("expected music, got {other:?}"),
    }
}

// ── Tempo ────────────────────────────────────────────────────────────────

#[test]
fn parse_tempo_text_and_metronome() {
    let m = parse_first_music("{ \\tempo \"Allegro\" 4 = 120 }");
    match m {
        Music::Tempo(t) => {
            assert_eq!(t.text, Some(Markup::Word("Allegro".into())));
            assert_eq!(t.duration.as_ref().unwrap().base, 4);
            assert_eq!(t.bpm, Some(TempoRange::Single(120)));
        }
        _ => panic!("expected Tempo, got {m:?}"),
    }
}

#[test]
fn parse_tempo_metronome_only() {
    let m = parse_first_music("{ \\tempo 2 = 60 }");
    match m {
        Music::Tempo(t) => {
            assert!(t.text.is_none());
            assert_eq!(t.duration.as_ref().unwrap().base, 2);
            assert_eq!(t.bpm, Some(TempoRange::Single(60)));
        }
        _ => panic!("expected Tempo, got {m:?}"),
    }
}

#[test]
fn parse_tempo_text_only() {
    let m = parse_first_music("{ \\tempo \"Andante\" }");
    match m {
        Music::Tempo(t) => {
            assert_eq!(t.text, Some(Markup::Word("Andante".into())));
            assert!(t.duration.is_none());
            assert!(t.bpm.is_none());
        }
        _ => panic!("expected Tempo, got {m:?}"),
    }
}

#[test]
fn parse_tempo_with_range() {
    let m = parse_first_music("{ \\tempo \"Vivace\" 4. = 132-144 }");
    match m {
        Music::Tempo(t) => {
            assert_eq!(t.text, Some(Markup::Word("Vivace".into())));
            let dur = t.duration.as_ref().unwrap();
            assert_eq!(dur.base, 4);
            assert_eq!(dur.dots, 1);
            assert_eq!(t.bpm, Some(TempoRange::Range(132, 144)));
        }
        _ => panic!("expected Tempo, got {m:?}"),
    }
}

#[test]
fn parse_tempo_dotted_duration() {
    let m = parse_first_music("{ \\tempo 4. = 100 }");
    match m {
        Music::Tempo(t) => {
            assert!(t.text.is_none());
            let dur = t.duration.as_ref().unwrap();
            assert_eq!(dur.base, 4);
            assert_eq!(dur.dots, 1);
            assert_eq!(t.bpm, Some(TempoRange::Single(100)));
        }
        _ => panic!("expected Tempo, got {m:?}"),
    }
}

#[test]
fn parse_tempo_with_markup() {
    let m = parse_first_music("{ \\tempo \\markup { \\bold Allegro } 4 = 120 }");
    match m {
        Music::Tempo(t) => {
            assert!(t.text.is_some());
            assert!(matches!(t.text.as_ref().unwrap(), Markup::List(_)));
            assert_eq!(t.duration.as_ref().unwrap().base, 4);
            assert_eq!(t.bpm, Some(TempoRange::Single(120)));
        }
        _ => panic!("expected Tempo, got {m:?}"),
    }
}

#[test]
fn parse_tempo_markup_text_only() {
    let m = parse_first_music("{ \\tempo \\markup { Slow } }");
    match m {
        Music::Tempo(t) => {
            assert!(t.text.is_some());
            assert!(t.duration.is_none());
            assert!(t.bpm.is_none());
        }
        _ => panic!("expected Tempo, got {m:?}"),
    }
}

// ── Mark ─────────────────────────────────────────────────────────────────

#[test]
fn parse_mark_default() {
    let m = parse_first_music("{ \\mark \\default }");
    match m {
        Music::Mark(mk) => {
            assert_eq!(mk.label, MarkLabel::Default);
        }
        _ => panic!("expected Mark, got {m:?}"),
    }
}

#[test]
fn parse_mark_string() {
    let m = parse_first_music("{ \\mark \"A\" }");
    match m {
        Music::Mark(mk) => {
            assert_eq!(mk.label, MarkLabel::Markup(Markup::Word("A".into())));
        }
        _ => panic!("expected Mark, got {m:?}"),
    }
}

#[test]
fn parse_mark_number() {
    let m = parse_first_music("{ \\mark 5 }");
    match m {
        Music::Mark(mk) => {
            assert_eq!(mk.label, MarkLabel::Number(5));
        }
        _ => panic!("expected Mark, got {m:?}"),
    }
}

#[test]
fn parse_mark_markup() {
    let m = parse_first_music("{ \\mark \\markup { \\bold A } }");
    match m {
        Music::Mark(mk) => {
            assert!(matches!(mk.label, MarkLabel::Markup(_)));
        }
        _ => panic!("expected Mark, got {m:?}"),
    }
}

// ── TextMark ─────────────────────────────────────────────────────────────

#[test]
fn parse_text_mark_string() {
    let m = parse_first_music("{ \\textMark \"Fine\" }");
    match m {
        Music::TextMark(tm) => {
            assert_eq!(tm.text, Markup::Word("Fine".into()));
        }
        _ => panic!("expected TextMark, got {m:?}"),
    }
}

#[test]
fn parse_text_mark_markup() {
    let m = parse_first_music("{ \\textMark \\markup { \\italic \"D.C.\" } }");
    match m {
        Music::TextMark(tm) => {
            assert!(matches!(tm.text, Markup::List(_)));
        }
        _ => panic!("expected TextMark, got {m:?}"),
    }
}

// ── Roundtrip ────────────────────────────────────────────────────────────

#[test]
fn roundtrip_tempo_text_and_metronome() {
    roundtrip("{ \\tempo \"Allegro\" 4 = 120 }");
}

#[test]
fn roundtrip_tempo_metronome_only() {
    roundtrip("{ \\tempo 2 = 60 }");
}

#[test]
fn roundtrip_tempo_text_only() {
    roundtrip("{ \\tempo \"Andante\" }");
}

#[test]
fn roundtrip_tempo_range() {
    roundtrip("{ \\tempo \"Vivace\" 4. = 132-144 }");
}

#[test]
fn roundtrip_mark_default() {
    roundtrip("{ \\mark \\default }");
}

#[test]
fn roundtrip_mark_string() {
    roundtrip("{ \\mark \"A\" }");
}

#[test]
fn roundtrip_mark_number() {
    roundtrip("{ \\mark 5 }");
}

#[test]
fn roundtrip_text_mark() {
    roundtrip("{ \\textMark \"Fine\" }");
}

#[test]
fn roundtrip_combined_tempo_and_marks() {
    roundtrip("{ \\tempo \"Allegro\" 4 = 120 c4 d e f \\mark \\default g4 a b c }");
}

#[test]
fn roundtrip_tempo_marks_fixture() {
    roundtrip_fixture("fragment_tempo_marks.ly");
}

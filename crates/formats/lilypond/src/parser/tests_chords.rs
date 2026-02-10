//! Tests for chord mode parsing.

use crate::model::*;
use crate::parser::parse;

#[test]
fn test_chordmode_bare_root() {
    let src = r#"\chordmode { c1 }"#;
    let file = parse(src).unwrap();
    assert_eq!(file.items.len(), 1);
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 1);
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    assert_eq!(ce.root.step, 'c');
    assert_eq!(ce.duration.as_ref().unwrap().base, 1);
    assert!(ce.quality.is_empty());
    assert!(ce.removals.is_empty());
    assert!(ce.inversion.is_none());
    assert!(ce.bass.is_none());
}

#[test]
fn test_chordmode_minor() {
    let src = r#"\chordmode { c:m }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    assert_eq!(ce.quality.len(), 1);
    assert_eq!(
        ce.quality[0],
        ChordQualityItem::Modifier(ChordModifier::Minor)
    );
}

#[test]
fn test_chordmode_seventh() {
    let src = r#"\chordmode { c:7 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    assert_eq!(ce.quality.len(), 1);
    assert_eq!(
        ce.quality[0],
        ChordQualityItem::Step(ChordStep {
            number: 7,
            alteration: StepAlteration::Natural,
        })
    );
}

#[test]
fn test_chordmode_dim7_inversion() {
    let src = r#"\chordmode { c:dim7/f }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    assert_eq!(ce.quality.len(), 2);
    assert_eq!(
        ce.quality[0],
        ChordQualityItem::Modifier(ChordModifier::Diminished)
    );
    assert_eq!(
        ce.quality[1],
        ChordQualityItem::Step(ChordStep {
            number: 7,
            alteration: StepAlteration::Natural,
        })
    );
    assert!(ce.inversion.is_some());
    assert_eq!(ce.inversion.as_ref().unwrap().step, 'f');
}

#[test]
fn test_chordmode_bass() {
    let src = r#"\chordmode { c:7/+g }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    assert_eq!(ce.quality.len(), 1);
    assert!(ce.bass.is_some());
    assert_eq!(ce.bass.as_ref().unwrap().step, 'g');
}

#[test]
fn test_chordmode_removal() {
    let src = r#"\chordmode { c:7^5 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    assert_eq!(ce.quality.len(), 1);
    assert_eq!(ce.removals.len(), 1);
    assert_eq!(ce.removals[0].number, 5);
}

#[test]
fn test_chordmode_step_alterations() {
    let src = r#"\chordmode { c:9+.5- }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    assert_eq!(ce.quality.len(), 2);
    assert_eq!(
        ce.quality[0],
        ChordQualityItem::Step(ChordStep {
            number: 9,
            alteration: StepAlteration::Sharp,
        })
    );
    assert_eq!(
        ce.quality[1],
        ChordQualityItem::Step(ChordStep {
            number: 5,
            alteration: StepAlteration::Flat,
        })
    );
}

#[test]
fn test_chordmode_complex() {
    // c:maj7.9^3/e => quality=[Major, Step(7), Step(9)], removals=[Step(3)], inversion=e
    let src = r#"\chordmode { c:maj7.9^3/e }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    assert_eq!(ce.quality.len(), 3);
    assert_eq!(
        ce.quality[0],
        ChordQualityItem::Modifier(ChordModifier::Major)
    );
    assert_eq!(
        ce.quality[1],
        ChordQualityItem::Step(ChordStep {
            number: 7,
            alteration: StepAlteration::Natural,
        })
    );
    assert_eq!(
        ce.quality[2],
        ChordQualityItem::Step(ChordStep {
            number: 9,
            alteration: StepAlteration::Natural,
        })
    );
    assert_eq!(ce.removals.len(), 1);
    assert_eq!(ce.removals[0].number, 3);
    assert!(ce.inversion.is_some());
    assert_eq!(ce.inversion.as_ref().unwrap().step, 'e');
}

#[test]
fn test_chordmode_complex_quality() {
    // Re-check: `maj7.9` has `maj` (modifier), then `7` (step), then `.9` (step)
    let src = r#"\chordmode { c:maj7.9 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::ChordModeEntry(ce) = &items[0] else {
        panic!("expected ChordModeEntry");
    };
    // Quality parsing: after `:`, first item is `maj` (modifier), then immediately `7` (step),
    // then `.` separator, then `9` (step).
    // But `maj` is a symbol and `7` follows as Unsigned — our parser parses modifier first,
    // then checks for next items. After `maj`, next token is `7` (Unsigned) — that should be
    // parsed as a step. Then `.` then `9`.
    assert_eq!(ce.quality.len(), 3);
    assert_eq!(
        ce.quality[0],
        ChordQualityItem::Modifier(ChordModifier::Major)
    );
    assert_eq!(
        ce.quality[1],
        ChordQualityItem::Step(ChordStep {
            number: 7,
            alteration: StepAlteration::Natural,
        })
    );
    assert_eq!(
        ce.quality[2],
        ChordQualityItem::Step(ChordStep {
            number: 9,
            alteration: StepAlteration::Natural,
        })
    );
}

#[test]
fn test_chords_shorthand() {
    let src = r#"\chords { c1 d:m }"#;
    let file = parse(src).unwrap();
    assert_eq!(file.items.len(), 1);
    let ToplevelExpression::Music(Music::ContextedMusic {
        keyword,
        context_type,
        music,
        ..
    }) = &file.items[0]
    else {
        panic!("expected ContextedMusic");
    };
    assert_eq!(*keyword, ContextKeyword::New);
    assert_eq!(context_type, "ChordNames");
    let Music::ChordMode { body } = music.as_ref() else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 2);
}

#[test]
fn test_chordmode_with_rests_and_barchecks() {
    let src = r#"\chordmode { c1 | r1 | s1 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::ChordMode { body }) = &file.items[0] else {
        panic!("expected ChordMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 5); // chord, barcheck, rest, barcheck, skip
    assert!(matches!(items[0], Music::ChordModeEntry(_)));
    assert!(matches!(items[1], Music::BarCheck));
    assert!(matches!(items[2], Music::Rest(_)));
    assert!(matches!(items[3], Music::BarCheck));
    assert!(matches!(items[4], Music::Skip(_)));
}

#[test]
fn test_chordmode_roundtrip_serialization() {
    let src = r#"\chordmode { c1 c:m c:7 c:dim7/f }"#;
    let file = parse(src).unwrap();
    let output = crate::serializer::serialize(&file);
    // Parse again and compare
    let file2 = parse(&output).unwrap();
    assert_eq!(file.items.len(), file2.items.len());
}

#[test]
fn test_chordmode_fixture_parse() {
    let src = std::fs::read_to_string("tests/fixtures/lilypond/fragment_chordmode.ly")
        .or_else(|_| {
            std::fs::read_to_string("../../../tests/fixtures/lilypond/fragment_chordmode.ly")
        })
        .expect("fixture file should exist");
    let file = parse(&src).unwrap();
    assert!(file.version.is_some());
    assert_eq!(file.items.len(), 1);
}

#[test]
fn test_chordmode_validation() {
    let src = r#"\chordmode { c:7 }"#;
    let file = parse(src).unwrap();
    crate::validator::validate(&file).unwrap();
}

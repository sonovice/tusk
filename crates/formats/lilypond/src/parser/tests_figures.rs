//! Tests for figured bass mode parsing.

use crate::model::*;
use crate::parser::parse;

#[test]
fn test_figuremode_basic() {
    let src = r#"\figuremode { \<6 4\>4 \<7 5\>4 }"#;
    let file = parse(src).unwrap();
    assert_eq!(file.items.len(), 1);
    let ToplevelExpression::Music(Music::FigureMode { body }) = &file.items[0] else {
        panic!("expected FigureMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 2);

    // First figure: <6 4>4
    let Music::Figure(fe) = &items[0] else {
        panic!("expected Figure");
    };
    assert_eq!(fe.figures.len(), 2);
    assert_eq!(fe.figures[0].number, Some(6));
    assert_eq!(fe.figures[1].number, Some(4));
    assert_eq!(fe.duration.as_ref().unwrap().base, 4);

    // Second figure: <7 5>4
    let Music::Figure(fe2) = &items[1] else {
        panic!("expected Figure");
    };
    assert_eq!(fe2.figures.len(), 2);
    assert_eq!(fe2.figures[0].number, Some(7));
    assert_eq!(fe2.figures[1].number, Some(5));
}

#[test]
fn test_figures_shorthand() {
    let src = r#"\figures { \<3\>2 }"#;
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
    assert_eq!(context_type, "FiguredBass");
    let Music::FigureMode { body } = music.as_ref() else {
        panic!("expected FigureMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 1);
    let Music::Figure(fe) = &items[0] else {
        panic!("expected Figure");
    };
    assert_eq!(fe.figures.len(), 1);
    assert_eq!(fe.figures[0].number, Some(3));
    assert_eq!(fe.duration.as_ref().unwrap().base, 2);
}

#[test]
fn test_figure_space() {
    let src = r#"\figuremode { \<6 _\>4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::FigureMode { body }) = &file.items[0] else {
        panic!("expected FigureMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::Figure(fe) = &items[0] else {
        panic!("expected Figure");
    };
    assert_eq!(fe.figures.len(), 2);
    assert_eq!(fe.figures[0].number, Some(6));
    assert_eq!(fe.figures[1].number, None); // space
}

#[test]
fn test_figure_alterations() {
    let src = r#"\figuremode { \<6+ 4- 5!\>4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::FigureMode { body }) = &file.items[0] else {
        panic!("expected FigureMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::Figure(fe) = &items[0] else {
        panic!("expected Figure");
    };
    assert_eq!(fe.figures.len(), 3);
    assert_eq!(fe.figures[0].alteration, FigureAlteration::Sharp);
    assert_eq!(fe.figures[1].alteration, FigureAlteration::Flat);
    assert_eq!(fe.figures[2].alteration, FigureAlteration::ForcedNatural);
}

#[test]
fn test_figure_modifications() {
    let src = r#"\figuremode { \<5\+ 3/ 7\!\>4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::FigureMode { body }) = &file.items[0] else {
        panic!("expected FigureMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::Figure(fe) = &items[0] else {
        panic!("expected Figure");
    };
    assert_eq!(fe.figures.len(), 3);
    assert_eq!(
        fe.figures[0].modifications,
        vec![FiguredBassModification::Augmented]
    );
    assert_eq!(
        fe.figures[1].modifications,
        vec![FiguredBassModification::Diminished]
    );
    assert_eq!(
        fe.figures[2].modifications,
        vec![FiguredBassModification::NoContinuation]
    );
}

#[test]
fn test_figure_brackets() {
    let src = r#"\figuremode { \<[6 4]\>4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::FigureMode { body }) = &file.items[0] else {
        panic!("expected FigureMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::Figure(fe) = &items[0] else {
        panic!("expected Figure");
    };
    assert_eq!(fe.figures.len(), 2);
    assert!(fe.figures[0].bracket_start);
    assert!(!fe.figures[0].bracket_stop);
    assert!(!fe.figures[1].bracket_start);
    assert!(fe.figures[1].bracket_stop);
}

#[test]
fn test_figure_with_rest_and_skip() {
    let src = r#"\figuremode { \<6\>4 r4 s4 \<3\>4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::FigureMode { body }) = &file.items[0] else {
        panic!("expected FigureMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 4);
    assert!(matches!(&items[0], Music::Figure(_)));
    assert!(matches!(&items[1], Music::Rest(_)));
    assert!(matches!(&items[2], Music::Skip(_)));
    assert!(matches!(&items[3], Music::Figure(_)));
}

#[test]
fn test_figure_bar_check() {
    let src = r#"\figuremode { \<6\>4 | \<3\>4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::FigureMode { body }) = &file.items[0] else {
        panic!("expected FigureMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 3);
    assert!(matches!(&items[1], Music::BarCheck));
}

#[test]
fn test_figure_roundtrip_serialization() {
    let src = r#"\figuremode { \<6 4\>4 \<7 5\>4 }"#;
    let file = parse(src).unwrap();
    let output = crate::serializer::serialize(&file);
    // Re-parse the serialized output
    let file2 = parse(&output).unwrap();
    let output2 = crate::serializer::serialize(&file2);
    assert_eq!(output, output2, "roundtrip serialization mismatch");
}

#[test]
fn test_figure_roundtrip_modifications() {
    let src = r#"\figuremode { \<5\+ 3/\>4 \<7\! _\>2 }"#;
    let file = parse(src).unwrap();
    let output = crate::serializer::serialize(&file);
    let file2 = parse(&output).unwrap();
    let output2 = crate::serializer::serialize(&file2);
    assert_eq!(output, output2, "roundtrip serialization mismatch");
}

#[test]
fn test_figure_roundtrip_brackets() {
    let src = r#"\figuremode { \<[6 4]\>4 }"#;
    let file = parse(src).unwrap();
    let output = crate::serializer::serialize(&file);
    let file2 = parse(&output).unwrap();
    let output2 = crate::serializer::serialize(&file2);
    assert_eq!(output, output2, "roundtrip serialization mismatch");
}

#[test]
fn test_figure_roundtrip_alterations() {
    let src = r#"\figuremode { \<6+ 4- 5!\>4 }"#;
    let file = parse(src).unwrap();
    let output = crate::serializer::serialize(&file);
    let file2 = parse(&output).unwrap();
    let output2 = crate::serializer::serialize(&file2);
    assert_eq!(output, output2, "roundtrip serialization mismatch");
}

#[test]
fn test_figures_shorthand_roundtrip() {
    let src = r#"\figures { \<6 4\>4 \<7 5\>2 }"#;
    let file = parse(src).unwrap();
    let output = crate::serializer::serialize(&file);
    let file2 = parse(&output).unwrap();
    let output2 = crate::serializer::serialize(&file2);
    assert_eq!(output, output2, "roundtrip serialization mismatch");
}

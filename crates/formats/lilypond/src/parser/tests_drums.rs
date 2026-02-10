//! Tests for drum mode parsing.

use crate::model::*;
use crate::parser::parse;

#[test]
fn test_drummode_basic() {
    let src = r#"\drummode { bd4 sn4 hh4 }"#;
    let file = parse(src).unwrap();
    assert_eq!(file.items.len(), 1);
    let ToplevelExpression::Music(Music::DrumMode { body }) = &file.items[0] else {
        panic!("expected DrumMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 3);
    let Music::DrumNote(dn) = &items[0] else {
        panic!("expected DrumNote");
    };
    assert_eq!(dn.drum_type, "bd");
    assert_eq!(dn.duration.as_ref().unwrap().base, 4);
}

#[test]
fn test_drummode_long_names() {
    let src = r#"\drummode { bassdrum4 snare8 hihat16 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::DrumMode { body }) = &file.items[0] else {
        panic!("expected DrumMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 3);
    let Music::DrumNote(dn0) = &items[0] else {
        panic!("expected DrumNote");
    };
    assert_eq!(dn0.drum_type, "bassdrum");
    let Music::DrumNote(dn1) = &items[1] else {
        panic!("expected DrumNote");
    };
    assert_eq!(dn1.drum_type, "snare");
    let Music::DrumNote(dn2) = &items[2] else {
        panic!("expected DrumNote");
    };
    assert_eq!(dn2.drum_type, "hihat");
}

#[test]
fn test_drummode_with_rests_and_barchecks() {
    let src = r#"\drummode { bd4 sn4 | r4 hh4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::DrumMode { body }) = &file.items[0] else {
        panic!("expected DrumMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 5); // bd, sn, barcheck, r, hh
    assert!(matches!(items[0], Music::DrumNote(_)));
    assert!(matches!(items[1], Music::DrumNote(_)));
    assert!(matches!(items[2], Music::BarCheck));
    assert!(matches!(items[3], Music::Rest(_)));
    assert!(matches!(items[4], Music::DrumNote(_)));
}

#[test]
fn test_drummode_drum_chord() {
    let src = r#"\drummode { <bd sn>4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::DrumMode { body }) = &file.items[0] else {
        panic!("expected DrumMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 1);
    let Music::DrumChord(dc) = &items[0] else {
        panic!("expected DrumChord");
    };
    assert_eq!(dc.drum_types, vec!["bd", "sn"]);
    assert_eq!(dc.duration.as_ref().unwrap().base, 4);
}

#[test]
fn test_drums_shorthand() {
    let src = r#"\drums { bd4 sn8 }"#;
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
    assert_eq!(context_type, "DrumStaff");
    let Music::DrumMode { body } = music.as_ref() else {
        panic!("expected DrumMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 2);
}

#[test]
fn test_drummode_post_events() {
    let src = r#"\drummode { bd4( sn4) }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::DrumMode { body }) = &file.items[0] else {
        panic!("expected DrumMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    let Music::DrumNote(dn0) = &items[0] else {
        panic!("expected DrumNote");
    };
    assert!(dn0.post_events.contains(&PostEvent::SlurStart));
    let Music::DrumNote(dn1) = &items[1] else {
        panic!("expected DrumNote");
    };
    assert!(dn1.post_events.contains(&PostEvent::SlurEnd));
}

#[test]
fn test_drummode_roundtrip_serialization() {
    let src = r#"\drummode { bd4 sn4 hh8 hh8 }"#;
    let file = parse(src).unwrap();
    let output = crate::serializer::serialize(&file);
    let file2 = parse(&output).unwrap();
    assert_eq!(file.items.len(), file2.items.len());
}

#[test]
fn test_drummode_chord_roundtrip() {
    let src = r#"\drummode { <bd sn>4 hh8 }"#;
    let file = parse(src).unwrap();
    let output = crate::serializer::serialize(&file);
    let file2 = parse(&output).unwrap();
    assert_eq!(file.items.len(), file2.items.len());
}

#[test]
fn test_drummode_fixture_parse() {
    let src = std::fs::read_to_string("tests/fixtures/lilypond/fragment_drummode.ly")
        .or_else(|_| {
            std::fs::read_to_string("../../../tests/fixtures/lilypond/fragment_drummode.ly")
        })
        .expect("fixture file should exist");
    let file = parse(&src).unwrap();
    assert!(file.version.is_some());
    assert_eq!(file.items.len(), 1);
}

#[test]
fn test_drummode_validation() {
    let src = r#"\drummode { bd4 sn4 hh4 }"#;
    let file = parse(src).unwrap();
    crate::validator::validate(&file).unwrap();
}

#[test]
fn test_drummode_skip() {
    let src = r#"\drummode { bd4 s4 hh4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::DrumMode { body }) = &file.items[0] else {
        panic!("expected DrumMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 3);
    assert!(matches!(items[1], Music::Skip(_)));
}

#[test]
fn test_drummode_various_pitches() {
    // Test a mix of common abbreviations
    let src = r#"\drummode { cymc4 hhc16 hho8 tomfl4 tommh4 }"#;
    let file = parse(src).unwrap();
    let ToplevelExpression::Music(Music::DrumMode { body }) = &file.items[0] else {
        panic!("expected DrumMode");
    };
    let Music::Sequential(items) = body.as_ref() else {
        panic!("expected Sequential");
    };
    assert_eq!(items.len(), 5);
    for item in items {
        assert!(matches!(item, Music::DrumNote(_)));
    }
}

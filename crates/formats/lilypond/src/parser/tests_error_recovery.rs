//! Tests for error recovery and erroneous quotes handling.

use super::*;

// ---------------------------------------------------------------------------
// Erroneous quotes: mixed ' and , in octave marks
// ---------------------------------------------------------------------------

#[test]
fn mixed_octave_marks_warns() {
    let (ast, warnings) = parse_with_warnings("{ c', }").unwrap();
    // Net octave from `',` is 0 (one up, one down)
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            let n = match &items[0] {
                Music::Note(n) => n,
                other => panic!("expected Note, got {other:?}"),
            };
            assert_eq!(n.pitch.octave, 0);
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
    assert_eq!(warnings.len(), 1);
    assert!(matches!(
        &warnings[0],
        ParseWarning::MixedOctaveMarks { .. }
    ));
}

#[test]
fn mixed_octave_marks_comma_then_quote() {
    let (ast, warnings) = parse_with_warnings("{ c,' }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            let n = match &items[0] {
                Music::Note(n) => n,
                other => panic!("expected Note, got {other:?}"),
            };
            assert_eq!(n.pitch.octave, 0);
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
    assert_eq!(warnings.len(), 1);
    assert!(matches!(
        &warnings[0],
        ParseWarning::MixedOctaveMarks { .. }
    ));
}

#[test]
fn consistent_octave_marks_no_warning() {
    let (_, warnings) = parse_with_warnings("{ c'' d,, }").unwrap();
    assert!(warnings.is_empty());
}

// ---------------------------------------------------------------------------
// Erroneous quotes: octave marks after duration
// ---------------------------------------------------------------------------

#[test]
fn octave_after_duration_warns_and_applies() {
    // `c4''` — the `''` after duration 4 should warn and apply to pitch
    let (ast, warnings) = parse_with_warnings("{ c4'' }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            let n = match &items[0] {
                Music::Note(n) => n,
                other => panic!("expected Note, got {other:?}"),
            };
            assert_eq!(n.pitch.step, 'c');
            assert_eq!(n.pitch.octave, 2); // erroneous quotes applied
            assert_eq!(n.duration.as_ref().unwrap().base, 4);
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
    assert_eq!(warnings.len(), 1);
    assert!(matches!(
        &warnings[0],
        ParseWarning::OctaveAfterDuration { .. }
    ));
}

#[test]
fn octave_after_duration_with_octave_check() {
    // `c='4,` — erroneous comma after dur should add to octave_check
    let (ast, warnings) = parse_with_warnings("{ c='4, }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            let n = match &items[0] {
                Music::Note(n) => n,
                other => panic!("expected Note, got {other:?}"),
            };
            assert_eq!(n.pitch.octave_check, Some(0)); // 1 (from =') + -1 (erroneous ,)
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
    assert_eq!(warnings.len(), 1);
    assert!(matches!(
        &warnings[0],
        ParseWarning::OctaveAfterDuration { .. }
    ));
}

#[test]
fn no_octave_after_duration_no_warning() {
    let (_, warnings) = parse_with_warnings("{ c''4 }").unwrap();
    assert!(warnings.is_empty());
}

// ---------------------------------------------------------------------------
// Recovery: parse error inside sequential block
// ---------------------------------------------------------------------------

#[test]
fn recovery_in_sequential_block() {
    // `{ c4 = d4 }` — `=` is unexpected in music position; recovery should
    // skip past it and continue parsing d4
    let (ast, warnings) = parse_with_warnings("{ c4 = d4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            // c4 should be parsed successfully
            assert!(matches!(&items[0], Music::Note(_)));
            // After recovery, d4 should also be parsed
            assert!(items.len() >= 2);
            if let Music::Note(n) = &items[items.len() - 1] {
                assert_eq!(n.pitch.step, 'd');
            }
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
    assert!(!warnings.is_empty());
    assert!(
        warnings
            .iter()
            .any(|w| matches!(w, ParseWarning::RecoveredError { .. }))
    );
}

#[test]
fn recovery_stops_at_barcheck() {
    // Recovery should stop at `|` (bar check) and continue parsing
    let (ast, warnings) = parse_with_warnings("{ c4 = | d4 }").unwrap();
    match &ast.items[0] {
        ToplevelExpression::Music(Music::Sequential(items)) => {
            assert!(matches!(&items[0], Music::Note(_)));
            // After recovery at `|`, d4 should be parsed
            assert!(items.len() >= 2);
        }
        other => panic!("expected Sequential, got {other:?}"),
    }
    assert!(!warnings.is_empty());
}

// ---------------------------------------------------------------------------
// Improved error messages
// ---------------------------------------------------------------------------

#[test]
fn unmatched_brace_eof_error() {
    let result = parse("{ c4");
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("closing brace"),
        "error should mention closing brace: {msg}"
    );
}

#[test]
fn missing_closing_angle_bracket() {
    let result = parse("<< { c4 }");
    let err = result.unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("'>>'"), "error should mention '>>': {msg}");
}

// ---------------------------------------------------------------------------
// ParseWarning Display
// ---------------------------------------------------------------------------

#[test]
fn warning_display_mixed_octave() {
    let w = ParseWarning::MixedOctaveMarks { offset: 42 };
    let s = w.to_string();
    assert!(s.contains("mixed"));
    assert!(s.contains("42"));
}

#[test]
fn warning_display_octave_after_duration() {
    let w = ParseWarning::OctaveAfterDuration {
        offset: 10,
        note_offset: 5,
    };
    let s = w.to_string();
    assert!(s.contains("precede duration"));
}

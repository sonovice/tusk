//! Import tests for tempo, mark, and textMark.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{Mei, MeiChild, ScoreChild, SectionChild};

fn parse_and_import(src: &str) -> Mei {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Walk MEI to find all Tempo control events in the first measure.
fn measure_tempos(mei: &Mei) -> Vec<&tusk_model::elements::Tempo> {
    let mut tempos = Vec::new();
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let tusk_model::elements::MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let ScoreChild::Section(section) = sc {
                                for sec_c in &section.children {
                                    if let SectionChild::Measure(measure) = sec_c {
                                        for mc2 in &measure.children {
                                            if let MeasureChild::Tempo(t) = mc2 {
                                                tempos.push(t.as_ref());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    tempos
}

/// Walk MEI to find all Dir control events in the first measure.
fn measure_dirs(mei: &Mei) -> Vec<&tusk_model::elements::Dir> {
    let mut dirs = Vec::new();
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let tusk_model::elements::MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let ScoreChild::Section(section) = sc {
                                for sec_c in &section.children {
                                    if let SectionChild::Measure(measure) = sec_c {
                                        for mc2 in &measure.children {
                                            if let MeasureChild::Dir(d) = mc2 {
                                                dirs.push(d.as_ref());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    dirs
}

/// Walk MEI to find the staffDef label.
fn staff_def_label(mei: &Mei) -> Option<String> {
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let tusk_model::elements::MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let ScoreChild::ScoreDef(sd) = sc {
                                for sdc in &sd.children {
                                    if let tusk_model::elements::ScoreDefChild::StaffGrp(grp) = sdc
                                    {
                                        for gc in &grp.children {
                                            if let tusk_model::elements::StaffGrpChild::StaffDef(
                                                sdef,
                                            ) = gc
                                            {
                                                return sdef.labelled.label.clone();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Tempo import tests
// ---------------------------------------------------------------------------

#[test]
fn import_tempo_creates_mei_tempo() {
    let mei = parse_and_import(r#"{ \tempo "Allegro" 4 = 120 c'4 }"#);
    let tempos = measure_tempos(&mei);
    assert_eq!(tempos.len(), 1, "should create one <tempo>");

    let t = tempos[0];
    // Check @mm = 120
    assert_eq!(t.tempo_log.mm.as_ref().map(|v| v.0), Some(120.0));
    // Check @startid references the note
    assert!(t.tempo_log.startid.is_some());
    // Check text content
    assert!(!t.children.is_empty(), "should have text content");
}

#[test]
fn import_tempo_metronome_only() {
    let mei = parse_and_import(r#"{ \tempo 2 = 60 c'4 }"#);
    let tempos = measure_tempos(&mei);
    assert_eq!(tempos.len(), 1);

    let t = tempos[0];
    assert_eq!(t.tempo_log.mm.as_ref().map(|v| v.0), Some(60.0));
}

#[test]
fn import_tempo_text_only() {
    let mei = parse_and_import(r#"{ \tempo "Andante" c'4 }"#);
    let tempos = measure_tempos(&mei);
    assert_eq!(tempos.len(), 1);

    let t = tempos[0];
    // No @mm for text-only tempo
    assert!(t.tempo_log.mm.is_none());
    // Should have text content
    assert!(!t.children.is_empty());
}

#[test]
fn import_tempo_range() {
    let mei = parse_and_import(r#"{ \tempo "Vivace" 4. = 132-144 c'4 }"#);
    let tempos = measure_tempos(&mei);
    assert_eq!(tempos.len(), 1);

    let t = tempos[0];
    // @mm = 132 (low end)
    assert_eq!(t.tempo_log.mm.as_ref().map(|v| v.0), Some(132.0));
    // @mm.dots = 1
    assert_eq!(t.tempo_log.mm_dots.as_ref().map(|d| d.0), Some(1));
}

// ---------------------------------------------------------------------------
// Mark import tests
// ---------------------------------------------------------------------------

#[test]
fn import_mark_creates_dir() {
    let mei = parse_and_import(r#"{ \mark \default c'4 }"#);
    let dirs = measure_dirs(&mei);
    let mark_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .label
                .as_deref()
                .is_some_and(|l| l.starts_with("lilypond:mark,"))
        })
        .collect();
    assert_eq!(mark_dirs.len(), 1, "should create one mark Dir");
}

#[test]
fn import_mark_string() {
    let mei = parse_and_import(r#"{ \mark "A" c'4 }"#);
    let dirs = measure_dirs(&mei);
    let mark_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .label
                .as_deref()
                .is_some_and(|l| l.starts_with("lilypond:mark,"))
        })
        .collect();
    assert_eq!(mark_dirs.len(), 1);
}

#[test]
fn import_textmark_creates_dir() {
    let mei = parse_and_import(r#"{ \textMark "Fine" c'4 }"#);
    let dirs = measure_dirs(&mei);
    let textmark_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .label
                .as_deref()
                .is_some_and(|l| l.starts_with("lilypond:textmark,"))
        })
        .collect();
    assert_eq!(textmark_dirs.len(), 1, "should create one textMark Dir");
}

// ---------------------------------------------------------------------------
// Event sequence label tests
// ---------------------------------------------------------------------------

#[test]
fn import_tempo_in_event_sequence() {
    let mei = parse_and_import(r#"{ \tempo 4 = 100 c'4 d'4 }"#);
    let label = staff_def_label(&mei).expect("should have staffDef label");
    assert!(
        label.contains("\"Tempo\""),
        "event sequence should contain Tempo: {label}"
    );
}

#[test]
fn import_mark_in_event_sequence() {
    let mei = parse_and_import(r#"{ \mark \default c'4 d'4 }"#);
    let label = staff_def_label(&mei).expect("should have staffDef label");
    assert!(
        label.contains("\"Mark\""),
        "event sequence should contain Mark: {label}"
    );
}

#[test]
fn import_textmark_in_event_sequence() {
    let mei = parse_and_import(r#"{ \textMark "Fine" c'4 d'4 }"#);
    let label = staff_def_label(&mei).expect("should have staffDef label");
    assert!(
        label.contains("\"TextMark\""),
        "event sequence should contain TextMark: {label}"
    );
}

// ---------------------------------------------------------------------------
// Fixture test
// ---------------------------------------------------------------------------

#[test]
fn import_fixture_tempo_marks() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_tempo_marks.ly"
    ))
    .unwrap();
    let mei = parse_and_import(&src);

    // Should have multiple tempo control events
    let tempos = measure_tempos(&mei);
    assert!(
        tempos.len() >= 3,
        "should have at least 3 tempos: got {}",
        tempos.len()
    );

    // Should have mark/textMark dirs
    let dirs = measure_dirs(&mei);
    let mark_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common.label.as_deref().is_some_and(|l| {
                l.starts_with("lilypond:mark,") || l.starts_with("lilypond:textmark,")
            })
        })
        .collect();
    assert!(
        mark_dirs.len() >= 3,
        "should have at least 3 mark/textMark dirs: got {}",
        mark_dirs.len()
    );

    // Event sequence should have all entries
    let label = staff_def_label(&mei).expect("should have staffDef label");
    assert!(
        label.contains("\"Tempo\""),
        "event sequence should contain Tempo entries: {label}"
    );
    assert!(
        label.contains("\"Mark\""),
        "event sequence should contain Mark entries: {label}"
    );
    assert!(
        label.contains("\"TextMark\""),
        "event sequence should contain TextMark entries: {label}"
    );
}

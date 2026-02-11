//! Tests for property operation import to MEI.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{Mei, MeiChild};

fn parse_and_import(src: &str) -> Mei {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Get all Dir elements from the first measure.
fn collect_dirs(mei: &Mei) -> Vec<&tusk_model::elements::Dir> {
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
                            if let tusk_model::elements::ScoreChild::Section(section) = sc {
                                for sec_c in &section.children {
                                    if let tusk_model::elements::SectionChild::Measure(measure) =
                                        sec_c
                                    {
                                        for mc2 in &measure.children {
                                            if let MeasureChild::Dir(dir) = mc2 {
                                                dirs.push(dir.as_ref());
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

#[test]
fn import_override_creates_dir() {
    let mei = parse_and_import("{ \\override NoteHead.color = #red c4 d e f }");
    let dirs = collect_dirs(&mei);
    let prop_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .label
                .as_deref()
                .is_some_and(|l| l.starts_with("lilypond:prop,"))
        })
        .collect();
    assert_eq!(prop_dirs.len(), 1, "expected one property dir");
    let label = prop_dirs[0].common.label.as_deref().unwrap();
    assert!(
        label.contains("override"),
        "label should contain override: {label}"
    );
    assert!(
        label.contains("NoteHead.color"),
        "label should contain path: {label}"
    );
}

#[test]
fn import_set_creates_dir() {
    let mei = parse_and_import("{ \\set Staff.instrumentName = \"Piano\" c4 }");
    let dirs = collect_dirs(&mei);
    let prop_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .label
                .as_deref()
                .is_some_and(|l| l.starts_with("lilypond:prop,"))
        })
        .collect();
    assert_eq!(prop_dirs.len(), 1);
    let label = prop_dirs[0].common.label.as_deref().unwrap();
    assert!(label.contains("set"), "label should contain set: {label}");
}

#[test]
fn import_revert_creates_dir() {
    let mei =
        parse_and_import("{ \\override NoteHead.color = #red c4 \\revert NoteHead.color d4 }");
    let dirs = collect_dirs(&mei);
    let prop_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .label
                .as_deref()
                .is_some_and(|l| l.starts_with("lilypond:prop,"))
        })
        .collect();
    assert_eq!(prop_dirs.len(), 2, "expected two property dirs");
}

#[test]
fn import_once_override_creates_dir() {
    let mei = parse_and_import("{ \\once \\override NoteHead.color = #red c4 }");
    let dirs = collect_dirs(&mei);
    let prop_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .label
                .as_deref()
                .is_some_and(|l| l.starts_with("lilypond:prop,"))
        })
        .collect();
    assert_eq!(prop_dirs.len(), 1);
    let label = prop_dirs[0].common.label.as_deref().unwrap();
    assert!(label.contains("once"), "label should contain once: {label}");
}

#[test]
fn import_tweak_on_note_label() {
    let mei = parse_and_import("{ c4\\tweak color #red -. }");
    // Find the first note and check its label
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    for dc in &mdiv.children {
                        let tusk_model::elements::MdivChild::Score(score) = dc;
                        for sc in &score.children {
                            if let tusk_model::elements::ScoreChild::Section(section) = sc {
                                for sec_c in &section.children {
                                    if let tusk_model::elements::SectionChild::Measure(measure) =
                                        sec_c
                                    {
                                        for mc2 in &measure.children {
                                            if let MeasureChild::Staff(staff) = mc2 {
                                                for sl in &staff.children {
                                                    let tusk_model::elements::StaffChild::Layer(
                                                        layer,
                                                    ) = sl;
                                                    for lc in &layer.children {
                                                        if let LayerChild::Note(note) = lc {
                                                            let label = note
                                                                .common
                                                                .label
                                                                .as_deref()
                                                                .unwrap_or("");
                                                            assert!(
                                                                label.contains("tusk:tweak,"),
                                                                "note label should contain tweak: {label}"
                                                            );
                                                            return;
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
            }
        }
    }
    panic!("no note found in MEI");
}

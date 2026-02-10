//! Import tests for drum events → MEI notes with drum labels.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{Mei, MeiChild, ScoreChild, SectionChild};

fn parse_and_import(src: &str) -> Mei {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Walk MEI to find all notes in the first layer of the first staff.
fn layer_notes(mei: &Mei) -> Vec<&tusk_model::elements::Note> {
    let mut notes = Vec::new();
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
                                            if let MeasureChild::Staff(staff) = mc2 {
                                                for lc in &staff.children {
                                                    let tusk_model::elements::StaffChild::Layer(
                                                        layer,
                                                    ) = lc;
                                                    for item in &layer.children {
                                                        if let LayerChild::Note(note) = item {
                                                            notes.push(note.as_ref());
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
    notes
}

#[test]
fn import_drummode_creates_notes() {
    let mei = parse_and_import("\\drummode { bd4 sn4 hh4 }");
    let notes = layer_notes(&mei);
    assert_eq!(notes.len(), 3, "should create 3 notes from drum events");
}

#[test]
fn drum_note_has_label() {
    let mei = parse_and_import("\\drummode { bd4 }");
    let notes = layer_notes(&mei);
    assert_eq!(notes.len(), 1);
    let label = notes[0].common.label.as_deref().unwrap();
    assert!(
        label.starts_with("lilypond:drum,"),
        "label should have drum prefix: {label}"
    );
    assert!(
        label.contains("bd"),
        "label should contain drum type: {label}"
    );
}

#[test]
fn drum_chord_has_label() {
    let mei = parse_and_import("\\drummode { <bd sn>4 }");
    let notes = layer_notes(&mei);
    assert_eq!(notes.len(), 1);
    let label = notes[0].common.label.as_deref().unwrap();
    assert!(
        label.starts_with("lilypond:drum,"),
        "label should have drum prefix: {label}"
    );
    assert!(label.contains("bd"), "label should contain bd: {label}");
    assert!(label.contains("sn"), "label should contain sn: {label}");
}

#[test]
fn drum_note_has_xml_id() {
    let mei = parse_and_import("\\drummode { bd4 }");
    let notes = layer_notes(&mei);
    assert!(
        notes[0].common.xml_id.is_some(),
        "drum note should have xml:id"
    );
    assert!(
        notes[0]
            .common
            .xml_id
            .as_deref()
            .unwrap()
            .starts_with("ly-note-"),
        "xml:id should start with ly-note-"
    );
}

#[test]
fn drum_note_has_duration() {
    let mei = parse_and_import("\\drummode { bd8 }");
    let notes = layer_notes(&mei);
    assert!(
        notes[0].note_log.dur.is_some(),
        "drum note should have @dur"
    );
}

#[test]
fn import_drum_staff_context() {
    let mei = parse_and_import("\\new DrumStaff \\drummode { bd4 sn4 }");
    let notes = layer_notes(&mei);
    assert_eq!(notes.len(), 2);
    // Should have DrumStaff context in staffDef label
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
                                    if let ScoreDefChild::StaffGrp(grp) = sdc {
                                        for gc in &grp.children {
                                            if let StaffGrpChild::StaffDef(sdef) = gc {
                                                let label = sdef.labelled.label.as_deref().unwrap();
                                                assert!(
                                                    label.contains("DrumStaff"),
                                                    "staffDef label should contain DrumStaff: {label}"
                                                );
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

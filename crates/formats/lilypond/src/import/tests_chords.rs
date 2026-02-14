//! Import tests for chord-mode events → MEI harm.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{HarmChild, Mei, MeiChild, ScoreChild, SectionChild};
use tusk_model::ExtensionStore;

fn parse_and_import(src: &str) -> (Mei, ExtensionStore) {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Walk MEI to find all Harm control events in the first measure.
fn measure_harms(mei: &Mei) -> Vec<&tusk_model::elements::Harm> {
    let mut harms = Vec::new();
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
                                            if let MeasureChild::Harm(h) = mc2 {
                                                harms.push(h.as_ref());
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
    harms
}

#[test]
fn import_chordmode_creates_harm_elements() {
    let (mei, _ext_store) = parse_and_import("\\chordmode { c1 c:m c:7 c:dim7/f }");
    let harms = measure_harms(&mei);
    assert_eq!(harms.len(), 4, "should create 4 harm elements");
}

#[test]
fn harm_has_chord_mode_label() {
    let (mei, ext_store) = parse_and_import("\\chordmode { c1 }");
    let harms = measure_harms(&mei);
    assert_eq!(harms.len(), 1);
    let id = harms[0].common.xml_id.as_deref().unwrap();
    assert!(
        ext_store.chord_mode_info(id).is_some(),
        "should have chord_mode_info in ext_store"
    );
}

#[test]
fn harm_has_text_child() {
    let (mei, _ext_store) = parse_and_import("\\chordmode { c:m7 }");
    let harms = measure_harms(&mei);
    assert_eq!(harms.len(), 1);
    let text = match &harms[0].children[0] {
        HarmChild::Text(t) => t.as_str(),
    };
    // Should contain the serialized chord mode event
    assert!(text.contains("c"), "text should contain root: {text}");
    assert!(text.contains("m"), "text should contain m: {text}");
    assert!(text.contains("7"), "text should contain 7: {text}");
}

#[test]
fn harm_has_tstamp_for_standalone() {
    // Standalone chordmode with no notes uses @tstamp
    let (mei, _ext_store) = parse_and_import("\\chordmode { c1 d1 }");
    let harms = measure_harms(&mei);
    assert_eq!(harms.len(), 2);
    // First chord at beat 1
    assert!(
        harms[0].harm_log.tstamp.is_some(),
        "first harm should have @tstamp"
    );
    assert_eq!(harms[0].harm_log.tstamp.as_ref().unwrap().0, 1.0);
    // Second chord at beat 5 (after a whole note = 4 beats)
    assert!(
        harms[1].harm_log.tstamp.is_some(),
        "second harm should have @tstamp"
    );
    assert_eq!(harms[1].harm_log.tstamp.as_ref().unwrap().0, 5.0);
}

#[test]
fn import_chords_shorthand() {
    // \chords is shorthand for \new ChordNames \chordmode
    let (mei, _ext_store) = parse_and_import(r#"\score { << \chords { c1 d:m } \new Staff { c'1 d'1 } >> }"#);
    let harms = measure_harms(&mei);
    assert_eq!(harms.len(), 2, "should create 2 harm elements");
}

#[test]
fn harm_label_contains_quality() {
    let (mei, ext_store) = parse_and_import("\\chordmode { c:dim7/f }");
    let harms = measure_harms(&mei);
    let id = harms[0].common.xml_id.as_deref().unwrap();
    let info = ext_store.chord_mode_info(id).expect("should have chord_mode_info");
    assert!(info.serialized.contains("dim"), "serialized should contain dim: {}", info.serialized);
    assert!(info.serialized.contains("7"), "serialized should contain 7: {}", info.serialized);
    assert!(info.serialized.contains("f"), "serialized should contain f: {}", info.serialized);
}

#[test]
fn harm_xml_id_is_set() {
    let (mei, _ext_store) = parse_and_import("\\chordmode { c1 }");
    let harms = measure_harms(&mei);
    assert!(harms[0].common.xml_id.is_some(), "harm should have xml:id");
    assert!(
        harms[0]
            .common
            .xml_id
            .as_deref()
            .unwrap()
            .starts_with("ly-harm-"),
        "harm xml:id should start with ly-harm-"
    );
}

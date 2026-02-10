//! Import tests for chord-mode events → MEI harm.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{HarmChild, Mei, MeiChild, ScoreChild, SectionChild};

fn parse_and_import(src: &str) -> Mei {
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
    let mei = parse_and_import("\\chordmode { c1 c:m c:7 c:dim7/f }");
    let harms = measure_harms(&mei);
    assert_eq!(harms.len(), 4, "should create 4 harm elements");
}

#[test]
fn harm_has_chord_mode_label() {
    let mei = parse_and_import("\\chordmode { c1 }");
    let harms = measure_harms(&mei);
    assert_eq!(harms.len(), 1);
    let label = harms[0].common.label.as_deref().unwrap();
    assert!(
        label.starts_with("lilypond:chord-mode,"),
        "label should have chord-mode prefix: {label}"
    );
}

#[test]
fn harm_has_text_child() {
    let mei = parse_and_import("\\chordmode { c:m7 }");
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
    let mei = parse_and_import("\\chordmode { c1 d1 }");
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
    let mei = parse_and_import(r#"\score { << \chords { c1 d:m } \new Staff { c'1 d'1 } >> }"#);
    let harms = measure_harms(&mei);
    assert_eq!(harms.len(), 2, "should create 2 harm elements");
}

#[test]
fn harm_label_contains_quality() {
    let mei = parse_and_import("\\chordmode { c:dim7/f }");
    let harms = measure_harms(&mei);
    let label = harms[0].common.label.as_deref().unwrap();
    assert!(label.contains("dim"), "label should contain dim: {label}");
    assert!(label.contains("7"), "label should contain 7: {label}");
    assert!(label.contains("f"), "label should contain f: {label}");
}

#[test]
fn harm_xml_id_is_set() {
    let mei = parse_and_import("\\chordmode { c1 }");
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

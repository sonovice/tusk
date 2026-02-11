//! Import tests for figured bass events → MEI fb.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{FbChild, Mei, MeiChild, ScoreChild, SectionChild};

fn parse_and_import(src: &str) -> Mei {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Walk MEI to find all Fb control events in the first measure.
fn measure_fbs(mei: &Mei) -> Vec<&tusk_model::elements::Fb> {
    let mut fbs = Vec::new();
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
                                            if let MeasureChild::Fb(fb) = mc2 {
                                                fbs.push(fb.as_ref());
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
    fbs
}

#[test]
fn import_figures_creates_fb_elements() {
    let mei = parse_and_import("\\figures { \\<6 4\\>4 \\<7 5\\>4 \\<3\\>2 }");
    let fbs = measure_fbs(&mei);
    assert_eq!(fbs.len(), 3, "should create 3 fb elements");
}

#[test]
fn fb_has_figure_label() {
    let mei = parse_and_import("\\figures { \\<6 4\\>4 }");
    let fbs = measure_fbs(&mei);
    assert_eq!(fbs.len(), 1);
    let label = fbs[0].common.label.as_deref().unwrap();
    assert!(
        label.starts_with("tusk:figure,"),
        "label should have figure prefix: {label}"
    );
}

#[test]
fn fb_has_f_children() {
    let mei = parse_and_import("\\figures { \\<6 4\\>4 }");
    let fbs = measure_fbs(&mei);
    assert_eq!(fbs.len(), 1);
    assert_eq!(fbs[0].children.len(), 2, "should have 2 <f> children");
    // Check first <f> text
    let FbChild::F(f0) = &fbs[0].children[0];
    let text0: String = f0
        .children
        .iter()
        .map(|c| {
            let tusk_model::elements::FChild::Text(t) = c;
            t.as_str()
        })
        .collect();
    assert_eq!(text0, "6");
    // Check second <f> text
    let FbChild::F(f1) = &fbs[0].children[1];
    let text1: String = f1
        .children
        .iter()
        .map(|c| {
            let tusk_model::elements::FChild::Text(t) = c;
            t.as_str()
        })
        .collect();
    assert_eq!(text1, "4");
}

#[test]
fn fb_xml_id_is_set() {
    let mei = parse_and_import("\\figures { \\<6 4\\>4 }");
    let fbs = measure_fbs(&mei);
    assert!(fbs[0].common.xml_id.is_some(), "fb should have xml:id");
    assert!(
        fbs[0]
            .common
            .xml_id
            .as_deref()
            .unwrap()
            .starts_with("ly-fb-"),
        "fb xml:id should start with ly-fb-"
    );
}

#[test]
fn import_figuremode_creates_fb() {
    let mei = parse_and_import("\\figuremode { \\<5\\+ 3\\>4 }");
    let fbs = measure_fbs(&mei);
    assert_eq!(fbs.len(), 1, "figuremode should create fb");
}

#[test]
fn fb_alterations_in_text() {
    let mei = parse_and_import("\\figuremode { \\<6+ 4-\\>4 }");
    let fbs = measure_fbs(&mei);
    assert_eq!(fbs.len(), 1);
    let FbChild::F(f0) = &fbs[0].children[0];
    let text0: String = f0
        .children
        .iter()
        .map(|c| {
            let tusk_model::elements::FChild::Text(t) = c;
            t.as_str()
        })
        .collect();
    assert_eq!(text0, "6#", "sharp alteration should show as #");
    let FbChild::F(f1) = &fbs[0].children[1];
    let text1: String = f1
        .children
        .iter()
        .map(|c| {
            let tusk_model::elements::FChild::Text(t) = c;
            t.as_str()
        })
        .collect();
    assert_eq!(text1, "4b", "flat alteration should show as b");
}

#[test]
fn fb_figure_space() {
    let mei = parse_and_import("\\figuremode { \\<6! _\\>2 }");
    let fbs = measure_fbs(&mei);
    assert_eq!(fbs.len(), 1);
    // Second figure should be space
    let FbChild::F(f1) = &fbs[0].children[1];
    let text1: String = f1
        .children
        .iter()
        .map(|c| {
            let tusk_model::elements::FChild::Text(t) = c;
            t.as_str()
        })
        .collect();
    assert_eq!(text1, "_", "figure space should show as _");
}

//! Tests for music function call import to MEI.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::{Mei, MeiChild};
use tusk_model::ExtensionStore;

fn parse_and_import(src: &str) -> (Mei, ExtensionStore) {
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
fn import_music_function_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ \\someFunction { c4 d e f } g4 }");
    let dirs = collect_dirs(&mei);
    let func_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .xml_id
                .as_deref()
                .is_some_and(|id| ext_store.function_call(id).is_some())
        })
        .collect();
    assert_eq!(func_dirs.len(), 1, "expected one function dir");
    let id = func_dirs[0].common.xml_id.as_deref().unwrap();
    let fc = ext_store.function_call(id).unwrap();
    assert_eq!(fc.name, "someFunction");
}

#[test]
fn import_music_function_with_string_arg() {
    let (mei, ext_store) = parse_and_import("{ \\tag \"part\" { c4 d e f } g4 }");
    let dirs = collect_dirs(&mei);
    let func_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .xml_id
                .as_deref()
                .is_some_and(|id| ext_store.function_call(id).is_some())
        })
        .collect();
    assert_eq!(func_dirs.len(), 1);
    let id = func_dirs[0].common.xml_id.as_deref().unwrap();
    let fc = ext_store.function_call(id).unwrap();
    assert_eq!(fc.name, "tag");
}

#[test]
fn import_music_function_has_startid() {
    let (mei, ext_store) = parse_and_import("{ \\someFunction { c4 d } e4 }");
    let dirs = collect_dirs(&mei);
    let func_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .xml_id
                .as_deref()
                .is_some_and(|id| ext_store.function_call(id).is_some())
        })
        .collect();
    assert_eq!(func_dirs.len(), 1);
    // The function dir should have a startid pointing to a note
    assert!(
        func_dirs[0].dir_log.startid.is_some(),
        "function dir should have a startid"
    );
}

#[test]
fn import_partial_function_creates_dir() {
    let (mei, ext_store) = parse_and_import("{ \\tag #'score \\etc c4 d e f }");
    let dirs = collect_dirs(&mei);
    let func_dirs: Vec<_> = dirs
        .iter()
        .filter(|d| {
            d.common
                .xml_id
                .as_deref()
                .is_some_and(|id| ext_store.function_call(id).is_some())
        })
        .collect();
    assert_eq!(func_dirs.len(), 1, "expected one function dir for partial");
    let id = func_dirs[0].common.xml_id.as_deref().unwrap();
    let fc = ext_store.function_call(id).unwrap();
    assert_eq!(fc.name, "tag");
    assert!(fc.is_partial, "should be partial function");
}

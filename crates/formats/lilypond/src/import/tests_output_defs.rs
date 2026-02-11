//! Tests for import of header, paper, layout, and midi blocks.

use crate::import::import;
use crate::parser::Parser;
use tusk_model::elements::{MeiChild, MeiHeadChild};

fn parse(src: &str) -> crate::model::LilyPondFile {
    Parser::new(src).unwrap().parse().unwrap()
}

/// Collect all ExtMeta labels from MeiHead.
fn ext_meta_labels(mei: &tusk_model::elements::Mei) -> Vec<String> {
    let mut labels = Vec::new();
    for child in &mei.children {
        if let MeiChild::MeiHead(head) = child {
            for hc in &head.children {
                if let MeiHeadChild::ExtMeta(ext) = hc
                    && let Some(label) = &ext.common.label
                {
                    labels.push(label.clone());
                }
            }
        }
    }
    labels
}

/// Get the ScoreDef label from MEI.
fn score_def_label(mei: &tusk_model::elements::Mei) -> String {
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            let body = match music.children.first() {
                Some(tusk_model::elements::MusicChild::Body(b)) => b,
                _ => continue,
            };
            let mdiv = match body.children.first() {
                Some(tusk_model::elements::BodyChild::Mdiv(m)) => m,
                _ => continue,
            };
            let score = match mdiv.children.first() {
                Some(tusk_model::elements::MdivChild::Score(s)) => s,
                _ => continue,
            };
            for sc in &score.children {
                if let tusk_model::elements::ScoreChild::ScoreDef(sd) = sc {
                    return sd.common.label.clone().unwrap_or_default();
                }
            }
        }
    }
    String::new()
}

#[test]
fn header_title_populates_mei_title() {
    let file = parse("\\header { title = \"My Title\" }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    for child in &mei.children {
        if let MeiChild::MeiHead(head) = child {
            for hc in &head.children {
                if let MeiHeadChild::FileDesc(fd) = hc {
                    for fc in &fd.children {
                        if let tusk_model::elements::FileDescChild::TitleStmt(ts) = fc {
                            assert!(!ts.children.is_empty(), "titleStmt should have title");
                            let tusk_model::elements::TitleStmtChild::Title(t) = &ts.children[0];
                            assert!(!t.children.is_empty());
                            let tusk_model::elements::TitleChild::Text(s) = &t.children[0] else {
                                panic!("expected text child");
                            };
                            assert_eq!(s, "My Title");
                            return;
                        }
                    }
                }
            }
        }
    }
    panic!("title not found in MEI");
}

#[test]
fn header_stored_as_typed_output_def() {
    let file = parse("\\header { title = \"Test\" composer = \"Bach\" }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    let output_defs_label = labels
        .iter()
        .find(|l| l.starts_with("tusk:output-defs,"))
        .expect("output-defs ExtMeta not found");
    // JSON should contain Header kind with title and composer assignments
    assert!(
        output_defs_label.contains("Header"),
        "got: {output_defs_label}"
    );
    assert!(
        output_defs_label.contains("title"),
        "got: {output_defs_label}"
    );
    assert!(
        output_defs_label.contains("composer"),
        "got: {output_defs_label}"
    );
}

#[test]
fn output_defs_ext_meta_has_summary_text() {
    let file = parse("\\header { title = \"Test\" }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    for child in &mei.children {
        if let MeiChild::MeiHead(head) = child {
            for hc in &head.children {
                if let MeiHeadChild::ExtMeta(ext) = hc
                    && ext
                        .common
                        .label
                        .as_deref()
                        .is_some_and(|l| l.starts_with("tusk:output-defs,"))
                {
                    assert!(!ext.children.is_empty());
                    let tusk_model::elements::ExtMetaChild::Text(t) = &ext.children[0];
                    assert!(t.contains("Header"), "summary should list kind, got: {t}");
                    return;
                }
            }
        }
    }
    panic!("output-defs ExtMeta not found");
}

#[test]
fn paper_stored_as_typed_output_def() {
    let file = parse("\\paper { indent = 0 }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    let output_defs_label = labels
        .iter()
        .find(|l| l.starts_with("tusk:output-defs,"))
        .expect("output-defs ExtMeta not found");
    assert!(
        output_defs_label.contains("Paper"),
        "got: {output_defs_label}"
    );
    assert!(
        output_defs_label.contains("indent"),
        "got: {output_defs_label}"
    );
}

#[test]
fn layout_stored_as_typed_output_def() {
    let file = parse("\\layout { ragged-right = ##t }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    assert!(
        labels.iter().any(|l| l.starts_with("tusk:output-defs,")),
        "output-defs ExtMeta not found"
    );
}

#[test]
fn midi_stored_as_typed_output_def() {
    let file = parse("\\midi { }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    assert!(
        labels.iter().any(|l| l.starts_with("tusk:output-defs,")),
        "output-defs ExtMeta not found"
    );
}

#[test]
fn score_level_header_in_score_def_label() {
    let file = parse("\\score { { c4 } \\header { piece = \"Intro\" } }");
    let mei = import(&file).unwrap();
    let label = score_def_label(&mei);
    assert!(
        label.contains("tusk:score-output-defs,"),
        "score-output-defs label missing, got: {label}"
    );
    assert!(
        label.contains("Header"),
        "should contain Header kind, got: {label}"
    );
}

#[test]
fn score_level_layout_in_score_def_label() {
    let file = parse("\\score { { c4 } \\layout { ragged-right = ##t } }");
    let mei = import(&file).unwrap();
    let label = score_def_label(&mei);
    assert!(
        label.contains("tusk:score-output-defs,"),
        "score-output-defs label missing, got: {label}"
    );
    assert!(
        label.contains("Layout"),
        "should contain Layout kind, got: {label}"
    );
}

#[test]
fn score_level_midi_in_score_def_label() {
    let file = parse("\\score { { c4 } \\midi { } }");
    let mei = import(&file).unwrap();
    let label = score_def_label(&mei);
    assert!(
        label.contains("tusk:score-output-defs,"),
        "score-output-defs label missing, got: {label}"
    );
    assert!(
        label.contains("Midi"),
        "should contain Midi kind, got: {label}"
    );
}

#[test]
fn no_header_gives_empty_title_stmt() {
    let file = parse("\\score { { c4 } }");
    let mei = import(&file).unwrap();

    for child in &mei.children {
        if let MeiChild::MeiHead(head) = child {
            for hc in &head.children {
                if let MeiHeadChild::FileDesc(fd) = hc {
                    for fc in &fd.children {
                        if let tusk_model::elements::FileDescChild::TitleStmt(ts) = fc {
                            assert!(ts.children.is_empty());
                            return;
                        }
                    }
                }
            }
        }
    }
    panic!("fileDesc/titleStmt not found");
}

#[test]
fn layout_with_context_stored() {
    let src = "\\layout {\n  \\context {\n    \\Score\n    \\remove \"Bar_number_engraver\"\n  }\n}\n\\score { { c4 } }";
    let file = parse(src);
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    let output_defs_label = labels
        .iter()
        .find(|l| l.starts_with("tusk:output-defs,"))
        .expect("output-defs ExtMeta not found");
    // JSON should contain Layout kind with context block info
    assert!(
        output_defs_label.contains("Layout"),
        "should contain Layout, got: {output_defs_label}"
    );
    assert!(
        output_defs_label.contains("Score") || output_defs_label.contains("context"),
        "should contain context info, got: {output_defs_label}"
    );
}

#[test]
fn labels_use_tusk_prefix() {
    let file = parse(
        "\\header { title = \"Test\" }\n\\paper { indent = 0 }\n\\layout { }\n\\midi { }\n\\score { { c4 } \\header { piece = \"P\" } \\layout { } \\midi { } }",
    );
    let mei = import(&file).unwrap();

    // ExtMeta labels must use tusk: prefix
    let labels = ext_meta_labels(&mei);
    assert!(!labels.is_empty(), "expected at least one ExtMeta label");
    for label in &labels {
        assert!(
            label.starts_with("tusk:"),
            "expected tusk: prefix, got: {label}"
        );
    }

    // ScoreDef label segments must use tusk: prefix
    let sd_label = score_def_label(&mei);
    for segment in sd_label.split('|') {
        if !segment.is_empty() {
            assert!(
                segment.starts_with("tusk:"),
                "expected tusk: prefix in ScoreDef label segment, got: {segment}"
            );
        }
    }
}

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
fn header_stored_as_ext_meta() {
    let file = parse("\\header { title = \"Test\" composer = \"Bach\" }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    let header_label = labels
        .iter()
        .find(|l| l.starts_with("lilypond:header,"))
        .expect("header ExtMeta not found");
    assert!(header_label.contains("title"));
    assert!(header_label.contains("composer"));
}

#[test]
fn header_ext_meta_has_summary_text() {
    let file = parse("\\header { title = \"Test\" composer = \"Bach\" }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    for child in &mei.children {
        if let MeiChild::MeiHead(head) = child {
            for hc in &head.children {
                if let MeiHeadChild::ExtMeta(ext) = hc
                    && ext
                        .common
                        .label
                        .as_deref()
                        .is_some_and(|l| l.starts_with("lilypond:header,"))
                {
                    assert!(!ext.children.is_empty());
                    let tusk_model::elements::ExtMetaChild::Text(t) = &ext.children[0];
                    assert!(t.contains("title: Test"));
                    assert!(t.contains("composer: Bach"));
                    return;
                }
            }
        }
    }
    panic!("header ExtMeta not found");
}

#[test]
fn paper_stored_as_ext_meta() {
    let file = parse("\\paper { indent = 0 }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    let paper = labels
        .iter()
        .find(|l| l.starts_with("lilypond:paper,"))
        .expect("paper ExtMeta not found");
    assert!(paper.contains("indent"));
}

#[test]
fn layout_stored_as_ext_meta() {
    let file = parse("\\layout { ragged-right = ##t }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    assert!(
        labels.iter().any(|l| l.starts_with("lilypond:layout,")),
        "layout ExtMeta not found"
    );
}

#[test]
fn midi_stored_as_ext_meta() {
    let file = parse("\\midi { }\n\\score { { c4 } }");
    let mei = import(&file).unwrap();

    let labels = ext_meta_labels(&mei);
    assert!(
        labels.iter().any(|l| l.starts_with("lilypond:midi,")),
        "midi ExtMeta not found"
    );
}

#[test]
fn score_level_header_in_score_def_label() {
    let file = parse("\\score { { c4 } \\header { piece = \"Intro\" } }");
    let mei = import(&file).unwrap();
    let label = score_def_label(&mei);
    assert!(
        label.contains("lilypond:score-header,"),
        "score-header label missing, got: {label}"
    );
}

#[test]
fn score_level_layout_in_score_def_label() {
    let file = parse("\\score { { c4 } \\layout { ragged-right = ##t } }");
    let mei = import(&file).unwrap();
    let label = score_def_label(&mei);
    assert!(
        label.contains("lilypond:score-layout,"),
        "score-layout label missing, got: {label}"
    );
}

#[test]
fn score_level_midi_in_score_def_label() {
    let file = parse("\\score { { c4 } \\midi { } }");
    let mei = import(&file).unwrap();
    let label = score_def_label(&mei);
    assert!(
        label.contains("lilypond:score-midi,"),
        "score-midi label missing, got: {label}"
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
    let layout_label = labels
        .iter()
        .find(|l| l.starts_with("lilypond:layout,"))
        .expect("layout ExtMeta not found");
    assert!(
        layout_label.contains("context") || layout_label.contains("Score"),
        "layout label should contain context info, got: {layout_label}"
    );
}

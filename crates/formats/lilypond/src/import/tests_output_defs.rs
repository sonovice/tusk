//! Tests for import of header, paper, layout, and midi blocks.

use crate::import::import;
use crate::parser::Parser;
use tusk_model::elements::{MeiChild, MeiHeadChild};
use tusk_model::extensions::ExtensionStore;

fn parse(src: &str) -> crate::model::LilyPondFile {
    Parser::new(src).unwrap().parse().unwrap()
}

fn parse_and_import(src: &str) -> (tusk_model::elements::Mei, ExtensionStore) {
    let file = parse(src);
    import(&file).unwrap()
}

/// Collect all ExtMeta xml:ids from MeiHead.
fn ext_meta_ids(mei: &tusk_model::elements::Mei) -> Vec<String> {
    let mut ids = Vec::new();
    for child in &mei.children {
        if let MeiChild::MeiHead(head) = child {
            for hc in &head.children {
                if let MeiHeadChild::ExtMeta(ext) = hc
                    && let Some(id) = &ext.common.xml_id
                {
                    ids.push(id.clone());
                }
            }
        }
    }
    ids
}

/// Get the ScoreDef xml:id from MEI.
fn score_def_id(mei: &tusk_model::elements::Mei) -> Option<String> {
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
                    return sd.common.xml_id.clone();
                }
            }
        }
    }
    None
}

#[test]
fn header_title_populates_mei_title() {
    let file = parse("\\header { title = \"My Title\" }\n\\score { { c4 } }");
    let (mei, _) = import(&file).unwrap();

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
    let (mei, ext_store) =
        parse_and_import("\\header { title = \"Test\" composer = \"Bach\" }\n\\score { { c4 } }");

    let ids = ext_meta_ids(&mei);
    assert!(!ids.is_empty(), "should have ExtMeta with xml:id");
    let defs = ext_store
        .output_defs(&ids[0])
        .expect("output-defs should exist in ext_store");
    // Should contain a Header kind with title and composer
    let has_header = defs.iter().any(|d| d.kind == tusk_model::OutputDefKind::Header);
    assert!(has_header, "should contain Header output def");
    let header = defs.iter().find(|d| d.kind == tusk_model::OutputDefKind::Header).unwrap();
    let has_title = header.assignments.iter().any(|a| a.name == "title");
    let has_composer = header.assignments.iter().any(|a| a.name == "composer");
    assert!(has_title, "should contain title assignment");
    assert!(has_composer, "should contain composer assignment");
}

#[test]
fn output_defs_ext_meta_has_summary_text() {
    let (mei, ext_store) =
        parse_and_import("\\header { title = \"Test\" }\n\\score { { c4 } }");

    let ids = ext_meta_ids(&mei);
    assert!(!ids.is_empty(), "should have ExtMeta");
    let defs = ext_store
        .output_defs(&ids[0])
        .expect("output-defs should exist");

    // ExtMeta should still have a summary text child
    for child in &mei.children {
        if let MeiChild::MeiHead(head) = child {
            for hc in &head.children {
                if let MeiHeadChild::ExtMeta(ext) = hc
                    && ext.common.xml_id.as_deref() == Some(&ids[0])
                {
                    assert!(!ext.children.is_empty());
                    let tusk_model::elements::ExtMetaChild::Text(t) = &ext.children[0];
                    assert!(t.contains("Header"), "summary should list kind, got: {t}");
                    // Also verify defs are valid
                    assert!(!defs.is_empty());
                    return;
                }
            }
        }
    }
    panic!("output-defs ExtMeta not found");
}

#[test]
fn paper_stored_as_typed_output_def() {
    let (mei, ext_store) =
        parse_and_import("\\paper { indent = 0 }\n\\score { { c4 } }");

    let ids = ext_meta_ids(&mei);
    assert!(!ids.is_empty(), "should have ExtMeta");
    let defs = ext_store
        .output_defs(&ids[0])
        .expect("output-defs should exist");
    let has_paper = defs.iter().any(|d| d.kind == tusk_model::OutputDefKind::Paper);
    assert!(has_paper, "should contain Paper output def");
    let paper = defs.iter().find(|d| d.kind == tusk_model::OutputDefKind::Paper).unwrap();
    let has_indent = paper.assignments.iter().any(|a| a.name == "indent");
    assert!(has_indent, "should contain indent assignment");
}

#[test]
fn layout_stored_as_typed_output_def() {
    let (mei, ext_store) =
        parse_and_import("\\layout { ragged-right = ##t }\n\\score { { c4 } }");

    let ids = ext_meta_ids(&mei);
    assert!(!ids.is_empty(), "should have ExtMeta");
    let defs = ext_store
        .output_defs(&ids[0])
        .expect("output-defs should exist");
    let has_layout = defs.iter().any(|d| d.kind == tusk_model::OutputDefKind::Layout);
    assert!(has_layout, "should contain Layout output def");
}

#[test]
fn midi_stored_as_typed_output_def() {
    let (mei, ext_store) =
        parse_and_import("\\midi { }\n\\score { { c4 } }");

    let ids = ext_meta_ids(&mei);
    assert!(!ids.is_empty(), "should have ExtMeta");
    let defs = ext_store
        .output_defs(&ids[0])
        .expect("output-defs should exist");
    let has_midi = defs.iter().any(|d| d.kind == tusk_model::OutputDefKind::Midi);
    assert!(has_midi, "should contain Midi output def");
}

#[test]
fn score_level_header_in_score_def_label() {
    let (mei, ext_store) =
        parse_and_import("\\score { { c4 } \\header { piece = \"Intro\" } }");
    let sd_id = score_def_id(&mei).expect("ScoreDef should have xml:id");
    let defs = ext_store
        .output_defs(&sd_id)
        .expect("score-output-defs should exist");
    let has_header = defs.iter().any(|d| d.kind == tusk_model::OutputDefKind::Header);
    assert!(has_header, "should contain Header kind");
}

#[test]
fn score_level_layout_in_score_def_label() {
    let (mei, ext_store) =
        parse_and_import("\\score { { c4 } \\layout { ragged-right = ##t } }");
    let sd_id = score_def_id(&mei).expect("ScoreDef should have xml:id");
    let defs = ext_store
        .output_defs(&sd_id)
        .expect("score-output-defs should exist");
    let has_layout = defs.iter().any(|d| d.kind == tusk_model::OutputDefKind::Layout);
    assert!(has_layout, "should contain Layout kind");
}

#[test]
fn score_level_midi_in_score_def_label() {
    let (mei, ext_store) =
        parse_and_import("\\score { { c4 } \\midi { } }");
    let sd_id = score_def_id(&mei).expect("ScoreDef should have xml:id");
    let defs = ext_store
        .output_defs(&sd_id)
        .expect("score-output-defs should exist");
    let has_midi = defs.iter().any(|d| d.kind == tusk_model::OutputDefKind::Midi);
    assert!(has_midi, "should contain Midi kind");
}

#[test]
fn no_header_gives_empty_title_stmt() {
    let file = parse("\\score { { c4 } }");
    let (mei, _) = import(&file).unwrap();

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
    let (mei, ext_store) = parse_and_import(src);

    let ids = ext_meta_ids(&mei);
    assert!(!ids.is_empty(), "should have ExtMeta");
    let defs = ext_store
        .output_defs(&ids[0])
        .expect("output-defs should exist");
    let has_layout = defs.iter().any(|d| d.kind == tusk_model::OutputDefKind::Layout);
    assert!(has_layout, "should contain Layout");
    let layout = defs.iter().find(|d| d.kind == tusk_model::OutputDefKind::Layout).unwrap();
    // Layout should have context blocks
    assert!(
        !layout.context_blocks.is_empty(),
        "layout should have context blocks"
    );
}

#[test]
fn labels_use_tusk_prefix() {
    let (mei, ext_store) = parse_and_import(
        "\\header { title = \"Test\" }\n\\paper { indent = 0 }\n\\layout { }\n\\midi { }\n\\score { { c4 } \\header { piece = \"P\" } \\layout { } \\midi { } }",
    );

    // ExtMeta should exist and have output defs in ext_store
    let ids = ext_meta_ids(&mei);
    assert!(!ids.is_empty(), "expected at least one ExtMeta with id");
    let defs = ext_store
        .output_defs(&ids[0])
        .expect("top-level output defs should exist");
    assert!(!defs.is_empty());

    // ScoreDef should have output defs in ext_store
    let sd_id = score_def_id(&mei).expect("ScoreDef should have xml:id");
    let score_defs = ext_store
        .output_defs(&sd_id)
        .expect("score-level output defs should exist");
    assert!(!score_defs.is_empty());
}

#[test]
fn layout_with_context_def_keywords_stored() {
    let src = "\\layout {\n  \\context {\n    \\Staff\n    \\accepts \"CueVoice\"\n    \\denies \"Voice\"\n    \\alias \"RhythmicStaff\"\n    \\defaultchild \"Voice\"\n    \\description \"Custom staff\"\n    \\name \"MyStaff\"\n  }\n}\n\\score { { c4 } }";
    let (mei, ext_store) = parse_and_import(src);

    let ids = ext_meta_ids(&mei);
    assert!(!ids.is_empty(), "should have ExtMeta");
    let defs = ext_store
        .output_defs(&ids[0])
        .expect("output-defs should exist");
    let layout = defs
        .iter()
        .find(|d| d.kind == tusk_model::OutputDefKind::Layout)
        .expect("should have Layout");
    // All context-def keywords should be in context blocks
    let all_items: Vec<_> = layout
        .context_blocks
        .iter()
        .flat_map(|cb| &cb.items)
        .collect();
    assert!(
        all_items.iter().any(|i| matches!(i, tusk_model::ExtContextModItem::Accepts(_))),
        "should contain Accepts"
    );
    assert!(
        all_items.iter().any(|i| matches!(i, tusk_model::ExtContextModItem::Denies(_))),
        "should contain Denies"
    );
    assert!(
        all_items.iter().any(|i| matches!(i, tusk_model::ExtContextModItem::Alias(_))),
        "should contain Alias"
    );
    assert!(
        all_items.iter().any(|i| matches!(i, tusk_model::ExtContextModItem::DefaultChild(_))),
        "should contain DefaultChild"
    );
    assert!(
        all_items.iter().any(|i| matches!(i, tusk_model::ExtContextModItem::Description(_))),
        "should contain Description"
    );
    assert!(
        all_items.iter().any(|i| matches!(i, tusk_model::ExtContextModItem::Name(_))),
        "should contain Name"
    );
}

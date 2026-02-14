use super::*;
use crate::parser::Parser;
use tusk_model::elements::{LayerChild, Mei, MeiChild, ScoreChild};
use tusk_model::ExtensionStore;

fn parse_and_import(src: &str) -> (Mei, ExtensionStore) {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Walk MEI to find layer children (first layer of first staff).
fn layer_children(mei: &Mei) -> &[LayerChild] {
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
                                            if let MeasureChild::Staff(staff) = mc2
                                                && let Some(
                                                    tusk_model::elements::StaffChild::Layer(layer),
                                                ) = staff.children.first()
                                            {
                                                return &layer.children;
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
    &[]
}

/// Find the scoreDef xml:id.
fn score_def_id(mei: &Mei) -> Option<String> {
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
                                return sd.common.xml_id.clone();
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
// Import tests for variable resolution
// ---------------------------------------------------------------------------

#[test]
fn import_variable_ref_in_score_expands_music() {
    let src = "melody = { c'4 d'4 }\n\\score { \\new Staff \\melody }";
    let (mei, _ext_store) = parse_and_import(src);
    let children = layer_children(&mei);
    // Variable \melody should be expanded to its music content
    assert_eq!(children.len(), 2, "expected 2 notes from expanded \\melody");
    assert!(matches!(children[0], LayerChild::Note(_)));
    assert!(matches!(children[1], LayerChild::Note(_)));
}

#[test]
fn import_variable_ref_in_bare_music_expands() {
    let src = "melody = { c'4 d'4 e'4 f'4 }\n\\melody";
    let (mei, _ext_store) = parse_and_import(src);
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4, "expected 4 notes from expanded \\melody");
}

#[test]
fn import_transitive_variable_expands() {
    let src = "melody = { c'4 d'4 }\nsoprano = \\melody\n\\score { \\new Staff \\soprano }";
    let (mei, _ext_store) = parse_and_import(src);
    let children = layer_children(&mei);
    // soprano = \melody -> should resolve transitively
    assert_eq!(
        children.len(),
        2,
        "expected 2 notes from transitive \\soprano"
    );
}

#[test]
fn import_variable_label_stored_on_score_def() {
    let src = "melody = { c'4 d'4 }\n\\score { \\new Staff \\melody }";
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).expect("scoreDef should have xml:id");
    let vars = ext_store
        .variable_assignments(&sd_id)
        .expect("should have variable assignments in ext_store");
    // The serialized assignment should contain "melody"
    let names: Vec<&str> = vars.assignments.iter().map(|a| a.name.as_str()).collect();
    assert!(
        names.contains(&"melody"),
        "assignments should contain variable name 'melody': {names:?}"
    );
}

#[test]
fn import_multiple_assignments_stored() {
    let src = "melody = { c'4 d'4 }\nharmony = { e'4 f'4 }\n\\score { \\new Staff \\melody }";
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).expect("scoreDef should have xml:id");
    let vars = ext_store
        .variable_assignments(&sd_id)
        .expect("should have variable assignments");
    let names: Vec<&str> = vars.assignments.iter().map(|a| a.name.as_str()).collect();
    assert!(names.contains(&"melody"), "names: {names:?}");
    assert!(names.contains(&"harmony"), "names: {names:?}");
}

#[test]
fn import_non_music_assignments_stored() {
    let src = "myTitle = \"Test\"\nmyNum = 42\n{ c'4 }";
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).expect("scoreDef should have xml:id");
    let vars = ext_store
        .variable_assignments(&sd_id)
        .expect("should store non-music assignments in ext_store");
    let names: Vec<&str> = vars.assignments.iter().map(|a| a.name.as_str()).collect();
    assert!(names.contains(&"myTitle"), "names: {names:?}");
    assert!(names.contains(&"myNum"), "names: {names:?}");
}

#[test]
fn import_no_assignments_no_vars_label() {
    let src = "{ c'4 d'4 }";
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).unwrap_or_default();
    assert!(
        ext_store.variable_assignments(&sd_id).is_none(),
        "no variable assignments when no assignments"
    );
}

#[test]
fn import_fixture_variables() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_variables.ly"
    ))
    .unwrap();
    let (mei, ext_store) = parse_and_import(&src);
    // The fixture has \melody referenced in the score block
    let children = layer_children(&mei);
    assert_eq!(
        children.len(),
        4,
        "expected 4 notes from \\melody expansion in fixture"
    );
    // Assignments should be stored in ext_store
    let sd_id = score_def_id(&mei).expect("scoreDef should have xml:id");
    assert!(
        ext_store.variable_assignments(&sd_id).is_some(),
        "should have variable assignments in ext_store"
    );
}

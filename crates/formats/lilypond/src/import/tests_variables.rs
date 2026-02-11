use super::*;
use crate::parser::Parser;
use tusk_model::elements::{LayerChild, Mei, MeiChild, ScoreChild};

fn parse_and_import(src: &str) -> Mei {
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

/// Find the scoreDef label.
fn score_def_label(mei: &Mei) -> Option<String> {
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
                                return sd.common.label.clone();
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
    let mei = parse_and_import(src);
    let children = layer_children(&mei);
    // Variable \melody should be expanded to its music content
    assert_eq!(children.len(), 2, "expected 2 notes from expanded \\melody");
    assert!(matches!(children[0], LayerChild::Note(_)));
    assert!(matches!(children[1], LayerChild::Note(_)));
}

#[test]
fn import_variable_ref_in_bare_music_expands() {
    let src = "melody = { c'4 d'4 e'4 f'4 }\n\\melody";
    let mei = parse_and_import(src);
    let children = layer_children(&mei);
    assert_eq!(children.len(), 4, "expected 4 notes from expanded \\melody");
}

#[test]
fn import_transitive_variable_expands() {
    let src = "melody = { c'4 d'4 }\nsoprano = \\melody\n\\score { \\new Staff \\soprano }";
    let mei = parse_and_import(src);
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
    let mei = parse_and_import(src);
    let label = score_def_label(&mei).unwrap_or_default();
    assert!(
        label.contains("tusk:vars,"),
        "scoreDef label should contain lilypond:vars: {label}"
    );
    // The serialized assignment should be present (escaped)
    assert!(
        label.contains("melody"),
        "label should contain variable name: {label}"
    );
}

#[test]
fn import_multiple_assignments_stored() {
    let src = "melody = { c'4 d'4 }\nharmony = { e'4 f'4 }\n\\score { \\new Staff \\melody }";
    let mei = parse_and_import(src);
    let label = score_def_label(&mei).unwrap_or_default();
    assert!(label.contains("melody"), "label: {label}");
    assert!(label.contains("harmony"), "label: {label}");
}

#[test]
fn import_non_music_assignments_stored() {
    let src = "myTitle = \"Test\"\nmyNum = 42\n{ c'4 }";
    let mei = parse_and_import(src);
    let label = score_def_label(&mei).unwrap_or_default();
    assert!(
        label.contains("tusk:vars,"),
        "should store non-music assignments: {label}"
    );
    assert!(label.contains("myTitle"), "label: {label}");
    assert!(label.contains("myNum"), "label: {label}");
}

#[test]
fn import_no_assignments_no_vars_label() {
    let src = "{ c'4 d'4 }";
    let mei = parse_and_import(src);
    let label = score_def_label(&mei).unwrap_or_default();
    assert!(
        !label.contains("tusk:vars,"),
        "no vars label when no assignments: {label}"
    );
}

#[test]
fn import_fixture_variables() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_variables.ly"
    ))
    .unwrap();
    let mei = parse_and_import(&src);
    // The fixture has \melody referenced in the score block
    let children = layer_children(&mei);
    assert_eq!(
        children.len(),
        4,
        "expected 4 notes from \\melody expansion in fixture"
    );
    // Assignments should be stored
    let label = score_def_label(&mei).unwrap_or_default();
    assert!(label.contains("tusk:vars,"), "label: {label}");
}

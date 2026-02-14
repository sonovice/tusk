use super::*;
use crate::parser::Parser;
use tusk_model::elements::{Mei, MeiChild, ScoreChild};
use tusk_model::ExtensionStore;

fn parse_and_import(src: &str) -> (Mei, ExtensionStore) {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
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
// Top-level markup import tests
// ---------------------------------------------------------------------------

#[test]
fn import_toplevel_markup_stored() {
    let src = r#"\markup { "Title Page" }
\score { \new Staff { c'4 } }"#;
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).expect("scoreDef should have xml:id");
    let markups = ext_store
        .toplevel_markups(&sd_id)
        .expect("should have toplevel markups in ext_store");
    assert_eq!(markups.len(), 1);
    assert_eq!(markups[0].position, 0);
    assert!(matches!(markups[0].kind, ToplevelMarkupKind::Markup(_)));
    if let ToplevelMarkupKind::Markup(ref s) = markups[0].kind {
        assert!(s.contains("Title Page"), "serialized markup: {s}");
    }
}

#[test]
fn import_toplevel_markuplist_stored() {
    let src = r#"\markuplist { "First" "Second" }
\score { \new Staff { c'4 } }"#;
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).expect("scoreDef should have xml:id");
    let markups = ext_store
        .toplevel_markups(&sd_id)
        .expect("should have toplevel markups in ext_store");
    assert_eq!(markups.len(), 1);
    assert_eq!(markups[0].position, 0);
    assert!(
        matches!(markups[0].kind, ToplevelMarkupKind::MarkupList(_)),
        "expected MarkupList variant"
    );
}

#[test]
fn import_toplevel_markup_ordering_preserved() {
    let src = r#"\markup { "Before" }
\score { \new Staff { c'4 } }
\markup { "After" }"#;
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).expect("scoreDef should have xml:id");
    let markups = ext_store
        .toplevel_markups(&sd_id)
        .expect("should have toplevel markups in ext_store");
    assert_eq!(markups.len(), 2, "two top-level markups");
    // First markup at position 0 (before score)
    assert_eq!(markups[0].position, 0);
    // Second markup at position 2 (after score at position 1)
    assert_eq!(markups[1].position, 2);
}

#[test]
fn import_mixed_markup_and_markuplist() {
    let src = r#"\markup { "Title" }
\markuplist { "A" "B" }
\score { \new Staff { c'4 } }"#;
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).expect("scoreDef should have xml:id");
    let markups = ext_store
        .toplevel_markups(&sd_id)
        .expect("should have toplevel markups in ext_store");
    assert_eq!(markups.len(), 2);
    assert!(matches!(markups[0].kind, ToplevelMarkupKind::Markup(_)));
    assert!(matches!(markups[1].kind, ToplevelMarkupKind::MarkupList(_)));
    assert_eq!(markups[0].position, 0);
    assert_eq!(markups[1].position, 1);
}

#[test]
fn import_no_toplevel_markup_no_label_segment() {
    let src = r#"\score { \new Staff { c'4 } }"#;
    let (mei, ext_store) = parse_and_import(src);
    let sd_id = score_def_id(&mei).unwrap_or_default();
    // No toplevel markups in ext_store
    assert!(
        ext_store.toplevel_markups(&sd_id).is_none(),
        "should not have toplevel markups in ext_store"
    );
}

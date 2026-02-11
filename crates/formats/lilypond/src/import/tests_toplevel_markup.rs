use super::*;
use crate::parser::Parser;
use tusk_model::elements::{Mei, MeiChild, ScoreChild};

fn parse_and_import(src: &str) -> Mei {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
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

/// Extract the `tusk:toplevel-markup,{json}` segment from a label.
fn extract_toplevel_markup_json(label: &str) -> Option<&str> {
    for segment in label.split('|') {
        if let Some(json) = segment.strip_prefix("tusk:toplevel-markup,") {
            return Some(json);
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
    let mei = parse_and_import(src);
    let label = score_def_label(&mei).expect("scoreDef should have label");
    let json = extract_toplevel_markup_json(&label).expect("should have toplevel-markup segment");
    let markups: Vec<ToplevelMarkup> = serde_json::from_str(json).unwrap();
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
    let mei = parse_and_import(src);
    let label = score_def_label(&mei).expect("scoreDef should have label");
    let json = extract_toplevel_markup_json(&label).expect("should have toplevel-markup segment");
    let markups: Vec<ToplevelMarkup> = serde_json::from_str(json).unwrap();
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
    let mei = parse_and_import(src);
    let label = score_def_label(&mei).expect("scoreDef should have label");
    let json = extract_toplevel_markup_json(&label).expect("should have toplevel-markup segment");
    let markups: Vec<ToplevelMarkup> = serde_json::from_str(json).unwrap();
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
    let mei = parse_and_import(src);
    let label = score_def_label(&mei).expect("scoreDef should have label");
    let json = extract_toplevel_markup_json(&label).expect("should have toplevel-markup segment");
    let markups: Vec<ToplevelMarkup> = serde_json::from_str(json).unwrap();
    assert_eq!(markups.len(), 2);
    assert!(matches!(markups[0].kind, ToplevelMarkupKind::Markup(_)));
    assert!(matches!(markups[1].kind, ToplevelMarkupKind::MarkupList(_)));
    assert_eq!(markups[0].position, 0);
    assert_eq!(markups[1].position, 1);
}

#[test]
fn import_no_toplevel_markup_no_label_segment() {
    let src = r#"\score { \new Staff { c'4 } }"#;
    let mei = parse_and_import(src);
    let label = score_def_label(&mei);
    // Either no label at all, or no toplevel-markup segment
    if let Some(ref l) = label {
        assert!(
            extract_toplevel_markup_json(l).is_none(),
            "should not have toplevel-markup segment"
        );
    }
}

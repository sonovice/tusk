//! Tests for book/bookpart import.

use super::*;
use crate::parser::Parser;
use tusk_model::elements::Mei;
use tusk_model::ExtensionStore;

fn parse_and_import(src: &str) -> (Mei, ExtensionStore) {
    let file = Parser::new(src).unwrap().parse().unwrap();
    import(&file).unwrap()
}

/// Helper: collect all mdiv elements from MEI.
fn all_mdivs(mei: &Mei) -> Vec<&tusk_model::elements::Mdiv> {
    let mut mdivs = Vec::new();
    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let tusk_model::elements::BodyChild::Mdiv(mdiv) = bc;
                    mdivs.push(mdiv.as_ref());
                }
            }
        }
    }
    mdivs
}

/// Helper: extract BookStructure from ext_store via mdiv xml:id.
fn mdiv_book_structure(mdiv: &tusk_model::elements::Mdiv, ext_store: &ExtensionStore) -> Option<tusk_model::BookStructure> {
    let id = mdiv.common.xml_id.as_deref()?;
    ext_store.book_structure(id).cloned()
}

// -------------------------------------------------------------------------
// Book with single score
// -------------------------------------------------------------------------

#[test]
fn book_single_score_creates_mdiv_with_structure() {
    let (mei, ext_store) = parse_and_import(
        r#"\version "2.24.0"
\book {
  \score { { c4 d e f } }
}"#,
    );
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 1);

    let bs = mdiv_book_structure(mdivs[0], &ext_store).expect("book structure should exist");
    assert_eq!(bs.book_index, Some(0));
    assert_eq!(bs.bookpart_index, None);
    assert_eq!(bs.score_index, Some(0));
}

// -------------------------------------------------------------------------
// Book with header and paper
// -------------------------------------------------------------------------

#[test]
fn book_with_header_paper_stores_output_defs() {
    let (mei, ext_store) = parse_and_import(
        r#"\version "2.24.0"
\book {
  \header { title = "My Book" }
  \paper { indent = 0 }
  \score { { c4 d e f } }
}"#,
    );
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 1);

    let bs = mdiv_book_structure(mdivs[0], &ext_store).expect("book structure");
    assert_eq!(bs.book_output_defs.len(), 2);
    assert_eq!(
        bs.book_output_defs[0].kind,
        tusk_model::OutputDefKind::Header
    );
    assert_eq!(
        bs.book_output_defs[1].kind,
        tusk_model::OutputDefKind::Paper
    );
}

// -------------------------------------------------------------------------
// Bookpart with score
// -------------------------------------------------------------------------

#[test]
fn bookpart_in_book_creates_mdiv() {
    let (mei, ext_store) = parse_and_import(
        r#"\version "2.24.0"
\book {
  \bookpart {
    \score { { c4 d e f } }
  }
}"#,
    );
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 1);

    let bs = mdiv_book_structure(mdivs[0], &ext_store).expect("book structure");
    assert_eq!(bs.book_index, Some(0));
    assert_eq!(bs.bookpart_index, Some(0));
    assert_eq!(bs.score_index, Some(0));
}

// -------------------------------------------------------------------------
// Bookpart with header and paper
// -------------------------------------------------------------------------

#[test]
fn bookpart_header_paper_stored_in_structure() {
    let (mei, ext_store) = parse_and_import(
        r#"\version "2.24.0"
\book {
  \bookpart {
    \header { title = "Part 1" }
    \paper { indent = 10 }
    \score { { c4 d e f } }
  }
}"#,
    );
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 1);

    let bs = mdiv_book_structure(mdivs[0], &ext_store).expect("book structure");
    assert_eq!(bs.bookpart_output_defs.len(), 2);
    assert_eq!(
        bs.bookpart_output_defs[0].kind,
        tusk_model::OutputDefKind::Header
    );
    assert_eq!(
        bs.bookpart_output_defs[1].kind,
        tusk_model::OutputDefKind::Paper
    );
}

// -------------------------------------------------------------------------
// Nested book > bookpart > score
// -------------------------------------------------------------------------

#[test]
fn nested_book_bookpart_score_hierarchy() {
    let (mei, ext_store) = parse_and_import(
        r#"\version "2.24.0"
\book {
  \header { title = "Book Title" }
  \bookpart {
    \header { title = "Part 1" }
    \score { { c4 d e f } }
  }
  \bookpart {
    \header { title = "Part 2" }
    \score { { g4 a b c' } }
  }
}"#,
    );
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 2);

    // First bookpart score
    let bs0 = mdiv_book_structure(mdivs[0], &ext_store).expect("bs0");
    assert_eq!(bs0.book_index, Some(0));
    assert_eq!(bs0.bookpart_index, Some(0));
    assert_eq!(bs0.score_index, Some(0));
    assert_eq!(bs0.book_output_defs.len(), 1); // book header
    assert_eq!(bs0.bookpart_output_defs.len(), 1); // bookpart 1 header

    // Second bookpart score
    let bs1 = mdiv_book_structure(mdivs[1], &ext_store).expect("bs1");
    assert_eq!(bs1.book_index, Some(0));
    assert_eq!(bs1.bookpart_index, Some(1));
    assert_eq!(bs1.score_index, Some(0));
    assert_eq!(bs1.book_output_defs.len(), 1);
    assert_eq!(bs1.bookpart_output_defs.len(), 1);
}

// -------------------------------------------------------------------------
// Multiple bookparts
// -------------------------------------------------------------------------

#[test]
fn multiple_bookparts_create_separate_mdivs() {
    let (mei, ext_store) = parse_and_import(
        r#"\version "2.24.0"
\book {
  \bookpart { \score { { c4 } } }
  \bookpart { \score { { d4 } } }
  \bookpart { \score { { e4 } } }
}"#,
    );
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 3);

    // Check @n numbering
    assert_eq!(mdivs[0].common.n.as_ref().unwrap().0, "1");
    assert_eq!(mdivs[1].common.n.as_ref().unwrap().0, "2");
    assert_eq!(mdivs[2].common.n.as_ref().unwrap().0, "3");

    // Check hierarchy
    for (i, mdiv) in mdivs.iter().enumerate() {
        let bs = mdiv_book_structure(mdiv, &ext_store).expect("book structure");
        assert_eq!(bs.bookpart_index, Some(i));
    }
}

// -------------------------------------------------------------------------
// Book with direct scores (no bookparts)
// -------------------------------------------------------------------------

#[test]
fn book_direct_scores_without_bookparts() {
    let (mei, ext_store) = parse_and_import(
        r#"\version "2.24.0"
\book {
  \score { { c4 d e f } }
  \score { { g4 a b c' } }
}"#,
    );
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 2);

    let bs0 = mdiv_book_structure(mdivs[0], &ext_store).expect("bs0");
    assert_eq!(bs0.book_index, Some(0));
    assert_eq!(bs0.bookpart_index, None);
    assert_eq!(bs0.score_index, Some(0));

    let bs1 = mdiv_book_structure(mdivs[1], &ext_store).expect("bs1");
    assert_eq!(bs1.book_index, Some(0));
    assert_eq!(bs1.bookpart_index, None);
    assert_eq!(bs1.score_index, Some(1));
}

// -------------------------------------------------------------------------
// Non-book file still works (backward compatibility)
// -------------------------------------------------------------------------

#[test]
fn non_book_single_score_no_book_structure() {
    let (mei, ext_store) = parse_and_import(
        r#"\version "2.24.0"
\score { { c4 d e f } }"#,
    );
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 1);

    // No book structure on mdiv
    assert!(mdiv_book_structure(mdivs[0], &ext_store).is_none());
}

#[test]
fn bare_music_no_book_structure() {
    let (mei, ext_store) = parse_and_import("{ c4 d e f }");
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 1);
    assert!(mdiv_book_structure(mdivs[0], &ext_store).is_none());
}

// -------------------------------------------------------------------------
// Music content preserved inside book scores
// -------------------------------------------------------------------------

#[test]
fn book_score_music_content_preserved() {
    let (mei, _ext_store) = parse_and_import(
        r#"\version "2.24.0"
\book {
  \score { { c4 d e f } }
}"#,
    );
    // Should have notes
    let mdivs = all_mdivs(&mei);
    assert_eq!(mdivs.len(), 1);
    // Walk into score → section → measure → staff → layer → notes
    let mdiv = mdivs[0];
    let MdivChild::Score(score) = &mdiv.children[0];
    let section = score.children.iter().find_map(|c| {
        if let ScoreChild::Section(s) = c {
            Some(s)
        } else {
            None
        }
    });
    assert!(section.is_some(), "score should have section");
}

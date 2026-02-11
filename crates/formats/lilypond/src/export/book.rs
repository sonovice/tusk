//! Book/bookpart reconstruction from MEI multi-mdiv structure.
//!
//! When the import path stores `BookStructure` extensions on `<mdiv>` elements,
//! this module reads them back and reconstructs `ToplevelExpression::Book` /
//! `BookPart` wrappers around scores.

use tusk_model::elements::{BodyChild, Mei, MeiChild};
use tusk_model::extensions::{BookStructure, OutputDefKind};

use crate::import::output_def_conv;
use crate::import::signatures::unescape_label_value;
use crate::model::{BookBlock, BookItem, BookPartBlock, BookPartItem, ToplevelExpression};

/// A single score entry extracted from an mdiv with its book structure metadata.
pub(super) struct MdivEntry<'a> {
    pub score: &'a tusk_model::elements::Score,
    pub book_structure: BookStructure,
}

/// Find all mdivs that have `tusk:book-structure` labels.
///
/// Returns `None` if there are no mdivs or none have book structure labels.
pub(super) fn find_book_entries(mei: &Mei) -> Option<Vec<MdivEntry<'_>>> {
    let mut entries = Vec::new();

    for child in &mei.children {
        if let MeiChild::Music(music) = child {
            for mc in &music.children {
                let tusk_model::elements::MusicChild::Body(body) = mc;
                for bc in &body.children {
                    let BodyChild::Mdiv(mdiv) = bc;
                    if let Some(bs) = extract_book_structure(mdiv) {
                        for mdiv_child in &mdiv.children {
                            let tusk_model::elements::MdivChild::Score(score) = mdiv_child;
                            entries.push(MdivEntry {
                                score,
                                book_structure: bs.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    if entries.is_empty() {
        None
    } else {
        Some(entries)
    }
}

/// Extract `BookStructure` from an mdiv's label.
fn extract_book_structure(mdiv: &tusk_model::elements::Mdiv) -> Option<BookStructure> {
    let label = mdiv.common.label.as_deref()?;
    let escaped = label.strip_prefix("tusk:book-structure,")?;
    let json = unescape_label_value(escaped);
    serde_json::from_str(&json).ok()
}

/// Reconstruct `ToplevelExpression::Book` items from grouped mdiv entries.
///
/// Groups entries by `book_index`, then within each book groups by `bookpart_index`.
/// Returns a list of `ToplevelExpression::Book` items ready for the output file.
pub(super) fn reconstruct_books(
    entries: &[MdivEntry<'_>],
    export_score_fn: &dyn Fn(&tusk_model::elements::Score) -> crate::model::ScoreBlock,
) -> Vec<ToplevelExpression> {
    // Group by book_index
    let mut books: Vec<(usize, Vec<&MdivEntry<'_>>)> = Vec::new();

    for entry in entries {
        let bi = entry.book_structure.book_index.unwrap_or(0);
        if let Some(pos) = books.iter().position(|(idx, _)| *idx == bi) {
            books[pos].1.push(entry);
        } else {
            books.push((bi, vec![entry]));
        }
    }

    books.sort_by_key(|(idx, _)| *idx);

    let mut result = Vec::new();
    for (_book_idx, book_entries) in books {
        let book = build_book_block(book_entries, export_score_fn);
        result.push(ToplevelExpression::Book(book));
    }
    result
}

/// Build a `BookBlock` from entries belonging to the same book.
fn build_book_block(
    entries: Vec<&MdivEntry<'_>>,
    export_score_fn: &dyn Fn(&tusk_model::elements::Score) -> crate::model::ScoreBlock,
) -> BookBlock {
    let mut items: Vec<BookItem> = Vec::new();

    // Extract book-level output defs from the first entry (they're duplicated across all)
    if let Some(first) = entries.first() {
        for od in &first.book_structure.book_output_defs {
            match od.kind {
                OutputDefKind::Header => {
                    items.push(BookItem::Header(output_def_conv::output_def_to_header(od)));
                }
                OutputDefKind::Paper => {
                    items.push(BookItem::Paper(output_def_conv::output_def_to_paper(od)));
                }
                _ => {}
            }
        }
    }

    // Separate entries into bookpart-grouped and direct scores
    let has_bookparts = entries
        .iter()
        .any(|e| e.book_structure.bookpart_index.is_some());

    if has_bookparts {
        // Group by bookpart_index
        let mut bookparts: Vec<(Option<usize>, Vec<&MdivEntry<'_>>)> = Vec::new();
        for entry in &entries {
            let bpi = entry.book_structure.bookpart_index;
            if let Some(pos) = bookparts.iter().position(|(idx, _)| *idx == bpi) {
                bookparts[pos].1.push(entry);
            } else {
                bookparts.push((bpi, vec![entry]));
            }
        }
        // Sort: None (direct scores) first, then by bookpart index
        bookparts.sort_by_key(|(idx, _)| match idx {
            None => (0, 0),
            Some(i) => (1, *i),
        });

        for (bpi, bp_entries) in bookparts {
            if bpi.is_some() {
                let bp = build_bookpart_block(bp_entries, export_score_fn);
                items.push(BookItem::BookPart(bp));
            } else {
                // Direct scores in book (no bookpart)
                for entry in bp_entries {
                    items.push(BookItem::Score(export_score_fn(entry.score)));
                }
            }
        }
    } else {
        // All scores directly in book (no bookparts)
        for entry in &entries {
            items.push(BookItem::Score(export_score_fn(entry.score)));
        }
    }

    BookBlock { items }
}

/// Build a `BookPartBlock` from entries belonging to the same bookpart.
fn build_bookpart_block(
    entries: Vec<&MdivEntry<'_>>,
    export_score_fn: &dyn Fn(&tusk_model::elements::Score) -> crate::model::ScoreBlock,
) -> BookPartBlock {
    let mut items: Vec<BookPartItem> = Vec::new();

    // Extract bookpart-level output defs from the first entry
    if let Some(first) = entries.first() {
        for od in &first.book_structure.bookpart_output_defs {
            match od.kind {
                OutputDefKind::Header => {
                    items.push(BookPartItem::Header(output_def_conv::output_def_to_header(
                        od,
                    )));
                }
                OutputDefKind::Paper => {
                    items.push(BookPartItem::Paper(output_def_conv::output_def_to_paper(
                        od,
                    )));
                }
                _ => {}
            }
        }
    }

    // Add scores sorted by score_index
    let mut sorted: Vec<&MdivEntry<'_>> = entries;
    sorted.sort_by_key(|e| e.book_structure.score_index.unwrap_or(0));

    for entry in sorted {
        items.push(BookPartItem::Score(export_score_fn(entry.score)));
    }

    BookPartBlock { items }
}

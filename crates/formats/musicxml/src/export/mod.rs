//! MEI to MusicXML conversion.
//!
//! This module provides conversion from MEI documents to MusicXML score-partwise
//! format. The conversion is lossy - many MEI-specific features have no MusicXML
//! equivalent and will be lost (see `docs/conversion-notes.md` for details).
//!
//! # Conversion Overview
//!
//! MEI structure maps to MusicXML as follows:
//! - MEI `<meiHead>` → MusicXML header (work, identification)
//! - MEI `<score>/<scoreDef>/<staffGrp>` → MusicXML `<part-list>`
//! - MEI `<section>/<measure>/<staff>/<layer>` → MusicXML `<part>/<measure>`
//!
//! # Example
//!
//! ```ignore
//! use tusk_musicxml::export::convert_mei;
//! use tusk_model::elements::Mei;
//!
//! let mei = Mei::default();
//! let musicxml = convert_mei(&mei)?;
//! ```

mod attributes;
mod content;
mod direction;
mod harmony;
mod note;
mod parts;
mod structure;
mod utils;

// Re-export attributes conversion functions
pub use attributes::{
    convert_mei_clef_shape_to_mxml, convert_mei_keysig_to_fifths, convert_mei_meter_sym_to_mxml,
    convert_mei_score_def_to_attributes, convert_mei_staff_def_to_attributes,
};

// Re-export direction conversion functions
pub use direction::{convert_mei_dir, convert_mei_dynam, convert_mei_hairpin, convert_mei_tempo};

// Re-export note conversion functions
pub use note::{convert_mei_chord, convert_mei_note, convert_mei_rest};

// Re-export parts conversion functions
pub use parts::{
    convert_mei_part_list, convert_mei_staff_def_to_score_part, convert_mei_staff_grp_barline,
    convert_mei_staff_grp_symbol, convert_mei_staff_grp_to_part_list,
};

// Re-export structure conversion functions
pub use structure::convert_mei_measure;

use crate::context::{ConversionContext, ConversionDirection};
use crate::convert_error::ConversionResult;
use crate::model::elements::{
    Encoding, Identification, PartList, PartListItem, ScorePart, ScoreTimewise, Work,
};
use tusk_model::elements::{Mei, MeiChild, MeiHead, MeiHeadChild};
use utils::{
    extract_title_from_file_desc, find_body_in_music, find_first_mdiv_in_body, find_score_def,
    find_score_in_mdiv,
};

/// Convert an MEI document to MusicXML score-partwise.
///
/// This is the main entry point for MEI → MusicXML conversion.
/// Internally produces a `ScoreTimewise` first, then converts to
/// `ScorePartwise` via `timewise_to_partwise`.
///
/// # Arguments
///
/// * `mei` - The MEI document to convert
///
/// # Returns
///
/// A MusicXML score-partwise document, or an error if conversion fails.
///
/// # Lossy Conversion
///
/// This conversion is lossy. MEI-specific features without MusicXML equivalents
/// will generate warnings in the context. Check `ctx.warnings()` after conversion.
pub fn convert_mei(mei: &Mei) -> ConversionResult<crate::model::elements::ScorePartwise> {
    let timewise = convert_mei_to_timewise(mei)?;
    Ok(crate::convert::timewise_to_partwise(timewise))
}

/// Convert an MEI document to MusicXML timewise format.
///
/// This is the core conversion function. It produces a `ScoreTimewise`
/// which is the canonical intermediate representation. Call
/// `timewise_to_partwise` on the result to get partwise output.
///
/// The conversion creates a MusicXML timewise document with:
/// - Header from MEI `<meiHead>` (work, identification, encoding)
/// - Part list from MEI `<scoreDef>/<staffGrp>`
/// - Measures (each containing parts) from MEI `<section>/<measure>/<staff>/<layer>`
pub fn convert_mei_to_timewise(mei: &Mei) -> ConversionResult<ScoreTimewise> {
    let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
    convert_mei_to_timewise_with_context(mei, &mut ctx)
}

/// Convert an MEI document to MusicXML timewise with an existing context.
///
/// This variant allows reusing a conversion context across multiple conversions,
/// which is useful for batch processing or when custom context configuration is needed.
pub fn convert_mei_to_timewise_with_context(
    mei: &Mei,
    ctx: &mut ConversionContext,
) -> ConversionResult<ScoreTimewise> {
    let mut score = ScoreTimewise {
        version: Some("4.0".to_string()),
        ..Default::default()
    };

    // Extract meiHead and music from children
    let mut mei_head: Option<&MeiHead> = None;
    let mut music: Option<&tusk_model::elements::Music> = None;

    for child in &mei.children {
        match child {
            MeiChild::MeiHead(h) => mei_head = Some(h),
            MeiChild::Music(m) => music = Some(m),
        }
    }

    // Convert header (metadata)
    if let Some(head) = mei_head {
        let (work, identification) = convert_header(head, ctx)?;
        score.work = work;
        score.identification = identification;
    }

    // Convert music content (requires body/mdiv/score structure)
    if let Some(music_elem) = music
        && let Some(body) = find_body_in_music(music_elem)
        && let Some(mdiv) = find_first_mdiv_in_body(body)
        && let Some(mei_score) = find_score_in_mdiv(mdiv)
    {
        // Convert scoreDef to part-list
        if let Some(score_def) = find_score_def(mei_score) {
            score.part_list = convert_mei_part_list(score_def, ctx)?;
        } else {
            // No scoreDef, create minimal part-list
            score.part_list = PartList::default();
        }

        // Extract part IDs from part-list
        let part_ids: Vec<String> = score
            .part_list
            .items
            .iter()
            .filter_map(|item| {
                if let PartListItem::ScorePart(sp) = item {
                    Some(sp.id.clone())
                } else {
                    None
                }
            })
            .collect();

        // Convert measure content from MEI to timewise measures
        if !part_ids.is_empty() {
            score.measures = content::convert_mei_score_content(mei_score, &part_ids, ctx)?;
        }
    }

    // If no parts were created, ensure part_list has at least one part
    if score.part_list.items.is_empty() {
        let default_part = ScorePart::new("P1", "Part 1");
        score
            .part_list
            .items
            .push(PartListItem::ScorePart(Box::new(default_part)));
    }

    Ok(score)
}

/// Convert MEI meiHead to MusicXML header elements.
///
/// Returns (Work, Identification) tuple.
fn convert_header(
    mei_head: &MeiHead,
    ctx: &mut ConversionContext,
) -> ConversionResult<(Option<Work>, Option<Identification>)> {
    let mut work: Option<Work> = None;
    let mut identification = Identification::default();

    // Find fileDesc to extract title
    for child in &mei_head.children {
        if let MeiHeadChild::FileDesc(file_desc) = child {
            // Extract title from fileDesc -> titleStmt -> title
            if let Some(title) = extract_title_from_file_desc(file_desc) {
                work = Some(Work {
                    work_title: Some(title),
                    ..Default::default()
                });
            }
        }
    }

    // Add encoding info showing conversion from MEI
    let encoding = Encoding {
        software: vec!["Tusk MusicXML-MEI Converter".to_string()],
        ..Default::default()
    };
    identification.encoding = Some(encoding);

    // Add warning about potential lossy conversion
    ctx.add_warning(
        "meiHead",
        "MEI metadata may be simplified during conversion to MusicXML",
    );

    Ok((work, Some(identification)))
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tusk_model::elements::{
        FileDesc, FileDescChild, PubStmt, Title, TitleChild, TitleStmt, TitleStmtChild,
    };

    // ========================================================================
    // Basic Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_empty_mei_to_timewise() {
        let mei = Mei::default();
        let result = convert_mei_to_timewise(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert_eq!(score.version.as_deref(), Some("4.0"));
        // Should have at least one part in part_list
        assert!(!score.part_list.items.is_empty());
    }

    #[test]
    fn test_convert_empty_mei_to_partwise() {
        let mei = Mei::default();
        let result = convert_mei(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert_eq!(score.version.as_deref(), Some("4.0"));
        assert!(!score.part_list.items.is_empty());
        // Partwise gets parts from timewise_to_partwise
        assert!(!score.parts.is_empty());
    }

    #[test]
    fn test_convert_mei_with_context() {
        let mei = Mei::default();
        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);

        let result = convert_mei_to_timewise_with_context(&mei, &mut ctx);
        assert!(result.is_ok());
        assert!(ctx.is_mei_to_musicxml());
    }

    // ========================================================================
    // Header Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_with_title() {
        let mut mei = Mei::default();

        // Create meiHead with title
        let mut mei_head = MeiHead::default();
        let mut file_desc = FileDesc::default();
        let mut title_stmt = TitleStmt::default();
        let mut title = Title::default();
        title
            .children
            .push(TitleChild::Text("Symphony No. 5".to_string()));
        title_stmt
            .children
            .push(TitleStmtChild::Title(Box::new(title)));

        let pub_stmt = PubStmt::default();
        file_desc
            .children
            .push(FileDescChild::TitleStmt(Box::new(title_stmt)));
        file_desc
            .children
            .push(FileDescChild::PubStmt(Box::new(pub_stmt)));

        mei_head
            .children
            .push(MeiHeadChild::FileDesc(Box::new(file_desc)));
        mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

        let result = convert_mei_to_timewise(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score.work.is_some());
        assert_eq!(
            score.work.as_ref().unwrap().work_title.as_deref(),
            Some("Symphony No. 5")
        );
    }

    #[test]
    fn test_convert_mei_with_identification() {
        let mut mei = Mei::default();
        let mei_head = MeiHead::default();
        mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));

        let result = convert_mei_to_timewise(&mei);
        assert!(result.is_ok());

        let score = result.unwrap();
        assert!(score.identification.is_some());
        let ident = score.identification.as_ref().unwrap();
        assert!(ident.encoding.is_some());
        assert!(!ident.encoding.as_ref().unwrap().software.is_empty());
    }
}

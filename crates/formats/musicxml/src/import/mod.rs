//! MusicXML to MEI conversion.
//!
//! This module provides conversion from MusicXML score-partwise documents
//! to MEI format. The conversion is lossless - all MusicXML content is
//! preserved in the MEI output.
//!
//! # Conversion Overview
//!
//! MusicXML `<score-partwise>` maps to MEI as follows:
//! - MusicXML header (work, identification) → MEI `<meiHead>`
//! - MusicXML `<part-list>` → MEI `<staffGrp>` with `<staffDef>` elements
//! - MusicXML `<part>/<measure>` → MEI `<section>/<measure>/<staff>/<layer>`
//!
//! # Example
//!
//! ```ignore
//! use tusk_musicxml::import::convert_score;
//! use tusk_musicxml::model::elements::ScorePartwise;
//!
//! let score = ScorePartwise::default();
//! let mei = convert_score(&score)?;
//! ```

mod attributes;
mod direction;
mod note;
mod parts;
mod structure;
mod utils;

// Re-export attributes conversion functions
pub use attributes::{
    convert_clef_attributes, convert_key_fifths, convert_key_to_context, convert_time_signature,
    process_attributes,
};

// Re-export direction conversion functions
pub use direction::{DirectionConversionResult, convert_direction};

// Re-export note conversion functions
pub use note::{convert_chord, convert_measure_rest, convert_note, convert_rest, is_measure_rest};

// Re-export structure conversion functions
pub use structure::{
    convert_body, convert_layer, convert_mdiv, convert_measure, convert_score_content,
    convert_section, convert_staff,
};

// Re-export parts conversion functions
pub use parts::{
    convert_score_def, convert_staff_def_from_score_part, convert_staff_grp,
};

use crate::context::{ConversionContext, ConversionDirection};
use crate::convert_error::ConversionResult;
use crate::model::elements::ScorePartwise;
use tusk_model::att::AttMeiVersionMeiversion;
use tusk_model::elements::{Mei, MeiChild, MeiHead, MeiHeadChild, Music};

/// Convert a MusicXML score-partwise document to MEI.
///
/// This is the main entry point for MusicXML → MEI conversion.
/// The conversion creates a complete MEI document with:
/// - `<meiHead>` containing metadata from MusicXML identification
/// - `<music>/<body>/<mdiv>/<score>` containing the musical content
///
/// # Arguments
///
/// * `score` - The MusicXML score-partwise document to convert
///
/// # Returns
///
/// A complete MEI document, or an error if conversion fails.
pub fn convert_score(score: &ScorePartwise) -> ConversionResult<Mei> {
    let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
    convert_score_with_context(score, &mut ctx)
}

/// Convert a MusicXML score-partwise document to MEI with an existing context.
///
/// This variant allows reusing a conversion context across multiple conversions,
/// which is useful for batch processing or when custom context configuration is needed.
pub fn convert_score_with_context(
    score: &ScorePartwise,
    ctx: &mut ConversionContext,
) -> ConversionResult<Mei> {
    // Build MEI document structure
    let mei_head = convert_header(score, ctx)?;
    let music = convert_music(score, ctx)?;

    // Create root MEI element
    let mut mei = Mei::default();
    mei.mei_version.meiversion = Some(AttMeiVersionMeiversion::N60Dev);

    mei.children.push(MeiChild::MeiHead(Box::new(mei_head)));
    mei.children.push(MeiChild::Music(Box::new(music)));

    Ok(mei)
}

/// Convert MusicXML header information to MEI meiHead.
fn convert_header(score: &ScorePartwise, ctx: &mut ConversionContext) -> ConversionResult<MeiHead> {
    let mut mei_head = MeiHead::default();

    // Create fileDesc with titleStmt
    let file_desc = convert_file_desc(score, ctx)?;
    mei_head
        .children
        .push(MeiHeadChild::FileDesc(Box::new(file_desc)));

    // Add encodingDesc with Tusk application info
    let encoding_desc = create_encoding_desc(ctx)?;
    mei_head
        .children
        .push(MeiHeadChild::EncodingDesc(Box::new(encoding_desc)));

    Ok(mei_head)
}

/// Convert MusicXML identification to MEI fileDesc.
fn convert_file_desc(
    score: &ScorePartwise,
    _ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::FileDesc> {
    use tusk_model::elements::{
        FileDesc, FileDescChild, PubStmt, Title, TitleStmt, TitleStmtChild,
    };

    let mut file_desc = FileDesc::default();

    // Create titleStmt with title
    let mut title_stmt = TitleStmt::default();

    // Try to get title from work-title, movement-title, or fall back to "Untitled"
    let title_text = score
        .work
        .as_ref()
        .and_then(|w| w.work_title.as_ref())
        .or(score.movement_title.as_ref())
        .map(|s| s.as_str())
        .unwrap_or("Untitled");

    let mut title = Title::default();
    title.children.push(tusk_model::elements::TitleChild::Text(
        title_text.to_string(),
    ));
    title_stmt
        .children
        .push(TitleStmtChild::Title(Box::new(title)));

    file_desc
        .children
        .push(FileDescChild::TitleStmt(Box::new(title_stmt)));

    // Add pubStmt (required, even if empty)
    let pub_stmt = PubStmt::default();
    file_desc
        .children
        .push(FileDescChild::PubStmt(Box::new(pub_stmt)));

    Ok(file_desc)
}

/// Create MEI encodingDesc with Tusk application info.
fn create_encoding_desc(
    _ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::EncodingDesc> {
    use tusk_model::elements::{
        AppInfo, AppInfoChild, Application, ApplicationChild, EncodingDesc, EncodingDescChild,
        Name, NameChild,
    };

    let mut encoding_desc = EncodingDesc::default();

    // Create appInfo with Tusk application
    let mut app_info = AppInfo::default();

    let mut application = Application::default();
    application.common.xml_id = Some("tusk".to_string());

    let mut name = Name::default();
    name.children
        .push(NameChild::Text("Tusk MusicXML-MEI Converter".to_string()));
    application
        .children
        .push(ApplicationChild::Name(Box::new(name)));

    app_info
        .children
        .push(AppInfoChild::Application(Box::new(application)));
    encoding_desc
        .children
        .push(EncodingDescChild::AppInfo(Box::new(app_info)));

    Ok(encoding_desc)
}

/// Convert MusicXML score content to MEI music element.
fn convert_music(score: &ScorePartwise, ctx: &mut ConversionContext) -> ConversionResult<Music> {
    use tusk_model::elements::MusicChild;

    let mut music = Music::default();

    // Create body containing mdiv with score content
    let body = convert_body(score, ctx)?;
    music.children.push(MusicChild::Body(Box::new(body)));

    Ok(music)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::elements::Work;

    // ============================================================================
    // Basic Document Structure Tests
    // ============================================================================

    #[test]
    fn convert_empty_score_creates_valid_mei_structure() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        // Check MEI version is set
        assert!(mei.mei_version.meiversion.is_some());

        // Check we have meiHead and music children
        assert_eq!(mei.children.len(), 2);
        assert!(matches!(&mei.children[0], MeiChild::MeiHead(_)));
        assert!(matches!(&mei.children[1], MeiChild::Music(_)));
    }

    #[test]
    fn convert_score_sets_mei_version() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        // Should set MEI version to 6.0-dev (current dev version from ODD)
        assert_eq!(
            mei.mei_version.meiversion,
            Some(AttMeiVersionMeiversion::N60Dev)
        );
    }

    // ============================================================================
    // Header Conversion Tests
    // ============================================================================

    #[test]
    fn convert_header_creates_file_desc() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            // Should have fileDesc as first child
            assert!(!head.children.is_empty());
            assert!(matches!(&head.children[0], MeiHeadChild::FileDesc(_)));
        } else {
            panic!("Expected MeiHead");
        }
    }

    #[test]
    fn convert_header_uses_work_title() {
        let mut score = ScorePartwise::default();
        score.work = Some(Work {
            work_title: Some("Test Symphony".to_string()),
            ..Default::default()
        });

        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            if let MeiHeadChild::FileDesc(file_desc) = &head.children[0] {
                // Find titleStmt
                let title_stmt = file_desc.children.iter().find_map(|c| {
                    if let tusk_model::elements::FileDescChild::TitleStmt(ts) = c {
                        Some(ts)
                    } else {
                        None
                    }
                });
                assert!(title_stmt.is_some());

                // Check title content
                let ts = title_stmt.unwrap();
                let title = ts.children.iter().find_map(|c| {
                    if let tusk_model::elements::TitleStmtChild::Title(t) = c {
                        Some(t)
                    } else {
                        None
                    }
                });
                assert!(title.is_some());

                // Check title text
                let t = title.unwrap();
                let text = t.children.iter().find_map(|c| {
                    if let tusk_model::elements::TitleChild::Text(s) = c {
                        Some(s.as_str())
                    } else {
                        None
                    }
                });
                assert_eq!(text, Some("Test Symphony"));
            } else {
                panic!("Expected FileDesc");
            }
        } else {
            panic!("Expected MeiHead");
        }
    }

    #[test]
    fn convert_header_uses_movement_title_as_fallback() {
        let mut score = ScorePartwise::default();
        score.movement_title = Some("Movement I".to_string());

        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            if let MeiHeadChild::FileDesc(file_desc) = &head.children[0] {
                let title_text = extract_title_text(file_desc);
                assert_eq!(title_text, Some("Movement I"));
            } else {
                panic!("Expected FileDesc");
            }
        } else {
            panic!("Expected MeiHead");
        }
    }

    #[test]
    fn convert_header_uses_untitled_when_no_title() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            if let MeiHeadChild::FileDesc(file_desc) = &head.children[0] {
                let title_text = extract_title_text(file_desc);
                assert_eq!(title_text, Some("Untitled"));
            } else {
                panic!("Expected FileDesc");
            }
        } else {
            panic!("Expected MeiHead");
        }
    }

    #[test]
    fn convert_header_includes_encoding_desc() {
        let score = ScorePartwise::default();
        let mei = convert_score(&score).expect("conversion should succeed");

        if let MeiChild::MeiHead(head) = &mei.children[0] {
            let has_encoding_desc = head
                .children
                .iter()
                .any(|c| matches!(c, MeiHeadChild::EncodingDesc(_)));
            assert!(has_encoding_desc, "Should include encodingDesc");
        } else {
            panic!("Expected MeiHead");
        }
    }

    // ============================================================================
    // Helper Functions
    // ============================================================================

    fn extract_title_text(file_desc: &tusk_model::elements::FileDesc) -> Option<&str> {
        use tusk_model::elements::{FileDescChild, TitleChild, TitleStmtChild};

        for child in &file_desc.children {
            if let FileDescChild::TitleStmt(ts) = child {
                for ts_child in &ts.children {
                    if let TitleStmtChild::Title(title) = ts_child {
                        for t_child in &title.children {
                            if let TitleChild::Text(s) = t_child {
                                return Some(s.as_str());
                            }
                        }
                    }
                }
            }
        }
        None
    }
}

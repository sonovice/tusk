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
pub(crate) mod figured_bass;
pub(crate) mod harmony;
mod note;
mod parts;
pub(crate) mod print;
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
pub use parts::{convert_score_def, convert_staff_def_from_score_part, convert_staff_grp};

use crate::context::{ConversionContext, ConversionDirection};
use crate::convert_error::ConversionResult;
use crate::model::elements::ScorePartwise;
use tusk_model::elements::{Mei, MeiChild, MeiHead, MeiHeadChild, Music};

/// Label prefix for MEI extMeta carrying roundtrip identification JSON.
pub(crate) const IDENTIFICATION_LABEL_PREFIX: &str = "musicxml:identification,";
/// Label prefix for MEI extMeta carrying roundtrip work JSON.
pub(crate) const WORK_LABEL_PREFIX: &str = "musicxml:work,";
/// Label prefix for MEI extMeta carrying roundtrip movement-number.
pub(crate) const MOVEMENT_NUMBER_LABEL_PREFIX: &str = "musicxml:movement-number,";
/// Label prefix for MEI extMeta carrying roundtrip movement-title.
pub(crate) const MOVEMENT_TITLE_LABEL_PREFIX: &str = "musicxml:movement-title,";
/// Label prefix for MEI extMeta carrying roundtrip defaults JSON.
pub(crate) const DEFAULTS_LABEL_PREFIX: &str = "musicxml:defaults,";
/// Label prefix for MEI extMeta carrying roundtrip credits JSON.
pub(crate) const CREDITS_LABEL_PREFIX: &str = "musicxml:credits,";

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
    mei.mei_version.meiversion = Some("6.0-dev".to_string());

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

    // Store full identification as JSON in extMeta for lossless roundtrip.
    // The generated MEI model's header types are too limited to hold all
    // MusicXML identification fields (TitleStmt only has Title, PubStmt
    // only has Unpub, NotesStmt has no children, etc.), so we use extMeta
    // which is designed for "non-MEI metadata formats".
    //
    // Only store when there's meaningful data beyond the default Tusk
    // encoding that the export creates. This prevents the triangle MEI
    // roundtrip from diverging: without this guard, export adds a default
    // Identification (Tusk software), the next import stores it in extMeta,
    // and MEI₂ gets an extMeta that MEI₁ didn't have.
    if let Some(identification) = &score.identification {
        if has_meaningful_identification(identification) {
            if let Ok(json) = serde_json::to_string(identification) {
                let ext_meta = create_ext_meta(
                    ctx,
                    "identification",
                    IDENTIFICATION_LABEL_PREFIX,
                    &json,
                    &identification_summary(identification),
                );
                mei_head
                    .children
                    .push(MeiHeadChild::ExtMeta(Box::new(ext_meta)));
            }
        }
    }

    // Store work data (work-number, opus) in extMeta for roundtrip of
    // fields beyond the basic work-title already in titleStmt.
    if let Some(work) = &score.work {
        if work.work_number.is_some() || work.opus.is_some() {
            if let Ok(json) = serde_json::to_string(work) {
                let ext_meta =
                    create_ext_meta(ctx, "work", WORK_LABEL_PREFIX, &json, &work_summary(work));
                mei_head
                    .children
                    .push(MeiHeadChild::ExtMeta(Box::new(ext_meta)));
            }
        }
    }

    // Store movement-number if present
    if let Some(movement_number) = &score.movement_number {
        let ext_meta = create_ext_meta(
            ctx,
            "mvmt-num",
            MOVEMENT_NUMBER_LABEL_PREFIX,
            movement_number,
            movement_number,
        );
        mei_head
            .children
            .push(MeiHeadChild::ExtMeta(Box::new(ext_meta)));
    }

    // Store movement-title if present
    if let Some(movement_title) = &score.movement_title {
        let ext_meta = create_ext_meta(
            ctx,
            "mvmt-title",
            MOVEMENT_TITLE_LABEL_PREFIX,
            movement_title,
            movement_title,
        );
        mei_head
            .children
            .push(MeiHeadChild::ExtMeta(Box::new(ext_meta)));
    }

    // Store defaults (layout, appearance, fonts) in extMeta for lossless roundtrip.
    // MEI scoreDef attributes can hold some of these (page dimensions, margins,
    // spacing, fonts) but not all (appearance line-widths, note-sizes, glyphs,
    // system-dividers, etc.). The extMeta JSON preserves the full Defaults struct.
    if let Some(defaults) = &score.defaults {
        if let Ok(json) = serde_json::to_string(defaults) {
            let ext_meta = create_ext_meta(
                ctx,
                "defaults",
                DEFAULTS_LABEL_PREFIX,
                &json,
                &defaults_summary(defaults),
            );
            mei_head
                .children
                .push(MeiHeadChild::ExtMeta(Box::new(ext_meta)));
        }
    }

    // Store credits in extMeta for lossless roundtrip. MEI pgHead/pgFoot only
    // support text and anchoredText children — no structured credit-type, font
    // attributes, positioning, or images. The extMeta JSON preserves the full
    // Vec<Credit> with all formatting, positioning, links, and bookmarks.
    if !score.credits.is_empty() {
        if let Ok(json) = serde_json::to_string(&score.credits) {
            let ext_meta = create_ext_meta(
                ctx,
                "credits",
                CREDITS_LABEL_PREFIX,
                &json,
                &credits_summary(&score.credits),
            );
            mei_head
                .children
                .push(MeiHeadChild::ExtMeta(Box::new(ext_meta)));
        }
    }

    Ok(mei_head)
}

/// Create an extMeta element with a label prefix + data and human-readable text.
fn create_ext_meta(
    ctx: &mut ConversionContext,
    id_suffix: &str,
    label_prefix: &str,
    data: &str,
    summary_text: &str,
) -> tusk_model::elements::ExtMeta {
    use tusk_model::elements::{ExtMeta, ExtMetaChild};

    let mut ext_meta = ExtMeta::default();
    ext_meta.common.xml_id = Some(ctx.generate_id_with_suffix(id_suffix));
    ext_meta.bibl.analog = Some(format!("{label_prefix}{data}"));
    ext_meta
        .children
        .push(ExtMetaChild::Text(summary_text.to_string()));
    ext_meta
}

/// Check if identification has meaningful data beyond the default Tusk encoding.
///
/// Returns false for identifications that only contain the Tusk software
/// entry (which is the default added by the export path). This prevents
/// a growing extMeta on each roundtrip.
fn has_meaningful_identification(ident: &crate::model::elements::Identification) -> bool {
    if !ident.creators.is_empty()
        || !ident.rights.is_empty()
        || ident.source.is_some()
        || !ident.relations.is_empty()
        || ident.miscellaneous.is_some()
    {
        return true;
    }
    // Check encoding: meaningful if it has anything beyond just Tusk software
    if let Some(enc) = &ident.encoding {
        if enc.encoding_date.is_some()
            || !enc.encoders.is_empty()
            || !enc.encoding_descriptions.is_empty()
            || !enc.supports.is_empty()
        {
            return true;
        }
        // software: meaningful if there's any entry besides "Tusk MusicXML-MEI Converter"
        let non_tusk_sw = enc
            .software
            .iter()
            .any(|s| s != "Tusk MusicXML-MEI Converter");
        if non_tusk_sw {
            return true;
        }
    }
    false
}

/// Build a human-readable summary of identification metadata.
fn identification_summary(ident: &crate::model::elements::Identification) -> String {
    let mut parts = Vec::new();
    for creator in &ident.creators {
        if let Some(t) = &creator.text_type {
            parts.push(format!("{}: {}", t, creator.value));
        } else {
            parts.push(creator.value.clone());
        }
    }
    for right in &ident.rights {
        parts.push(format!("rights: {}", right.value));
    }
    if let Some(source) = &ident.source {
        parts.push(format!("source: {source}"));
    }
    if let Some(enc) = &ident.encoding {
        for sw in &enc.software {
            parts.push(format!("software: {sw}"));
        }
    }
    if parts.is_empty() {
        "identification".to_string()
    } else {
        parts.join("; ")
    }
}

/// Build a human-readable summary of work metadata.
fn work_summary(work: &crate::model::elements::Work) -> String {
    let mut parts = Vec::new();
    if let Some(n) = &work.work_number {
        parts.push(format!("number: {n}"));
    }
    if let Some(opus) = &work.opus {
        parts.push(format!("opus: {}", opus.href));
    }
    if parts.is_empty() {
        "work".to_string()
    } else {
        parts.join("; ")
    }
}

/// Build a human-readable summary of defaults metadata.
fn defaults_summary(defaults: &crate::model::elements::Defaults) -> String {
    let mut parts = Vec::new();
    if let Some(scaling) = &defaults.scaling {
        parts.push(format!(
            "scaling: {}mm/{}tenths",
            scaling.millimeters, scaling.tenths
        ));
    }
    if let Some(pl) = &defaults.page_layout {
        if let (Some(h), Some(w)) = (pl.page_height, pl.page_width) {
            parts.push(format!("page: {w}x{h}"));
        }
    }
    if defaults.system_layout.is_some() {
        parts.push("system-layout".to_string());
    }
    if !defaults.staff_layouts.is_empty() {
        parts.push(format!("{} staff-layout(s)", defaults.staff_layouts.len()));
    }
    if defaults.appearance.is_some() {
        parts.push("appearance".to_string());
    }
    if defaults.music_font.is_some() {
        parts.push("music-font".to_string());
    }
    if defaults.word_font.is_some() {
        parts.push("word-font".to_string());
    }
    if !defaults.lyric_fonts.is_empty() {
        parts.push("lyric-font".to_string());
    }
    if parts.is_empty() {
        "defaults".to_string()
    } else {
        parts.join("; ")
    }
}

/// Build a human-readable summary of credits.
fn credits_summary(credits: &[crate::model::elements::Credit]) -> String {
    use crate::model::elements::CreditContent;
    let mut parts = Vec::new();
    for credit in credits {
        if let Some(CreditContent::Words(words)) = &credit.content {
            for w in &words.words {
                if !w.value.is_empty() {
                    let truncated: String = w.value.chars().take(40).collect();
                    parts.push(truncated);
                }
            }
        } else if let Some(CreditContent::Image(_)) = &credit.content {
            parts.push("[image]".to_string());
        }
    }
    if parts.is_empty() {
        "credits".to_string()
    } else {
        parts.join("; ")
    }
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
pub(crate) mod test_utils {
    use crate::model::elements::{PartName, ScorePart};

    /// Helper to create a ScorePart with the given id and name.
    pub fn make_score_part(id: &str, name: &str) -> ScorePart {
        ScorePart {
            id: id.to_string(),
            identification: None,
            part_links: vec![],
            part_name: PartName {
                value: name.to_string(),
                ..Default::default()
            },
            part_name_display: None,
            part_abbreviation: None,
            part_abbreviation_display: None,
            groups: vec![],
            score_instruments: vec![],
            players: vec![],
            midi_assignments: vec![],
        }
    }
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

        // Should set MEI version to 6.0-dev (current dev version from RNG schema)
        assert_eq!(mei.mei_version.meiversion.as_deref(), Some("6.0-dev"));
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
                    let tusk_model::elements::TitleStmtChild::Title(t) = c;
                    Some(t)
                });
                assert!(title.is_some());

                // Check title text
                let t = title.unwrap();
                let text = t.children.iter().find_map(|c| {
                    let tusk_model::elements::TitleChild::Text(s) = c else {
                        return None;
                    };
                    Some(s.as_str())
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
                    let TitleStmtChild::Title(title) = ts_child;
                    for t_child in &title.children {
                        let TitleChild::Text(s) = t_child else {
                            continue;
                        };
                        return Some(s.as_str());
                    }
                }
            }
        }
        None
    }
}

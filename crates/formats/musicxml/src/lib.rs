// Allow clippy lints that are widespread in generated/mechanical code
#![allow(
    clippy::collapsible_if,
    clippy::derivable_impls,
    clippy::doc_lazy_continuation
)]

//! MusicXML parsing, serialization, and conversion for Tusk.
//!
//! This crate handles reading and writing MusicXML files, as well as
//! bidirectional conversion between MusicXML and MEI formats.
//!
//! # Supported Versions
//!
//! - MusicXML 4.0 (primary target)
//! - MusicXML 3.1 (with upgrade to 4.0)
//! - MusicXML 3.0 (with upgrade to 4.0)
//! - MusicXML 2.0 (with upgrade to 4.0)
//!
//! # Document Types
//!
//! Both `score-partwise` and `score-timewise` formats are supported.
//!
//! **Timewise is the canonical internal format.** All roundtrip comparisons
//! and internal operations use `ScoreTimewise`. External files are written
//! as partwise (the industry convention) and automatically converted on
//! import/export.
//!
//! # Module Organization
//!
//! - `model` - MusicXML intermediate data model (types matching XSD schema)
//! - `parser` - MusicXML parsing from XML to the model types
//! - `versions` - Version-specific parsing and upgrade logic
//! - `context` - Conversion context for tracking state during conversion
//! - `convert` - Partwise ↔ Timewise conversion (mirrors `parttime.xsl` / `timepart.xsl`)
//! - `import` - MusicXML → MEI conversion (lossless)
//! - `export` - MEI → MusicXML conversion (lossy, see docs/conversion-notes.md)
//!
//! # Pipeline
//!
//! ```text
//! Import: partwise XML → parse → ScorePartwise → partwise_to_timewise → ScoreTimewise → MEI
//! Export: MEI → ScoreTimewise → timewise_to_partwise → ScorePartwise → serialize → partwise XML
//! Roundtrip comparison: in ScoreTimewise space
//! ```
//!
//! # Conversion Example
//!
//! ```ignore
//! use tusk_musicxml::{import, export, import_timewise, export_timewise};
//! use tusk_musicxml::model::elements::ScorePartwise;
//!
//! // MusicXML → MEI (lossless, via timewise)
//! let musicxml_score: ScorePartwise = /* ... */;
//! let mei_doc = import(&musicxml_score)?;
//!
//! // MEI → MusicXML (lossy)
//! let mei_doc: tusk_model::elements::Mei = /* ... */;
//! let musicxml_score = export(&mei_doc)?;
//!
//! // Direct timewise access
//! let timewise = import_timewise(&musicxml_score)?; // ScorePartwise → ScoreTimewise
//! let timewise = export_timewise(&mei_doc)?;        // MEI → ScoreTimewise
//! ```

pub mod context;
pub mod convert;
pub mod convert_error;
pub mod export;
pub mod import;
pub mod model;
pub mod parser;
pub mod serializer;
pub mod versions;

// Re-export commonly used types from model
pub use model::*;
pub use parser::{ParseError, parse_score_partwise, parse_score_timewise};

// Re-export conversion context types
pub use context::{ConversionContext, ConversionDirection, PendingSlur, PendingTie};
pub use convert::{partwise_to_timewise, timewise_to_partwise};
pub use convert_error::{ConversionError, ConversionResult};

// Re-export serializer types
pub use serializer::{
    MusicXmlSerialize, MusicXmlWriter, SerializeConfig, SerializeError, SerializeResult,
};

/// Import a MusicXML score-partwise document to MEI (lossless conversion).
///
/// This is the main entry point for MusicXML → MEI conversion.
///
/// # Example
///
/// ```ignore
/// use tusk_musicxml::{import, model::elements::ScorePartwise};
///
/// let score = ScorePartwise::default();
/// let mei = import(&score)?;
/// ```
pub fn import(
    score: &model::elements::ScorePartwise,
) -> ConversionResult<tusk_model::elements::Mei> {
    import::convert_score(score)
}

/// Convert a partwise MusicXML score to a timewise representation.
///
/// This is a pure structural transformation (mirrors `parttime.xsl`).
/// Use this for roundtrip comparison — timewise is the canonical format.
pub fn import_timewise(score: &model::elements::ScorePartwise) -> model::elements::ScoreTimewise {
    convert::partwise_to_timewise(score.clone())
}

/// Export an MEI document to MusicXML score-partwise (lossy conversion).
///
/// This is the main entry point for MEI → MusicXML conversion.
/// Internally produces a timewise representation first, then converts
/// to partwise. Note: This conversion is lossy. Many MEI-specific
/// features have no MusicXML equivalent and will be lost.
///
/// # Example
///
/// ```ignore
/// use tusk_musicxml::export;
/// use tusk_model::elements::Mei;
///
/// let mei = Mei::default();
/// let musicxml = export(&mei)?;
/// ```
pub fn export(mei: &tusk_model::elements::Mei) -> ConversionResult<model::elements::ScorePartwise> {
    let timewise = export::convert_mei_to_timewise(mei)?;
    Ok(convert::timewise_to_partwise(timewise))
}

/// Export an MEI document to MusicXML timewise (lossy conversion).
///
/// Returns the intermediate timewise representation directly, useful for
/// roundtrip comparison and debugging.
pub fn export_timewise(
    mei: &tusk_model::elements::Mei,
) -> ConversionResult<model::elements::ScoreTimewise> {
    export::convert_mei_to_timewise(mei)
}

/// Serialize a MusicXML score-partwise document to a partwise XML string.
///
/// This is the main entry point for MusicXML serialization.
///
/// # Example
///
/// ```ignore
/// use tusk_musicxml::{serialize, model::elements::ScorePartwise};
///
/// let score = ScorePartwise::default();
/// let xml = serialize(&score)?;
/// ```
pub fn serialize(score: &model::elements::ScorePartwise) -> SerializeResult<String> {
    score.to_musicxml_string()
}

/// Serialize a MusicXML score-timewise document to a timewise XML string.
///
/// Writes `<score-timewise>` as root element with the timewise DOCTYPE.
pub fn serialize_timewise_score(score: &model::elements::ScoreTimewise) -> SerializeResult<String> {
    serializer::serialize_timewise(score)
}

// ---------------------------------------------------------------------------
// Unified format trait implementations
// ---------------------------------------------------------------------------

/// MusicXML format handler.
///
/// Implements the unified [`tusk_format`] traits so that MusicXML can be
/// used interchangeably with other formats through the [`FormatRegistry`].
///
/// **Import** (MusicXML → MEI) is lossless: the full pipeline parses XML
/// into a `ScorePartwise`, converts to timewise, and then maps to MEI.
///
/// **Export** (MEI → MusicXML) is lossy: many MEI-specific features have
/// no MusicXML equivalent and will be dropped.
///
/// [`FormatRegistry`]: tusk_format::FormatRegistry
pub struct MusicXmlFormat;

impl tusk_format::Format for MusicXmlFormat {
    fn id(&self) -> &'static str {
        "musicxml"
    }

    fn name(&self) -> &'static str {
        "MusicXML"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["musicxml", "xml", "mxl"]
    }

    fn detect(&self, content: &[u8]) -> bool {
        // Only check the first 4 KB for efficiency with large files.
        let prefix = &content[..content.len().min(4096)];
        let s = std::str::from_utf8(prefix).unwrap_or("");
        s.contains("<score-partwise") || s.contains("<score-timewise") || s.contains("musicxml.org")
    }
}

impl tusk_format::Importer for MusicXmlFormat {
    fn import_from_str(&self, input: &str) -> tusk_format::FormatResult<tusk_format::Mei> {
        let score = crate::parse_score_partwise(input)
            .or_else(|_| crate::parse_score_timewise(input))
            .map_err(tusk_format::FormatError::parse)?;
        crate::import(&score).map_err(tusk_format::FormatError::conversion)
    }
}

impl tusk_format::Exporter for MusicXmlFormat {
    fn export_to_string(&self, mei: &tusk_format::Mei) -> tusk_format::FormatResult<String> {
        let score = crate::export(mei).map_err(tusk_format::FormatError::conversion)?;
        crate::serialize(&score).map_err(tusk_format::FormatError::serialize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_empty_score_succeeds() {
        let score = model::elements::ScorePartwise::default();
        assert!(import(&score).is_ok());
    }

    #[test]
    fn musicxml_format_trait_metadata() {
        use tusk_format::Format;
        let fmt = MusicXmlFormat;
        assert_eq!(fmt.id(), "musicxml");
        assert_eq!(fmt.name(), "MusicXML");
        assert!(fmt.extensions().contains(&"musicxml"));
        assert!(fmt.extensions().contains(&"xml"));
    }

    #[test]
    fn musicxml_format_detect() {
        use tusk_format::Format;
        let fmt = MusicXmlFormat;
        assert!(fmt.detect(b"<?xml?><score-partwise version=\"4.0\">"));
        assert!(fmt.detect(b"<?xml?><score-timewise version=\"4.0\">"));
        assert!(!fmt.detect(b"<mei xmlns=\"http://www.music-encoding.org/ns/mei\">"));
    }
}

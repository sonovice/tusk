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
//! Timewise documents are converted to partwise internally.
//!
//! # Module Organization
//!
//! - `model` - MusicXML intermediate data model (types matching XSD schema)
//! - `parser` - MusicXML parsing from XML to the model types
//! - `versions` - Version-specific parsing and upgrade logic
//! - `context` - Conversion context for tracking state during conversion
//! - `import` - MusicXML → MEI conversion (lossless)
//! - `export` - MEI → MusicXML conversion (lossy, see docs/conversion-notes.md)
//!
//! # Conversion Example
//!
//! ```ignore
//! use tusk_musicxml::{import, export};
//! use tusk_musicxml::model::elements::ScorePartwise;
//!
//! // MusicXML → MEI (lossless)
//! let musicxml_score: ScorePartwise = /* ... */;
//! let mei_doc = import(&musicxml_score)?;
//!
//! // MEI → MusicXML (lossy)
//! let mei_doc: tusk_model::elements::Mei = /* ... */;
//! let musicxml_score = export(&mei_doc)?;
//! ```

pub mod context;
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
pub use convert_error::{ConversionError, ConversionResult};

// Re-export serializer types
pub use serializer::{MusicXmlSerialize, MusicXmlWriter, SerializeConfig, SerializeError, SerializeResult};

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

/// Export an MEI document to MusicXML score-partwise (lossy conversion).
///
/// This is the main entry point for MEI → MusicXML conversion.
/// Note: This conversion is lossy. Many MEI-specific features have no
/// MusicXML equivalent and will be lost. See docs/conversion-notes.md.
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
    export::convert_mei(mei)
}

/// Serialize a MusicXML score-partwise document to an XML string.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crate_compiles() {
        // Smoke test: ensure types can be instantiated
        let _ = Step::C;
        let _ = YesNo::Yes;
    }
}

//! MusicXML parsing and serialization for Tusk.
//!
//! This crate handles reading and writing MusicXML files.
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

pub mod model;
pub mod parser;
pub mod versions;

// Re-export commonly used types
pub use model::*;
pub use parser::{ParseError, parse_score_partwise};

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

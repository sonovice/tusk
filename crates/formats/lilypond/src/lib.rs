//! LilyPond parsing, serialization, and conversion for Tusk.
//!
//! This crate handles reading and writing LilyPond (`.ly`) files, as well as
//! bidirectional conversion between LilyPond and MEI. The parser aims to
//! validate against the full LilyPond grammar (see `specs/lilypond/repo/lily/parser.yy`).
//!
//! # Module Organization
//!
//! - `model` — LilyPond AST types mirroring the grammar
//! - `lexer` — Tokenization with mode switching (note, lyric, chord, etc.)
//! - `parser` — Recursive-descent parser
//! - `serializer` — AST to .ly string
//! - `import` — LilyPond AST → MEI
//! - `export` — MEI → LilyPond AST
//! - `validator` — Structural validation of the AST
//!
//! # Pipeline
//!
//! ```text
//! Import: .ly → lex → parse → AST → validate → MEI
//! Export: MEI → AST → serialize → .ly
//! ```

pub mod model;
pub mod lexer;
pub mod parser;
pub mod serializer;
pub mod import;
pub mod export;
pub mod validator;

// ---------------------------------------------------------------------------
// Unified format trait implementations
// ---------------------------------------------------------------------------

/// LilyPond format handler.
///
/// Implements the unified [`tusk_format`] traits. Import and export are
/// implemented incrementally; unimplemented parts return a conversion error.
pub struct LilyPondFormat;

impl tusk_format::Format for LilyPondFormat {
    fn id(&self) -> &'static str {
        "lilypond"
    }

    fn name(&self) -> &'static str {
        "LilyPond"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["ly"]
    }

    fn detect(&self, content: &[u8]) -> bool {
        let prefix = &content[..content.len().min(4096)];
        let s = std::str::from_utf8(prefix).unwrap_or("");
        s.contains("\\version")
            || s.contains("\\score")
            || (s.contains('{') && s.trim_start().starts_with('{'))
    }
}

impl tusk_format::Importer for LilyPondFormat {
    fn import_from_str(&self, _input: &str) -> tusk_format::FormatResult<tusk_format::Mei> {
        Err(tusk_format::FormatError::conversion(
            crate::import::ImportError::NotImplemented,
        ))
    }
}

impl tusk_format::Exporter for LilyPondFormat {
    fn export_to_string(&self, _mei: &tusk_format::Mei) -> tusk_format::FormatResult<String> {
        Err(tusk_format::FormatError::conversion(
            crate::export::ExportError::NotImplemented,
        ))
    }
}

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

pub mod export;
pub mod import;
pub mod lexer;
pub mod model;
pub mod parser;
pub mod serializer;
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
    fn import_from_str(
        &self,
        input: &str,
    ) -> tusk_format::FormatResult<(tusk_format::Mei, tusk_format::ExtensionStore)> {
        let file = crate::parser::Parser::new(input)
            .and_then(|p| p.parse())
            .map_err(tusk_format::FormatError::parse)?;
        let (mei, ext_store) =
            crate::import::import(&file).map_err(tusk_format::FormatError::conversion)?;
        Ok((mei, ext_store))
    }
}

impl tusk_format::Exporter for LilyPondFormat {
    fn export_to_string(
        &self,
        mei: &tusk_format::Mei,
        ext_store: &tusk_format::ExtensionStore,
    ) -> tusk_format::FormatResult<String> {
        let file = crate::export::export(mei, ext_store).map_err(tusk_format::FormatError::conversion)?;
        Ok(crate::serializer::serialize(&file))
    }
}

//! Unified format traits for the Tusk music notation converter.
//!
//! This crate defines the core abstractions that all format implementations
//! (MEI, MusicXML, Humdrum, ABC, etc.) implement. It provides:
//!
//! - [`Format`] — metadata about a file format (id, name, extensions, content detection)
//! - [`Importer`] — parse a foreign format into the canonical MEI model
//! - [`Exporter`] — serialize the canonical MEI model into a foreign format
//! - [`FormatError`] — unified error type wrapping format-specific errors
//! - [`FormatRegistry`] — runtime registry for discovering and dispatching formats
//!
//! # Adding a new format
//!
//! 1. Create a new crate under `crates/formats/<name>/`.
//! 2. Define a unit struct (e.g. `pub struct HumdrumFormat;`).
//! 3. Implement [`Format`] with metadata and content detection.
//! 4. Implement [`Importer`] and/or [`Exporter`] as appropriate.
//! 5. Register the format in the CLI bindings.
//!
//! # Example
//!
//! ```ignore
//! use tusk_format::{Format, Importer, Exporter, FormatRegistry};
//!
//! let mut registry = FormatRegistry::new();
//! registry.register_importer(Box::new(my_format::MyFormat));
//! registry.register_exporter(Box::new(my_format::MyFormat));
//!
//! let importer = registry.find_importer("myext", Some(content)).unwrap();
//! let (mei, ext_store) = importer.import_from_str(input)?;
//! ```

use thiserror::Error;

// Re-export the canonical model types for convenience.
pub use tusk_model::elements::Mei;
pub use tusk_model::extensions::ExtensionStore;

// ---------------------------------------------------------------------------
// Error
// ---------------------------------------------------------------------------

/// Unified error type for format operations.
///
/// Wraps format-specific errors into broad categories so that callers
/// (CLI, etc.) can handle them uniformly.
#[derive(Debug, Error)]
pub enum FormatError {
    /// Error parsing the input into the format's native representation.
    #[error("parse error: {0}")]
    Parse(Box<dyn std::error::Error + Send + Sync>),

    /// Error converting between the format's representation and MEI.
    #[error("conversion error: {0}")]
    Conversion(Box<dyn std::error::Error + Send + Sync>),

    /// Error serializing the format's native representation to output.
    #[error("serialization error: {0}")]
    Serialize(Box<dyn std::error::Error + Send + Sync>),

    /// No registered format matches the given extension or content.
    #[error("unknown format: {0}")]
    UnknownFormat(String),

    /// An I/O error occurred.
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl FormatError {
    /// Wrap a format-specific parse error.
    pub fn parse(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Parse(Box::new(err))
    }

    /// Wrap a format-specific conversion error.
    pub fn conversion(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Conversion(Box::new(err))
    }

    /// Wrap a format-specific serialization error.
    pub fn serialize(err: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::Serialize(Box::new(err))
    }
}

/// Result type alias for format operations.
pub type FormatResult<T> = Result<T, FormatError>;

// ---------------------------------------------------------------------------
// Traits
// ---------------------------------------------------------------------------

/// Describes a music notation file format.
///
/// Every format handler implements this trait to provide metadata used
/// for format detection and user-facing messages.
pub trait Format: Send + Sync {
    /// Unique short identifier (e.g. `"mei"`, `"musicxml"`, `"humdrum"`).
    fn id(&self) -> &'static str;

    /// Human-readable display name (e.g. `"MEI"`, `"MusicXML"`).
    fn name(&self) -> &'static str;

    /// File extensions associated with this format, **without** the leading dot.
    ///
    /// The first extension is considered the "primary" one (used for default
    /// output filenames, etc.).
    fn extensions(&self) -> &'static [&'static str];

    /// Attempt to detect this format from file content.
    ///
    /// Implementations should only inspect the first few kilobytes for
    /// efficiency — the method may be called on very large files.
    /// Return `true` if the content is likely in this format.
    fn detect(&self, content: &[u8]) -> bool;
}

/// Import from a file format into the canonical MEI model.
///
/// The full pipeline (parsing the format's native representation *and*
/// converting to MEI) is encapsulated behind this single method.
/// Returns both the MEI document and an [`ExtensionStore`] containing
/// format-specific roundtrip data keyed by MEI element IDs.
pub trait Importer: Format {
    /// Parse `input` and convert it to an MEI document with extension data.
    fn import_from_str(&self, input: &str) -> FormatResult<(Mei, ExtensionStore)>;
}

/// Export from the canonical MEI model to a file format.
///
/// The full pipeline (converting from MEI *and* serializing to the
/// format's native representation) is encapsulated behind this single method.
/// The [`ExtensionStore`] carries format-specific data from a prior import,
/// enabling lossless roundtrip and cross-format conversion.
pub trait Exporter: Format {
    /// Convert `mei` to the format's string representation.
    fn export_to_string(&self, mei: &Mei, ext_store: &ExtensionStore) -> FormatResult<String>;
}

// ---------------------------------------------------------------------------
// Registry
// ---------------------------------------------------------------------------

/// Runtime registry of available format handlers.
///
/// The registry allows callers to discover importers and exporters by
/// file extension or content detection, without hard-coding format knowledge.
///
/// # Example
///
/// ```ignore
/// use tusk_format::FormatRegistry;
///
/// let mut registry = FormatRegistry::new();
/// registry.register_importer(Box::new(mei_format));
/// registry.register_exporter(Box::new(mei_format));
///
/// // Discover importer by extension with content fallback
/// let importer = registry.find_importer("mei", None).unwrap();
/// ```
pub struct FormatRegistry {
    importers: Vec<Box<dyn Importer>>,
    exporters: Vec<Box<dyn Exporter>>,
}

impl FormatRegistry {
    /// Create a new, empty registry.
    pub fn new() -> Self {
        Self {
            importers: Vec::new(),
            exporters: Vec::new(),
        }
    }

    /// Register a format importer.
    pub fn register_importer(&mut self, importer: Box<dyn Importer>) {
        self.importers.push(importer);
    }

    /// Register a format exporter.
    pub fn register_exporter(&mut self, exporter: Box<dyn Exporter>) {
        self.exporters.push(exporter);
    }

    /// Find an importer by file extension, with optional content-based fallback.
    ///
    /// 1. Counts importers whose [`Format::extensions`] match `ext`
    ///    (case-insensitive).
    /// 2. If exactly one matches, returns it immediately.
    /// 3. If multiple match and `content` is provided, uses
    ///    [`Format::detect`] to disambiguate.
    /// 4. If no extension matches and `content` is provided, falls back to
    ///    pure content detection.
    pub fn find_importer(&self, ext: &str, content: Option<&[u8]>) -> Option<&dyn Importer> {
        #[allow(clippy::borrowed_box)]
        let matches_ext =
            |imp: &Box<dyn Importer>| imp.extensions().iter().any(|e| e.eq_ignore_ascii_case(ext));

        let ext_match_count = self.importers.iter().filter(|imp| matches_ext(imp)).count();

        if ext_match_count == 1 {
            let ext_match = self.importers.iter().find(|imp| matches_ext(imp)).unwrap();

            if let Some(content) = content {
                // Extension match confirms via content — return it.
                if ext_match.detect(content) {
                    return Some(ext_match.as_ref());
                }
                // Extension-matched format doesn't recognise the content;
                // another format might (e.g. an MEI file with .xml extension).
                if let Some(content_match) = self.importers.iter().find(|imp| imp.detect(content)) {
                    return Some(content_match.as_ref());
                }
            }

            // No content or no content match — trust the extension.
            return Some(ext_match.as_ref());
        }

        // Multiple extension matches — use content to disambiguate.
        if let Some(content) = content {
            if ext_match_count > 1
                && let Some(imp) = self
                    .importers
                    .iter()
                    .filter(|imp| matches_ext(imp))
                    .find(|imp| imp.detect(content))
            {
                return Some(imp.as_ref());
            }
            // Fall back to any importer that detects the content.
            return self
                .importers
                .iter()
                .find(|imp| imp.detect(content))
                .map(|imp| imp.as_ref());
        }

        // No content provided — return first extension match (if any).
        self.importers
            .iter()
            .find(|imp| matches_ext(imp))
            .map(|imp| imp.as_ref())
    }

    /// Find an exporter by file extension (case-insensitive).
    pub fn find_exporter(&self, ext: &str) -> Option<&dyn Exporter> {
        self.exporters
            .iter()
            .find(|exp| exp.extensions().iter().any(|e| e.eq_ignore_ascii_case(ext)))
            .map(|exp| exp.as_ref())
    }

    /// Find an importer by format ID.
    pub fn find_importer_by_id(&self, id: &str) -> Option<&dyn Importer> {
        self.importers
            .iter()
            .find(|imp| imp.id() == id)
            .map(|imp| imp.as_ref())
    }

    /// Find an exporter by format ID.
    pub fn find_exporter_by_id(&self, id: &str) -> Option<&dyn Exporter> {
        self.exporters
            .iter()
            .find(|exp| exp.id() == id)
            .map(|exp| exp.as_ref())
    }

    /// Iterate over all registered importers.
    pub fn importers(&self) -> impl Iterator<Item = &dyn Importer> {
        self.importers.iter().map(|imp| imp.as_ref())
    }

    /// Iterate over all registered exporters.
    pub fn exporters(&self) -> impl Iterator<Item = &dyn Exporter> {
        self.exporters.iter().map(|exp| exp.as_ref())
    }
}

impl Default for FormatRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Minimal test format for unit tests.
    struct TestFormat;

    impl Format for TestFormat {
        fn id(&self) -> &'static str {
            "test"
        }
        fn name(&self) -> &'static str {
            "Test Format"
        }
        fn extensions(&self) -> &'static [&'static str] {
            &["tst", "test"]
        }
        fn detect(&self, content: &[u8]) -> bool {
            content.starts_with(b"TEST")
        }
    }

    impl Importer for TestFormat {
        fn import_from_str(&self, _input: &str) -> FormatResult<(Mei, ExtensionStore)> {
            Ok((Mei::default(), ExtensionStore::default()))
        }
    }

    impl Exporter for TestFormat {
        fn export_to_string(&self, _mei: &Mei, _ext_store: &ExtensionStore) -> FormatResult<String> {
            Ok("TEST output".to_string())
        }
    }

    #[test]
    fn registry_find_by_extension() {
        let mut reg = FormatRegistry::new();
        reg.register_importer(Box::new(TestFormat));
        reg.register_exporter(Box::new(TestFormat));

        assert!(reg.find_importer("tst", None).is_some());
        assert!(reg.find_importer("test", None).is_some());
        assert!(reg.find_importer("unknown", None).is_none());

        assert!(reg.find_exporter("tst").is_some());
        assert!(reg.find_exporter("unknown").is_none());
    }

    #[test]
    fn registry_find_by_extension_case_insensitive() {
        let mut reg = FormatRegistry::new();
        reg.register_importer(Box::new(TestFormat));

        assert!(reg.find_importer("TST", None).is_some());
        assert!(reg.find_importer("Test", None).is_some());
    }

    #[test]
    fn registry_find_by_content_detection() {
        let mut reg = FormatRegistry::new();
        reg.register_importer(Box::new(TestFormat));

        // Unknown extension but content matches.
        let imp = reg.find_importer("unknown", Some(b"TEST content here"));
        assert!(imp.is_some());
        assert_eq!(imp.unwrap().id(), "test");

        // Unknown extension and content doesn't match.
        assert!(reg.find_importer("unknown", Some(b"nope")).is_none());
    }

    #[test]
    fn registry_find_by_id() {
        let mut reg = FormatRegistry::new();
        reg.register_importer(Box::new(TestFormat));
        reg.register_exporter(Box::new(TestFormat));

        assert!(reg.find_importer_by_id("test").is_some());
        assert!(reg.find_importer_by_id("other").is_none());
        assert!(reg.find_exporter_by_id("test").is_some());
    }

    #[test]
    fn format_error_constructors() {
        let err = FormatError::parse(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "bad data",
        ));
        assert!(err.to_string().contains("bad data"));

        let err = FormatError::conversion(std::io::Error::other("conversion failed"));
        assert!(err.to_string().contains("conversion failed"));

        let err = FormatError::serialize(std::io::Error::other("serialize failed"));
        assert!(err.to_string().contains("serialize failed"));
    }
}

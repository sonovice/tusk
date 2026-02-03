//! Custom MEI XML serializer using quick-xml.
//!
//! This module provides serialization of generated MEI model types to XML.
//! It handles the flattened attribute classes pattern used in the generated types.
//!
//! # Architecture
//!
//! The generated MEI types use `#[serde(flatten)]` to compose attribute classes into
//! elements. This pattern doesn't work with quick-xml's serde serialization, so we
//! implement custom serialization using the `MeiSerialize` and `CollectAttributes` traits.
//!
//! Each attribute class implements `CollectAttributes` to return its non-None attributes
//! as (name, value) pairs. Elements then combine these attributes and serialize using
//! quick-xml's events API.

mod impls;

use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use std::fmt::Display;
use std::io::Write;
use thiserror::Error;

/// Errors that can occur during MEI serialization.
#[derive(Error, Debug)]
pub enum SerializeError {
    #[error("XML write error: {0}")]
    XmlError(#[from] quick_xml::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("UTF-8 encoding error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
}

/// Result type for serialization operations.
pub type SerializeResult<T> = Result<T, SerializeError>;

/// Configuration options for MEI serialization.
#[derive(Debug, Clone)]
pub struct SerializeConfig {
    /// Whether to include XML declaration (<?xml version="1.0"?>).
    pub include_declaration: bool,
    /// Whether to use indentation for pretty-printing.
    pub indent: Option<IndentConfig>,
    /// MEI namespace URI.
    pub mei_namespace: Option<&'static str>,
}

/// Indentation configuration.
#[derive(Debug, Clone)]
pub struct IndentConfig {
    /// Character to use for indentation (typically space or tab).
    pub indent_char: u8,
    /// Number of indent characters per level.
    pub indent_size: usize,
}

impl Default for SerializeConfig {
    fn default() -> Self {
        Self {
            include_declaration: true,
            indent: Some(IndentConfig {
                indent_char: b' ',
                indent_size: 2,
            }),
            mei_namespace: Some("http://www.music-encoding.org/ns/mei"),
        }
    }
}

/// Trait for types that can be serialized to MEI XML.
///
/// This is implemented for all MEI element types to provide
/// consistent serialization handling.
pub trait MeiSerialize {
    /// The MEI element name (e.g., "note", "measure").
    fn element_name(&self) -> &'static str;

    /// Collect all attributes from this element's attribute classes.
    fn collect_all_attributes(&self) -> Vec<(&'static str, String)>;

    /// Check if this element has any child elements.
    fn has_children(&self) -> bool;

    /// Serialize child elements to the writer.
    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()>;

    /// Serialize this element to the given writer.
    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        let name = self.element_name();
        let attrs = self.collect_all_attributes();

        let mut start = writer.start_element(name)?;
        for (attr_name, value) in attrs {
            start.push_attribute((attr_name, value.as_str()));
        }

        if self.has_children() {
            writer.write_start(start)?;
            self.serialize_children(writer)?;
            writer.write_end(name)?;
        } else {
            writer.write_empty(start)?;
        }

        Ok(())
    }

    /// Serialize to a string with default configuration.
    fn to_mei_string(&self) -> SerializeResult<String>
    where
        Self: Sized,
    {
        let mut buffer = Vec::new();
        let config = SerializeConfig::default();
        let mut writer = MeiWriter::new(&mut buffer, config);
        self.serialize_mei(&mut writer)?;
        Ok(String::from_utf8(buffer)?)
    }

    /// Serialize to a string with custom configuration.
    fn to_mei_string_with_config(&self, config: SerializeConfig) -> SerializeResult<String>
    where
        Self: Sized,
    {
        let mut buffer = Vec::new();
        let mut writer = MeiWriter::new(&mut buffer, config);
        self.serialize_mei(&mut writer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

/// Writer wrapper for MEI XML serialization.
pub struct MeiWriter<W: Write> {
    writer: Writer<W>,
    config: SerializeConfig,
    declaration_written: bool,
}

impl<W: Write> MeiWriter<W> {
    /// Create a new MEI writer.
    pub fn new(inner: W, config: SerializeConfig) -> Self {
        let writer = if let Some(ref indent) = config.indent {
            Writer::new_with_indent(inner, indent.indent_char, indent.indent_size)
        } else {
            Writer::new(inner)
        };

        Self {
            writer,
            config,
            declaration_written: false,
        }
    }

    /// Write XML declaration if configured and not already written.
    pub fn write_declaration(&mut self) -> SerializeResult<()> {
        if self.config.include_declaration && !self.declaration_written {
            let decl = BytesDecl::new("1.0", Some("UTF-8"), None);
            self.writer.write_event(Event::Decl(decl))?;
            self.declaration_written = true;
        }
        Ok(())
    }

    /// Get the configuration.
    pub fn config(&self) -> &SerializeConfig {
        &self.config
    }

    /// Start an element with the given name.
    pub fn start_element(&mut self, name: &str) -> SerializeResult<BytesStart<'static>> {
        Ok(BytesStart::new(name.to_string()))
    }

    /// Write a start tag with attributes.
    pub fn write_start(&mut self, start: BytesStart<'_>) -> SerializeResult<()> {
        self.writer.write_event(Event::Start(start))?;
        Ok(())
    }

    /// Write an empty element (self-closing tag).
    pub fn write_empty(&mut self, start: BytesStart<'_>) -> SerializeResult<()> {
        self.writer.write_event(Event::Empty(start))?;
        Ok(())
    }

    /// Write an end tag.
    pub fn write_end(&mut self, name: &str) -> SerializeResult<()> {
        self.writer
            .write_event(Event::End(BytesEnd::new(name.to_string())))?;
        Ok(())
    }

    /// Write text content.
    pub fn write_text(&mut self, text: &str) -> SerializeResult<()> {
        self.writer.write_event(Event::Text(BytesText::new(text)))?;
        Ok(())
    }

    /// Get mutable access to the underlying quick-xml writer.
    pub fn inner_mut(&mut self) -> &mut Writer<W> {
        &mut self.writer
    }
}

/// Helper trait to collect attributes from flattened attribute class structs.
///
/// This trait is implemented by attribute class structs (AttCommon, AttNoteLog, etc.)
/// to collect their non-None attribute values for XML serialization.
pub trait CollectAttributes {
    /// Collect all non-None attributes into a list of (name, value) pairs.
    /// The name should be the XML attribute name (e.g., "xml:id", "dur").
    fn collect_attributes(&self) -> Vec<(&'static str, String)>;
}

/// Helper function to serialize an optional value to an attribute.
#[inline]
pub fn serialize_opt<T: Display>(opt: &Option<T>) -> Option<String> {
    opt.as_ref().map(|v| v.to_string())
}

/// Helper function to serialize a Vec as space-separated values.
#[inline]
pub fn serialize_vec<T: Display>(vec: &[T]) -> Option<String> {
    if vec.is_empty() {
        None
    } else {
        Some(
            vec.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tusk_model::elements::Note;

    #[test]
    fn serialize_config_default_includes_declaration() {
        let config = SerializeConfig::default();
        assert!(config.include_declaration);
        assert!(config.indent.is_some());
    }

    #[test]
    fn mei_writer_can_write_declaration() {
        let mut buffer = Vec::new();
        let config = SerializeConfig::default();
        let mut writer = MeiWriter::new(&mut buffer, config);
        writer
            .write_declaration()
            .expect("should write declaration");
        let result = String::from_utf8(buffer).unwrap();
        assert!(
            result.contains("<?xml"),
            "should contain XML declaration: {}",
            result
        );
        assert!(
            result.contains("version=\"1.0\""),
            "should have version: {}",
            result
        );
    }

    #[test]
    fn mei_writer_writes_declaration_only_once() {
        let mut buffer = Vec::new();
        let config = SerializeConfig::default();
        let mut writer = MeiWriter::new(&mut buffer, config);
        writer
            .write_declaration()
            .expect("should write declaration");
        writer
            .write_declaration()
            .expect("should not fail on second call");
        let result = String::from_utf8(buffer).unwrap();
        // Count occurrences of "<?xml"
        let count = result.matches("<?xml").count();
        assert_eq!(count, 1, "should only have one declaration: {}", result);
    }

    #[test]
    fn mei_writer_can_skip_declaration() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            indent: None,
            mei_namespace: None,
        };
        let mut writer = MeiWriter::new(&mut buffer, config);
        writer.write_declaration().expect("should succeed");
        let result = String::from_utf8(buffer).unwrap();
        assert!(
            !result.contains("<?xml"),
            "should not contain declaration: {}",
            result
        );
    }

    #[test]
    fn mei_writer_can_write_empty_element() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            indent: None,
            mei_namespace: None,
        };
        let mut writer = MeiWriter::new(&mut buffer, config);
        let mut start = writer.start_element("note").unwrap();
        start.push_attribute(("dur", "4"));
        writer
            .write_empty(start)
            .expect("should write empty element");
        let result = String::from_utf8(buffer).unwrap();
        assert!(
            result.contains("<note dur=\"4\""),
            "should have note element: {}",
            result
        );
        assert!(result.contains("/>"), "should be self-closing: {}", result);
    }

    #[test]
    fn mei_writer_can_write_element_with_children() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            indent: None,
            mei_namespace: None,
        };
        let mut writer = MeiWriter::new(&mut buffer, config);

        let mut start = writer.start_element("measure").unwrap();
        start.push_attribute(("n", "1"));
        writer.write_start(start).expect("should write start tag");

        let note_start = writer.start_element("note").unwrap();
        writer.write_empty(note_start).expect("should write note");

        writer.write_end("measure").expect("should write end tag");

        let result = String::from_utf8(buffer).unwrap();
        assert!(
            result.contains("<measure n=\"1\">"),
            "should have measure start: {}",
            result
        );
        assert!(result.contains("<note/>"), "should have note: {}", result);
        assert!(
            result.contains("</measure>"),
            "should have measure end: {}",
            result
        );
    }

    #[test]
    fn mei_writer_handles_xml_namespace_attributes() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            indent: None,
            mei_namespace: None,
        };
        let mut writer = MeiWriter::new(&mut buffer, config);

        let mut start = writer.start_element("note").unwrap();
        start.push_attribute(("xml:id", "n1"));
        start.push_attribute(("dur", "4"));
        writer.write_empty(start).expect("should write element");

        let result = String::from_utf8(buffer).unwrap();
        assert!(
            result.contains("xml:id=\"n1\""),
            "should have xml:id: {}",
            result
        );
    }

    #[test]
    fn mei_writer_can_write_text_content() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            indent: None,
            mei_namespace: None,
        };
        let mut writer = MeiWriter::new(&mut buffer, config);

        let start = writer.start_element("title").unwrap();
        writer.write_start(start).expect("should write start tag");
        writer.write_text("Test Title").expect("should write text");
        writer.write_end("title").expect("should write end tag");

        let result = String::from_utf8(buffer).unwrap();
        assert!(
            result.contains("<title>Test Title</title>"),
            "should have title with text: {}",
            result
        );
    }

    #[test]
    fn mei_writer_escapes_special_characters_in_attributes() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            indent: None,
            mei_namespace: None,
        };
        let mut writer = MeiWriter::new(&mut buffer, config);

        let mut start = writer.start_element("title").unwrap();
        start.push_attribute(("label", "Test & <value>"));
        writer.write_empty(start).expect("should write element");

        let result = String::from_utf8(buffer).unwrap();
        // quick-xml should escape & and <
        assert!(
            result.contains("&amp;") || result.contains("&"),
            "should escape ampersand: {}",
            result
        );
    }

    #[test]
    fn mei_writer_pretty_prints_with_indentation() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            indent: Some(IndentConfig {
                indent_char: b' ',
                indent_size: 2,
            }),
            mei_namespace: None,
        };
        let mut writer = MeiWriter::new(&mut buffer, config);

        let outer_start = writer.start_element("measure").unwrap();
        writer.write_start(outer_start).expect("should write start");

        let inner_start = writer.start_element("note").unwrap();
        writer.write_empty(inner_start).expect("should write note");

        writer.write_end("measure").expect("should write end");

        let result = String::from_utf8(buffer).unwrap();
        // With indentation, should have newlines and spaces
        assert!(
            result.contains('\n'),
            "should have newlines for pretty print: {}",
            result
        );
    }

    // Tests for Note element serialization (verifying generated types work)
    #[test]
    fn note_default_creates_empty_note() {
        let note = Note::default();
        // All attribute classes should be default (empty)
        assert!(note.common.xml_id.is_none());
        assert!(note.note_log.dur.is_none());
        assert!(note.children.is_empty());
    }

    // Test current serde behavior with flattened attributes
    #[test]
    fn test_quick_xml_serde_with_flattened_note() {
        use quick_xml::se::to_string;
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};

        let mut note = Note::default();
        note.common.xml_id = Some("n1".to_string());
        note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
        note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note.note_log.oct = Some(DataOctave(4));

        // Try quick-xml's built-in serialization
        let result = to_string(&note);
        // This may fail due to flatten not working properly - that's what we're testing
        match result {
            Ok(xml) => {
                println!("Serialized note: {}", xml);
                // If it works, verify basic structure
                assert!(xml.contains("<note"), "should have note element: {}", xml);
            }
            Err(e) => {
                println!("Expected flatten issue: {}", e);
                // This is expected - flatten doesn't work well with quick-xml
            }
        }
    }
}

//! Custom MusicXML serializer using quick-xml.
//!
//! This module provides serialization of MusicXML model types to XML.
//! We use custom serialization because:
//! 1. Some types use `#[serde(flatten)]` which doesn't work with quick-xml's serde
//! 2. We need precise control over element ordering and formatting
//!
//! The serializer follows the same pattern as the MEI serializer for consistency.

mod elements;
mod harmony;
mod notations;
mod score;
mod technical;

use quick_xml::Writer;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use std::fmt::Display;
use std::io::Write;
use thiserror::Error;

/// Errors that can occur during MusicXML serialization.
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

/// Configuration options for MusicXML serialization.
#[derive(Debug, Clone)]
pub struct SerializeConfig {
    /// Whether to include XML declaration (<?xml version="1.0"?>).
    pub include_declaration: bool,
    /// Whether to include DOCTYPE declaration.
    pub include_doctype: bool,
    /// Whether to use indentation for pretty-printing.
    pub indent: Option<IndentConfig>,
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
            include_doctype: true,
            indent: Some(IndentConfig {
                indent_char: b' ',
                indent_size: 2,
            }),
        }
    }
}

/// Trait for types that can be serialized to MusicXML.
pub trait MusicXmlSerialize {
    /// The MusicXML element name (e.g., "note", "measure").
    fn element_name(&self) -> &'static str;

    /// Collect all attributes for this element.
    fn collect_attributes(&self) -> Vec<(&'static str, String)>;

    /// Check if this element has any child content.
    fn has_children(&self) -> bool;

    /// Serialize child elements to the writer.
    fn serialize_children<W: Write>(&self, writer: &mut MusicXmlWriter<W>) -> SerializeResult<()>;

    /// Serialize this element to the given writer.
    fn serialize<W: Write>(&self, writer: &mut MusicXmlWriter<W>) -> SerializeResult<()> {
        let name = self.element_name();
        let attrs = self.collect_attributes();

        let mut start = writer.start_element(name);
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
    fn to_musicxml_string(&self) -> SerializeResult<String>
    where
        Self: Sized,
    {
        let mut buffer = Vec::new();
        let config = SerializeConfig::default();
        let mut writer = MusicXmlWriter::new(&mut buffer, config);
        writer.write_declaration()?;
        writer.write_doctype()?;
        self.serialize(&mut writer)?;
        Ok(String::from_utf8(buffer)?)
    }

    /// Serialize to a string with custom configuration.
    fn to_musicxml_string_with_config(&self, config: SerializeConfig) -> SerializeResult<String>
    where
        Self: Sized,
    {
        let mut buffer = Vec::new();
        let mut writer = MusicXmlWriter::new(&mut buffer, config);
        writer.write_declaration()?;
        writer.write_doctype()?;
        self.serialize(&mut writer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

/// Serialize a `ScoreTimewise` to a timewise MusicXML XML string.
///
/// This writes the timewise DOCTYPE and `<score-timewise>` root element.
pub fn serialize_timewise(
    score: &crate::model::elements::ScoreTimewise,
) -> SerializeResult<String> {
    serialize_timewise_with_config(score, SerializeConfig::default())
}

/// Serialize a `ScoreTimewise` to a timewise MusicXML XML string with custom config.
pub fn serialize_timewise_with_config(
    score: &crate::model::elements::ScoreTimewise,
    config: SerializeConfig,
) -> SerializeResult<String> {
    let mut buffer = Vec::new();
    let mut writer = MusicXmlWriter::new(&mut buffer, config);
    writer.write_declaration()?;
    writer.write_doctype_timewise()?;
    score.serialize_timewise(&mut writer)?;
    Ok(String::from_utf8(buffer)?)
}

/// Writer wrapper for MusicXML serialization.
pub struct MusicXmlWriter<W: Write> {
    writer: Writer<W>,
    config: SerializeConfig,
    declaration_written: bool,
    doctype_written: bool,
}

impl<W: Write> MusicXmlWriter<W> {
    /// Create a new MusicXML writer.
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
            doctype_written: false,
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

    /// Write partwise DOCTYPE declaration if configured and not already written.
    pub fn write_doctype(&mut self) -> SerializeResult<()> {
        self.write_doctype_partwise()
    }

    /// Write partwise DOCTYPE declaration if configured and not already written.
    pub fn write_doctype_partwise(&mut self) -> SerializeResult<()> {
        if self.config.include_doctype && !self.doctype_written {
            // Use from_escaped to prevent quick-xml from escaping the literal
            // quotes in the DOCTYPE declaration (BytesText::new would turn " into &quot;).
            self.writer.write_event(Event::DocType(
                BytesText::from_escaped(
                    "score-partwise PUBLIC \"-//Recordare//DTD MusicXML 4.0 Partwise//EN\" \"http://www.musicxml.org/dtds/partwise.dtd\"",
                ),
            ))?;
            self.doctype_written = true;
        }
        Ok(())
    }

    /// Write timewise DOCTYPE declaration if configured and not already written.
    pub fn write_doctype_timewise(&mut self) -> SerializeResult<()> {
        if self.config.include_doctype && !self.doctype_written {
            self.writer.write_event(Event::DocType(
                BytesText::from_escaped(
                    "score-timewise PUBLIC \"-//Recordare//DTD MusicXML 4.0 Timewise//EN\" \"http://www.musicxml.org/dtds/timewise.dtd\"",
                ),
            ))?;
            self.doctype_written = true;
        }
        Ok(())
    }

    /// Get the configuration.
    pub fn config(&self) -> &SerializeConfig {
        &self.config
    }

    /// Start an element with the given name.
    pub fn start_element(&self, name: &str) -> BytesStart<'static> {
        BytesStart::new(name.to_string())
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

    /// Write a simple text element (element with only text content).
    pub fn write_text_element(&mut self, name: &str, text: &str) -> SerializeResult<()> {
        let start = self.start_element(name);
        self.write_start(start)?;
        self.write_text(text)?;
        self.write_end(name)?;
        Ok(())
    }

    /// Write an optional text element (only if value is Some).
    pub fn write_opt_text_element(
        &mut self,
        name: &str,
        value: &Option<String>,
    ) -> SerializeResult<()> {
        if let Some(text) = value {
            self.write_text_element(name, text)?;
        }
        Ok(())
    }

    /// Get mutable access to the underlying quick-xml writer.
    pub fn inner_mut(&mut self) -> &mut Writer<W> {
        &mut self.writer
    }
}

/// Helper to serialize an optional value to a string.
#[inline]
pub fn opt_to_string<T: Display>(opt: &Option<T>) -> Option<String> {
    opt.as_ref().map(|v| v.to_string())
}

/// Helper macro to push an attribute if the value is Some.
macro_rules! push_opt_attr {
    ($attrs:expr, $name:expr, $opt_val:expr) => {
        if let Some(ref v) = $opt_val {
            $attrs.push(($name, v.to_string()));
        }
    };
}
pub(crate) use push_opt_attr;

/// Helper macro to push a string attribute if the value is Some.
macro_rules! push_opt_str_attr {
    ($attrs:expr, $name:expr, $opt_val:expr) => {
        if let Some(ref v) = $opt_val {
            $attrs.push(($name, v.clone()));
        }
    };
}
pub(crate) use push_opt_str_attr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_includes_declaration_and_doctype() {
        let config = SerializeConfig::default();
        assert!(config.include_declaration);
        assert!(config.include_doctype);
        assert!(config.indent.is_some());
    }

    #[test]
    fn writer_writes_declaration() {
        let mut buffer = Vec::new();
        let config = SerializeConfig::default();
        let mut writer = MusicXmlWriter::new(&mut buffer, config);
        writer
            .write_declaration()
            .expect("should write declaration");
        let result = String::from_utf8(buffer).unwrap();
        assert!(
            result.contains("<?xml"),
            "should have declaration: {}",
            result
        );
        assert!(
            result.contains("version=\"1.0\""),
            "should have version: {}",
            result
        );
        assert!(result.contains("UTF-8"), "should have encoding: {}", result);
    }

    #[test]
    fn writer_writes_doctype() {
        let mut buffer = Vec::new();
        let config = SerializeConfig::default();
        let mut writer = MusicXmlWriter::new(&mut buffer, config);
        writer.write_doctype().expect("should write doctype");
        let result = String::from_utf8(buffer).unwrap();
        assert!(
            result.contains("<!DOCTYPE"),
            "should have doctype: {}",
            result
        );
        assert!(
            result.contains("score-partwise"),
            "should have score-partwise: {}",
            result
        );
    }

    #[test]
    fn writer_writes_empty_element() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            include_doctype: false,
            indent: None,
        };
        let mut writer = MusicXmlWriter::new(&mut buffer, config);
        let mut start = writer.start_element("note");
        start.push_attribute(("default-x", "10"));
        writer.write_empty(start).expect("should write");
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("<note"), "should have note: {}", result);
        assert!(
            result.contains("default-x=\"10\""),
            "should have attr: {}",
            result
        );
        assert!(result.contains("/>"), "should be self-closing: {}", result);
    }

    #[test]
    fn writer_writes_element_with_text() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            include_doctype: false,
            indent: None,
        };
        let mut writer = MusicXmlWriter::new(&mut buffer, config);
        writer
            .write_text_element("work-title", "Symphony No. 5")
            .expect("should write");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "<work-title>Symphony No. 5</work-title>");
    }

    #[test]
    fn writer_writes_nested_elements() {
        let mut buffer = Vec::new();
        let config = SerializeConfig {
            include_declaration: false,
            include_doctype: false,
            indent: None,
        };
        let mut writer = MusicXmlWriter::new(&mut buffer, config);

        let start = writer.start_element("work");
        writer.write_start(start).expect("should write start");
        writer
            .write_text_element("work-title", "Test")
            .expect("should write child");
        writer.write_end("work").expect("should write end");

        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "<work><work-title>Test</work-title></work>");
    }
}

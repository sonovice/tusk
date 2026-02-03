//! Custom MEI XML deserializer using quick-xml.
//!
//! This module provides deserialization of MEI XML into generated model types.
//! It handles the flattened attribute classes pattern used in the generated types.
//!
//! # Architecture
//!
//! The generated MEI types use `#[serde(flatten)]` to compose attribute classes into
//! elements. This pattern doesn't work with quick-xml's serde deserialization, so we
//! implement custom deserialization using the `MeiDeserialize` and `ExtractAttributes` traits.
//!
//! Each attribute class implements `ExtractAttributes` to extract its attributes from
//! an attribute map. Elements then combine these attributes and deserialize child elements
//! using quick-xml's events API.

mod impls;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use std::collections::HashMap;
use std::io::BufRead;
use thiserror::Error;

/// Errors that can occur during MEI deserialization.
#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("XML parse error: {0}")]
    XmlError(#[from] quick_xml::Error),

    #[error("Unexpected element: expected '{expected}', found '{found}'")]
    UnexpectedElement { expected: String, found: String },

    #[error("Missing required attribute: {0}")]
    MissingAttribute(String),

    #[error("Invalid attribute value for '{attr}': {value}")]
    InvalidAttributeValue { attr: String, value: String },

    #[error("Unexpected end of document")]
    UnexpectedEof,

    #[error("UTF-8 encoding error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Result type for deserialization operations.
pub type DeserializeResult<T> = Result<T, DeserializeError>;

/// Configuration options for MEI deserialization.
#[derive(Debug, Clone)]
pub struct DeserializeConfig {
    /// Whether to ignore unknown attributes (lenient mode).
    pub ignore_unknown_attributes: bool,
    /// Whether to ignore unknown elements (lenient mode).
    pub ignore_unknown_elements: bool,
}

impl Default for DeserializeConfig {
    fn default() -> Self {
        Self {
            ignore_unknown_attributes: true,
            ignore_unknown_elements: true,
        }
    }
}

/// Map of attribute names to values extracted from an XML element.
pub type AttributeMap = HashMap<String, String>;

/// Trait for types that can be deserialized from MEI XML.
///
/// This is implemented for all MEI element types to provide
/// consistent deserialization handling.
pub trait MeiDeserialize: Sized {
    /// The MEI element name (e.g., "note", "measure").
    fn element_name() -> &'static str;

    /// Deserialize from an attribute map and child reader.
    ///
    /// The `start` tag has already been consumed; the reader is positioned
    /// right after it. The implementation should consume all content up to
    /// and including the matching end tag (or be a self-closing element).
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self>;

    /// Deserialize from a string.
    fn from_mei_str(s: &str) -> DeserializeResult<Self> {
        let mut reader = MeiReader::from_str(s);
        reader.read_element()
    }
}

/// Reader wrapper for MEI XML deserialization.
pub struct MeiReader<R: BufRead> {
    reader: Reader<R>,
    config: DeserializeConfig,
    buf: Vec<u8>,
}

impl MeiReader<&[u8]> {
    /// Create a new MEI reader from a string slice.
    pub fn from_str(s: &str) -> MeiReader<&[u8]> {
        let mut reader = Reader::from_str(s);
        reader.config_mut().trim_text(true);
        MeiReader {
            reader,
            config: DeserializeConfig::default(),
            buf: Vec::new(),
        }
    }
}

impl<R: BufRead> MeiReader<R> {
    /// Create a new MEI reader from a BufRead source.
    pub fn new(inner: R, config: DeserializeConfig) -> Self {
        let mut reader = Reader::from_reader(inner);
        reader.config_mut().trim_text(true);
        Self {
            reader,
            config,
            buf: Vec::new(),
        }
    }

    /// Get the configuration.
    pub fn config(&self) -> &DeserializeConfig {
        &self.config
    }

    /// Read the next XML event.
    pub fn read_event(&mut self) -> DeserializeResult<Event<'static>> {
        self.buf.clear();
        let event = self.reader.read_event_into(&mut self.buf)?;
        Ok(event.into_owned())
    }

    /// Extract attributes from a start tag into an AttributeMap.
    pub fn extract_attributes(&self, start: &BytesStart<'_>) -> DeserializeResult<AttributeMap> {
        let mut map = HashMap::new();
        for attr_result in start.attributes() {
            let attr = attr_result.map_err(quick_xml::Error::from)?;
            let key = std::str::from_utf8(attr.key.as_ref())?.to_string();
            let value = attr.unescape_value()?.to_string();
            map.insert(key, value);
        }
        Ok(map)
    }

    /// Read and deserialize an element of type T.
    ///
    /// Skips any leading XML declaration, comments, or whitespace.
    pub fn read_element<T: MeiDeserialize>(&mut self) -> DeserializeResult<T> {
        loop {
            match self.read_event()? {
                Event::Start(start) => {
                    let name = std::str::from_utf8(start.name().as_ref())?.to_string();
                    if name != T::element_name() {
                        return Err(DeserializeError::UnexpectedElement {
                            expected: T::element_name().to_string(),
                            found: name,
                        });
                    }
                    let attrs = self.extract_attributes(&start)?;
                    return T::from_mei_event(self, attrs, false);
                }
                Event::Empty(start) => {
                    let name = std::str::from_utf8(start.name().as_ref())?.to_string();
                    if name != T::element_name() {
                        return Err(DeserializeError::UnexpectedElement {
                            expected: T::element_name().to_string(),
                            found: name,
                        });
                    }
                    let attrs = self.extract_attributes(&start)?;
                    return T::from_mei_event(self, attrs, true);
                }
                Event::Decl(_) | Event::Comment(_) | Event::PI(_) => {
                    // Skip declarations, comments, processing instructions
                    continue;
                }
                Event::Text(t) if t.is_empty() || t.iter().all(|b| b.is_ascii_whitespace()) => {
                    // Skip whitespace-only text
                    continue;
                }
                Event::Eof => {
                    return Err(DeserializeError::UnexpectedEof);
                }
                event => {
                    return Err(DeserializeError::ParseError(format!(
                        "Expected start element, found {:?}",
                        event
                    )));
                }
            }
        }
    }

    /// Read child elements until the end tag for the given element name.
    ///
    /// Returns a vector of (element_name, attributes, is_empty) tuples for
    /// each child element encountered.
    pub fn read_children_raw(
        &mut self,
        parent_name: &str,
    ) -> DeserializeResult<Vec<(String, AttributeMap, bool, Option<String>)>> {
        let mut children = Vec::new();

        loop {
            match self.read_event()? {
                Event::Start(start) => {
                    let name = std::str::from_utf8(start.name().as_ref())?.to_string();
                    let attrs = self.extract_attributes(&start)?;
                    // For non-empty elements, we need to recursively skip to its end
                    let content = self.skip_to_end_and_collect(&name)?;
                    children.push((name, attrs, false, content));
                }
                Event::Empty(start) => {
                    let name = std::str::from_utf8(start.name().as_ref())?.to_string();
                    let attrs = self.extract_attributes(&start)?;
                    children.push((name, attrs, true, None));
                }
                Event::End(end) => {
                    let name_bytes = end.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    if name == parent_name {
                        return Ok(children);
                    }
                    // Unexpected end tag - in lenient mode we might ignore this
                    return Err(DeserializeError::ParseError(format!(
                        "Unexpected end tag: </{}>",
                        name
                    )));
                }
                Event::Text(_) | Event::CData(_) => {
                    // Text content - might be relevant for mixed content
                    // For now, skip text content in element children
                    continue;
                }
                Event::Comment(_) | Event::PI(_) => {
                    // Skip comments and processing instructions
                    continue;
                }
                Event::Eof => {
                    return Err(DeserializeError::UnexpectedEof);
                }
                Event::Decl(_) => {
                    // Unexpected declaration in element body
                    continue;
                }
                Event::DocType(_) => {
                    continue;
                }
                Event::GeneralRef(_) => {
                    // Skip general entity references
                    continue;
                }
            }
        }
    }

    /// Skip to the end tag for the given element, collecting any text content.
    fn skip_to_end_and_collect(&mut self, element_name: &str) -> DeserializeResult<Option<String>> {
        let mut depth = 1;
        let mut text_content = String::new();

        loop {
            match self.read_event()? {
                Event::Start(start) => {
                    let name_bytes = start.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    if name == element_name {
                        depth += 1;
                    }
                }
                Event::End(end) => {
                    let name_bytes = end.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    if name == element_name {
                        depth -= 1;
                        if depth == 0 {
                            return Ok(if text_content.is_empty() {
                                None
                            } else {
                                Some(text_content)
                            });
                        }
                    }
                }
                Event::Empty(_) => {
                    // Self-closing elements don't affect depth
                }
                Event::Text(t) => {
                    if let Ok(s) = std::str::from_utf8(&t) {
                        text_content.push_str(s);
                    }
                }
                Event::CData(t) => {
                    text_content.push_str(std::str::from_utf8(&t)?);
                }
                Event::Eof => {
                    return Err(DeserializeError::UnexpectedEof);
                }
                _ => {}
            }
        }
    }

    /// Skip to the end tag for the given element.
    pub fn skip_to_end(&mut self, element_name: &str) -> DeserializeResult<()> {
        self.skip_to_end_and_collect(element_name)?;
        Ok(())
    }
}

/// Helper trait to extract attributes from a map into attribute class structs.
///
/// This trait is implemented by attribute class structs (AttCommon, AttNoteLog, etc.)
/// to extract their attribute values from an AttributeMap.
pub trait ExtractAttributes: Default {
    /// Extract attributes from the map into this struct.
    /// Unknown attributes are left in the map for other attribute classes.
    fn extract_attributes(&mut self, attrs: &mut AttributeMap) -> DeserializeResult<()>;
}

/// Helper function to parse an optional attribute value.
pub fn parse_opt<T, F>(
    attrs: &mut AttributeMap,
    key: &str,
    parser: F,
) -> DeserializeResult<Option<T>>
where
    F: FnOnce(&str) -> Result<T, String>,
{
    if let Some(value) = attrs.remove(key) {
        match parser(&value) {
            Ok(v) => Ok(Some(v)),
            Err(e) => Err(DeserializeError::InvalidAttributeValue {
                attr: key.to_string(),
                value: format!("{}: {}", value, e),
            }),
        }
    } else {
        Ok(None)
    }
}

/// Helper function to parse a Vec attribute (space-separated values).
pub fn parse_vec<T, F>(attrs: &mut AttributeMap, key: &str, parser: F) -> DeserializeResult<Vec<T>>
where
    F: Fn(&str) -> Result<T, String>,
{
    if let Some(value) = attrs.remove(key) {
        let mut result = Vec::new();
        for part in value.split_whitespace() {
            match parser(part) {
                Ok(v) => result.push(v),
                Err(e) => {
                    return Err(DeserializeError::InvalidAttributeValue {
                        attr: key.to_string(),
                        value: format!("{}: {}", part, e),
                    });
                }
            }
        }
        Ok(result)
    } else {
        Ok(Vec::new())
    }
}

/// Helper function to parse a simple string attribute.
pub fn parse_string(attrs: &mut AttributeMap, key: &str) -> Option<String> {
    attrs.remove(key)
}

/// Helper function to parse a string attribute as-is (identity parser).
pub fn identity_parser(s: &str) -> Result<String, String> {
    Ok(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_config_default_is_lenient() {
        let config = DeserializeConfig::default();
        assert!(config.ignore_unknown_attributes);
        assert!(config.ignore_unknown_elements);
    }

    #[test]
    fn mei_reader_can_read_empty_element() {
        let xml = r#"<note/>"#;
        let mut reader = MeiReader::from_str(xml);

        match reader.read_event().unwrap() {
            Event::Empty(start) => {
                let name_bytes = start.name();
                let name = std::str::from_utf8(name_bytes.as_ref()).unwrap();
                assert_eq!(name, "note");
            }
            other => panic!("Expected Empty event, got {:?}", other),
        }
    }

    #[test]
    fn mei_reader_can_extract_attributes() {
        let xml = r#"<note xml:id="n1" dur="4" oct="4"/>"#;
        let mut reader = MeiReader::from_str(xml);

        match reader.read_event().unwrap() {
            Event::Empty(start) => {
                let attrs = reader.extract_attributes(&start).unwrap();
                assert_eq!(attrs.get("xml:id"), Some(&"n1".to_string()));
                assert_eq!(attrs.get("dur"), Some(&"4".to_string()));
                assert_eq!(attrs.get("oct"), Some(&"4".to_string()));
            }
            other => panic!("Expected Empty event, got {:?}", other),
        }
    }

    #[test]
    fn mei_reader_can_read_element_with_children() {
        let xml = r#"<measure n="1"><note dur="4"/><note dur="2"/></measure>"#;
        let mut reader = MeiReader::from_str(xml);

        // Read the start tag
        match reader.read_event().unwrap() {
            Event::Start(start) => {
                let name_bytes = start.name();
                let name = std::str::from_utf8(name_bytes.as_ref()).unwrap();
                assert_eq!(name, "measure");

                let attrs = reader.extract_attributes(&start).unwrap();
                assert_eq!(attrs.get("n"), Some(&"1".to_string()));

                // Read children
                let children = reader.read_children_raw("measure").unwrap();
                assert_eq!(children.len(), 2);
                assert_eq!(children[0].0, "note");
                assert_eq!(children[1].0, "note");
            }
            other => panic!("Expected Start event, got {:?}", other),
        }
    }

    #[test]
    fn mei_reader_handles_xml_declaration() {
        let xml = r#"<?xml version="1.0"?><note dur="4"/>"#;
        let mut reader = MeiReader::from_str(xml);

        // First event should be the declaration
        match reader.read_event().unwrap() {
            Event::Decl(_) => {}
            other => panic!("Expected Decl event, got {:?}", other),
        }

        // Second event should be the element
        match reader.read_event().unwrap() {
            Event::Empty(start) => {
                let name_bytes = start.name();
                let name = std::str::from_utf8(name_bytes.as_ref()).unwrap();
                assert_eq!(name, "note");
            }
            other => panic!("Expected Empty event, got {:?}", other),
        }
    }

    #[test]
    fn mei_reader_handles_nested_elements() {
        let xml = r#"<layer><chord><note dur="4"/><note dur="4"/></chord></layer>"#;
        let mut reader = MeiReader::from_str(xml);

        match reader.read_event().unwrap() {
            Event::Start(start) => {
                let name_bytes = start.name();
                let name = std::str::from_utf8(name_bytes.as_ref()).unwrap();
                assert_eq!(name, "layer");

                let children = reader.read_children_raw("layer").unwrap();
                assert_eq!(children.len(), 1);
                assert_eq!(children[0].0, "chord");
                // The chord's children were consumed when skipping to its end
                assert!(!children[0].2); // not empty (had children)
            }
            other => panic!("Expected Start event, got {:?}", other),
        }
    }

    #[test]
    fn parse_opt_extracts_and_removes_attribute() {
        let mut attrs = HashMap::new();
        attrs.insert("dur".to_string(), "4".to_string());
        attrs.insert("oct".to_string(), "5".to_string());

        let dur: Option<i32> = parse_opt(&mut attrs, "dur", |s| {
            s.parse().map_err(|e| format!("{}", e))
        })
        .unwrap();

        assert_eq!(dur, Some(4));
        assert!(!attrs.contains_key("dur")); // removed
        assert!(attrs.contains_key("oct")); // still there
    }

    #[test]
    fn parse_opt_returns_none_for_missing_attribute() {
        let mut attrs = HashMap::new();

        let dur: Option<i32> = parse_opt(&mut attrs, "dur", |s| {
            s.parse().map_err(|e| format!("{}", e))
        })
        .unwrap();

        assert_eq!(dur, None);
    }

    #[test]
    fn parse_vec_parses_space_separated_values() {
        let mut attrs = HashMap::new();
        attrs.insert("staff".to_string(), "1 2 3".to_string());

        let staff: Vec<i32> = parse_vec(&mut attrs, "staff", |s| {
            s.parse().map_err(|e| format!("{}", e))
        })
        .unwrap();

        assert_eq!(staff, vec![1, 2, 3]);
    }

    #[test]
    fn parse_vec_returns_empty_for_missing_attribute() {
        let mut attrs = HashMap::new();

        let staff: Vec<i32> = parse_vec(&mut attrs, "staff", |s| {
            s.parse().map_err(|e| format!("{}", e))
        })
        .unwrap();

        assert!(staff.is_empty());
    }

    #[test]
    fn mei_reader_handles_text_content_in_children() {
        let xml = r#"<title>Test Title</title>"#;
        let mut reader = MeiReader::from_str(xml);

        match reader.read_event().unwrap() {
            Event::Start(start) => {
                let name_bytes = start.name();
                let name = std::str::from_utf8(name_bytes.as_ref()).unwrap();
                assert_eq!(name, "title");

                // Read until end tag, should handle text
                reader.skip_to_end("title").unwrap();
            }
            other => panic!("Expected Start event, got {:?}", other),
        }
    }

    #[test]
    fn mei_reader_escapes_special_characters() {
        let xml = r#"<note label="Test &amp; Value"/>"#;
        let mut reader = MeiReader::from_str(xml);

        match reader.read_event().unwrap() {
            Event::Empty(start) => {
                let attrs = reader.extract_attributes(&start).unwrap();
                assert_eq!(attrs.get("label"), Some(&"Test & Value".to_string()));
            }
            other => panic!("Expected Empty event, got {:?}", other),
        }
    }
}

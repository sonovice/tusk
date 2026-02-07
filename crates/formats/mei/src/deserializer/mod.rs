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
//!
//! # Namespace Handling
//!
//! MEI documents use the namespace `http://www.music-encoding.org/ns/mei`. The deserializer:
//! - Accepts documents with or without namespace declarations (lenient mode)
//! - Strips namespace prefixes from element/attribute names when parsing
//! - Optionally validates namespace URIs (strict mode)

pub(crate) mod impls;

use quick_xml::Reader;
use quick_xml::escape::resolve_predefined_entity;
use quick_xml::events::{BytesRef, BytesStart, Event};
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

/// The expected MEI namespace URI.
pub const MEI_NAMESPACE: &str = "http://www.music-encoding.org/ns/mei";

/// Configuration options for MEI deserialization.
#[derive(Debug, Clone)]
pub struct DeserializeConfig {
    /// Whether to ignore unknown attributes (lenient mode).
    pub ignore_unknown_attributes: bool,
    /// Whether to ignore unknown elements (lenient mode).
    pub ignore_unknown_elements: bool,
    /// Whether to validate that the document uses the correct MEI namespace.
    /// When false (default), namespace declarations are accepted but not validated.
    pub validate_namespace: bool,
}

impl Default for DeserializeConfig {
    fn default() -> Self {
        Self {
            ignore_unknown_attributes: true,
            ignore_unknown_elements: true,
            validate_namespace: false,
        }
    }
}

impl DeserializeConfig {
    /// Create a strict configuration that validates namespace.
    pub fn strict() -> Self {
        Self {
            ignore_unknown_attributes: false,
            ignore_unknown_elements: false,
            validate_namespace: true,
        }
    }
}

/// Map of attribute names to values extracted from an XML element.
pub type AttributeMap = HashMap<String, String>;

/// Raw child element data: (element_name, attributes, is_empty, text_content).
pub type RawChildElement = (String, AttributeMap, bool, Option<String>);

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
    /// Pending mixed content event to return on next read_next_mixed_content call.
    pending_mixed_content_event: Option<MixedContent>,
    /// Flag indicating that mixed content reading hit the end tag.
    mixed_content_ended: bool,
}

impl MeiReader<&[u8]> {
    /// Create a new MEI reader from a string slice.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> MeiReader<&[u8]> {
        let reader = Reader::from_str(s);
        // Note: We don't use trim_text(true) because it interferes with proper
        // handling of entity references like &amp;. The text "Test &amp; Value"
        // would be split into ["Test ", "amp", " Value"] and trim_text would
        // strip the spaces, resulting in "Test&Value" instead of "Test & Value".
        // Instead, we manually skip whitespace-only text nodes where appropriate.
        MeiReader {
            reader,
            config: DeserializeConfig::default(),
            buf: Vec::new(),
            pending_mixed_content_event: None,
            mixed_content_ended: false,
        }
    }
}

impl<R: BufRead> MeiReader<R> {
    /// Create a new MEI reader from a BufRead source.
    pub fn new(inner: R, config: DeserializeConfig) -> Self {
        let reader = Reader::from_reader(inner);
        // Note: We don't use trim_text(true) - see from_str for explanation.
        Self {
            reader,
            config,
            buf: Vec::new(),
            pending_mixed_content_event: None,
            mixed_content_ended: false,
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
    /// Returns a vector of raw child element data for each child element encountered.
    pub fn read_children_raw(
        &mut self,
        parent_name: &str,
    ) -> DeserializeResult<Vec<RawChildElement>> {
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
                Event::GeneralRef(r) => {
                    resolve_xml_entity(&r, &mut text_content)?;
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

    /// Read text content until the end tag for the given element.
    ///
    /// This skips to the end tag while collecting any text content encountered.
    /// Returns the collected text if any, or None if the element had no text.
    pub fn read_text_until_end(&mut self, element_name: &str) -> DeserializeResult<Option<String>> {
        self.skip_to_end_and_collect(element_name)
    }

    /// Read and parse a child element of type T.
    ///
    /// This method reads the next element event and deserializes it into the requested type.
    /// The element must already have been detected via `read_children_raw` or similar;
    /// this method handles the actual parsing with proper recursion.
    pub fn read_child_element<T: MeiDeserialize>(
        &mut self,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<T> {
        T::from_mei_event(self, attrs, is_empty)
    }

    /// Read children of a parent element, returning start events for each child.
    ///
    /// Unlike `read_children_raw`, this method yields control back to the caller
    /// for each child start event, allowing proper recursive parsing.
    /// Returns None when the end tag for the parent is encountered.
    pub fn read_next_child_start(
        &mut self,
        parent_name: &str,
    ) -> DeserializeResult<Option<(String, AttributeMap, bool)>> {
        loop {
            match self.read_event()? {
                Event::Start(start) => {
                    let name = std::str::from_utf8(start.name().as_ref())?.to_string();
                    let attrs = self.extract_attributes(&start)?;
                    return Ok(Some((name, attrs, false)));
                }
                Event::Empty(start) => {
                    let name = std::str::from_utf8(start.name().as_ref())?.to_string();
                    let attrs = self.extract_attributes(&start)?;
                    return Ok(Some((name, attrs, true)));
                }
                Event::End(end) => {
                    let name_bytes = end.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    if name == parent_name {
                        return Ok(None);
                    }
                    // Unexpected end tag - in lenient mode we might ignore this
                    return Err(DeserializeError::ParseError(format!(
                        "Unexpected end tag: </{}>",
                        name
                    )));
                }
                Event::Text(_) | Event::CData(_) => {
                    // Skip text content
                    continue;
                }
                Event::Comment(_) | Event::PI(_) | Event::Decl(_) | Event::DocType(_) => {
                    continue;
                }
                Event::Eof => {
                    return Err(DeserializeError::UnexpectedEof);
                }
                Event::GeneralRef(_) => {
                    continue;
                }
            }
        }
    }

    /// Read the next child item from a mixed content element (text + elements).
    ///
    /// Returns either:
    /// - `Some(MixedContent::Element(name, attrs, is_empty))` for child elements
    /// - `Some(MixedContent::Text(text))` for text content
    /// - `None` when the end tag for the parent is encountered
    ///
    /// This is used for elements like `<publisher>` that can contain both text
    /// and child elements like `<corpName>`.
    ///
    /// Note: This collects consecutive text nodes and entity references into a single
    /// Text result. Entity references like `&amp;` are resolved to their character values.
    pub fn read_next_mixed_content(
        &mut self,
        parent_name: &str,
    ) -> DeserializeResult<Option<MixedContent>> {
        // Check if we previously hit the end tag after returning text
        if self.mixed_content_ended {
            self.mixed_content_ended = false;
            return Ok(None);
        }

        // Check if we have a pending event from a previous call
        if let Some(pending) = self.pending_mixed_content_event.take() {
            return Ok(Some(pending));
        }

        let mut text_accumulator = String::new();

        loop {
            match self.read_event()? {
                Event::Start(start) => {
                    let name = std::str::from_utf8(start.name().as_ref())?.to_string();
                    let attrs = self.extract_attributes(&start)?;
                    // If we have accumulated text, return it first and save element for next call
                    if !text_accumulator.trim().is_empty() {
                        self.pending_mixed_content_event =
                            Some(MixedContent::Element(name, attrs, false));
                        return Ok(Some(MixedContent::Text(text_accumulator)));
                    }
                    return Ok(Some(MixedContent::Element(name, attrs, false)));
                }
                Event::Empty(start) => {
                    let name = std::str::from_utf8(start.name().as_ref())?.to_string();
                    let attrs = self.extract_attributes(&start)?;
                    // If we have accumulated text, return it first and save element for next call
                    if !text_accumulator.trim().is_empty() {
                        self.pending_mixed_content_event =
                            Some(MixedContent::Element(name, attrs, true));
                        return Ok(Some(MixedContent::Text(text_accumulator)));
                    }
                    return Ok(Some(MixedContent::Element(name, attrs, true)));
                }
                Event::End(end) => {
                    let name_bytes = end.name();
                    let name = std::str::from_utf8(name_bytes.as_ref())?;
                    if name == parent_name {
                        // Return any accumulated text before signaling end
                        if !text_accumulator.trim().is_empty() {
                            // Mark that we've hit the end so next call returns None
                            self.mixed_content_ended = true;
                            return Ok(Some(MixedContent::Text(text_accumulator)));
                        }
                        return Ok(None);
                    }
                    // Unexpected end tag - in lenient mode we might ignore this
                    return Err(DeserializeError::ParseError(format!(
                        "Unexpected end tag: </{}>",
                        name
                    )));
                }
                Event::Text(t) => {
                    let text = std::str::from_utf8(&t)?.to_string();
                    text_accumulator.push_str(&text);
                    // Continue to collect more text/entity refs
                    continue;
                }
                Event::CData(t) => {
                    let text = std::str::from_utf8(&t)?.to_string();
                    text_accumulator.push_str(&text);
                    continue;
                }
                Event::GeneralRef(r) => {
                    resolve_xml_entity(&r, &mut text_accumulator)?;
                    continue;
                }
                Event::Comment(_) | Event::PI(_) | Event::Decl(_) | Event::DocType(_) => {
                    continue;
                }
                Event::Eof => {
                    return Err(DeserializeError::UnexpectedEof);
                }
            }
        }
    }

    /// Check if mixed content reading has ended (used after returning final text).
    pub fn mixed_content_has_ended(&mut self) -> bool {
        if self.mixed_content_ended {
            self.mixed_content_ended = false;
            true
        } else {
            false
        }
    }
}

/// Content returned from mixed content reading.
#[derive(Debug)]
pub enum MixedContent {
    /// An element with name, attributes, and empty flag
    Element(String, AttributeMap, bool),
    /// Text content
    Text(String),
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

/// Resolve an XML entity reference and append the result to a string buffer.
///
/// Handles predefined entities (`&amp;`, `&lt;`, etc.), character references
/// (`&#x30;`, `&#49;`), and unknown entities (preserved as-is).
pub fn resolve_xml_entity(r: &BytesRef<'_>, buf: &mut String) -> Result<(), std::str::Utf8Error> {
    let entity_name = std::str::from_utf8(r)?;
    if let Some(resolved) = resolve_predefined_entity(entity_name) {
        buf.push_str(resolved);
    } else if let Ok(Some(ch)) = r.resolve_char_ref() {
        buf.push(ch);
    } else {
        buf.push('&');
        buf.push_str(entity_name);
        buf.push(';');
    }
    Ok(())
}

/// Strip namespace prefix from an element or attribute name.
///
/// MEI documents may use namespace prefixes like `mei:note` instead of just `note`.
/// This function removes such prefixes, but preserves special prefixes like `xml:`.
///
/// Examples:
/// - `"mei:note"` → `"note"`
/// - `"note"` → `"note"`
/// - `"xml:id"` → `"xml:id"` (preserved)
/// - `"xlink:href"` → `"xlink:href"` (preserved)
pub fn strip_namespace_prefix(name: &str) -> &str {
    // Preserve xml: and xlink: prefixes as they have semantic meaning
    if name.starts_with("xml:") || name.starts_with("xlink:") {
        return name;
    }

    // Strip any other namespace prefix
    if let Some(pos) = name.find(':') {
        &name[pos + 1..]
    } else {
        name
    }
}

/// Check if an attribute is a namespace declaration.
pub fn is_namespace_declaration(name: &str) -> bool {
    name == "xmlns" || name.starts_with("xmlns:")
}

/// Extract namespace declarations from an attribute map.
///
/// Returns the extracted namespaces as (prefix, uri) pairs where prefix is None
/// for the default namespace.
pub fn extract_namespaces(attrs: &mut AttributeMap) -> Vec<(Option<String>, String)> {
    let mut namespaces = Vec::new();
    let ns_keys: Vec<String> = attrs
        .keys()
        .filter(|k| is_namespace_declaration(k))
        .cloned()
        .collect();

    for key in ns_keys {
        if let Some(uri) = attrs.remove(&key) {
            let prefix = if key == "xmlns" {
                None
            } else {
                Some(key.strip_prefix("xmlns:").unwrap_or(&key).to_string())
            };
            namespaces.push((prefix, uri));
        }
    }

    namespaces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_config_default_is_lenient() {
        let config = DeserializeConfig::default();
        assert!(config.ignore_unknown_attributes);
        assert!(config.ignore_unknown_elements);
        assert!(!config.validate_namespace);
    }

    #[test]
    fn deserialize_config_strict_validates_namespace() {
        let config = DeserializeConfig::strict();
        assert!(!config.ignore_unknown_attributes);
        assert!(!config.ignore_unknown_elements);
        assert!(config.validate_namespace);
    }

    // ============================================================================
    // Namespace handling tests
    // ============================================================================

    #[test]
    fn strip_namespace_prefix_removes_mei_prefix() {
        assert_eq!(strip_namespace_prefix("mei:note"), "note");
        assert_eq!(strip_namespace_prefix("mei:measure"), "measure");
    }

    #[test]
    fn strip_namespace_prefix_preserves_unprefixed_names() {
        assert_eq!(strip_namespace_prefix("note"), "note");
        assert_eq!(strip_namespace_prefix("dur"), "dur");
    }

    #[test]
    fn strip_namespace_prefix_preserves_xml_prefix() {
        assert_eq!(strip_namespace_prefix("xml:id"), "xml:id");
        assert_eq!(strip_namespace_prefix("xml:base"), "xml:base");
        assert_eq!(strip_namespace_prefix("xml:lang"), "xml:lang");
    }

    #[test]
    fn strip_namespace_prefix_preserves_xlink_prefix() {
        assert_eq!(strip_namespace_prefix("xlink:href"), "xlink:href");
        assert_eq!(strip_namespace_prefix("xlink:actuate"), "xlink:actuate");
    }

    #[test]
    fn is_namespace_declaration_detects_xmlns() {
        assert!(is_namespace_declaration("xmlns"));
        assert!(is_namespace_declaration("xmlns:mei"));
        assert!(is_namespace_declaration("xmlns:xlink"));
        assert!(!is_namespace_declaration("xml:id"));
        assert!(!is_namespace_declaration("dur"));
    }

    #[test]
    fn extract_namespaces_extracts_default_namespace() {
        let mut attrs = HashMap::new();
        attrs.insert("xmlns".to_string(), MEI_NAMESPACE.to_string());
        attrs.insert("meiversion".to_string(), "5.1".to_string());

        let namespaces = extract_namespaces(&mut attrs);

        assert_eq!(namespaces.len(), 1);
        assert_eq!(namespaces[0].0, None); // default namespace
        assert_eq!(namespaces[0].1, MEI_NAMESPACE);

        // xmlns should be removed, meiversion should remain
        assert!(!attrs.contains_key("xmlns"));
        assert!(attrs.contains_key("meiversion"));
    }

    #[test]
    fn extract_namespaces_extracts_prefixed_namespaces() {
        let mut attrs = HashMap::new();
        attrs.insert("xmlns".to_string(), MEI_NAMESPACE.to_string());
        attrs.insert(
            "xmlns:xlink".to_string(),
            "http://www.w3.org/1999/xlink".to_string(),
        );
        attrs.insert("meiversion".to_string(), "5.1".to_string());

        let namespaces = extract_namespaces(&mut attrs);

        assert_eq!(namespaces.len(), 2);

        // Check default namespace
        let default_ns = namespaces.iter().find(|(p, _)| p.is_none());
        assert!(default_ns.is_some());
        assert_eq!(default_ns.unwrap().1, MEI_NAMESPACE);

        // Check xlink namespace
        let xlink_ns = namespaces
            .iter()
            .find(|(p, _)| p.as_deref() == Some("xlink"));
        assert!(xlink_ns.is_some());
        assert_eq!(xlink_ns.unwrap().1, "http://www.w3.org/1999/xlink");

        // Both xmlns attrs should be removed
        assert!(!attrs.contains_key("xmlns"));
        assert!(!attrs.contains_key("xmlns:xlink"));
    }

    #[test]
    fn mei_reader_handles_document_with_namespace() {
        let xml = r#"<?xml version="1.0"?><mei xmlns="http://www.music-encoding.org/ns/mei" meiversion="5.1"><music/></mei>"#;
        let mut reader = MeiReader::from_str(xml);

        // Skip declaration
        match reader.read_event().unwrap() {
            Event::Decl(_) => {}
            other => panic!("Expected Decl, got {:?}", other),
        }

        // Read mei element
        match reader.read_event().unwrap() {
            Event::Start(start) => {
                let name_bytes = start.name();
                let name = std::str::from_utf8(name_bytes.as_ref()).unwrap();
                assert_eq!(name, "mei");

                let mut attrs = reader.extract_attributes(&start).unwrap();

                // Should have xmlns and meiversion
                assert!(attrs.contains_key("xmlns"));
                assert!(attrs.contains_key("meiversion"));

                // Extract namespaces
                let namespaces = extract_namespaces(&mut attrs);
                assert_eq!(namespaces.len(), 1);
                assert_eq!(namespaces[0].1, MEI_NAMESPACE);

                // xmlns should be removed, meiversion should remain
                assert!(!attrs.contains_key("xmlns"));
                assert_eq!(attrs.get("meiversion"), Some(&"5.1".to_string()));
            }
            other => panic!("Expected Start, got {:?}", other),
        }
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

    #[test]
    fn mei_reader_handles_ampersand_in_text_content() {
        let xml = r#"<title>Test &amp; Value</title>"#;
        let mut reader = MeiReader::from_str(xml);

        match reader.read_event().unwrap() {
            Event::Start(_) => {
                // Read text content
                let text = reader.read_text_until_end("title").unwrap();
                assert_eq!(text, Some("Test & Value".to_string()));
            }
            other => panic!("Expected Start event, got {:?}", other),
        }
    }
}

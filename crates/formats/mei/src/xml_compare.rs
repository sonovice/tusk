//! XML comparison utilities for MEI roundtrip testing.
//!
//! Provides tree-based XML comparison that handles acceptable differences:
//! - Whitespace and formatting
//! - Attribute ordering
//! - Namespace prefix variations
//! - XML declaration differences
//! - Empty vs self-closing element syntax

use quick_xml::Reader;
use quick_xml::events::Event;
use std::collections::HashMap;

/// A canonical representation of an XML element for comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalElement {
    /// Element name without namespace prefix.
    pub name: String,
    /// Attributes as a map (order-independent).
    pub attributes: HashMap<String, String>,
    /// Child elements in document order.
    pub children: Vec<CanonicalNode>,
}

/// A node in the canonical XML tree.
#[derive(Debug, Clone, PartialEq)]
pub enum CanonicalNode {
    /// An element node.
    Element(CanonicalElement),
    /// Text content (whitespace-normalized).
    Text(String),
}

/// Errors that can occur during XML comparison.
#[derive(Debug)]
pub enum CompareError {
    /// XML parsing error.
    ParseError(String),
    /// Structural differences found.
    Differences(Vec<Difference>),
}

impl std::fmt::Display for CompareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompareError::ParseError(msg) => write!(f, "XML parse error: {}", msg),
            CompareError::Differences(diffs) => {
                writeln!(f, "Found {} differences:", diffs.len())?;
                for diff in diffs {
                    writeln!(f, "  - {}", diff)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for CompareError {}

/// A single difference between two XML documents.
#[derive(Debug, Clone)]
pub struct Difference {
    /// Path to the differing element (e.g., "mei/music/body/mdiv").
    pub path: String,
    /// Description of the difference.
    pub description: String,
}

impl std::fmt::Display for Difference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "at {}: {}", self.path, self.description)
    }
}

/// Strip namespace prefix from an element or attribute name.
///
/// Examples:
/// - `mei:note` -> `note`
/// - `xml:id` -> `xml:id` (preserved - xml namespace is special)
/// - `note` -> `note`
fn strip_namespace_prefix(name: &str) -> &str {
    if name.starts_with("xml:") {
        // Preserve xml: prefix (for xml:id, xml:lang, etc.)
        name
    } else if let Some(pos) = name.find(':') {
        &name[pos + 1..]
    } else {
        name
    }
}

/// Parse XML string into a canonical tree representation.
pub fn parse_canonical(xml: &str) -> Result<CanonicalElement, CompareError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<CanonicalElement> = Vec::new();
    let mut root: Option<CanonicalElement> = None;

    loop {
        let event = reader
            .read_event_into(&mut buf)
            .map_err(|e| CompareError::ParseError(e.to_string()))?;

        match event {
            Event::Start(e) => {
                let name_bytes = e.name();
                let name_str = std::str::from_utf8(name_bytes.as_ref())
                    .map_err(|e| CompareError::ParseError(e.to_string()))?;
                let name = strip_namespace_prefix(name_str).to_string();

                let mut attributes = HashMap::new();
                for attr_result in e.attributes() {
                    let attr = attr_result.map_err(|e| CompareError::ParseError(e.to_string()))?;
                    let key = std::str::from_utf8(attr.key.as_ref())
                        .map_err(|e| CompareError::ParseError(e.to_string()))?;
                    // Skip xmlns declarations - they're not semantic content
                    if key.starts_with("xmlns") {
                        continue;
                    }
                    let key = strip_namespace_prefix(key).to_string();
                    let value = attr
                        .unescape_value()
                        .map_err(|e| CompareError::ParseError(e.to_string()))?
                        .to_string();
                    attributes.insert(key, value);
                }

                let element = CanonicalElement {
                    name,
                    attributes,
                    children: Vec::new(),
                };
                stack.push(element);
            }

            Event::Empty(e) => {
                let name_bytes = e.name();
                let name_str = std::str::from_utf8(name_bytes.as_ref())
                    .map_err(|e| CompareError::ParseError(e.to_string()))?;
                let name = strip_namespace_prefix(name_str).to_string();

                let mut attributes = HashMap::new();
                for attr_result in e.attributes() {
                    let attr = attr_result.map_err(|e| CompareError::ParseError(e.to_string()))?;
                    let key = std::str::from_utf8(attr.key.as_ref())
                        .map_err(|e| CompareError::ParseError(e.to_string()))?;
                    if key.starts_with("xmlns") {
                        continue;
                    }
                    let key = strip_namespace_prefix(key).to_string();
                    let value = attr
                        .unescape_value()
                        .map_err(|e| CompareError::ParseError(e.to_string()))?
                        .to_string();
                    attributes.insert(key, value);
                }

                let element = CanonicalElement {
                    name,
                    attributes,
                    children: Vec::new(),
                };

                if let Some(parent) = stack.last_mut() {
                    parent.children.push(CanonicalNode::Element(element));
                } else {
                    root = Some(element);
                }
            }

            Event::End(_) => {
                if let Some(element) = stack.pop() {
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(CanonicalNode::Element(element));
                    } else {
                        root = Some(element);
                    }
                }
            }

            Event::Text(e) => {
                let text =
                    std::str::from_utf8(&e).map_err(|e| CompareError::ParseError(e.to_string()))?;
                // Normalize whitespace in text content
                let text = text.trim().to_string();
                if !text.is_empty() && let Some(parent) = stack.last_mut() {
                    parent.children.push(CanonicalNode::Text(text));
                }
            }

            Event::CData(e) => {
                let text = std::str::from_utf8(&e)
                    .map_err(|e| CompareError::ParseError(e.to_string()))?
                    .trim()
                    .to_string();
                if !text.is_empty() && let Some(parent) = stack.last_mut() {
                    parent.children.push(CanonicalNode::Text(text));
                }
            }

            Event::Eof => break,

            // Skip declarations, comments, processing instructions, general references
            Event::Decl(_)
            | Event::Comment(_)
            | Event::PI(_)
            | Event::DocType(_)
            | Event::GeneralRef(_) => {}
        }

        buf.clear();
    }

    root.ok_or_else(|| CompareError::ParseError("No root element found".to_string()))
}

/// Compare two canonical elements and collect all differences.
fn compare_elements(
    elem1: &CanonicalElement,
    elem2: &CanonicalElement,
    path: &str,
    diffs: &mut Vec<Difference>,
) {
    let current_path = if path.is_empty() {
        elem1.name.clone()
    } else {
        format!("{}/{}", path, elem1.name)
    };

    // Compare element names
    if elem1.name != elem2.name {
        diffs.push(Difference {
            path: current_path.clone(),
            description: format!(
                "element name mismatch: '{}' vs '{}'",
                elem1.name, elem2.name
            ),
        });
        return; // No point comparing children if names differ
    }

    // Compare attributes
    compare_attributes(&elem1.attributes, &elem2.attributes, &current_path, diffs);

    // Compare children
    compare_children(&elem1.children, &elem2.children, &current_path, diffs);
}

/// Compare two attribute maps.
fn compare_attributes(
    attrs1: &HashMap<String, String>,
    attrs2: &HashMap<String, String>,
    path: &str,
    diffs: &mut Vec<Difference>,
) {
    // Find attributes in first but not second
    for (key, value1) in attrs1 {
        match attrs2.get(key) {
            Some(value2) => {
                if value1 != value2 {
                    diffs.push(Difference {
                        path: path.to_string(),
                        description: format!(
                            "attribute '{}' value mismatch: '{}' vs '{}'",
                            key, value1, value2
                        ),
                    });
                }
            }
            None => {
                diffs.push(Difference {
                    path: path.to_string(),
                    description: format!(
                        "attribute '{}' missing in output (was '{}')",
                        key, value1
                    ),
                });
            }
        }
    }

    // Find attributes in second but not first
    for key in attrs2.keys() {
        if !attrs1.contains_key(key) {
            diffs.push(Difference {
                path: path.to_string(),
                description: format!(
                    "unexpected attribute '{}' in output (value '{}')",
                    key,
                    attrs2.get(key).unwrap()
                ),
            });
        }
    }
}

/// Compare two lists of child nodes.
fn compare_children(
    children1: &[CanonicalNode],
    children2: &[CanonicalNode],
    path: &str,
    diffs: &mut Vec<Difference>,
) {
    let mut idx1 = 0;
    let mut idx2 = 0;

    while idx1 < children1.len() && idx2 < children2.len() {
        match (&children1[idx1], &children2[idx2]) {
            (CanonicalNode::Element(e1), CanonicalNode::Element(e2)) => {
                compare_elements(e1, e2, path, diffs);
                idx1 += 1;
                idx2 += 1;
            }
            (CanonicalNode::Text(t1), CanonicalNode::Text(t2)) => {
                if t1 != t2 {
                    diffs.push(Difference {
                        path: path.to_string(),
                        description: format!("text content mismatch: '{}' vs '{}'", t1, t2),
                    });
                }
                idx1 += 1;
                idx2 += 1;
            }
            (CanonicalNode::Element(e), CanonicalNode::Text(t)) => {
                diffs.push(Difference {
                    path: path.to_string(),
                    description: format!(
                        "node type mismatch at position {}: expected element '{}', found text '{}'",
                        idx1, e.name, t
                    ),
                });
                idx1 += 1;
                idx2 += 1;
            }
            (CanonicalNode::Text(t), CanonicalNode::Element(e)) => {
                diffs.push(Difference {
                    path: path.to_string(),
                    description: format!(
                        "node type mismatch at position {}: expected text '{}', found element '{}'",
                        idx1, t, e.name
                    ),
                });
                idx1 += 1;
                idx2 += 1;
            }
        }
    }

    // Report remaining children in first
    while idx1 < children1.len() {
        match &children1[idx1] {
            CanonicalNode::Element(e) => {
                diffs.push(Difference {
                    path: path.to_string(),
                    description: format!("element '{}' missing in output", e.name),
                });
            }
            CanonicalNode::Text(t) => {
                diffs.push(Difference {
                    path: path.to_string(),
                    description: format!("text '{}' missing in output", t),
                });
            }
        }
        idx1 += 1;
    }

    // Report remaining children in second
    while idx2 < children2.len() {
        match &children2[idx2] {
            CanonicalNode::Element(e) => {
                diffs.push(Difference {
                    path: path.to_string(),
                    description: format!("unexpected element '{}' in output", e.name),
                });
            }
            CanonicalNode::Text(t) => {
                diffs.push(Difference {
                    path: path.to_string(),
                    description: format!("unexpected text '{}' in output", t),
                });
            }
        }
        idx2 += 1;
    }
}

/// Compare two XML strings and return any differences.
///
/// Returns Ok(()) if the XML documents are semantically equivalent,
/// or Err with a list of differences if they differ.
pub fn compare_xml(xml1: &str, xml2: &str) -> Result<(), CompareError> {
    let tree1 = parse_canonical(xml1)?;
    let tree2 = parse_canonical(xml2)?;

    let mut diffs = Vec::new();
    compare_elements(&tree1, &tree2, "", &mut diffs);

    if diffs.is_empty() {
        Ok(())
    } else {
        Err(CompareError::Differences(diffs))
    }
}

/// Get a detailed comparison report between two XML strings.
///
/// Always returns a list (possibly empty) of differences found.
pub fn get_differences(xml1: &str, xml2: &str) -> Result<Vec<Difference>, CompareError> {
    let tree1 = parse_canonical(xml1)?;
    let tree2 = parse_canonical(xml2)?;

    let mut diffs = Vec::new();
    compare_elements(&tree1, &tree2, "", &mut diffs);
    Ok(diffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_namespace_prefix() {
        assert_eq!(strip_namespace_prefix("mei:note"), "note");
        assert_eq!(strip_namespace_prefix("note"), "note");
        assert_eq!(strip_namespace_prefix("xml:id"), "xml:id");
        assert_eq!(strip_namespace_prefix("foo:bar:baz"), "bar:baz");
    }

    #[test]
    fn test_identical_xml() {
        let xml1 = r#"<root><child attr="value">text</child></root>"#;
        let xml2 = r#"<root><child attr="value">text</child></root>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_different_attribute_order() {
        let xml1 = r#"<note pname="c" oct="4"/>"#;
        let xml2 = r#"<note oct="4" pname="c"/>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_whitespace_differences() {
        let xml1 = r#"<root>
            <child>text</child>
        </root>"#;
        let xml2 = r#"<root><child>text</child></root>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_namespace_prefix_differences() {
        let xml1 = r#"<mei:note xmlns:mei="http://example.com">text</mei:note>"#;
        let xml2 = r#"<note>text</note>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_xml_declaration_ignored() {
        let xml1 = r#"<?xml version="1.0" encoding="UTF-8"?><root/>"#;
        let xml2 = r#"<root/>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_empty_vs_self_closing() {
        let xml1 = r#"<root><child></child></root>"#;
        let xml2 = r#"<root><child/></root>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_missing_attribute() {
        let xml1 = r#"<note pname="c" oct="4"/>"#;
        let xml2 = r#"<note pname="c"/>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err());
        if let Err(CompareError::Differences(diffs)) = result {
            assert_eq!(diffs.len(), 1);
            assert!(diffs[0].description.contains("oct"));
        }
    }

    #[test]
    fn test_different_attribute_value() {
        let xml1 = r#"<note pname="c"/>"#;
        let xml2 = r#"<note pname="d"/>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err());
        if let Err(CompareError::Differences(diffs)) = result {
            assert_eq!(diffs.len(), 1);
            assert!(diffs[0].description.contains("pname"));
            assert!(diffs[0].description.contains("'c'"));
            assert!(diffs[0].description.contains("'d'"));
        }
    }

    #[test]
    fn test_missing_element() {
        let xml1 = r#"<root><child1/><child2/></root>"#;
        let xml2 = r#"<root><child1/></root>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err());
        if let Err(CompareError::Differences(diffs)) = result {
            assert!(!diffs.is_empty());
            assert!(diffs.iter().any(|d| d.description.contains("child2")));
        }
    }

    #[test]
    fn test_extra_element() {
        let xml1 = r#"<root><child1/></root>"#;
        let xml2 = r#"<root><child1/><child2/></root>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err());
        if let Err(CompareError::Differences(diffs)) = result {
            assert!(!diffs.is_empty());
            assert!(diffs.iter().any(|d| d.description.contains("unexpected")));
        }
    }

    #[test]
    fn test_different_text_content() {
        let xml1 = r#"<root>hello</root>"#;
        let xml2 = r#"<root>world</root>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err());
    }

    #[test]
    fn test_preserves_xml_id() {
        let xml1 = r#"<note xml:id="n1"/>"#;
        let xml2 = r#"<note xml:id="n1"/>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_xmlns_ignored() {
        let xml1 = r#"<mei xmlns="http://www.music-encoding.org/ns/mei"><note/></mei>"#;
        let xml2 = r#"<mei><note/></mei>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_deeply_nested() {
        let xml1 = r#"<a><b><c attr="v"><d>text</d></c></b></a>"#;
        let xml2 = r#"<a><b><c attr="v"><d>text</d></c></b></a>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_deeply_nested_difference() {
        let xml1 = r#"<a><b><c attr="v1"/></b></a>"#;
        let xml2 = r#"<a><b><c attr="v2"/></b></a>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err());
        if let Err(CompareError::Differences(diffs)) = result {
            assert_eq!(diffs.len(), 1);
            assert!(diffs[0].path.contains("a/b/c"));
        }
    }
}

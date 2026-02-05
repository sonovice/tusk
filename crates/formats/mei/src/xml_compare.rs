//! XML comparison utilities for MEI roundtrip testing.
//!
//! Provides tree-based XML comparison that handles acceptable differences:
//! - Whitespace and formatting
//! - Attribute ordering
//! - Namespace prefix variations
//! - XML declaration differences
//! - Empty vs self-closing element syntax
//! - meiversion attribute on root `<mei>` element (export uses codegen version)
//! - MEI version element migrations (e.g., composer→creator for MEI 5.1→6.0)

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

/// Deprecated element → replacement mappings (MEI 5.1 → 6.0)
/// Format: (deprecated_name, replacement_name, implicit_role_value)
///
/// When deprecated elements like `<composer>` are migrated to `<creator>`,
/// the element name itself implies a role. The migration adds this as an
/// explicit `@role` attribute, but the original element didn't have it.
const ELEMENT_MIGRATIONS: &[(&str, &str, &str)] = &[
    ("composer", "creator", "cmp"),
    ("lyricist", "creator", "lyr"),
    ("arranger", "creator", "arr"),
    ("author", "creator", "aut"),
    ("librettist", "creator", "lbt"),
];

/// Check if two element names are equivalent considering MEI version migrations.
///
/// MEI 5.1 deprecated several elements that were renamed in MEI 6.0:
/// - composer, lyricist, arranger, author, librettist → creator
///
/// When comparing roundtrip output, we need to treat these as equivalent since
/// import converts deprecated elements to their replacements, and export always
/// uses the current (6.0) element names.
fn elements_are_equivalent(name1: &str, name2: &str) -> bool {
    if name1 == name2 {
        return true;
    }

    for &(deprecated, replacement, _) in ELEMENT_MIGRATIONS {
        // Check both directions: deprecated→replacement or replacement←deprecated
        if (name1 == deprecated && name2 == replacement)
            || (name1 == replacement && name2 == deprecated)
        {
            return true;
        }
    }

    false
}

/// Get the implicit role value for a deprecated element migration.
///
/// When `<composer>` is migrated to `<creator>`, the role "cmp" is implicit
/// from the element name. Returns Some(role) if this is a migration pair,
/// where name1 is the deprecated element and name2 is the replacement.
fn get_implicit_migration_role(
    deprecated_name: &str,
    replacement_name: &str,
) -> Option<&'static str> {
    for &(deprecated, replacement, implicit_role) in ELEMENT_MIGRATIONS {
        if deprecated_name == deprecated && replacement_name == replacement {
            return Some(implicit_role);
        }
    }
    None
}

/// Parse XML string into a canonical tree representation.
pub fn parse_canonical(xml: &str) -> Result<CanonicalElement, CompareError> {
    use quick_xml::escape::resolve_predefined_entity;

    let mut reader = Reader::from_str(xml);
    // Don't use trim_text(true) as it interferes with proper text accumulation
    // We'll manually normalize whitespace when flushing text

    let mut buf = Vec::new();
    let mut stack: Vec<CanonicalElement> = Vec::new();
    let mut root: Option<CanonicalElement> = None;
    // Accumulator for text content (handles interleaved Text and GeneralRef events)
    let mut text_accumulator = String::new();

    /// Flush accumulated text to the parent element
    fn flush_text(text_acc: &mut String, stack: &mut [CanonicalElement]) {
        let text = text_acc.trim().to_string();
        if !text.is_empty()
            && let Some(parent) = stack.last_mut()
        {
            parent.children.push(CanonicalNode::Text(text));
        }
        text_acc.clear();
    }

    loop {
        let event = reader
            .read_event_into(&mut buf)
            .map_err(|e| CompareError::ParseError(e.to_string()))?;

        match event {
            Event::Start(e) => {
                // Flush any accumulated text before processing element
                flush_text(&mut text_accumulator, &mut stack);

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
                // Flush any accumulated text before processing element
                flush_text(&mut text_accumulator, &mut stack);

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
                // Flush any accumulated text before closing element
                flush_text(&mut text_accumulator, &mut stack);

                if let Some(element) = stack.pop() {
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(CanonicalNode::Element(element));
                    } else {
                        root = Some(element);
                    }
                }
            }

            Event::Text(e) => {
                // Accumulate text content (will be flushed when element/end encountered)
                let text =
                    std::str::from_utf8(&e).map_err(|e| CompareError::ParseError(e.to_string()))?;
                text_accumulator.push_str(text);
            }

            Event::CData(e) => {
                // CDATA is also text content
                let text =
                    std::str::from_utf8(&e).map_err(|e| CompareError::ParseError(e.to_string()))?;
                text_accumulator.push_str(text);
            }

            Event::GeneralRef(r) => {
                // Resolve entity references (e.g., &amp; -> &, &quot; -> ", &lt; -> <)
                let entity_name =
                    std::str::from_utf8(&r).map_err(|e| CompareError::ParseError(e.to_string()))?;
                if let Some(resolved) = resolve_predefined_entity(entity_name) {
                    text_accumulator.push_str(resolved);
                } else if let Ok(Some(ch)) = r.resolve_char_ref() {
                    // Character reference like &#x30; or &#49;
                    text_accumulator.push(ch);
                } else {
                    // Unknown entity - preserve as-is with & and ;
                    text_accumulator.push('&');
                    text_accumulator.push_str(entity_name);
                    text_accumulator.push(';');
                }
            }

            Event::Eof => break,

            // Skip declarations, comments, processing instructions
            Event::Decl(_) | Event::Comment(_) | Event::PI(_) | Event::DocType(_) => {}
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

    // Check if this is the root <mei> element
    let is_root_mei = path.is_empty() && elem1.name == "mei";

    // Compare element names (considering MEI version migrations like composer→creator)
    if !elements_are_equivalent(&elem1.name, &elem2.name) {
        diffs.push(Difference {
            path: current_path.clone(),
            description: format!(
                "element name mismatch: '{}' vs '{}'",
                elem1.name, elem2.name
            ),
        });
        return; // No point comparing children if names differ
    }

    // Check if this is a deprecated element migration (e.g., composer→creator)
    // If so, we need to ignore the implicit role attribute that gets added
    let implicit_role = get_implicit_migration_role(&elem1.name, &elem2.name);

    // Compare attributes (skip meiversion on root <mei> element, skip implicit role on migrations)
    compare_attributes(
        &elem1.attributes,
        &elem2.attributes,
        &current_path,
        diffs,
        is_root_mei,
        implicit_role,
    );

    // Compare children
    compare_children(&elem1.children, &elem2.children, &current_path, diffs);
}

/// Compare two attribute maps.
///
/// If `skip_meiversion` is true, the `meiversion` attribute is ignored.
/// This is needed because MEI export always uses the version from codegen
/// (currently 6.0-dev from ODD spec), not the original file's version.
///
/// If `implicit_migration_role` is Some, the `role` attribute with that value
/// is ignored when it appears only in the second (output) document. This handles
/// the case where deprecated elements like `<composer>` are migrated to `<creator>`
/// and the implicit role is added as an explicit attribute.
fn compare_attributes(
    attrs1: &HashMap<String, String>,
    attrs2: &HashMap<String, String>,
    path: &str,
    diffs: &mut Vec<Difference>,
    skip_meiversion: bool,
    implicit_migration_role: Option<&str>,
) {
    // Find attributes in first but not second
    for (key, value1) in attrs1 {
        // Skip meiversion on root <mei> element - export uses codegen version
        if skip_meiversion && key == "meiversion" {
            continue;
        }
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
    for (key, value) in attrs2 {
        // Skip meiversion on root <mei> element - export uses codegen version
        if skip_meiversion && key == "meiversion" {
            continue;
        }
        // Skip implicit role attribute added during element migration
        // e.g., <composer> → <creator role="cmp"> adds role="cmp" implicitly
        if key == "role"
            && let Some(implicit_role) = implicit_migration_role
            && value == implicit_role
        {
            continue;
        }
        if !attrs1.contains_key(key) {
            diffs.push(Difference {
                path: path.to_string(),
                description: format!(
                    "unexpected attribute '{}' in output (value '{}')",
                    key, value
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

    #[test]
    fn test_meiversion_ignored_on_root_mei() {
        // meiversion attribute on root <mei> element should be ignored
        // because export uses codegen version (6.0-dev), not original version (5.1)
        let xml1 = r#"<mei meiversion="5.1"><music/></mei>"#;
        let xml2 = r#"<mei meiversion="6.0-dev"><music/></mei>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_meiversion_missing_in_output_ok() {
        // Original has meiversion, output doesn't - should be ok
        let xml1 = r#"<mei meiversion="5.1"><music/></mei>"#;
        let xml2 = r#"<mei><music/></mei>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_meiversion_added_in_output_ok() {
        // Original doesn't have meiversion, output does - should be ok
        let xml1 = r#"<mei><music/></mei>"#;
        let xml2 = r#"<mei meiversion="6.0-dev"><music/></mei>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_meiversion_not_ignored_on_nested_element() {
        // meiversion on non-root elements should still be compared
        // (hypothetical - meiversion only appears on root, but let's be safe)
        let xml1 = r#"<root><mei meiversion="5.1"/></root>"#;
        let xml2 = r#"<root><mei meiversion="6.0-dev"/></root>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err());
        if let Err(CompareError::Differences(diffs)) = result {
            assert!(diffs.iter().any(|d| d.description.contains("meiversion")));
        }
    }

    // ============================================================================
    // MEI Version Migration Tests (deprecated element → replacement)
    // ============================================================================

    #[test]
    fn test_elements_are_equivalent_same_name() {
        assert!(elements_are_equivalent("note", "note"));
        assert!(elements_are_equivalent("creator", "creator"));
    }

    #[test]
    fn test_elements_are_equivalent_composer_creator() {
        // composer (MEI 5.1 deprecated) → creator (MEI 6.0)
        assert!(elements_are_equivalent("composer", "creator"));
        assert!(elements_are_equivalent("creator", "composer"));
    }

    #[test]
    fn test_elements_are_equivalent_lyricist_creator() {
        // lyricist (MEI 5.1 deprecated) → creator (MEI 6.0)
        assert!(elements_are_equivalent("lyricist", "creator"));
        assert!(elements_are_equivalent("creator", "lyricist"));
    }

    #[test]
    fn test_elements_are_equivalent_arranger_creator() {
        // arranger (MEI 5.1 deprecated) → creator (MEI 6.0)
        assert!(elements_are_equivalent("arranger", "creator"));
        assert!(elements_are_equivalent("creator", "arranger"));
    }

    #[test]
    fn test_elements_are_equivalent_author_creator() {
        // author (MEI 5.1 deprecated) → creator (MEI 6.0)
        assert!(elements_are_equivalent("author", "creator"));
        assert!(elements_are_equivalent("creator", "author"));
    }

    #[test]
    fn test_elements_are_equivalent_different_elements() {
        // Unrelated elements should NOT be equivalent
        assert!(!elements_are_equivalent("note", "rest"));
        assert!(!elements_are_equivalent("composer", "editor"));
        assert!(!elements_are_equivalent("lyricist", "contributor"));
    }

    #[test]
    fn test_composer_to_creator_migration_in_compare() {
        // Test that composer→creator is accepted in full XML comparison
        let xml1 = r#"<titleStmt><composer>Johann S. Bach</composer></titleStmt>"#;
        let xml2 = r#"<titleStmt><creator>Johann S. Bach</creator></titleStmt>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "composer→creator migration should be accepted"
        );
    }

    #[test]
    fn test_lyricist_to_creator_migration_in_compare() {
        let xml1 = r#"<titleStmt><lyricist>Text Author</lyricist></titleStmt>"#;
        let xml2 = r#"<titleStmt><creator>Text Author</creator></titleStmt>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "lyricist→creator migration should be accepted"
        );
    }

    #[test]
    fn test_arranger_to_creator_migration_in_compare() {
        let xml1 = r#"<titleStmt><arranger>Arr. Name</arranger></titleStmt>"#;
        let xml2 = r#"<titleStmt><creator>Arr. Name</creator></titleStmt>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "arranger→creator migration should be accepted"
        );
    }

    #[test]
    fn test_author_to_creator_migration_in_compare() {
        let xml1 = r#"<titleStmt><author>Author Name</author></titleStmt>"#;
        let xml2 = r#"<titleStmt><creator>Author Name</creator></titleStmt>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "author→creator migration should be accepted"
        );
    }

    #[test]
    fn test_multiple_migrations_in_same_document() {
        // Test multiple deprecated elements in the same document
        let xml1 = r#"<titleStmt>
            <composer>Composer Name</composer>
            <lyricist>Lyricist Name</lyricist>
        </titleStmt>"#;
        let xml2 = r#"<titleStmt>
            <creator>Composer Name</creator>
            <creator>Lyricist Name</creator>
        </titleStmt>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "multiple migrations should be accepted"
        );
    }

    #[test]
    fn test_migration_with_nested_content() {
        // Test that nested content is still compared correctly after migration
        let xml1 = r#"<composer><persName>J.S. Bach</persName></composer>"#;
        let xml2 = r#"<creator><persName>J.S. Bach</persName></creator>"#;
        assert!(compare_xml(xml1, xml2).is_ok());
    }

    #[test]
    fn test_migration_with_different_nested_content_fails() {
        // Migration equivalence doesn't mean content is ignored
        let xml1 = r#"<composer><persName>J.S. Bach</persName></composer>"#;
        let xml2 = r#"<creator><persName>W.A. Mozart</persName></creator>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err(), "different content should still fail");
    }

    // ============================================================================
    // Implicit Role Attribute Tests (deprecated element migration adds @role)
    // ============================================================================

    #[test]
    fn test_composer_migration_with_implicit_role_attribute() {
        // When <composer> is migrated to <creator>, the deserializer adds role="cmp"
        // This implicit role should be ignored in comparisons
        let xml1 = r#"<composer>Johann S. Bach</composer>"#;
        let xml2 = r#"<creator role="cmp">Johann S. Bach</creator>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "implicit role='cmp' from composer→creator migration should be ignored"
        );
    }

    #[test]
    fn test_lyricist_migration_with_implicit_role_attribute() {
        let xml1 = r#"<lyricist>Text Author</lyricist>"#;
        let xml2 = r#"<creator role="lyr">Text Author</creator>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "implicit role='lyr' from lyricist→creator migration should be ignored"
        );
    }

    #[test]
    fn test_arranger_migration_with_implicit_role_attribute() {
        let xml1 = r#"<arranger>Arr. Name</arranger>"#;
        let xml2 = r#"<creator role="arr">Arr. Name</creator>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "implicit role='arr' from arranger→creator migration should be ignored"
        );
    }

    #[test]
    fn test_author_migration_with_implicit_role_attribute() {
        let xml1 = r#"<author>Author Name</author>"#;
        let xml2 = r#"<creator role="aut">Author Name</creator>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "implicit role='aut' from author→creator migration should be ignored"
        );
    }

    #[test]
    fn test_migration_with_wrong_implicit_role_fails() {
        // If the role doesn't match the expected implicit role, it should fail
        let xml1 = r#"<composer>Johann S. Bach</composer>"#;
        let xml2 = r#"<creator role="lyr">Johann S. Bach</creator>"#; // wrong role!
        let result = compare_xml(xml1, xml2);
        assert!(
            result.is_err(),
            "wrong implicit role should fail comparison"
        );
    }

    #[test]
    fn test_migration_preserves_explicit_role_from_original() {
        // If the original deprecated element had a role attribute, it should be preserved
        let xml1 = r#"<composer role="custom">Johann S. Bach</composer>"#;
        let xml2 = r#"<creator role="custom">Johann S. Bach</creator>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "explicit role from original should be preserved"
        );
    }

    #[test]
    fn test_migration_with_nested_content_and_implicit_role() {
        // Test nested content with implicit role
        let xml1 = r#"<composer><persName>J.S. Bach</persName></composer>"#;
        let xml2 = r#"<creator role="cmp"><persName>J.S. Bach</persName></creator>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "nested content with implicit role should work"
        );
    }

    #[test]
    fn test_non_migrated_elements_with_extra_role_fails() {
        // For non-migrated elements, an extra role attribute should still fail
        let xml1 = r#"<title>Test Title</title>"#;
        let xml2 = r#"<title role="main">Test Title</title>"#;
        let result = compare_xml(xml1, xml2);
        assert!(
            result.is_err(),
            "extra role on non-migrated element should fail"
        );
    }
}

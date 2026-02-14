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
use std::collections::BTreeMap;

use crate::deserializer::strip_namespace_prefix;

/// A canonical representation of an XML element for comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalElement {
    /// Element name without namespace prefix.
    pub name: String,
    /// Attributes as a map (order-independent).
    pub attributes: BTreeMap<String, String>,
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

/// Containers where child element order is not semantically meaningful.
///
/// For these elements, children are compared by matching (name, key_attributes)
/// rather than by position. This allows roundtrip tests to pass even when
/// element order differs between input and output.
const UNORDERED_CONTAINERS: &[&str] = &[
    // Metadata containers
    "meiHead",
    "fileDesc",
    "titleStmt",
    "pubStmt",
    "respStmt",
    "sourceDesc",
    "bibl",
    "encodingDesc",
    "workList",
    "manifestationList",
    "manifestation",
    "physDesc",
    // Part/staff groupings (staffDef order doesn't matter)
    "staffGrp",
    // Score definition containers
    "scoreDef",
    // Measure contents - staff elements and control events can be in any order
    // What matters is @staff and @tstamp attributes, not XML order
    "measure",
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

                let mut attributes = BTreeMap::new();
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

                let mut attributes = BTreeMap::new();
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
                crate::deserializer::resolve_xml_entity(&r, &mut text_accumulator)
                    .map_err(|e| CompareError::ParseError(e.to_string()))?;
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

    // Compare children (use unordered comparison for metadata containers)
    compare_children(
        &elem1.children,
        &elem2.children,
        &current_path,
        &elem1.name,
        diffs,
    );
}

/// Normalize whitespace in a string for comparison.
///
/// This handles XML list type semantics where leading/trailing whitespace is trimmed
/// and internal whitespace is collapsed to single spaces. This is standard XML behavior
/// for attributes defined as `rng:list` in RelaxNG (e.g., bezier, bulge coordinates).
fn normalize_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Check if two attribute values are semantically equivalent.
///
/// Compare two f64 values with epsilon tolerance for rounding differences.
/// Uses relative epsilon for large values, absolute epsilon for small values.
fn floats_equivalent(a: f64, b: f64) -> bool {
    let diff = (a - b).abs();
    if diff == 0.0 {
        return true;
    }
    let max_abs = a.abs().max(b.abs());
    // Relative tolerance of ~1e-12, or absolute tolerance of 1e-10 for small values
    diff <= max_abs * 1e-12 || diff < 1e-10
}

/// This handles:
/// - Whitespace normalization (for XML list types)
/// - Numeric equivalence (1.00 == 1, 2.5 == 2.50)
///
/// For numeric comparison, values are compared with epsilon tolerance
/// to handle f64 rounding differences (e.g., tstamp 2.4166666666666665 vs 2.416666666666667).
fn attribute_values_equivalent(val1: &str, val2: &str) -> bool {
    // First try whitespace-normalized string comparison
    let norm1 = normalize_whitespace(val1);
    let norm2 = normalize_whitespace(val2);
    if norm1 == norm2 {
        return true;
    }

    // Try numeric comparison for single values (with epsilon for float precision)
    if let (Ok(n1), Ok(n2)) = (norm1.parse::<f64>(), norm2.parse::<f64>()) {
        if floats_equivalent(n1, n2) {
            return true;
        }
    }

    // Try tstamp2 comparison ("Nm+B" format) with float tolerance on the beat part
    if let (Some((m1, b1)), Some((m2, b2))) =
        (parse_tstamp2_parts(&norm1), parse_tstamp2_parts(&norm2))
    {
        if m1 == m2 && floats_equivalent(b1, b2) {
            return true;
        }
    }

    // Try numeric comparison for whitespace-separated lists (like bezier coordinates)
    let parts1: Vec<&str> = norm1.split_whitespace().collect();
    let parts2: Vec<&str> = norm2.split_whitespace().collect();
    if parts1.len() == parts2.len() && parts1.len() > 1 {
        let all_numeric_equal = parts1.iter().zip(parts2.iter()).all(|(p1, p2)| {
            // Try string equality first
            if p1 == p2 {
                return true;
            }
            // Then try numeric equality (with epsilon)
            if let (Ok(n1), Ok(n2)) = (p1.parse::<f64>(), p2.parse::<f64>()) {
                floats_equivalent(n1, n2)
            } else {
                false
            }
        });
        if all_numeric_equal {
            return true;
        }
    }

    false
}

/// Compare two attribute maps.
///
/// If `skip_meiversion` is true, the `meiversion` attribute is ignored.
/// This is needed because MEI export always uses the version from codegen
/// (currently 6.0-dev from RNG schema), not the original file's version.
///
/// If `implicit_migration_role` is Some, the `role` attribute with that value
/// is ignored when it appears only in the second (output) document. This handles
/// the case where deprecated elements like `<composer>` are migrated to `<creator>`
/// and the implicit role is added as an explicit attribute.
fn compare_attributes(
    attrs1: &BTreeMap<String, String>,
    attrs2: &BTreeMap<String, String>,
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
        // Skip xml:id comparisons - these are auto-generated and will differ between imports
        if key == "xml:id" {
            continue;
        }
        match attrs2.get(key) {
            Some(value2) => {
                // Check semantic equivalence (whitespace normalization + numeric equality)
                if !attribute_values_equivalent(value1, value2) {
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
        // Skip xml:id comparisons - these are auto-generated and will differ between imports
        if key == "xml:id" {
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
///
/// For containers in UNORDERED_CONTAINERS, children are matched by element key
/// (name + identifying attributes) rather than by position. This allows roundtrip
/// tests to pass when element order differs but content is semantically equivalent.
fn compare_children(
    children1: &[CanonicalNode],
    children2: &[CanonicalNode],
    path: &str,
    parent_name: &str,
    diffs: &mut Vec<Difference>,
) {
    if UNORDERED_CONTAINERS.contains(&parent_name) {
        compare_children_unordered(children1, children2, path, diffs);
    } else {
        compare_children_ordered(children1, children2, path, diffs);
    }
}

/// Normalize element name for key generation, considering MEI version migrations.
///
/// This ensures deprecated elements (composer, lyricist, etc.) match their
/// replacement (creator) during unordered comparison.
fn normalize_element_name(name: &str) -> &str {
    for &(deprecated, replacement, _) in ELEMENT_MIGRATIONS {
        if name == deprecated {
            return replacement;
        }
    }
    name
}

/// Recursively collect all text content from an element and its descendants.
fn collect_deep_text(elem: &CanonicalElement) -> String {
    let mut text = String::new();
    for child in &elem.children {
        match child {
            CanonicalNode::Text(t) => text.push_str(t),
            CanonicalNode::Element(e) => text.push_str(&collect_deep_text(e)),
        }
    }
    text
}

/// Get element-specific key suffix for control events that need extra disambiguation.
///
/// Parse a tstamp2 value ("Nm+B") into (measures, beat) parts.
fn parse_tstamp2_parts(s: &str) -> Option<(&str, f64)> {
    let (m_part, b_part) = s.split_once("m+")?;
    let b = b_part.parse::<f64>().ok()?;
    Some((m_part, b))
}

/// Normalize a tstamp2 value ("Nm+B") for float-stable comparison.
/// Parses the beat portion as f64 and formats with fixed precision.
fn normalize_tstamp2(s: &str) -> String {
    if let Some((m_part, b_part)) = s.split_once("m+") {
        if let Ok(b) = b_part.parse::<f64>() {
            return format!("{}m+{:.10}", m_part, b);
        }
    }
    s.to_string()
}

/// Some control events can share the same staff+tstamp but differ in type-specific
/// attributes (e.g., two hairpins at same position with different @form, or two
/// pedals with different @dir).
fn control_event_type_key(name: &str, elem: &CanonicalElement) -> String {
    match name {
        "hairpin" => elem
            .attributes
            .get("form")
            .map(|f| format!(",form={}", f))
            .unwrap_or_default(),
        "pedal" => elem
            .attributes
            .get("dir")
            .map(|d| format!(",dir={}", d))
            .unwrap_or_default(),
        // Dir disambiguation: use @xml:id (direction content stored in ExtensionStore)
        "dir" => elem
            .attributes
            .get("xml:id")
            .map(|id| format!(",id={}", id))
            .unwrap_or_default(),
        _ => String::new(),
    }
}

/// Get a key for matching elements in unordered comparison.
///
/// Elements are matched by (name, key_attribute) where key_attribute depends
/// on the element type:
/// - staffDef: @n attribute
/// - staff: @n attribute
/// - Control events (dir, dynam, slur, etc.): @staff + @tstamp or @startid
/// - creator/contributor: @role or text content
/// - Others: first identifying attribute or text content
fn get_element_key(elem: &CanonicalElement) -> String {
    // Normalize element name for migrations (composer → creator, etc.)
    let name = normalize_element_name(&elem.name);

    let key_attr = match name {
        // Staff-like elements keyed by @n
        "staffDef" | "layerDef" | "instrDef" | "staff" => elem.attributes.get("n"),

        // staffGrp keyed by @symbol + child <label> text + contained staffDef @n values
        // to distinguish multiple staffGrp siblings
        "staffGrp" => {
            let mut parts = Vec::new();
            if let Some(sym) = elem.attributes.get("symbol") {
                parts.push(format!("sym={}", sym));
            }
            if let Some(bar) = elem.attributes.get("bar.thru") {
                parts.push(format!("bar={}", bar));
            }
            // Collect child label text and staffDef @n values for disambiguation
            for child in &elem.children {
                if let CanonicalNode::Element(child_elem) = child {
                    if child_elem.name == "label" {
                        let label_text: String = child_elem
                            .children
                            .iter()
                            .filter_map(|c| {
                                if let CanonicalNode::Text(t) = c {
                                    Some(t.as_str())
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if !label_text.is_empty() {
                            parts.push(format!(
                                "label={}",
                                label_text.chars().take(20).collect::<String>()
                            ));
                        }
                    } else if child_elem.name == "staffDef" {
                        if let Some(n) = child_elem.attributes.get("n") {
                            parts.push(format!("n={}", n));
                        }
                    }
                }
            }
            if !parts.is_empty() {
                return format!("staffGrp[{}]", parts.join(","));
            }
            return name.to_string();
        }

        // Control events keyed by staff+tstamp or startid
        "dir" | "dynam" | "tempo" | "hairpin" | "slur" | "tie" | "fermata" | "trill"
        | "mordent" | "turn" | "artic" | "breath" | "caesura" | "pedal" | "arpeg"
        | "tupletSpan" | "beamSpan" | "phrase" | "fing" | "fingGrp" | "harm" | "gliss" | "bend"
        | "ornam" | "octave" | "lv" | "bracketSpan" | "curve" | "line" | "reh" | "repeatMark"
        | "attacca" | "cpMark" | "sp" | "stageDir" | "anchoredText" | "harpPedal" | "metaMark" => {
            // Try startid first (for spanners), then staff+tstamp
            // Include @staff to disambiguate control events on same note but different staves
            if let Some(startid) = elem.attributes.get("startid") {
                let staff_key = elem
                    .attributes
                    .get("staff")
                    .map(|s| format!(",staff={}", s))
                    .unwrap_or_default();
                // Include @endid to disambiguate spanners with same start but different end
                let endid_key = elem
                    .attributes
                    .get("endid")
                    .map(|e| format!(",endid={}", e))
                    .unwrap_or_default();
                // Include @curvedir for slurs/ties with same start/end but different curve direction
                let curvedir_key = elem
                    .attributes
                    .get("curvedir")
                    .map(|c| format!(",curvedir={}", c))
                    .unwrap_or_default();
                // Include @lform to disambiguate solid vs dashed slurs
                let lform_key = elem
                    .attributes
                    .get("lform")
                    .map(|l| format!(",lform={}", l))
                    .unwrap_or_default();
                // Include @place to disambiguate above/below at same startid
                let place_key = elem
                    .attributes
                    .get("place")
                    .map(|p| format!(",place={}", p))
                    .unwrap_or_default();
                // Include @tstamp2 for spanners with same startid/endid but different duration
                let tstamp2_key = elem
                    .attributes
                    .get("tstamp2")
                    .map(|t| format!(",tstamp2={}", normalize_tstamp2(t)))
                    .unwrap_or_default();
                // Include element-specific attributes (e.g., @dir for pedal, @form for hairpin)
                // Multiple control events can share the same startid (e.g., pedal down/up)
                let type_key = control_event_type_key(name, elem);
                // Include text content for text-bearing elements (dir, dynam)
                let text = collect_deep_text(elem);
                let text_key = if !text.is_empty() {
                    format!(",text={}", text.chars().take(30).collect::<String>())
                } else {
                    String::new()
                };
                return format!(
                    "{}[startid={}{}{}{}{}{}{}{}{}]",
                    name,
                    startid,
                    staff_key,
                    endid_key,
                    curvedir_key,
                    lform_key,
                    place_key,
                    tstamp2_key,
                    type_key,
                    text_key
                );
            }
            if let Some(tstamp) = elem.attributes.get("tstamp") {
                // Normalize tstamp float to consistent precision for key matching
                let tstamp_key = if let Ok(ts) = tstamp.parse::<f64>() {
                    format!("{:.10}", ts)
                } else {
                    tstamp.clone()
                };
                let staff_key = elem
                    .attributes
                    .get("staff")
                    .map(|s| format!("staff={},", s))
                    .unwrap_or_default();
                // Include text content for text-bearing elements to disambiguate
                // multiple dirs/dynams at the same staff+tstamp
                let text = collect_deep_text(elem);
                let text_key = if !text.is_empty() {
                    format!(",text={}", text.chars().take(30).collect::<String>())
                } else {
                    String::new()
                };
                // Include @place to disambiguate above/below at same position
                let place_key = if let Some(place) = elem.attributes.get("place") {
                    format!(",place={}", place)
                } else {
                    String::new()
                };
                // Include @curvedir for slurs/ties with same tstamp but different direction
                let curvedir_key = elem
                    .attributes
                    .get("curvedir")
                    .map(|c| format!(",curvedir={}", c))
                    .unwrap_or_default();
                // Include @tstamp2 for spanners (hairpin, slur, etc.) that start at
                // the same beat but end at different points
                let tstamp2_key = elem
                    .attributes
                    .get("tstamp2")
                    .map(|t| format!(",tstamp2={}", normalize_tstamp2(t)))
                    .unwrap_or_default();
                // Include visual positioning attributes to disambiguate
                // otherwise-identical control events at the same position.
                // Normalize float values for consistent comparison.
                let mut vis_keys = String::new();
                for attr in &["vo", "ho", "startto", "endto", "opening", "vgrp"] {
                    if let Some(val) = elem.attributes.get(*attr) {
                        let normalized = if let Ok(f) = val.parse::<f64>() {
                            format!("{:.10}", f)
                        } else {
                            val.clone()
                        };
                        vis_keys.push_str(&format!(",{}={}", attr, normalized));
                    }
                }
                // Include element-specific attributes for disambiguation
                let type_key = control_event_type_key(name, elem);
                return format!(
                    "{}[{}tstamp={}{}{}{}{}{}{}]",
                    name,
                    staff_key,
                    tstamp_key,
                    place_key,
                    curvedir_key,
                    text_key,
                    tstamp2_key,
                    vis_keys,
                    type_key
                );
            }
            elem.attributes.get("staff")
        }

        // Person names keyed by @role + text content
        "persName" => {
            let role = elem.attributes.get("role").cloned().unwrap_or_default();
            let text = collect_deep_text(elem);
            if !role.is_empty() || !text.is_empty() {
                return format!(
                    "persName[role={},text={}]",
                    role,
                    text.chars().take(40).collect::<String>()
                );
            }
            None
        }

        // Corp names keyed by text content
        "corpName" => {
            let text = collect_deep_text(elem);
            if !text.is_empty() {
                return format!(
                    "corpName[text={}]",
                    text.chars().take(60).collect::<String>()
                );
            }
            None
        }

        // Resp elements keyed by text content
        "resp" => {
            let text = collect_deep_text(elem);
            if !text.is_empty() {
                return format!("resp[text={}]", text.chars().take(40).collect::<String>());
            }
            None
        }

        "creator" | "contributor" | "editor" => {
            // Key by normalized name + text only, never by @role.
            // Role can differ between input (implicit from deprecated element name like
            // <composer>) and output (explicit @role="cmp" on <creator>), or be absent
            // entirely on a bare <creator>. FIFO matching (remove(0) in
            // compare_children_unordered) handles same-text creators with different
            // roles by preserving document order.
            let text = collect_deep_text(elem);
            if !text.is_empty() {
                return format!(
                    "{}[text={}]",
                    name,
                    text.chars().take(40).collect::<String>()
                );
            }
            None
        }

        // Source/manifestation keyed by first identifier text (within bibl for source)
        "manifestation" => {
            for child in &elem.children {
                if let CanonicalNode::Element(child_elem) = child {
                    if child_elem.name == "identifier" {
                        let id_text: String = child_elem
                            .children
                            .iter()
                            .filter_map(|c| {
                                if let CanonicalNode::Text(t) = c {
                                    Some(t.as_str())
                                } else {
                                    None
                                }
                            })
                            .collect();
                        if !id_text.is_empty() {
                            return format!(
                                "manifestation[id={}]",
                                id_text.chars().take(200).collect::<String>()
                            );
                        }
                    }
                }
            }
            None
        }

        "source" => {
            // Key sources by bibl deep text to distinguish sources with same identifiers
            // but different content (e.g., one with full metadata, one with just URI)
            for child in &elem.children {
                if let CanonicalNode::Element(child_elem) = child {
                    if child_elem.name == "bibl" {
                        let bibl_text = collect_deep_text(child_elem);
                        if !bibl_text.is_empty() {
                            return format!(
                                "source[bibl={}]",
                                bibl_text.chars().take(200).collect::<String>()
                            );
                        }
                    }
                }
            }
            None
        }

        // Identifier elements keyed by @type + text content
        "identifier" => {
            let type_val = elem.attributes.get("type").cloned().unwrap_or_default();
            let text: String = elem
                .children
                .iter()
                .filter_map(|c| {
                    if let CanonicalNode::Text(t) = c {
                        Some(t.as_str())
                    } else {
                        None
                    }
                })
                .collect();
            if !type_val.is_empty() || !text.is_empty() {
                return format!(
                    "identifier[type={},text={}]",
                    type_val,
                    text.chars().take(200).collect::<String>()
                );
            }
            None
        }

        // sb/pb keyed by @xml:id (print data lives in ExtensionStore)
        "sb" | "pb" => {
            if let Some(id) = elem.attributes.get("xml:id") {
                return format!("{}[id={}]", name, id);
            }
            None
        }

        // extMeta keyed by @analog prefix (carries musicxml: roundtrip data)
        "extMeta" => {
            if let Some(analog) = elem.attributes.get("analog") {
                let prefix = analog.split(',').next().unwrap_or(analog);
                return format!(
                    "extMeta[analog={}]",
                    prefix.chars().take(60).collect::<String>()
                );
            }
            None
        }

        _ => elem.attributes.get("n"),
    };

    if let Some(attr) = key_attr {
        format!("{}[@={}]", name, attr)
    } else {
        // Fall back to deep text content for elements without key attributes
        // Uses recursive text collection to handle elements whose text is in children
        // (e.g., <respStmt><corpName>text</corpName></respStmt>)
        let text = collect_deep_text(elem);
        if !text.is_empty() {
            format!(
                "{}[text={}]",
                name,
                text.chars().take(120).collect::<String>()
            )
        } else {
            name.to_string()
        }
    }
}

/// Compare children in unordered mode (for metadata containers).
fn compare_children_unordered(
    children1: &[CanonicalNode],
    children2: &[CanonicalNode],
    path: &str,
    diffs: &mut Vec<Difference>,
) {
    // Extract elements from both lists
    let elems1: Vec<&CanonicalElement> = children1
        .iter()
        .filter_map(|c| match c {
            CanonicalNode::Element(e) => Some(e),
            _ => None,
        })
        .collect();
    let elems2: Vec<&CanonicalElement> = children2
        .iter()
        .filter_map(|c| match c {
            CanonicalNode::Element(e) => Some(e),
            _ => None,
        })
        .collect();

    // Build key-to-element map for second list
    let mut elems2_by_key: std::collections::BTreeMap<String, Vec<&CanonicalElement>> =
        std::collections::BTreeMap::new();
    for elem in &elems2 {
        let key = get_element_key(elem);
        elems2_by_key.entry(key).or_default().push(elem);
    }

    // Match elements from first list
    let mut matched_keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    for elem1 in &elems1 {
        let key = get_element_key(elem1);
        if let Some(matches) = elems2_by_key.get_mut(&key) {
            if !matches.is_empty() {
                let elem2 = matches.remove(0);
                // Found match - compare the elements
                compare_elements(elem1, elem2, path, diffs);
                matched_keys.insert(key);
            } else {
                // All matches consumed
                diffs.push(Difference {
                    path: path.to_string(),
                    description: format!(
                        "element '{}' missing in output (key: {})",
                        elem1.name, key
                    ),
                });
            }
        } else {
            diffs.push(Difference {
                path: path.to_string(),
                description: format!("element '{}' missing in output (key: {})", elem1.name, key),
            });
        }
    }

    // Report unmatched elements from second list
    for (key, remaining) in &elems2_by_key {
        for elem in remaining {
            diffs.push(Difference {
                path: path.to_string(),
                description: format!(
                    "unexpected element '{}' in output (key: {})",
                    elem.name, key
                ),
            });
        }
    }

    // Compare text nodes in order (they're usually not significant in unordered containers)
    let texts1: Vec<&str> = children1
        .iter()
        .filter_map(|c| match c {
            CanonicalNode::Text(t) => Some(t.as_str()),
            _ => None,
        })
        .collect();
    let texts2: Vec<&str> = children2
        .iter()
        .filter_map(|c| match c {
            CanonicalNode::Text(t) => Some(t.as_str()),
            _ => None,
        })
        .collect();

    // Only report if text content differs significantly
    let all_text1: String = texts1.join(" ");
    let all_text2: String = texts2.join(" ");
    if normalize_whitespace(&all_text1) != normalize_whitespace(&all_text2) {
        diffs.push(Difference {
            path: path.to_string(),
            description: format!(
                "text content mismatch: '{}' vs '{}'",
                all_text1.chars().take(50).collect::<String>(),
                all_text2.chars().take(50).collect::<String>()
            ),
        });
    }
}

/// Compare children in strict order (for musical content).
fn compare_children_ordered(
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
        assert_eq!(strip_namespace_prefix("xlink:href"), "xlink:href");
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

    // ============================================================================
    // Attribute Value Whitespace Normalization Tests
    // ============================================================================

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("hello world"), "hello world");
        assert_eq!(normalize_whitespace("  hello world"), "hello world");
        assert_eq!(normalize_whitespace("hello world  "), "hello world");
        assert_eq!(normalize_whitespace("  hello  world  "), "hello world");
        assert_eq!(normalize_whitespace("-7 -12"), "-7 -12");
        assert_eq!(normalize_whitespace("  -7 -12"), "-7 -12");
        assert_eq!(normalize_whitespace("  -7  -12  "), "-7 -12");
    }

    #[test]
    fn test_attribute_whitespace_normalization_bezier() {
        // XML list types (like bezier) normalize whitespace per XML Schema/RelaxNG
        // Leading whitespace should be ignored in comparison
        let xml1 = r#"<slur bezier="  -7 -12"/>"#;
        let xml2 = r#"<slur bezier="-7 -12"/>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "leading whitespace in bezier should be normalized"
        );
    }

    #[test]
    fn test_attribute_whitespace_normalization_multiple_spaces() {
        // Multiple internal spaces should be normalized to single space
        let xml1 = r#"<slur bezier="19  45  -32  118"/>"#;
        let xml2 = r#"<slur bezier="19 45 -32 118"/>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "multiple internal spaces should be normalized"
        );
    }

    #[test]
    fn test_attribute_whitespace_normalization_trailing() {
        // Trailing whitespace should be ignored
        let xml1 = r#"<slur bezier="-7 -12  "/>"#;
        let xml2 = r#"<slur bezier="-7 -12"/>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "trailing whitespace should be normalized"
        );
    }

    #[test]
    fn test_attribute_values_differ_semantically() {
        // Different values should still be caught
        let xml1 = r#"<slur bezier="  -7 -12"/>"#;
        let xml2 = r#"<slur bezier="-7 -15"/>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err(), "different values should still be detected");
    }

    // ============================================================================
    // Numeric Value Equivalence Tests
    // ============================================================================

    #[test]
    fn test_numeric_equivalence_trailing_zeros() {
        // 1.00 and 1 are semantically equivalent
        let xml1 = r#"<dir tstamp="1.00"/>"#;
        let xml2 = r#"<dir tstamp="1"/>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "1.00 and 1 should be equivalent"
        );
    }

    #[test]
    fn test_numeric_equivalence_decimal() {
        // 2.50 and 2.5 are semantically equivalent
        let xml1 = r#"<note dur="2.50"/>"#;
        let xml2 = r#"<note dur="2.5"/>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "2.50 and 2.5 should be equivalent"
        );
    }

    #[test]
    fn test_numeric_equivalence_integer_vs_float() {
        // 4 and 4.0 are semantically equivalent
        let xml1 = r#"<note oct="4"/>"#;
        let xml2 = r#"<note oct="4.0"/>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "4 and 4.0 should be equivalent"
        );
    }

    #[test]
    fn test_numeric_list_equivalence() {
        // Numeric lists with equivalent values
        let xml1 = r#"<slur bezier="1.00 2.50 3.0"/>"#;
        let xml2 = r#"<slur bezier="1 2.5 3"/>"#;
        assert!(
            compare_xml(xml1, xml2).is_ok(),
            "numeric lists with equivalent values should match"
        );
    }

    #[test]
    fn test_numeric_different_values_still_fail() {
        // Different numeric values should still fail
        let xml1 = r#"<dir tstamp="1.5"/>"#;
        let xml2 = r#"<dir tstamp="2"/>"#;
        let result = compare_xml(xml1, xml2);
        assert!(result.is_err(), "different numeric values should fail");
    }

    #[test]
    fn test_attribute_values_equivalent_fn() {
        // Test the helper function directly
        assert!(attribute_values_equivalent("1.00", "1"));
        assert!(attribute_values_equivalent("1", "1.00"));
        assert!(attribute_values_equivalent("2.5", "2.50"));
        assert!(attribute_values_equivalent("4", "4.0"));
        assert!(attribute_values_equivalent("-7 -12", "-7.0 -12.00"));
        assert!(!attribute_values_equivalent("1.5", "2"));
        assert!(!attribute_values_equivalent("hello", "world"));
        // Non-numeric strings should use exact match
        assert!(attribute_values_equivalent("hello", "hello"));
        assert!(!attribute_values_equivalent("hello", "Hello"));
    }
}

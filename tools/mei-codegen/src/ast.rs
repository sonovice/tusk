//! AST types for MEI ODD specification elements.
//!
//! These structures represent the parsed content of MEI ODD files
//! and are used to generate Rust code.

#![allow(dead_code)]

use std::collections::HashMap;

/// A value in an enumerated data type.
#[derive(Debug, Clone)]
pub struct DataTypeValue {
    /// The MEI identifier for this value (e.g., "s", "f", "1", "breve").
    pub ident: String,
    /// Human-readable description.
    pub desc: String,
}

/// A Schematron constraint rule from constraintSpec.
#[derive(Debug, Clone)]
pub struct Constraint {
    /// Constraint identifier (from constraintSpec/@ident).
    pub ident: String,
    /// XPath context (from sch:rule/@context).
    pub context: String,
    /// Assertion test (from sch:assert/@test).
    pub test: String,
    /// Error/warning message.
    pub message: String,
    /// Role: "error" or "warning" (default: "error").
    pub role: String,
}

/// The kind of data type defined by a macroSpec.
#[derive(Debug, Clone)]
pub enum DataTypeKind {
    /// Closed enumeration of values.
    ValList(Vec<DataTypeValue>),
    /// XML Schema primitive type with optional pattern constraint.
    Primitive {
        type_name: String,
        pattern: Option<String>,
        min_inclusive: Option<String>,
        max_inclusive: Option<String>,
    },
    /// Reference to another data type.
    Reference(String),
    /// Union of multiple data types (alternate).
    Alternate(Vec<DataTypeRef>),
    /// Choice between multiple patterns.
    Choice(Vec<DataTypeRef>),
}

/// A reference to a data type (used in alternate/choice).
#[derive(Debug, Clone)]
pub enum DataTypeRef {
    /// Reference via macroRef key="...".
    MacroRef(String),
    /// Reference via rng:ref name="...".
    RngRef(String),
}

/// A data type definition from macroSpec type="dt".
#[derive(Debug, Clone)]
pub struct DataType {
    /// MEI identifier (e.g., "data.DURATION.cmn").
    pub ident: String,
    /// Source module (e.g., "MEI.cmn").
    pub module: String,
    /// Human-readable description.
    pub desc: String,
    /// The type definition.
    pub kind: DataTypeKind,
}

/// A pattern entity from macroSpec type="pe".
/// These define reusable content patterns for element definitions.
#[derive(Debug, Clone)]
pub struct PatternEntity {
    /// MEI identifier (e.g., "macro.anyXML").
    pub ident: String,
    /// Source module.
    pub module: String,
    /// Human-readable description.
    pub desc: String,
    /// The content pattern.
    pub content: ContentModel,
}

/// The data type of an attribute (in attDef).
#[derive(Debug, Clone)]
pub enum AttributeDataType {
    /// Reference to a data type via rng:ref.
    Ref(String),
    /// Inline closed value list.
    InlineValList(Vec<DataTypeValue>),
    /// XML Schema primitive type.
    Primitive {
        type_name: String,
        pattern: Option<String>,
    },
    /// Space-separated list of values (rng:list).
    List {
        /// Inner data type (what each list item is).
        inner: Box<AttributeDataType>,
        /// Minimum occurrences (1 for oneOrMore, 0 for zeroOrMore).
        min_occurs: u32,
    },
}

/// An attribute definition from attDef.
#[derive(Debug, Clone)]
pub struct Attribute {
    /// Attribute name (e.g., "dur", "pname", "xml:id").
    pub ident: String,
    /// Human-readable description.
    pub desc: String,
    /// Usage: "opt" (optional), "rec" (recommended), "req" (required).
    pub usage: String,
    /// The attribute's data type.
    pub datatype: Option<AttributeDataType>,
    /// Default value if any.
    pub default_val: Option<String>,
    /// Maximum occurrences (for multi-valued attributes).
    pub max_occurs: Option<String>,
    /// Schematron constraints on this attribute.
    pub constraints: Vec<Constraint>,
}

/// An attribute class from classSpec type="atts".
#[derive(Debug, Clone)]
pub struct AttClass {
    /// MEI identifier (e.g., "att.duration.log").
    pub ident: String,
    /// Source module.
    pub module: String,
    /// Human-readable description.
    pub desc: String,
    /// Parent classes (via memberOf).
    pub member_of: Vec<String>,
    /// Attributes defined directly in this class.
    pub attributes: Vec<Attribute>,
    /// Class-level Schematron constraints.
    pub constraints: Vec<Constraint>,
}

/// A model class from classSpec type="model".
/// These group elements that can appear in specific content model positions.
#[derive(Debug, Clone)]
pub struct ModelClass {
    /// MEI identifier (e.g., "model.eventLike").
    pub ident: String,
    /// Source module.
    pub module: String,
    /// Human-readable description.
    pub desc: String,
    /// Parent model classes (via memberOf).
    pub member_of: Vec<String>,
}

/// A content model item (RelaxNG pattern).
#[derive(Debug, Clone)]
pub enum ContentItem {
    /// Empty content.
    Empty,
    /// Text content.
    Text,
    /// Reference to an element or model class.
    Ref(String),
    /// Reference to a macro/pattern entity.
    MacroRef(String),
    /// Zero or more occurrences.
    ZeroOrMore(Box<ContentModel>),
    /// One or more occurrences.
    OneOrMore(Box<ContentModel>),
    /// Optional (zero or one).
    Optional(Box<ContentModel>),
    /// Choice between alternatives.
    Choice(Vec<ContentModel>),
    /// Sequence (group).
    Group(Box<ContentModel>),
    /// Unordered content (elements can appear in any order).
    Interleave(Vec<ContentModel>),
    /// Space-separated list of values.
    List(Box<ContentModel>),
    /// Any element (from any namespace except MEI).
    AnyElement,
}

/// A content model is a sequence of content items.
pub type ContentModel = Vec<ContentItem>;

/// An element definition from elementSpec.
#[derive(Debug, Clone)]
pub struct Element {
    /// Element name (e.g., "note", "measure").
    pub ident: String,
    /// Source module.
    pub module: String,
    /// Short name/gloss.
    pub gloss: String,
    /// Human-readable description.
    pub desc: String,
    /// Classes this element belongs to (attribute classes and model classes).
    pub member_of: Vec<String>,
    /// Child element content model.
    pub content: ContentModel,
    /// Element-level Schematron constraints.
    pub constraints: Vec<Constraint>,
}

/// All definitions collected from MEI ODD files.
#[derive(Debug, Default)]
pub struct OddDefinitions {
    /// Data types from macroSpec type="dt".
    pub data_types: HashMap<String, DataType>,
    /// Pattern entities from macroSpec type="pe".
    pub pattern_entities: HashMap<String, PatternEntity>,
    /// Attribute classes from classSpec type="atts".
    pub att_classes: HashMap<String, AttClass>,
    /// Model classes from classSpec type="model".
    pub model_classes: HashMap<String, ModelClass>,
    /// Elements from elementSpec.
    pub elements: HashMap<String, Element>,
}

impl OddDefinitions {
    /// Create a new empty definitions collection.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get all elements that are members of a given model class.
    pub fn elements_in_model(&self, model_ident: &str) -> Vec<&Element> {
        self.elements
            .values()
            .filter(|e| e.member_of.iter().any(|m| m == model_ident))
            .collect()
    }

    /// Recursively collect all attributes for an attribute class,
    /// including inherited attributes from parent classes.
    pub fn collect_attributes(&self, class_ident: &str) -> Vec<Attribute> {
        let mut attrs = Vec::new();
        if let Some(ac) = self.att_classes.get(class_ident) {
            // Add attributes from parent classes first
            for parent in &ac.member_of {
                attrs.extend(self.collect_attributes(parent));
            }
            // Add this class's own attributes
            attrs.extend(ac.attributes.clone());
        }
        attrs
    }

    /// Get all attribute classes that an element inherits from.
    pub fn element_att_classes(&self, elem: &Element) -> Vec<&AttClass> {
        elem.member_of
            .iter()
            .filter_map(|m| self.att_classes.get(m))
            .collect()
    }

    /// Get all model classes that an element belongs to.
    pub fn element_model_classes(&self, elem: &Element) -> Vec<&ModelClass> {
        elem.member_of
            .iter()
            .filter_map(|m| self.model_classes.get(m))
            .collect()
    }

    /// Recursively resolve a model class reference to concrete element names.
    pub fn resolve_model_to_elements(&self, model_ident: &str) -> Vec<String> {
        let mut elements = Vec::new();

        // Find elements directly in this model
        for elem in self.elements.values() {
            if elem.member_of.contains(&model_ident.to_string()) {
                elements.push(elem.ident.clone());
            }
        }

        // Find child model classes and recurse
        // Model classes can have child model classes via memberOf
        // But typically elements declare memberOf model classes, not vice versa
        // So we primarily rely on the element search above

        // Also check if any model class inherits from this one
        for mc in self.model_classes.values() {
            if mc.member_of.contains(&model_ident.to_string()) {
                elements.extend(self.resolve_model_to_elements(&mc.ident));
            }
        }

        elements.sort();
        elements.dedup();
        elements
    }
}

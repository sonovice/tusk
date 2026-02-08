//! AST for the MusicXML XSD schema.
//!
//! Represents parsed simple types, complex types, groups, attribute groups,
//! and top-level elements used to generate Rust code.

#![allow(dead_code)]

use std::collections::HashMap;

/// Schema AST built from the XSD file.
#[derive(Debug, Default)]
pub struct Schema {
    pub simple_types: HashMap<String, SimpleType>,
    pub complex_types: HashMap<String, ComplexType>,
    pub groups: HashMap<String, Group>,
    pub attribute_groups: HashMap<String, AttributeGroup>,
    /// Top-level element name -> type name (or inline content)
    pub elements: HashMap<String, ElementDecl>,
}

/// xs:simpleType: enumeration or restriction base (alias).
#[derive(Debug, Clone)]
pub enum SimpleType {
    /// Enumeration of allowed values.
    Enum {
        base: String,
        values: Vec<String>,
    },
    /// Restriction to a base type (pattern, min/max, etc.) - emit as alias or String.
    Alias {
        base: String,
        pattern: Option<String>,
    },
}

/// xs:complexType: element content and attributes.
#[derive(Debug, Clone)]
pub struct ComplexType {
    pub doc: Option<String>,
    /// Content: sequence, choice, or simpleContent extension.
    pub content: ComplexContent,
    /// Attributes from attributeGroup refs and direct xs:attribute.
    pub attribute_groups: Vec<String>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComplexContent {
    /// Empty (no child elements).
    Empty,
    /// xs:simpleContent/xs:extension base="...".
    SimpleExtension { base: String },
    /// xs:sequence, xs:choice, or xs:all of particles.
    Model(Particle),
}

/// Content model particle: sequence, choice, or element ref.
#[derive(Debug, Clone, PartialEq)]
pub enum Particle {
    Sequence(Vec<Particle>),
    Choice(Vec<Particle>),
    GroupRef(String),
    Element(ElementParticle),
}

/// Element in content model: name, type ref, min/max occurs.
#[derive(Debug, Clone, PartialEq)]
pub struct ElementParticle {
    pub name: String,
    pub type_name: Option<String>,
    pub min_occurs: u32,
    pub max_occurs: Option<u32>, // None = 1
}

/// xs:attribute.
#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub type_name: String,
    pub required: bool,
    pub default_value: Option<String>,
}

/// xs:group: named group with content model.
#[derive(Debug, Clone)]
pub struct Group {
    pub doc: Option<String>,
    pub content: Particle,
}

/// xs:attributeGroup: named group of attributes.
#[derive(Debug, Clone)]
pub struct AttributeGroup {
    pub doc: Option<String>,
    pub attributes: Vec<Attribute>,
    pub attribute_group_refs: Vec<String>,
}

/// Top-level xs:element (e.g. score-partwise).
#[derive(Debug, Clone)]
pub struct ElementDecl {
    pub doc: Option<String>,
    /// type="..." reference when present.
    pub type_name: Option<String>,
    /// Inline anonymous complexType when no type= attribute.
    pub inline_complex_type: Option<ComplexType>,
}

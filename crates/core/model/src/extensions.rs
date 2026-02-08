//! Extension point for custom data not in the MEI spec.
//!
//! Custom attributes and elements live under a dedicated namespace so they never
//! collide with MEI or future MEI changes. Round-trip is preserved: unknown
//! content in the extension namespace is read into [ExtensionBag] and written
//! back in a deterministic order (by namespace, then by name).
//!
//! **Namespace**: Project URI not yet defined. Use placeholder
//! `http://tusk.example.org/ns/ext` until a project URI is available.

use serde::{Deserialize, Serialize};

/// Placeholder URI for Tusk extension namespace. Replace with project URI when defined.
pub const TUSK_EXT_NS: &str = "http://tusk.example.org/ns/ext";

/// Bag of custom attributes and child elements at the root (or per-element) level.
///
/// Serialization order: custom attributes first (sorted by namespace, then local name),
/// then custom elements (sorted by namespace, then local name). Same namespace
/// declarations as in the document.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ExtensionBag {
    /// Custom attributes: (namespace_uri, local_name, value).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_attributes: Vec<(String, String, String)>,

    /// Custom child elements (namespace, local name, attributes, content).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub custom_elements: Vec<ExtensionElement>,
}

/// A single custom element in the extension namespace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtensionElement {
    pub namespace: String,
    pub local_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<(String, String, String)>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ExtensionContent>,
}

/// Content of an extension element: raw XML string or a list of child extension elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExtensionContent {
    /// Raw XML string (preserves order and any non-extension markup).
    Raw(String),
    /// Child extension elements only.
    Children(Vec<ExtensionElement>),
}

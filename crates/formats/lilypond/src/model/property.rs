//! Property operation types for LilyPond AST.
//!
//! These model `\override`, `\revert`, `\set`, `\unset`, and `\tweak`
//! commands which modify grob and context properties.

/// A dot-separated property path (e.g. `Staff.TimeSignature.color`).
///
/// Mirrors the `grob_prop_path` and `context_prop_spec` productions.
/// The segments are stored in order: `["Staff", "TimeSignature", "color"]`.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyPath {
    pub segments: Vec<String>,
}

impl PropertyPath {
    /// Create from a vec of segments.
    pub fn new(segments: Vec<String>) -> Self {
        Self { segments }
    }

    /// Format as dot-separated string (e.g. `Staff.TimeSignature.color`).
    pub fn to_dotted(&self) -> String {
        self.segments.join(".")
    }
}

/// A property value: the right-hand side of `\override ... = value` or
/// `\set ... = value`.
///
/// Mirrors the `scalar` production in the grammar. Values can be scheme
/// expressions, strings, numbers, or identifiers.
#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValue {
    /// A Scheme expression (e.g. `#red`, `#(rgb-color 1 0 0)`, `##t`).
    SchemeExpr(super::SchemeExpr),
    /// A quoted string (e.g. `"Piano"`).
    String(String),
    /// A number (e.g. `5`, `2.5`).
    Number(f64),
    /// An identifier reference (e.g. `\default`).
    Identifier(String),
}

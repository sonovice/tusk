//! Property operation types for LilyPond AST.
//!
//! These model `\override`, `\revert`, `\set`, `\unset`, and `\tweak`
//! commands which modify grob and context properties.

/// A single segment of a property path.
///
/// Most segments are named identifiers (`Staff`, `NoteHead`, `color`),
/// but LilyPond also allows Scheme expressions as path components:
/// `#'font-size` (quoted symbol) or `#'(bound-details left text)` (quoted list
/// that expands to multiple path symbols).
#[derive(Debug, Clone, PartialEq)]
pub enum PathSegment {
    /// A bare identifier (e.g. `Staff`, `color`, `bound-details`).
    Named(String),
    /// A Scheme expression used as a property path component
    /// (e.g. `#'font-size`, `#'(bound-details left text)`).
    Scheme(super::SchemeExpr),
}

/// A property path: a sequence of dot-separated segments that may include
/// Scheme-based components.
///
/// Examples:
/// - `Staff.TimeSignature.color` → three `Named` segments
/// - `#'font-size` → one `Scheme(Symbol("font-size"))` segment
/// - `Staff.NoteHead #'(bound-details left text)` → mixed segments
///
/// Mirrors the `grob_prop_path`, `context_prop_spec`, and `property_path`
/// productions.
#[derive(Debug, Clone, PartialEq)]
pub struct PropertyPath {
    pub segments: Vec<PathSegment>,
}

impl PropertyPath {
    /// Create from a vec of path segments.
    pub fn new_segments(segments: Vec<PathSegment>) -> Self {
        Self { segments }
    }

    /// Create from plain string segments (backwards compatibility).
    pub fn new(named: Vec<String>) -> Self {
        Self {
            segments: named.into_iter().map(PathSegment::Named).collect(),
        }
    }

    /// Format as dot-separated string for display. Scheme segments are
    /// shown in their source form (e.g. `#'font-size`).
    pub fn to_dotted(&self) -> String {
        use super::scheme::SchemeExpr;
        let mut out = String::new();
        for (i, seg) in self.segments.iter().enumerate() {
            match seg {
                PathSegment::Named(s) => {
                    if i > 0 {
                        out.push('.');
                    }
                    out.push_str(s);
                }
                PathSegment::Scheme(expr) => {
                    // Scheme segments follow the previous segment with a space
                    // (no dot), or start the path.
                    if i > 0 {
                        out.push(' ');
                    }
                    match expr {
                        SchemeExpr::Symbol(s) => {
                            out.push_str("#'");
                            out.push_str(s);
                        }
                        SchemeExpr::QuotedList(raw) => {
                            out.push_str("#'");
                            out.push_str(raw);
                        }
                        other => out.push_str(&format!("{other:?}")),
                    }
                }
            }
        }
        out
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

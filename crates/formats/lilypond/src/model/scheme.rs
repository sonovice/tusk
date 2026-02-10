//! Scheme expression AST types.
//!
//! Represents the subset of Scheme (Guile) expressions commonly used in
//! LilyPond files. This is NOT a full Guile parser — we handle the common
//! patterns used in scores and leave complex expressions as opaque text.

use super::Music;

/// A Scheme expression as used in LilyPond (preceded by `#` in source).
///
/// Examples:
/// - `#t` / `#f` — booleans (from `##t` / `##f` in source)
/// - `#42`, `#3.5` — numbers
/// - `#"hello"` — strings
/// - `#'symbol` — quoted symbols
/// - `#(list 1 2 3)` — S-expression lists
/// - `#red`, `#LEFT` — bare symbol references
/// - `#@(values 1 2)` — multi-value (rare)
/// - `##{ \markup { ... } #}` — embedded LilyPond
#[derive(Debug, Clone, PartialEq)]
pub enum SchemeExpr {
    /// Boolean: `##t` or `##f`.
    Bool(bool),
    /// Integer number: `#42`.
    Integer(i64),
    /// Floating-point number: `#3.5`.
    Float(f64),
    /// String literal: `#"hello"`.
    String(String),
    /// Quoted symbol: `#'symbol`.
    Symbol(String),
    /// Bare identifier reference: `#red`, `#LEFT`.
    Identifier(String),
    /// S-expression list: `#(func arg1 arg2)`.
    /// Stores the full parenthesized content as raw text (including parens).
    /// A full Scheme evaluator would parse into nested expressions,
    /// but for LilyPond roundtrip we preserve the exact text.
    List(String),
    /// Embedded LilyPond: `##{ music-or-markup #}`.
    EmbeddedLilypond(Vec<Music>),
    /// Pair literal: `#(a . b)` — stored as raw text like List.
    /// (Pairs are a subset of list syntax in Scheme.)
    /// Opaque/unparsed Scheme expression (fallback for complex cases).
    Raw(String),
}

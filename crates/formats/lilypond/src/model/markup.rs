//! Markup AST types for `\markup` and `\markuplist`.
//!
//! Mirrors the `full_markup`, `markup_top`, `markup_list`, `markup_braced_list`,
//! `markup_head_1_list`, `simple_markup`, `markup_word` productions in the grammar.

use super::ScoreBlock;

/// A markup expression.
///
/// Mirrors `full_markup`: either a structured markup tree or a single word.
#[derive(Debug, Clone, PartialEq)]
pub enum Markup {
    /// A plain word or symbol (STRING or SYMBOL in grammar).
    Word(String),
    /// A quoted string literal.
    String(String),
    /// A markup command/function applied to content.
    ///
    /// `\bold \italic "text"` is represented as nested commands:
    /// `Command { name: "bold", args: [Command { name: "italic", args: [Word("text")] }] }`
    Command { name: String, args: Vec<Markup> },
    /// A braced markup list: `{ markup1 markup2 ... }`.
    ///
    /// Corresponds to `markup_braced_list` in the grammar.
    List(Vec<Markup>),
    /// An embedded `\score { ... }` inside markup.
    Score(ScoreBlock),
    /// A `\markuplist` command result.
    MarkupList(MarkupList),
    /// An identifier reference (e.g. `\myMarkup`).
    Identifier(String),
    /// A Scheme expression (e.g. `#(markup ...)` or `#red`).
    Scheme(super::SchemeExpr),
    /// A number literal inside markup (e.g. standalone `42` or `3.5`).
    Number(f64),
}

/// A `\markuplist` expression.
///
/// Mirrors `full_markup_list` in the grammar.
#[derive(Debug, Clone, PartialEq)]
pub struct MarkupList {
    pub items: Vec<Markup>,
}

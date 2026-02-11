//! Markup AST types for `\markup` and `\markuplist`.
//!
//! Mirrors the `full_markup`, `markup_top`, `markup_list`, `markup_braced_list`,
//! `markup_head_1_list`, `simple_markup`, `markup_word`, `partial_markup`,
//! `markup_command_list`, and `simple_markup_noword` productions in the grammar.

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
    /// A partially applied markup function terminated by `\etc`.
    ///
    /// `\markup \bold \etc` — the prefix commands are stored without their
    /// final argument, creating a reusable partial function.
    /// Mirrors the `partial_markup` / `markup_partial_function` productions.
    Partial {
        /// Prefix command chain (outermost first).
        commands: Vec<String>,
        /// Arguments already supplied (from non-prefix commands in the chain).
        args: Vec<Markup>,
    },
}

/// A `\markuplist` expression.
///
/// Mirrors `full_markup_list` in the grammar.
#[derive(Debug, Clone, PartialEq)]
pub struct MarkupList {
    pub items: Vec<Markup>,
}

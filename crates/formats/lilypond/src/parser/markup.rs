//! Parser for `\markup` and `\markuplist` expressions.
//!
//! Implements the `full_markup`, `markup_top`, `markup_list`,
//! `markup_braced_list`, `markup_head_1_list`, `simple_markup`,
//! `markup_word`, `partial_markup`, `markup_command_list`,
//! `markup_uncomposed_list`, and `simple_markup_noword` grammar productions.

use super::{ParseError, Parser};
use crate::lexer::Token;
use crate::model::markup::{Markup, MarkupList};

/// Known markup prefix commands: take exactly ONE markup argument.
///
/// These correspond to `markup_head_1_item` in the grammar.
/// Only includes commands with signature `(markup?) -> markup`.
const MARKUP_PREFIX_COMMANDS: &[&str] = &[
    // Font modifiers
    "bold",
    "italic",
    "caps",
    "dynamic",
    "fontCaps",
    "huge",
    "large",
    "larger",
    "medium",
    "normal-text",
    "normalsize",
    "number",
    "roman",
    "sans",
    "serif",
    "small",
    "smallCaps",
    "smaller",
    "sub",
    "super",
    "teeny",
    "text",
    "tiny",
    "typewriter",
    "underline",
    "upright",
    // Single-markup layout modifiers
    "box",
    "bracket",
    "center-align",
    "circle",
    "left-align",
    "normalcolor",
    "oval",
    "parenthesize",
    "right-align",
    "rounded-box",
    "transparent",
    "vcenter",
    "whiteout",
];

/// Known markup list commands: take a braced list of markups.
///
/// These correspond to `MARKUP_LIST_FUNCTION` in the grammar.
const MARKUP_LIST_COMMANDS: &[&str] = &[
    "center-column",
    "column",
    "concat",
    "dir-column",
    "fill-line",
    "general-align",
    "justify",
    "left-column",
    "line",
    "overlay",
    "right-column",
    "table",
    "wordwrap",
    "string-lines",
    "wordwrap-lines",
];

/// Commands that return markup lists (used in `\markuplist` context).
///
/// These are `MARKUP_LIST_FUNCTION` commands that produce a list of markups
/// rather than a single markup, used in `markup_command_list` and
/// `markup_uncomposed_list` productions.
const MARKUP_LIST_RETURNING_COMMANDS: &[&str] = &[
    "column-lines",
    "wordwrap-lines",
    "string-lines",
    "map-markup-commands",
    "table-of-contents",
    "override-lines",
    "justified-lines",
    "wordwrap-internal",
];

impl<'src> Parser<'src> {
    // ──────────────────────────────────────────────────────────────────
    // \markup entry point: full_markup
    // ──────────────────────────────────────────────────────────────────

    /// Parse `\markup { ... }` or `\markup \command ...` or `\markup "word"`.
    ///
    /// Also handles `partial_markup`: `\markup \bold \etc`.
    pub(super) fn parse_markup(&mut self) -> Result<Markup, ParseError> {
        self.expect(&Token::Markup)?;
        self.parse_markup_top()
    }

    /// Parse the content after `\markup` keyword.
    fn parse_markup_top(&mut self) -> Result<Markup, ParseError> {
        match self.peek() {
            Token::BraceOpen => {
                let items = self.parse_markup_braced_list()?;
                Ok(Markup::List(items))
            }
            _ => self.parse_markup_composed(),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // \markuplist entry point
    // ──────────────────────────────────────────────────────────────────

    /// Parse `\markuplist { ... }` or `\markuplist \command ...`.
    ///
    /// Handles `markup_uncomposed_list`: braced list, list-returning command,
    /// or identifier.
    pub(super) fn parse_markuplist(&mut self) -> Result<MarkupList, ParseError> {
        self.expect(&Token::MarkupList)?;
        self.parse_markup_uncomposed_list()
    }

    /// Parse `markup_uncomposed_list`: the content after `\markuplist`.
    ///
    /// Grammar: `markup_braced_list | markup_command_list | markup_scm MARKUPLIST_IDENTIFIER`
    fn parse_markup_uncomposed_list(&mut self) -> Result<MarkupList, ParseError> {
        match self.peek() {
            Token::BraceOpen => {
                let items = self.parse_markup_braced_list()?;
                Ok(MarkupList { items })
            }
            Token::EscapedWord(name) if is_list_returning_command(name) => {
                let cmd = self.parse_markup_command_list()?;
                Ok(MarkupList { items: vec![cmd] })
            }
            _ => {
                let item = self.parse_markup_composed()?;
                Ok(MarkupList { items: vec![item] })
            }
        }
    }

    /// Parse `markup_command_list`: a list-returning command with arguments.
    ///
    /// Grammar: `MARKUP_LIST_FUNCTION markup_command_list_arguments`
    ///
    /// Like braced list commands, the items inside `{ ... }` are unpacked
    /// as individual args of the Command.
    fn parse_markup_command_list(&mut self) -> Result<Markup, ParseError> {
        let name = match self.advance()?.token {
            Token::EscapedWord(s) => s,
            other => {
                return Err(ParseError::Unexpected {
                    found: other,
                    offset: self.offset(),
                    expected: "markup list command".into(),
                });
            }
        };
        if *self.peek() == Token::BraceOpen {
            let items = self.parse_markup_braced_list()?;
            Ok(Markup::Command { name, args: items })
        } else {
            let arg = self.parse_markup_composed()?;
            Ok(Markup::Command {
                name,
                args: vec![arg],
            })
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // markup_braced_list: { markup markup ... }
    // ──────────────────────────────────────────────────────────────────

    fn parse_markup_braced_list(&mut self) -> Result<Vec<Markup>, ParseError> {
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_markup_item()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(items)
    }

    fn parse_markup_item(&mut self) -> Result<Markup, ParseError> {
        match self.peek() {
            Token::BraceOpen => {
                let items = self.parse_markup_braced_list()?;
                Ok(Markup::List(items))
            }
            _ => self.parse_markup_composed(),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Composed markup: optional prefix commands + simple markup
    // ──────────────────────────────────────────────────────────────────

    /// Parse optional chain of prefix commands applied to a simple markup.
    ///
    /// If the chain ends with `\etc` instead of a simple markup, produces
    /// `Markup::Partial` (the `partial_markup` / `markup_partial_function`
    /// grammar productions).
    fn parse_markup_composed(&mut self) -> Result<Markup, ParseError> {
        let mut prefixes: Vec<String> = Vec::new();
        while let Token::EscapedWord(name) = self.peek() {
            if is_prefix_command(name) {
                let name = name.clone();
                self.advance()?;
                prefixes.push(name);
            } else {
                break;
            }
        }

        // Check for partial markup: prefix chain terminated by \etc
        if *self.peek() == Token::Etc && !prefixes.is_empty() {
            self.advance()?;
            return Ok(Markup::Partial {
                commands: prefixes,
                args: Vec::new(),
            });
        }

        let mut markup = self.parse_simple_markup()?;

        // Wrap in nested Command from innermost to outermost
        for name in prefixes.into_iter().rev() {
            markup = Markup::Command {
                name,
                args: vec![markup],
            };
        }

        Ok(markup)
    }

    // ──────────────────────────────────────────────────────────────────
    // simple_markup: the atomic content unit
    // ──────────────────────────────────────────────────────────────────

    fn parse_simple_markup(&mut self) -> Result<Markup, ParseError> {
        match self.peek() {
            Token::String(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::String(s) => Ok(Markup::String(s)),
                    _ => unreachable!(),
                }
            }
            Token::Symbol(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Symbol(s) => Ok(Markup::Word(s)),
                    _ => unreachable!(),
                }
            }
            Token::NoteName(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::NoteName(s) => Ok(Markup::Word(s)),
                    _ => unreachable!(),
                }
            }
            Token::Unsigned(n) => {
                let n = *n;
                self.advance()?;
                Ok(Markup::Number(n as f64))
            }
            Token::Real(n) => {
                let n = *n;
                self.advance()?;
                Ok(Markup::Number(n))
            }
            Token::Score => {
                let score = self.parse_score_block()?;
                Ok(Markup::Score(score))
            }
            Token::Hash => {
                let expr = self.parse_scheme_expr()?;
                Ok(Markup::Scheme(expr))
            }
            // List command: \column { ... }, \line { ... }, \column-lines { ... }, etc.
            Token::EscapedWord(name) if is_braced_list_command(name) => {
                let name = name.clone();
                self.advance()?;
                if *self.peek() == Token::BraceOpen {
                    let items = self.parse_markup_braced_list()?;
                    Ok(Markup::Command { name, args: items })
                } else {
                    let arg = self.parse_markup_composed()?;
                    Ok(Markup::Command {
                        name,
                        args: vec![arg],
                    })
                }
            }
            // General command: consume scheme/number args then markup
            Token::EscapedWord(name) if is_any_known_command(name) => {
                let name = name.clone();
                self.advance()?;
                self.parse_command_args(name)
            }
            // Unknown \word: treat as identifier
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Markup::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            Token::BraceOpen => {
                let items = self.parse_markup_braced_list()?;
                Ok(Markup::List(items))
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "markup content (string, word, command, or braced list)".into(),
            }),
        }
    }

    /// Parse `simple_markup_noword`: like `simple_markup` but excludes bare
    /// words (Symbol/NoteName tokens). Used in disambiguation contexts where
    /// a bare word would be ambiguous.
    ///
    /// Grammar: `SCORE '{' score_body '}' | MARKUP_FUNCTION markup_command_basic_arguments
    ///         | markup_scm MARKUP_IDENTIFIER`
    #[allow(dead_code)]
    pub(super) fn parse_simple_markup_noword(&mut self) -> Result<Markup, ParseError> {
        match self.peek() {
            Token::String(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::String(s) => Ok(Markup::String(s)),
                    _ => unreachable!(),
                }
            }
            Token::Unsigned(n) => {
                let n = *n;
                self.advance()?;
                Ok(Markup::Number(n as f64))
            }
            Token::Real(n) => {
                let n = *n;
                self.advance()?;
                Ok(Markup::Number(n))
            }
            Token::Score => {
                let score = self.parse_score_block()?;
                Ok(Markup::Score(score))
            }
            Token::Hash => {
                let expr = self.parse_scheme_expr()?;
                Ok(Markup::Scheme(expr))
            }
            Token::EscapedWord(name) if is_braced_list_command(name) => {
                let name = name.clone();
                self.advance()?;
                if *self.peek() == Token::BraceOpen {
                    let items = self.parse_markup_braced_list()?;
                    Ok(Markup::Command { name, args: items })
                } else {
                    let arg = self.parse_markup_composed()?;
                    Ok(Markup::Command {
                        name,
                        args: vec![arg],
                    })
                }
            }
            Token::EscapedWord(name) if is_any_known_command(name) => {
                let name = name.clone();
                self.advance()?;
                self.parse_command_args(name)
            }
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Markup::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            Token::BraceOpen => {
                let items = self.parse_markup_braced_list()?;
                Ok(Markup::List(items))
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "markup content excluding bare words".into(),
            }),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Command argument parsing
    // ──────────────────────────────────────────────────────────────────

    /// Parse arguments for a non-prefix, non-list command.
    /// Consumes Scheme/number args, then a final markup/string/word.
    fn parse_command_args(&mut self, name: String) -> Result<Markup, ParseError> {
        let mut args = Vec::new();

        loop {
            match self.peek() {
                // Scheme argument
                Token::Hash => {
                    let expr = self.parse_scheme_expr()?;
                    args.push(Markup::Scheme(expr));
                }
                // Number argument
                Token::Unsigned(n) => {
                    let n = *n;
                    self.advance()?;
                    args.push(Markup::Number(n as f64));
                }
                Token::Real(n) => {
                    let n = *n;
                    self.advance()?;
                    args.push(Markup::Number(n));
                }
                // Braced list
                Token::BraceOpen => {
                    let items = self.parse_markup_braced_list()?;
                    args.push(Markup::List(items));
                    break;
                }
                // String: consume as final arg
                Token::String(_) => {
                    let tok = self.advance()?;
                    match tok.token {
                        Token::String(s) => args.push(Markup::String(s)),
                        _ => unreachable!(),
                    }
                    break;
                }
                // Word/symbol: consume as final arg
                Token::Symbol(_) | Token::NoteName(_) => {
                    let tok = self.advance()?;
                    match tok.token {
                        Token::Symbol(s) | Token::NoteName(s) => args.push(Markup::Word(s)),
                        _ => unreachable!(),
                    }
                    break;
                }
                // Composed markup as final arg
                Token::EscapedWord(_) | Token::Score => {
                    let inner = self.parse_markup_composed()?;
                    args.push(inner);
                    break;
                }
                // End of context
                _ => break,
            }
        }

        Ok(Markup::Command { name, args })
    }
}

// ──────────────────────────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────────────────────────

fn is_prefix_command(name: &str) -> bool {
    MARKUP_PREFIX_COMMANDS.contains(&name)
}

fn is_list_command(name: &str) -> bool {
    MARKUP_LIST_COMMANDS.contains(&name)
}

fn is_list_returning_command(name: &str) -> bool {
    MARKUP_LIST_RETURNING_COMMANDS.contains(&name) || MARKUP_LIST_COMMANDS.contains(&name)
}

/// Commands that take a braced list of markups as their primary argument.
/// Includes both regular list commands and list-returning commands.
fn is_braced_list_command(name: &str) -> bool {
    is_list_command(name) || MARKUP_LIST_RETURNING_COMMANDS.contains(&name)
}

/// Known non-prefix commands that take misc args (scheme, number, etc.)
/// before their markup content.
const MARKUP_OTHER_COMMANDS: &[&str] = &[
    "abs-fontsize",
    "char",
    "draw-circle",
    "draw-line",
    "finger",
    "flat",
    "fermata",
    "fontsize",
    "fraction",
    "fret-diagram",
    "fret-diagram-terse",
    "fret-diagram-verbose",
    "fromproperty",
    "halign",
    "harp-pedal",
    "hcenter-in",
    "hspace",
    "lookup",
    "lower",
    "magnify",
    "markalphabet",
    "markletter",
    "musicglyph",
    "natural",
    "normal-size-sub",
    "normal-size-super",
    "note",
    "note-by-number",
    "null",
    "on-the-fly",
    "override",
    "pad-around",
    "pad-markup",
    "pad-to-box",
    "pad-x",
    "page-link",
    "page-ref",
    "path",
    "pattern",
    "postscript",
    "property-recursive",
    "put-adjacent",
    "raise",
    "replace",
    "rest",
    "rest-by-number",
    "rotate",
    "scale",
    "sharp",
    "simple",
    "slashed-digit",
    "stencil",
    "strut",
    "tied-lyric",
    "translate",
    "translate-scaled",
    "triangle",
    "vspace",
    "verbatim-file",
    "with-color",
    "with-dimensions",
    "with-link",
    "with-outline",
    "with-url",
    "woodwind-diagram",
    "wordwrap-string",
    // extras
    "doubleflat",
    "doublesharp",
    "eyeglasses",
    "sesquiflat",
    "sesquisharp",
    "semiflat",
    "semisharp",
    "concat",
    "fill-with-pattern",
    "justify-field",
    "justify-string",
    "wordwrap-field",
    "wordwrap-internal",
];

fn is_any_known_command(name: &str) -> bool {
    is_prefix_command(name)
        || is_list_command(name)
        || is_list_returning_command(name)
        || MARKUP_OTHER_COMMANDS.contains(&name)
}

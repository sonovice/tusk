//! Parser helpers for property operations.
//!
//! Handles `\override`, `\revert`, `\set`, `\unset`, `\tweak`, and `\once`.
//! Extracted from `parser/mod.rs` to keep the main file under 1500 LOC.

use super::{ParseError, Parser};
use crate::lexer::Token;
use crate::model::*;

impl<'src> Parser<'src> {
    // ──────────────────────────────────────────────────────────────────
    // Property path: (NamedSegment | SchemeSegment) (. NamedSegment | SchemeSegment)*
    // ──────────────────────────────────────────────────────────────────

    /// Parse a property path with optional Scheme-based segments.
    ///
    /// Handles:
    /// - `Staff.TimeSignature.color` — dot-separated named segments
    /// - `#'font-size` — single Scheme quoted symbol
    /// - `#'(bound-details left text)` — Scheme quoted list
    /// - `Staff #'fontSize` — mixed: named context + Scheme property
    /// - `NoteHead.bound-details #'(left text)` — mixed dot/scheme
    pub(super) fn parse_property_path(&mut self) -> Result<PropertyPath, ParseError> {
        let first = self.parse_path_segment_mixed()?;
        let mut segments = vec![first];
        loop {
            if *self.peek() == Token::Dot {
                self.advance()?; // consume `.`
                segments.push(self.parse_path_segment_mixed()?);
            } else if self.is_scheme_path_ahead()? {
                // #'symbol or #'(list) — Scheme quoted path segment (no dot)
                let expr = self.parse_scheme_expr()?;
                segments.push(PathSegment::Scheme(expr));
            } else {
                break;
            }
        }
        Ok(PropertyPath::new_segments(segments))
    }

    /// Parse a single segment of a property path: named or Scheme.
    fn parse_path_segment_mixed(&mut self) -> Result<PathSegment, ParseError> {
        match &self.current.token {
            Token::Hash => {
                let expr = self.parse_scheme_expr()?;
                Ok(PathSegment::Scheme(expr))
            }
            Token::Symbol(s) => {
                let s = s.clone();
                self.advance()?;
                Ok(PathSegment::Named(s))
            }
            Token::NoteName(s) => {
                let s = s.clone();
                self.advance()?;
                Ok(PathSegment::Named(s))
            }
            Token::String(_) => {
                let s = self.expect_string()?;
                Ok(PathSegment::Named(s))
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "property path segment (symbol, note name, string, or #'scheme)".into(),
            }),
        }
    }

    /// Check if the next tokens form `#'` (a Scheme quoted path segment).
    ///
    /// Only `#'symbol` and `#'(list)` are valid as path components; bare
    /// `#value` (like `#red`, `#3`) is a property value, not a path segment.
    fn is_scheme_path_ahead(&mut self) -> Result<bool, ParseError> {
        if *self.peek() != Token::Hash {
            return Ok(false);
        }
        Ok(matches!(self.peek2()?, Token::Quote))
    }

    // ──────────────────────────────────────────────────────────────────
    // Property value: scalar (scheme, string, number, identifier)
    // ──────────────────────────────────────────────────────────────────

    /// Parse a property value (the RHS of `= value`).
    pub(super) fn parse_property_value(&mut self) -> Result<PropertyValue, ParseError> {
        match self.peek() {
            Token::Hash => {
                let expr = self.parse_scheme_expr()?;
                Ok(PropertyValue::SchemeExpr(expr))
            }
            Token::String(_) => {
                let s = self.expect_string()?;
                Ok(PropertyValue::String(s))
            }
            Token::Unsigned(n) => {
                let n = *n as f64;
                self.advance()?;
                Ok(PropertyValue::Number(n))
            }
            Token::Real(n) => {
                let n = *n;
                self.advance()?;
                Ok(PropertyValue::Number(n))
            }
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(PropertyValue::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "property value (scheme, string, number, or identifier)".into(),
            }),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // \override path = value
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_override(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \override
        let path = self.parse_property_path()?;
        self.expect(&Token::Equals)?;
        let value = self.parse_property_value()?;
        Ok(Music::Override { path, value })
    }

    // ──────────────────────────────────────────────────────────────────
    // \revert path
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_revert(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \revert
        let path = self.parse_property_path()?;
        Ok(Music::Revert { path })
    }

    // ──────────────────────────────────────────────────────────────────
    // \set path = value
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_set(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \set
        let path = self.parse_property_path()?;
        self.expect(&Token::Equals)?;
        let value = self.parse_property_value()?;
        Ok(Music::Set { path, value })
    }

    // ──────────────────────────────────────────────────────────────────
    // \unset path
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_unset(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \unset
        let path = self.parse_property_path()?;
        Ok(Music::Unset { path })
    }

    // ──────────────────────────────────────────────────────────────────
    // \once music
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_once(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \once
        let music = Box::new(self.parse_music()?);
        Ok(Music::Once { music })
    }

    // ──────────────────────────────────────────────────────────────────
    // \tweak path value (post-event)
    // ──────────────────────────────────────────────────────────────────

    /// Parse a `\tweak path value` post-event.
    pub(super) fn parse_tweak_post_event(&mut self) -> Result<PostEvent, ParseError> {
        self.advance()?; // consume \tweak
        let path = self.parse_property_path()?;
        let value = self.parse_property_value()?;
        Ok(PostEvent::Tweak { path, value })
    }

    // ──────────────────────────────────────────────────────────────────
    // Context mod item property operations
    // ──────────────────────────────────────────────────────────────────

    /// Parse `\override path = value` inside a context/with block.
    pub(super) fn parse_context_mod_override(&mut self) -> Result<ContextModItem, ParseError> {
        self.advance()?; // consume \override
        let path = self.parse_property_path()?;
        self.expect(&Token::Equals)?;
        let value = self.parse_property_value()?;
        Ok(ContextModItem::Override { path, value })
    }

    /// Parse `\revert path` inside a context/with block.
    pub(super) fn parse_context_mod_revert(&mut self) -> Result<ContextModItem, ParseError> {
        self.advance()?; // consume \revert
        let path = self.parse_property_path()?;
        Ok(ContextModItem::Revert { path })
    }

    /// Parse `\set path = value` inside a context/with block.
    pub(super) fn parse_context_mod_set(&mut self) -> Result<ContextModItem, ParseError> {
        self.advance()?; // consume \set
        let path = self.parse_property_path()?;
        self.expect(&Token::Equals)?;
        let value = self.parse_property_value()?;
        Ok(ContextModItem::Set { path, value })
    }

    /// Parse `\unset path` inside a context/with block.
    pub(super) fn parse_context_mod_unset(&mut self) -> Result<ContextModItem, ParseError> {
        self.advance()?; // consume \unset
        let path = self.parse_property_path()?;
        Ok(ContextModItem::Unset { path })
    }

    // ──────────────────────────────────────────────────────────────────
    // Context block and context mod items (moved from mod.rs)
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_context_mod_block(&mut self) -> Result<ContextModBlock, ParseError> {
        self.expect(&Token::Context)?;
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_context_mod_item()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(ContextModBlock { items })
    }

    pub(super) fn parse_context_mod_item(&mut self) -> Result<ContextModItem, ParseError> {
        match self.peek() {
            Token::Override => self.parse_context_mod_override(),
            Token::Revert => self.parse_context_mod_revert(),
            Token::Set => self.parse_context_mod_set(),
            Token::Unset => self.parse_context_mod_unset(),
            Token::Consists => {
                self.advance()?;
                let name = self.parse_engraver_name()?;
                Ok(ContextModItem::Consists(name))
            }
            Token::Remove => {
                self.advance()?;
                let name = self.parse_engraver_name()?;
                Ok(ContextModItem::Remove(name))
            }
            Token::Denies => {
                self.advance()?;
                let name = self.parse_context_def_string_arg()?;
                Ok(ContextModItem::Denies(name))
            }
            Token::Accepts => {
                self.advance()?;
                let name = self.parse_context_def_string_arg()?;
                Ok(ContextModItem::Accepts(name))
            }
            Token::Alias => {
                self.advance()?;
                let name = self.parse_context_def_string_arg()?;
                Ok(ContextModItem::Alias(name))
            }
            Token::DefaultChild => {
                self.advance()?;
                let name = self.parse_context_def_string_arg()?;
                Ok(ContextModItem::DefaultChild(name))
            }
            Token::Description => {
                self.advance()?;
                let name = self.parse_context_def_string_arg()?;
                Ok(ContextModItem::Description(name))
            }
            Token::Name => {
                self.advance()?;
                let name = self.parse_context_def_string_arg()?;
                Ok(ContextModItem::Name(name))
            }
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(ContextModItem::ContextRef(s)),
                    _ => unreachable!(),
                }
            }
            Token::Symbol(_) | Token::NoteName(_) => {
                let a = self.parse_output_def_assignment()?;
                Ok(ContextModItem::Assignment(a))
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "context modifier (\\consists, \\remove, \\ContextName, or assignment)"
                    .into(),
            }),
        }
    }

    /// Parse a string or symbol argument for context_def_mod keywords
    /// (`\denies`, `\accepts`, `\alias`, `\defaultchild`, `\description`, `\name`).
    fn parse_context_def_string_arg(&mut self) -> Result<String, ParseError> {
        match &self.current.token {
            Token::String(_) => self.expect_string(),
            Token::Symbol(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Symbol(s) => Ok(s),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "string or symbol argument".into(),
            }),
        }
    }

    pub(super) fn parse_engraver_name(&mut self) -> Result<String, ParseError> {
        match &self.current.token {
            Token::String(_) => self.expect_string(),
            Token::Symbol(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Symbol(s) => Ok(s),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "engraver name (string or symbol)".into(),
            }),
        }
    }
}

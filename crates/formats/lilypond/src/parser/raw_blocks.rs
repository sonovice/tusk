//! Parser helpers for Scheme expressions.
//!
//! Parses the subset of Scheme used in LilyPond: booleans, numbers, strings,
//! quoted symbols, identifiers, S-expression lists, and embedded LilyPond.

use super::{ParseError, Parser};
use crate::lexer::Token;
use crate::model::scheme::SchemeExpr;

impl<'src> Parser<'src> {
    // ──────────────────────────────────────────────────────────────────
    // Scheme expression (structured)
    // ──────────────────────────────────────────────────────────────────

    /// Parse a Scheme expression starting at `#`.
    ///
    /// Returns a structured [`SchemeExpr`].
    pub(super) fn parse_scheme_expr(&mut self) -> Result<SchemeExpr, ParseError> {
        self.expect(&Token::Hash)?;
        self.parse_scheme_after_hash()
    }

    /// Parse Scheme content after the leading `#` has been consumed.
    fn parse_scheme_after_hash(&mut self) -> Result<SchemeExpr, ParseError> {
        match self.peek().clone() {
            Token::ParenOpen => {
                // #( ... ) — S-expression list; capture raw text
                let start = self.offset();
                self.advance()?;
                let mut depth = 1u32;
                while depth > 0 && !self.at_eof() {
                    match self.peek() {
                        Token::ParenOpen => {
                            depth += 1;
                            self.advance()?;
                        }
                        Token::ParenClose => {
                            depth -= 1;
                            if depth > 0 {
                                self.advance()?;
                            }
                        }
                        _ => {
                            self.advance()?;
                        }
                    }
                }
                let close = self.expect(&Token::ParenClose)?;
                let raw = self.src[start..close.span.end].to_string();
                Ok(SchemeExpr::List(raw))
            }
            Token::Hash => {
                // ## — double-hash forms: ##t, ##f, or ##{ embedded LilyPond #}
                self.advance()?; // consume second #
                match self.peek().clone() {
                    Token::BraceOpen => {
                        // ##{ ... #} — embedded LilyPond
                        self.parse_embedded_lilypond()
                    }
                    Token::Symbol(ref s) if s == "t" => {
                        self.advance()?;
                        Ok(SchemeExpr::Bool(true))
                    }
                    Token::Symbol(ref s) if s == "f" => {
                        self.advance()?;
                        Ok(SchemeExpr::Bool(false))
                    }
                    Token::NoteName(ref s) if s == "f" => {
                        self.advance()?;
                        Ok(SchemeExpr::Bool(false))
                    }
                    _ => {
                        // Other ##X form — capture as raw
                        let start = self.offset();
                        let tok = self.advance()?;
                        let raw = format!("#{}", &self.src[start..tok.span.end]);
                        Ok(SchemeExpr::Raw(raw))
                    }
                }
            }
            Token::Quote => {
                // #'symbol — quoted symbol
                self.advance()?; // consume '
                let tok = self.advance()?;
                let name = self.src[tok.span.start..tok.span.end].to_string();
                Ok(SchemeExpr::Symbol(name))
            }
            Token::String(s) => {
                // #"string"
                let s = s.clone();
                self.advance()?;
                Ok(SchemeExpr::String(s))
            }
            Token::Unsigned(n) => {
                let val = n as i64;
                self.advance()?;
                Ok(SchemeExpr::Integer(val))
            }
            Token::Real(f) => {
                self.advance()?;
                Ok(SchemeExpr::Float(f))
            }
            Token::Dash => {
                // #-N — negative number
                self.advance()?;
                match self.peek().clone() {
                    Token::Unsigned(n) => {
                        let val = -(n as i64);
                        self.advance()?;
                        Ok(SchemeExpr::Integer(val))
                    }
                    Token::Real(f) => {
                        let val = -f;
                        self.advance()?;
                        Ok(SchemeExpr::Float(val))
                    }
                    _ => {
                        // Just #- something; treat as raw
                        let tok = self.advance()?;
                        let raw = format!("-{}", &self.src[tok.span.start..tok.span.end]);
                        Ok(SchemeExpr::Raw(raw))
                    }
                }
            }
            _ => {
                // #identifier — bare identifier (e.g. #red, #LEFT)
                let tok = self.advance()?;
                let name = self.src[tok.span.start..tok.span.end].to_string();
                Ok(SchemeExpr::Identifier(name))
            }
        }
    }

    /// Parse `##{ music... #}` embedded LilyPond.
    ///
    /// Called after `##` has been consumed and `{` is the current token.
    fn parse_embedded_lilypond(&mut self) -> Result<SchemeExpr, ParseError> {
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        loop {
            if self.at_eof() {
                return Err(ParseError::UnexpectedEof {
                    expected: "#} (end of embedded LilyPond)".into(),
                });
            }
            // Check for #} terminator
            if matches!(self.peek(), Token::Hash) {
                // Peek ahead: consume # and check for }
                let hash_offset = self.offset();
                self.advance()?;
                if matches!(self.peek(), Token::BraceClose) {
                    self.advance()?; // consume }
                    break;
                }
                // Not #} — the # starts a Scheme expression within embedded LilyPond.
                // Parse it as a scheme expr (we already consumed the #).
                let scheme = self.parse_scheme_after_hash()?;
                // Wrap the inner scheme in the Music tree as an event with raw text
                items.push(super::super::model::Music::Event(format!(
                    "#{}",
                    scheme_expr_to_suffix(&scheme)
                )));
                let _ = hash_offset; // used for documentation
                continue;
            }
            let m = self.parse_music()?;
            items.push(m);
        }
        Ok(SchemeExpr::EmbeddedLilypond(items))
    }
}

/// Convert a SchemeExpr to the text that follows `#` in source.
pub(crate) fn scheme_expr_to_suffix(expr: &SchemeExpr) -> String {
    match expr {
        SchemeExpr::Bool(true) => "#t".to_string(),
        SchemeExpr::Bool(false) => "#f".to_string(),
        SchemeExpr::Integer(n) => n.to_string(),
        SchemeExpr::Float(f) => f.to_string(),
        SchemeExpr::String(s) => {
            format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
        }
        SchemeExpr::Symbol(s) => format!("'{s}"),
        SchemeExpr::Identifier(s) => s.clone(),
        SchemeExpr::List(raw) => raw.clone(),
        SchemeExpr::EmbeddedLilypond(_) => "#{ ... #}".to_string(),
        SchemeExpr::Raw(raw) => raw.clone(),
    }
}

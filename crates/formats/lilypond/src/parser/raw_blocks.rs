//! Parser helpers for opaque/raw blocks (scheme).
//!
//! Extracted from `parser/mod.rs` to keep the main file under 1500 LOC.

use super::{ParseError, Parser};
use crate::lexer::Token;

impl<'src> Parser<'src> {
    // ──────────────────────────────────────────────────────────────────
    // Scheme (raw, for now)
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_scheme_raw(&mut self) -> Result<String, ParseError> {
        let start = self.offset();
        self.expect(&Token::Hash)?;

        match self.peek() {
            Token::ParenOpen => {
                // #( ... ) — balance parens
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
                self.expect(&Token::ParenClose)?;
                Ok(self.src[start..self.lexer.position()].to_string())
            }
            _ => {
                // #value — single token
                let tok = self.advance()?;
                Ok(self.src[start..tok.span.end].to_string())
            }
        }
    }
}

//! Parser helpers for music function calls and partial application.
//!
//! Extracted from `parser/mod.rs` to keep the main file under 1500 LOC.

use super::{ParseError, Parser};
use crate::lexer::Token;
use crate::model::*;

impl<'src> Parser<'src> {
    // ──────────────────────────────────────────────────────────────────
    // Identifier or music function call
    // ──────────────────────────────────────────────────────────────────

    /// Parse `\name` followed by optional function arguments.
    ///
    /// If arguments follow the identifier, produces `Music::MusicFunction`.
    /// If `\etc` terminates the argument list, produces `Music::PartialFunction`.
    /// Otherwise produces `Music::Identifier`.
    pub(super) fn parse_identifier_or_function_call(&mut self) -> Result<Music, ParseError> {
        let tok = self.advance()?;
        let name = match tok.token {
            Token::EscapedWord(s) => s,
            _ => unreachable!(),
        };

        let args = self.parse_function_args()?;

        // Check for \etc (partial application)
        if *self.peek() == Token::Etc {
            self.advance()?;
            return Ok(Music::PartialFunction { name, args });
        }

        if args.is_empty() {
            Ok(Music::Identifier(name))
        } else {
            Ok(Music::MusicFunction { name, args })
        }
    }

    /// Greedily collect function arguments after a function name.
    ///
    /// Consumes tokens that are unambiguously function arguments:
    /// - String literals
    /// - Numeric literals (unsigned or real)
    /// - Scheme expressions (`#...`)
    /// - `\default`
    /// - Duration-like fractions `N/M` (when not followed by music)
    /// - Braced music `{ ... }` and simultaneous music `<< ... >>`
    ///
    /// Stops when encountering tokens that can't be function arguments.
    fn parse_function_args(&mut self) -> Result<Vec<FunctionArg>, ParseError> {
        let mut args = Vec::new();
        loop {
            match self.peek() {
                Token::String(_) => {
                    let s = self.expect_string()?;
                    args.push(FunctionArg::String(s));
                }
                Token::Unsigned(_) => {
                    let tok = self.advance()?;
                    let n = match tok.token {
                        Token::Unsigned(n) => n,
                        _ => unreachable!(),
                    };
                    // Check for fraction N/M
                    if *self.peek() == Token::Slash {
                        self.advance()?; // consume /
                        if let Token::Unsigned(d) = self.peek() {
                            let d = *d;
                            self.advance()?;
                            // This is a fraction — store as two-part number
                            args.push(FunctionArg::Number(n as f64 / d as f64));
                        } else {
                            args.push(FunctionArg::Number(n as f64));
                        }
                    } else {
                        args.push(FunctionArg::Number(n as f64));
                    }
                }
                Token::Real(n) => {
                    let n = *n;
                    self.advance()?;
                    args.push(FunctionArg::Number(n));
                }
                Token::Hash => {
                    let raw = self.parse_scheme_raw()?;
                    args.push(FunctionArg::SchemeExpr(raw));
                }
                Token::Default => {
                    self.advance()?;
                    args.push(FunctionArg::Default);
                }
                Token::BraceOpen => {
                    let m = self.parse_sequential_music()?;
                    args.push(FunctionArg::Music(m));
                }
                Token::DoubleAngleOpen => {
                    let m = self.parse_simultaneous_music()?;
                    args.push(FunctionArg::Music(m));
                }
                _ => break,
            }
        }
        Ok(args)
    }
}

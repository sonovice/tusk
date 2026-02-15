//! Parser helpers for music function calls and partial application.
//!
//! Extracted from `parser/mod.rs` to keep the main file under 1500 LOC.

use super::{ParseError, Parser};
use crate::lexer::Token;
use crate::model::*;

// Valid steno duration base values.
const STENO_DURATIONS: &[u64] = &[1, 2, 4, 8, 16, 32, 64, 128, 256];

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
    /// - Numeric literals (unsigned or real), including fractions `N/M`
    /// - Scheme expressions (`#...`)
    /// - `\default`
    /// - Duration values (e.g. `4.` when an unsigned is a valid steno duration)
    /// - Symbol lists (`Staff.NoteHead.color` — dot-separated symbol chains)
    /// - Identifier arguments (`\varName`)
    /// - Braced music `{ ... }` and simultaneous music `<< ... >>`
    ///
    /// **`pitch_or_music` note**: Bare note names (`Token::NoteName`) are NOT
    /// consumed here because we lack function signature information to
    /// disambiguate pitch arguments from music events. Functions that need
    /// bare pitch arguments (`\transpose`, `\fixed`, `\relative`) are handled
    /// as special cases in `parse_music()`. For generic `\identifier` calls,
    /// a following pitch is parsed as a separate music event by the caller.
    ///
    /// Stops when encountering tokens that can't be function arguments.
    fn parse_function_args(&mut self) -> Result<Vec<FunctionArg>, ParseError> {
        let mut args = Vec::new();
        loop {
            // Check for symbol list first (requires two-token lookahead)
            if self.peek_is_symbol_list_start() {
                let sl = self.parse_symbol_list_arg()?;
                args.push(FunctionArg::SymbolList(sl));
                continue;
            }
            match self.peek() {
                Token::String(_) => {
                    let s = self.expect_string()?;
                    args.push(FunctionArg::String(s));
                }
                // Bare symbol as function argument (e.g. `\omit TupletNumber`,
                // `\keepWithTag X`, `\tag layout`). Only consumed when NOT
                // followed by `=` (which would indicate an assignment LHS).
                Token::Symbol(s) if s != "r" && s != "s" && s != "R" && s != "q" => {
                    let s = s.clone();
                    // Peek ahead: `Symbol =` is an assignment, not a function arg.
                    if matches!(self.peek2(), Ok(Token::Equals)) {
                        break;
                    }
                    self.advance()?;
                    args.push(FunctionArg::SymbolList(vec![s]));
                }
                Token::Unsigned(n) => {
                    let n = *n;
                    self.advance()?;
                    // Check for fraction N/M
                    if *self.peek() == Token::Slash {
                        self.advance()?; // consume /
                        if let Token::Unsigned(d) = self.peek() {
                            let d = *d;
                            self.advance()?;
                            args.push(FunctionArg::Number(n as f64 / d as f64));
                        } else {
                            args.push(FunctionArg::Number(n as f64));
                        }
                    } else if STENO_DURATIONS.contains(&n) && self.peek_is_duration_suffix() {
                        // Unsigned that is a valid duration base followed by dots
                        // → parse as duration argument
                        let dots = self.parse_dots();
                        let multipliers = self.parse_multipliers()?;
                        args.push(FunctionArg::Duration(Duration {
                            base: n as u32,
                            dots,
                            multipliers,
                        }));
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
                    let expr = self.parse_scheme_expr()?;
                    args.push(FunctionArg::SchemeExpr(expr));
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

    // ──────────────────────────────────────────────────────────────────
    // pitch_or_music disambiguation
    // ──────────────────────────────────────────────────────────────────

    /// Parse a `pitch_or_music` argument: either a bare pitch (note name with
    /// optional octave/accidental marks) or a full music expression.
    ///
    /// Mirrors the grammar's `pitch_or_music` production:
    /// - If the current token is a `NoteName`, parse it as a note event
    ///   (which may have duration and post-events, or be bare).
    /// - Otherwise, parse a full music expression.
    ///
    /// Used by callers that know they expect a pitch or music argument
    /// (e.g. context music bodies, some function argument positions).
    pub(super) fn parse_pitch_or_music(&mut self) -> Result<Music, ParseError> {
        if matches!(self.peek(), Token::NoteName(_)) {
            self.parse_note_event()
        } else {
            self.parse_music()
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Symbol list: Symbol (.Symbol)* as function argument
    // ──────────────────────────────────────────────────────────────────

    /// Check if current token starts a symbol list (Symbol followed by `.`).
    fn peek_is_symbol_list_start(&mut self) -> bool {
        if !matches!(&self.current.token, Token::Symbol(_)) {
            return false;
        }
        matches!(self.peek2(), Ok(Token::Dot))
    }

    /// Parse a `symbol_list_arg`: dot-separated symbols/strings/integers.
    ///
    /// Mirrors the grammar's `symbol_list_arg` / `symbol_list_rev` productions.
    /// Returns the segments as strings.
    fn parse_symbol_list_arg(&mut self) -> Result<Vec<String>, ParseError> {
        let first = self.parse_symbol_list_element()?;
        let mut segments = vec![first];
        while *self.peek() == Token::Dot {
            self.advance()?; // consume `.`
            segments.push(self.parse_symbol_list_element()?);
        }
        Ok(segments)
    }

    /// Parse a single element of a symbol list.
    fn parse_symbol_list_element(&mut self) -> Result<String, ParseError> {
        match &self.current.token {
            Token::Symbol(s) => {
                let s = s.clone();
                self.advance()?;
                Ok(s)
            }
            Token::NoteName(s) => {
                let s = s.clone();
                self.advance()?;
                Ok(s)
            }
            Token::String(_) => self.expect_string(),
            Token::Unsigned(n) => {
                let s = n.to_string();
                self.advance()?;
                Ok(s)
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "symbol list element (symbol, string, or number)".into(),
            }),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Duration argument helpers
    // ──────────────────────────────────────────────────────────────────

    /// Check if current position has a duration suffix (dot).
    /// Used to distinguish `4.` (duration) from `4` (number) in function args.
    fn peek_is_duration_suffix(&self) -> bool {
        *self.peek() == Token::Dot
    }
}

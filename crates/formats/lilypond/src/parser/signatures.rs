//! Parsing of signatures, tuplets, and fraction-based constructs.
//!
//! Extracted from `parser/mod.rs` to keep file sizes under 1500 LOC.

use crate::lexer::Token;
use crate::model::*;

use super::{ParseError, Parser};

impl<'src> Parser<'src> {
    /// Parse `\tuplet n/m [duration] { music }`.
    ///
    /// The fraction is stored as-is: `\tuplet 3/2` → numerator=3, denominator=2.
    /// An optional span duration may appear between the fraction and body.
    pub(super) fn parse_tuplet(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Tuplet)?;
        let (numerator, denominator) = self.parse_fraction("\\tuplet")?;
        // Optional span duration before the music body
        let span_duration = self.parse_optional_duration()?;
        let body = Box::new(self.parse_music()?);
        Ok(Music::Tuplet {
            numerator,
            denominator,
            span_duration,
            body,
        })
    }

    /// Parse `\times n/m { music }`.
    ///
    /// `\times` uses the inverse convention: `\times 2/3` means "play 3 in the
    /// time of 2", equivalent to `\tuplet 3/2`. We store in `\tuplet` form
    /// (swapped) for uniform representation.
    pub(super) fn parse_times(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Times)?;
        let (num, den) = self.parse_fraction("\\times")?;
        let body = Box::new(self.parse_music()?);
        // Invert: \times 2/3 → \tuplet 3/2
        Ok(Music::Tuplet {
            numerator: den,
            denominator: num,
            span_duration: None,
            body,
        })
    }

    /// Parse a fraction `N/M` (two unsigned integers separated by `/`).
    fn parse_fraction(&mut self, context: &str) -> Result<(u32, u32), ParseError> {
        let numerator = match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Unsigned(n) => n as u32,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: format!("numerator after {context}"),
                });
            }
        };
        self.expect(&Token::Slash)?;
        let denominator = match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Unsigned(n) => n as u32,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: format!("denominator after {context}"),
                });
            }
        };
        Ok((numerator, denominator))
    }

    // ──────────────────────────────────────────────────────────────────
    // \repeat type count { body } [\alternative { { a } { b } }]
    // ──────────────────────────────────────────────────────────────────

    /// Parse `\repeat type count { body } [\alternative { ... }]`.
    ///
    /// Grammar: `REPEAT simple_string unsigned_integer music [alternative_music]`
    pub(super) fn parse_repeat(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Repeat)?;

        // Repeat type: simple_string (e.g. "volta", "unfold", "percent", "tremolo")
        let type_name = self.expect_simple_string()?;
        let repeat_type =
            RepeatType::from_name(&type_name).ok_or_else(|| ParseError::Unexpected {
                found: Token::Symbol(type_name.clone()),
                offset: self.offset(),
                expected: "repeat type (volta, unfold, percent, tremolo, segno)".into(),
            })?;

        // Repeat count: unsigned integer
        let count = match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Unsigned(n) => n as u32,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "repeat count (unsigned integer)".into(),
                });
            }
        };

        // Music body
        let body = Box::new(self.parse_music()?);

        // Optional \alternative { ... }
        let alternatives = if *self.peek() == Token::Alternative {
            Some(self.parse_alternative_block()?)
        } else {
            None
        };

        Ok(Music::Repeat {
            repeat_type,
            count,
            body,
            alternatives,
        })
    }

    /// Parse `\alternative { { a } { b } ... }` — returns the list of alternative music blocks.
    ///
    /// Grammar: `ALTERNATIVE basic_music` where basic_music is typically `{ { a } { b } }`.
    fn parse_alternative_block(&mut self) -> Result<Vec<Music>, ParseError> {
        self.expect(&Token::Alternative)?;
        self.expect(&Token::BraceOpen)?;
        let mut alts = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            alts.push(self.parse_music()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(alts)
    }

    /// Parse a standalone `\alternative` at the music level.
    ///
    /// This is valid per the grammar (`alternative_music` is in `basic_music`),
    /// though rare outside of `\repeat`. We wrap it as a sequential block.
    pub(super) fn parse_alternative_as_music(&mut self) -> Result<Music, ParseError> {
        let alts = self.parse_alternative_block()?;
        // Standalone \alternative is sequential music per LilyPond grammar
        Ok(Music::Sequential(alts))
    }

    // ──────────────────────────────────────────────────────────────────
    // \clef "name"
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_clef(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \clef (EscapedWord("clef"))
        // Clef name: quoted string or bare symbol
        let name = match &self.current.token {
            Token::String(_) => self.expect_string()?,
            Token::Symbol(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Symbol(s) => s,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "clef name (string or symbol)".into(),
                });
            }
        };
        Ok(Music::Clef(Clef { name }))
    }

    // ──────────────────────────────────────────────────────────────────
    // \key pitch \mode
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_key_signature(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \key (EscapedWord("key"))

        // Parse tonic pitch (note name + optional octave marks)
        let offset = self.offset();
        let note_name = match &self.current.token {
            Token::NoteName(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::NoteName(s) => s,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "pitch after \\key".into(),
                });
            }
        };

        let (step, alter) =
            Pitch::from_note_name(&note_name).ok_or_else(|| ParseError::InvalidNoteName {
                name: note_name.clone(),
                offset,
            })?;

        // Optional octave marks (rarely used in \key but valid)
        let octave = self.parse_quotes();

        let pitch = Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        };

        // Parse mode: \major, \minor, \dorian, etc.
        let mode = match &self.current.token {
            Token::EscapedWord(s) => {
                let mode = Mode::from_name(s).ok_or_else(|| ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "mode (\\major, \\minor, \\dorian, etc.)".into(),
                })?;
                self.advance()?;
                mode
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "mode (\\major, \\minor, \\dorian, etc.)".into(),
                });
            }
        };

        Ok(Music::KeySignature(KeySignature { pitch, mode }))
    }

    // ──────────────────────────────────────────────────────────────────
    // \time [n+m+...]/d
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_time_signature(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::Time)?;

        // Parse numerator(s): N or N+M+... (compound)
        let mut numerators = Vec::new();
        match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Unsigned(n) => numerators.push(n as u32),
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "time signature numerator".into(),
                });
            }
        }

        // Check for additive numerators: +N+M...
        while *self.peek() == Token::Plus {
            self.advance()?; // consume +
            match self.peek() {
                Token::Unsigned(_) => {
                    let tok = self.advance()?;
                    match tok.token {
                        Token::Unsigned(n) => numerators.push(n as u32),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    return Err(ParseError::Unexpected {
                        found: self.current.token.clone(),
                        offset: self.offset(),
                        expected: "number after + in time signature".into(),
                    });
                }
            }
        }

        // Expect /
        self.expect(&Token::Slash)?;

        // Parse denominator
        let denominator = match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::Unsigned(n) => n as u32,
                    _ => unreachable!(),
                }
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "time signature denominator".into(),
                });
            }
        };

        Ok(Music::TimeSignature(TimeSignature {
            numerators,
            denominator,
        }))
    }
}

//! Figured bass mode parsing: `\figuremode`, `\figures`.

use crate::lexer::Token;
use crate::model::*;

use super::{ParseError, Parser};

impl<'src> Parser<'src> {
    /// Parse `\figuremode { ... }` — figured bass mode music.
    ///
    /// Grammar: `FIGUREMODE grouped_music_list` (in figure state).
    pub(super) fn parse_figure_mode(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \figuremode
        let body = Box::new(self.parse_figure_body()?);
        Ok(Music::FigureMode { body })
    }

    /// Parse `\figures { ... }` — shorthand for `\new FiguredBass \figuremode { ... }`.
    ///
    /// Grammar: `FIGURES mode_changing_head_with_context optional_context_mods
    ///           figure_mode_music`
    pub(super) fn parse_figures_shorthand(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \figures

        // Optional \with { ... }
        let with_block = self.parse_optional_context_mods()?;

        let body = Box::new(self.parse_figure_body()?);
        let figure_mode = Music::FigureMode { body };
        Ok(Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: "FiguredBass".to_string(),
            name: None,
            with_block,
            music: Box::new(figure_mode),
        })
    }

    /// Parse a figure mode body: `{ ... }` where content is interpreted as
    /// figured bass events rather than notes.
    fn parse_figure_body(&mut self) -> Result<Music, ParseError> {
        match self.peek() {
            Token::BraceOpen => {
                self.advance()?; // consume {
                let mut items = Vec::new();
                while *self.peek() != Token::BraceClose && !self.at_eof() {
                    items.push(self.parse_figure_element()?);
                }
                self.expect(&Token::BraceClose)?;
                Ok(Music::Sequential(items))
            }
            Token::EscapedWord(_) => {
                // Identifier reference (e.g. \myFigures)
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "figure mode body (braces or identifier)".into(),
            }),
        }
    }

    /// Parse a single element inside a figure mode body.
    ///
    /// Elements: figure events (`\< ... \>`), rests, skips, bar checks.
    fn parse_figure_element(&mut self) -> Result<Music, ParseError> {
        match self.peek() {
            // Bar check
            Token::Pipe => {
                self.advance()?;
                Ok(Music::BarCheck)
            }
            // Rest (no post-events in figure mode — \< \> are figure delimiters)
            Token::Symbol(s) if s == "r" => {
                self.advance()?;
                let duration = self.parse_optional_duration()?;
                Ok(Music::Rest(RestEvent {
                    duration,
                    post_events: vec![],
                }))
            }
            // Skip (no post-events in figure mode)
            Token::Symbol(s) if s == "s" => {
                self.advance()?;
                let duration = self.parse_optional_duration()?;
                Ok(Music::Skip(SkipEvent {
                    duration,
                    post_events: vec![],
                }))
            }
            // Identifier
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            // Figure event: <figures> or \<figures\> with duration
            Token::EscapedAngleOpen | Token::AngleOpen => self.parse_figure_event(),
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "figure mode element (figure event, rest, skip, or bar check)".into(),
            }),
        }
    }

    /// Parse a figure event: `<figures>dur` or `\<figures\>dur`.
    ///
    /// Grammar: `FIGURE_OPEN figure_list FIGURE_CLOSE`
    fn parse_figure_event(&mut self) -> Result<Music, ParseError> {
        let escaped = *self.peek() == Token::EscapedAngleOpen;
        self.advance()?; // consume < or \<
        let close = if escaped {
            Token::EscapedAngleClose
        } else {
            Token::AngleClose
        };
        let mut figures = Vec::new();
        while *self.peek() != close && !self.at_eof() {
            figures.push(self.parse_br_bass_figure()?);
        }
        self.expect(&close)?;
        let duration = self.parse_optional_duration()?;
        Ok(Music::Figure(FigureEvent { figures, duration }))
    }

    /// Parse a bracketed bass figure: `['['] bass_figure`.
    ///
    /// Grammar: `br_bass_figure: bass_figure | '[' bass_figure`
    fn parse_br_bass_figure(&mut self) -> Result<BassFigure, ParseError> {
        let bracket_start = if *self.peek() == Token::BracketOpen {
            self.advance()?; // consume [
            true
        } else {
            false
        };

        let mut fig = self.parse_bass_figure()?;
        fig.bracket_start = bracket_start;
        Ok(fig)
    }

    /// Parse a bass figure: number or `_`, followed by optional alterations,
    /// modifications, and bracket stop.
    ///
    /// Grammar: `bass_figure: FIGURE_SPACE | bass_number [FIGURE_ALTERATION_EXPR]
    ///           [figured_bass_modification] [']']`
    fn parse_bass_figure(&mut self) -> Result<BassFigure, ParseError> {
        let number = match self.peek() {
            // Figure space `_`
            Token::Underscore => {
                self.advance()?;
                None
            }
            // Figure number
            Token::Unsigned(n) => {
                let n = *n as u32;
                self.advance()?;
                Some(n)
            }
            _ => {
                return Err(ParseError::Unexpected {
                    found: self.current.token.clone(),
                    offset: self.offset(),
                    expected: "bass figure (number or _)".into(),
                });
            }
        };

        // Parse optional alteration: +, -, !
        let alteration = self.parse_figure_alteration();

        // Parse optional modifications: \+, \!, /, \\
        let modifications = self.parse_figure_modifications();

        // Parse optional bracket stop: ]
        let bracket_stop = if *self.peek() == Token::BracketClose {
            self.advance()?;
            true
        } else {
            false
        };

        Ok(BassFigure {
            number,
            alteration,
            modifications,
            bracket_start: false,
            bracket_stop,
        })
    }

    /// Parse figure alteration: `+`, `-`, `!` after a figure number.
    fn parse_figure_alteration(&mut self) -> FigureAlteration {
        match self.peek() {
            Token::Plus => {
                self.advance().ok();
                if *self.peek() == Token::Plus {
                    self.advance().ok();
                    FigureAlteration::DoubleSharp
                } else {
                    FigureAlteration::Sharp
                }
            }
            Token::Dash => {
                self.advance().ok();
                if *self.peek() == Token::Dash {
                    self.advance().ok();
                    FigureAlteration::DoubleFlat
                } else {
                    FigureAlteration::Flat
                }
            }
            Token::Exclamation => {
                self.advance().ok();
                FigureAlteration::ForcedNatural
            }
            _ => FigureAlteration::Natural,
        }
    }

    /// Parse figured bass modifications: `\+`, `\!`, `/`, `\\`.
    fn parse_figure_modifications(&mut self) -> Vec<FiguredBassModification> {
        let mut mods = Vec::new();
        loop {
            match self.peek() {
                Token::EscapedPlus => {
                    self.advance().ok();
                    mods.push(FiguredBassModification::Augmented);
                }
                Token::EscapedExclamation => {
                    self.advance().ok();
                    mods.push(FiguredBassModification::NoContinuation);
                }
                Token::Slash => {
                    self.advance().ok();
                    mods.push(FiguredBassModification::Diminished);
                }
                Token::DoubleBackslash => {
                    self.advance().ok();
                    mods.push(FiguredBassModification::AugmentedSlash);
                }
                _ => break,
            }
        }
        mods
    }
}

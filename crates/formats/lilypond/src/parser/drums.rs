//! Drum mode parsing: `\drummode`, `\drums`.

use crate::lexer::Token;
use crate::model::note::is_drum_pitch;
use crate::model::*;

use super::{ParseError, Parser};

impl<'src> Parser<'src> {
    /// Parse `\drummode { ... }` — drum mode music.
    ///
    /// Grammar: `DRUMMODE grouped_music_list` (in drum state).
    pub(super) fn parse_drum_mode(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \drummode
        let body = Box::new(self.parse_drum_body()?);
        Ok(Music::DrumMode { body })
    }

    /// Parse `\drums { ... }` — shorthand for `\new DrumStaff \drummode { ... }`.
    ///
    /// Grammar: `DRUMS mode_changing_head_with_context optional_context_mods
    ///           drum_mode_music`
    pub(super) fn parse_drums_shorthand(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \drums

        // Optional \with { ... }
        let with_block = self.parse_optional_context_mods()?;

        let body = Box::new(self.parse_drum_body()?);
        let drum_mode = Music::DrumMode { body };
        Ok(Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: "DrumStaff".to_string(),
            name: None,
            with_block,
            music: Box::new(drum_mode),
        })
    }

    /// Parse a drum mode body: `{ ... }` where content is interpreted as drum events.
    fn parse_drum_body(&mut self) -> Result<Music, ParseError> {
        match self.peek() {
            Token::BraceOpen => {
                self.advance()?; // consume {
                let mut items = Vec::new();
                while *self.peek() != Token::BraceClose && !self.at_eof() {
                    items.push(self.parse_drum_element()?);
                }
                self.expect(&Token::BraceClose)?;
                Ok(Music::Sequential(items))
            }
            Token::EscapedWord(_) => {
                // Identifier reference (e.g. \myDrums)
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "drum mode body (braces or identifier)".into(),
            }),
        }
    }

    /// Parse a single element inside a drum mode body.
    ///
    /// Elements: drum note events, rests, skips, bar checks, simultaneous (`<< >>`).
    fn parse_drum_element(&mut self) -> Result<Music, ParseError> {
        match self.peek() {
            // Bar check
            Token::Pipe => {
                self.advance()?;
                Ok(Music::BarCheck)
            }
            // Rest
            Token::Symbol(s) if s == "r" => {
                self.advance()?;
                let duration = self.parse_optional_duration()?;
                let post_events = self.parse_post_events();
                Ok(Music::Rest(RestEvent {
                    duration,
                    post_events,
                }))
            }
            // Skip
            Token::Symbol(s) if s == "s" => {
                self.advance()?;
                let duration = self.parse_optional_duration()?;
                let post_events = self.parse_post_events();
                Ok(Music::Skip(SkipEvent {
                    duration,
                    post_events,
                }))
            }
            // Multi-measure rest
            Token::Symbol(s) if s == "R" => {
                self.advance()?;
                let duration = self.parse_optional_duration()?;
                let post_events = self.parse_post_events();
                Ok(Music::MultiMeasureRest(MultiMeasureRestEvent {
                    duration,
                    post_events,
                }))
            }
            // Simultaneous music: << ... >>
            Token::DoubleAngleOpen => self.parse_drum_simultaneous(),
            // Chord: < ... >
            Token::AngleOpen => self.parse_drum_chord(),
            // Repeat
            Token::Repeat => self.parse_repeat(),
            // Identifier
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            // Drum pitch name (recognized as Symbol or NoteName by lexer)
            Token::Symbol(s) if is_drum_pitch(s) => self.parse_drum_note_event(),
            Token::NoteName(s) if is_drum_pitch(s) => self.parse_drum_note_event(),
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "drum mode element (drum pitch, rest, skip, or bar check)".into(),
            }),
        }
    }

    /// Parse a drum note event: `drum_pitch [duration] [post_events]`.
    fn parse_drum_note_event(&mut self) -> Result<Music, ParseError> {
        let tok = self.advance()?;
        let drum_type = match tok.token {
            Token::Symbol(s) | Token::NoteName(s) => s,
            _ => unreachable!(),
        };

        let duration = self.parse_optional_duration()?;
        let post_events = self.parse_post_events();

        Ok(Music::DrumNote(DrumNoteEvent {
            drum_type,
            duration,
            post_events,
        }))
    }

    /// Parse simultaneous music inside drum mode: `<< ... >>`.
    fn parse_drum_simultaneous(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::DoubleAngleOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::DoubleAngleClose && !self.at_eof() {
            // Skip `\\` voice separators
            if *self.peek() == Token::DoubleBackslash {
                self.advance()?;
                continue;
            }
            // Inside simultaneous, parse sequential blocks or individual elements
            match self.peek() {
                Token::BraceOpen => {
                    items.push(self.parse_drum_sequential()?);
                }
                _ => {
                    items.push(self.parse_drum_element()?);
                }
            }
        }
        self.expect(&Token::DoubleAngleClose)?;
        Ok(Music::Simultaneous(items))
    }

    /// Parse a sequential block inside drum mode: `{ ... }`.
    fn parse_drum_sequential(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::BraceOpen)?;
        let mut items = Vec::new();
        while *self.peek() != Token::BraceClose && !self.at_eof() {
            items.push(self.parse_drum_element()?);
        }
        self.expect(&Token::BraceClose)?;
        Ok(Music::Sequential(items))
    }

    /// Parse a drum chord: `< drum_pitch1 drum_pitch2 ... > duration post_events`.
    fn parse_drum_chord(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::AngleOpen)?;
        let mut drum_types = Vec::new();
        while *self.peek() != Token::AngleClose && !self.at_eof() {
            let tok = self.advance()?;
            let drum_type = match tok.token {
                Token::Symbol(s) | Token::NoteName(s) if is_drum_pitch(&s) => s,
                other => {
                    return Err(ParseError::Unexpected {
                        found: other,
                        offset: self.offset(),
                        expected: "drum pitch name in chord".into(),
                    });
                }
            };
            drum_types.push(drum_type);
        }
        self.expect(&Token::AngleClose)?;
        let duration = self.parse_optional_duration()?;
        let post_events = self.parse_post_events();

        Ok(Music::DrumChord(DrumChordEvent {
            drum_types,
            duration,
            post_events,
        }))
    }
}

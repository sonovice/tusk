//! Chord mode parsing: `\chordmode`, `\chords`.

use crate::lexer::Token;
use crate::model::*;

use super::{ParseError, Parser};

impl<'src> Parser<'src> {
    /// Parse `\chordmode { ... }` — chord mode music.
    ///
    /// Grammar: `CHORDMODE grouped_music_list` (in chord state).
    pub(super) fn parse_chord_mode(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \chordmode
        let body = Box::new(self.parse_chord_body()?);
        Ok(Music::ChordMode { body })
    }

    /// Parse `\chords { ... }` — shorthand for `\new ChordNames \chordmode { ... }`.
    ///
    /// Grammar: `CHORDS mode_changing_head_with_context optional_context_mods
    ///           chord_mode_music`
    pub(super) fn parse_chords_shorthand(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \chords

        // Optional \with { ... }
        let with_block = self.parse_optional_context_mods()?;

        let body = Box::new(self.parse_chord_body()?);
        let chord_mode = Music::ChordMode { body };
        Ok(Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: "ChordNames".to_string(),
            name: None,
            with_block,
            music: Box::new(chord_mode),
        })
    }

    /// Parse a chord mode body: `{ ... }` where content is interpreted as chord events.
    fn parse_chord_body(&mut self) -> Result<Music, ParseError> {
        match self.peek() {
            Token::BraceOpen => {
                self.advance()?; // consume {
                let mut items = Vec::new();
                while *self.peek() != Token::BraceClose && !self.at_eof() {
                    items.push(self.parse_chord_mode_element()?);
                }
                self.expect(&Token::BraceClose)?;
                Ok(Music::Sequential(items))
            }
            Token::EscapedWord(_) => {
                // Identifier reference (e.g. \myChords)
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "chord mode body (braces or identifier)".into(),
            }),
        }
    }

    /// Parse a single element inside a chord mode body.
    ///
    /// Elements: chord events (note + quality), rests, skips, bar checks.
    fn parse_chord_mode_element(&mut self) -> Result<Music, ParseError> {
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
            // Chord repetition
            Token::Symbol(s) if s == "q" => self.parse_chord_repetition(),
            // Identifier
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            // Chord event: root pitch with optional quality
            Token::NoteName(_) => self.parse_chord_mode_event(),
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "chord mode element (chord, rest, skip, or bar check)".into(),
            }),
        }
    }

    /// Parse a chord-mode event: `root [duration] [:quality] [^removals] [/inversion] [/+bass]`.
    ///
    /// Mirrors `new_chord` in the grammar.
    fn parse_chord_mode_event(&mut self) -> Result<Music, ParseError> {
        let offset = self.offset();
        let tok = self.advance()?;
        let note_name = match tok.token {
            Token::NoteName(s) => s,
            _ => unreachable!(),
        };

        let (step, alter) =
            Pitch::from_note_name(&note_name).ok_or_else(|| ParseError::InvalidNoteName {
                name: note_name.clone(),
                offset,
            })?;

        let octave = self.parse_quotes();
        let root = Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        };

        // Parse optional duration (before quality modifiers)
        let duration = self.parse_optional_duration()?;

        // Parse quality chain: `:` introduces modifiers/steps
        let mut quality = Vec::new();
        let mut removals = Vec::new();
        let mut inversion = None;
        let mut bass = None;

        // The grammar allows chaining: `:quality`, `^removals`, `/inversion`, `/+bass`
        // in any order after the root + duration.
        loop {
            match self.peek() {
                // `:` — quality modifiers and step numbers
                Token::Colon => {
                    self.advance()?; // consume `:`
                    self.parse_chord_quality_items(&mut quality)?;
                }
                // `^` — removals
                Token::Caret => {
                    self.advance()?; // consume `^`
                    self.parse_chord_removals(&mut removals)?;
                }
                // `/` — inversion or `/+` bass
                Token::Slash => {
                    self.advance()?; // consume `/`
                    if *self.peek() == Token::Plus {
                        self.advance()?; // consume `+`
                        bass = Some(self.parse_chord_pitch()?);
                    } else {
                        inversion = Some(self.parse_chord_pitch()?);
                    }
                }
                _ => break,
            }
        }

        let post_events = self.parse_post_events();

        Ok(Music::ChordModeEntry(ChordModeEvent {
            root,
            duration,
            quality,
            removals,
            inversion,
            bass,
            post_events,
        }))
    }

    /// Parse quality items after `:` — a sequence of modifiers and step numbers.
    ///
    /// Grammar: `chord_items: chord_item | chord_items chord_item`
    /// where `chord_item: chord_separator | step_numbers | CHORD_MODIFIER`
    /// and `step_numbers: step_number | step_numbers '.' step_number`.
    ///
    /// Items chain without separators between types (e.g. `dim7`, `m7.9`).
    /// Only step numbers use `.` as internal separator.
    ///
    /// Note: The lexer may produce `Real(7.9)` for `7.9` — we decompose it
    /// into separate steps (7 and 9) since `.` is a step separator in chord mode.
    fn parse_chord_quality_items(
        &mut self,
        items: &mut Vec<ChordQualityItem>,
    ) -> Result<(), ParseError> {
        let mut first = true;
        loop {
            // Consume optional `.` separator between quality items.
            // The serializer always uses `.` between items; LilyPond also
            // accepts it between modifier and step (e.g. `dim.7`).
            if !first && *self.peek() == Token::Dot {
                let saved = self.current.clone();
                self.advance()?; // tentatively consume `.`
                // If next is a valid quality item, continue; otherwise backtrack
                match self.peek() {
                    Token::Symbol(s) | Token::NoteName(s)
                        if ChordModifier::from_name(s).is_some() => {}
                    Token::Unsigned(_) | Token::Real(_) => {}
                    _ => {
                        self.current = saved;
                        break;
                    }
                }
            }
            first = false;

            match self.peek() {
                // Named modifier: m, min, aug, dim, maj, sus
                Token::Symbol(s) | Token::NoteName(s) if ChordModifier::from_name(s).is_some() => {
                    let name = match &self.current.token {
                        Token::Symbol(s) | Token::NoteName(s) => s.clone(),
                        _ => unreachable!(),
                    };
                    self.advance()?;
                    if let Some(m) = ChordModifier::from_name(&name) {
                        items.push(ChordQualityItem::Modifier(m));
                    }
                }
                // Step number: unsigned integer with optional +/-
                Token::Unsigned(n) => {
                    let number = *n as u8;
                    self.advance()?;
                    let alteration = self.parse_step_alteration();
                    items.push(ChordQualityItem::Step(ChordStep { number, alteration }));
                }
                // Real number like 7.9 — lexer merged `7.9` into one token.
                // Decompose into step(7) + step(9).
                Token::Real(r) => {
                    let r = *r;
                    self.advance()?;
                    Self::decompose_real_into_steps(r, items);
                }
                _ => break,
            }
        }
        Ok(())
    }

    /// Decompose a real number (e.g. 7.9, 11.13) into separate chord steps.
    fn decompose_real_into_steps(r: f64, items: &mut Vec<ChordQualityItem>) {
        let s = r.to_string();
        for part in s.split('.') {
            if let Ok(n) = part.parse::<u8>() {
                items.push(ChordQualityItem::Step(ChordStep {
                    number: n,
                    alteration: StepAlteration::Natural,
                }));
            }
        }
    }

    /// Parse chord removals after `^`: step numbers separated by `.`.
    fn parse_chord_removals(&mut self, removals: &mut Vec<ChordStep>) -> Result<(), ParseError> {
        // First removal
        if let Some(step) = self.try_parse_step()? {
            removals.push(step);
        }

        // Subsequent removals separated by `.`
        while *self.peek() == Token::Dot {
            self.advance()?; // consume `.`
            if let Some(step) = self.try_parse_step()? {
                removals.push(step);
            }
        }

        Ok(())
    }

    /// Try to parse a step number with optional alteration.
    fn try_parse_step(&mut self) -> Result<Option<ChordStep>, ParseError> {
        if let Token::Unsigned(n) = self.peek() {
            let number = *n as u8;
            self.advance()?;
            let alteration = self.parse_step_alteration();
            Ok(Some(ChordStep { number, alteration }))
        } else {
            Ok(None)
        }
    }

    /// Parse optional step alteration: `+` (sharp) or `-` (flat).
    fn parse_step_alteration(&mut self) -> StepAlteration {
        match self.peek() {
            Token::Plus => {
                let _ = self.advance();
                StepAlteration::Sharp
            }
            Token::Dash => {
                let _ = self.advance();
                StepAlteration::Flat
            }
            _ => StepAlteration::Natural,
        }
    }

    /// Parse a bare pitch for inversion/bass (note name + octave, no duration/quality).
    fn parse_chord_pitch(&mut self) -> Result<Pitch, ParseError> {
        let offset = self.offset();
        let tok = self.advance()?;
        let note_name = match tok.token {
            Token::NoteName(s) => s,
            other => {
                return Err(ParseError::Unexpected {
                    found: other,
                    offset,
                    expected: "pitch in chord inversion/bass".into(),
                });
            }
        };

        let (step, alter) =
            Pitch::from_note_name(&note_name).ok_or_else(|| ParseError::InvalidNoteName {
                name: note_name.clone(),
                offset,
            })?;

        let octave = self.parse_quotes();

        Ok(Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        })
    }
}

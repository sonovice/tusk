//! Note, chord, rest, and post-event parsing.
//!
//! Extracted from `mod.rs` to keep file sizes manageable. Contains pitch
//! parsing, octave marks (quotes), duration, tremolo, and all post-event
//! handling including direction-prefixed articulations.

use crate::lexer::Token;
use crate::model::*;

use super::{ParseError, ParseWarning, Parser};

impl<'src> Parser<'src> {
    // ──────────────────────────────────────────────────────────────────
    // Note event: pitch exclamations questions optional_duration [\rest]
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_note_event(&mut self) -> Result<Music, ParseError> {
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

        // Parse octave marks (quotes)
        let octave = self.parse_quotes();

        // Parse exclamations and questions (force/cautionary accidentals)
        let force_accidental = self.try_consume(&Token::Exclamation);
        let cautionary = self.try_consume(&Token::Question);

        // Parse octave check: `=` followed by optional octave marks
        let octave_check = if *self.peek() == Token::Equals {
            self.advance()?; // consume `=`
            Some(self.parse_quotes())
        } else {
            None
        };

        // Parse optional duration
        let duration = self.parse_optional_duration()?;

        // Check for erroneous quotes after duration (e.g. `c4'` or `c4,,`)
        let erroneous = self.parse_erroneous_quotes(offset);

        // Parse optional tremolo `:N`
        let tremolo = self.parse_optional_tremolo();

        // Check for \rest (pitched rest)
        let pitched_rest = self.try_consume(&Token::Rest);

        let mut post_events = self.parse_post_events();
        if let Some(t) = tremolo {
            post_events.insert(0, t);
        }

        // Apply erroneous quotes: add to octave_check if present, else to octave
        let (octave, octave_check) = if erroneous != 0 {
            if let Some(oc) = octave_check {
                (octave, Some(oc.saturating_add(erroneous)))
            } else {
                (octave.saturating_add(erroneous), octave_check)
            }
        } else {
            (octave, octave_check)
        };

        Ok(Music::Note(NoteEvent {
            pitch: Pitch {
                step,
                alter,
                octave,
                force_accidental,
                cautionary,
                octave_check,
            },
            duration,
            pitched_rest,
            post_events,
        }))
    }

    // ──────────────────────────────────────────────────────────────────
    // Chord: < pitch1 pitch2 ... > duration post_events
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_chord(&mut self) -> Result<Music, ParseError> {
        self.expect(&Token::AngleOpen)?;
        let mut pitches = Vec::new();
        while *self.peek() != Token::AngleClose && !self.at_eof() {
            pitches.push(self.parse_chord_body_pitch()?);
        }
        self.expect(&Token::AngleClose)?;
        let duration = self.parse_optional_duration()?;
        let tremolo = self.parse_optional_tremolo();
        let mut post_events = self.parse_post_events();
        if let Some(t) = tremolo {
            post_events.insert(0, t);
        }
        Ok(Music::Chord(ChordEvent {
            pitches,
            duration,
            post_events,
        }))
    }

    /// Parse a single pitch element inside a chord body.
    ///
    /// Mirrors `chord_body_element`: pitch with octave marks, accidental
    /// markers (! ?), but no duration (duration is shared on the chord).
    pub(super) fn parse_chord_body_pitch(&mut self) -> Result<Pitch, ParseError> {
        let offset = self.offset();
        let tok = self.advance()?;
        let note_name = match tok.token {
            Token::NoteName(s) => s,
            other => {
                return Err(ParseError::Unexpected {
                    found: other,
                    offset,
                    expected: "pitch in chord body".into(),
                });
            }
        };

        let (step, alter) =
            Pitch::from_note_name(&note_name).ok_or_else(|| ParseError::InvalidNoteName {
                name: note_name.clone(),
                offset,
            })?;

        let octave = self.parse_quotes();
        let force_accidental = self.try_consume(&Token::Exclamation);
        let cautionary = self.try_consume(&Token::Question);

        // Octave check inside chord body
        let octave_check = if *self.peek() == Token::Equals {
            self.advance()?;
            Some(self.parse_quotes())
        } else {
            None
        };

        Ok(Pitch {
            step,
            alter,
            octave,
            force_accidental,
            cautionary,
            octave_check,
        })
    }

    // ──────────────────────────────────────────────────────────────────
    // Rest (r), skip (s), multi-measure rest (R)
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_rest_or_skip(&mut self) -> Result<Music, ParseError> {
        let tok = self.advance()?;
        let kind = match tok.token {
            Token::Symbol(s) => s,
            _ => unreachable!(),
        };
        let duration = self.parse_optional_duration()?;
        let tremolo = self.parse_optional_tremolo();
        let mut post_events = self.parse_post_events();
        if let Some(t) = tremolo {
            post_events.insert(0, t);
        }
        match kind.as_str() {
            "r" => Ok(Music::Rest(RestEvent {
                duration,
                post_events,
            })),
            "s" => Ok(Music::Skip(SkipEvent {
                duration,
                post_events,
            })),
            "R" => Ok(Music::MultiMeasureRest(MultiMeasureRestEvent {
                duration,
                post_events,
            })),
            _ => unreachable!(),
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Chord repetition: q
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_chord_repetition(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume `q`
        let duration = self.parse_optional_duration()?;
        let tremolo = self.parse_optional_tremolo();
        let mut post_events = self.parse_post_events();
        if let Some(t) = tremolo {
            post_events.insert(0, t);
        }
        Ok(Music::ChordRepetition(ChordRepetitionEvent {
            duration,
            post_events,
        }))
    }

    // ──────────────────────────────────────────────────────────────────
    // Post-events: tie ~, slur ( ), phrasing slur \( \)
    // ──────────────────────────────────────────────────────────────────

    /// Parse optional tremolo: `:` followed by optional unsigned integer.
    ///
    /// Mirrors the `tremolo_type` production in the grammar. Returns `None`
    /// if no colon is present; returns `Tremolo(0)` for bare `:`.
    pub(super) fn parse_optional_tremolo(&mut self) -> Option<PostEvent> {
        if *self.peek() != Token::Colon {
            return None;
        }
        let _ = self.advance(); // consume `:`
        if let Token::Unsigned(n) = self.peek() {
            let n = *n as u32;
            let _ = self.advance();
            Some(PostEvent::Tremolo(n))
        } else {
            Some(PostEvent::Tremolo(0))
        }
    }

    pub(super) fn parse_post_events(&mut self) -> Vec<PostEvent> {
        let mut events = Vec::new();
        loop {
            match self.peek() {
                Token::Tilde => {
                    let _ = self.advance();
                    events.push(PostEvent::Tie);
                }
                Token::ParenOpen => {
                    let _ = self.advance();
                    events.push(PostEvent::SlurStart);
                }
                Token::ParenClose => {
                    let _ = self.advance();
                    events.push(PostEvent::SlurEnd);
                }
                Token::EscapedParenOpen => {
                    let _ = self.advance();
                    events.push(PostEvent::PhrasingSlurStart);
                }
                Token::EscapedParenClose => {
                    let _ = self.advance();
                    events.push(PostEvent::PhrasingSlurEnd);
                }
                Token::BracketOpen => {
                    let _ = self.advance();
                    events.push(PostEvent::BeamStart);
                }
                Token::BracketClose => {
                    let _ = self.advance();
                    events.push(PostEvent::BeamEnd);
                }
                Token::EscapedAngleOpen => {
                    let _ = self.advance();
                    events.push(PostEvent::Crescendo);
                }
                Token::EscapedAngleClose => {
                    let _ = self.advance();
                    events.push(PostEvent::Decrescendo);
                }
                Token::EscapedExclamation => {
                    let _ = self.advance();
                    events.push(PostEvent::HairpinEnd);
                }
                Token::EscapedWord(s) if note::is_dynamic_marking(s) => {
                    let s = s.clone();
                    let _ = self.advance();
                    events.push(PostEvent::Dynamic(s));
                }
                // Tweak: \tweak path value
                Token::Tweak => {
                    if let Ok(ev) = self.parse_tweak_post_event() {
                        events.push(ev);
                    } else {
                        break;
                    }
                }
                // Undirected ornaments/scripts: \trill, \mordent, \turn, etc.
                Token::EscapedWord(s) if note::is_ornament_or_script(s) => {
                    let s = s.clone();
                    let _ = self.advance();
                    events.push(PostEvent::NamedArticulation {
                        direction: note::Direction::Neutral,
                        name: s,
                    });
                }
                // Undirected string number: \1, \2, etc.
                Token::EscapedUnsigned(n) if *n <= 9 => {
                    let number = *n as u8;
                    let _ = self.advance();
                    events.push(PostEvent::StringNumber {
                        direction: note::Direction::Neutral,
                        number,
                    });
                }
                // Direction prefixes: -, ^, _ followed by script/fingering/articulation
                Token::Dash | Token::Caret | Token::Underscore => {
                    if let Some(ev) = self.try_parse_directed_post_event() {
                        events.push(ev);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        events
    }

    /// Try to parse a direction-prefixed post-event: `-X`, `^X`, `_X`.
    ///
    /// Returns `None` if the token after the direction prefix isn't a valid
    /// post-event (the direction token is NOT consumed in that case).
    fn try_parse_directed_post_event(&mut self) -> Option<PostEvent> {
        let direction = match self.peek() {
            Token::Dash => note::Direction::Neutral,
            Token::Caret => note::Direction::Up,
            Token::Underscore => note::Direction::Down,
            _ => return None,
        };

        // We need lookahead: peek past the direction to see what follows.
        // Save state for potential backtrack.
        let saved = self.current.clone();
        let _ = self.advance(); // consume direction token

        match self.peek() {
            // Script abbreviation: . - > ^ + ! _
            Token::Dot => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Dot,
                })
            }
            Token::Dash => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Dash,
                })
            }
            Token::AngleClose => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Accent,
                })
            }
            Token::Caret => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Marcato,
                })
            }
            Token::Plus => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Stopped,
                })
            }
            Token::Exclamation => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Staccatissimo,
                })
            }
            Token::Underscore => {
                let _ = self.advance();
                Some(PostEvent::Articulation {
                    direction,
                    script: note::ScriptAbbreviation::Portato,
                })
            }
            // Fingering: digit 0-9
            Token::Unsigned(n) if *n <= 9 => {
                let digit = *n as u8;
                let _ = self.advance();
                Some(PostEvent::Fingering { direction, digit })
            }
            // Named articulation: \name (e.g. \staccato, \accent, \trill)
            Token::EscapedWord(name) => {
                let name = name.clone();
                let _ = self.advance();
                Some(PostEvent::NamedArticulation { direction, name })
            }
            // String number: \1, \2, etc.
            Token::EscapedUnsigned(n) if *n <= 9 => {
                let number = *n as u8;
                let _ = self.advance();
                Some(PostEvent::StringNumber { direction, number })
            }
            // Text script: "string" after direction (gen_text_def)
            Token::String(s) => {
                let s = s.clone();
                let _ = self.advance();
                Some(PostEvent::TextScript {
                    direction,
                    text: crate::model::markup::Markup::String(s),
                })
            }
            // Text script: \markup {...} after direction (gen_text_def)
            Token::Markup => match self.parse_markup() {
                Ok(m) => Some(PostEvent::TextScript { direction, text: m }),
                Err(_) => {
                    self.current = saved;
                    None
                }
            },
            _ => {
                // Not a valid post-event after direction — backtrack
                self.current = saved;
                None
            }
        }
    }

    // ──────────────────────────────────────────────────────────────────
    // Quotes: octave marks (' and ,)
    // ──────────────────────────────────────────────────────────────────

    /// Parse octave marks (quotes `'` and commas `,`).
    ///
    /// Emits a warning if the marks mix directions (e.g. `c',` or `c,'`),
    /// mirroring the `erroneous_quotes` production in the reference grammar.
    /// The net octave offset is still computed and returned.
    pub(super) fn parse_quotes(&mut self) -> i8 {
        let mut octave: i8 = 0;
        let mut saw_up = false;
        let mut saw_down = false;
        let offset = self.offset();
        loop {
            match self.peek() {
                Token::Quote => {
                    saw_up = true;
                    octave = octave.saturating_add(1);
                    let _ = self.advance();
                }
                Token::Comma => {
                    saw_down = true;
                    octave = octave.saturating_sub(1);
                    let _ = self.advance();
                }
                _ => break,
            }
        }
        if saw_up && saw_down {
            self.warn(ParseWarning::MixedOctaveMarks { offset });
        }
        octave
    }

    /// Parse erroneous quotes after a duration (e.g. `c4'` or `c4,,`).
    ///
    /// Mirrors the `erroneous_quotes` production: octave marks that appear
    /// after the duration are misplaced. We consume them, emit a warning,
    /// and return the net offset so the caller can apply the correction.
    fn parse_erroneous_quotes(&mut self, note_offset: usize) -> i8 {
        if !matches!(self.peek(), Token::Quote | Token::Comma) {
            return 0;
        }
        let offset = self.offset();
        let net = self.parse_quotes();
        if net != 0 {
            self.warn(ParseWarning::OctaveAfterDuration {
                offset,
                note_offset,
            });
        }
        net
    }

    // ──────────────────────────────────────────────────────────────────
    // Optional duration: UNSIGNED dots multipliers
    // ──────────────────────────────────────────────────────────────────

    pub(super) fn parse_optional_duration(&mut self) -> Result<Option<Duration>, ParseError> {
        match self.peek() {
            Token::Unsigned(_) => {
                let tok = self.advance()?;
                let base = match tok.token {
                    Token::Unsigned(n) => n as u32,
                    _ => unreachable!(),
                };
                let dots = self.parse_dots();
                let multipliers = self.parse_multipliers()?;
                Ok(Some(Duration {
                    base,
                    dots,
                    multipliers,
                }))
            }
            _ => Ok(None),
        }
    }

    /// Parse dots: zero or more `.` tokens.
    pub(super) fn parse_dots(&mut self) -> u8 {
        let mut dots: u8 = 0;
        while *self.peek() == Token::Dot {
            dots = dots.saturating_add(1);
            let _ = self.advance();
        }
        dots
    }

    /// Parse multipliers: zero or more `*N` or `*N/M` sequences.
    pub(super) fn parse_multipliers(&mut self) -> Result<Vec<(u32, u32)>, ParseError> {
        let mut multipliers = Vec::new();
        while *self.peek() == Token::Star {
            let _ = self.advance(); // consume `*`
            if let Token::Unsigned(n) = self.peek() {
                let n = *n as u32;
                let _ = self.advance();
                if *self.peek() == Token::Slash {
                    let _ = self.advance(); // consume `/`
                    if let Token::Unsigned(d) = self.peek() {
                        let d = *d as u32;
                        let _ = self.advance();
                        multipliers.push((n, d));
                    } else {
                        // `*N/` without denominator — treat as `*N/1`
                        multipliers.push((n, 1));
                    }
                } else {
                    multipliers.push((n, 1));
                }
            }
        }
        Ok(multipliers)
    }

    /// Try to consume a specific token, returning true if consumed.
    pub(super) fn try_consume(&mut self, token: &Token) -> bool {
        if self.peek() == token {
            let _ = self.advance();
            true
        } else {
            false
        }
    }
}

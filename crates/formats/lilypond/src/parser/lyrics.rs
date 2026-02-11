//! Lyric mode parsing: `\lyricmode`, `\lyrics`, `\addlyrics`, `\lyricsto`.

use crate::lexer::Token;
use crate::model::*;

use super::{ParseError, Parser};

impl<'src> Parser<'src> {
    /// Parse `\lyricmode { ... }` — lyric mode music.
    ///
    /// Grammar: `LYRICMODE lyric_mode_music`
    /// where `lyric_mode_music` = `grouped_music_list` in lyric state.
    pub(super) fn parse_lyric_mode(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \lyricmode
        let body = Box::new(self.parse_lyric_body()?);
        Ok(Music::LyricMode { body })
    }

    /// Parse `\lyrics { ... }` — shorthand for `\new Lyrics \lyricmode { ... }`.
    ///
    /// Grammar: `LYRICS mode_changing_head_with_context optional_context_mods
    ///           lyric_mode_music`
    ///
    /// In practice `\lyrics { ... }` is equivalent to
    /// `\new Lyrics \lyricmode { ... }`.
    pub(super) fn parse_lyrics_shorthand(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \lyrics

        // Optional \with { ... }
        let with_block = self.parse_optional_context_mods()?;

        let body = Box::new(self.parse_lyric_body()?);
        let lyric_mode = Music::LyricMode { body };
        Ok(Music::ContextedMusic {
            keyword: ContextKeyword::New,
            context_type: "Lyrics".to_string(),
            name: None,
            with_block,
            music: Box::new(lyric_mode),
        })
    }

    /// Parse `\lyricsto "voice" [context_mods] { ... }` — attach lyrics to voice.
    ///
    /// Grammar: `LYRICSTO simple_string optional_context_mods lyric_mode_music`
    pub(super) fn parse_lyricsto(&mut self) -> Result<Music, ParseError> {
        self.advance()?; // consume \lyricsto
        let voice_id = self.expect_simple_string()?;
        // Optional \with { ... } (consumed but not stored — no separate field)
        let _with_block = self.parse_optional_context_mods()?;
        let lyrics = Box::new(self.parse_lyric_body()?);
        Ok(Music::LyricsTo { voice_id, lyrics })
    }

    /// Try to consume trailing `\addlyrics` after a music expression.
    ///
    /// Grammar: `new_lyrics: ADDLYRICS optional_context_mods lyric_mode_music
    ///           | new_lyrics ADDLYRICS ...`
    ///
    /// Returns `Some(Music::AddLyrics { ... })` if `\addlyrics` follows,
    /// wrapping the original music. Returns `None` if no `\addlyrics`.
    pub(super) fn try_wrap_addlyrics(&mut self, music: Music) -> Result<Music, ParseError> {
        if *self.peek() != Token::AddLyrics {
            return Ok(music);
        }

        let mut lyrics = Vec::new();
        while *self.peek() == Token::AddLyrics {
            self.advance()?; // consume \addlyrics
            // Optional \with { ... }
            let _with_block = self.parse_optional_context_mods()?;
            lyrics.push(self.parse_lyric_body()?);
        }

        Ok(Music::AddLyrics {
            music: Box::new(music),
            lyrics,
        })
    }

    /// Parse a lyric body: a `{ ... }` block where content is interpreted as
    /// lyric elements (syllables, hyphens, extenders) rather than notes.
    fn parse_lyric_body(&mut self) -> Result<Music, ParseError> {
        // Accept { ... }, \lyricmode { ... }, and identifier references
        match self.peek() {
            Token::BraceOpen => {
                self.advance()?; // consume {
                let mut items = Vec::new();
                while *self.peek() != Token::BraceClose && !self.at_eof() {
                    items.push(self.parse_lyric_element()?);
                }
                self.expect(&Token::BraceClose)?;
                Ok(Music::Sequential(items))
            }
            Token::LyricMode => {
                // \lyricmode { ... } — explicit lyric mode wrapper
                self.parse_lyric_mode()
            }
            Token::EscapedWord(_) => {
                // Identifier reference (e.g. \text)
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "lyric body (braces, \\lyricmode, or identifier)".into(),
            }),
        }
    }

    /// Parse a single lyric element inside a lyric body.
    ///
    /// Grammar: `lyric_element_music: lyric_element optional_notemode_duration
    ///           post_events`
    ///
    /// A lyric element is a word (Symbol/NoteName), string, or skip (`_`).
    fn parse_lyric_element(&mut self) -> Result<Music, ParseError> {
        match self.peek() {
            // Skip in lyrics: `_` means a skipped syllable (no text under this note)
            Token::Symbol(s) if s == "s" => {
                self.advance()?;
                let duration = self.parse_optional_duration()?;
                let post_events = self.parse_lyric_post_events();
                Ok(Music::Skip(SkipEvent {
                    duration,
                    post_events,
                }))
            }
            // Bar check in lyrics
            Token::Pipe => {
                self.advance()?;
                Ok(Music::BarCheck)
            }
            // Identifier reference (e.g. \markup, \set, etc.)
            Token::EscapedWord(_) => {
                let tok = self.advance()?;
                match tok.token {
                    Token::EscapedWord(s) => Ok(Music::Identifier(s)),
                    _ => unreachable!(),
                }
            }
            // Text syllable: Symbol, NoteName, or String
            Token::Symbol(_) | Token::NoteName(_) | Token::String(_) => {
                let text = match &self.current.token {
                    Token::Symbol(s) | Token::NoteName(s) => s.clone(),
                    Token::String(s) => s.clone(),
                    _ => unreachable!(),
                };
                self.advance()?;
                let duration = self.parse_optional_duration()?;
                let post_events = self.parse_lyric_post_events();
                Ok(Music::Lyric(LyricEvent {
                    text,
                    duration,
                    post_events,
                }))
            }
            // Underscore alone as lyric skip (LilyPond's `_` in lyric mode)
            Token::Underscore => {
                self.advance()?;
                // Check for `__` (extender) — but standalone `_` is a skip
                if *self.peek() == Token::Underscore {
                    // This is actually `__` at the start — not valid as a standalone element
                    // Treat as skip with extender
                    self.advance()?;
                    let duration = self.parse_optional_duration()?;
                    let mut post_events = self.parse_lyric_post_events();
                    post_events.insert(0, PostEvent::LyricExtender);
                    Ok(Music::Lyric(LyricEvent {
                        text: String::new(),
                        duration,
                        post_events,
                    }))
                } else {
                    // Simple skip `_`
                    let duration = self.parse_optional_duration()?;
                    let post_events = self.parse_lyric_post_events();
                    Ok(Music::Lyric(LyricEvent {
                        text: "_".to_string(),
                        duration,
                        post_events,
                    }))
                }
            }
            _ => Err(ParseError::Unexpected {
                found: self.current.token.clone(),
                offset: self.offset(),
                expected: "lyric element (syllable, skip, or bar check)".into(),
            }),
        }
    }

    /// Parse post-events in lyric mode.
    ///
    /// In lyric mode, `--` is a lyric hyphen and `__` is a lyric extender.
    /// These are two consecutive tokens (Dash+Dash or Underscore+Underscore)
    /// since the lexer doesn't have mode switching.
    fn parse_lyric_post_events(&mut self) -> Vec<PostEvent> {
        let mut events = Vec::new();
        loop {
            match self.peek() {
                // `--` lyric hyphen: two consecutive Dash tokens
                Token::Dash => {
                    let saved = self.current.clone();
                    let _ = self.advance();
                    if *self.peek() == Token::Dash {
                        let _ = self.advance();
                        events.push(PostEvent::LyricHyphen);
                    } else {
                        // Single dash — backtrack, not a lyric post-event
                        self.current = saved;
                        break;
                    }
                }
                // `__` lyric extender: two consecutive Underscore tokens
                Token::Underscore => {
                    let saved = self.current.clone();
                    let _ = self.advance();
                    if *self.peek() == Token::Underscore {
                        let _ = self.advance();
                        events.push(PostEvent::LyricExtender);
                    } else {
                        // Single underscore — backtrack
                        self.current = saved;
                        break;
                    }
                }
                Token::Tilde => {
                    let _ = self.advance();
                    events.push(PostEvent::Tie);
                }
                _ => break,
            }
        }
        events
    }
}

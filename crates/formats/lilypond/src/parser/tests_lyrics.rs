//! Parser tests for lyric mode constructs (Phase 20).

use crate::model::*;
use crate::parser::parse;
use crate::serializer::serialize;

fn parse_music_fragment(input: &str) -> Music {
    let file = parse(input).expect("parse failed");
    match &file.items[0] {
        ToplevelExpression::Music(m) => m.clone(),
        _ => panic!("expected Music, got {:?}", file.items[0]),
    }
}

// ── \lyricmode ──────────────────────────────────────────────────────

#[test]
fn parse_lyricmode_basic() {
    let m = parse_music_fragment("\\lyricmode { hello world }");
    match &m {
        Music::LyricMode { body } => match body.as_ref() {
            Music::Sequential(items) => {
                assert_eq!(items.len(), 2);
                match &items[0] {
                    Music::Lyric(le) => assert_eq!(le.text, "hello"),
                    other => panic!("expected Lyric, got {other:?}"),
                }
                match &items[1] {
                    Music::Lyric(le) => assert_eq!(le.text, "world"),
                    other => panic!("expected Lyric, got {other:?}"),
                }
            }
            other => panic!("expected Sequential, got {other:?}"),
        },
        other => panic!("expected LyricMode, got {other:?}"),
    }
}

#[test]
fn parse_lyricmode_with_duration() {
    let m = parse_music_fragment("\\lyricmode { hel4 lo8 }");
    match &m {
        Music::LyricMode { body } => match body.as_ref() {
            Music::Sequential(items) => {
                assert_eq!(items.len(), 2);
                match &items[0] {
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "hel");
                        assert_eq!(le.duration.as_ref().unwrap().base, 4);
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
                match &items[1] {
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "lo");
                        assert_eq!(le.duration.as_ref().unwrap().base, 8);
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
            }
            other => panic!("expected Sequential, got {other:?}"),
        },
        other => panic!("expected LyricMode, got {other:?}"),
    }
}

#[test]
fn parse_lyricmode_hyphen() {
    let m = parse_music_fragment("\\lyricmode { Hal -- le -- lu -- jah }");
    match &m {
        Music::LyricMode { body } => match body.as_ref() {
            Music::Sequential(items) => {
                assert_eq!(items.len(), 4);
                // "Hal" --
                match &items[0] {
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "Hal");
                        assert_eq!(le.post_events.len(), 1);
                        assert_eq!(le.post_events[0], PostEvent::LyricHyphen);
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
                // "le" --
                match &items[1] {
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "le");
                        assert_eq!(le.post_events.len(), 1);
                        assert_eq!(le.post_events[0], PostEvent::LyricHyphen);
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
                // "lu" --
                match &items[2] {
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "lu");
                        assert_eq!(le.post_events.len(), 1);
                        assert_eq!(le.post_events[0], PostEvent::LyricHyphen);
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
                // "jah" (no hyphen)
                match &items[3] {
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "jah");
                        assert!(le.post_events.is_empty());
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
            }
            other => panic!("expected Sequential, got {other:?}"),
        },
        other => panic!("expected LyricMode, got {other:?}"),
    }
}

#[test]
fn parse_lyricmode_extender() {
    let m = parse_music_fragment("\\lyricmode { hold __ me }");
    match &m {
        Music::LyricMode { body } => match body.as_ref() {
            Music::Sequential(items) => {
                assert_eq!(items.len(), 2);
                match &items[0] {
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "hold");
                        assert_eq!(le.post_events.len(), 1);
                        assert_eq!(le.post_events[0], PostEvent::LyricExtender);
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
                match &items[1] {
                    // "me" — note name "e" is consumed as part of word "me"
                    // Actually "me" is not a note name (m is not a-g for start)
                    // Wait — "me" starts with 'm' which is not a-g. It's a Symbol.
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "me");
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
            }
            other => panic!("expected Sequential, got {other:?}"),
        },
        other => panic!("expected LyricMode, got {other:?}"),
    }
}

#[test]
fn parse_lyricmode_note_names_as_text() {
    // In lyric mode, note names should be treated as syllable text
    let m = parse_music_fragment("\\lyricmode { do re mi fa }");
    match &m {
        Music::LyricMode { body } => match body.as_ref() {
            Music::Sequential(items) => {
                assert_eq!(items.len(), 4);
                // "do" is not a note name (starts with 'd' but "do" doesn't match note pattern)
                // Actually "d" IS a note name. Let me check: "do" - d+o... is_note_name("do") returns false
                // since "o" is not a valid suffix. So "do" is a Symbol.
                for (i, expected) in ["do", "re", "mi", "fa"].iter().enumerate() {
                    match &items[i] {
                        Music::Lyric(le) => assert_eq!(le.text, *expected),
                        other => panic!("expected Lyric for '{expected}', got {other:?}"),
                    }
                }
            }
            other => panic!("expected Sequential, got {other:?}"),
        },
        other => panic!("expected LyricMode, got {other:?}"),
    }
}

#[test]
fn parse_lyricmode_string_syllable() {
    let m = parse_music_fragment("\\lyricmode { \"hello world\" }");
    match &m {
        Music::LyricMode { body } => match body.as_ref() {
            Music::Sequential(items) => {
                assert_eq!(items.len(), 1);
                match &items[0] {
                    Music::Lyric(le) => assert_eq!(le.text, "hello world"),
                    other => panic!("expected Lyric, got {other:?}"),
                }
            }
            other => panic!("expected Sequential, got {other:?}"),
        },
        other => panic!("expected LyricMode, got {other:?}"),
    }
}

#[test]
fn parse_lyricmode_tie() {
    let m = parse_music_fragment("\\lyricmode { la~la }");
    match &m {
        Music::LyricMode { body } => match body.as_ref() {
            Music::Sequential(items) => {
                assert_eq!(items.len(), 2);
                match &items[0] {
                    Music::Lyric(le) => {
                        assert_eq!(le.text, "la");
                        assert_eq!(le.post_events, vec![PostEvent::Tie]);
                    }
                    other => panic!("expected Lyric, got {other:?}"),
                }
            }
            other => panic!("expected Sequential, got {other:?}"),
        },
        other => panic!("expected LyricMode, got {other:?}"),
    }
}

// ── \lyrics ────────────────────────────────────────────────────────

#[test]
fn parse_lyrics_shorthand() {
    let m = parse_music_fragment("\\lyrics { la la }");
    // \lyrics expands to \new Lyrics \lyricmode { ... }
    match &m {
        Music::ContextedMusic {
            keyword,
            context_type,
            name,
            music,
            ..
        } => {
            assert_eq!(*keyword, ContextKeyword::New);
            assert_eq!(context_type, "Lyrics");
            assert!(name.is_none());
            match music.as_ref() {
                Music::LyricMode { body } => match body.as_ref() {
                    Music::Sequential(items) => assert_eq!(items.len(), 2),
                    other => panic!("expected Sequential, got {other:?}"),
                },
                other => panic!("expected LyricMode, got {other:?}"),
            }
        }
        other => panic!("expected ContextedMusic, got {other:?}"),
    }
}

// ── \addlyrics ────────────────────────────────────────────────────

#[test]
fn parse_addlyrics_basic() {
    let m = parse_music_fragment("{ c4 d e f } \\addlyrics { one two three four }");
    match &m {
        Music::AddLyrics { music, lyrics } => {
            // Music is sequential { c d e f }
            match music.as_ref() {
                Music::Sequential(items) => assert_eq!(items.len(), 4),
                other => panic!("expected Sequential, got {other:?}"),
            }
            // One lyric block
            assert_eq!(lyrics.len(), 1);
            match &lyrics[0] {
                Music::Sequential(items) => {
                    assert_eq!(items.len(), 4);
                    match &items[0] {
                        Music::Lyric(le) => assert_eq!(le.text, "one"),
                        other => panic!("expected Lyric, got {other:?}"),
                    }
                }
                other => panic!("expected Sequential, got {other:?}"),
            }
        }
        other => panic!("expected AddLyrics, got {other:?}"),
    }
}

#[test]
fn parse_addlyrics_chained() {
    let m = parse_music_fragment(
        "{ c4 d e f } \\addlyrics { one two three four } \\addlyrics { la la la la }",
    );
    match &m {
        Music::AddLyrics { music, lyrics } => {
            match music.as_ref() {
                Music::Sequential(_) => {}
                other => panic!("expected Sequential, got {other:?}"),
            }
            assert_eq!(lyrics.len(), 2);
        }
        other => panic!("expected AddLyrics, got {other:?}"),
    }
}

#[test]
fn parse_addlyrics_with_hyphens() {
    let m = parse_music_fragment("{ c4 d e f } \\addlyrics { hel -- lo wo -- rld }");
    match &m {
        Music::AddLyrics { lyrics, .. } => {
            assert_eq!(lyrics.len(), 1);
            match &lyrics[0] {
                Music::Sequential(items) => {
                    assert_eq!(items.len(), 4);
                    match &items[0] {
                        Music::Lyric(le) => {
                            assert_eq!(le.text, "hel");
                            assert_eq!(le.post_events, vec![PostEvent::LyricHyphen]);
                        }
                        other => panic!("expected Lyric, got {other:?}"),
                    }
                }
                other => panic!("expected Sequential, got {other:?}"),
            }
        }
        other => panic!("expected AddLyrics, got {other:?}"),
    }
}

// ── \lyricsto ──────────────────────────────────────────────────────

#[test]
fn parse_lyricsto_basic() {
    let m = parse_music_fragment("\\lyricsto \"melody\" { do re mi }");
    match &m {
        Music::LyricsTo { voice_id, lyrics } => {
            assert_eq!(voice_id, "melody");
            match lyrics.as_ref() {
                Music::Sequential(items) => assert_eq!(items.len(), 3),
                other => panic!("expected Sequential, got {other:?}"),
            }
        }
        other => panic!("expected LyricsTo, got {other:?}"),
    }
}

#[test]
fn parse_lyricsto_with_identifier() {
    let m = parse_music_fragment("\\lyricsto \"one\" \\text");
    match &m {
        Music::LyricsTo { voice_id, lyrics } => {
            assert_eq!(voice_id, "one");
            match lyrics.as_ref() {
                Music::Identifier(s) => assert_eq!(s, "text"),
                other => panic!("expected Identifier, got {other:?}"),
            }
        }
        other => panic!("expected LyricsTo, got {other:?}"),
    }
}

// ── Roundtrip tests ────────────────────────────────────────────────

#[test]
fn roundtrip_lyricmode() {
    let input = "\\lyricmode { hello world }\n";
    let file = parse(input).expect("parse failed");
    let output = serialize(&file);
    let reparsed = parse(&output).expect("reparse failed");
    assert_eq!(file, reparsed);
}

#[test]
fn roundtrip_lyricmode_hyphen() {
    let input = "\\lyricmode { Hal -- le -- lu -- jah }\n";
    let file = parse(input).expect("parse failed");
    let output = serialize(&file);
    let reparsed = parse(&output).expect("reparse failed");
    assert_eq!(file, reparsed);
}

#[test]
fn roundtrip_lyricmode_extender() {
    let input = "\\lyricmode { hold __ me }\n";
    let file = parse(input).expect("parse failed");
    let output = serialize(&file);
    let reparsed = parse(&output).expect("reparse failed");
    assert_eq!(file, reparsed);
}

#[test]
fn roundtrip_addlyrics() {
    let input = "{ c4 d4 e4 f4 } \\addlyrics { one two three four }\n";
    let file = parse(input).expect("parse failed");
    let output = serialize(&file);
    let reparsed = parse(&output).expect("reparse failed");
    assert_eq!(file, reparsed);
}

#[test]
fn roundtrip_lyricsto() {
    let input = "\\lyricsto \"melody\" { do re mi }\n";
    let file = parse(input).expect("parse failed");
    let output = serialize(&file);
    let reparsed = parse(&output).expect("reparse failed");
    assert_eq!(file, reparsed);
}

#[test]
fn roundtrip_lyrics_shorthand() {
    let input = "\\lyrics { la la la }\n";
    let file = parse(input).expect("parse failed");
    let output = serialize(&file);
    let reparsed = parse(&output).expect("reparse failed");
    assert_eq!(file, reparsed);
}

// ── Fixture roundtrip ──────────────────────────────────────────────

#[test]
fn roundtrip_fixture_score_with_lyrics() {
    // Test the score block with \lyricsto
    let input = r#"\score {
  <<
    \new Voice = "melody" {
      c4 d e f
    }
    \new Lyrics \lyricsto "melody" {
      do re mi fa
    }
  >>
}
"#;
    let file = parse(input).expect("parse failed");
    let output = serialize(&file);
    let reparsed = parse(&output).expect("reparse failed");
    assert_eq!(file, reparsed);
}

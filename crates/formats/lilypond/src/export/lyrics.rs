//! Lyric export: MEI `<verse>` / `<syl>` → LilyPond lyrics.
//!
//! Extracts verse/syl children from notes and reconstructs LilyPond
//! lyric mode constructs (`\addlyrics`, `\lyricsto`, `\lyricmode`).

use tusk_model::elements::{LayerChild, NoteChild, SylChild, VerseChild};

use crate::model::Music;
use crate::model::note::{LyricEvent, PostEvent};

/// Information about lyrics attachment, parsed from staffDef label.
#[derive(Debug)]
pub(super) struct LyricsExportInfo {
    pub style: LyricsExportStyle,
}

/// How lyrics should be attached in the LilyPond output.
#[derive(Debug)]
pub(super) enum LyricsExportStyle {
    /// `\addlyrics { ... }` with count of lyric lines
    AddLyrics { count: usize },
    /// `\lyricsto "voice_id" { ... }`
    LyricsTo { voice_id: String },
    /// `\lyricmode { ... }` (standalone)
    LyricMode,
}

/// Parse lyrics export info from a staffDef label segment.
///
/// Label format: `lilypond:lyrics,STYLE[,voice=ID][,count=N]`
pub(super) fn parse_lyrics_label(label: &str) -> Option<LyricsExportInfo> {
    let rest = label.strip_prefix("lilypond:lyrics,")?;
    let parts: Vec<&str> = rest.split(',').collect();
    if parts.is_empty() {
        return None;
    }

    match parts[0] {
        "addlyrics" => {
            let count = parts
                .iter()
                .find_map(|p| p.strip_prefix("count="))
                .and_then(|c| c.parse().ok())
                .unwrap_or(1);
            Some(LyricsExportInfo {
                style: LyricsExportStyle::AddLyrics { count },
            })
        }
        "lyricsto" => {
            let voice_id = parts
                .iter()
                .find_map(|p| p.strip_prefix("voice="))
                .unwrap_or("")
                .to_string();
            Some(LyricsExportInfo {
                style: LyricsExportStyle::LyricsTo { voice_id },
            })
        }
        "lyricmode" => Some(LyricsExportInfo {
            style: LyricsExportStyle::LyricMode,
        }),
        _ => None,
    }
}

/// Extract lyric events from notes in a layer, for a given verse number.
///
/// Returns a list of `Music::Lyric` events, one per note, with hyphens
/// and extenders reconstructed from `@wordpos`, `@con`, and labels.
pub(super) fn extract_lyrics_from_layer(layer_children: &[LayerChild], verse_n: u32) -> Vec<Music> {
    let mut lyrics = Vec::new();
    extract_lyrics_from_children(layer_children, verse_n, &mut lyrics);
    lyrics
}

fn extract_lyrics_from_children(children: &[LayerChild], verse_n: u32, lyrics: &mut Vec<Music>) {
    let verse_n_str = verse_n.to_string();
    for child in children {
        match child {
            LayerChild::Note(note) => {
                if let Some(lyric) = extract_lyric_from_note_children(&note.children, &verse_n_str)
                {
                    lyrics.push(Music::Lyric(lyric));
                }
            }
            LayerChild::Chord(chord) => {
                if let Some(tusk_model::elements::ChordChild::Note(note)) = chord.children.first()
                    && let Some(lyric) =
                        extract_lyric_from_note_children(&note.children, &verse_n_str)
                {
                    lyrics.push(Music::Lyric(lyric));
                }
            }
            LayerChild::Beam(beam) => {
                for bc in &beam.children {
                    match bc {
                        tusk_model::elements::BeamChild::Note(note) => {
                            if let Some(lyric) =
                                extract_lyric_from_note_children(&note.children, &verse_n_str)
                            {
                                lyrics.push(Music::Lyric(lyric));
                            }
                        }
                        tusk_model::elements::BeamChild::Chord(chord) => {
                            if let Some(tusk_model::elements::ChordChild::Note(note)) =
                                chord.children.first()
                                && let Some(lyric) =
                                    extract_lyric_from_note_children(&note.children, &verse_n_str)
                            {
                                lyrics.push(Music::Lyric(lyric));
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

/// Extract a LyricEvent from a note's children for a specific verse number.
fn extract_lyric_from_note_children(
    children: &[NoteChild],
    verse_n_str: &str,
) -> Option<LyricEvent> {
    for nc in children {
        if let NoteChild::Verse(verse) = nc
            && verse.common.n.as_ref().is_some_and(|n| n.0 == verse_n_str)
        {
            return verse_to_lyric_event(verse);
        }
    }
    None
}

/// Convert a Verse element to a LyricEvent.
fn verse_to_lyric_event(verse: &tusk_model::elements::Verse) -> Option<LyricEvent> {
    // Get the first Syl child
    let syl = verse.children.iter().find_map(|vc| {
        if let VerseChild::Syl(syl) = vc {
            Some(syl.as_ref())
        } else {
            None
        }
    })?;

    // Extract text
    let text = syl
        .children
        .first()
        .map(|sc| {
            let SylChild::Text(t) = sc;
            t.clone()
        })
        .unwrap_or_default();

    if text.is_empty() {
        return None;
    }

    // Reconstruct post-events from MEI attributes
    let mut post_events = Vec::new();

    // Hyphen: con="d" means a dash connector follows
    if syl.syl_log.con.as_deref() == Some("d") {
        post_events.push(PostEvent::LyricHyphen);
    }

    // Extender: check for label
    if syl
        .common
        .label
        .as_deref()
        .is_some_and(|l| l == "lilypond:extender")
    {
        post_events.push(PostEvent::LyricExtender);
    }

    Some(LyricEvent {
        text,
        duration: None,
        post_events,
    })
}

/// Wrap staff music with lyrics constructs based on export info.
///
/// For `\addlyrics`: wraps the staff music in `Music::AddLyrics { music, lyrics }`
/// For `\lyricsto`: adds `Music::LyricsTo` as a sibling in simultaneous
pub(super) fn wrap_music_with_lyrics(
    staff_music: Music,
    layer_children: &[LayerChild],
    info: &LyricsExportInfo,
) -> Music {
    match &info.style {
        LyricsExportStyle::AddLyrics { count } => {
            let mut all_lyrics = Vec::new();
            for verse_n in 1..=*count {
                let lyric_items = extract_lyrics_from_layer(layer_children, verse_n as u32);
                if !lyric_items.is_empty() {
                    let lyric_body = Music::LyricMode {
                        body: Box::new(Music::Sequential(lyric_items)),
                    };
                    all_lyrics.push(lyric_body);
                }
            }
            if all_lyrics.is_empty() {
                staff_music
            } else {
                Music::AddLyrics {
                    music: Box::new(staff_music),
                    lyrics: all_lyrics,
                }
            }
        }
        LyricsExportStyle::LyricsTo { voice_id } => {
            let lyric_items = extract_lyrics_from_layer(layer_children, 1);
            if lyric_items.is_empty() {
                staff_music
            } else {
                let lyric_body = Music::LyricMode {
                    body: Box::new(Music::Sequential(lyric_items)),
                };
                let lyrics_to = Music::LyricsTo {
                    voice_id: voice_id.clone(),
                    lyrics: Box::new(lyric_body),
                };
                // Return the staff music as-is; lyrics_to will be added as sibling
                // The caller handles adding to simultaneous
                // Actually, we need to return the lyrics_to separately
                // For now, wrap in simultaneous with the staff music
                // This will be handled at a higher level
                Music::Simultaneous(vec![staff_music, lyrics_to])
            }
        }
        LyricsExportStyle::LyricMode => {
            // Standalone lyric mode — not wrapped
            let lyric_items = extract_lyrics_from_layer(layer_children, 1);
            if lyric_items.is_empty() {
                staff_music
            } else {
                // Just return music followed by lyrics in a sequential block
                Music::AddLyrics {
                    music: Box::new(staff_music),
                    lyrics: vec![Music::LyricMode {
                        body: Box::new(Music::Sequential(lyric_items)),
                    }],
                }
            }
        }
    }
}

//! Lyric export: MEI `<verse>` / `<syl>` → LilyPond lyrics.
//!
//! Extracts verse/syl children from notes and reconstructs LilyPond
//! lyric mode constructs (`\addlyrics`, `\lyricsto`, `\lyricmode`).

use tusk_model::elements::{LayerChild, NoteChild, SylChild, VerseChild};
use tusk_model::{LyricsInfo as ExtLyricsInfo, LyricsStyle};
use tusk_model::extensions::ExtensionStore;

use crate::model::Music;
use crate::model::note::{LyricEvent, PostEvent};
use crate::model::property::{PropertyPath, PropertyValue};
use crate::model::scheme::SchemeExpr;

/// Information about lyrics attachment, parsed from staffDef label.
#[derive(Debug)]
pub(super) struct LyricsExportInfo {
    pub style: LyricsExportStyle,
}

/// How lyrics should be attached in the LilyPond output.
#[derive(Debug)]
pub(super) enum LyricsExportStyle {
    /// `\addlyrics { ... }` with verse identifiers (may be numeric "1" or arbitrary "part1verse1")
    AddLyrics { verse_ids: Vec<String> },
    /// `\lyricsto "voice_id" { ... }`
    LyricsTo { voice_id: String },
    /// `\lyricmode { ... }` (standalone)
    LyricMode,
}

/// Convert a typed ExtLyricsInfo to LyricsExportInfo.
pub(super) fn ext_lyrics_info_to_export(ext: &ExtLyricsInfo) -> Option<LyricsExportInfo> {
    let style = match ext.style {
        LyricsStyle::AddLyrics => LyricsExportStyle::AddLyrics {
            verse_ids: (1..=ext.count.unwrap_or(1)).map(|n| n.to_string()).collect(),
        },
        LyricsStyle::LyricsTo => LyricsExportStyle::LyricsTo {
            voice_id: ext.voice_id.clone().unwrap_or_default(),
        },
        LyricsStyle::LyricMode => LyricsExportStyle::LyricMode,
    };
    Some(LyricsExportInfo { style })
}

/// Extract lyric events from notes across multiple layer-child slices, for a given verse ID.
///
/// Returns a list of `Music::Lyric` events, one per note, with hyphens
/// and extenders reconstructed from `@wordpos`, `@con`, and ext_store.
/// When `measure_numbers` is provided, inserts `Music::LineComment("m.N")`
/// at each measure boundary for debugging.
pub(super) fn extract_lyrics_from_layers(
    layer_slices: &[&[LayerChild]],
    verse_id: &str,
    ext_store: &ExtensionStore,
    measure_numbers: &[String],
) -> Vec<Music> {
    let mut lyrics = Vec::new();
    for (i, children) in layer_slices.iter().enumerate() {
        if let Some(n) = measure_numbers.get(i) {
            lyrics.push(Music::LineComment(format!("m.{}", n)));
        }
        extract_lyrics_from_children(children, verse_id, &mut lyrics, ext_store);
    }
    // Strip trailing skips/comments — no need to pad after last real syllable
    while lyrics
        .last()
        .is_some_and(|m| is_lyric_skip(m) || matches!(m, Music::LineComment(_)))
    {
        lyrics.pop();
    }
    // If no actual syllables exist (only skips/comments), return empty
    if !lyrics
        .iter()
        .any(|m| matches!(m, Music::Lyric(le) if le.text != "_"))
    {
        return Vec::new();
    }
    lyrics
}

/// LilyPond lyric skip: `_` in lyrics mode means "skip this note".
fn lyric_skip() -> Music {
    Music::Lyric(LyricEvent {
        text: "_".to_string(),
        duration: None,
        post_events: vec![],
    })
}

fn is_lyric_skip(m: &Music) -> bool {
    matches!(m, Music::Lyric(le) if le.text == "_")
}

/// Check if a note is "tied-to" (continuation of a tie) — \addlyrics skips these.
fn is_tied_to_note(note: &tusk_model::elements::Note) -> bool {
    note.note_anl.tie.as_ref().is_some_and(|t| t.0 == "m" || t.0 == "t")
}

/// Check if a note is a grace note — \addlyrics skips these.
fn is_grace_note(note: &tusk_model::elements::Note) -> bool {
    note.note_log.grace.is_some()
}

/// Check if a chord is "tied-to" — check chord-level tie or ANY note's tie.
/// LilyPond's `\lyricsto`/`\addlyrics` treat a chord as melismatic (skip it)
/// when any constituent note has an incoming tie, not just the first.
fn is_tied_to_chord(chord: &tusk_model::elements::Chord) -> bool {
    if chord.chord_anl.tie.as_ref().is_some_and(|t| t.0 == "m" || t.0 == "t") {
        return true;
    }
    chord.children.iter().any(|cc| {
        let tusk_model::elements::ChordChild::Note(note) = cc;
        is_tied_to_note(note)
    })
}

/// Check if a chord is a grace chord — \addlyrics skips these.
fn is_grace_chord(chord: &tusk_model::elements::Chord) -> bool {
    chord.chord_log.grace.is_some()
}

fn extract_lyrics_from_children(children: &[LayerChild], verse_id: &str, lyrics: &mut Vec<Music>, ext_store: &ExtensionStore) {
    let verse_n_str = verse_id;
    for child in children {
        match child {
            LayerChild::Note(note) => {
                // \addlyrics auto-skips tied-to and grace notes
                if is_tied_to_note(note) || is_grace_note(note) {
                    continue;
                }
                if let Some(lyric) = extract_lyric_from_note_children(&note.children, verse_n_str, ext_store) {
                    lyrics.push(Music::Lyric(lyric));
                } else {
                    lyrics.push(lyric_skip());
                }
            }
            LayerChild::Chord(chord) => {
                if is_tied_to_chord(chord) || is_grace_chord(chord) {
                    continue;
                }
                let lyric = chord.children.first().and_then(|cc| {
                    let tusk_model::elements::ChordChild::Note(note) = cc;
                    extract_lyric_from_note_children(&note.children, verse_n_str, ext_store)
                });
                if let Some(lyric) = lyric {
                    lyrics.push(Music::Lyric(lyric));
                } else {
                    lyrics.push(lyric_skip());
                }
            }
            LayerChild::Beam(beam) => {
                for bc in &beam.children {
                    match bc {
                        tusk_model::elements::BeamChild::Note(note) => {
                            if is_tied_to_note(note) || is_grace_note(note) {
                                continue;
                            }
                            if let Some(lyric) =
                                extract_lyric_from_note_children(&note.children, verse_n_str, ext_store)
                            {
                                lyrics.push(Music::Lyric(lyric));
                            } else {
                                lyrics.push(lyric_skip());
                            }
                        }
                        tusk_model::elements::BeamChild::Chord(chord) => {
                            if is_tied_to_chord(chord) || is_grace_chord(chord) {
                                continue;
                            }
                            let lyric = chord.children.first().and_then(|cc| {
                                let tusk_model::elements::ChordChild::Note(note) = cc;
                                extract_lyric_from_note_children(&note.children, verse_n_str, ext_store)
                            });
                            if let Some(lyric) = lyric {
                                lyrics.push(Music::Lyric(lyric));
                            } else {
                                lyrics.push(lyric_skip());
                            }
                        }
                        _ => {}
                    }
                }
            }
            // Rests, spaces, clefs, etc.: \addlyrics auto-skips these
            _ => {}
        }
    }
}

/// Extract a LyricEvent from a note's children for a specific verse number.
fn extract_lyric_from_note_children(
    children: &[NoteChild],
    verse_n_str: &str,
    ext_store: &ExtensionStore,
) -> Option<LyricEvent> {
    for nc in children {
        if let NoteChild::Verse(verse) = nc
            && verse.common.n.as_ref().is_some_and(|n| n.0 == verse_n_str)
        {
            return verse_to_lyric_event(verse, ext_store);
        }
    }
    None
}

/// Convert a Verse element to a LyricEvent.
fn verse_to_lyric_event(verse: &tusk_model::elements::Verse, ext_store: &ExtensionStore) -> Option<LyricEvent> {
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

    // Extender: check ext_store by syl xml:id
    if syl
        .common
        .xml_id
        .as_deref()
        .is_some_and(|id| ext_store.lyric_extender(id).is_some())
    {
        post_events.push(PostEvent::LyricExtender);
    }

    Some(LyricEvent {
        text,
        duration: None,
        post_events,
    })
}

/// Build the `\set melismaBusyProperties = #'(tie)` command.
fn melisma_set_cmd() -> Music {
    Music::Set {
        path: PropertyPath::new(vec!["melismaBusyProperties".to_string()]),
        value: PropertyValue::SchemeExpr(SchemeExpr::QuotedList("(tieMelismaBusy)".to_string())),
    }
}

/// Prepend `\set melismaBusyProperties = #'(tie)` into the innermost sequential
/// music block, so `\addlyrics` aligns one syllable per sounding note.
///
/// By default LilyPond's `\addlyrics` skips notes under slurs, ties, and manual
/// melisma markers. Our lyrics extraction only skips tied-to notes and grace notes,
/// so we disable slur melisma to match.
fn disable_slur_melisma(music: Music) -> Music {
    match music {
        // \new Staff { ... } → inject into the inner music
        Music::ContextedMusic {
            keyword,
            context_type,
            name,
            with_block,
            music: inner,
        } => Music::ContextedMusic {
            keyword,
            context_type,
            name,
            with_block,
            music: Box::new(disable_slur_melisma(*inner)),
        },
        // \drummode { ... } → inject into inner
        Music::DrumMode { body } => Music::DrumMode {
            body: Box::new(disable_slur_melisma(*body)),
        },
        // { item1 item2 ... } → prepend \set
        Music::Sequential(mut items) => {
            items.insert(0, melisma_set_cmd());
            Music::Sequential(items)
        }
        other => Music::Sequential(vec![melisma_set_cmd(), other]),
    }
}

/// Wrap staff music with `\new Lyrics \lyricsto` for multi-staff contexts.
///
/// Returns (staff_music with melisma disabled, vec of Lyrics context exprs).
/// Used when `\addlyrics` inside `<< >>` misaligns with multi-voice bars.
pub(super) fn wrap_music_with_lyricsto(
    staff_music: Music,
    layer_slices: &[&[LayerChild]],
    info: &LyricsExportInfo,
    ext_store: &ExtensionStore,
    voice_name: &str,
    measure_numbers: &[String],
) -> (Music, Vec<Music>) {
    let verse_ids: Vec<String> = match &info.style {
        LyricsExportStyle::AddLyrics { verse_ids } => verse_ids.clone(),
        _ => vec!["1".to_string()],
    };

    let mut lyrics_exprs = Vec::new();
    for verse_id in &verse_ids {
        let lyric_items = extract_lyrics_from_layers(layer_slices, verse_id, ext_store, measure_numbers);
        if !lyric_items.is_empty() {
            // \new Lyrics \lyricsto "voice" { ... }
            // \lyricsto implies lyric mode, so no \lyricmode wrapper needed
            let lyrics_to = Music::LyricsTo {
                voice_id: voice_name.to_string(),
                lyrics: Box::new(Music::Sequential(lyric_items)),
            };
            lyrics_exprs.push(Music::ContextedMusic {
                keyword: crate::model::ContextKeyword::New,
                context_type: "Lyrics".to_string(),
                name: None,
                with_block: None,
                music: Box::new(lyrics_to),
            });
        }
    }

    if lyrics_exprs.is_empty() {
        (staff_music, vec![])
    } else {
        (disable_slur_melisma(staff_music), lyrics_exprs)
    }
}

/// Wrap staff music with lyrics constructs based on export info.
///
/// For `\addlyrics`: wraps the staff music in `Music::AddLyrics { music, lyrics }`
/// For `\lyricsto`: adds `Music::LyricsTo` as a sibling in simultaneous
pub(super) fn wrap_music_with_lyrics(
    staff_music: Music,
    layer_slices: &[&[LayerChild]],
    info: &LyricsExportInfo,
    ext_store: &ExtensionStore,
    measure_numbers: &[String],
) -> Music {
    match &info.style {
        LyricsExportStyle::AddLyrics { verse_ids } => {
            let mut all_lyrics = Vec::new();
            for verse_id in verse_ids {
                let lyric_items = extract_lyrics_from_layers(layer_slices, verse_id, ext_store, measure_numbers);
                if !lyric_items.is_empty() {
                    // Use Sequential directly — \addlyrics implies lyric mode
                    all_lyrics.push(Music::Sequential(lyric_items));
                }
            }
            if all_lyrics.is_empty() {
                staff_music
            } else {
                Music::AddLyrics {
                    music: Box::new(disable_slur_melisma(staff_music)),
                    lyrics: all_lyrics,
                }
            }
        }
        LyricsExportStyle::LyricsTo { voice_id } => {
            let first_id = "1";
            let lyric_items = extract_lyrics_from_layers(layer_slices, first_id, ext_store, measure_numbers);
            if lyric_items.is_empty() {
                staff_music
            } else {
                // \lyricsto already implies lyric mode — no \lyricmode wrapper needed
                let lyrics_to = Music::LyricsTo {
                    voice_id: voice_id.clone(),
                    lyrics: Box::new(Music::Sequential(lyric_items)),
                };
                let lyrics_ctx = Music::ContextedMusic {
                    keyword: crate::model::ContextKeyword::New,
                    context_type: "Lyrics".to_string(),
                    name: None,
                    with_block: None,
                    music: Box::new(lyrics_to),
                };
                Music::Simultaneous(vec![staff_music, lyrics_ctx])
            }
        }
        LyricsExportStyle::LyricMode => {
            let first_id = "1";
            let lyric_items = extract_lyrics_from_layers(layer_slices, first_id, ext_store, measure_numbers);
            if lyric_items.is_empty() {
                staff_music
            } else {
                Music::AddLyrics {
                    music: Box::new(disable_slur_melisma(staff_music)),
                    lyrics: vec![Music::Sequential(lyric_items)],
                }
            }
        }
    }
}

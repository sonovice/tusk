//! Lyric import: LilyPond lyrics → MEI `<verse>` / `<syl>` on notes.
//!
//! Lyrics are collected as a flat list of `LyricSyllable` items that are
//! then matched 1:1 to notes in the staff layer.

use tusk_model::elements::{NoteChild, Syl, SylChild, Verse, VerseChild};
use tusk_model::ExtensionStore;

use crate::model::{LyricEvent, Music, PostEvent};

/// A collected lyric syllable ready to be attached to a note.
#[derive(Debug)]
pub(super) struct LyricSyllable {
    /// The syllable text.
    pub text: String,
    /// Whether a hyphen (`--`) follows this syllable.
    pub has_hyphen: bool,
    /// Whether an extender (`__`) follows this syllable.
    pub has_extender: bool,
}

/// Information about lyrics attached to a staff, for roundtrip.
#[derive(Debug)]
pub(super) struct LyricsInfo {
    /// The type of lyrics attachment.
    pub style: LyricsStyle,
    /// The lyric syllables collected from the lyrics body.
    pub syllables: Vec<LyricSyllable>,
}

/// How lyrics were attached in the LilyPond source.
#[derive(Debug, Clone)]
pub(super) enum LyricsStyle {
    /// `\addlyrics { ... }` (verse index for multiple addlyrics)
    AddLyrics { _index: usize },
    /// `\lyricsto "voice_id" { ... }`
    LyricsTo { voice_id: String },
    /// `\lyricmode { ... }` (standalone, no attachment)
    #[allow(dead_code)]
    LyricMode,
}

/// Collect lyric syllables from a lyrics music expression.
///
/// Walks the AST looking for `Music::Lyric(LyricEvent)` nodes and
/// converts them to flat `LyricSyllable` items, preserving hyphen
/// and extender information from post-events.
pub(super) fn collect_lyric_syllables(music: &Music) -> Vec<LyricSyllable> {
    let mut syllables = Vec::new();
    collect_lyrics_inner(music, &mut syllables);
    syllables
}

fn collect_lyrics_inner(music: &Music, syllables: &mut Vec<LyricSyllable>) {
    match music {
        Music::Lyric(lyric) => {
            syllables.push(lyric_event_to_syllable(lyric));
        }
        Music::LyricMode { body } => {
            collect_lyrics_inner(body, syllables);
        }
        Music::Sequential(items) | Music::Simultaneous(items) => {
            for item in items {
                collect_lyrics_inner(item, syllables);
            }
        }
        Music::ContextedMusic { music, .. } => {
            collect_lyrics_inner(music, syllables);
        }
        // Skip events are lyric placeholders (underscore `_` in lyric mode)
        Music::Skip(_) => {
            // A skip in lyric mode means "no syllable on this note"
            // We push an empty syllable to maintain 1:1 alignment
            syllables.push(LyricSyllable {
                text: String::new(),
                has_hyphen: false,
                has_extender: false,
            });
        }
        _ => {}
    }
}

fn lyric_event_to_syllable(lyric: &LyricEvent) -> LyricSyllable {
    let has_hyphen = lyric
        .post_events
        .iter()
        .any(|pe| matches!(pe, PostEvent::LyricHyphen));
    let has_extender = lyric
        .post_events
        .iter()
        .any(|pe| matches!(pe, PostEvent::LyricExtender));
    LyricSyllable {
        text: lyric.text.clone(),
        has_hyphen,
        has_extender,
    }
}

/// Extract lyrics info from an `AddLyrics` music construct.
///
/// Returns the music body (stripped of lyrics) and a list of lyrics info
/// for each `\addlyrics` block.
pub(super) fn extract_addlyrics(music: &Music) -> Option<(&Music, Vec<LyricsInfo>)> {
    if let Music::AddLyrics {
        music: body,
        lyrics,
    } = music
    {
        let mut infos = Vec::new();
        for (i, ly) in lyrics.iter().enumerate() {
            let syllables = collect_lyric_syllables(ly);
            infos.push(LyricsInfo {
                style: LyricsStyle::AddLyrics { _index: i },
                syllables,
            });
        }
        Some((body, infos))
    } else {
        None
    }
}

/// Extract lyrics info from a `LyricsTo` music construct.
pub(super) fn extract_lyricsto(music: &Music) -> Option<LyricsInfo> {
    if let Music::LyricsTo { voice_id, lyrics } = music {
        let syllables = collect_lyric_syllables(lyrics);
        Some(LyricsInfo {
            style: LyricsStyle::LyricsTo {
                voice_id: voice_id.clone(),
            },
            syllables,
        })
    } else {
        None
    }
}

/// Attach lyric syllables to notes in a layer as MEI Verse/Syl children.
///
/// Matches syllables to notes 1:1 (in order). Each syllable becomes a
/// `<verse n="VERSE_NUM"><syl>text</syl></verse>` child on the note.
///
/// `verse_n` is 1-based (first lyrics = verse 1, second = verse 2, etc.)
pub(super) fn attach_lyrics_to_layer(
    layer_children: &mut [tusk_model::elements::LayerChild],
    syllables: &[LyricSyllable],
    verse_n: u32,
    ext_store: &mut ExtensionStore,
) {
    let mut syl_idx = 0;
    for child in layer_children.iter_mut() {
        if syl_idx >= syllables.len() {
            break;
        }
        match child {
            tusk_model::elements::LayerChild::Note(note) => {
                let syl = &syllables[syl_idx];
                syl_idx += 1;
                if !syl.text.is_empty() {
                    let verse = build_verse(syl, verse_n, ext_store);
                    note.children.push(NoteChild::Verse(Box::new(verse)));
                }
            }
            tusk_model::elements::LayerChild::Chord(chord) => {
                let syl = &syllables[syl_idx];
                syl_idx += 1;
                if !syl.text.is_empty() {
                    // Attach verse to first note in chord
                    if let Some(tusk_model::elements::ChordChild::Note(note)) =
                        chord.children.first_mut()
                    {
                        let verse = build_verse(syl, verse_n, ext_store);
                        note.children.push(NoteChild::Verse(Box::new(verse)));
                    }
                }
            }
            tusk_model::elements::LayerChild::Rest(_)
            | tusk_model::elements::LayerChild::MRest(_) => {
                // Rests consume a syllable slot (skip/silence)
                syl_idx += 1;
            }
            tusk_model::elements::LayerChild::Beam(beam) => {
                // Process notes inside beam
                for bc in &mut beam.children {
                    if syl_idx >= syllables.len() {
                        break;
                    }
                    match bc {
                        tusk_model::elements::BeamChild::Note(note) => {
                            let syl = &syllables[syl_idx];
                            syl_idx += 1;
                            if !syl.text.is_empty() {
                                let verse = build_verse(syl, verse_n, ext_store);
                                note.children.push(NoteChild::Verse(Box::new(verse)));
                            }
                        }
                        tusk_model::elements::BeamChild::Chord(chord) => {
                            let syl = &syllables[syl_idx];
                            syl_idx += 1;
                            if !syl.text.is_empty()
                                && let Some(tusk_model::elements::ChordChild::Note(note)) =
                                    chord.children.first_mut()
                            {
                                let verse = build_verse(syl, verse_n, ext_store);
                                note.children.push(NoteChild::Verse(Box::new(verse)));
                            }
                        }
                        tusk_model::elements::BeamChild::Rest(_) => {
                            syl_idx += 1;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

/// Counter for generating synthetic syl IDs.
static SYL_COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// Build a MEI Verse element from a lyric syllable.
fn build_verse(syl: &LyricSyllable, verse_n: u32, ext_store: &mut ExtensionStore) -> Verse {
    let mut verse = Verse::default();
    verse.common.n = Some(tusk_model::generated::data::DataWord(verse_n.to_string()));

    let mut mei_syl = Syl::default();
    mei_syl.children.push(SylChild::Text(syl.text.clone()));

    // Set wordpos and con based on hyphen/extender
    if syl.has_hyphen {
        // This syllable is followed by a hyphen -> it's either initial or medial
        // We'll refine in a second pass (see below), but for now mark with con="d"
        mei_syl.syl_log.con = Some("d".to_string());
    }

    // Store extender in ext_store for roundtrip
    if syl.has_extender {
        let n = SYL_COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let syl_id = format!("ly-syl-{n}");
        mei_syl.common.xml_id = Some(syl_id.clone());
        ext_store.insert_lyric_extender(syl_id, tusk_model::LyricExtender);
    }

    verse.children.push(VerseChild::Syl(Box::new(mei_syl)));
    verse
}

/// Refine wordpos on syllables within a verse based on hyphen connectivity.
///
/// After initial attachment, we do a second pass to set @wordpos:
/// - Syllable with con="d" that starts a hyphenated word → wordpos="i" (initial)
/// - Syllable with con="d" that continues a hyphenated word → wordpos="m" (medial)
/// - Syllable that ends a hyphenated word (no con, but preceded by con="d") → wordpos="t" (terminal)
pub(super) fn refine_wordpos(
    layer_children: &mut [tusk_model::elements::LayerChild],
    verse_n: u32,
) {
    // Collect mutable references to all syl elements for the target verse
    let mut syls: Vec<&mut Syl> = Vec::new();
    collect_syls_from_layer(layer_children, verse_n, &mut syls);

    // Now set wordpos based on connector pattern
    let len = syls.len();
    for i in 0..len {
        let has_con = syls[i].syl_log.con.as_deref() == Some("d");
        let prev_has_con = if i > 0 {
            syls[i - 1].syl_log.con.as_deref() == Some("d")
        } else {
            false
        };

        if has_con && !prev_has_con {
            // Start of a hyphenated word
            syls[i].syl_log.wordpos = Some("i".to_string());
        } else if has_con && prev_has_con {
            // Middle of a hyphenated word
            syls[i].syl_log.wordpos = Some("m".to_string());
        } else if !has_con && prev_has_con {
            // End of a hyphenated word
            syls[i].syl_log.wordpos = Some("t".to_string());
        }
        // If !has_con && !prev_has_con → single syllable word, no wordpos needed
    }
}

/// Collect mutable references to Syl elements from layer children for a given verse.
fn collect_syls_from_layer<'a>(
    children: &'a mut [tusk_model::elements::LayerChild],
    verse_n: u32,
    syls: &mut Vec<&'a mut Syl>,
) {
    let verse_n_str = verse_n.to_string();
    for child in children.iter_mut() {
        match child {
            tusk_model::elements::LayerChild::Note(note) => {
                collect_syls_from_note_children(&mut note.children, &verse_n_str, syls);
            }
            tusk_model::elements::LayerChild::Chord(chord) => {
                for cc in &mut chord.children {
                    let tusk_model::elements::ChordChild::Note(note) = cc;
                    collect_syls_from_note_children(&mut note.children, &verse_n_str, syls);
                }
            }
            tusk_model::elements::LayerChild::Beam(beam) => {
                for bc in &mut beam.children {
                    match bc {
                        tusk_model::elements::BeamChild::Note(note) => {
                            collect_syls_from_note_children(&mut note.children, &verse_n_str, syls);
                        }
                        tusk_model::elements::BeamChild::Chord(chord) => {
                            for cc in &mut chord.children {
                                let tusk_model::elements::ChordChild::Note(note) = cc;
                                collect_syls_from_note_children(
                                    &mut note.children,
                                    &verse_n_str,
                                    syls,
                                );
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

fn collect_syls_from_note_children<'a>(
    children: &'a mut [NoteChild],
    verse_n_str: &str,
    syls: &mut Vec<&'a mut Syl>,
) {
    for nc in children.iter_mut() {
        if let NoteChild::Verse(verse) = nc
            && verse.common.n.as_ref().is_some_and(|n| n.0 == verse_n_str)
        {
            for vc in &mut verse.children {
                if let VerseChild::Syl(syl) = vc {
                    syls.push(syl);
                }
            }
        }
    }
}

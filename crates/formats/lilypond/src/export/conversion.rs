//! MEI -> LilyPond pitch/duration/event conversion for export.

use tusk_model::elements::ChordChild;
use tusk_model::extensions::ExtensionStore;
use tusk_model::generated::data::{DataAccidentalGesturalBasic, DataDurationCmn};

use crate::model::note::{ChordEvent, ChordRepetitionEvent, PostEvent};
use crate::model::pitch::Pitch;
use crate::model::{Duration, MultiMeasureRestEvent, Music, NoteEvent, RestEvent, SkipEvent};

/// MEI defaults extracted from scoreDef/staffDef for dur.default and oct.default.
#[derive(Debug, Clone, Default)]
pub(super) struct MeiDefaults {
    /// Default duration from @dur.default, pre-converted to LilyPond Duration.
    pub dur: Option<Duration>,
    /// Default octave from @oct.default (raw MEI octave value, 0-based).
    pub oct: Option<u64>,
}

/// Convert MEI DataDurationCmn to LilyPond duration base.
fn mei_dur_to_base(dur: &DataDurationCmn) -> u32 {
    match dur {
        DataDurationCmn::N1 => 1,
        DataDurationCmn::N2 => 2,
        DataDurationCmn::N4 => 4,
        DataDurationCmn::N8 => 8,
        DataDurationCmn::N16 => 16,
        DataDurationCmn::N32 => 32,
        DataDurationCmn::N64 => 64,
        DataDurationCmn::N128 => 128,
        DataDurationCmn::Long => 1, // fallback
        DataDurationCmn::Breve => 1,
        _ => 4,
    }
}

/// Convert MEI DataDuration to LilyPond Duration (for dur.default).
pub(super) fn mei_data_dur_to_ly(dur: &tusk_model::generated::data::DataDuration) -> Option<Duration> {
    match dur {
        tusk_model::generated::data::DataDuration::MeiDataDurationCmn(cmn) => Some(Duration {
            base: mei_dur_to_base(cmn),
            dots: 0,
            multipliers: Vec::new(),
        }),
        _ => None,
    }
}

/// Convert MEI gestural accidental to alter in half-steps.
fn accid_ges_to_alter(accid: &DataAccidentalGesturalBasic) -> f32 {
    match accid {
        DataAccidentalGesturalBasic::S => 1.0,
        DataAccidentalGesturalBasic::Ss => 2.0,
        DataAccidentalGesturalBasic::F => -1.0,
        DataAccidentalGesturalBasic::Ff => -2.0,
        DataAccidentalGesturalBasic::N => 0.0,
        _ => 0.0,
    }
}

/// Convert MEI octave (0-based) to LilyPond octave marks (relative to c = octave 3).
fn mei_oct_to_marks(oct: u64) -> i8 {
    (oct as i8) - 3
}

/// Extract duration from an MEI note, falling back to dur.default.
fn extract_note_duration(note: &tusk_model::elements::Note, defaults: &MeiDefaults) -> Option<Duration> {
    if let Some(ref dur) = note.note_log.dur {
        let base = match dur {
            tusk_model::generated::data::DataDuration::MeiDataDurationCmn(cmn) => mei_dur_to_base(cmn),
            _ => return None,
        };
        let dots = note.note_log.dots.as_ref().map(|d| d.0 as u8).unwrap_or(0);
        Some(Duration {
            base,
            dots,
            multipliers: Vec::new(),
        })
    } else {
        defaults.dur.clone()
    }
}

/// Extract duration from an MEI rest, falling back to dur.default.
fn extract_rest_duration(rest: &tusk_model::elements::Rest, defaults: &MeiDefaults) -> Option<Duration> {
    if let Some(ref dur) = rest.rest_log.dur {
        let base = match dur {
            tusk_model::generated::data::DataDurationrests::MeiDataDurationCmn(cmn) => {
                mei_dur_to_base(cmn)
            }
            _ => return None,
        };
        let dots = rest.rest_log.dots.as_ref().map(|d| d.0 as u8).unwrap_or(0);
        Some(Duration {
            base,
            dots,
            multipliers: Vec::new(),
        })
    } else {
        defaults.dur.clone()
    }
}

// ---------------------------------------------------------------------------
// Event conversion
// ---------------------------------------------------------------------------

/// Extract a LilyPond Pitch from an MEI Note (for use inside chords -- no duration).
fn extract_pitch_from_note(note: &tusk_model::elements::Note, defaults: &MeiDefaults) -> Pitch {
    let step = note
        .note_log
        .pname
        .as_ref()
        .and_then(|p| p.0.chars().next())
        .unwrap_or('c');

    let octave = note
        .note_log
        .oct
        .as_ref()
        .map(|o| mei_oct_to_marks(o.0))
        .unwrap_or_else(|| {
            defaults.oct.map(mei_oct_to_marks).unwrap_or(0)
        });

    let alter = note
        .note_ges
        .accid_ges
        .as_ref()
        .and_then(|ag| match ag {
            tusk_model::generated::data::DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
                basic,
            ) => Some(accid_ges_to_alter(basic)),
            _ => None,
        })
        .unwrap_or(0.0);

    let mut force_accidental = false;
    let mut cautionary = false;
    for child in &note.children {
        if let tusk_model::elements::NoteChild::Accid(accid) = child {
            force_accidental = true;
            if accid.accid_log.func.as_deref() == Some("cautionary") {
                cautionary = true;
                force_accidental = false;
            }
        }
    }

    Pitch {
        step,
        alter,
        octave,
        force_accidental,
        cautionary,
        octave_check: None,
    }
}

/// Convert an MEI Chord to a LilyPond ChordEvent or ChordRepetitionEvent.
///
/// If the chord has a chord_repetition in ext_store, it originated from a `q`
/// (chord repetition) and is emitted as `Music::ChordRepetition` for lossless
/// roundtrip.
pub(super) fn convert_mei_chord(chord: &tusk_model::elements::Chord, ext_store: &ExtensionStore, defaults: &MeiDefaults) -> Music {
    let duration = extract_chord_duration(chord, defaults);

    // Chord tie: if any child note has @tie="i" or "m", the chord has a tie
    let mut post_events = Vec::new();
    let has_tie = chord.children.iter().any(|child| {
        let ChordChild::Note(note) = child;
        matches!(note.note_anl.tie.as_ref(), Some(t) if t.0 == "i" || t.0 == "m")
    });
    if has_tie {
        post_events.push(PostEvent::Tie);
    }

    // Restore tweak post-events from ext_store
    restore_tweak_post_events_from_ext(chord.common.xml_id.as_deref(), &mut post_events, ext_store);

    // Check for chord repetition in ext_store
    if chord.common.xml_id.as_deref().is_some_and(|id| ext_store.chord_repetition(id).is_some()) {
        return Music::ChordRepetition(ChordRepetitionEvent {
            duration,
            post_events,
        });
    }

    let pitches: Vec<Pitch> = chord
        .children
        .iter()
        .map(|child| {
            let ChordChild::Note(note) = child;
            extract_pitch_from_note(note, defaults)
        })
        .collect();

    Music::Chord(ChordEvent {
        pitches,
        duration,
        post_events,
    })
}

/// Extract duration from an MEI chord, falling back to dur.default.
fn extract_chord_duration(chord: &tusk_model::elements::Chord, defaults: &MeiDefaults) -> Option<Duration> {
    if let Some(ref dur) = chord.chord_log.dur {
        let base = match dur {
            tusk_model::generated::data::DataDuration::MeiDataDurationCmn(cmn) => mei_dur_to_base(cmn),
            _ => return None,
        };
        let dots = chord
            .chord_log
            .dots
            .as_ref()
            .map(|d| d.0 as u8)
            .unwrap_or(0);
        Some(Duration {
            base,
            dots,
            multipliers: Vec::new(),
        })
    } else {
        defaults.dur.clone()
    }
}

/// Convert an MEI Note to a LilyPond NoteEvent (or DrumNote/DrumChord if labeled).
pub(super) fn convert_mei_note(note: &tusk_model::elements::Note, ext_store: &ExtensionStore, defaults: &MeiDefaults) -> Music {
    // Check for drum event in ext_store first
    if let Some(drum_music) = try_convert_drum_ext(note, ext_store) {
        return drum_music;
    }

    let pitch = extract_pitch_from_note(note, defaults);
    let duration = extract_note_duration(note, defaults);
    let mut post_events = Vec::new();

    // @tie="i" or "m" -> PostEvent::Tie (start or continuation)
    if let Some(ref tie) = note.note_anl.tie
        && (tie.0 == "i" || tie.0 == "m")
    {
        post_events.push(PostEvent::Tie);
    }

    // Restore tweak post-events from ext_store
    restore_tweak_post_events_from_ext(note.common.xml_id.as_deref(), &mut post_events, ext_store);

    Music::Note(NoteEvent {
        pitch,
        duration,
        pitched_rest: false,
        post_events,
    })
}

/// Try to convert a note with a drum event in ext_store back to drum mode music.
fn try_convert_drum_ext(note: &tusk_model::elements::Note, ext_store: &ExtensionStore) -> Option<Music> {
    let id = note.common.xml_id.as_deref()?;
    let de = ext_store.drum_event(id)?;
    parse_drum_event_str(&de.serialized)
}

/// Parse a serialized drum event string back into Music::DrumNote or Music::DrumChord.
///
/// Re-parses through the LilyPond parser by wrapping in `\drummode { ... }`.
fn parse_drum_event_str(s: &str) -> Option<Music> {
    use crate::parser::Parser;
    let src = format!("\\drummode {{ {s} }}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::DrumMode { body }) = item {
            if let Music::Sequential(items) = body.as_ref() {
                for m in items {
                    match m {
                        Music::DrumNote(_) | Music::DrumChord(_) => return Some(m.clone()),
                        _ => {}
                    }
                }
            }
            match body.as_ref() {
                Music::DrumNote(_) | Music::DrumChord(_) => return Some(*body.clone()),
                _ => {}
            }
        }
    }
    None
}

/// Convert an MEI Rest to a LilyPond RestEvent or pitched rest.
pub(super) fn convert_mei_rest(rest: &tusk_model::elements::Rest, ext_store: &ExtensionStore, defaults: &MeiDefaults) -> Music {
    // Check for pitched rest in ext_store
    if let Some(id) = rest.common.xml_id.as_deref()
        && let Some(pr) = ext_store.pitched_rest(id)
        && let Some(note_event) = parse_pitched_rest_label(&pr.pitch, rest, defaults)
    {

        return Music::Note(note_event);
    }

    Music::Rest(RestEvent {
        duration: extract_rest_duration(rest, defaults),
        post_events: Vec::new(),
    })
}

/// Parse a pitched rest label back into a NoteEvent.
fn parse_pitched_rest_label(
    pitch_str: &str,
    rest: &tusk_model::elements::Rest,
    defaults: &MeiDefaults,
) -> Option<NoteEvent> {
    // Split into note name and octave marks
    let mut note_end = 0;
    for (i, c) in pitch_str.char_indices() {
        if c == '\'' || c == ',' {
            note_end = i;
            break;
        }
        note_end = i + c.len_utf8();
    }
    let note_name = &pitch_str[..note_end];
    let octave_str = &pitch_str[note_end..];

    let (step, alter) = Pitch::from_note_name(note_name)?;
    let octave = octave_str
        .chars()
        .map(|c| if c == '\'' { 1i8 } else { -1i8 })
        .sum();

    Some(NoteEvent {
        pitch: Pitch {
            step,
            alter,
            octave,
            force_accidental: false,
            cautionary: false,
            octave_check: None,
        },
        duration: extract_rest_duration(rest, defaults),
        pitched_rest: true,
        post_events: vec![],
    })
}

/// Convert an MEI MRest to a LilyPond MultiMeasureRestEvent.
pub(super) fn convert_mei_mrest(mrest: &tusk_model::elements::MRest, ext_store: &ExtensionStore) -> Music {
    // Restore duration from ext_store
    let duration = mrest.common.xml_id.as_deref().and_then(|id| {
        let info = ext_store.mrest_info(id)?;
        Some(Duration {
            base: info.base,
            dots: info.dots,
            multipliers: info.multipliers.clone(),
        })
    });

    Music::MultiMeasureRest(MultiMeasureRestEvent {
        duration,
        post_events: Vec::new(),
    })
}

/// Extract duration from an MEI space.
fn extract_space_duration(space: &tusk_model::elements::Space) -> Option<Duration> {
    let dur = space.space_log.dur.as_ref()?;
    let base = match dur {
        tusk_model::generated::data::DataDuration::MeiDataDurationCmn(cmn) => mei_dur_to_base(cmn),
        _ => return None,
    };
    let dots = space
        .space_log
        .dots
        .as_ref()
        .map(|d| d.0 as u8)
        .unwrap_or(0);
    Some(Duration {
        base,
        dots,
        multipliers: Vec::new(),
    })
}

/// Convert an MEI Space to a LilyPond SkipEvent.
pub(super) fn convert_mei_space(space: &tusk_model::elements::Space) -> Music {
    Music::Skip(SkipEvent {
        duration: extract_space_duration(space),
        post_events: Vec::new(),
    })
}


/// Restore tweak post-events from ext_store.
fn restore_tweak_post_events_from_ext(xml_id: Option<&str>, post_events: &mut Vec<PostEvent>, ext_store: &ExtensionStore) {
    let id = match xml_id {
        Some(id) => id,
        None => return,
    };
    if let Some(tweaks) = ext_store.tweak_infos(id) {
        for tweak_info in tweaks {
            if let Some(tweak) = parse_tweak_str(&tweak_info.path) {
                post_events.push(tweak);
            }
        }
    }
}

/// Parse a serialized tweak string (e.g. `\tweak color #red`) back into a PostEvent::Tweak.
fn parse_tweak_str(s: &str) -> Option<PostEvent> {
    use crate::parser::Parser;
    // Wrap in a note context: `{ c4 <tweak> -. }` so the parser sees it as a post-event
    let src = format!("{{ c4{s} -. }}");
    let file = Parser::new(&src).ok()?.parse().ok()?;
    for item in &file.items {
        if let crate::model::ToplevelExpression::Music(Music::Sequential(items)) = item {
            for m in items {
                if let Music::Note(n) = m {
                    for pe in &n.post_events {
                        if matches!(pe, PostEvent::Tweak { .. }) {
                            return Some(pe.clone());
                        }
                    }
                }
            }
        }
    }
    None
}

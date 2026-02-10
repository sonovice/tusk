//! MEI -> LilyPond pitch/duration/event conversion for export.

use tusk_model::elements::ChordChild;
use tusk_model::generated::data::{DataAccidentalGesturalBasic, DataDurationCmn};

use crate::model::note::{ChordEvent, ChordRepetitionEvent, PostEvent};
use crate::model::pitch::Pitch;
use crate::model::{Duration, MultiMeasureRestEvent, Music, NoteEvent, RestEvent};

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

/// Extract duration from an MEI note.
fn extract_note_duration(note: &tusk_model::elements::Note) -> Option<Duration> {
    let dur = note.note_log.dur.as_ref()?;
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
}

/// Extract duration from an MEI rest.
fn extract_rest_duration(rest: &tusk_model::elements::Rest) -> Option<Duration> {
    let dur = rest.rest_log.dur.as_ref()?;
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
}

// ---------------------------------------------------------------------------
// Event conversion
// ---------------------------------------------------------------------------

/// Extract a LilyPond Pitch from an MEI Note (for use inside chords -- no duration).
fn extract_pitch_from_note(note: &tusk_model::elements::Note) -> Pitch {
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
        .unwrap_or(0);

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
/// If the chord has a `lilypond:chord-rep` label, it originated from a `q`
/// (chord repetition) and is emitted as `Music::ChordRepetition` for lossless
/// roundtrip.
pub(super) fn convert_mei_chord(chord: &tusk_model::elements::Chord) -> Music {
    let duration = extract_chord_duration(chord);

    // Chord tie: if any child note has @tie="i" or "m", the chord has a tie
    let mut post_events = Vec::new();
    let has_tie = chord.children.iter().any(|child| {
        let ChordChild::Note(note) = child;
        matches!(note.note_anl.tie.as_ref(), Some(t) if t.0 == "i" || t.0 == "m")
    });
    if has_tie {
        post_events.push(PostEvent::Tie);
    }

    // Check for chord repetition label
    if chord
        .common
        .label
        .as_deref()
        .is_some_and(|l| l == "lilypond:chord-rep")
    {
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
            extract_pitch_from_note(note)
        })
        .collect();

    Music::Chord(ChordEvent {
        pitches,
        duration,
        post_events,
    })
}

/// Extract duration from an MEI chord.
fn extract_chord_duration(chord: &tusk_model::elements::Chord) -> Option<Duration> {
    let dur = chord.chord_log.dur.as_ref()?;
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
}

/// Convert an MEI Note to a LilyPond NoteEvent.
pub(super) fn convert_mei_note(note: &tusk_model::elements::Note) -> Music {
    let pitch = extract_pitch_from_note(note);
    let duration = extract_note_duration(note);
    let mut post_events = Vec::new();

    // @tie="i" or "m" → PostEvent::Tie (start or continuation)
    if let Some(ref tie) = note.note_anl.tie
        && (tie.0 == "i" || tie.0 == "m")
    {
        post_events.push(PostEvent::Tie);
    }

    Music::Note(NoteEvent {
        pitch,
        duration,
        pitched_rest: false,
        post_events,
    })
}

/// Convert an MEI Rest to a LilyPond RestEvent or pitched rest.
pub(super) fn convert_mei_rest(rest: &tusk_model::elements::Rest) -> Music {
    // Check for pitched rest label
    if let Some(label) = &rest.common.label
        && let Some(pitch_str) = label.strip_prefix("lilypond:pitched-rest,")
        && let Some(note_event) = parse_pitched_rest_label(pitch_str, rest)
    {
        return Music::Note(note_event);
    }

    Music::Rest(RestEvent {
        duration: extract_rest_duration(rest),
        post_events: vec![],
    })
}

/// Parse a pitched rest label back into a NoteEvent.
fn parse_pitched_rest_label(
    pitch_str: &str,
    rest: &tusk_model::elements::Rest,
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
        duration: extract_rest_duration(rest),
        pitched_rest: true,
        post_events: vec![],
    })
}

/// Convert an MEI MRest to a LilyPond MultiMeasureRestEvent.
pub(super) fn convert_mei_mrest(mrest: &tusk_model::elements::MRest) -> Music {
    // Restore duration from label
    let duration = mrest
        .common
        .label
        .as_ref()
        .and_then(|l| l.strip_prefix("lilypond:mrest,"))
        .and_then(parse_mrest_label);

    Music::MultiMeasureRest(MultiMeasureRestEvent {
        duration,
        post_events: vec![],
    })
}

/// Parse mrest label back to Duration.
fn parse_mrest_label(label: &str) -> Option<Duration> {
    let mut base = None;
    let mut dots = 0u8;
    let mut multipliers = Vec::new();

    for part in label.split(',') {
        if let Some(val) = part.strip_prefix("dur=") {
            base = val.parse().ok();
        } else if let Some(val) = part.strip_prefix("dots=") {
            dots = val.parse().unwrap_or(0);
        } else if let Some(val) = part.strip_prefix("mul=") {
            if let Some((n, d)) = val.split_once('/') {
                if let (Ok(n), Ok(d)) = (n.parse(), d.parse()) {
                    multipliers.push((n, d));
                }
            } else if let Ok(n) = val.parse() {
                multipliers.push((n, 1));
            }
        }
    }

    Some(Duration {
        base: base?,
        dots,
        multipliers,
    })
}

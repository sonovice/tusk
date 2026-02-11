//! Pitch / duration / accidental and event conversion for LilyPond import.

use tusk_model::elements::{Accid, Chord, ChordChild, MRest, Note, NoteChild, Rest};
use tusk_model::generated::data::{
    DataAccidentalGestural, DataAccidentalGesturalBasic, DataAccidentalWritten,
    DataAccidentalWrittenBasic, DataAugmentdot, DataDuration, DataDurationCmn, DataDurationrests,
    DataOctave, DataPitchname,
};

use crate::model::{self, Duration, NoteEvent, RestEvent};

/// Convert LilyPond step char to MEI pitch name string.
fn step_to_pname(step: char) -> DataPitchname {
    DataPitchname(step.to_string())
}

/// Convert LilyPond octave marks to MEI absolute octave.
///
/// LilyPond absolute octave convention: c (no marks) = octave 3,
/// c' = 4, c'' = 5, c, = 2, c,, = 1.
fn octave_to_mei(octave_marks: i8) -> DataOctave {
    DataOctave((3 + octave_marks as i64).max(0) as u64)
}

/// Convert LilyPond alter (half-steps) to MEI gestural accidental.
fn alter_to_accid_ges(alter: f32) -> Option<DataAccidentalGestural> {
    let key = (alter * 2.0) as i32;
    match key {
        0 => None,
        2 => Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::S,
        )),
        4 => Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::Ss,
        )),
        -2 => Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::F,
        )),
        -4 => Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::Ff,
        )),
        _ => None,
    }
}

/// Convert LilyPond alter (half-steps) to MEI written accidental.
fn alter_to_accid_written(alter: f32) -> Option<DataAccidentalWritten> {
    let key = (alter * 2.0) as i32;
    match key {
        0 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::N,
        )),
        2 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::S,
        )),
        4 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::Ss,
        )),
        -2 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::F,
        )),
        -4 => Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::Ff,
        )),
        _ => None,
    }
}

/// Convert LilyPond duration base to MEI DataDurationCmn.
fn duration_base_to_mei(base: u32) -> Option<DataDurationCmn> {
    match base {
        1 => Some(DataDurationCmn::N1),
        2 => Some(DataDurationCmn::N2),
        4 => Some(DataDurationCmn::N4),
        8 => Some(DataDurationCmn::N8),
        16 => Some(DataDurationCmn::N16),
        32 => Some(DataDurationCmn::N32),
        64 => Some(DataDurationCmn::N64),
        128 => Some(DataDurationCmn::N128),
        _ => None,
    }
}

/// Apply duration to an MEI note's @dur and @dots.
fn apply_duration_to_note(dur: &Duration, note: &mut Note) {
    if let Some(cmn) = duration_base_to_mei(dur.base) {
        note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(cmn));
    }
    if dur.dots > 0 {
        note.note_log.dots = Some(DataAugmentdot(dur.dots as u64));
    }
}

/// Apply duration to an MEI rest's @dur and @dots.
fn apply_duration_to_rest(dur: &Duration, rest: &mut Rest) {
    if let Some(cmn) = duration_base_to_mei(dur.base) {
        rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(cmn));
    }
    if dur.dots > 0 {
        rest.rest_log.dots = Some(DataAugmentdot(dur.dots as u64));
    }
}

// ---------------------------------------------------------------------------
// Event conversion
// ---------------------------------------------------------------------------

/// Convert a LilyPond pitch to an MEI Note (for use inside a chord -- no duration).
fn convert_pitch_to_note(pitch: &crate::model::Pitch, id: u32) -> Note {
    let mut mei_note = Note::default();
    mei_note.common.xml_id = Some(format!("ly-note-{id}"));

    mei_note.note_log.pname = Some(step_to_pname(pitch.step));
    mei_note.note_log.oct = Some(octave_to_mei(pitch.octave));

    if let Some(accid_ges) = alter_to_accid_ges(pitch.alter) {
        mei_note.note_ges.accid_ges = Some(accid_ges);
    }

    if (pitch.force_accidental || pitch.cautionary)
        && let Some(accid_written) = alter_to_accid_written(pitch.alter)
    {
        let mut accid = Accid::default();
        accid.accid_log.accid = Some(accid_written);
        if pitch.cautionary {
            accid.accid_log.func = Some("cautionary".to_string());
        }
        mei_note.children.push(NoteChild::Accid(Box::new(accid)));
    }

    mei_note
}

/// Convert a LilyPond chord (multiple pitches + shared duration) to an MEI Chord.
pub(super) fn convert_chord(
    pitches: &[crate::model::Pitch],
    duration: Option<&Duration>,
    id_counter: &mut u32,
) -> Chord {
    let chord_id = *id_counter;
    let mut mei_chord = Chord::default();
    mei_chord.common.xml_id = Some(format!("ly-chord-{chord_id}"));

    // Duration on the chord element
    if let Some(dur) = duration {
        if let Some(cmn) = duration_base_to_mei(dur.base) {
            mei_chord.chord_log.dur = Some(DataDuration::MeiDataDurationCmn(cmn));
        }
        if dur.dots > 0 {
            mei_chord.chord_log.dots = Some(DataAugmentdot(dur.dots as u64));
        }
    }

    // Child notes (one per pitch, no individual duration)
    for pitch in pitches {
        *id_counter += 1;
        let mei_note = convert_pitch_to_note(pitch, *id_counter);
        mei_chord
            .children
            .push(ChordChild::Note(Box::new(mei_note)));
    }

    mei_chord
}

/// Convert a LilyPond NoteEvent to an MEI Note.
pub(super) fn convert_note(note: &NoteEvent, id: u32) -> Note {
    let mut mei_note = Note::default();
    mei_note.common.xml_id = Some(format!("ly-note-{id}"));

    // Pitch
    mei_note.note_log.pname = Some(step_to_pname(note.pitch.step));
    mei_note.note_log.oct = Some(octave_to_mei(note.pitch.octave));

    // Gestural accidental
    if let Some(accid_ges) = alter_to_accid_ges(note.pitch.alter) {
        mei_note.note_ges.accid_ges = Some(accid_ges);
    }

    // Written accidental (force or cautionary)
    if (note.pitch.force_accidental || note.pitch.cautionary)
        && let Some(accid_written) = alter_to_accid_written(note.pitch.alter)
    {
        let mut accid = Accid::default();
        accid.accid_log.accid = Some(accid_written);
        if note.pitch.cautionary {
            accid.accid_log.func = Some("cautionary".to_string());
        }
        mei_note.children.push(NoteChild::Accid(Box::new(accid)));
    }

    // Duration
    if let Some(ref dur) = note.duration {
        apply_duration_to_note(dur, &mut mei_note);
    }

    mei_note
}

/// Convert a LilyPond RestEvent to an MEI Rest.
pub(super) fn convert_rest(rest: &RestEvent, id: u32) -> Rest {
    let mut mei_rest = Rest::default();
    mei_rest.common.xml_id = Some(format!("ly-rest-{id}"));

    if let Some(ref dur) = rest.duration {
        apply_duration_to_rest(dur, &mut mei_rest);
    }

    mei_rest
}

/// Convert a pitched rest (note with \rest) to an MEI Rest with label.
pub(super) fn convert_pitched_rest(note: &NoteEvent, id: u32) -> Rest {
    let mut mei_rest = Rest::default();
    mei_rest.common.xml_id = Some(format!("ly-rest-{id}"));

    // Store pitch position as typed JSON label for roundtrip
    let pr = tusk_model::PitchedRest {
        pitch: format!("{}{}", note.pitch.to_note_name(), note.pitch.octave_marks()),
    };
    let json = serde_json::to_string(&pr).unwrap_or_default();
    mei_rest.common.label = Some(format!("tusk:pitched-rest,{json}"));

    if let Some(ref dur) = note.duration {
        apply_duration_to_rest(dur, &mut mei_rest);
    }

    mei_rest
}

/// Convert a LilyPond DrumNoteEvent to an MEI Note.
///
/// Drum notes are stored as unpitched MEI notes with `@label` carrying
/// typed JSON for lossless roundtrip. Duration is applied normally.
pub(super) fn convert_drum_note(dn: &crate::model::note::DrumNoteEvent, id: u32) -> Note {
    let mut mei_note = Note::default();
    mei_note.common.xml_id = Some(format!("ly-note-{id}"));

    // Serialize the drum event as typed JSON label
    let serialized = crate::serializer::serialize_drum_note_event(dn);
    let de = tusk_model::DrumEvent { serialized };
    let json = super::utils::escape_json_pipe(&serde_json::to_string(&de).unwrap_or_default());
    mei_note.common.label = Some(format!("tusk:drum,{json}"));

    // Duration
    if let Some(ref dur) = dn.duration {
        apply_duration_to_note(dur, &mut mei_note);
    }

    mei_note
}

/// Convert a LilyPond DrumChordEvent to an MEI Note.
///
/// Drum chords (simultaneous drum hits) are stored as a single MEI note with
/// `@label` carrying typed JSON for lossless roundtrip.
pub(super) fn convert_drum_chord(dc: &crate::model::note::DrumChordEvent, id: u32) -> Note {
    let mut mei_note = Note::default();
    mei_note.common.xml_id = Some(format!("ly-note-{id}"));

    // Serialize the drum chord event as typed JSON label
    let serialized = crate::serializer::serialize_drum_chord_event(dc);
    let de = tusk_model::DrumEvent { serialized };
    let json = super::utils::escape_json_pipe(&serde_json::to_string(&de).unwrap_or_default());
    mei_note.common.label = Some(format!("tusk:drum,{json}"));

    // Duration
    if let Some(ref dur) = dc.duration {
        apply_duration_to_note(dur, &mut mei_note);
    }

    mei_note
}

/// Convert a LilyPond MultiMeasureRestEvent to an MEI MRest.
pub(super) fn convert_mrest(rest: &model::MultiMeasureRestEvent, id: u32) -> MRest {
    let mut mei_mrest = MRest::default();
    mei_mrest.common.xml_id = Some(format!("ly-mrest-{id}"));

    // Store full duration info as typed JSON label for lossless roundtrip
    if let Some(ref dur) = rest.duration {
        let info = tusk_model::MultiMeasureRestInfo {
            base: dur.base,
            dots: dur.dots,
            multipliers: dur.multipliers.clone(),
        };
        let json = serde_json::to_string(&info).unwrap_or_default();
        mei_mrest.common.label = Some(format!("tusk:mrest,{json}"));
    }

    mei_mrest
}

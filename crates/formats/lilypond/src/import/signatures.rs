//! Clef / key / time signature conversion for LilyPond import.

use tusk_model::elements::StaffDef;
use tusk_model::generated::data::{
    DataClefline, DataClefshape, DataKeyfifths, DataOctaveDis, DataStaffrelBasic,
};

use crate::model;

use super::LyEvent;

/// Apply clef/key/time from the event stream onto a staffDef and return
/// a label segment encoding the full event sequence for roundtrip.
///
/// The label format is `lilypond:events,TYPE@POS;TYPE@POS;...` where:
/// - TYPE is `clef:NAME`, `key:STEP.ALTER.MODE`, or `time:N+M/D`
/// - POS is the 0-based index in the note/rest event stream
pub(super) fn apply_signatures_to_staff_def(
    events: &[LyEvent],
    staff_def: &mut StaffDef,
) -> String {
    let mut first_clef = true;
    let mut first_key = true;
    let mut first_time = true;
    let mut note_index = 0u32;
    let mut entries = Vec::new();

    for event in events {
        match event {
            LyEvent::Clef(c) => {
                entries.push(format!("clef:{}@{note_index}", c.name));
                if first_clef {
                    apply_clef_to_staff_def(c, staff_def);
                    first_clef = false;
                }
            }
            LyEvent::KeySig(ks) => {
                let fifths = key_to_fifths(&ks.pitch, &ks.mode);
                entries.push(format!(
                    "key:{}.{}.{}@{note_index}",
                    ks.pitch.step,
                    ks.pitch.alter,
                    ks.mode.as_str()
                ));
                if first_key {
                    staff_def.staff_def_log.keysig = Some(DataKeyfifths(fifths.to_string()));
                    first_key = false;
                }
            }
            LyEvent::TimeSig(ts) => {
                let count: String = ts
                    .numerators
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join("+");
                entries.push(format!("time:{}/{}@{note_index}", count, ts.denominator));
                if first_time {
                    staff_def.staff_def_log.meter_count = Some(count);
                    staff_def.staff_def_log.meter_unit = Some(ts.denominator.to_string());
                    first_time = false;
                }
            }
            LyEvent::AutoBeamOn => {
                entries.push(format!("autobeamon@{note_index}"));
            }
            LyEvent::AutoBeamOff => {
                entries.push(format!("autobeamoff@{note_index}"));
            }
            LyEvent::Note(_)
            | LyEvent::Chord { .. }
            | LyEvent::Rest(_)
            | LyEvent::PitchedRest(_)
            | LyEvent::MeasureRest(_) => {
                note_index += 1;
            }
            LyEvent::Skip(_) => {}
        }
    }

    if entries.is_empty() {
        String::new()
    } else {
        format!("lilypond:events,{}", entries.join(";"))
    }
}

/// Apply a LilyPond clef to MEI staffDef attributes.
fn apply_clef_to_staff_def(clef: &model::Clef, staff_def: &mut StaffDef) {
    let (shape, line, dis, dis_place) = clef_name_to_mei(&clef.name);
    staff_def.staff_def_log.clef_shape = Some(shape);
    staff_def.staff_def_log.clef_line = Some(DataClefline(line));
    if let Some(d) = dis {
        staff_def.staff_def_log.clef_dis = Some(DataOctaveDis(d));
    }
    if let Some(dp) = dis_place {
        staff_def.staff_def_log.clef_dis_place = Some(dp);
    }
}

/// Map LilyPond clef name to MEI clef attributes (shape, line, dis, dis.place).
fn clef_name_to_mei(name: &str) -> (DataClefshape, u64, Option<u64>, Option<DataStaffrelBasic>) {
    // Split off transposition suffix (_8, ^15, _15, ^8)
    let (base, dis, dis_place) = parse_clef_transposition(name);

    let (shape, line) = match base {
        "treble" | "violin" | "G" | "G2" => (DataClefshape::G, 2),
        "french" => (DataClefshape::G, 1),
        "GG" => (DataClefshape::Gg, 2),
        "tenorG" => (DataClefshape::G, 2), // tenor G clef (octave transposed)
        "soprano" => (DataClefshape::C, 1),
        "mezzosoprano" => (DataClefshape::C, 2),
        "alto" | "C" => (DataClefshape::C, 3),
        "tenor" => (DataClefshape::C, 4),
        "baritone" => (DataClefshape::C, 5),
        "varbaritone" => (DataClefshape::F, 3),
        "bass" | "F" => (DataClefshape::F, 4),
        "subbass" => (DataClefshape::F, 5),
        "percussion" | "varpercussion" => (DataClefshape::Perc, 3),
        "tab" => (DataClefshape::Tab, 5),
        // Variant C clefs
        "varC" | "altovarC" => (DataClefshape::C, 3),
        "tenorvarC" => (DataClefshape::C, 4),
        "baritonevarC" => (DataClefshape::C, 5),
        _ => (DataClefshape::G, 2), // fallback to treble
    };

    // tenorG has implicit 8vb transposition
    let (dis, dis_place) = if base == "tenorG" && dis.is_none() {
        (Some(8), Some(DataStaffrelBasic::Below))
    } else {
        (dis, dis_place)
    };

    (shape, line, dis, dis_place)
}

/// Parse clef transposition suffix: `_8`, `^8`, `_15`, `^15`.
fn parse_clef_transposition(name: &str) -> (&str, Option<u64>, Option<DataStaffrelBasic>) {
    for (suffix, dis, place) in [
        ("_8", 8u64, DataStaffrelBasic::Below),
        ("^8", 8, DataStaffrelBasic::Above),
        ("_15", 15, DataStaffrelBasic::Below),
        ("^15", 15, DataStaffrelBasic::Above),
    ] {
        if let Some(base) = name.strip_suffix(suffix) {
            return (base, Some(dis), Some(place));
        }
    }
    (name, None, None)
}

/// Convert a LilyPond key (pitch + mode) to circle-of-fifths count.
///
/// Positive = sharps, negative = flats.
pub(super) fn key_to_fifths(pitch: &crate::model::pitch::Pitch, mode: &crate::model::Mode) -> i32 {
    // Major keys: C=0, G=1, D=2, A=3, E=4, B=5, F#=6, Cb=-7, Gb=-6, Db=-5, Ab=-4, Eb=-3, Bb=-2, F=-1
    // The fifths value for a major key is based on the pitch's position on the circle of fifths.
    let base_fifths = pitch_to_major_fifths(pitch.step, pitch.alter);

    // Mode offsets relative to major: minor = -3, dorian = -2, phrygian = -4, etc.
    let mode_offset = match mode {
        crate::model::Mode::Major | crate::model::Mode::Ionian => 0,
        crate::model::Mode::Minor | crate::model::Mode::Aeolian => -3,
        crate::model::Mode::Dorian => -2,
        crate::model::Mode::Phrygian => -4,
        crate::model::Mode::Lydian => 1,
        crate::model::Mode::Mixolydian => -1,
        crate::model::Mode::Locrian => -5,
    };

    base_fifths + mode_offset
}

/// Convert a pitch (step + alter) to its major-key position on the circle of fifths.
fn pitch_to_major_fifths(step: char, alter: f32) -> i32 {
    // Natural note positions on circle of fifths (for major keys):
    // F=-1, C=0, G=1, D=2, A=3, E=4, B=5
    let natural_fifths = match step {
        'c' => 0,
        'd' => 2,
        'e' => 4,
        'f' => -1,
        'g' => 1,
        'a' => 3,
        'b' => 5,
        _ => 0,
    };
    // Each sharp adds 7 fifths, each flat subtracts 7
    let alter_offset = (alter * 2.0) as i32; // half-steps -> alter units
    // Sharp = +1.0 alter = +7 fifths, flat = -1.0 alter = -7 fifths
    natural_fifths + alter_offset * 7 / 2
}

/// Convert MEI clef attributes back to LilyPond clef name.
pub fn mei_clef_to_name(
    shape: &DataClefshape,
    line: u64,
    dis: Option<u64>,
    dis_place: Option<&DataStaffrelBasic>,
) -> String {
    let base = match (shape, line) {
        (DataClefshape::G, 2) => "treble",
        (DataClefshape::G, 1) => "french",
        (DataClefshape::Gg, 2) => "GG",
        (DataClefshape::C, 1) => "soprano",
        (DataClefshape::C, 2) => "mezzosoprano",
        (DataClefshape::C, 3) => "alto",
        (DataClefshape::C, 4) => "tenor",
        (DataClefshape::C, 5) => "baritone",
        (DataClefshape::F, 3) => "varbaritone",
        (DataClefshape::F, 4) => "bass",
        (DataClefshape::F, 5) => "subbass",
        (DataClefshape::Perc, _) => "percussion",
        (DataClefshape::Tab, _) => "tab",
        _ => "treble",
    };

    // Check for tenorG special case
    if base == "treble" && dis == Some(8) && dis_place == Some(&DataStaffrelBasic::Below) {
        return "tenorG".to_string();
    }

    // Append transposition suffix
    match (dis, dis_place) {
        (Some(8), Some(DataStaffrelBasic::Below)) => format!("{base}_8"),
        (Some(8), Some(DataStaffrelBasic::Above)) => format!("{base}^8"),
        (Some(15), Some(DataStaffrelBasic::Below)) => format!("{base}_15"),
        (Some(15), Some(DataStaffrelBasic::Above)) => format!("{base}^15"),
        _ => base.to_string(),
    }
}

/// Convert MEI key fifths value back to LilyPond pitch + mode.
pub fn fifths_to_key(fifths: i32) -> (crate::model::pitch::Pitch, crate::model::Mode) {
    // For simplicity, always export as major key.
    // The event sequence label preserves the original mode.
    let (step, alter) = major_fifths_to_pitch(fifths);
    let pitch = crate::model::pitch::Pitch {
        step,
        alter,
        octave: 0,
        force_accidental: false,
        cautionary: false,
        octave_check: None,
    };
    (pitch, crate::model::Mode::Major)
}

/// Convert circle-of-fifths position to a major key tonic.
fn major_fifths_to_pitch(fifths: i32) -> (char, f32) {
    match fifths {
        0 => ('c', 0.0),
        1 => ('g', 0.0),
        2 => ('d', 0.0),
        3 => ('a', 0.0),
        4 => ('e', 0.0),
        5 => ('b', 0.0),
        6 => ('f', 1.0), // F#
        7 => ('c', 1.0), // C#
        -1 => ('f', 0.0),
        -2 => ('b', -1.0), // Bb
        -3 => ('e', -1.0), // Eb
        -4 => ('a', -1.0), // Ab
        -5 => ('d', -1.0), // Db
        -6 => ('g', -1.0), // Gb
        -7 => ('c', -1.0), // Cb
        _ => ('c', 0.0),
    }
}

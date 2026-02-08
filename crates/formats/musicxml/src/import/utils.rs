//! Duration, pitch, and ID helper utilities for MusicXML to MEI conversion.
//!
//! This module contains low-level conversion helpers for:
//! - Pitch name conversion
//! - Accidental conversion (gestural and written)
//! - Duration/note type conversion
//! - Stem direction conversion
//! - Grace note conversion
//! - Beat unit conversion

use crate::model::note::{AccidentalValue, NoteTypeValue, StemValue};
use tusk_model::data::{
    DataAccidentalGestural, DataAccidentalGesturalBasic, DataAccidentalGesturalExtended,
    DataAccidentalWritten, DataAccidentalWrittenBasic, DataAccidentalWrittenExtended, DataDuration,
    DataDurationCmn, DataGrace, DataPitchname, DataStemdirection, DataStemdirectionBasic,
};

// ============================================================================
// Pitch Conversion Helpers
// ============================================================================

/// Convert MusicXML Step to MEI DataPitchname.
pub(crate) fn convert_pitch_name(step: crate::model::data::Step) -> DataPitchname {
    use crate::model::data::Step;

    let name = match step {
        Step::A => "a",
        Step::B => "b",
        Step::C => "c",
        Step::D => "d",
        Step::E => "e",
        Step::F => "f",
        Step::G => "g",
    };
    DataPitchname::from(name.to_string())
}

/// Convert MusicXML alter value to MEI gestural accidental.
///
/// Handles both standard semitone alterations and quarter-tone microtonal values.
/// MusicXML alter is in semitones (e.g., -0.5 = quarter-tone flat, 1.5 = three quarter-tones sharp).
pub(crate) fn convert_alter_to_gestural_accid(alter: f64) -> DataAccidentalGestural {
    use tusk_model::data::DataAccidentalGestural::{
        MeiDataAccidentalGesturalBasic as Basic, MeiDataAccidentalGesturalExtended as Ext,
    };

    // Use epsilon comparison for floating point
    let eps = 1e-9;
    if (alter - (-3.0)).abs() < eps {
        Basic(DataAccidentalGesturalBasic::Tf)
    } else if (alter - (-2.5)).abs() < eps {
        Ext(DataAccidentalGesturalExtended::Ffd) // Five quarter-tones flat
    } else if (alter - (-2.0)).abs() < eps {
        Basic(DataAccidentalGesturalBasic::Ff)
    } else if (alter - (-1.5)).abs() < eps {
        Ext(DataAccidentalGesturalExtended::Fd) // Three quarter-tones flat
    } else if (alter - (-1.0)).abs() < eps {
        Basic(DataAccidentalGesturalBasic::F)
    } else if (alter - (-0.5)).abs() < eps {
        Ext(DataAccidentalGesturalExtended::Fu) // Quarter-tone flat
    } else if alter.abs() < eps {
        Basic(DataAccidentalGesturalBasic::N)
    } else if (alter - 0.5).abs() < eps {
        Ext(DataAccidentalGesturalExtended::Sd) // Quarter-tone sharp
    } else if (alter - 1.0).abs() < eps {
        Basic(DataAccidentalGesturalBasic::S)
    } else if (alter - 1.5).abs() < eps {
        Ext(DataAccidentalGesturalExtended::Su) // Three quarter-tones sharp
    } else if (alter - 2.0).abs() < eps {
        Basic(DataAccidentalGesturalBasic::Ss)
    } else if (alter - 2.5).abs() < eps {
        Ext(DataAccidentalGesturalExtended::Xu) // Five quarter-tones sharp
    } else if (alter - 3.0).abs() < eps {
        Basic(DataAccidentalGesturalBasic::Ts)
    } else {
        // For unsupported microtone values, use natural as fallback
        Basic(DataAccidentalGesturalBasic::N)
    }
}

/// Convert MusicXML AccidentalValue to MEI DataAccidentalWritten.
pub(crate) fn convert_accidental_value(value: AccidentalValue) -> DataAccidentalWritten {
    use tusk_model::data::DataAccidentalWritten::{
        MeiDataAccidentalWrittenBasic as Basic, MeiDataAccidentalWrittenExtended as Ext,
    };

    match value {
        AccidentalValue::Sharp => Basic(DataAccidentalWrittenBasic::S),
        AccidentalValue::Natural => Basic(DataAccidentalWrittenBasic::N),
        AccidentalValue::Flat => Basic(DataAccidentalWrittenBasic::F),
        AccidentalValue::DoubleSharp | AccidentalValue::SharpSharp => {
            Basic(DataAccidentalWrittenBasic::X)
        }
        AccidentalValue::FlatFlat => Basic(DataAccidentalWrittenBasic::Ff),
        AccidentalValue::NaturalSharp => Basic(DataAccidentalWrittenBasic::Ns),
        AccidentalValue::NaturalFlat => Basic(DataAccidentalWrittenBasic::Nf),
        AccidentalValue::TripleSharp => Basic(DataAccidentalWrittenBasic::Ts),
        AccidentalValue::TripleFlat => Basic(DataAccidentalWrittenBasic::Tf),
        // Quarter-tone accidentals → MEI extended written accidentals
        AccidentalValue::QuarterFlat => Ext(DataAccidentalWrittenExtended::Nd), // natural lowered by quarter-tone
        AccidentalValue::QuarterSharp => Ext(DataAccidentalWrittenExtended::Nu), // natural raised by quarter-tone
        AccidentalValue::ThreeQuartersFlat => Ext(DataAccidentalWrittenExtended::Fd), // flat lowered by quarter-tone
        AccidentalValue::ThreeQuartersSharp => Ext(DataAccidentalWrittenExtended::Su), // sharp raised by quarter-tone
        // Arrow variants → MEI extended written accidentals
        AccidentalValue::SharpDown => Ext(DataAccidentalWrittenExtended::Sd),
        AccidentalValue::SharpUp => Ext(DataAccidentalWrittenExtended::Su),
        AccidentalValue::NaturalDown => Ext(DataAccidentalWrittenExtended::Nd),
        AccidentalValue::NaturalUp => Ext(DataAccidentalWrittenExtended::Nu),
        AccidentalValue::FlatDown => Ext(DataAccidentalWrittenExtended::Fd),
        AccidentalValue::FlatUp => Ext(DataAccidentalWrittenExtended::Fu),
        AccidentalValue::DoubleSharpDown => Ext(DataAccidentalWrittenExtended::Xd),
        AccidentalValue::DoubleSharpUp => Ext(DataAccidentalWrittenExtended::Xu),
        AccidentalValue::FlatFlatDown => Ext(DataAccidentalWrittenExtended::Ffd),
        AccidentalValue::FlatFlatUp => Ext(DataAccidentalWrittenExtended::Ffu),
        AccidentalValue::ArrowDown => Ext(DataAccidentalWrittenExtended::Nd),
        AccidentalValue::ArrowUp => Ext(DataAccidentalWrittenExtended::Nu),
        // Slash variants — closest basic equivalents (no MEI extended match)
        AccidentalValue::SlashQuarterSharp | AccidentalValue::SlashSharp => {
            Basic(DataAccidentalWrittenBasic::S)
        }
        AccidentalValue::SlashFlat | AccidentalValue::DoubleSlashFlat => {
            Basic(DataAccidentalWrittenBasic::F)
        }
        // Numbered sharps/flats (Stein-Zimmermann notation) — closest basic equivalents
        AccidentalValue::Sharp1
        | AccidentalValue::Sharp2
        | AccidentalValue::Sharp3
        | AccidentalValue::Sharp5 => Basic(DataAccidentalWrittenBasic::S),
        AccidentalValue::Flat1
        | AccidentalValue::Flat2
        | AccidentalValue::Flat3
        | AccidentalValue::Flat4 => Basic(DataAccidentalWrittenBasic::F),
        // Persian accidentals
        AccidentalValue::Sori => Ext(DataAccidentalWrittenExtended::Nu), // quarter-tone sharp
        AccidentalValue::Koron => Ext(DataAccidentalWrittenExtended::Nd), // quarter-tone flat
        // Other
        AccidentalValue::Other => Basic(DataAccidentalWrittenBasic::N),
    }
}

// ============================================================================
// Duration Conversion Helpers
// ============================================================================

/// Convert MusicXML NoteTypeValue to MEI DataDuration.
pub(crate) fn convert_note_type_to_duration(note_type: NoteTypeValue) -> DataDuration {
    let dur = match note_type {
        NoteTypeValue::Maxima => DataDurationCmn::Long, // MEI doesn't have maxima, use long
        NoteTypeValue::Long => DataDurationCmn::Long,
        NoteTypeValue::Breve => DataDurationCmn::Breve,
        NoteTypeValue::Whole => DataDurationCmn::N1,
        NoteTypeValue::Half => DataDurationCmn::N2,
        NoteTypeValue::Quarter => DataDurationCmn::N4,
        NoteTypeValue::Eighth => DataDurationCmn::N8,
        NoteTypeValue::N16th => DataDurationCmn::N16,
        NoteTypeValue::N32nd => DataDurationCmn::N32,
        NoteTypeValue::N64th => DataDurationCmn::N64,
        NoteTypeValue::N128th => DataDurationCmn::N128,
        NoteTypeValue::N256th => DataDurationCmn::N256,
        NoteTypeValue::N512th => DataDurationCmn::N512,
        NoteTypeValue::N1024th => DataDurationCmn::N1024,
    };
    DataDuration::MeiDataDurationCmn(dur)
}

/// Convert MusicXML NoteTypeValue to MEI DataDurationCmn.
///
/// Similar to `convert_note_type_to_duration` but returns the CMN-specific type
/// for use with rests (which use `DataDurationrests` instead of `DataDuration`).
pub(crate) fn convert_note_type_to_duration_cmn(note_type: NoteTypeValue) -> DataDurationCmn {
    match note_type {
        NoteTypeValue::Maxima => DataDurationCmn::Long, // MEI doesn't have maxima, use long
        NoteTypeValue::Long => DataDurationCmn::Long,
        NoteTypeValue::Breve => DataDurationCmn::Breve,
        NoteTypeValue::Whole => DataDurationCmn::N1,
        NoteTypeValue::Half => DataDurationCmn::N2,
        NoteTypeValue::Quarter => DataDurationCmn::N4,
        NoteTypeValue::Eighth => DataDurationCmn::N8,
        NoteTypeValue::N16th => DataDurationCmn::N16,
        NoteTypeValue::N32nd => DataDurationCmn::N32,
        NoteTypeValue::N64th => DataDurationCmn::N64,
        NoteTypeValue::N128th => DataDurationCmn::N128,
        NoteTypeValue::N256th => DataDurationCmn::N256,
        NoteTypeValue::N512th => DataDurationCmn::N512,
        NoteTypeValue::N1024th => DataDurationCmn::N1024,
    }
}

/// Convert a beat unit string to MEI DataDuration.
pub(crate) fn beat_unit_string_to_duration(beat_unit: &str) -> Option<DataDuration> {
    let cmn = match beat_unit {
        "long" => DataDurationCmn::Long,
        "breve" => DataDurationCmn::Breve,
        "whole" => DataDurationCmn::N1,
        "half" => DataDurationCmn::N2,
        "quarter" => DataDurationCmn::N4,
        "eighth" => DataDurationCmn::N8,
        "16th" => DataDurationCmn::N16,
        "32nd" => DataDurationCmn::N32,
        "64th" => DataDurationCmn::N64,
        "128th" => DataDurationCmn::N128,
        "256th" => DataDurationCmn::N256,
        "512th" => DataDurationCmn::N512,
        "1024th" => DataDurationCmn::N1024,
        _ => return None,
    };
    Some(DataDuration::MeiDataDurationCmn(cmn))
}

/// Convert MusicXML grace note to MEI grace attribute.
pub(crate) fn convert_grace(grace: &crate::model::note::Grace) -> DataGrace {
    use crate::model::data::YesNo;

    // MusicXML grace/@slash="yes" → MEI @grace="unacc" (unaccented/slashed)
    // MusicXML grace/@slash="no" or absent → MEI @grace="acc" (accented/no slash)
    match grace.slash {
        Some(YesNo::Yes) => DataGrace::Unacc,
        _ => DataGrace::Acc,
    }
}

// ============================================================================
// Stem Direction Conversion
// ============================================================================

/// Convert MusicXML StemValue to MEI DataStemdirection.
pub(crate) fn convert_stem_direction(stem: StemValue) -> DataStemdirection {
    match stem {
        StemValue::Up => DataStemdirection::MeiDataStemdirectionBasic(DataStemdirectionBasic::Up),
        StemValue::Down => {
            DataStemdirection::MeiDataStemdirectionBasic(DataStemdirectionBasic::Down)
        }
        StemValue::Double => {
            // MEI doesn't have double, default to up
            DataStemdirection::MeiDataStemdirectionBasic(DataStemdirectionBasic::Up)
        }
        StemValue::None => {
            // No stem, but still need a direction value
            DataStemdirection::MeiDataStemdirectionBasic(DataStemdirectionBasic::Up)
        }
    }
}

// ============================================================================
// Dynamics Conversion Helpers
// ============================================================================

/// Convert a MusicXML dynamics value to string.
pub(crate) fn dynamics_value_to_string(value: &crate::model::direction::DynamicsValue) -> String {
    use crate::model::direction::DynamicsValue;

    match value {
        DynamicsValue::Ppp => "ppp".to_string(),
        DynamicsValue::Pp => "pp".to_string(),
        DynamicsValue::P => "p".to_string(),
        DynamicsValue::Mp => "mp".to_string(),
        DynamicsValue::Mf => "mf".to_string(),
        DynamicsValue::F => "f".to_string(),
        DynamicsValue::Ff => "ff".to_string(),
        DynamicsValue::Fff => "fff".to_string(),
        DynamicsValue::Fp => "fp".to_string(),
        DynamicsValue::Pf => "pf".to_string(),
        DynamicsValue::Sf => "sf".to_string(),
        DynamicsValue::Sfz => "sfz".to_string(),
        DynamicsValue::Sfp => "sfp".to_string(),
        DynamicsValue::Sfpp => "sfpp".to_string(),
        DynamicsValue::Sffz => "sffz".to_string(),
        DynamicsValue::Sfzp => "sfzp".to_string(),
        DynamicsValue::Rf => "rf".to_string(),
        DynamicsValue::Rfz => "rfz".to_string(),
        DynamicsValue::Fz => "fz".to_string(),
        DynamicsValue::N => "n".to_string(),
        DynamicsValue::Pppp => "pppp".to_string(),
        DynamicsValue::Ffff => "ffff".to_string(),
        DynamicsValue::Ppppp => "ppppp".to_string(),
        DynamicsValue::Fffff => "fffff".to_string(),
        DynamicsValue::Pppppp => "pppppp".to_string(),
        DynamicsValue::Ffffff => "ffffff".to_string(),
        DynamicsValue::OtherDynamics(s) => s.clone(),
    }
}

// ============================================================================
// Metronome/Tempo Formatting Helpers
// ============================================================================

/// Format metronome marking as text for display.
pub(crate) fn format_metronome_text(beat_unit: &str, dots: usize, per_minute: &str) -> String {
    let beat_unit_symbol = match beat_unit {
        "whole" => "𝅝",
        "half" => "𝅗𝅥",
        "quarter" => "♩",
        "eighth" => "♪",
        "16th" => "𝅘𝅥𝅯",
        _ => beat_unit,
    };

    let dot_string = ".".repeat(dots);
    format!("{}{} = {}", beat_unit_symbol, dot_string, per_minute)
}

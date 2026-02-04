//! Note, rest, and chord conversion from MusicXML to MEI.

use crate::context::ConversionContext;
use crate::error::ConversionResult;
use tusk_model::att::AttAccidLogFunc;
use tusk_model::data::{
    DataAccidentalGestural, DataAccidentalGesturalBasic, DataAccidentalWritten,
    DataAccidentalWrittenBasic, DataAugmentdot, DataBoolean, DataDuration, DataDurationCmn,
    DataGrace, DataOctave, DataPitchname, DataStemdirection, DataStemdirectionBasic,
};
use tusk_model::elements::{Accid, Chord, ChordChild, NoteChild};
use tusk_musicxml::model::note::{AccidentalValue, FullNoteContent, NoteTypeValue, StemValue};

/// Convert a MusicXML note to MEI note.
///
/// This function handles the conversion of a pitched MusicXML note to MEI,
/// including:
/// - Pitch (step, octave, alter)
/// - Duration (note type and dots)
/// - Accidentals (written accidentals)
/// - Grace notes
/// - Stem direction
///
/// # Arguments
///
/// * `note` - The MusicXML note to convert
/// * `ctx` - The conversion context for tracking state
///
/// # Returns
///
/// An MEI Note element, or an error if conversion fails.
pub fn convert_note(
    note: &tusk_musicxml::model::note::Note,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::Note> {
    use tusk_model::elements::Note as MeiNote;

    let mut mei_note = MeiNote::default();

    // Generate and set xml:id
    let note_id = ctx.generate_id_with_suffix("note");
    mei_note.common.xml_id = Some(note_id.clone());

    // Map original ID if present
    if let Some(ref orig_id) = note.id {
        ctx.map_id(orig_id, note_id);
    }

    // Convert pitch (for pitched notes)
    if let FullNoteContent::Pitch(ref pitch) = note.content {
        // Convert pitch name (step)
        mei_note.note_log.pname = Some(convert_pitch_name(pitch.step));

        // Convert octave
        mei_note.note_log.oct = Some(DataOctave::from(pitch.octave as u64));

        // Store gestural accidental (@accid.ges) for sounding pitch
        // This represents the actual sounding pitch after key signature and accidentals
        if let Some(alter) = pitch.alter {
            mei_note.note_ges.accid_ges = Some(convert_alter_to_gestural_accid(alter));
        }
    }

    // Convert duration
    convert_note_duration(note, &mut mei_note, ctx);

    // Convert grace note
    if note.is_grace()
        && let Some(ref grace) = note.grace
    {
        mei_note.note_log.grace = Some(convert_grace(grace));
    }

    // Convert written accidental (if present)
    if let Some(ref accidental) = note.accidental {
        let accid = convert_accidental(accidental, ctx)?;
        mei_note.children.push(NoteChild::Accid(Box::new(accid)));
    }

    // Convert stem direction (if present)
    if let Some(ref stem) = note.stem {
        mei_note.note_vis.stem_dir = Some(convert_stem_direction(stem.value));
    }

    // Convert cue note
    if note.is_cue() {
        mei_note.note_log.cue = Some(DataBoolean::True);
    }

    Ok(mei_note)
}

/// Convert MusicXML Step to MEI DataPitchname.
pub(crate) fn convert_pitch_name(step: tusk_musicxml::model::data::Step) -> DataPitchname {
    use tusk_musicxml::model::data::Step;

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
pub(crate) fn convert_alter_to_gestural_accid(alter: f64) -> DataAccidentalGestural {
    // Map common alterations to gestural accidentals
    match alter as i32 {
        -2 => DataAccidentalGestural::DataAccidentalGesturalBasic(DataAccidentalGesturalBasic::Ff),
        -1 => DataAccidentalGestural::DataAccidentalGesturalBasic(DataAccidentalGesturalBasic::F),
        0 => DataAccidentalGestural::DataAccidentalGesturalBasic(DataAccidentalGesturalBasic::N),
        1 => DataAccidentalGestural::DataAccidentalGesturalBasic(DataAccidentalGesturalBasic::S),
        2 => DataAccidentalGestural::DataAccidentalGesturalBasic(DataAccidentalGesturalBasic::Ss), // Double sharp
        _ => {
            // For microtones or other alterations, use natural as fallback
            DataAccidentalGestural::DataAccidentalGesturalBasic(DataAccidentalGesturalBasic::N)
        }
    }
}

/// Convert note duration information from MusicXML to MEI.
pub(crate) fn convert_note_duration(
    note: &tusk_musicxml::model::note::Note,
    mei_note: &mut tusk_model::elements::Note,
    ctx: &ConversionContext,
) {
    // Convert note type to MEI duration
    if let Some(ref note_type) = note.note_type {
        mei_note.note_log.dur = Some(convert_note_type_to_duration(note_type.value));
    } else if let Some(duration) = note.duration {
        // Try to infer note type from duration value
        if let Some((inferred_type, _dots)) = ctx.duration_context().infer_note_type(duration) {
            mei_note.note_log.dur = Some(convert_note_type_to_duration(inferred_type));
        }
    }

    // Convert dots
    let dot_count = note.dots.len() as u64;
    if dot_count > 0 {
        mei_note.note_log.dots = Some(DataAugmentdot::from(dot_count));
    }

    // Store gestural duration in ppq (divisions) for MIDI/playback
    if let Some(duration) = note.duration {
        mei_note.note_ges.dur_ppq = Some(duration as u64);
    }
}

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
    DataDuration::DataDurationCmn(dur)
}

/// Convert MusicXML grace note to MEI grace attribute.
pub(crate) fn convert_grace(grace: &tusk_musicxml::model::note::Grace) -> DataGrace {
    use tusk_musicxml::model::data::YesNo;

    // MusicXML grace/@slash="yes" → MEI @grace="unacc" (unaccented/slashed)
    // MusicXML grace/@slash="no" or absent → MEI @grace="acc" (accented/no slash)
    match grace.slash {
        Some(YesNo::Yes) => DataGrace::Unacc,
        _ => DataGrace::Acc,
    }
}

/// Convert MusicXML accidental to MEI accid element.
pub(crate) fn convert_accidental(
    accidental: &tusk_musicxml::model::note::Accidental,
    ctx: &mut ConversionContext,
) -> ConversionResult<Accid> {
    use tusk_musicxml::model::data::YesNo;

    let mut accid = Accid::default();

    // Generate ID
    let accid_id = ctx.generate_id_with_suffix("accid");
    accid.common.xml_id = Some(accid_id);

    // Convert accidental value
    accid.accid_log.accid = Some(convert_accidental_value(accidental.value));

    // Convert cautionary flag
    if let Some(YesNo::Yes) = accidental.cautionary {
        accid.accid_log.func = Some(AttAccidLogFunc::Caution);
    }

    // Convert editorial flag
    if let Some(YesNo::Yes) = accidental.editorial {
        accid.accid_log.func = Some(AttAccidLogFunc::Edit);
    }

    // Convert parentheses/bracket enclosure
    if let Some(YesNo::Yes) = accidental.parentheses {
        accid.accid_vis.enclose = Some(tusk_model::data::DataEnclosure::Paren);
    } else if let Some(YesNo::Yes) = accidental.bracket {
        accid.accid_vis.enclose = Some(tusk_model::data::DataEnclosure::Brack);
    }

    Ok(accid)
}

/// Convert MusicXML AccidentalValue to MEI DataAccidentalWritten.
pub(crate) fn convert_accidental_value(value: AccidentalValue) -> DataAccidentalWritten {
    let basic = match value {
        AccidentalValue::Sharp => DataAccidentalWrittenBasic::S,
        AccidentalValue::Natural => DataAccidentalWrittenBasic::N,
        AccidentalValue::Flat => DataAccidentalWrittenBasic::F,
        AccidentalValue::DoubleSharp | AccidentalValue::SharpSharp => DataAccidentalWrittenBasic::X,
        AccidentalValue::FlatFlat => DataAccidentalWrittenBasic::Ff,
        AccidentalValue::NaturalSharp => DataAccidentalWrittenBasic::Ns,
        AccidentalValue::NaturalFlat => DataAccidentalWrittenBasic::Nf,
        AccidentalValue::TripleSharp => DataAccidentalWrittenBasic::Ts,
        AccidentalValue::TripleFlat => DataAccidentalWrittenBasic::Tf,
        // For extended accidentals (quarter tones, etc.), use the closest basic equivalent
        AccidentalValue::QuarterFlat => DataAccidentalWrittenBasic::F,
        AccidentalValue::QuarterSharp => DataAccidentalWrittenBasic::S,
        AccidentalValue::ThreeQuartersFlat => DataAccidentalWrittenBasic::Ff,
        AccidentalValue::ThreeQuartersSharp => DataAccidentalWrittenBasic::Ss,
        // Arrow variants map to basic equivalents
        AccidentalValue::SharpDown | AccidentalValue::SharpUp => DataAccidentalWrittenBasic::S,
        AccidentalValue::NaturalDown | AccidentalValue::NaturalUp => DataAccidentalWrittenBasic::N,
        AccidentalValue::FlatDown | AccidentalValue::FlatUp => DataAccidentalWrittenBasic::F,
        AccidentalValue::DoubleSharpDown | AccidentalValue::DoubleSharpUp => {
            DataAccidentalWrittenBasic::X
        }
        AccidentalValue::FlatFlatDown | AccidentalValue::FlatFlatUp => {
            DataAccidentalWrittenBasic::Ff
        }
        AccidentalValue::ArrowDown | AccidentalValue::ArrowUp => DataAccidentalWrittenBasic::N,
        // Slash variants
        AccidentalValue::SlashQuarterSharp | AccidentalValue::SlashSharp => {
            DataAccidentalWrittenBasic::S
        }
        AccidentalValue::SlashFlat | AccidentalValue::DoubleSlashFlat => {
            DataAccidentalWrittenBasic::F
        }
        // Numbered sharps/flats (Stein-Zimmermann notation)
        AccidentalValue::Sharp1
        | AccidentalValue::Sharp2
        | AccidentalValue::Sharp3
        | AccidentalValue::Sharp5 => DataAccidentalWrittenBasic::S,
        AccidentalValue::Flat1
        | AccidentalValue::Flat2
        | AccidentalValue::Flat3
        | AccidentalValue::Flat4 => DataAccidentalWrittenBasic::F,
        // Persian accidentals
        AccidentalValue::Sori => DataAccidentalWrittenBasic::S, // Quarter-tone sharp
        AccidentalValue::Koron => DataAccidentalWrittenBasic::F, // Quarter-tone flat
        // Other
        AccidentalValue::Other => DataAccidentalWrittenBasic::N,
    };
    DataAccidentalWritten::DataAccidentalWrittenBasic(basic)
}

/// Convert MusicXML StemValue to MEI DataStemdirection.
pub(crate) fn convert_stem_direction(stem: StemValue) -> DataStemdirection {
    match stem {
        StemValue::Up => DataStemdirection::DataStemdirectionBasic(DataStemdirectionBasic::Up),
        StemValue::Down => DataStemdirection::DataStemdirectionBasic(DataStemdirectionBasic::Down),
        StemValue::Double => {
            // MEI doesn't have double, default to up
            DataStemdirection::DataStemdirectionBasic(DataStemdirectionBasic::Up)
        }
        StemValue::None => {
            // No stem, but still need a direction value
            DataStemdirection::DataStemdirectionBasic(DataStemdirectionBasic::Up)
        }
    }
}

/// Convert a MusicXML rest to MEI rest.
///
/// This function handles the conversion of a MusicXML rest (non-measure rest)
/// to an MEI `<rest>` element, including:
/// - Duration (note type and dots)
/// - Gestural duration in ppq
/// - Cue rests
///
/// # Arguments
///
/// * `note` - The MusicXML note element (must be a rest)
/// * `ctx` - The conversion context for tracking state
///
/// # Returns
///
/// An MEI Rest element, or an error if conversion fails.
pub fn convert_rest(
    note: &tusk_musicxml::model::note::Note,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::Rest> {
    use tusk_model::elements::Rest as MeiRest;
    use tusk_model::generated::data::{DataAugmentdot, DataBoolean, DataDurationrests};

    let mut mei_rest = MeiRest::default();

    // Generate and set xml:id
    let rest_id = ctx.generate_id_with_suffix("rest");
    mei_rest.common.xml_id = Some(rest_id.clone());

    // Map original ID if present
    if let Some(ref orig_id) = note.id {
        ctx.map_id(orig_id, rest_id);
    }

    // Convert duration
    if let Some(ref note_type) = note.note_type {
        let dur = convert_note_type_to_duration_cmn(note_type.value);
        mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(dur));
    } else if let Some(duration) = note.duration {
        // Try to infer note type from duration value
        if let Some((inferred_type, _dots)) = ctx.duration_context().infer_note_type(duration) {
            let dur = convert_note_type_to_duration_cmn(inferred_type);
            mei_rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(dur));
        }
    }

    // Convert dots
    let dot_count = note.dots.len() as u64;
    if dot_count > 0 {
        mei_rest.rest_log.dots = Some(DataAugmentdot::from(dot_count));
    }

    // Store gestural duration in ppq (divisions)
    if let Some(duration) = note.duration {
        mei_rest.rest_ges.dur_ppq = Some(duration as u64);
    }

    // Handle cue rest
    if note.cue.is_some() {
        mei_rest.rest_log.cue = Some(DataBoolean::True);
    }

    Ok(mei_rest)
}

/// Convert a MusicXML measure rest to MEI mRest.
///
/// This function handles the conversion of a MusicXML whole-measure rest
/// (where `rest/@measure="yes"`) to an MEI `<mRest>` element.
///
/// # Arguments
///
/// * `note` - The MusicXML note element (must be a measure rest)
/// * `ctx` - The conversion context for tracking state
///
/// # Returns
///
/// An MEI MRest element, or an error if conversion fails.
pub fn convert_measure_rest(
    note: &tusk_musicxml::model::note::Note,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::MRest> {
    use tusk_model::elements::MRest;
    use tusk_model::generated::data::DataBoolean;

    let mut mei_mrest = MRest::default();

    // Generate and set xml:id
    let mrest_id = ctx.generate_id_with_suffix("mrest");
    mei_mrest.common.xml_id = Some(mrest_id.clone());

    // Map original ID if present
    if let Some(ref orig_id) = note.id {
        ctx.map_id(orig_id, mrest_id);
    }

    // Store gestural duration in ppq (divisions)
    // Measure rests don't need a written duration (they fill the measure)
    if let Some(duration) = note.duration {
        mei_mrest.m_rest_ges.dur_ppq = Some(duration as u64);
    }

    // Handle cue rest
    if note.cue.is_some() {
        mei_mrest.m_rest_log.cue = Some(DataBoolean::True);
    }

    Ok(mei_mrest)
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

/// Check if a MusicXML rest is a whole-measure rest.
pub fn is_measure_rest(note: &tusk_musicxml::model::note::Note) -> bool {
    use tusk_musicxml::model::data::YesNo;
    use tusk_musicxml::model::note::FullNoteContent;

    match &note.content {
        FullNoteContent::Rest(rest) => rest.measure == Some(YesNo::Yes),
        _ => false,
    }
}

/// Convert a group of MusicXML notes forming a chord to MEI chord.
///
/// In MusicXML, a chord is represented as a sequence of notes where all notes
/// after the first have the `<chord/>` element, indicating they share timing
/// with the previous note. All notes in a chord must have the same duration.
///
/// # Arguments
///
/// * `notes` - A slice of MusicXML notes forming the chord. The first note
///   should NOT have the chord flag; subsequent notes should have it.
/// * `ctx` - The conversion context for tracking state
///
/// # Returns
///
/// An MEI Chord element containing all the notes, or an error if conversion fails.
pub fn convert_chord(
    notes: &[tusk_musicxml::model::note::Note],
    ctx: &mut ConversionContext,
) -> ConversionResult<Chord> {
    let mut mei_chord = Chord::default();

    // Generate and set xml:id
    let chord_id = ctx.generate_id_with_suffix("chord");
    mei_chord.common.xml_id = Some(chord_id);

    // Get duration info from the first note (all notes in a chord share duration)
    if let Some(first_note) = notes.first() {
        // Convert duration
        if let Some(ref note_type) = first_note.note_type {
            mei_chord.chord_log.dur = Some(convert_note_type_to_duration(note_type.value));
        } else if let Some(duration) = first_note.duration {
            // Try to infer note type from duration value
            if let Some((inferred_type, _dots)) = ctx.duration_context().infer_note_type(duration) {
                mei_chord.chord_log.dur = Some(convert_note_type_to_duration(inferred_type));
            }
        }

        // Convert dots
        let dot_count = first_note.dots.len() as u64;
        if dot_count > 0 {
            mei_chord.chord_log.dots = Some(DataAugmentdot::from(dot_count));
        }

        // Store gestural duration in ppq (divisions)
        if let Some(duration) = first_note.duration {
            mei_chord.chord_ges.dur_ppq = Some(duration as u64);
        }

        // Handle grace chord
        if first_note.is_grace()
            && let Some(ref grace) = first_note.grace
        {
            mei_chord.chord_log.grace = Some(convert_grace(grace));
        }

        // Handle cue chord
        if first_note.is_cue() {
            mei_chord.chord_log.cue = Some(DataBoolean::True);
        }
    }

    // Convert each note in the chord and add as children
    for note in notes {
        let mei_note = convert_note(note, ctx)?;
        mei_chord
            .children
            .push(ChordChild::Note(Box::new(mei_note)));
    }

    Ok(mei_chord)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;

    // ============================================================================
    // Note Conversion Tests
    // ============================================================================

    #[test]
    fn convert_note_sets_pitch_name() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.pname.as_ref().unwrap().0, "c");
    }

    #[test]
    fn convert_note_sets_octave() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::new(Step::G, 5), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.oct.as_ref().unwrap().0, 5);
    }

    #[test]
    fn convert_note_with_sharp_alter() {
        use tusk_model::data::{DataAccidentalGestural, DataAccidentalGesturalBasic};
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::with_alter(Step::F, 1.0, 4), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.pname.as_ref().unwrap().0, "f");
        assert_eq!(
            mei_note.note_ges.accid_ges,
            Some(DataAccidentalGestural::DataAccidentalGesturalBasic(
                DataAccidentalGesturalBasic::S
            ))
        );
    }

    #[test]
    fn convert_note_with_flat_alter() {
        use tusk_model::data::{DataAccidentalGestural, DataAccidentalGesturalBasic};
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::with_alter(Step::B, -1.0, 4), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.pname.as_ref().unwrap().0, "b");
        assert_eq!(
            mei_note.note_ges.accid_ges,
            Some(DataAccidentalGestural::DataAccidentalGesturalBasic(
                DataAccidentalGesturalBasic::F
            ))
        );
    }

    #[test]
    fn convert_note_with_duration() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, NoteType, NoteTypeValue, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_note_with_dots() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Dot, Note, NoteType, NoteTypeValue, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::D, 4), 6.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        note.dots.push(Dot::default()); // One dot

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
        assert_eq!(mei_note.note_log.dots.as_ref().unwrap().0, 1);
    }

    #[test]
    fn convert_note_infers_duration_from_divisions() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        // No note_type, but duration is set
        let note = Note::pitched(Pitch::new(Step::A, 4), 4.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0); // 4 divisions = quarter note

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        // Should infer quarter note from duration=4 with divisions=4
        assert_eq!(
            mei_note.note_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_note_stores_gestural_duration() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::new(Step::C, 4), 96.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        // Should store duration in ppq
        assert_eq!(mei_note.note_ges.dur_ppq, Some(96));
    }

    #[test]
    fn convert_grace_note_unaccented() {
        use tusk_musicxml::model::data::{Step, YesNo};
        use tusk_musicxml::model::note::{Grace, Note, Pitch};

        let mut grace = Grace::default();
        grace.slash = Some(YesNo::Yes); // Slashed grace note

        let note = Note::grace_note(Pitch::new(Step::D, 5), grace);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.grace, Some(DataGrace::Unacc));
    }

    #[test]
    fn convert_grace_note_accented() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Grace, Note, Pitch};

        // No slash = accented grace note
        let note = Note::grace_note(Pitch::new(Step::E, 4), Grace::default());

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.grace, Some(DataGrace::Acc));
    }

    #[test]
    fn convert_note_with_written_accidental_sharp() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::with_alter(Step::F, 1.0, 4), 4.0);
        note.accidental = Some(Accidental::new(AccidentalValue::Sharp));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        // Should have an accid child
        let accid_child = mei_note
            .children
            .iter()
            .find(|c| matches!(c, NoteChild::Accid(_)));
        assert!(accid_child.is_some());

        if let Some(NoteChild::Accid(accid)) = accid_child {
            assert_eq!(
                accid.accid_log.accid,
                Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
                    DataAccidentalWrittenBasic::S
                ))
            );
        }
    }

    #[test]
    fn convert_note_with_written_accidental_flat() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::with_alter(Step::B, -1.0, 4), 4.0);
        note.accidental = Some(Accidental::new(AccidentalValue::Flat));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        let accid_child = mei_note
            .children
            .iter()
            .find(|c| matches!(c, NoteChild::Accid(_)));
        assert!(accid_child.is_some());

        if let Some(NoteChild::Accid(accid)) = accid_child {
            assert_eq!(
                accid.accid_log.accid,
                Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
                    DataAccidentalWrittenBasic::F
                ))
            );
        }
    }

    #[test]
    fn convert_note_with_cautionary_accidental() {
        use tusk_musicxml::model::data::{Step, YesNo};
        use tusk_musicxml::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        let mut accidental = Accidental::new(AccidentalValue::Natural);
        accidental.cautionary = Some(YesNo::Yes);
        note.accidental = Some(accidental);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(accid.accid_log.func, Some(AttAccidLogFunc::Caution));
        } else {
            panic!("Expected accid child");
        }
    }

    #[test]
    fn convert_note_with_editorial_accidental() {
        use tusk_musicxml::model::data::{Step, YesNo};
        use tusk_musicxml::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::G, 4), 4.0);
        let mut accidental = Accidental::new(AccidentalValue::Sharp);
        accidental.editorial = Some(YesNo::Yes);
        note.accidental = Some(accidental);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(accid.accid_log.func, Some(AttAccidLogFunc::Edit));
        } else {
            panic!("Expected accid child");
        }
    }

    #[test]
    fn convert_note_with_parentheses_accidental() {
        use tusk_musicxml::model::data::{Step, YesNo};
        use tusk_musicxml::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::A, 4), 4.0);
        let mut accidental = Accidental::new(AccidentalValue::Natural);
        accidental.parentheses = Some(YesNo::Yes);
        note.accidental = Some(accidental);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(
                accid.accid_vis.enclose,
                Some(tusk_model::data::DataEnclosure::Paren)
            );
        } else {
            panic!("Expected accid child");
        }
    }

    #[test]
    fn convert_note_with_stem_up() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch, Stem, StemValue};

        let mut note = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note.stem = Some(Stem::new(StemValue::Up));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_vis.stem_dir,
            Some(DataStemdirection::DataStemdirectionBasic(
                DataStemdirectionBasic::Up
            ))
        );
    }

    #[test]
    fn convert_note_with_stem_down() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch, Stem, StemValue};

        let mut note = Note::pitched(Pitch::new(Step::A, 5), 4.0);
        note.stem = Some(Stem::new(StemValue::Down));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_vis.stem_dir,
            Some(DataStemdirection::DataStemdirectionBasic(
                DataStemdirectionBasic::Down
            ))
        );
    }

    #[test]
    fn convert_cue_note() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::C, 5), 4.0);
        note.cue = Some(Empty);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.cue, Some(DataBoolean::True));
    }

    #[test]
    fn convert_note_generates_xml_id() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::new(Step::D, 4), 4.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert!(mei_note.common.xml_id.is_some());
        assert!(mei_note.common.xml_id.as_ref().unwrap().contains("note"));
    }

    #[test]
    fn convert_note_maps_original_id() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::F, 4), 4.0);
        note.id = Some("original-note-id".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        // Should map the original ID to the MEI ID
        let mei_id = ctx.get_mei_id("original-note-id");
        assert!(mei_id.is_some());
        assert_eq!(mei_id, mei_note.common.xml_id.as_deref());
    }

    #[test]
    fn convert_note_all_pitch_names() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, Pitch};

        let steps = [
            (Step::A, "a"),
            (Step::B, "b"),
            (Step::C, "c"),
            (Step::D, "d"),
            (Step::E, "e"),
            (Step::F, "f"),
            (Step::G, "g"),
        ];

        for (step, expected) in steps {
            let note = Note::pitched(Pitch::new(step, 4), 4.0);
            let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
            let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

            assert_eq!(
                mei_note.note_log.pname.as_ref().unwrap().0,
                expected,
                "Failed for step {:?}",
                step
            );
        }
    }

    #[test]
    fn convert_note_various_durations() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Note, NoteType, NoteTypeValue, Pitch};

        let durations = [
            (NoteTypeValue::Whole, DataDurationCmn::N1),
            (NoteTypeValue::Half, DataDurationCmn::N2),
            (NoteTypeValue::Quarter, DataDurationCmn::N4),
            (NoteTypeValue::Eighth, DataDurationCmn::N8),
            (NoteTypeValue::N16th, DataDurationCmn::N16),
            (NoteTypeValue::N32nd, DataDurationCmn::N32),
            (NoteTypeValue::N64th, DataDurationCmn::N64),
        ];

        for (mxml_dur, mei_dur) in durations {
            let mut note = Note::pitched(Pitch::new(Step::C, 4), 4.0);
            note.note_type = Some(NoteType::new(mxml_dur));

            let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
            let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

            assert_eq!(
                mei_note.note_log.dur,
                Some(DataDuration::DataDurationCmn(mei_dur)),
                "Failed for duration {:?}",
                mxml_dur
            );
        }
    }

    #[test]
    fn convert_note_double_sharp_accidental() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::with_alter(Step::F, 2.0, 4), 4.0);
        note.accidental = Some(Accidental::new(AccidentalValue::DoubleSharp));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(
                accid.accid_log.accid,
                Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
                    DataAccidentalWrittenBasic::X
                ))
            );
        } else {
            panic!("Expected accid child");
        }
    }

    #[test]
    fn convert_note_double_flat_accidental() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::with_alter(Step::B, -2.0, 4), 4.0);
        note.accidental = Some(Accidental::new(AccidentalValue::FlatFlat));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(
                accid.accid_log.accid,
                Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
                    DataAccidentalWrittenBasic::Ff
                ))
            );
        } else {
            panic!("Expected accid child");
        }
    }

    // ============================================================================
    // Rest Conversion Tests
    // ============================================================================

    #[test]
    fn convert_rest_creates_mei_rest() {
        use tusk_musicxml::model::note::{Note, Rest};

        let note = Note::rest(Rest::new(), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        // Rest should have an xml:id
        assert!(mei_rest.common.xml_id.is_some());
    }

    #[test]
    fn convert_rest_with_duration() {
        use tusk_model::generated::data::{DataDurationCmn, DataDurationrests};
        use tusk_musicxml::model::note::{Note, NoteType, NoteTypeValue, Rest};

        let mut note = Note::rest(Rest::new(), 4.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(
            mei_rest.rest_log.dur,
            Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_rest_with_dots() {
        use tusk_model::generated::data::DataAugmentdot;
        use tusk_musicxml::model::note::{Dot, Note, NoteType, NoteTypeValue, Rest};

        let mut note = Note::rest(Rest::new(), 6.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        note.dots.push(Dot::default());

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_rest.rest_log.dots, Some(DataAugmentdot(1)));
    }

    #[test]
    fn convert_rest_infers_duration_from_divisions() {
        use tusk_model::generated::data::{DataDurationCmn, DataDurationrests};
        use tusk_musicxml::model::note::{Note, Rest};

        // A rest with duration 4 when divisions=4 is a quarter note
        let note = Note::rest(Rest::new(), 4.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(
            mei_rest.rest_log.dur,
            Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_rest_stores_gestural_duration() {
        use tusk_musicxml::model::note::{Note, Rest};

        let note = Note::rest(Rest::new(), 8.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_rest.rest_ges.dur_ppq, Some(8));
    }

    #[test]
    fn convert_rest_generates_xml_id() {
        use tusk_musicxml::model::note::{Note, Rest};

        let note = Note::rest(Rest::new(), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert!(mei_rest.common.xml_id.is_some());
        assert!(mei_rest.common.xml_id.as_ref().unwrap().contains("rest"));
    }

    #[test]
    fn convert_rest_maps_original_id() {
        use tusk_musicxml::model::note::{Note, Rest};

        let mut note = Note::rest(Rest::new(), 4.0);
        note.id = Some("original-rest-id".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        let mei_id = mei_rest.common.xml_id.as_ref().unwrap();

        // Check the ID mapping was stored
        assert_eq!(ctx.get_mei_id("original-rest-id"), Some(mei_id.as_str()));
    }

    #[test]
    fn convert_cue_rest() {
        use tusk_model::generated::data::DataBoolean;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Rest};

        let mut note = Note::rest(Rest::new(), 4.0);
        note.cue = Some(Empty);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_rest.rest_log.cue, Some(DataBoolean::True));
    }

    #[test]
    fn convert_rest_various_durations() {
        use tusk_model::generated::data::{DataDurationCmn, DataDurationrests};
        use tusk_musicxml::model::note::{Note, NoteType, NoteTypeValue, Rest};

        let test_cases = [
            (NoteTypeValue::Whole, DataDurationCmn::N1),
            (NoteTypeValue::Half, DataDurationCmn::N2),
            (NoteTypeValue::Quarter, DataDurationCmn::N4),
            (NoteTypeValue::Eighth, DataDurationCmn::N8),
            (NoteTypeValue::N16th, DataDurationCmn::N16),
        ];

        for (mxml_type, mei_dur) in test_cases {
            let mut note = Note::rest(Rest::new(), 4.0);
            note.note_type = Some(NoteType::new(mxml_type));

            let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

            let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
            assert_eq!(
                mei_rest.rest_log.dur,
                Some(DataDurationrests::DataDurationCmn(mei_dur)),
                "Failed for {:?}",
                mxml_type
            );
        }
    }

    #[test]
    fn convert_measure_rest_creates_mrest() {
        use tusk_musicxml::model::data::YesNo;
        use tusk_musicxml::model::note::{Note, Rest};

        let mut rest = Rest::new();
        rest.measure = Some(YesNo::Yes);
        let note = Note::rest(rest, 16.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_mrest = convert_measure_rest(&note, &mut ctx).expect("conversion should succeed");
        assert!(mei_mrest.common.xml_id.is_some());
    }

    #[test]
    fn convert_measure_rest_generates_xml_id() {
        use tusk_musicxml::model::data::YesNo;
        use tusk_musicxml::model::note::{Note, Rest};

        let mut rest = Rest::new();
        rest.measure = Some(YesNo::Yes);
        let note = Note::rest(rest, 16.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_mrest = convert_measure_rest(&note, &mut ctx).expect("conversion should succeed");
        assert!(mei_mrest.common.xml_id.as_ref().unwrap().contains("mrest"));
    }

    #[test]
    fn convert_measure_rest_stores_gestural_duration() {
        use tusk_musicxml::model::data::YesNo;
        use tusk_musicxml::model::note::{Note, Rest};

        let mut rest = Rest::new();
        rest.measure = Some(YesNo::Yes);
        let note = Note::rest(rest, 16.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_mrest = convert_measure_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_mrest.m_rest_ges.dur_ppq, Some(16));
    }

    #[test]
    fn convert_cue_measure_rest() {
        use tusk_model::generated::data::DataBoolean;
        use tusk_musicxml::model::data::YesNo;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Rest};

        let mut rest = Rest::new();
        rest.measure = Some(YesNo::Yes);
        let mut note = Note::rest(rest, 16.0);
        note.cue = Some(Empty);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_mrest = convert_measure_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_mrest.m_rest_log.cue, Some(DataBoolean::True));
    }

    // ============================================================================
    // Chord Conversion Tests
    // ============================================================================

    #[test]
    fn convert_chord_creates_mei_chord() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Pitch};

        // Create a C major chord (C4, E4, G4)
        let note1 = Note::pitched(Pitch::new(Step::C, 4), 4.0); // First note - no chord flag
        let mut note2 = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note2.chord = Some(Empty); // Chord flag
        let mut note3 = Note::pitched(Pitch::new(Step::G, 4), 4.0);
        note3.chord = Some(Empty); // Chord flag

        let notes = vec![note1, note2, note3];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        // Chord should have xml:id
        assert!(mei_chord.common.xml_id.is_some());
    }

    #[test]
    fn convert_chord_contains_all_notes() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Pitch};

        // Create a C major chord (C4, E4, G4)
        let note1 = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        let mut note2 = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note2.chord = Some(Empty);
        let mut note3 = Note::pitched(Pitch::new(Step::G, 4), 4.0);
        note3.chord = Some(Empty);

        let notes = vec![note1, note2, note3];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        // Should have 3 note children
        let note_count = mei_chord
            .children
            .iter()
            .filter(|c| matches!(c, ChordChild::Note(_)))
            .count();
        assert_eq!(note_count, 3);
    }

    #[test]
    fn convert_chord_sets_duration() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, NoteType, NoteTypeValue, Pitch};

        let mut note1 = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        note1.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        let mut note2 = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note2.chord = Some(Empty);
        note2.note_type = Some(NoteType::new(NoteTypeValue::Quarter));

        let notes = vec![note1, note2];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        // Chord should have duration set
        assert_eq!(
            mei_chord.chord_log.dur,
            Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_chord_sets_dots() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Dot, Note, NoteType, NoteTypeValue, Pitch};

        let mut note1 = Note::pitched(Pitch::new(Step::D, 4), 6.0);
        note1.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        note1.dots.push(Dot::default());
        let mut note2 = Note::pitched(Pitch::new(Step::F, 4), 6.0);
        note2.chord = Some(Empty);
        note2.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        note2.dots.push(Dot::default());

        let notes = vec![note1, note2];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_chord.chord_log.dots.as_ref().unwrap().0, 1);
    }

    #[test]
    fn convert_chord_stores_gestural_duration() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note1 = Note::pitched(Pitch::new(Step::A, 3), 96.0);
        let mut note2 = Note::pitched(Pitch::new(Step::C, 4), 96.0);
        note2.chord = Some(Empty);

        let notes = vec![note1, note2];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_chord.chord_ges.dur_ppq, Some(96));
    }

    #[test]
    fn convert_chord_generates_xml_id() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note1 = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        let mut note2 = Note::pitched(Pitch::new(Step::G, 4), 4.0);
        note2.chord = Some(Empty);

        let notes = vec![note1, note2];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        let id = mei_chord.common.xml_id.as_ref().unwrap();
        assert!(id.contains("chord"));
    }

    #[test]
    fn convert_chord_note_pitches_preserved() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Pitch};

        let note1 = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        let mut note2 = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note2.chord = Some(Empty);
        let mut note3 = Note::pitched(Pitch::new(Step::G, 4), 4.0);
        note3.chord = Some(Empty);

        let notes = vec![note1, note2, note3];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        // Extract pitch names from note children
        let pitches: Vec<&str> = mei_chord
            .children
            .iter()
            .filter_map(|c| match c {
                ChordChild::Note(n) => n.note_log.pname.as_ref().map(|p| p.0.as_str()),
                _ => None,
            })
            .collect();

        assert_eq!(pitches, vec!["c", "e", "g"]);
    }

    #[test]
    fn convert_chord_with_accidentals() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Note, Pitch};

        // C# E G# chord
        let note1 = Note::pitched(Pitch::with_alter(Step::C, 1.0, 4), 4.0);
        let mut note2 = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note2.chord = Some(Empty);
        let mut note3 = Note::pitched(Pitch::with_alter(Step::G, 1.0, 4), 4.0);
        note3.chord = Some(Empty);

        let notes = vec![note1, note2, note3];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        // Should have 3 notes, first and last with gestural accidentals
        let notes: Vec<_> = mei_chord
            .children
            .iter()
            .filter_map(|c| match c {
                ChordChild::Note(n) => Some(n.as_ref()),
                _ => None,
            })
            .collect();

        assert_eq!(notes.len(), 3);
        // First note (C#) should have sharp accidental
        assert!(notes[0].note_ges.accid_ges.is_some());
        // Second note (E) has no alteration
        assert!(notes[1].note_ges.accid_ges.is_none());
        // Third note (G#) should have sharp accidental
        assert!(notes[2].note_ges.accid_ges.is_some());
    }

    #[test]
    fn convert_chord_grace_notes() {
        use tusk_musicxml::model::data::Step;
        use tusk_musicxml::model::elements::Empty;
        use tusk_musicxml::model::note::{Grace, Note, Pitch};

        let note1 = Note::grace_note(Pitch::new(Step::C, 4), Grace::default());
        let mut note2 = Note::grace_note(Pitch::new(Step::E, 4), Grace::default());
        note2.chord = Some(Empty);

        let notes = vec![note1, note2];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        // Grace chord should have grace attribute
        assert_eq!(mei_chord.chord_log.grace, Some(DataGrace::Acc));
    }
}

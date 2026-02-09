//! Note, rest, and chord conversion from MusicXML to MEI.

use crate::context::ConversionContext;
use crate::context::{PendingSlur, PendingTuplet};
use crate::convert_error::ConversionResult;
use crate::import::utils::{
    convert_accidental_value, convert_alter_to_gestural_accid, convert_grace,
    convert_note_type_to_duration, convert_note_type_to_duration_cmn, convert_pitch_name,
    convert_stem_direction,
};
use crate::model::StartStop;
use crate::model::StartStopContinue;
use crate::model::notations::{Articulations, TiedType};
use crate::model::note::{FullNoteContent, Note as MusicXmlNote};
// Some of these are only used in #[cfg(test)] modules; allow unused when building lib without tests.
#[allow(unused_imports)]
use tusk_model::data::{
    DataAccidentalWritten, DataAccidentalWrittenBasic, DataArticulation, DataAugmentdot,
    DataBoolean, DataDuration, DataDurationCmn, DataDurationrests, DataEnclosure, DataGrace,
    DataOctave, DataPitchname, DataStaffloc, DataStemdirection, DataStemdirectionBasic, DataTie,
};
use tusk_model::elements::{Accid, Chord, ChordChild, Note as MeiNote, NoteChild};

/// Convert a MusicXML note to MEI note.
///
/// This function handles the conversion of a MusicXML note to MEI,
/// including:
/// - Pitch (step, octave, alter) for pitched notes
/// - Staff location (@loc) for unpitched percussion notes
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
    note: &crate::model::note::Note,
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

    // Convert pitch/unpitched content
    match &note.content {
        FullNoteContent::Pitch(pitch) => {
            mei_note.note_log.pname = Some(convert_pitch_name(pitch.step));
            mei_note.note_log.oct = Some(DataOctave::from(pitch.octave as u64));

            if let Some(alter) = pitch.alter {
                mei_note.note_ges.accid_ges = Some(convert_alter_to_gestural_accid(alter));
            }
        }
        FullNoteContent::Unpitched(unpitched) => {
            // For unpitched notes (percussion), use @loc for visual staff positioning
            // @loc is calculated from display-step and display-octave
            if let (Some(display_step), Some(display_octave)) =
                (unpitched.display_step, unpitched.display_octave)
            {
                mei_note.note_vis.loc = Some(DataStaffloc::from(calculate_staff_loc(
                    display_step,
                    display_octave,
                )));
            }
            // Note: pname and oct are NOT set for unpitched notes
        }
        FullNoteContent::Rest(_) => {
            // Rests are handled by convert_rest, not convert_note
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

    // Convert ties (using @tie attribute)
    convert_ties(note, &mut mei_note);

    // Convert articulations
    convert_articulations(note, &mut mei_note, ctx);

    // Process slurs (track pending, resolve completed)
    let note_id = mei_note.common.xml_id.clone().unwrap_or_default();
    process_slurs(note, &note_id, ctx);

    // Process tuplets (track pending, resolve completed)
    process_tuplets(note, &note_id, ctx);

    // Process ornaments (create control events)
    process_ornaments(note, &note_id, ctx);

    // Process fermatas
    process_fermatas(note, &note_id, ctx);

    // Process arpeggiate/non-arpeggiate
    process_arpeggiate(note, &note_id, ctx);

    // Process glissandos/slides
    process_glissandos(note, &note_id, ctx);

    // Process standalone accidental marks
    process_accidental_marks(note, &note_id, ctx);

    // Process technical notations
    process_technical(note, &note_id, ctx);

    Ok(mei_note)
}

/// Calculate MEI staff location (@loc) from MusicXML display-step and display-octave.
///
/// MEI @loc represents position on the staff where 0 is the bottom line.
/// Each step (A-G) increments by 1. This gives a rough staff position.
///
/// For a standard 5-line staff with treble clef:
/// - E4 is the bottom line (loc=0)
/// - F4 is the first space (loc=1)
/// - G4 is the second line (loc=2)
/// etc.
///
/// We calculate a relative position, recognizing that the actual mapping
/// depends on the clef in use. We use a simple formula that preserves
/// the display position for round-trip conversion.
fn calculate_staff_loc(step: crate::model::data::Step, octave: u8) -> i64 {
    use crate::model::data::Step;

    // Convert step to a numeric value (C=0, D=1, E=2, F=3, G=4, A=5, B=6)
    let step_value = match step {
        Step::C => 0,
        Step::D => 1,
        Step::E => 2,
        Step::F => 3,
        Step::G => 4,
        Step::A => 5,
        Step::B => 6,
    };

    // Calculate absolute position: 7 steps per octave
    // This gives us a continuous value we can use for @loc
    // The value is relative, and the actual staff position depends on clef
    (octave as i64) * 7 + step_value
}

/// Convert note duration information from MusicXML to MEI.
pub(crate) fn convert_note_duration(
    note: &crate::model::note::Note,
    mei_note: &mut tusk_model::elements::Note,
    ctx: &ConversionContext,
) {
    // Convert note type to MEI duration
    if let Some(ref note_type) = note.note_type {
        let dur = convert_note_type_to_duration(note_type.value);
        mei_note.note_log.dur = Some(dur);
    } else if let Some(duration) = note.duration {
        if let Some((inferred_type, _dots)) = ctx.duration_context().infer_note_type(duration) {
            let dur = convert_note_type_to_duration(inferred_type);
            mei_note.note_log.dur = Some(dur);
        }
    }

    // Convert dots
    let dot_count = note.dots.len() as u64;
    if dot_count > 0 {
        mei_note.note_log.dots = Some(DataAugmentdot::from(dot_count));
    }

    // Store gestural duration in ppq (MEI @dur.ppq is Option<String>)
    if let Some(duration) = note.duration {
        mei_note.note_ges.dur_ppq = Some((duration as u64).to_string());
    }
}

/// Convert MusicXML accidental to MEI accid element.
pub(crate) fn convert_accidental(
    accidental: &crate::model::note::Accidental,
    ctx: &mut ConversionContext,
) -> ConversionResult<Accid> {
    use crate::model::data::YesNo;

    let mut accid = Accid::default();

    // Generate ID
    let accid_id = ctx.generate_id_with_suffix("accid");
    accid.common.xml_id = Some(accid_id);

    // Convert accidental value
    accid.accid_log.accid = Some(convert_accidental_value(accidental.value));

    // Convert cautionary/editorial (MEI @func is Option<String>)
    if let Some(YesNo::Yes) = accidental.cautionary {
        accid.accid_log.func = Some("caution".to_string());
    }
    if let Some(YesNo::Yes) = accidental.editorial {
        accid.accid_log.func = Some("edit".to_string());
    }

    // Convert enclosure
    use tusk_model::data::DataEnclosure;
    if let Some(YesNo::Yes) = accidental.parentheses {
        accid.accid_vis.enclose = Some(DataEnclosure::Paren);
    } else if let Some(YesNo::Yes) = accidental.bracket {
        accid.accid_vis.enclose = Some(DataEnclosure::Brack);
    }

    Ok(accid)
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
    note: &crate::model::note::Note,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::Rest> {
    use tusk_model::elements::Rest as MeiRest;

    let mut mei_rest = MeiRest::default();

    // Generate and set xml:id
    let rest_id = ctx.generate_id_with_suffix("rest");
    mei_rest.common.xml_id = Some(rest_id.clone());

    // Map original ID if present
    if let Some(ref orig_id) = note.id {
        ctx.map_id(orig_id, rest_id);
    }

    // Convert duration — only set @dur when MusicXML has an explicit <type>,
    // not when inferred from <duration>. Whole-measure rests intentionally omit
    // <type> and rely on <duration> alone; dur_ppq captures this below.
    if let Some(ref note_type) = note.note_type {
        let dur = convert_note_type_to_duration_cmn(note_type.value);
        mei_rest.rest_log.dur = Some(tusk_model::data::DataDurationrests::MeiDataDurationCmn(dur));
    }

    let dot_count = note.dots.len() as u64;
    if dot_count > 0 {
        mei_rest.rest_log.dots = Some(DataAugmentdot::from(dot_count));
    }

    if let Some(duration) = note.duration {
        mei_rest.rest_ges.dur_ppq = Some((duration as u64).to_string());
    }

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
    note: &crate::model::note::Note,
    ctx: &mut ConversionContext,
) -> ConversionResult<tusk_model::elements::MRest> {
    use tusk_model::elements::MRest;

    let mut mei_mrest = MRest::default();

    // Generate and set xml:id
    let mrest_id = ctx.generate_id_with_suffix("mrest");
    mei_mrest.common.xml_id = Some(mrest_id.clone());

    // Map original ID if present
    if let Some(ref orig_id) = note.id {
        ctx.map_id(orig_id, mrest_id);
    }

    if let Some(duration) = note.duration {
        mei_mrest.m_rest_ges.dur_ppq = Some((duration as u64).to_string());
    }

    if note.cue.is_some() {
        mei_mrest.m_rest_log.cue = Some(DataBoolean::True);
    }

    Ok(mei_mrest)
}

/// Check if a MusicXML rest is a whole-measure rest.
pub fn is_measure_rest(note: &crate::model::note::Note) -> bool {
    use crate::model::data::YesNo;
    use crate::model::note::FullNoteContent;

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
    notes: &[crate::model::note::Note],
    ctx: &mut ConversionContext,
) -> ConversionResult<Chord> {
    let mut mei_chord = Chord::default();

    // Generate and set xml:id
    let chord_id = ctx.generate_id_with_suffix("chord");
    mei_chord.common.xml_id = Some(chord_id);

    // Get duration info from the first note (all notes in a chord share duration)
    if let Some(first_note) = notes.first() {
        if let Some(ref note_type) = first_note.note_type {
            let dur = convert_note_type_to_duration(note_type.value);
            mei_chord.chord_log.dur = Some(dur);
        } else if let Some(duration) = first_note.duration {
            if let Some((inferred_type, _dots)) = ctx.duration_context().infer_note_type(duration) {
                let dur = convert_note_type_to_duration(inferred_type);
                mei_chord.chord_log.dur = Some(dur);
            }
        }

        let dot_count = first_note.dots.len() as u64;
        if dot_count > 0 {
            mei_chord.chord_log.dots = Some(DataAugmentdot::from(dot_count));
        }

        if let Some(duration) = first_note.duration {
            mei_chord.chord_ges.dur_ppq = Some((duration as u64).to_string());
        }

        if first_note.is_grace()
            && let Some(ref grace) = first_note.grace
        {
            mei_chord.chord_log.grace = Some(convert_grace(grace));
        }

        if first_note.is_cue() {
            mei_chord.chord_log.cue = Some(DataBoolean::True);
        }
    }

    // Convert each note in the chord and add as children
    for note in notes {
        let mut mei_note = convert_note(note, ctx)?;
        // Move stem direction from individual notes to chord level:
        // MusicXML puts stem on the first note only, but MEI convention stores
        // it on chord_vis.stem_dir. Clear from individual notes to avoid duplication.
        if mei_chord.chord_vis.stem_dir.is_none() {
            if let Some(stem_dir) = mei_note.note_vis.stem_dir.take() {
                mei_chord.chord_vis.stem_dir = Some(stem_dir);
            }
        } else {
            mei_note.note_vis.stem_dir = None;
        }
        mei_chord
            .children
            .push(ChordChild::Note(Box::new(mei_note)));
    }

    Ok(mei_chord)
}

// ============================================================================
// Tie Conversion
// ============================================================================

/// Convert MusicXML tie information to MEI @tie attribute.
///
/// MusicXML has both `<tie>` (sound/playback) and `<tied>` (notation/visual) elements.
/// We check both to determine the tie state. MEI uses the @tie attribute with values:
/// - "i" = initial (tie starts)
/// - "m" = medial (tie continues)
/// - "t" = terminal (tie ends)
fn convert_ties(note: &MusicXmlNote, mei_note: &mut MeiNote) {
    // Check both <tie> (sound) and <tied> (notation) elements
    let has_start = note.ties.iter().any(|t| t.tie_type == StartStop::Start)
        || note
            .notations
            .as_ref()
            .map(|n| {
                n.tied
                    .iter()
                    .any(|t| matches!(t.tied_type, TiedType::Start))
            })
            .unwrap_or(false);

    let has_stop = note.ties.iter().any(|t| t.tie_type == StartStop::Stop)
        || note
            .notations
            .as_ref()
            .map(|n| n.tied.iter().any(|t| matches!(t.tied_type, TiedType::Stop)))
            .unwrap_or(false);

    // Set @tie attribute (MEI @tie is Option<String>: "i", "m", "t")
    if has_start && has_stop {
        mei_note.note_anl.tie = Some(DataTie::from("m".to_string()));
    } else if has_start {
        mei_note.note_anl.tie = Some(DataTie::from("i".to_string()));
    } else if has_stop {
        mei_note.note_anl.tie = Some(DataTie::from("t".to_string()));
    }
}

// ============================================================================
// Articulation Conversion
// ============================================================================

/// Convert MusicXML articulations to MEI @artic attribute and note label for breath/caesura.
///
/// Maps MusicXML articulation elements (accent, staccato, tenuto, etc.)
/// to MEI DataArticulation. Breath-mark and caesura are stored in note common.label
/// (e.g. "musicxml:breath-mark", "musicxml:caesura") for roundtrip since MEI @artic
/// has no equivalent and Breath/Caesura are not note children in our schema.
fn convert_articulations(note: &MusicXmlNote, mei_note: &mut MeiNote, ctx: &mut ConversionContext) {
    if let Some(ref notations) = note.notations {
        if let Some(ref artics) = notations.articulations {
            let tokens = articulations_to_mei(artics);
            if tokens.len() > 1 {
                ctx.add_warning(
                    "artic",
                    "Multiple articulations on note; only first is stored in MEI @artic",
                );
            }
            mei_note.note_anl.artic = tokens.first().copied();

            // Store breath-mark and/or caesura in label for roundtrip
            let mut labels = Vec::<&str>::new();
            if artics.breath_mark.is_some() {
                labels.push("musicxml:breath-mark");
            }
            if artics.caesura.is_some() {
                labels.push("musicxml:caesura");
            }
            if !labels.is_empty() {
                mei_note.common.label = Some(labels.join(","));
            }
        }
    }
}

// ============================================================================
// Slur Processing
// ============================================================================

/// Process slurs from note notations.
///
/// This tracks pending slurs when a start is found, and resolves them
/// when a stop is found by adding to the context's completed slurs list.
fn process_slurs(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    if let Some(ref notations) = note.notations {
        // MusicXML staff within the part (for matching start/stop pairs)
        let staff = note.staff.unwrap_or(1);
        // MEI global staff number (for the @staff attribute on the slur)
        let mei_staff = ctx.staff().unwrap_or(1);
        // Part ID to scope slur matching within a single part
        let part_id = ctx.position().part_id.clone().unwrap_or_default();

        for slur in &notations.slurs {
            let number = slur.number.unwrap_or(1);

            match slur.slur_type {
                StartStopContinue::Start => {
                    ctx.add_pending_slur(PendingSlur {
                        start_id: note_id.to_string(),
                        part_id: part_id.clone(),
                        staff,
                        number,
                        mei_staff,
                    });
                }
                StartStopContinue::Stop => {
                    // Try to resolve a matching pending slur
                    if let Some(pending) = ctx.resolve_slur(&part_id, staff, number) {
                        ctx.add_completed_slur(
                            pending.start_id,
                            note_id.to_string(),
                            pending.mei_staff,
                        );
                    }
                }
                StartStopContinue::Continue => {
                    // Continue slurs don't need any action
                }
            }
        }
    }
}

// ============================================================================
// Tuplet Processing
// ============================================================================

/// Process tuplets from note notations and time-modification.
///
/// On tuplet start: creates a PendingTuplet with the note's time-modification ratio.
/// On tuplet stop: resolves the pending tuplet into a CompletedTuplet.
fn process_tuplets(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    if let Some(ref notations) = note.notations {
        let staff = note.staff.unwrap_or(1);
        let mei_staff = ctx.staff().unwrap_or(1);
        let part_id = ctx.position().part_id.clone().unwrap_or_default();

        for tuplet in &notations.tuplets {
            let number = tuplet.number.unwrap_or(1);

            match tuplet.tuplet_type {
                StartStop::Start => {
                    // Get num/numbase from time-modification
                    let (num, numbase) = note
                        .time_modification
                        .as_ref()
                        .map(|tm| (tm.actual_notes, tm.normal_notes))
                        .unwrap_or((3, 2));

                    let bracket = tuplet.bracket.map(|b| b == crate::model::data::YesNo::Yes);

                    ctx.add_pending_tuplet(PendingTuplet {
                        start_id: note_id.to_string(),
                        part_id: part_id.clone(),
                        staff,
                        number,
                        mei_staff,
                        num,
                        numbase,
                        bracket,
                        show_number: tuplet.show_number,
                        placement: tuplet.placement,
                    });
                }
                StartStop::Stop => {
                    if let Some(pending) = ctx.resolve_tuplet(&part_id, staff, number) {
                        ctx.add_completed_tuplet(crate::context::CompletedTuplet {
                            start_id: pending.start_id,
                            end_id: note_id.to_string(),
                            mei_staff: pending.mei_staff,
                            num: pending.num,
                            numbase: pending.numbase,
                            bracket: pending.bracket,
                            show_number: pending.show_number,
                            placement: pending.placement,
                        });
                    }
                }
            }
        }
    }
}

// ============================================================================
// Ornament Processing
// ============================================================================

/// Process ornaments from note notations.
///
/// Creates MEI control events (trill, mordent, turn, ornam) from MusicXML ornament
/// notations. These are collected in the context and emitted after all staves are
/// processed, matching the pattern used for slurs and tuplets.
///
/// Mapping:
/// - trill-mark → MEI `<trill>` with @startid, @staff, @place
/// - mordent → MEI `<mordent>` with @form="lower", @long, @startid, @staff, @place
/// - inverted-mordent → MEI `<mordent>` with @form="upper", @startid, @staff, @place
/// - turn → MEI `<turn>` with @form="upper", @startid, @staff, @place
/// - delayed-turn → MEI `<turn>` with @form="upper", @delayed="true"
/// - inverted-turn → MEI `<turn>` with @form="lower"
/// - delayed-inverted-turn → MEI `<turn>` with @form="lower", @delayed="true"
/// - vertical-turn, inverted-vertical-turn, shake, schleifer, haydn → MEI `<ornam>`
///   with musicxml: label for lossless roundtrip
/// - tremolo → MEI `<ornam>` with musicxml:tremolo label (bTrem/fTrem are containers)
/// - wavy-line → MEI `<ornam>` with musicxml:wavy-line label
/// - other-ornament → MEI `<ornam>` with musicxml:other-ornament label
fn process_ornaments(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::import::direction::convert_placement;
    use crate::model::data::AboveBelow;
    use tusk_model::data::{DataBoolean, DataStaffrel, DataUri};
    use tusk_model::elements::{
        MeasureChild, Mordent as MeiMordent, Ornam, OrnamChild, Trill, Turn,
    };

    let ornaments = match note.notations {
        Some(ref notations) => match notations.ornaments {
            Some(ref orn) => orn,
            None => return,
        },
        None => return,
    };

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    // Helper: convert placement for ornaments
    let place_for =
        |p: Option<AboveBelow>| -> Option<DataStaffrel> { convert_placement(p.as_ref()) };

    // trill-mark → MEI <trill>
    if let Some(ref trill_mark) = ornaments.trill_mark {
        let mut trill = Trill::default();
        trill.common.xml_id = Some(ctx.generate_id_with_suffix("trill"));
        trill.trill_log.startid = Some(startid.clone());
        trill.trill_log.staff = Some(staff_str.clone());
        trill.trill_vis.place = place_for(trill_mark.placement);
        ctx.add_ornament_event(MeasureChild::Trill(Box::new(trill)));
    }

    // mordent → MEI <mordent> with @form="lower"
    if let Some(ref mordent) = ornaments.mordent {
        let mut mei_mordent = MeiMordent::default();
        mei_mordent.common.xml_id = Some(ctx.generate_id_with_suffix("mordent"));
        mei_mordent.mordent_log.startid = Some(startid.clone());
        mei_mordent.mordent_log.staff = Some(staff_str.clone());
        mei_mordent.mordent_log.form = Some("lower".to_string());
        if let Some(crate::model::data::YesNo::Yes) = mordent.long {
            mei_mordent.mordent_log.long = Some(DataBoolean::True);
        }
        mei_mordent.mordent_vis.place = place_for(mordent.placement);
        ctx.add_ornament_event(MeasureChild::Mordent(Box::new(mei_mordent)));
    }

    // inverted-mordent → MEI <mordent> with @form="upper"
    if let Some(ref inv_mordent) = ornaments.inverted_mordent {
        let mut mei_mordent = MeiMordent::default();
        mei_mordent.common.xml_id = Some(ctx.generate_id_with_suffix("mordent"));
        mei_mordent.mordent_log.startid = Some(startid.clone());
        mei_mordent.mordent_log.staff = Some(staff_str.clone());
        mei_mordent.mordent_log.form = Some("upper".to_string());
        if let Some(crate::model::data::YesNo::Yes) = inv_mordent.long {
            mei_mordent.mordent_log.long = Some(DataBoolean::True);
        }
        mei_mordent.mordent_vis.place = place_for(inv_mordent.placement);
        ctx.add_ornament_event(MeasureChild::Mordent(Box::new(mei_mordent)));
    }

    // turn → MEI <turn> with @form="upper"
    if let Some(ref turn) = ornaments.turn {
        let mut mei_turn = Turn::default();
        mei_turn.common.xml_id = Some(ctx.generate_id_with_suffix("turn"));
        mei_turn.turn_log.startid = Some(startid.clone());
        mei_turn.turn_log.staff = Some(staff_str.clone());
        mei_turn.turn_log.form = Some("upper".to_string());
        mei_turn.turn_vis.place = place_for(turn.placement);
        ctx.add_ornament_event(MeasureChild::Turn(Box::new(mei_turn)));
    }

    // delayed-turn → MEI <turn> with @form="upper", @delayed="true"
    if let Some(ref delayed_turn) = ornaments.delayed_turn {
        let mut mei_turn = Turn::default();
        mei_turn.common.xml_id = Some(ctx.generate_id_with_suffix("turn"));
        mei_turn.turn_log.startid = Some(startid.clone());
        mei_turn.turn_log.staff = Some(staff_str.clone());
        mei_turn.turn_log.form = Some("upper".to_string());
        mei_turn.turn_log.delayed = Some(DataBoolean::True);
        mei_turn.turn_vis.place = place_for(delayed_turn.placement);
        ctx.add_ornament_event(MeasureChild::Turn(Box::new(mei_turn)));
    }

    // inverted-turn → MEI <turn> with @form="lower"
    if let Some(ref inv_turn) = ornaments.inverted_turn {
        let mut mei_turn = Turn::default();
        mei_turn.common.xml_id = Some(ctx.generate_id_with_suffix("turn"));
        mei_turn.turn_log.startid = Some(startid.clone());
        mei_turn.turn_log.staff = Some(staff_str.clone());
        mei_turn.turn_log.form = Some("lower".to_string());
        mei_turn.turn_vis.place = place_for(inv_turn.placement);
        ctx.add_ornament_event(MeasureChild::Turn(Box::new(mei_turn)));
    }

    // delayed-inverted-turn → MEI <turn> with @form="lower", @delayed="true"
    if let Some(ref delayed_inv_turn) = ornaments.delayed_inverted_turn {
        let mut mei_turn = Turn::default();
        mei_turn.common.xml_id = Some(ctx.generate_id_with_suffix("turn"));
        mei_turn.turn_log.startid = Some(startid.clone());
        mei_turn.turn_log.staff = Some(staff_str.clone());
        mei_turn.turn_log.form = Some("lower".to_string());
        mei_turn.turn_log.delayed = Some(DataBoolean::True);
        mei_turn.turn_vis.place = place_for(delayed_inv_turn.placement);
        ctx.add_ornament_event(MeasureChild::Turn(Box::new(mei_turn)));
    }

    // vertical-turn → MEI <ornam> with label for roundtrip
    if let Some(ref vt) = ornaments.vertical_turn {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        ornam.common.label = Some("musicxml:vertical-turn".to_string());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(vt.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // inverted-vertical-turn → MEI <ornam> with label
    if let Some(ref ivt) = ornaments.inverted_vertical_turn {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        ornam.common.label = Some("musicxml:inverted-vertical-turn".to_string());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(ivt.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // shake → MEI <ornam> with label
    if let Some(ref shake) = ornaments.shake {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        ornam.common.label = Some("musicxml:shake".to_string());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(shake.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // schleifer → MEI <ornam> with label
    if ornaments.schleifer.is_some() {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        ornam.common.label = Some("musicxml:schleifer".to_string());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // tremolo → MEI <ornam> with label encoding type and value
    if let Some(ref tremolo) = ornaments.tremolo {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        let type_str = match tremolo.tremolo_type {
            crate::model::data::TremoloType::Single => "single",
            crate::model::data::TremoloType::Start => "start",
            crate::model::data::TremoloType::Stop => "stop",
            crate::model::data::TremoloType::Unmeasured => "unmeasured",
        };
        let value_str = tremolo.value.unwrap_or(0).to_string();
        ornam.common.label = Some(format!(
            "musicxml:tremolo,type={},value={}",
            type_str, value_str
        ));
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(tremolo.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // haydn → MEI <ornam> with label
    if let Some(ref haydn) = ornaments.haydn {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        ornam.common.label = Some("musicxml:haydn".to_string());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(haydn.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // wavy-line → MEI <ornam> with label encoding type and number
    if let Some(ref wavy) = ornaments.wavy_line {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        let type_str = match wavy.wavy_line_type {
            StartStopContinue::Start => "start",
            StartStopContinue::Stop => "stop",
            StartStopContinue::Continue => "continue",
        };
        let number_str = wavy.number.unwrap_or(1).to_string();
        ornam.common.label = Some(format!(
            "musicxml:wavy-line,type={},number={}",
            type_str, number_str
        ));
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(wavy.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // other-ornament → MEI <ornam> with label and text content
    if let Some(ref other) = ornaments.other_ornament {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        ornam.common.label = Some("musicxml:other-ornament".to_string());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(other.placement);
        if !other.value.is_empty() {
            ornam.children.push(OrnamChild::Text(other.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // accidental-mark within ornaments → skip for now (rare, complex mapping)
}

// ============================================================================
// Fermata Processing
// ============================================================================

/// Process fermata notations into MEI `<fermata>` control events.
fn process_fermatas(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::model::data::UprightInverted;
    use crate::model::notations::FermataShape;
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic, DataUri};
    use tusk_model::elements::{Fermata as MeiFermata, MeasureChild};

    let fermatas = match note.notations {
        Some(ref n) => &n.fermatas,
        None => return,
    };
    if fermatas.is_empty() {
        return;
    }

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    for fermata in fermatas {
        let mut f = MeiFermata::default();
        f.common.xml_id = Some(ctx.generate_id_with_suffix("fermata"));
        f.fermata_log.startid = Some(startid.clone());
        f.fermata_log.staff = Some(staff_str.clone());

        // Map MusicXML type → MEI @place (upright=above, inverted=below)
        f.fermata_vis.place = match fermata.fermata_type {
            Some(UprightInverted::Upright) => {
                Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above))
            }
            Some(UprightInverted::Inverted) => {
                Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Below))
            }
            None => None,
        };

        // Map MusicXML shape → MEI @shape
        f.fermata_vis.shape = fermata.shape.as_ref().and_then(|s| match s {
            FermataShape::Normal | FermataShape::Empty => None,
            FermataShape::Angled => Some("angular".to_string()),
            FermataShape::Square => Some("square".to_string()),
            FermataShape::DoubleAngled => Some("double-angular".to_string()),
            FermataShape::DoubleSquare => Some("double-square".to_string()),
            FermataShape::DoubleDot => Some("double-dot".to_string()),
            FermataShape::HalfCurve => Some("half-curve".to_string()),
            FermataShape::Curlew => Some("curlew".to_string()),
        });

        // Map MusicXML type → MEI @form (inv → inv, upright → default)
        f.fermata_vis.form = match fermata.fermata_type {
            Some(UprightInverted::Inverted) => Some("inv".to_string()),
            _ => None,
        };

        ctx.add_ornament_event(MeasureChild::Fermata(Box::new(f)));
    }
}

// ============================================================================
// Arpeggiate Processing
// ============================================================================

/// Process arpeggiate/non-arpeggiate notations into MEI `<arpeg>` control events.
fn process_arpeggiate(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::model::data::UpDown;
    use tusk_model::data::DataUri;
    use tusk_model::elements::{Arpeg, MeasureChild};

    let notations = match note.notations {
        Some(ref n) => n,
        None => return,
    };

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    // arpeggiate → MEI <arpeg>
    if let Some(ref arp) = notations.arpeggiate {
        let mut a = Arpeg::default();
        a.common.xml_id = Some(ctx.generate_id_with_suffix("arpeg"));
        a.arpeg_log.startid = Some(startid.clone());
        a.arpeg_log.staff = Some(staff_str.clone());
        a.arpeg_log.order = arp.direction.as_ref().map(|d| match d {
            UpDown::Up => "up".to_string(),
            UpDown::Down => "down".to_string(),
        });
        ctx.add_ornament_event(MeasureChild::Arpeg(Box::new(a)));
    }

    // non-arpeggiate → MEI <arpeg> with @order="nonarp"
    if let Some(ref _nonarp) = notations.non_arpeggiate {
        let mut a = Arpeg::default();
        a.common.xml_id = Some(ctx.generate_id_with_suffix("arpeg"));
        a.arpeg_log.startid = Some(startid.clone());
        a.arpeg_log.staff = Some(staff_str.clone());
        a.arpeg_log.order = Some("nonarp".to_string());
        a.common.label = Some("musicxml:non-arpeggiate".to_string());
        ctx.add_ornament_event(MeasureChild::Arpeg(Box::new(a)));
    }
}

// ============================================================================
// Glissando/Slide Processing
// ============================================================================

/// Process glissando and slide notations into MEI `<gliss>` control events.
///
/// Glissandos and slides are start/stop spanners. On start, a PendingGliss is created.
/// On stop, the pending gliss is resolved into a completed gliss with both startid and endid.
fn process_glissandos(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::context::PendingGliss;
    use crate::model::data::LineType;

    let notations = match note.notations {
        Some(ref n) => n,
        None => return,
    };

    let staff = note.staff.unwrap_or(1);
    let mei_staff = ctx.staff().unwrap_or(1);
    let part_id = ctx.position().part_id.clone().unwrap_or_default();

    let line_type_str = |lt: &LineType| match lt {
        LineType::Solid => "solid",
        LineType::Dashed => "dashed",
        LineType::Dotted => "dotted",
        LineType::Wavy => "wavy",
    };

    // glissando elements
    for gliss in &notations.glissandos {
        let number = gliss.number.unwrap_or(1);
        match gliss.glissando_type {
            StartStop::Start => {
                ctx.add_pending_gliss(PendingGliss {
                    start_id: note_id.to_string(),
                    part_id: part_id.clone(),
                    staff,
                    number,
                    mei_staff,
                    line_type: gliss
                        .line_type
                        .as_ref()
                        .map(|lt| line_type_str(lt).to_string()),
                    text: gliss.text.clone(),
                    label: None,
                });
            }
            StartStop::Stop => {
                if let Some(pending) = ctx.resolve_gliss(&part_id, staff, number) {
                    ctx.add_completed_gliss(crate::context::CompletedGliss {
                        start_id: pending.start_id,
                        end_id: note_id.to_string(),
                        mei_staff: pending.mei_staff,
                        line_type: pending.line_type,
                        text: pending.text,
                        label: None,
                    });
                }
            }
        }
    }

    // slide elements → same as glissando but with label for roundtrip
    for slide in &notations.slides {
        let number = slide.number.unwrap_or(1);
        match slide.slide_type {
            StartStop::Start => {
                ctx.add_pending_gliss(PendingGliss {
                    start_id: note_id.to_string(),
                    part_id: part_id.clone(),
                    staff,
                    number,
                    mei_staff,
                    line_type: slide
                        .line_type
                        .as_ref()
                        .map(|lt| line_type_str(lt).to_string()),
                    text: slide.text.clone(),
                    label: Some("musicxml:slide".to_string()),
                });
            }
            StartStop::Stop => {
                if let Some(pending) = ctx.resolve_gliss(&part_id, staff, number) {
                    ctx.add_completed_gliss(crate::context::CompletedGliss {
                        start_id: pending.start_id,
                        end_id: note_id.to_string(),
                        mei_staff: pending.mei_staff,
                        line_type: pending.line_type,
                        text: pending.text,
                        label: pending.label,
                    });
                }
            }
        }
    }
}

// ============================================================================
// Standalone Accidental-Mark Processing
// ============================================================================

/// Process standalone accidental-mark notations (outside ornaments).
///
/// Maps to MEI `<ornam>` with label for roundtrip fidelity.
fn process_accidental_marks(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{MeasureChild, Ornam, OrnamChild};

    let notations = match note.notations {
        Some(ref n) => n,
        None => return,
    };
    if notations.accidental_marks.is_empty() {
        return;
    }

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    for am in &notations.accidental_marks {
        let mut ornam = Ornam::default();
        ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
        ornam.common.label = Some(format!("musicxml:accidental-mark,value={}", am.value));
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        if let Some(ref placement) = am.placement {
            use crate::import::direction::convert_placement;
            ornam.ornam_vis.place = convert_placement(Some(placement));
        }
        if !am.value.is_empty() {
            ornam.children.push(OrnamChild::Text(am.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }
}

/// Process technical notations into MEI `<ornam>` control events with `musicxml:` labels.
fn process_technical(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::import::direction::convert_placement;
    use crate::model::data::AboveBelow;
    use crate::model::technical::*;
    use tusk_model::data::DataUri;
    use tusk_model::elements::{MeasureChild, Ornam, OrnamChild};

    let tech = match note.notations {
        Some(ref n) => match n.technical {
            Some(ref t) => t,
            None => return,
        },
        None => return,
    };

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    let place_for = |p: Option<AboveBelow>| convert_placement(p.as_ref());

    // Helper macro to create an ornam with label and add it
    macro_rules! emit {
        ($label:expr, $placement:expr) => {{
            let mut ornam = Ornam::default();
            ornam.common.xml_id = Some(ctx.generate_id_with_suffix("ornam"));
            ornam.common.label = Some($label);
            ornam.ornam_log.startid = Some(startid.clone());
            ornam.ornam_log.staff = Some(staff_str.clone());
            ornam.ornam_vis.place = place_for($placement);
            ornam
        }};
    }

    // Simple empty-placement types
    for v in &tech.up_bow {
        let o = emit!("musicxml:up-bow".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.down_bow {
        let o = emit!("musicxml:down-bow".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.open_string {
        let o = emit!("musicxml:open-string".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.thumb_position {
        let o = emit!("musicxml:thumb-position".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.double_tongue {
        let o = emit!("musicxml:double-tongue".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.triple_tongue {
        let o = emit!("musicxml:triple-tongue".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.snap_pizzicato {
        let o = emit!("musicxml:snap-pizzicato".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.fingernails {
        let o = emit!("musicxml:fingernails".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.brass_bend {
        let o = emit!("musicxml:brass-bend".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.flip {
        let o = emit!("musicxml:flip".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.smear {
        let o = emit!("musicxml:smear".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.golpe {
        let o = emit!("musicxml:golpe".into(), v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Empty-placement-smufl types (stopped, open, half-muted)
    for v in &tech.stopped {
        let mut label = "musicxml:stopped".to_string();
        if let Some(ref s) = v.smufl {
            label.push_str(&format!(",smufl={}", s));
        }
        let o = emit!(label, v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.open {
        let mut label = "musicxml:open".to_string();
        if let Some(ref s) = v.smufl {
            label.push_str(&format!(",smufl={}", s));
        }
        let o = emit!(label, v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.half_muted {
        let mut label = "musicxml:half-muted".to_string();
        if let Some(ref s) = v.smufl {
            label.push_str(&format!(",smufl={}", s));
        }
        let o = emit!(label, v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Fingering (text content + substitution/alternate attrs)
    for v in &tech.fingering {
        let mut label = "musicxml:fingering".to_string();
        if matches!(v.substitution, Some(crate::model::data::YesNo::Yes)) {
            label.push_str(",substitution=yes");
        }
        if matches!(v.alternate, Some(crate::model::data::YesNo::Yes)) {
            label.push_str(",alternate=yes");
        }
        let mut o = emit!(label, v.placement);
        if !v.value.is_empty() {
            o.children.push(OrnamChild::Text(v.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Pluck (text content)
    for v in &tech.pluck {
        let mut o = emit!("musicxml:pluck".into(), v.placement);
        if !v.value.is_empty() {
            o.children.push(OrnamChild::Text(v.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Fret (numeric text)
    for v in &tech.fret {
        let mut o = emit!("musicxml:fret".into(), None);
        o.children.push(OrnamChild::Text(v.value.to_string()));
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // String (numeric text)
    for v in &tech.string {
        let mut o = emit!("musicxml:string".into(), v.placement);
        o.children.push(OrnamChild::Text(v.value.to_string()));
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Hammer-on / Pull-off (type + number + text)
    for v in &tech.hammer_on {
        let type_str = match v.ho_type {
            crate::model::data::StartStop::Start => "start",
            crate::model::data::StartStop::Stop => "stop",
        };
        let mut label = format!("musicxml:hammer-on,type={}", type_str);
        if let Some(n) = v.number {
            label.push_str(&format!(",number={}", n));
        }
        let mut o = emit!(label, v.placement);
        if !v.text.is_empty() {
            o.children.push(OrnamChild::Text(v.text.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.pull_off {
        let type_str = match v.ho_type {
            crate::model::data::StartStop::Start => "start",
            crate::model::data::StartStop::Stop => "stop",
        };
        let mut label = format!("musicxml:pull-off,type={}", type_str);
        if let Some(n) = v.number {
            label.push_str(&format!(",number={}", n));
        }
        let mut o = emit!(label, v.placement);
        if !v.text.is_empty() {
            o.children.push(OrnamChild::Text(v.text.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Tap (text + hand)
    for v in &tech.tap {
        let mut label = "musicxml:tap".to_string();
        if let Some(ref h) = v.hand {
            let h_str = match h {
                TapHand::Left => "left",
                TapHand::Right => "right",
            };
            label.push_str(&format!(",hand={}", h_str));
        }
        let mut o = emit!(label, v.placement);
        if !v.value.is_empty() {
            o.children.push(OrnamChild::Text(v.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Heel / Toe (substitution)
    for v in &tech.heel {
        let mut label = "musicxml:heel".to_string();
        if matches!(v.substitution, Some(crate::model::data::YesNo::Yes)) {
            label.push_str(",substitution=yes");
        }
        let o = emit!(label, v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.toe {
        let mut label = "musicxml:toe".to_string();
        if matches!(v.substitution, Some(crate::model::data::YesNo::Yes)) {
            label.push_str(",substitution=yes");
        }
        let o = emit!(label, v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Bend (alter + pre-bend/release/with-bar + shape)
    for v in &tech.bend {
        let mut label = format!("musicxml:bend,alter={}", v.bend_alter);
        if v.pre_bend.is_some() {
            label.push_str(",pre-bend");
        }
        if let Some(ref rel) = v.release {
            label.push_str(",release");
            if let Some(offset) = rel.offset {
                label.push_str(&format!("={}", offset));
            }
        }
        if let Some(ref shape) = v.shape {
            let s = match shape {
                BendShape::Straight => "straight",
                BendShape::Curved => "curved",
            };
            label.push_str(&format!(",shape={}", s));
        }
        let mut o = emit!(label, None);
        if let Some(ref wb) = v.with_bar {
            if !wb.value.is_empty() {
                o.children.push(OrnamChild::Text(wb.value.clone()));
            }
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Hole (hole-type, hole-closed, hole-shape)
    for v in &tech.hole {
        let closed_str = match v.hole_closed.value {
            HoleClosedValue::Yes => "yes",
            HoleClosedValue::No => "no",
            HoleClosedValue::Half => "half",
        };
        let mut label = format!("musicxml:hole,closed={}", closed_str);
        if let Some(ref loc) = v.hole_closed.location {
            let loc_str = match loc {
                HoleClosedLocation::Right => "right",
                HoleClosedLocation::Bottom => "bottom",
                HoleClosedLocation::Left => "left",
                HoleClosedLocation::Top => "top",
            };
            label.push_str(&format!(",location={}", loc_str));
        }
        if let Some(ref ht) = v.hole_type {
            label.push_str(&format!(",hole-type={}", ht));
        }
        if let Some(ref hs) = v.hole_shape {
            label.push_str(&format!(",hole-shape={}", hs));
        }
        let o = emit!(label, v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Arrow (directional or circular)
    for v in &tech.arrow {
        match &v.content {
            ArrowContent::Directional {
                direction,
                style,
                arrowhead,
            } => {
                let mut label = format!("musicxml:arrow,direction={}", direction);
                if let Some(s) = style {
                    label.push_str(&format!(",style={}", s));
                }
                if *arrowhead {
                    label.push_str(",arrowhead");
                }
                let o = emit!(label, v.placement);
                ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
            }
            ArrowContent::Circular(value) => {
                let label = format!("musicxml:arrow,circular={}", value);
                let o = emit!(label, v.placement);
                ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
            }
        }
    }

    // Handbell (text value)
    for v in &tech.handbell {
        let mut o = emit!("musicxml:handbell".into(), v.placement);
        o.children.push(OrnamChild::Text(v.value.clone()));
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Harmon mute (harmon-closed value + location)
    for v in &tech.harmon_mute {
        let closed_str = match v.harmon_closed.value {
            HarmonClosedValue::Yes => "yes",
            HarmonClosedValue::No => "no",
            HarmonClosedValue::Half => "half",
        };
        let mut label = format!("musicxml:harmon-mute,closed={}", closed_str);
        if let Some(ref loc) = v.harmon_closed.location {
            let loc_str = match loc {
                HarmonClosedLocation::Right => "right",
                HarmonClosedLocation::Bottom => "bottom",
                HarmonClosedLocation::Left => "left",
                HarmonClosedLocation::Top => "top",
            };
            label.push_str(&format!(",location={}", loc_str));
        }
        let o = emit!(label, v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Harmonic (natural/artificial/base-pitch/touching-pitch/sounding-pitch)
    for v in &tech.harmonic {
        let mut parts = Vec::new();
        if v.natural.is_some() {
            parts.push("natural");
        }
        if v.artificial.is_some() {
            parts.push("artificial");
        }
        if v.base_pitch.is_some() {
            parts.push("base-pitch");
        }
        if v.touching_pitch.is_some() {
            parts.push("touching-pitch");
        }
        if v.sounding_pitch.is_some() {
            parts.push("sounding-pitch");
        }
        let label = if parts.is_empty() {
            "musicxml:harmonic".to_string()
        } else {
            format!("musicxml:harmonic,{}", parts.join(","))
        };
        let o = emit!(label, v.placement);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Other-technical (text + smufl)
    for v in &tech.other_technical {
        let mut label = "musicxml:other-technical".to_string();
        if let Some(ref s) = v.smufl {
            label.push_str(&format!(",smufl={}", s));
        }
        let mut o = emit!(label, v.placement);
        if !v.value.is_empty() {
            o.children.push(OrnamChild::Text(v.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
}

/// Convert a MusicXML Articulations struct to MEI DataArticulation values.
fn articulations_to_mei(artics: &Articulations) -> Vec<DataArticulation> {
    let mut result = Vec::new();

    if artics.accent.is_some() {
        result.push(DataArticulation::Acc);
    }
    if artics.strong_accent.is_some() {
        result.push(DataArticulation::Marc);
    }
    if artics.staccato.is_some() {
        result.push(DataArticulation::Stacc);
    }
    if artics.tenuto.is_some() {
        result.push(DataArticulation::Ten);
    }
    if artics.detached_legato.is_some() {
        // Detached-legato is tenuto + staccato combined
        result.push(DataArticulation::Ten);
        result.push(DataArticulation::Stacc);
    }
    if artics.staccatissimo.is_some() {
        result.push(DataArticulation::Stacciss);
    }
    if artics.spiccato.is_some() {
        result.push(DataArticulation::Spicc);
    }
    if artics.scoop.is_some() {
        result.push(DataArticulation::Scoop);
    }
    if artics.plop.is_some() {
        result.push(DataArticulation::Plop);
    }
    if artics.doit.is_some() {
        result.push(DataArticulation::Doit);
    }
    if artics.falloff.is_some() {
        result.push(DataArticulation::Fall);
    }
    if artics.stress.is_some() {
        result.push(DataArticulation::Stress);
    }
    if artics.unstress.is_some() {
        result.push(DataArticulation::Unstress);
    }
    if artics.soft_accent.is_some() {
        result.push(DataArticulation::AccSoft);
    }
    // breath_mark and caesura are stored via note common.label in convert_articulations

    result
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
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_log.pname,
            Some(DataPitchname::from("c".to_string()))
        );
    }

    #[test]
    fn convert_note_sets_octave() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::new(Step::G, 5), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.oct, Some(DataOctave::from(5)));
    }

    #[test]
    fn convert_note_with_sharp_alter() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};
        use tusk_model::data::{DataAccidentalGestural, DataAccidentalGesturalBasic};

        let note = Note::pitched(Pitch::with_alter(Step::F, 1.0, 4), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_log.pname,
            Some(DataPitchname::from("f".to_string()))
        );
        assert_eq!(
            mei_note.note_ges.accid_ges,
            Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
                DataAccidentalGesturalBasic::S
            ))
        );
    }

    #[test]
    fn convert_note_with_flat_alter() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};
        use tusk_model::data::{DataAccidentalGestural, DataAccidentalGesturalBasic};

        let note = Note::pitched(Pitch::with_alter(Step::B, -1.0, 4), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_log.pname,
            Some(DataPitchname::from("b".to_string()))
        );
        assert_eq!(
            mei_note.note_ges.accid_ges,
            Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
                DataAccidentalGesturalBasic::F
            ))
        );
    }

    #[test]
    fn convert_note_with_duration() {
        use crate::model::data::Step;
        use crate::model::note::{Note, NoteType, NoteTypeValue, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_log.dur,
            Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_note_with_dots() {
        use crate::model::data::Step;
        use crate::model::note::{Dot, Note, NoteType, NoteTypeValue, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::D, 4), 6.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        note.dots.push(Dot::default()); // One dot

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_log.dur,
            Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4))
        );
        assert_eq!(mei_note.note_log.dots, Some(DataAugmentdot::from(1u64)));
    }

    #[test]
    fn convert_note_infers_duration_from_divisions() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};

        // No note_type, but duration is set
        let note = Note::pitched(Pitch::new(Step::A, 4), 4.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0); // 4 divisions = quarter note

        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        // Should infer quarter note from duration=4 with divisions=4
        assert_eq!(
            mei_note.note_log.dur,
            Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_note_stores_gestural_duration() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::new(Step::C, 4), 96.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        // Should store duration in ppq
        assert_eq!(mei_note.note_ges.dur_ppq.as_deref(), Some("96"));
    }

    #[test]
    fn convert_grace_note_unaccented() {
        use crate::model::data::{Step, YesNo};
        use crate::model::note::{Grace, Note, Pitch};

        let mut grace = Grace::default();
        grace.slash = Some(YesNo::Yes); // Slashed grace note

        let note = Note::grace_note(Pitch::new(Step::D, 5), grace);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.grace, Some(DataGrace::Unacc));
    }

    #[test]
    fn convert_grace_note_accented() {
        use crate::model::data::Step;
        use crate::model::note::{Grace, Note, Pitch};

        // No slash = accented grace note
        let note = Note::grace_note(Pitch::new(Step::E, 4), Grace::default());

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.grace, Some(DataGrace::Acc));
    }

    #[test]
    fn convert_note_with_written_accidental_sharp() {
        use crate::model::data::Step;
        use crate::model::note::{Accidental, AccidentalValue, Note, Pitch};

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
                Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
                    DataAccidentalWrittenBasic::S
                ))
            );
        }
    }

    #[test]
    fn convert_note_with_written_accidental_flat() {
        use crate::model::data::Step;
        use crate::model::note::{Accidental, AccidentalValue, Note, Pitch};

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
                Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
                    DataAccidentalWrittenBasic::F
                ))
            );
        }
    }

    #[test]
    fn convert_note_with_cautionary_accidental() {
        use crate::model::data::{Step, YesNo};
        use crate::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::C, 4), 4.0);
        let mut accidental = Accidental::new(AccidentalValue::Natural);
        accidental.cautionary = Some(YesNo::Yes);
        note.accidental = Some(accidental);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(accid.accid_log.func.as_deref(), Some("caution"));
        } else {
            panic!("Expected accid child");
        }
    }

    #[test]
    fn convert_note_with_editorial_accidental() {
        use crate::model::data::{Step, YesNo};
        use crate::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::G, 4), 4.0);
        let mut accidental = Accidental::new(AccidentalValue::Sharp);
        accidental.editorial = Some(YesNo::Yes);
        note.accidental = Some(accidental);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(accid.accid_log.func.as_deref(), Some("edit"));
        } else {
            panic!("Expected accid child");
        }
    }

    #[test]
    fn convert_note_with_parentheses_accidental() {
        use crate::model::data::{Step, YesNo};
        use crate::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::A, 4), 4.0);
        let mut accidental = Accidental::new(AccidentalValue::Natural);
        accidental.parentheses = Some(YesNo::Yes);
        note.accidental = Some(accidental);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(accid.accid_vis.enclose, Some(DataEnclosure::Paren));
        } else {
            panic!("Expected accid child");
        }
    }

    #[test]
    fn convert_note_with_stem_up() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch, Stem, StemValue};

        let mut note = Note::pitched(Pitch::new(Step::E, 4), 4.0);
        note.stem = Some(Stem::new(StemValue::Up));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_vis.stem_dir,
            Some(DataStemdirection::MeiDataStemdirectionBasic(
                DataStemdirectionBasic::Up
            ))
        );
    }

    #[test]
    fn convert_note_with_stem_down() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch, Stem, StemValue};

        let mut note = Note::pitched(Pitch::new(Step::A, 5), 4.0);
        note.stem = Some(Stem::new(StemValue::Down));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(
            mei_note.note_vis.stem_dir,
            Some(DataStemdirection::MeiDataStemdirectionBasic(
                DataStemdirectionBasic::Down
            ))
        );
    }

    #[test]
    fn convert_cue_note() {
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Pitch};

        let mut note = Note::pitched(Pitch::new(Step::C, 5), 4.0);
        note.cue = Some(Empty);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_note.note_log.cue, Some(DataBoolean::True));
    }

    #[test]
    fn convert_note_generates_xml_id() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};

        let note = Note::pitched(Pitch::new(Step::D, 4), 4.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        assert!(mei_note.common.xml_id.is_some());
        assert!(mei_note.common.xml_id.as_ref().unwrap().contains("note"));
    }

    #[test]
    fn convert_note_maps_original_id() {
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};

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
        use crate::model::data::Step;
        use crate::model::note::{Note, Pitch};

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
                mei_note.note_log.pname,
                Some(DataPitchname::from(expected.to_string())),
                "Failed for step {:?}",
                step
            );
        }
    }

    #[test]
    fn convert_note_various_durations() {
        use crate::model::data::Step;
        use crate::model::note::{Note, NoteType, NoteTypeValue, Pitch};
        use tusk_model::data::DataDurationCmn;

        let dur_map: &[(_, DataDurationCmn)] = &[
            (NoteTypeValue::Whole, DataDurationCmn::N1),
            (NoteTypeValue::Half, DataDurationCmn::N2),
            (NoteTypeValue::Quarter, DataDurationCmn::N4),
            (NoteTypeValue::Eighth, DataDurationCmn::N8),
            (NoteTypeValue::N16th, DataDurationCmn::N16),
            (NoteTypeValue::N32nd, DataDurationCmn::N32),
            (NoteTypeValue::N64th, DataDurationCmn::N64),
        ];
        for (mxml_dur, expected_cmn) in dur_map {
            let mut note = Note::pitched(Pitch::new(Step::C, 4), 4.0);
            note.note_type = Some(NoteType::new(*mxml_dur));

            let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
            let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

            assert_eq!(
                mei_note.note_log.dur,
                Some(DataDuration::MeiDataDurationCmn(*expected_cmn)),
                "Failed for duration {:?}",
                mxml_dur
            );
        }
    }

    #[test]
    fn convert_note_double_sharp_accidental() {
        use crate::model::data::Step;
        use crate::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::with_alter(Step::F, 2.0, 4), 4.0);
        note.accidental = Some(Accidental::new(AccidentalValue::DoubleSharp));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(
                accid.accid_log.accid,
                Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
                    DataAccidentalWrittenBasic::X
                ))
            );
        } else {
            panic!("Expected accid child");
        }
    }

    #[test]
    fn convert_note_double_flat_accidental() {
        use crate::model::data::Step;
        use crate::model::note::{Accidental, AccidentalValue, Note, Pitch};

        let mut note = Note::pitched(Pitch::with_alter(Step::B, -2.0, 4), 4.0);
        note.accidental = Some(Accidental::new(AccidentalValue::FlatFlat));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        let mei_note = convert_note(&note, &mut ctx).expect("conversion should succeed");

        if let Some(NoteChild::Accid(accid)) = mei_note.children.first() {
            assert_eq!(
                accid.accid_log.accid,
                Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
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
        use crate::model::note::{Note, Rest};

        let note = Note::rest(Rest::new(), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        // Rest should have an xml:id
        assert!(mei_rest.common.xml_id.is_some());
    }

    #[test]
    fn convert_rest_with_duration() {
        use crate::model::note::{Note, NoteType, NoteTypeValue, Rest};

        let mut note = Note::rest(Rest::new(), 4.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(
            mei_rest.rest_log.dur,
            Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_rest_with_dots() {
        use crate::model::note::{Dot, Note, NoteType, NoteTypeValue, Rest};

        let mut note = Note::rest(Rest::new(), 6.0);
        note.note_type = Some(NoteType::new(NoteTypeValue::Quarter));
        note.dots.push(Dot::default());

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_rest.rest_log.dots, Some(DataAugmentdot::from(1u64)));
    }

    #[test]
    fn convert_rest_without_type_omits_dur() {
        use crate::model::note::{Note, Rest};

        // A rest with duration but no explicit <type> should NOT get @dur
        // (whole-measure rests rely on dur_ppq only)
        let note = Note::rest(Rest::new(), 4.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);
        ctx.set_divisions(4.0);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_rest.rest_log.dur, None);
    }

    #[test]
    fn convert_rest_stores_gestural_duration() {
        use crate::model::note::{Note, Rest};

        let note = Note::rest(Rest::new(), 8.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_rest.rest_ges.dur_ppq.as_deref(), Some("8"));
    }

    #[test]
    fn convert_rest_generates_xml_id() {
        use crate::model::note::{Note, Rest};

        let note = Note::rest(Rest::new(), 4.0);
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert!(mei_rest.common.xml_id.is_some());
        assert!(mei_rest.common.xml_id.as_ref().unwrap().contains("rest"));
    }

    #[test]
    fn convert_rest_maps_original_id() {
        use crate::model::note::{Note, Rest};

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
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Rest};

        let mut note = Note::rest(Rest::new(), 4.0);
        note.cue = Some(Empty);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_rest.rest_log.cue, Some(DataBoolean::True));
    }

    #[test]
    fn convert_rest_various_durations() {
        use crate::model::note::{Note, NoteType, NoteTypeValue, Rest};
        use tusk_model::data::DataDurationCmn;

        let test_cases: &[(NoteTypeValue, DataDurationCmn)] = &[
            (NoteTypeValue::Whole, DataDurationCmn::N1),
            (NoteTypeValue::Half, DataDurationCmn::N2),
            (NoteTypeValue::Quarter, DataDurationCmn::N4),
            (NoteTypeValue::Eighth, DataDurationCmn::N8),
            (NoteTypeValue::N16th, DataDurationCmn::N16),
        ];
        for (mxml_type, expected_cmn) in test_cases {
            let mut note = Note::rest(Rest::new(), 4.0);
            note.note_type = Some(NoteType::new(*mxml_type));

            let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

            let mei_rest = convert_rest(&note, &mut ctx).expect("conversion should succeed");
            assert_eq!(
                mei_rest.rest_log.dur,
                Some(DataDurationrests::MeiDataDurationCmn(*expected_cmn)),
                "Failed for {:?}",
                mxml_type
            );
        }
    }

    #[test]
    fn convert_measure_rest_creates_mrest() {
        use crate::model::data::YesNo;
        use crate::model::note::{Note, Rest};

        let mut rest = Rest::new();
        rest.measure = Some(YesNo::Yes);
        let note = Note::rest(rest, 16.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_mrest = convert_measure_rest(&note, &mut ctx).expect("conversion should succeed");
        assert!(mei_mrest.common.xml_id.is_some());
    }

    #[test]
    fn convert_measure_rest_generates_xml_id() {
        use crate::model::data::YesNo;
        use crate::model::note::{Note, Rest};

        let mut rest = Rest::new();
        rest.measure = Some(YesNo::Yes);
        let note = Note::rest(rest, 16.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_mrest = convert_measure_rest(&note, &mut ctx).expect("conversion should succeed");
        assert!(mei_mrest.common.xml_id.as_ref().unwrap().contains("mrest"));
    }

    #[test]
    fn convert_measure_rest_stores_gestural_duration() {
        use crate::model::data::YesNo;
        use crate::model::note::{Note, Rest};

        let mut rest = Rest::new();
        rest.measure = Some(YesNo::Yes);
        let note = Note::rest(rest, 16.0);

        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_mrest = convert_measure_rest(&note, &mut ctx).expect("conversion should succeed");
        assert_eq!(mei_mrest.m_rest_ges.dur_ppq.as_deref(), Some("16"));
    }

    #[test]
    fn convert_cue_measure_rest() {
        use crate::model::data::YesNo;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Rest};

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
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Pitch};

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
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Pitch};

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
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, NoteType, NoteTypeValue, Pitch};

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
            Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4))
        );
    }

    #[test]
    fn convert_chord_sets_dots() {
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Dot, Note, NoteType, NoteTypeValue, Pitch};

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

        assert_eq!(mei_chord.chord_log.dots, Some(DataAugmentdot::from(1u64)));
    }

    #[test]
    fn convert_chord_stores_gestural_duration() {
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Pitch};

        let note1 = Note::pitched(Pitch::new(Step::A, 3), 96.0);
        let mut note2 = Note::pitched(Pitch::new(Step::C, 4), 96.0);
        note2.chord = Some(Empty);

        let notes = vec![note1, note2];
        let mut ctx = ConversionContext::new(ConversionDirection::MusicXmlToMei);

        let mei_chord = convert_chord(&notes, &mut ctx).expect("conversion should succeed");

        assert_eq!(mei_chord.chord_ges.dur_ppq.as_deref(), Some("96"));
    }

    #[test]
    fn convert_chord_generates_xml_id() {
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Pitch};

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
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Pitch};

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
            .filter_map(|c| {
                let ChordChild::Note(n) = c;
                n.note_log.pname.as_ref().map(|p| p.0.as_str())
            })
            .collect();

        assert_eq!(pitches, vec!["c", "e", "g"]);
    }

    #[test]
    fn convert_chord_with_accidentals() {
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Note, Pitch};

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
            .filter_map(|c| {
                let ChordChild::Note(n) = c;
                Some(n.as_ref())
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
        use crate::model::data::Step;
        use crate::model::elements::Empty;
        use crate::model::note::{Grace, Note, Pitch};

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

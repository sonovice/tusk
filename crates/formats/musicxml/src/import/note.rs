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

    // Set xml:id — preserve original MusicXML ID if present, else generate
    let note_id = match note.id {
        Some(ref orig_id) => {
            let id = orig_id.clone();
            ctx.map_id(orig_id, &id);
            id
        }
        None => ctx.generate_id_with_suffix("note"),
    };
    mei_note.common.xml_id = Some(note_id);

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
        use crate::model::note::StemValue;
        match stem.value {
            StemValue::Double | StemValue::None => {
                // MEI has no double/none stem — stored in ExtensionStore via populate_note_ext_store
            }
            _ => {
                mei_note.note_vis.stem_dir = Some(convert_stem_direction(stem.value));
            }
        }
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

    // Process breath marks → MEI <breath> control events
    process_breath_marks(note, &note_id, ctx);

    // Process caesuras → MEI <caesura> control events
    process_caesuras(note, &note_id, ctx);

    // Process arpeggiate/non-arpeggiate
    process_arpeggiate(note, &note_id, ctx);

    // Process glissandos/slides
    process_glissandos(note, &note_id, ctx);

    // Process standalone accidental marks
    process_accidental_marks(note, &note_id, ctx);

    // Process other-notation elements
    process_other_notations(note, &note_id, ctx);

    // Process notation-level dynamics
    process_notation_dynamics(note, &note_id, ctx);

    // Process technical notations
    process_technical(note, &note_id, &mut mei_note, ctx);

    // Convert lyrics to MEI verse/syl children
    convert_lyrics(note, &mut mei_note);

    // Store notehead in MEI head_shape/head_fill; full data in ExtensionStore via populate_note_ext_store
    if let Some(ref nh) = note.notehead {
        convert_notehead_to_mei(nh, &mut mei_note);
    }

    // Import visual/position/print attributes (MEI @color, @visible)
    convert_note_visual_attrs(note, &mut mei_note);

    // Store all note-level extension data in ExtensionStore (typed, not labels)
    populate_note_ext_store(note, &mei_note, ctx);

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

    // Set xml:id — preserve original MusicXML ID if present, else generate
    let rest_id = match note.id {
        Some(ref orig_id) => {
            let id = orig_id.clone();
            ctx.map_id(orig_id, &id);
            id
        }
        None => ctx.generate_id_with_suffix("rest"),
    };
    mei_rest.common.xml_id = Some(rest_id.clone());

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

    // Rests can carry tuplet start/stop annotations — process them
    process_tuplets(note, &rest_id, ctx);

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

    // Set xml:id — preserve original MusicXML ID if present, else generate
    let mrest_id = match note.id {
        Some(ref orig_id) => {
            let id = orig_id.clone();
            ctx.map_id(orig_id, &id);
            id
        }
        None => ctx.generate_id_with_suffix("mrest"),
    };
    mei_mrest.common.xml_id = Some(mrest_id);

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

    // Always generate a fresh chord xml:id. Child notes keep their own IDs for
    // slur/phrase/tuplet references; the chord must have a distinct ID so that
    // fixup_tuplet_ids_for_chord can replace child note IDs with the chord ID.
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
    let mut first_stem_handled = false;
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
        // For special stem values (Double/None) in ExtensionStore, only keep on the first
        // chord note — chord stem applies to all notes, and export only emits on first note.
        if let Some(ref note_id) = mei_note.common.xml_id {
            if first_stem_handled {
                ctx.ext_store_mut().remove_stem_extras(note_id);
            } else if ctx.ext_store().stem_extras(note_id).is_some() {
                first_stem_handled = true;
            }
        }
        mei_chord
            .children
            .push(ChordChild::Note(Box::new(mei_note)));
    }

    // Replace child note IDs with chord ID in tuplet tracking.
    // MusicXML puts duplicate tuplet start/stop on each chord member,
    // but MEI tupletSpan should reference the chord, not internal notes.
    let child_note_ids: Vec<String> = mei_chord
        .children
        .iter()
        .filter_map(|cc| {
            let ChordChild::Note(note) = cc;
            note.common.xml_id.clone()
        })
        .collect();
    let chord_id = mei_chord.common.xml_id.clone().unwrap_or_default();
    ctx.fixup_tuplet_ids_for_chord(&child_note_ids, &chord_id);

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

/// Convert MusicXML articulations to MEI @artic attribute.
///
/// Maps MusicXML articulation elements (accent, staccato, tenuto, etc.)
/// to MEI DataArticulation. Only the first articulation is stored in @artic
/// (MEI model limitation); full articulation data is preserved in the
/// ExtensionStore for lossless roundtrip.
fn convert_articulations(
    note: &MusicXmlNote,
    mei_note: &mut MeiNote,
    _ctx: &mut ConversionContext,
) {
    if let Some(ref notations) = note.notations {
        if let Some(ref artics) = notations.articulations {
            let tokens = articulations_to_mei(artics);
            mei_note.note_anl.artic = tokens.first().copied();
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
///   with OrnamentDetailData in ExtensionStore
/// - tremolo single → pending bTrem wrapper; start/stop → pending fTrem wrapper
///   (resolved in structure.rs); unmeasured → OrnamentDetailData in ExtensionStore
/// - wavy-line, other-ornament → MEI `<ornam>` + OrnamentDetailData
fn process_ornaments(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::import::direction::convert_placement;
    use crate::model::data::AboveBelow;
    use tusk_model::data::{DataBoolean, DataStaffrel, DataUri};
    use tusk_model::elements::{
        MeasureChild, Mordent as MeiMordent, Ornam, OrnamChild, Trill, Turn,
    };
    use tusk_model::musicxml_ext::OrnamentDetailData;

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

    // vertical-turn → MEI <ornam> + ExtensionStore
    if let Some(ref vt) = ornaments.vertical_turn {
        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(vt.placement);
        ctx.ext_store_mut()
            .insert_ornament_detail(id, OrnamentDetailData::VerticalTurn);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // inverted-vertical-turn → MEI <ornam> + ExtensionStore
    if let Some(ref ivt) = ornaments.inverted_vertical_turn {
        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(ivt.placement);
        ctx.ext_store_mut()
            .insert_ornament_detail(id, OrnamentDetailData::InvertedVerticalTurn);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // shake → MEI <ornam> + ExtensionStore
    if let Some(ref shake) = ornaments.shake {
        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(shake.placement);
        ctx.ext_store_mut()
            .insert_ornament_detail(id, OrnamentDetailData::Shake);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // schleifer → MEI <ornam> + ExtensionStore
    if ornaments.schleifer.is_some() {
        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ctx.ext_store_mut()
            .insert_ornament_detail(id, OrnamentDetailData::Schleifer);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // tremolo → store type/value in context for post-processing in structure.rs.
    // After beam restructuring, the layer children are scanned to wrap notes in
    // bTrem (single) or fTrem (start/stop) containers.
    // Unmeasured tremolo has no MEI container equivalent; uses ornam label fallback.
    if let Some(ref tremolo) = ornaments.tremolo {
        use crate::context::PendingTremolo;
        match tremolo.tremolo_type {
            crate::model::data::TremoloType::Single
            | crate::model::data::TremoloType::Start
            | crate::model::data::TremoloType::Stop => {
                ctx.set_pending_tremolo(PendingTremolo {
                    tremolo_type: tremolo.tremolo_type,
                    value: tremolo.value.unwrap_or(3),
                });
            }
            crate::model::data::TremoloType::Unmeasured => {
                let mut ornam = Ornam::default();
                let id = ctx.generate_id_with_suffix("ornam");
                ornam.common.xml_id = Some(id.clone());
                ornam.ornam_log.startid = Some(startid.clone());
                ornam.ornam_log.staff = Some(staff_str.clone());
                ornam.ornam_vis.place = place_for(tremolo.placement);
                ctx.ext_store_mut().insert_ornament_detail(
                    id,
                    OrnamentDetailData::UnmeasuredTremolo {
                        tremolo_type: "unmeasured".to_string(),
                        value: tremolo.value,
                    },
                );
                ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
            }
        }
    }

    // haydn → MEI <ornam> + ExtensionStore
    if let Some(ref haydn) = ornaments.haydn {
        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(haydn.placement);
        ctx.ext_store_mut()
            .insert_ornament_detail(id, OrnamentDetailData::Haydn);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // wavy-line → MEI <ornam> + ExtensionStore
    if let Some(ref wavy) = ornaments.wavy_line {
        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        let type_str = match wavy.wavy_line_type {
            StartStopContinue::Start => "start",
            StartStopContinue::Stop => "stop",
            StartStopContinue::Continue => "continue",
        };
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(wavy.placement);
        ctx.ext_store_mut().insert_ornament_detail(
            id,
            OrnamentDetailData::WavyLine {
                wavy_line_type: type_str.to_string(),
                number: wavy.number,
            },
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // other-ornament → MEI <ornam> + ExtensionStore
    if let Some(ref other) = ornaments.other_ornament {
        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(other.placement);
        if !other.value.is_empty() {
            ornam.children.push(OrnamChild::Text(other.value.clone()));
        }
        ctx.ext_store_mut().insert_ornament_detail(
            id,
            OrnamentDetailData::OtherOrnament {
                text: other.value.clone(),
            },
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }

    // accidental-mark within ornaments → MEI <ornam> + ExtensionStore
    for acc_mark in &ornaments.accidental_marks {
        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        ornam.ornam_vis.place = place_for(acc_mark.placement);
        ctx.ext_store_mut().insert_ornament_detail(
            id,
            OrnamentDetailData::OrnamentAccidentalMark {
                value: acc_mark.value.clone(),
                placement: acc_mark.placement.as_ref().map(|p| match p {
                    AboveBelow::Above => "above".to_string(),
                    AboveBelow::Below => "below".to_string(),
                }),
            },
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }
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
// Breath Mark Processing
// ============================================================================

/// Process breath-mark articulations into MEI `<breath>` control events.
fn process_breath_marks(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::model::notations::BreathMarkValue;
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic, DataUri};
    use tusk_model::elements::{Breath as MeiBreath, MeasureChild};

    let artics = match note.notations {
        Some(ref n) => match n.articulations {
            Some(ref a) => a,
            None => return,
        },
        None => return,
    };
    let bm = match artics.breath_mark {
        Some(ref bm) => bm,
        None => return,
    };

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    let mut b = MeiBreath::default();
    b.common.xml_id = Some(ctx.generate_id_with_suffix("breath"));
    b.breath_log.startid = Some(startid);
    b.breath_log.staff = Some(staff_str);

    // Map placement → MEI @place
    b.breath_vis.place = match bm.placement {
        Some(crate::model::data::AboveBelow::Above) => {
            Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above))
        }
        Some(crate::model::data::AboveBelow::Below) => {
            Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Below))
        }
        None => None,
    };

    // Store breath-mark value in ExtensionStore for lossless roundtrip
    let value = match bm.value {
        Some(BreathMarkValue::Comma) => Some("comma".to_string()),
        Some(BreathMarkValue::Tick) => Some("tick".to_string()),
        Some(BreathMarkValue::Upbow) => Some("upbow".to_string()),
        Some(BreathMarkValue::Salzedo) => Some("salzedo".to_string()),
        Some(BreathMarkValue::Empty) | None => None,
    };
    if let Some(ref id) = b.common.xml_id {
        ctx.ext_store_mut().insert_ornament_detail(
            id.clone(),
            tusk_model::musicxml_ext::OrnamentDetailData::BreathMark { value },
        );
    }

    ctx.add_ornament_event(MeasureChild::Breath(Box::new(b)));
}

// ============================================================================
// Caesura Processing
// ============================================================================

/// Process caesura articulations into MEI `<caesura>` control events.
fn process_caesuras(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::model::notations::CaesuraValue;
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic, DataUri};
    use tusk_model::elements::{Caesura as MeiCaesura, MeasureChild};

    let artics = match note.notations {
        Some(ref n) => match n.articulations {
            Some(ref a) => a,
            None => return,
        },
        None => return,
    };
    let cs = match artics.caesura {
        Some(ref cs) => cs,
        None => return,
    };

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    let mut c = MeiCaesura::default();
    c.common.xml_id = Some(ctx.generate_id_with_suffix("caesura"));
    c.caesura_log.startid = Some(startid);
    c.caesura_log.staff = Some(staff_str);

    // Map placement → MEI @place
    c.caesura_vis.place = match cs.placement {
        Some(crate::model::data::AboveBelow::Above) => {
            Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Above))
        }
        Some(crate::model::data::AboveBelow::Below) => {
            Some(DataStaffrel::MeiDataStaffrelBasic(DataStaffrelBasic::Below))
        }
        None => None,
    };

    // Store caesura value in ExtensionStore for lossless roundtrip
    let value = match cs.value {
        Some(CaesuraValue::Normal) => Some("normal".to_string()),
        Some(CaesuraValue::Short) => Some("short".to_string()),
        Some(CaesuraValue::Thick) => Some("thick".to_string()),
        Some(CaesuraValue::Curved) => Some("curved".to_string()),
        Some(CaesuraValue::Single) => Some("single".to_string()),
        Some(CaesuraValue::Empty) | None => None,
    };
    if let Some(ref id) = c.common.xml_id {
        ctx.ext_store_mut().insert_ornament_detail(
            id.clone(),
            tusk_model::musicxml_ext::OrnamentDetailData::Caesura { value },
        );
    }

    ctx.add_ornament_event(MeasureChild::Caesura(Box::new(c)));
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

    // non-arpeggiate → MEI <arpeg> with @order="nonarp" + ExtensionStore
    if let Some(ref _nonarp) = notations.non_arpeggiate {
        let mut a = Arpeg::default();
        let id = ctx.generate_id_with_suffix("arpeg");
        a.common.xml_id = Some(id.clone());
        a.arpeg_log.startid = Some(startid.clone());
        a.arpeg_log.staff = Some(staff_str.clone());
        a.arpeg_log.order = Some("nonarp".to_string());
        ctx.ext_store_mut().insert_ornament_detail(
            id,
            tusk_model::musicxml_ext::OrnamentDetailData::NonArpeggiate,
        );
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
                    is_slide: false,
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
                        is_slide: pending.is_slide,
                    });
                }
            }
        }
    }

    // slide elements → same as glissando but stored as OrnamentDetailData::Slide
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
                    is_slide: true,
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
                        is_slide: pending.is_slide,
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
/// Maps to MEI `<ornam>` + OrnamentDetailData in ExtensionStore.
fn process_accidental_marks(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{MeasureChild, Ornam, OrnamChild};
    use tusk_model::musicxml_ext::OrnamentDetailData;

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
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        if let Some(ref placement) = am.placement {
            use crate::import::direction::convert_placement;
            ornam.ornam_vis.place = convert_placement(Some(placement));
        }
        if !am.value.is_empty() {
            ornam.children.push(OrnamChild::Text(am.value.clone()));
        }
        ctx.ext_store_mut().insert_ornament_detail(
            id,
            OrnamentDetailData::AccidentalMark {
                value: am.value.clone(),
                placement: am.placement.as_ref().map(|p| {
                    use crate::model::data::AboveBelow;
                    match p {
                        AboveBelow::Above => "above".to_string(),
                        AboveBelow::Below => "below".to_string(),
                    }
                }),
            },
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }
}

// ============================================================================
// Other-Notation Processing
// ============================================================================

/// Process `<other-notation>` elements into MEI `<ornam>` control events.
///
/// Stores OrnamentDetailData::OtherNotation in ExtensionStore.
fn process_other_notations(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{MeasureChild, Ornam, OrnamChild};
    use tusk_model::musicxml_ext::OrnamentDetailData;

    let notations = match note.notations {
        Some(ref n) => n,
        None => return,
    };
    if notations.other_notations.is_empty() {
        return;
    }

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    for on in &notations.other_notations {
        let type_str = match on.notation_type {
            crate::model::data::StartStopSingle::Start => "start",
            crate::model::data::StartStopSingle::Stop => "stop",
            crate::model::data::StartStopSingle::Single => "single",
        };

        let mut ornam = Ornam::default();
        let id = ctx.generate_id_with_suffix("ornam");
        ornam.common.xml_id = Some(id.clone());
        ornam.ornam_log.startid = Some(startid.clone());
        ornam.ornam_log.staff = Some(staff_str.clone());
        if let Some(ref placement) = on.placement {
            use crate::import::direction::convert_placement;
            ornam.ornam_vis.place = convert_placement(Some(placement));
        }
        if !on.text.is_empty() {
            ornam.children.push(OrnamChild::Text(on.text.clone()));
        }
        ctx.ext_store_mut().insert_ornament_detail(
            id,
            OrnamentDetailData::OtherNotation {
                notation_type: type_str.to_string(),
                number: on.number,
                smufl: on.smufl.clone(),
                text: on.text.clone(),
            },
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(ornam)));
    }
}

// ============================================================================
// Notation-Level Dynamics Processing
// ============================================================================

/// Process dynamics within `<notations>` into MEI `<dynam>` control events with `@startid`.
///
/// Unlike direction-level dynamics (which use `@tstamp`), notation-level dynamics
/// are attached to a specific note via `@startid`. OrnamentDetailData::NotationDynamics
/// is stored in ExtensionStore to distinguish them from direction-level dynamics.
fn process_notation_dynamics(note: &MusicXmlNote, note_id: &str, ctx: &mut ConversionContext) {
    use crate::import::direction::convert_placement;
    use crate::import::utils::dynamics_value_to_string;
    use tusk_model::data::DataUri;
    use tusk_model::elements::{Dynam, DynamChild, MeasureChild};
    use tusk_model::musicxml_ext::OrnamentDetailData;

    let notations = match note.notations {
        Some(ref n) => n,
        None => return,
    };
    if notations.dynamics.is_empty() {
        return;
    }

    let mei_staff = ctx.staff().unwrap_or(1);
    let staff_str = (mei_staff as u64).to_string();
    let startid = DataUri::from(format!("#{}", note_id));

    for dyn_elem in &notations.dynamics {
        let mut dynam = Dynam::default();
        let id = ctx.generate_id_with_suffix("dynam");
        dynam.common.xml_id = Some(id.clone());
        dynam.dynam_log.startid = Some(startid.clone());
        dynam.dynam_log.staff = Some(staff_str.clone());
        dynam.dynam_vis.place = convert_placement(dyn_elem.placement.as_ref());

        let text_content = dyn_elem
            .values
            .iter()
            .map(dynamics_value_to_string)
            .collect::<Vec<_>>()
            .join("");
        if !text_content.is_empty() {
            dynam.children.push(DynamChild::Text(text_content));
        }

        ctx.ext_store_mut()
            .insert_ornament_detail(id, OrnamentDetailData::NotationDynamics);
        ctx.add_ornament_event(MeasureChild::Dynam(Box::new(dynam)));
    }
}

/// Process technical notations into MEI `<ornam>`/`<fing>` control events + ExtensionStore.
fn process_technical(
    note: &MusicXmlNote,
    note_id: &str,
    mei_note: &mut MeiNote,
    ctx: &mut ConversionContext,
) {
    use crate::import::direction::convert_placement;
    use crate::model::data::AboveBelow;
    use crate::model::technical::*;
    use tusk_model::data::{DataArticulation, DataUri};
    use tusk_model::elements::{MeasureChild, Ornam, OrnamChild};
    use tusk_model::musicxml_ext::TechnicalDetailData;

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

    // Helper macro to create an ornam (no label) and add it with ExtensionStore data
    macro_rules! emit {
        ($placement:expr, $data:expr) => {{
            let mut ornam = Ornam::default();
            let id = ctx.generate_id_with_suffix("ornam");
            ornam.common.xml_id = Some(id.clone());
            ornam.ornam_log.startid = Some(startid.clone());
            ornam.ornam_log.staff = Some(staff_str.clone());
            ornam.ornam_vis.place = place_for($placement);
            ctx.ext_store_mut().insert_technical_detail(id, $data);
            ornam
        }};
    }

    // Helper: store a tech-artic in NoteExtras for note-level roundtrip
    let mut tech_artics: Vec<TechnicalDetailData> = Vec::new();

    // Up-bow / down-bow → native MEI @artic + TechArticulation in NoteExtras
    for v in &tech.up_bow {
        if mei_note.note_anl.artic.is_none() {
            mei_note.note_anl.artic = Some(DataArticulation::Upbow);
        }
        let mut data = TechnicalDetailData::UpBow;
        // For tech-artic with placement, use TechArticulation wrapper
        let placement_str = match v.placement {
            Some(AboveBelow::Below) => Some("below"),
            Some(AboveBelow::Above) => Some("above"),
            _ => None,
        };
        if placement_str.is_some() {
            // Embed placement in a TechArticulation with name encoding
            data = TechnicalDetailData::TechArticulation {
                name: format!(
                    "upbow{}",
                    placement_str.map(|p| format!(",{}", p)).unwrap_or_default()
                ),
            };
        }
        tech_artics.push(data);
    }
    for v in &tech.down_bow {
        if mei_note.note_anl.artic.is_none() {
            mei_note.note_anl.artic = Some(DataArticulation::Dnbow);
        }
        let placement_str = match v.placement {
            Some(AboveBelow::Below) => Some("below"),
            Some(AboveBelow::Above) => Some("above"),
            _ => None,
        };
        let data = if placement_str.is_some() {
            TechnicalDetailData::TechArticulation {
                name: format!(
                    "dnbow{}",
                    placement_str.map(|p| format!(",{}", p)).unwrap_or_default()
                ),
            }
        } else {
            TechnicalDetailData::DownBow
        };
        tech_artics.push(data);
    }

    // Simple placement-only types → ornam + ExtensionStore
    for v in &tech.open_string {
        let o = emit!(v.placement, TechnicalDetailData::OpenString);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.thumb_position {
        let o = emit!(v.placement, TechnicalDetailData::ThumbPosition);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.double_tongue {
        let o = emit!(v.placement, TechnicalDetailData::DoubleTongue);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.triple_tongue {
        let o = emit!(v.placement, TechnicalDetailData::TripleTongue);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Snap-pizzicato → native MEI @artic + TechArticulation
    for v in &tech.snap_pizzicato {
        if mei_note.note_anl.artic.is_none() {
            mei_note.note_anl.artic = Some(DataArticulation::Snap);
        }
        let placement_str = match v.placement {
            Some(AboveBelow::Below) => Some("below"),
            Some(AboveBelow::Above) => Some("above"),
            _ => None,
        };
        let data = if placement_str.is_some() {
            TechnicalDetailData::TechArticulation {
                name: format!(
                    "snap{}",
                    placement_str.map(|p| format!(",{}", p)).unwrap_or_default()
                ),
            }
        } else {
            TechnicalDetailData::SnapPizzicato
        };
        tech_artics.push(data);
    }

    for v in &tech.fingernails {
        let o = emit!(v.placement, TechnicalDetailData::Fingernails);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.brass_bend {
        let o = emit!(v.placement, TechnicalDetailData::BrassBend);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.flip {
        let o = emit!(v.placement, TechnicalDetailData::Flip);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.smear {
        let o = emit!(v.placement, TechnicalDetailData::Smear);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.golpe {
        let o = emit!(v.placement, TechnicalDetailData::Golpe);
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Stopped → native MEI @artic when no smufl; ornam + ExtensionStore with smufl
    for v in &tech.stopped {
        if v.smufl.is_none() {
            if mei_note.note_anl.artic.is_none() {
                mei_note.note_anl.artic = Some(DataArticulation::Stop);
            }
            let placement_str = match v.placement {
                Some(AboveBelow::Below) => Some("below"),
                Some(AboveBelow::Above) => Some("above"),
                _ => None,
            };
            let data = if placement_str.is_some() {
                TechnicalDetailData::TechArticulation {
                    name: format!(
                        "stop{}",
                        placement_str.map(|p| format!(",{}", p)).unwrap_or_default()
                    ),
                }
            } else {
                TechnicalDetailData::Stopped { smufl: None }
            };
            tech_artics.push(data);
        } else {
            let o = emit!(
                v.placement,
                TechnicalDetailData::Stopped {
                    smufl: v.smufl.clone(),
                }
            );
            ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
        }
    }
    for v in &tech.open {
        let o = emit!(
            v.placement,
            TechnicalDetailData::Open {
                smufl: v.smufl.clone(),
            }
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.half_muted {
        let o = emit!(
            v.placement,
            TechnicalDetailData::HalfMuted {
                smufl: v.smufl.clone(),
            }
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Fingering → native MEI <fing> + ExtensionStore
    for v in &tech.fingering {
        let mut fing = tusk_model::elements::Fing::default();
        let id = ctx.generate_id_with_suffix("fing");
        fing.common.xml_id = Some(id.clone());
        fing.fing_log.startid = Some(startid.clone());
        fing.fing_log.staff = Some(staff_str.clone());
        fing.fing_vis.place = place_for(v.placement);
        if !v.value.is_empty() {
            fing.children
                .push(tusk_model::elements::FingChild::Text(v.value.clone()));
        }
        let sub = matches!(v.substitution, Some(crate::model::data::YesNo::Yes));
        let alt = matches!(v.alternate, Some(crate::model::data::YesNo::Yes));
        if sub || alt {
            ctx.ext_store_mut().insert_technical_detail(
                id,
                TechnicalDetailData::Fingering {
                    substitution: if sub { Some(true) } else { None },
                    alternate: if alt { Some(true) } else { None },
                },
            );
        }
        ctx.add_ornament_event(MeasureChild::Fing(Box::new(fing)));
    }

    // Pluck (text content)
    for v in &tech.pluck {
        let mut o = emit!(
            v.placement,
            TechnicalDetailData::Pluck {
                value: v.value.clone(),
            }
        );
        if !v.value.is_empty() {
            o.children.push(OrnamChild::Text(v.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Fret (numeric text)
    for v in &tech.fret {
        let mut o = emit!(
            None::<AboveBelow>,
            TechnicalDetailData::Fret { value: v.value }
        );
        o.children.push(OrnamChild::Text(v.value.to_string()));
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // String (numeric text)
    for v in &tech.string {
        let mut o = emit!(
            v.placement,
            TechnicalDetailData::StringNum { value: v.value }
        );
        o.children.push(OrnamChild::Text(v.value.to_string()));
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Hammer-on / Pull-off
    for v in &tech.hammer_on {
        let type_str = match v.ho_type {
            crate::model::data::StartStop::Start => "start",
            crate::model::data::StartStop::Stop => "stop",
        };
        let mut o = emit!(
            v.placement,
            TechnicalDetailData::HammerOn {
                ho_type: type_str.to_string(),
                number: v.number,
                text: v.text.clone(),
            }
        );
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
        let mut o = emit!(
            v.placement,
            TechnicalDetailData::PullOff {
                po_type: type_str.to_string(),
                number: v.number,
                text: v.text.clone(),
            }
        );
        if !v.text.is_empty() {
            o.children.push(OrnamChild::Text(v.text.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Tap (text + hand)
    for v in &tech.tap {
        let hand_str = v.hand.as_ref().map(|h| match h {
            TapHand::Left => "left".to_string(),
            TapHand::Right => "right".to_string(),
        });
        let mut o = emit!(
            v.placement,
            TechnicalDetailData::Tap {
                hand: hand_str,
                value: v.value.clone(),
            }
        );
        if !v.value.is_empty() {
            o.children.push(OrnamChild::Text(v.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Heel / Toe
    for v in &tech.heel {
        let sub = matches!(v.substitution, Some(crate::model::data::YesNo::Yes));
        let o = emit!(
            v.placement,
            TechnicalDetailData::Heel {
                substitution: if sub { Some(true) } else { None },
            }
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }
    for v in &tech.toe {
        let sub = matches!(v.substitution, Some(crate::model::data::YesNo::Yes));
        let o = emit!(
            v.placement,
            TechnicalDetailData::Toe {
                substitution: if sub { Some(true) } else { None },
            }
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Bend
    for v in &tech.bend {
        let release = v.release.as_ref().map(|r| r.offset);
        let shape_str = v.shape.as_ref().map(|s| match s {
            BendShape::Straight => "straight".to_string(),
            BendShape::Curved => "curved".to_string(),
        });
        let with_bar_text = v
            .with_bar
            .as_ref()
            .filter(|wb| !wb.value.is_empty())
            .map(|wb| wb.value.clone());
        let mut o = emit!(
            None::<AboveBelow>,
            TechnicalDetailData::Bend {
                alter: v.bend_alter,
                pre_bend: v.pre_bend.as_ref().map(|_| true),
                release,
                shape: shape_str,
                with_bar: with_bar_text.clone(),
            }
        );
        if let Some(ref wb_text) = with_bar_text {
            o.children.push(OrnamChild::Text(wb_text.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Hole
    for v in &tech.hole {
        let closed_str = match v.hole_closed.value {
            HoleClosedValue::Yes => "yes",
            HoleClosedValue::No => "no",
            HoleClosedValue::Half => "half",
        };
        let loc_str = v.hole_closed.location.as_ref().map(|loc| match loc {
            HoleClosedLocation::Right => "right".to_string(),
            HoleClosedLocation::Bottom => "bottom".to_string(),
            HoleClosedLocation::Left => "left".to_string(),
            HoleClosedLocation::Top => "top".to_string(),
        });
        let o = emit!(
            v.placement,
            TechnicalDetailData::Hole {
                closed: closed_str.to_string(),
                location: loc_str,
                hole_type: v.hole_type.clone(),
                hole_shape: v.hole_shape.clone(),
            }
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Arrow
    for v in &tech.arrow {
        let content_data = match &v.content {
            ArrowContent::Directional {
                direction,
                style,
                arrowhead,
            } => tusk_model::musicxml_ext::ArrowContentData::Directional {
                direction: direction.clone(),
                style: style.clone(),
                arrowhead: *arrowhead,
            },
            ArrowContent::Circular(value) => {
                tusk_model::musicxml_ext::ArrowContentData::Circular(value.clone())
            }
        };
        let o = emit!(
            v.placement,
            TechnicalDetailData::Arrow {
                content: content_data,
            }
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Handbell
    for v in &tech.handbell {
        let mut o = emit!(
            v.placement,
            TechnicalDetailData::Handbell {
                value: v.value.clone(),
            }
        );
        o.children.push(OrnamChild::Text(v.value.clone()));
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Harmon mute
    for v in &tech.harmon_mute {
        let closed_str = match v.harmon_closed.value {
            HarmonClosedValue::Yes => "yes",
            HarmonClosedValue::No => "no",
            HarmonClosedValue::Half => "half",
        };
        let loc_str = v.harmon_closed.location.as_ref().map(|loc| match loc {
            HarmonClosedLocation::Right => "right".to_string(),
            HarmonClosedLocation::Bottom => "bottom".to_string(),
            HarmonClosedLocation::Left => "left".to_string(),
            HarmonClosedLocation::Top => "top".to_string(),
        });
        let o = emit!(
            v.placement,
            TechnicalDetailData::HarmonMute {
                closed: closed_str.to_string(),
                location: loc_str,
            }
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Harmonic
    for v in &tech.harmonic {
        let o = emit!(
            v.placement,
            TechnicalDetailData::Harmonic {
                natural: v.natural.as_ref().map(|_| true),
                artificial: v.artificial.as_ref().map(|_| true),
                base_pitch: v.base_pitch.as_ref().map(|_| true),
                touching_pitch: v.touching_pitch.as_ref().map(|_| true),
                sounding_pitch: v.sounding_pitch.as_ref().map(|_| true),
            }
        );
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Other-technical
    for v in &tech.other_technical {
        let mut o = emit!(
            v.placement,
            TechnicalDetailData::OtherTechnical {
                smufl: v.smufl.clone(),
                text: v.value.clone(),
            }
        );
        if !v.value.is_empty() {
            o.children.push(OrnamChild::Text(v.value.clone()));
        }
        ctx.add_ornament_event(MeasureChild::Ornam(Box::new(o)));
    }

    // Store tech-artics in NoteExtras
    if !tech_artics.is_empty() {
        let extras = ctx
            .ext_store_mut()
            .note_extras_map
            .entry(note_id.to_string())
            .or_default();
        extras.tech_artics = tech_artics;
    }
}

// ============================================================================
// Lyric Conversion
// ============================================================================

/// Convert MusicXML lyrics to MEI verse/syl children on the note.
///
/// Maps:
/// - `lyric/@number` → `verse/@n`
/// - `syllabic` → `syl/@wordpos` (begin→"i", middle→"m", end→"t", single→omitted)
/// - `syllabic` begin/middle → `syl/@con="d"` (dash connector)
/// - `text` value → `syl` text child
/// - MusicXML-only attrs roundtrip via `LyricExtras` in ExtensionStore
fn convert_lyrics(note: &MusicXmlNote, mei_note: &mut MeiNote) {
    use crate::model::lyric::{LyricContent, Syllabic};
    use tusk_model::elements::{Syl, SylChild, Verse, VerseChild};

    for lyric in &note.lyrics {
        let mut verse = Verse::default();

        // verse @n from lyric number
        if let Some(ref num) = lyric.number {
            verse.common.n = Some(tusk_model::data::DataWord::from(num.clone()));
        }

        match &lyric.content {
            LyricContent::Text {
                syllable_groups, ..
            } => {
                for (i, group) in syllable_groups.iter().enumerate() {
                    let mut syl = Syl::default();

                    // @wordpos from syllabic
                    if let Some(syllabic) = &group.syllabic {
                        match syllabic {
                            Syllabic::Begin => {
                                syl.syl_log.wordpos = Some("i".to_string());
                                syl.syl_log.con = Some("d".to_string());
                            }
                            Syllabic::Middle => {
                                syl.syl_log.wordpos = Some("m".to_string());
                                syl.syl_log.con = Some("d".to_string());
                            }
                            Syllabic::End => {
                                syl.syl_log.wordpos = Some("t".to_string());
                            }
                            Syllabic::Single => {
                                // No wordpos for single syllable
                            }
                        }
                    }

                    // Elision connector: if this syl follows an elision, mark previous syl @con="b"
                    if i > 0 && group.elision.is_some() {
                        // Update the connector on the previous syl
                        if let Some(VerseChild::Syl(prev_syl)) = verse.children.last_mut() {
                            prev_syl.syl_log.con = Some("b".to_string());
                        }
                    }

                    // Text content
                    syl.children.push(SylChild::Text(group.text.value.clone()));

                    verse.children.push(VerseChild::Syl(Box::new(syl)));
                }
            }
            LyricContent::ExtendOnly(_) | LyricContent::Laughing | LyricContent::Humming => {
                // No MEI structural content; roundtrip via ExtensionStore LyricExtras
            }
        }

        mei_note.children.push(NoteChild::Verse(Box::new(verse)));
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
    // breath_mark and caesura are stored as native MEI <breath>/<caesura> control events

    result
}

/// Populate ExtensionStore with typed note-level extension data.
fn populate_note_ext_store(
    note: &crate::model::note::Note,
    mei_note: &tusk_model::elements::Note,
    ctx: &mut ConversionContext,
) {
    use crate::model::note::{NoteVisualAttrs, StemValue};
    use tusk_model::musicxml_ext::{LyricExtras, NoteExtras, NoteVisualData, PlayData, StemExtras};

    let note_id = match mei_note.common.xml_id {
        Some(ref id) => id.clone(),
        None => return,
    };

    // StemExtras
    if let Some(ref stem) = note.stem {
        let stem_ext = match stem.value {
            StemValue::Double => Some(StemExtras::Double),
            StemValue::None => Some(StemExtras::None),
            _ => None,
        };
        if let Some(ext) = stem_ext {
            ctx.ext_store_mut().insert_stem_extras(note_id.clone(), ext);
        }
    }

    // NoteVisualData
    let vis = NoteVisualAttrs::from_note(note);
    if !vis.is_empty() {
        let nvd = NoteVisualData {
            default_x: vis.default_x,
            default_y: vis.default_y,
            relative_x: vis.relative_x,
            relative_y: vis.relative_y,
            print_object: vis
                .print_object
                .map(|v| matches!(v, crate::model::data::YesNo::Yes)),
            print_leger: vis
                .print_leger
                .map(|v| matches!(v, crate::model::data::YesNo::Yes)),
            print_spacing: vis
                .print_spacing
                .map(|v| matches!(v, crate::model::data::YesNo::Yes)),
            color: vis.color.clone(),
            dynamics: vis.dynamics,
            end_dynamics: vis.end_dynamics,
            attack: vis.attack,
            release: vis.release,
            pizzicato: vis
                .pizzicato
                .map(|v| matches!(v, crate::model::data::YesNo::Yes)),
        };
        ctx.ext_store_mut().insert_note_visual(note_id.clone(), nvd);
    }

    // NoteExtras
    let mut extras = NoteExtras::default();
    let mut has_extras = false;

    if let Some(ref nh) = note.notehead {
        extras.notehead = Some(nh.clone());
        has_extras = true;
    }
    if let Some(ref nht) = note.notehead_text {
        extras.notehead_text = Some(nht.clone());
        has_extras = true;
    }
    if let Some(ref play) = note.play {
        extras.play = Some(PlayData {
            id: play.id.clone(),
            entries: play.entries.clone(),
        });
        has_extras = true;
    }
    if let Some(ref listen) = note.listen {
        extras.listen = Some(listen.clone());
        has_extras = true;
    }
    if let Some(ref ft) = note.footnote {
        extras.footnote = Some(ft.clone());
        has_extras = true;
    }
    if let Some(ref lv) = note.level {
        extras.level = Some(lv.clone());
        has_extras = true;
    }
    if let Some(ref notations) = note.notations {
        if let Some(ref ft) = notations.footnote {
            extras.notations_footnote = Some(ft.clone());
            has_extras = true;
        }
        if let Some(ref lv) = notations.level {
            extras.notations_level = Some(lv.clone());
            has_extras = true;
        }
    }
    if !note.instruments.is_empty() {
        extras.instruments = note.instruments.iter().map(|i| i.id.clone()).collect();
        has_extras = true;
    }

    // Store full articulations for lossless multi-artic roundtrip
    // Strip breath_mark/caesura — those are now native MEI <breath>/<caesura> control events
    if let Some(ref notations) = note.notations {
        if let Some(ref artics) = notations.articulations {
            let mut artics_for_store = artics.clone();
            artics_for_store.breath_mark = None;
            artics_for_store.caesura = None;
            extras.articulations = Some(artics_for_store);
            has_extras = true;
        }
    }

    if has_extras {
        ctx.ext_store_mut()
            .insert_note_extras(note_id.clone(), extras);
    }

    // LyricExtras — store per-verse ext data on note id with verse suffix
    for lyric in &note.lyrics {
        let verse_key = match &lyric.number {
            Some(num) => format!("{}_v{}", note_id, num),
            None => format!("{}_v", note_id),
        };
        ctx.ext_store_mut()
            .insert_lyric_extras(verse_key, LyricExtras { lyric: Some(lyric.clone()) });
    }
}

/// Import note-level visual/position/print attributes to MEI.
///
/// Maps semantically equivalent attributes to MEI visual fields:
/// - `color` → MEI `@color`
/// - `print-object="no"` → MEI `@visible="false"`
///
/// All visual attributes (position, color, print, dynamics, attack/release,
/// pizzicato) are stored as JSON-in-label for lossless roundtrip.
fn convert_note_visual_attrs(
    note: &crate::model::note::Note,
    mei_note: &mut tusk_model::elements::Note,
) {
    use tusk_model::data::{DataBoolean, DataColor, DataColorvalues};

    // Map color to MEI @color
    if let Some(ref color) = note.color {
        mei_note.note_vis.color = Some(DataColor::MeiDataColorvalues(DataColorvalues(
            color.clone(),
        )));
    }

    // Map print-object="no" to MEI @visible="false"
    if note.print_object == Some(crate::model::data::YesNo::No) {
        mei_note.note_vis.visible = Some(DataBoolean::False);
    }

    // Full visual attrs stored in ExtensionStore via populate_note_ext_store
}

/// Convert MusicXML Notehead to MEI head_shape/head_fill + JSON-in-label for lossless roundtrip.
fn convert_notehead_to_mei(
    nh: &crate::model::note::Notehead,
    mei_note: &mut tusk_model::elements::Note,
) {
    use crate::model::note::NoteheadValue;
    use tusk_model::data::{DataFill, DataHeadshape, DataHeadshapeList};

    // Map notehead value to MEI head_shape where possible
    let head_shape = match nh.value {
        NoteheadValue::Diamond => Some(DataHeadshape::MeiDataHeadshapeList(
            DataHeadshapeList::Diamond,
        )),
        NoteheadValue::Triangle => Some(DataHeadshape::MeiDataHeadshapeList(
            DataHeadshapeList::Isotriangle,
        )),
        NoteheadValue::Square => Some(DataHeadshape::MeiDataHeadshapeList(
            DataHeadshapeList::Rectangle,
        )),
        NoteheadValue::Slash => Some(DataHeadshape::MeiDataHeadshapeList(
            DataHeadshapeList::Slash,
        )),
        NoteheadValue::Cross | NoteheadValue::X => {
            Some(DataHeadshape::MeiDataHeadshapeList(DataHeadshapeList::X))
        }
        NoteheadValue::Circled | NoteheadValue::CircleX => Some(
            DataHeadshape::MeiDataHeadshapeList(DataHeadshapeList::Circle),
        ),
        NoteheadValue::Rectangle => Some(DataHeadshape::MeiDataHeadshapeList(
            DataHeadshapeList::Rectangle,
        )),
        NoteheadValue::BackSlashed => Some(DataHeadshape::MeiDataHeadshapeList(
            DataHeadshapeList::Backslash,
        )),
        _ => None,
    };

    if let Some(shape) = head_shape {
        mei_note.note_vis.head_shape = Some(shape);
    }

    // Map filled attribute to MEI head_fill
    if let Some(ref filled) = nh.filled {
        use crate::model::data::YesNo;
        mei_note.note_vis.head_fill = Some(match filled {
            YesNo::Yes => DataFill::Solid,
            YesNo::No => DataFill::Void,
        });
    }

    // Full notehead stored in ExtensionStore via populate_note_ext_store
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

        let grace = Grace {
            slash: Some(YesNo::Yes), // Slashed grace note
            ..Default::default()
        };

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
            .map(|c| {
                let ChordChild::Note(n) = c;
                n.as_ref()
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

//! MEI note, rest, and chord conversion to MusicXML.
//!
//! This module handles conversion of MEI note, rest, and chord elements to their
//! MusicXML equivalents. Notes are converted to MusicXML `<note>` elements,
//! rests become `<note>` elements containing `<rest>`, and chords become sequences
//! of `<note>` elements with chord flags on all but the first.

use super::utils::{
    apply_dots, convert_mei_duration_rests_to_note_type, convert_mei_duration_to_note_type,
    convert_mei_stem_direction, duration_rests_to_quarter_notes, duration_to_quarter_notes,
};
use crate::context::ConversionContext;
use crate::convert_error::ConversionResult;

// ============================================================================
// MEI Note → MusicXML Note Conversion
// ============================================================================

/// Convert an MEI note to a MusicXML note.
///
/// This converts an MEI note element to MusicXML, including:
/// - Pitch (pname, oct → step, octave; accid.ges → alter) for pitched notes
/// - Unpitched (from @loc) for percussion/unpitched notes
/// - Duration (dur → type; dots → dot elements; calculated duration in divisions)
/// - Grace notes (@grace → grace element)
/// - Written accidentals (accid child → accidental element)
/// - Stem direction (@stem.dir → stem element)
/// - Cue notes (@cue → cue element)
/// - IDs (xml:id → id attribute)
///
/// # Lossy Conversion Notes
///
/// The following MEI attributes are lost in conversion:
/// - Analytical attributes (@pclass, @deg, etc.) - no MusicXML equivalent
/// - Gestural attributes other than accid.ges - limited MusicXML support
/// - Visual attributes beyond stem direction - partial support
/// - Editorial child elements (app, choice, etc.) - no MusicXML equivalent
///
/// # Arguments
///
/// * `mei_note` - The MEI note to convert
/// * `ctx` - The conversion context for state tracking
///
/// # Returns
///
/// A MusicXML Note element, or an error if conversion fails.
pub fn convert_mei_note(
    mei_note: &tusk_model::elements::Note,
    ctx: &mut ConversionContext,
) -> ConversionResult<crate::model::note::Note> {
    use crate::model::elements::Empty;
    use crate::model::note::{Dot, Note as MxmlNote, NoteType, Stem};

    // Build the MusicXML note
    let mut mxml_note: MxmlNote;

    // Determine if this is a grace note (grace notes have no duration)
    let is_grace = mei_note.note_log.grace.is_some();

    // Determine if this is an unpitched note (no pname = percussion/unpitched)
    let is_unpitched = mei_note.note_log.pname.is_none();

    if is_unpitched {
        // Unpitched note: convert @loc to display-step/display-octave
        let unpitched = convert_mei_loc_to_unpitched(mei_note);

        if is_grace {
            let grace = convert_mei_grace(mei_note);
            mxml_note = MxmlNote::unpitched_grace(unpitched, grace);
        } else {
            let duration = calculate_mei_note_duration(mei_note, ctx);
            mxml_note = MxmlNote::unpitched(unpitched, duration);
        }
    } else {
        // Pitched note: convert pname, oct to step, octave, alter
        let pitch = convert_mei_pitch(mei_note, ctx)?;

        if is_grace {
            let grace = convert_mei_grace(mei_note);
            mxml_note = MxmlNote::grace_note(pitch, grace);
        } else {
            let duration = calculate_mei_note_duration(mei_note, ctx);
            mxml_note = MxmlNote::pitched(pitch, duration);
        }
    }

    // Set ID from xml:id
    if let Some(ref xml_id) = mei_note.common.xml_id {
        mxml_note.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Convert note type (graphical duration)
    if let Some(ref dur) = mei_note.note_log.dur {
        mxml_note.note_type = Some(NoteType::new(convert_mei_duration_to_note_type(dur)));
    }

    // Convert dots
    if let Some(ref dots) = mei_note.note_log.dots {
        for _ in 0..dots.0 {
            mxml_note.dots.push(Dot::default());
        }
    }

    // Convert written accidental from child element
    for child in &mei_note.children {
        if let tusk_model::elements::NoteChild::Accid(accid) = child {
            mxml_note.accidental = Some(convert_mei_accid_to_mxml(accid, ctx)?);
        }
    }

    // Convert stem direction — check ext_store/label for special stems (Double/None)
    let stem_from_ext = mei_note.common.xml_id.as_ref().and_then(|id| {
        ctx.ext_store()
            .get(id)?
            .stem_extras
            .as_ref()
            .map(|se| match se {
                tusk_model::musicxml_ext::StemExtras::Double => {
                    crate::model::note::StemValue::Double
                }
                tusk_model::musicxml_ext::StemExtras::None => crate::model::note::StemValue::None,
            })
    });
    let stem_from_label = stem_from_ext.or_else(|| {
        mei_note.common.label.as_deref().and_then(|l| {
            l.split('|').find_map(|seg| {
                seg.strip_prefix("musicxml:stem,").map(|v| match v {
                    "double" => crate::model::note::StemValue::Double,
                    "none" => crate::model::note::StemValue::None,
                    _ => crate::model::note::StemValue::Up,
                })
            })
        })
    });
    if let Some(sv) = stem_from_label {
        mxml_note.stem = Some(Stem::new(sv));
    } else if let Some(ref stem_dir) = mei_note.note_vis.stem_dir {
        mxml_note.stem = Some(Stem::new(convert_mei_stem_direction(stem_dir)));
    }

    // Convert cue note
    if mei_note.note_log.cue.as_ref() == Some(&tusk_model::data::DataBoolean::True) {
        mxml_note.cue = Some(Empty);
    }

    // Convert articulations from ExtensionStore (full data) or MEI @artic (single value)
    convert_mei_articulations(mei_note, &mut mxml_note, ctx);

    // Convert ties from MEI @tie attribute to MusicXML <tie> elements
    convert_mei_ties(mei_note, &mut mxml_note);

    // Convert verse/syl children to MusicXML lyrics
    convert_mei_lyrics(mei_note, &mut mxml_note, ctx);

    // Restore note-level instrument references from label
    convert_mei_note_label_instruments(mei_note, &mut mxml_note, ctx);

    // Restore notehead, notehead-text, play, listen, footnote, level from label
    restore_note_label_elements(mei_note, &mut mxml_note, ctx);

    // Add warnings for lossy attributes
    add_note_conversion_warnings(mei_note, ctx);

    Ok(mxml_note)
}

/// Convert MEI @loc attribute to MusicXML Unpitched element.
///
/// The @loc attribute in MEI represents staff position (0 = bottom line).
/// We convert this back to MusicXML display-step and display-octave.
fn convert_mei_loc_to_unpitched(
    mei_note: &tusk_model::elements::Note,
) -> crate::model::note::Unpitched {
    use crate::model::data::Step;
    use crate::model::note::Unpitched;

    if let Some(ref loc) = mei_note.note_vis.loc {
        // Convert @loc back to display-step and display-octave
        let loc_value = loc.0 as i32;

        // loc = octave * 7 + step_value
        // where step_value: C=0, D=1, E=2, F=3, G=4, A=5, B=6
        let octave = (loc_value / 7) as u8;
        let step_value = loc_value % 7;

        let display_step = match step_value {
            0 => Step::C,
            1 => Step::D,
            2 => Step::E,
            3 => Step::F,
            4 => Step::G,
            5 => Step::A,
            6 => Step::B,
            _ => Step::C, // Fallback
        };

        Unpitched {
            display_step: Some(display_step),
            display_octave: Some(octave),
        }
    } else {
        // No @loc - return empty unpitched
        Unpitched::default()
    }
}

/// Convert MEI pitch attributes to MusicXML Pitch.
fn convert_mei_pitch(
    mei_note: &tusk_model::elements::Note,
    _ctx: &mut ConversionContext,
) -> ConversionResult<crate::model::note::Pitch> {
    use crate::model::data::Step;
    use crate::model::note::Pitch;

    // Get pitch name (pname)
    let step = if let Some(ref pname) = mei_note.note_log.pname {
        convert_mei_pname_to_step(pname.0.as_str())?
    } else {
        // Default to C if not specified (shouldn't happen in valid MEI)
        Step::C
    };

    // Get octave
    let octave = if let Some(ref oct) = mei_note.note_log.oct {
        oct.0 as u8
    } else {
        4 // Default octave
    };

    // Get chromatic alteration from gestural accidental
    let alter = convert_mei_gestural_accid_to_alter(&mei_note.note_ges.accid_ges);

    Ok(Pitch {
        step,
        alter,
        octave,
    })
}

/// Convert MEI pitch name string to MusicXML Step.
fn convert_mei_pname_to_step(pname: &str) -> ConversionResult<crate::model::data::Step> {
    use crate::model::data::Step;

    let name = pname.trim().to_lowercase();
    match name.as_str() {
        "a" => Ok(Step::A),
        "b" => Ok(Step::B),
        "c" => Ok(Step::C),
        "d" => Ok(Step::D),
        "e" => Ok(Step::E),
        "f" => Ok(Step::F),
        "g" => Ok(Step::G),
        _ => {
            // Return C as fallback for invalid pitch names
            Ok(Step::C)
        }
    }
}

/// Convert MEI gestural accidental (@accid.ges) to MusicXML alter value.
fn convert_mei_gestural_accid_to_alter(
    accid_ges: &Option<tusk_model::data::DataAccidentalGestural>,
) -> Option<f64> {
    use tusk_model::data::{
        DataAccidentalGestural, DataAccidentalGesturalBasic, DataAccidentalGesturalExtended,
    };

    accid_ges.as_ref().map(|a| match a {
        DataAccidentalGestural::MeiDataAccidentalGesturalBasic(basic) => match basic {
            DataAccidentalGesturalBasic::Tf => -3.0,
            DataAccidentalGesturalBasic::Ff => -2.0,
            DataAccidentalGesturalBasic::F => -1.0,
            DataAccidentalGesturalBasic::N => 0.0,
            DataAccidentalGesturalBasic::S => 1.0,
            DataAccidentalGesturalBasic::Ss => 2.0,
            DataAccidentalGesturalBasic::Ts => 3.0,
        },
        DataAccidentalGestural::MeiDataAccidentalGesturalExtended(ext) => match ext {
            DataAccidentalGesturalExtended::Su => 1.5,
            DataAccidentalGesturalExtended::Sd => 0.5,
            DataAccidentalGesturalExtended::Fu => -0.5,
            DataAccidentalGesturalExtended::Fd => -1.5,
            DataAccidentalGesturalExtended::Xu => 2.5,
            DataAccidentalGesturalExtended::Ffd => -2.5,
        },
        DataAccidentalGestural::MeiDataAccidentalAeu(_)
        | DataAccidentalGestural::MeiDataAccidentalPersian(_) => 0.0,
    })
}

/// Calculate MEI note duration in MusicXML divisions.
fn calculate_mei_note_duration(
    mei_note: &tusk_model::elements::Note,
    ctx: &ConversionContext,
) -> f64 {
    // First check if we have gestural duration in ppq (most accurate); MEI @dur.ppq is Option<String>
    if let Some(ref dur_ppq) = mei_note.note_ges.dur_ppq {
        if let Ok(n) = dur_ppq.parse::<f64>() {
            return n;
        }
    }

    // Calculate from written duration
    let divisions = ctx.divisions();
    let base_duration = if let Some(ref dur) = mei_note.note_log.dur {
        duration_to_quarter_notes(dur)
    } else {
        1.0 // Default to quarter note
    };

    // Apply dots
    let dot_count = mei_note.note_log.dots.as_ref().map(|d| d.0).unwrap_or(0);

    let dotted_duration = apply_dots(base_duration, dot_count);

    // Convert to divisions
    dotted_duration * divisions
}

/// Convert MEI grace attribute to MusicXML Grace element.
fn convert_mei_grace(mei_note: &tusk_model::elements::Note) -> crate::model::note::Grace {
    use crate::model::data::YesNo;
    use crate::model::note::Grace;
    use tusk_model::data::DataGrace;

    let mut grace = Grace::default();

    if let Some(ref grace_type) = mei_note.note_log.grace {
        match grace_type {
            DataGrace::Unacc => grace.slash = Some(YesNo::Yes),
            DataGrace::Acc | DataGrace::Unknown => grace.slash = Some(YesNo::No),
        }
    }

    grace
}

/// Convert MEI accid element to MusicXML Accidental.
fn convert_mei_accid_to_mxml(
    accid: &tusk_model::elements::Accid,
    _ctx: &mut ConversionContext,
) -> ConversionResult<crate::model::note::Accidental> {
    use crate::model::data::YesNo;
    use crate::model::note::{Accidental, AccidentalValue};

    let value = if let Some(ref accid_val) = accid.accid_log.accid {
        convert_mei_written_accid_to_mxml(accid_val)
    } else {
        AccidentalValue::Natural // Default if not specified
    };

    let mut mxml_accid = Accidental::new(value);

    // Convert cautionary/editorial function (@func is string in MEI: "caution", "edit")
    if let Some(ref func) = accid.accid_log.func {
        if func.as_str() == "caution" {
            mxml_accid.cautionary = Some(YesNo::Yes);
        } else if func.as_str() == "edit" {
            mxml_accid.editorial = Some(YesNo::Yes);
        }
    }
    // Convert enclosure
    use tusk_model::data::DataEnclosure;
    if let Some(ref enclose) = accid.accid_vis.enclose {
        match enclose {
            DataEnclosure::Paren => mxml_accid.parentheses = Some(YesNo::Yes),
            DataEnclosure::Brack | DataEnclosure::Box => mxml_accid.bracket = Some(YesNo::Yes), // no box in MusicXML, use bracket
            DataEnclosure::None => {}
        }
    }

    Ok(mxml_accid)
}

/// Convert MEI written accidental string to MusicXML AccidentalValue.
#[allow(dead_code)]
fn convert_mei_written_accid_str_to_mxml(s: &str) -> crate::model::note::AccidentalValue {
    use crate::model::note::AccidentalValue;
    match s.trim().to_lowercase().as_str() {
        "s" => AccidentalValue::Sharp,
        "f" => AccidentalValue::Flat,
        "ss" => AccidentalValue::SharpSharp,
        "x" => AccidentalValue::DoubleSharp,
        "ff" => AccidentalValue::FlatFlat,
        "xs" | "sx" | "ts" => AccidentalValue::TripleSharp,
        "tf" => AccidentalValue::TripleFlat,
        "n" => AccidentalValue::Natural,
        "nf" => AccidentalValue::NaturalFlat,
        "ns" => AccidentalValue::NaturalSharp,
        "nu" => AccidentalValue::QuarterSharp,
        "nd" => AccidentalValue::QuarterFlat,
        "su" => AccidentalValue::ThreeQuartersSharp,
        "sd" => AccidentalValue::SharpDown,
        "fu" => AccidentalValue::FlatUp,
        "fd" => AccidentalValue::ThreeQuartersFlat,
        "xu" => AccidentalValue::DoubleSharpUp,
        "xd" => AccidentalValue::DoubleSharpDown,
        "ffu" => AccidentalValue::FlatFlatUp,
        "ffd" => AccidentalValue::FlatFlatDown,
        _ => AccidentalValue::Natural,
    }
}

/// Convert MEI written accidental to MusicXML AccidentalValue (typed version for tests).
fn convert_mei_written_accid_to_mxml(
    accid: &tusk_model::data::DataAccidentalWritten,
) -> crate::model::note::AccidentalValue {
    use crate::model::note::AccidentalValue;
    use tusk_model::data::{
        DataAccidentalWritten, DataAccidentalWrittenBasic, DataAccidentalWrittenExtended,
    };

    match accid {
        DataAccidentalWritten::MeiDataAccidentalWrittenBasic(basic) => match basic {
            DataAccidentalWrittenBasic::S => AccidentalValue::Sharp,
            DataAccidentalWrittenBasic::F => AccidentalValue::Flat,
            DataAccidentalWrittenBasic::Ss => AccidentalValue::SharpSharp,
            DataAccidentalWrittenBasic::X => AccidentalValue::DoubleSharp,
            DataAccidentalWrittenBasic::Ff => AccidentalValue::FlatFlat,
            DataAccidentalWrittenBasic::Xs | DataAccidentalWrittenBasic::Sx => {
                AccidentalValue::TripleSharp
            }
            DataAccidentalWrittenBasic::Ts => AccidentalValue::TripleSharp,
            DataAccidentalWrittenBasic::Tf => AccidentalValue::TripleFlat,
            DataAccidentalWrittenBasic::N => AccidentalValue::Natural,
            DataAccidentalWrittenBasic::Nf => AccidentalValue::NaturalFlat,
            DataAccidentalWrittenBasic::Ns => AccidentalValue::NaturalSharp,
        },
        DataAccidentalWritten::MeiDataAccidentalWrittenExtended(ext) => match ext {
            DataAccidentalWrittenExtended::Nu => AccidentalValue::QuarterSharp,
            DataAccidentalWrittenExtended::Nd => AccidentalValue::QuarterFlat,
            DataAccidentalWrittenExtended::Su => AccidentalValue::ThreeQuartersSharp,
            DataAccidentalWrittenExtended::Sd => AccidentalValue::SharpDown,
            DataAccidentalWrittenExtended::Fu => AccidentalValue::FlatUp,
            DataAccidentalWrittenExtended::Fd => AccidentalValue::ThreeQuartersFlat,
            DataAccidentalWrittenExtended::Xu => AccidentalValue::DoubleSharpUp,
            DataAccidentalWrittenExtended::Xd => AccidentalValue::DoubleSharpDown,
            DataAccidentalWrittenExtended::Ffu => AccidentalValue::FlatFlatUp,
            DataAccidentalWrittenExtended::Ffd => AccidentalValue::FlatFlatDown,
            DataAccidentalWrittenExtended::N1qf => AccidentalValue::QuarterFlat,
            DataAccidentalWrittenExtended::N3qf => AccidentalValue::ThreeQuartersFlat,
            DataAccidentalWrittenExtended::N1qs => AccidentalValue::QuarterSharp,
            DataAccidentalWrittenExtended::N3qs => AccidentalValue::ThreeQuartersSharp,
        },
        // AEU/Persian — closest MusicXML equivalents
        _ => AccidentalValue::Natural,
    }
}

/// Convert MEI articulations to MusicXML.
///
/// Preferred path: read full `Articulations` from ExtensionStore (lossless roundtrip
/// of multiple articulations, breath-mark, caesura, other-articulation).
/// Fallback: reconstruct from MEI `@artic` (single value) + label segments.
fn convert_mei_articulations(
    mei_note: &tusk_model::elements::Note,
    mxml_note: &mut crate::model::note::Note,
    ctx: &ConversionContext,
) {
    use crate::model::notations::{Articulations, Notations};

    // Preferred: read full articulations from ExtensionStore
    if let Some(id) = &mei_note.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref extras) = ext.note_extras {
                if let Some(ref val) = extras.articulations {
                    if let Ok(artics) = serde_json::from_value::<Articulations>(val.clone()) {
                        if artics != Articulations::default() {
                            let notations =
                                mxml_note.notations.get_or_insert_with(Notations::default);
                            notations.articulations = Some(artics);
                        }
                        return;
                    }
                }
            }
        }
    }

    // Fallback: reconstruct from MEI @artic (single value)
    convert_mei_artic_single(mei_note.note_anl.artic.as_ref(), mxml_note);

    // Fallback: restore breath-mark, caesura, other-articulation from label segments
    convert_mei_note_label_articulations(mei_note, mxml_note);
}

/// Convert a single MEI DataArticulation to MusicXML notations (fallback path).
///
/// This handles all DataArticulation variants explicitly. Values that map to MusicXML
/// `<articulations>` are emitted there; values that map to `<technical>` are emitted
/// in the technical container; remaining values become `<other-articulation>`.
fn convert_mei_artic_single(
    artic: Option<&tusk_model::data::DataArticulation>,
    mxml_note: &mut crate::model::note::Note,
) {
    use crate::model::notations::{
        Articulations, EmptyPlacement, Notations, OtherArticulation, StrongAccent,
    };
    use crate::model::technical::{EmptyPlacementSmufl, Technical};
    use tusk_model::data::DataArticulation;

    let a = match artic {
        Some(x) => x,
        None => return,
    };

    let mut mxml_artic = Articulations::default();
    let mut mxml_tech = Technical::default();
    let mut other_name: Option<&str> = None;

    match a {
        // Direct articulation mappings
        DataArticulation::Acc => mxml_artic.accent = Some(EmptyPlacement::default()),
        DataArticulation::Marc => mxml_artic.strong_accent = Some(StrongAccent::default()),
        DataArticulation::Stacc => mxml_artic.staccato = Some(EmptyPlacement::default()),
        DataArticulation::Ten => mxml_artic.tenuto = Some(EmptyPlacement::default()),
        DataArticulation::Stacciss => mxml_artic.staccatissimo = Some(EmptyPlacement::default()),
        DataArticulation::Spicc => mxml_artic.spiccato = Some(EmptyPlacement::default()),
        DataArticulation::Scoop => mxml_artic.scoop = Some(EmptyPlacement::default()),
        DataArticulation::Plop => mxml_artic.plop = Some(EmptyPlacement::default()),
        DataArticulation::Doit => mxml_artic.doit = Some(EmptyPlacement::default()),
        DataArticulation::Fall => mxml_artic.falloff = Some(EmptyPlacement::default()),
        DataArticulation::Stress => mxml_artic.stress = Some(EmptyPlacement::default()),
        DataArticulation::Unstress => mxml_artic.unstress = Some(EmptyPlacement::default()),
        DataArticulation::AccSoft => mxml_artic.soft_accent = Some(EmptyPlacement::default()),
        // Technical mappings (MEI artic values that correspond to MusicXML <technical>)
        DataArticulation::Upbow => mxml_tech.up_bow.push(EmptyPlacement::default()),
        DataArticulation::Dnbow => mxml_tech.down_bow.push(EmptyPlacement::default()),
        DataArticulation::Harm => mxml_tech.harmonic.push(Default::default()),
        DataArticulation::Snap => mxml_tech.snap_pizzicato.push(EmptyPlacement::default()),
        DataArticulation::Fingernail => mxml_tech.fingernails.push(EmptyPlacement::default()),
        DataArticulation::Open => {
            mxml_tech.open.push(EmptyPlacementSmufl::default());
        }
        DataArticulation::Stop => {
            mxml_tech.stopped.push(EmptyPlacementSmufl::default());
        }
        DataArticulation::Dbltongue => mxml_tech.double_tongue.push(EmptyPlacement::default()),
        DataArticulation::Trpltongue => mxml_tech.triple_tongue.push(EmptyPlacement::default()),
        DataArticulation::Heel => mxml_tech.heel.push(Default::default()),
        DataArticulation::Toe => mxml_tech.toe.push(Default::default()),
        DataArticulation::Tap => mxml_tech.tap.push(Default::default()),
        DataArticulation::Flip => mxml_tech.flip.push(EmptyPlacement::default()),
        DataArticulation::Smear => mxml_tech.smear.push(EmptyPlacement::default()),
        // No direct MusicXML equivalent → other-articulation
        DataArticulation::AccInv => other_name = Some("acc-inv"),
        DataArticulation::AccLong => other_name = Some("acc-long"),
        DataArticulation::Rip => other_name = Some("rip"),
        DataArticulation::Longfall => other_name = Some("longfall"),
        DataArticulation::Bend => other_name = Some("bend"),
        DataArticulation::Shake => other_name = Some("shake"),
        DataArticulation::Damp => other_name = Some("damp"),
        DataArticulation::Dampall => other_name = Some("dampall"),
        DataArticulation::Lhpizz => other_name = Some("lhpizz"),
        DataArticulation::Dot => other_name = Some("dot"),
        DataArticulation::Stroke => other_name = Some("stroke"),
    }

    if mxml_artic != Articulations::default() {
        let notations = mxml_note.notations.get_or_insert_with(Notations::default);
        notations.articulations = Some(mxml_artic);
    }
    if !mxml_tech.is_empty() {
        let notations = mxml_note.notations.get_or_insert_with(Notations::default);
        notations.technical = Some(mxml_tech);
    }
    if let Some(name) = other_name {
        let notations = mxml_note.notations.get_or_insert_with(Notations::default);
        let artics = notations
            .articulations
            .get_or_insert_with(Articulations::default);
        artics.other_articulation.push(OtherArticulation {
            value: name.to_string(),
            ..Default::default()
        });
    }
}

/// Restore breath-mark, caesura, other-articulation from MEI note label (fallback).
fn convert_mei_note_label_articulations(
    mei_note: &tusk_model::elements::Note,
    mxml_note: &mut crate::model::note::Note,
) {
    let label = match mei_note.common.label.as_deref() {
        Some(l) => l,
        None => return,
    };

    use crate::model::notations::{
        Articulations, BreathMark, Caesura, Notations, OtherArticulation,
    };

    for segment in label.split('|') {
        if let Some(json) = segment.strip_prefix("musicxml:breath-mark,") {
            if let Ok(bm) = serde_json::from_str::<BreathMark>(json) {
                let notations = mxml_note.notations.get_or_insert_with(Notations::default);
                let artics = notations
                    .articulations
                    .get_or_insert_with(Articulations::default);
                artics.breath_mark = Some(bm);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:caesura,") {
            if let Ok(cs) = serde_json::from_str::<Caesura>(json) {
                let notations = mxml_note.notations.get_or_insert_with(Notations::default);
                let artics = notations
                    .articulations
                    .get_or_insert_with(Articulations::default);
                artics.caesura = Some(cs);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:other-articulation,") {
            if let Ok(oa) = serde_json::from_str::<OtherArticulation>(json) {
                let notations = mxml_note.notations.get_or_insert_with(Notations::default);
                let artics = notations
                    .articulations
                    .get_or_insert_with(Articulations::default);
                artics.other_articulation.push(oa);
            }
        }
    }
}

/// Restore note-level instrument references from ExtensionStore or MEI note label.
fn convert_mei_note_label_instruments(
    mei_note: &tusk_model::elements::Note,
    mxml_note: &mut crate::model::note::Note,
    ctx: &ConversionContext,
) {
    // Preferred: read from ExtensionStore
    if let Some(id) = &mei_note.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref extras) = ext.note_extras {
                if !extras.instruments.is_empty() {
                    for inst_id in &extras.instruments {
                        mxml_note
                            .instruments
                            .push(crate::model::note::Instrument::new(inst_id));
                    }
                    return;
                }
            }
        }
    }

    // Fallback: read from label
    let label = match mei_note.common.label.as_deref() {
        Some(l) => l,
        None => return,
    };
    for segment in label.split('|') {
        if let Some(ids_str) = segment.strip_prefix("musicxml:instruments,") {
            for id in ids_str.split(',') {
                if !id.is_empty() {
                    mxml_note
                        .instruments
                        .push(crate::model::note::Instrument::new(id));
                }
            }
        }
    }
}

/// Restore notehead, notehead-text, play, listen, footnote, level, visual from
/// ExtensionStore or MEI note label.
fn restore_note_label_elements(
    mei_note: &tusk_model::elements::Note,
    mxml_note: &mut crate::model::note::Note,
    ctx: &ConversionContext,
) {
    // Preferred: read typed data from ExtensionStore
    if let Some(id) = &mei_note.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            let mut found = false;

            // NoteExtras (notehead, play, listen, footnote, level, etc.)
            if let Some(ref extras) = ext.note_extras {
                if let Some(ref val) = extras.notehead {
                    if let Ok(nh) =
                        serde_json::from_value::<crate::model::note::Notehead>(val.clone())
                    {
                        mxml_note.notehead = Some(nh);
                        found = true;
                    }
                }
                if let Some(ref val) = extras.notehead_text {
                    if let Ok(nht) =
                        serde_json::from_value::<crate::model::note::NoteheadText>(val.clone())
                    {
                        mxml_note.notehead_text = Some(nht);
                        found = true;
                    }
                }
                if let Some(ref play_data) = extras.play {
                    if let Ok(val) = serde_json::to_value(play_data) {
                        if let Ok(play) =
                            serde_json::from_value::<crate::model::direction::Play>(val)
                        {
                            mxml_note.play = Some(play);
                            found = true;
                        }
                    }
                }
                if let Some(ref val) = extras.listen {
                    if let Ok(listen) =
                        serde_json::from_value::<crate::model::listening::Listen>(val.clone())
                    {
                        mxml_note.listen = Some(listen);
                        found = true;
                    }
                }
                if let Some(ref val) = extras.footnote {
                    if let Ok(ft) =
                        serde_json::from_value::<crate::model::note::FormattedText>(val.clone())
                    {
                        mxml_note.footnote = Some(ft);
                        found = true;
                    }
                }
                if let Some(ref val) = extras.level {
                    if let Ok(lv) = serde_json::from_value::<crate::model::note::Level>(val.clone())
                    {
                        mxml_note.level = Some(lv);
                        found = true;
                    }
                }
                if let Some(ref val) = extras.notations_footnote {
                    if let Ok(ft) =
                        serde_json::from_value::<crate::model::note::FormattedText>(val.clone())
                    {
                        let notations = mxml_note
                            .notations
                            .get_or_insert_with(crate::model::notations::Notations::default);
                        notations.footnote = Some(ft);
                        found = true;
                    }
                }
                if let Some(ref val) = extras.notations_level {
                    if let Ok(lv) = serde_json::from_value::<crate::model::note::Level>(val.clone())
                    {
                        let notations = mxml_note
                            .notations
                            .get_or_insert_with(crate::model::notations::Notations::default);
                        notations.level = Some(lv);
                        found = true;
                    }
                }
            }

            // NoteVisualData
            if let Some(ref _nvd) = ext.note_visual {
                // Visual data exists; reconstruct NoteVisualAttrs from label for now
                // (the typed NoteVisualData uses different field types than NoteVisualAttrs)
            }

            if found {
                // Also try visual from label (NoteVisualData→NoteVisualAttrs conversion
                // requires going through JSON anyway, so just parse label segment)
                if let Some(label) = mei_note.common.label.as_deref() {
                    for segment in label.split('|') {
                        if let Some(json) = segment.strip_prefix("musicxml:visual,") {
                            if let Ok(vis) =
                                serde_json::from_str::<crate::model::note::NoteVisualAttrs>(json)
                            {
                                vis.apply_to_note(mxml_note);
                            }
                        }
                    }
                }
                return;
            }
        }
    }

    // Fallback: read from label segments
    let label = match mei_note.common.label.as_deref() {
        Some(l) => l,
        None => return,
    };

    for segment in label.split('|') {
        if let Some(json) = segment.strip_prefix("musicxml:notehead,") {
            if let Ok(nh) = serde_json::from_str::<crate::model::note::Notehead>(json) {
                mxml_note.notehead = Some(nh);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:notehead-text,") {
            if let Ok(nht) = serde_json::from_str::<crate::model::note::NoteheadText>(json) {
                mxml_note.notehead_text = Some(nht);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:play,") {
            if let Ok(play) = serde_json::from_str::<crate::model::direction::Play>(json) {
                mxml_note.play = Some(play);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:listen,") {
            if let Ok(listen) = serde_json::from_str::<crate::model::listening::Listen>(json) {
                mxml_note.listen = Some(listen);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:footnote,") {
            if let Ok(ft) = serde_json::from_str::<crate::model::note::FormattedText>(json) {
                mxml_note.footnote = Some(ft);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:level,") {
            if let Ok(lv) = serde_json::from_str::<crate::model::note::Level>(json) {
                mxml_note.level = Some(lv);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:notations-footnote,") {
            if let Ok(ft) = serde_json::from_str::<crate::model::note::FormattedText>(json) {
                let notations = mxml_note
                    .notations
                    .get_or_insert_with(crate::model::notations::Notations::default);
                notations.footnote = Some(ft);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:notations-level,") {
            if let Ok(lv) = serde_json::from_str::<crate::model::note::Level>(json) {
                let notations = mxml_note
                    .notations
                    .get_or_insert_with(crate::model::notations::Notations::default);
                notations.level = Some(lv);
            }
        } else if let Some(json) = segment.strip_prefix("musicxml:visual,") {
            if let Ok(vis) = serde_json::from_str::<crate::model::note::NoteVisualAttrs>(json) {
                vis.apply_to_note(mxml_note);
            }
        }
    }
}

/// Convert MEI @tie attribute to MusicXML <tie> and <tied> elements.
///
/// MEI uses `@tie` with values "i" (initial), "m" (medial), "t" (terminal).
/// MusicXML uses `<tie type="start|stop">` elements on the note, plus
/// `<tied type="start|stop">` in notations for visual representation.
fn convert_mei_ties(
    mei_note: &tusk_model::elements::Note,
    mxml_note: &mut crate::model::note::Note,
) {
    use crate::model::StartStop;
    use crate::model::notations::{Notations, Tied, TiedType};
    use crate::model::note::Tie;

    // MEI @tie e.g. "i", "m", "t"
    if let Some(ref tie_val) = mei_note.note_anl.tie {
        let val = tie_val.0.as_str();
        match val {
            "i" => {
                // Initial: tie starts here
                mxml_note.ties.push(Tie {
                    tie_type: StartStop::Start,
                    time_only: None,
                });
                let notations = mxml_note.notations.get_or_insert_with(Notations::default);
                notations.tied.push(Tied::new(TiedType::Start));
            }
            "m" => {
                // Medial: tie continues (both stop from previous and start to next)
                mxml_note.ties.push(Tie {
                    tie_type: StartStop::Stop,
                    time_only: None,
                });
                mxml_note.ties.push(Tie {
                    tie_type: StartStop::Start,
                    time_only: None,
                });
                let notations = mxml_note.notations.get_or_insert_with(Notations::default);
                notations.tied.push(Tied::new(TiedType::Stop));
                notations.tied.push(Tied::new(TiedType::Start));
            }
            "t" => {
                // Terminal: tie ends here
                mxml_note.ties.push(Tie {
                    tie_type: StartStop::Stop,
                    time_only: None,
                });
                let notations = mxml_note.notations.get_or_insert_with(Notations::default);
                notations.tied.push(Tied::new(TiedType::Stop));
            }
            _ => {}
        }
    }
}

/// Add warnings for MEI attributes that are lost in conversion.
fn add_note_conversion_warnings(
    mei_note: &tusk_model::elements::Note,
    ctx: &mut ConversionContext,
) {
    // Warn about analytical attributes (100% loss), but skip tie and artic (handled above)
    {
        let mut check_anl = mei_note.note_anl.clone();
        check_anl.tie = None; // tie is converted, not lost
        check_anl.artic = None; // artic is not an analytical loss warning
        if check_anl != tusk_model::att::AttNoteAnl::default() {
            ctx.add_warning(
                "note",
                "MEI analytical attributes (@pclass, @deg, etc.) have no MusicXML equivalent",
            );
        }
    }

    // Warn about gestural attributes that aren't mapped
    if mei_note.note_ges.vel.is_some() {
        ctx.add_warning(
            "note",
            "MEI @vel (velocity) attribute is lost in MusicXML conversion",
        );
    }

    // NoteChild in generated model: Stem, Plica, Accid (if added); editorial variants skipped
}

// ============================================================================
// MEI Verse/Syl → MusicXML Lyric Conversion
// ============================================================================

/// Convert MEI verse/syl children to MusicXML lyrics.
///
/// Decodes roundtrip label from verse `@label` (format: `musicxml:lyric,...`).
fn convert_mei_lyrics(
    mei_note: &tusk_model::elements::Note,
    mxml_note: &mut crate::model::note::Note,
    ctx: &ConversionContext,
) {
    use crate::model::lyric::*;
    use tusk_model::elements::{NoteChild, SylChild, VerseChild};

    for child in &mei_note.children {
        let verse = match child {
            NoteChild::Verse(v) => v,
            _ => continue,
        };

        // Preferred: reconstruct full Lyric from ExtensionStore
        if let Some(note_id) = &mei_note.common.xml_id {
            let verse_key = match &verse.common.n {
                Some(n) => format!("{}_v{}", note_id, n.0),
                None => format!("{}_v", note_id),
            };
            if let Some(ext) = ctx.ext_store().get(&verse_key) {
                if let Some(ref le) = ext.lyric_extras {
                    if let Ok(lyric) =
                        serde_json::from_value::<crate::model::lyric::Lyric>(le.lyric.clone())
                    {
                        mxml_note.lyrics.push(lyric);
                        continue;
                    }
                }
            }
        }

        // Fallback: parse label
        let label = verse.common.label.as_deref().unwrap_or("");
        if !label.starts_with("musicxml:lyric") {
            continue;
        }

        let label_parts: Vec<&str> = if label.len() > "musicxml:lyric,".len() {
            label["musicxml:lyric,".len()..].split(',').collect()
        } else {
            Vec::new()
        };

        // Helper to find label value
        let label_val = |key: &str| -> Option<&str> {
            label_parts
                .iter()
                .find_map(|p| p.strip_prefix(key).and_then(|rest| rest.strip_prefix('=')))
        };
        let label_has = |key: &str| -> bool { label_parts.contains(&key) };

        // Reconstruct lyric number from verse @n
        let number = verse.common.n.as_ref().map(|n| n.0.clone());

        // Decode MusicXML-specific attributes from label
        let name = label_val("name").map(String::from);
        let justify = label_val("justify").and_then(|s| match s {
            "left" => Some(crate::model::data::LeftCenterRight::Left),
            "center" => Some(crate::model::data::LeftCenterRight::Center),
            "right" => Some(crate::model::data::LeftCenterRight::Right),
            _ => None,
        });
        let default_x = label_val("default-x").and_then(|s| s.parse().ok());
        let default_y = label_val("default-y").and_then(|s| s.parse().ok());
        let relative_x = label_val("relative-x").and_then(|s| s.parse().ok());
        let relative_y = label_val("relative-y").and_then(|s| s.parse().ok());
        let placement = label_val("placement").and_then(|s| match s {
            "above" => Some(crate::model::data::AboveBelow::Above),
            "below" => Some(crate::model::data::AboveBelow::Below),
            _ => None,
        });
        let color = label_val("color").map(String::from);
        let print_object = label_val("print-object").and_then(|s| match s {
            "yes" => Some(crate::model::data::YesNo::Yes),
            "no" => Some(crate::model::data::YesNo::No),
            _ => None,
        });
        let time_only = label_val("time-only").map(String::from);
        let id = label_val("id").map(String::from);
        let end_line = label_has("end-line");
        let end_paragraph = label_has("end-paragraph");

        // Determine content type
        let content = if label_has("laughing") {
            LyricContent::Laughing
        } else if label_has("humming") {
            LyricContent::Humming
        } else if let Some(ext_type) = label_val("extend-only") {
            LyricContent::ExtendOnly(Extend {
                extend_type: parse_ssc(ext_type),
            })
        } else if label_has("extend-only") {
            LyricContent::ExtendOnly(Extend { extend_type: None })
        } else {
            // Text content from syl children
            let mut syllable_groups = Vec::new();
            let mut elision_values: Vec<&str> = label_parts
                .iter()
                .filter_map(|p| p.strip_prefix("elision="))
                .collect();

            let syls: Vec<&tusk_model::elements::Syl> = verse
                .children
                .iter()
                .filter_map(|c| match c {
                    VerseChild::Syl(s) => Some(s.as_ref()),
                    _ => None,
                })
                .collect();

            for (i, syl) in syls.iter().enumerate() {
                // Extract text from syl children
                let text_value = syl
                    .children
                    .iter()
                    .map(|c| match c {
                        SylChild::Text(t) => t.as_str(),
                    })
                    .collect::<Vec<_>>()
                    .join("");

                // Reconstruct syllabic from @wordpos + @con
                let syllabic = match syl.syl_log.wordpos.as_deref() {
                    Some("i") => Some(Syllabic::Begin),
                    Some("m") => Some(Syllabic::Middle),
                    Some("t") => Some(Syllabic::End),
                    _ => None, // No wordpos = single (implicit)
                };

                // Elision: if this is the 2nd+ syl, pop an elision value
                let elision = if i > 0 && !elision_values.is_empty() {
                    let val = elision_values.remove(0);
                    Some(Elision {
                        value: val.to_string(),
                        font_family: None,
                        font_size: None,
                        font_style: None,
                        font_weight: None,
                        color: None,
                    })
                } else {
                    None
                };

                syllable_groups.push(SyllableGroup {
                    elision,
                    syllabic,
                    text: LyricText {
                        value: text_value,
                        font_family: None,
                        font_size: None,
                        font_style: None,
                        font_weight: None,
                        color: None,
                    },
                });
            }

            // Reconstruct extend
            let extend = if let Some(ext_type) = label_val("extend") {
                Some(Extend {
                    extend_type: parse_ssc(ext_type),
                })
            } else if label_has("extend") {
                Some(Extend { extend_type: None })
            } else {
                None
            };

            LyricContent::Text {
                syllable_groups,
                extend,
            }
        };

        let lyric = Lyric {
            number,
            name,
            justify,
            default_x,
            default_y,
            relative_x,
            relative_y,
            placement,
            color,
            print_object,
            time_only,
            id,
            content,
            end_line,
            end_paragraph,
            footnote: None,
            level: None,
        };

        mxml_note.lyrics.push(lyric);
    }
}

/// Parse start/stop/continue string.
fn parse_ssc(s: &str) -> Option<crate::model::StartStopContinue> {
    match s {
        "start" => Some(crate::model::StartStopContinue::Start),
        "stop" => Some(crate::model::StartStopContinue::Stop),
        "continue" => Some(crate::model::StartStopContinue::Continue),
        _ => None,
    }
}

// ============================================================================
// MEI Rest → MusicXML Rest Conversion
// ============================================================================

/// Convert an MEI rest to a MusicXML note containing a rest.
///
/// This converts an MEI rest element to MusicXML. In MusicXML, rests are
/// represented as `<note>` elements containing a `<rest>` child rather than
/// pitch information.
///
/// # Conversion Details
///
/// - Duration: MEI `@dur` → MusicXML `<type>` (graphical); calculated duration in divisions
/// - Dots: MEI `@dots` → MusicXML `<dot>` elements
/// - Cue rests: MEI `@cue` → MusicXML `<cue>` element
/// - IDs: MEI `xml:id` → MusicXML `@id` attribute
///
/// # Lossy Conversion Notes
///
/// The following MEI attributes are lost in conversion:
/// - Timing attributes (@tstamp, @tstamp.ges, @tstamp.real) - MusicXML uses position in measure
/// - Staff/layer positioning (@staff, @layer) - determined by measure/note sequence
/// - Analytical attributes - no MusicXML equivalent
/// - Editorial child elements (app, choice, etc.) - no MusicXML equivalent
/// - Facsimile references (@facs) - no MusicXML equivalent
///
/// # Arguments
///
/// * `mei_rest` - The MEI rest to convert
/// * `ctx` - The conversion context for state tracking
///
/// # Returns
///
/// A MusicXML Note element containing a Rest, or an error if conversion fails.
pub fn convert_mei_rest(
    mei_rest: &tusk_model::elements::Rest,
    ctx: &mut ConversionContext,
) -> ConversionResult<crate::model::note::Note> {
    use crate::model::elements::Empty;
    use crate::model::note::{Dot, Note as MxmlNote, NoteType, Rest as MxmlRest};

    // Calculate duration from MEI rest attributes
    let duration = calculate_mei_rest_duration(mei_rest, ctx);

    // Create the MusicXML rest element
    let mxml_rest = MxmlRest::new();

    // Create the MusicXML note containing the rest
    let mut mxml_note = MxmlNote::rest(mxml_rest, duration);

    // Set ID from xml:id
    if let Some(ref xml_id) = mei_rest.common.xml_id {
        mxml_note.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Convert note type (graphical duration)
    if let Some(ref dur) = mei_rest.rest_log.dur {
        if let Some(nv) = convert_mei_duration_rests_to_note_type(dur) {
            mxml_note.note_type = Some(NoteType::new(nv));
        }
    }

    // Convert dots - check both @dots attribute and <dot> children
    let dot_count = get_mei_rest_dot_count(mei_rest);
    for _ in 0..dot_count {
        mxml_note.dots.push(Dot::default());
    }

    // Convert cue rest
    if mei_rest.rest_log.cue.as_ref() == Some(&tusk_model::data::DataBoolean::True) {
        mxml_note.cue = Some(Empty);
    }

    // Add warnings for lossy attributes
    add_rest_conversion_warnings(mei_rest, ctx);

    Ok(mxml_note)
}

/// Calculate MEI rest duration in MusicXML divisions.
fn calculate_mei_rest_duration(
    mei_rest: &tusk_model::elements::Rest,
    ctx: &ConversionContext,
) -> f64 {
    // First check if we have gestural duration in ppq (most accurate); MEI @dur.ppq is Option<String>
    if let Some(ref dur_ppq) = mei_rest.rest_ges.dur_ppq {
        if let Ok(n) = dur_ppq.parse::<f64>() {
            return n;
        }
    }

    // Calculate from written duration
    let divisions = ctx.divisions();
    let base_duration = if let Some(ref dur) = mei_rest.rest_log.dur {
        duration_rests_to_quarter_notes(dur)
    } else {
        1.0 // Default to quarter note
    };

    // Apply dots
    let dot_count = get_mei_rest_dot_count(mei_rest);

    let dotted_duration = apply_dots(base_duration, dot_count);

    // Convert to divisions
    dotted_duration * divisions
}

/// Get the total dot count from MEI rest (both @dots attribute and <dot> children).
fn get_mei_rest_dot_count(mei_rest: &tusk_model::elements::Rest) -> u64 {
    use tusk_model::elements::RestChild;

    // First check the @dots attribute
    if let Some(ref dots) = mei_rest.rest_log.dots {
        return dots.to_string().parse::<u64>().unwrap_or(0);
    }

    // Count <dot> children as fallback
    mei_rest
        .children
        .iter()
        .filter(|c| matches!(c, RestChild::Dot(_)))
        .count() as u64
}

/// Add warnings for MEI rest attributes that are lost in conversion.
fn add_rest_conversion_warnings(
    mei_rest: &tusk_model::elements::Rest,
    ctx: &mut ConversionContext,
) {
    // Warn about timing attributes (100% loss)
    if mei_rest.rest_log.tstamp.is_some()
        || mei_rest.rest_log.tstamp_ges.is_some()
        || mei_rest.rest_log.tstamp_real.is_some()
    {
        ctx.add_warning(
            "rest",
            "MEI timing attributes (@tstamp, @tstamp.ges, @tstamp.real) are lost in MusicXML conversion",
        );
    }

    // Warn about staff/layer positioning (position determined by sequence in MusicXML)
    if mei_rest
        .rest_log
        .staff
        .as_ref()
        .is_some_and(|s| !s.is_empty())
        || mei_rest
            .rest_log
            .layer
            .as_ref()
            .is_some_and(|s| !s.is_empty())
    {
        ctx.add_warning(
            "rest",
            "MEI @staff/@layer attributes are not directly mapped; position in MusicXML is determined by sequence",
        );
    }

    // Warn about facsimile links; @facs is Option<String> or list
    if mei_rest
        .facsimile
        .facs
        .as_ref()
        .is_some_and(|s| !s.0.is_empty())
    {
        ctx.add_warning(
            "rest",
            "MEI @facs (facsimile link) has no MusicXML equivalent",
        );
    }

    // Warn about analytical attributes
    if mei_rest.rest_anl != tusk_model::att::AttRestAnl::default() {
        ctx.add_warning(
            "rest",
            "MEI analytical attributes have no MusicXML equivalent",
        );
    }

    // RestChild in generated model: Dot; editorial variants skipped

    // Warn about mensural durations
    if mei_rest.rest_log.dur.as_ref().is_some_and(|d| {
        matches!(
            d,
            tusk_model::data::DataDurationrests::MeiDataDurationrestsMensural(_)
        )
    }) {
        ctx.add_warning(
            "rest",
            "MEI mensural rest duration has no direct MusicXML equivalent",
        );
    }
}

// ============================================================================
// MEI Measure Rest → MusicXML Measure Rest Conversion
// ============================================================================

/// Convert an MEI mRest (measure rest) to a MusicXML note containing a measure rest.
///
/// This converts an MEI `<mRest>` element to MusicXML. In MusicXML, measure rests
/// are represented as `<note>` elements containing a `<rest measure="yes"/>` child.
///
/// # Arguments
///
/// * `mei_mrest` - The MEI mRest element to convert
/// * `ctx` - The conversion context for state tracking
///
/// # Returns
///
/// A MusicXML Note element containing a measure rest, or an error if conversion fails.
pub fn convert_mei_mrest(
    mei_mrest: &tusk_model::elements::MRest,
    ctx: &mut ConversionContext,
) -> ConversionResult<crate::model::note::Note> {
    use crate::model::note::{Note as MxmlNote, Rest as MxmlRest};

    // Calculate duration from MEI mRest attributes
    let duration = calculate_mei_mrest_duration(mei_mrest, ctx);

    // Create a MusicXML measure rest
    let mxml_rest = MxmlRest::measure_rest();

    // Create the MusicXML note containing the rest
    let mut mxml_note = MxmlNote::rest(mxml_rest, duration);

    // Set ID from xml:id
    if let Some(ref xml_id) = mei_mrest.common.xml_id {
        mxml_note.id = Some(xml_id.clone());
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Measure rests typically don't have a note type in MusicXML

    Ok(mxml_note)
}

/// Calculate MEI mRest duration in MusicXML divisions.
fn calculate_mei_mrest_duration(
    mei_mrest: &tusk_model::elements::MRest,
    ctx: &ConversionContext,
) -> f64 {
    // First check if we have gestural duration in ppq (most accurate); MEI @dur.ppq is Option<String>
    if let Some(ref dur_ppq) = mei_mrest.m_rest_ges.dur_ppq {
        if let Ok(n) = dur_ppq.parse::<f64>() {
            return n;
        }
    }

    // Default: return the divisions value (as mRests typically span a full measure)
    // The actual duration depends on the time signature, which should be encoded in dur.ppq
    let divisions = ctx.divisions();
    if divisions > 0.0 {
        // Default to 4 beats (whole measure in 4/4)
        divisions * 4.0
    } else {
        4.0
    }
}

// ============================================================================
// MEI Chord to MusicXML Conversion
// ============================================================================

/// Convert an MEI chord to a sequence of MusicXML notes.
///
/// In MusicXML, chords are represented as a sequence of `<note>` elements where
/// all notes after the first have a `<chord/>` child element, indicating they
/// share timing with the previous note.
///
/// # Conversion Details
///
/// - Each MEI note child becomes a MusicXML note
/// - The first MusicXML note does NOT have a chord flag
/// - Subsequent MusicXML notes have the `<chord/>` element
/// - Duration, dots, grace, and cue attributes from the chord apply to all notes
/// - Stem direction from the chord is applied to the first note only
/// - Individual note pitches and accidentals are preserved
///
/// # Lossy Conversion Notes
///
/// The following MEI chord attributes are lost in conversion:
/// - Timing attributes (@tstamp, @tstamp.ges, @tstamp.real) - MusicXML uses position
/// - Staff/layer positioning (@staff, @layer) - determined by sequence
/// - Analytical attributes (@chord.anl) - no MusicXML equivalent
/// - Visual attributes beyond stem direction - partial support
/// - Editorial child elements (app, choice, etc.) - no MusicXML equivalent
/// - Artic children (articulations on chord level)
///
/// # Arguments
///
/// * `mei_chord` - The MEI chord to convert
/// * `ctx` - The conversion context for state tracking
///
/// # Returns
///
/// A vector of MusicXML Note elements representing the chord, or an error if
/// conversion fails. The first note has no chord flag; subsequent notes have it.
pub fn convert_mei_chord(
    mei_chord: &tusk_model::elements::Chord,
    ctx: &mut ConversionContext,
) -> ConversionResult<Vec<crate::model::note::Note>> {
    use crate::model::elements::Empty;
    use crate::model::note::{Dot, Note as MxmlNote, NoteType, Stem};
    use tusk_model::elements::ChordChild;

    // Collect note children from the chord
    let mei_notes: Vec<&tusk_model::elements::Note> = mei_chord
        .children
        .iter()
        .map(|child| {
            let ChordChild::Note(note) = child;
            note.as_ref()
        })
        .collect();

    // Return empty vec if no notes
    if mei_notes.is_empty() {
        return Ok(Vec::new());
    }

    // Determine if this is a grace chord
    let is_grace = mei_chord.chord_log.grace.is_some();

    // Calculate chord duration from chord attributes
    let chord_duration = if is_grace {
        None
    } else {
        Some(calculate_mei_chord_duration(mei_chord, ctx))
    };

    // Get dots from chord level
    let chord_dot_count = mei_chord
        .chord_log
        .dots
        .as_ref()
        .map(|d| d.to_string().parse::<u64>().unwrap_or(0))
        .unwrap_or(0);

    // Get note type from chord duration
    let chord_note_type = mei_chord
        .chord_log
        .dur
        .as_ref()
        .map(convert_mei_duration_to_note_type);

    // Check for cue chord
    let is_cue = mei_chord.chord_log.cue.as_ref() == Some(&tusk_model::data::DataBoolean::True);

    let mut mxml_notes = Vec::with_capacity(mei_notes.len());

    for (i, mei_note) in mei_notes.iter().enumerate() {
        // Check if this note is unpitched (no pname = percussion)
        let is_unpitched = mei_note.note_log.pname.is_none();

        // Check if this note has its own duration that differs from the chord level
        // (MusicXML "multiple stop" notation: chord notes with different written durations)
        let note_has_own_duration = mei_note.note_ges.dur_ppq.is_some()
            && mei_note.note_ges.dur_ppq != mei_chord.chord_ges.dur_ppq;

        let note_duration = if note_has_own_duration {
            Some(calculate_mei_note_duration(mei_note, ctx))
        } else {
            chord_duration
        };

        // Create the MusicXML note
        let mut mxml_note = if is_unpitched {
            // Unpitched note in chord
            let unpitched = convert_mei_loc_to_unpitched(mei_note);
            if is_grace {
                let grace = convert_mei_grace_chord(mei_chord);
                MxmlNote::unpitched_grace(unpitched, grace)
            } else {
                MxmlNote::unpitched(unpitched, note_duration.unwrap_or(0.0))
            }
        } else {
            // Pitched note in chord
            let pitch = convert_mei_pitch(mei_note, ctx)?;
            if is_grace {
                let grace = convert_mei_grace_chord(mei_chord);
                MxmlNote::grace_note(pitch, grace)
            } else {
                MxmlNote::pitched(pitch, note_duration.unwrap_or(0.0))
            }
        };

        // Set chord flag for all notes except the first
        if i > 0 {
            mxml_note.chord = Some(Empty);
        }

        // Set note type and dots: use individual note's values if it has its own duration,
        // otherwise use chord-level values
        if note_has_own_duration {
            if let Some(ref dur) = mei_note.note_log.dur {
                mxml_note.note_type = Some(NoteType::new(convert_mei_duration_to_note_type(dur)));
            }
            let note_dot_count = mei_note.note_log.dots.as_ref().map(|d| d.0).unwrap_or(0);
            for _ in 0..note_dot_count {
                mxml_note.dots.push(Dot::default());
            }
        } else {
            if let Some(ref note_type) = chord_note_type {
                mxml_note.note_type = Some(NoteType::new(*note_type));
            }
            for _ in 0..chord_dot_count {
                mxml_note.dots.push(Dot::default());
            }
        }

        // Set cue if chord is cue
        if is_cue {
            mxml_note.cue = Some(Empty);
        }

        // Set stem direction on first note only (chord stem applies to all notes visually)
        // Check ext_store/label for special stems (Double/None), then chord-level, then note-level
        if i == 0 {
            let stem_from_ext = mei_note.common.xml_id.as_ref().and_then(|id| {
                ctx.ext_store()
                    .get(id)?
                    .stem_extras
                    .as_ref()
                    .map(|se| match se {
                        tusk_model::musicxml_ext::StemExtras::Double => {
                            crate::model::note::StemValue::Double
                        }
                        tusk_model::musicxml_ext::StemExtras::None => {
                            crate::model::note::StemValue::None
                        }
                    })
            });
            let chord_stem_from_label = stem_from_ext.or_else(|| {
                mei_note.common.label.as_deref().and_then(|l| {
                    l.split('|').find_map(|seg| {
                        seg.strip_prefix("musicxml:stem,").map(|v| match v {
                            "double" => crate::model::note::StemValue::Double,
                            "none" => crate::model::note::StemValue::None,
                            _ => crate::model::note::StemValue::Up,
                        })
                    })
                })
            });
            if let Some(sv) = chord_stem_from_label {
                mxml_note.stem = Some(Stem::new(sv));
            } else if let Some(ref stem_dir) = mei_chord.chord_vis.stem_dir {
                mxml_note.stem = Some(Stem::new(convert_mei_stem_direction(stem_dir)));
            } else if let Some(ref stem_dir) = mei_note.note_vis.stem_dir {
                mxml_note.stem = Some(Stem::new(convert_mei_stem_direction(stem_dir)));
            }
        }

        // Set ID from individual note's xml:id (needed for tie/slur references)
        if let Some(ref xml_id) = mei_note.common.xml_id {
            mxml_note.id = Some(xml_id.clone());
            ctx.map_id(xml_id, xml_id.clone());
        }

        // Handle individual note's written accidental
        for child in &mei_note.children {
            if let tusk_model::elements::NoteChild::Accid(accid) = child {
                mxml_note.accidental = Some(convert_mei_accid_to_mxml(accid, ctx)?);
            }
        }

        // Convert articulations from ExtensionStore (full data) or MEI @artic (single value)
        convert_mei_articulations(mei_note, &mut mxml_note, ctx);

        // Apply chord-level articulations to the first note (fallback for non-roundtrip MEI)
        if i == 0 {
            convert_mei_artic_single(mei_chord.chord_log.artic.as_ref(), &mut mxml_note);
        }

        // Convert ties from MEI @tie attribute to MusicXML <tie> elements
        convert_mei_ties(mei_note, &mut mxml_note);

        // Convert verse/syl children to MusicXML lyrics
        convert_mei_lyrics(mei_note, &mut mxml_note, ctx);

        // Restore note-level instrument references from label
        convert_mei_note_label_instruments(mei_note, &mut mxml_note, ctx);

        // Restore notehead, notehead-text, play, listen, footnote, level from label
        restore_note_label_elements(mei_note, &mut mxml_note, ctx);

        mxml_notes.push(mxml_note);
    }

    // Map the chord ID if present
    if let Some(ref xml_id) = mei_chord.common.xml_id {
        ctx.map_id(xml_id, xml_id.clone());
    }

    // Add warnings for lossy attributes
    add_chord_conversion_warnings(mei_chord, ctx);

    Ok(mxml_notes)
}

/// Calculate MEI chord duration in MusicXML divisions.
fn calculate_mei_chord_duration(
    mei_chord: &tusk_model::elements::Chord,
    ctx: &ConversionContext,
) -> f64 {
    // First check if we have gestural duration in ppq (most accurate); MEI @dur.ppq is Option<String>
    if let Some(ref dur_ppq) = mei_chord.chord_ges.dur_ppq {
        if let Ok(n) = dur_ppq.parse::<f64>() {
            return n;
        }
    }

    // Calculate from written duration
    let divisions = ctx.divisions();
    let base_duration = if let Some(ref dur) = mei_chord.chord_log.dur {
        duration_to_quarter_notes(dur)
    } else {
        1.0 // Default to quarter note
    };

    // Apply dots
    let dot_count = mei_chord.chord_log.dots.as_ref().map(|d| d.0).unwrap_or(0);

    let dotted_duration = apply_dots(base_duration, dot_count);

    // Convert to divisions
    dotted_duration * divisions
}

/// Convert MEI chord grace attribute to MusicXML Grace element.
fn convert_mei_grace_chord(mei_chord: &tusk_model::elements::Chord) -> crate::model::note::Grace {
    use crate::model::note::Grace;
    use tusk_model::data::DataGrace;

    let mut grace = Grace::default();

    if let Some(ref grace_type) = mei_chord.chord_log.grace {
        match grace_type {
            DataGrace::Acc => grace.slash = Some(crate::model::data::YesNo::Yes),
            DataGrace::Unacc => grace.slash = Some(crate::model::data::YesNo::No),
            _ => {}
        }
    }

    grace
}

/// Add warnings for MEI chord attributes that are lost in conversion.
fn add_chord_conversion_warnings(
    mei_chord: &tusk_model::elements::Chord,
    ctx: &mut ConversionContext,
) {
    // Warn about timing attributes (100% loss)
    if mei_chord.chord_log.tstamp.is_some()
        || mei_chord.chord_log.tstamp_ges.is_some()
        || mei_chord.chord_log.tstamp_real.is_some()
    {
        ctx.add_warning(
            "chord",
            "MEI timing attributes (@tstamp, @tstamp.ges, @tstamp.real) are lost in MusicXML conversion",
        );
    }

    // Warn about staff/layer positioning
    if mei_chord
        .chord_log
        .staff
        .as_ref()
        .is_some_and(|s| !s.is_empty())
        || mei_chord
            .chord_log
            .layer
            .as_ref()
            .is_some_and(|s| !s.is_empty())
    {
        ctx.add_warning(
            "chord",
            "MEI @staff/@layer attributes are not directly mapped; position in MusicXML is determined by sequence",
        );
    }

    // Warn about facsimile links
    if mei_chord
        .facsimile
        .facs
        .as_ref()
        .is_some_and(|s| !s.0.is_empty())
    {
        ctx.add_warning(
            "chord",
            "MEI @facs (facsimile link) has no MusicXML equivalent",
        );
    }

    // Warn about analytical attributes
    if mei_chord.chord_anl != tusk_model::att::AttChordAnl::default() {
        ctx.add_warning(
            "chord",
            "MEI analytical attributes have no MusicXML equivalent",
        );
    }

    // Warn about visual attributes beyond stem direction
    if mei_chord.chord_vis.color.is_some()
        || mei_chord.chord_vis.cluster.is_some()
        || mei_chord.chord_vis.stem_mod.is_some()
    {
        ctx.add_warning(
            "chord",
            "Some MEI visual attributes (color, cluster, stem.mod) have no direct MusicXML equivalent",
        );
    }

    // (chord-level artic is now exported on the first note)

    // ChordChild in generated model only has Note; editorial/artic/lyric children are skipped

    // Warn about mensural durations
    if mei_chord.chord_log.dur.as_ref().is_some_and(|d| {
        matches!(
            d,
            tusk_model::data::DataDuration::MeiDataDurationMensural(_)
        )
    }) {
        ctx.add_warning(
            "chord",
            "MEI mensural chord duration has no direct MusicXML equivalent",
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ConversionDirection;

    // ========================================================================
    // MEI Note → MusicXML Note Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_note_basic_pitch() {
        use crate::model::data::Step;
        use crate::model::note::FullNoteContent;
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(1.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check pitch
        if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
            assert_eq!(pitch.step, Step::C);
            assert_eq!(pitch.octave, 4);
            assert!(pitch.alter.is_none());
        } else {
            panic!("Expected pitched note");
        }
    }

    #[test]
    fn test_convert_mei_note_with_duration() {
        use crate::model::note::NoteTypeValue;
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("e".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(5u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N2)); // Half note

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0); // 4 divisions per quarter

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check note type
        assert!(mxml_note.note_type.is_some());
        assert_eq!(
            mxml_note.note_type.as_ref().unwrap().value,
            NoteTypeValue::Half
        );
        // Check duration in divisions (half note = 2 quarters = 8 divisions)
        assert!(mxml_note.duration.is_some());
        assert_eq!(mxml_note.duration.unwrap(), 8.0);
    }

    #[test]
    fn test_convert_mei_note_with_dots() {
        use tusk_model::data::{
            DataAugmentdot, DataDuration, DataDurationCmn, DataOctave, DataPitchname,
        };
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("g".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));
        mei_note.note_log.dots = Some(DataAugmentdot::from(1u64)); // Dotted quarter

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check dots
        assert_eq!(mxml_note.dots.len(), 1);
        // Dotted quarter = 1.5 quarters = 6 divisions
        assert_eq!(mxml_note.duration.unwrap(), 6.0);
    }

    #[test]
    fn test_convert_mei_note_with_accidental() {
        use crate::model::note::AccidentalValue;
        use tusk_model::data::{
            DataAccidentalWritten, DataAccidentalWrittenBasic, DataDuration, DataDurationCmn,
            DataOctave, DataPitchname,
        };
        use tusk_model::elements::{Accid, Note as MeiNote, NoteChild};

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("f".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

        // Add sharp accidental as child element
        let mut accid = Accid::default();
        accid.accid_log.accid = Some(DataAccidentalWritten::MeiDataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::S,
        ));
        mei_note.children.push(NoteChild::Accid(Box::new(accid)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check accidental
        assert!(mxml_note.accidental.is_some());
        assert_eq!(
            mxml_note.accidental.as_ref().unwrap().value,
            AccidentalValue::Sharp
        );
    }

    #[test]
    fn test_convert_mei_note_with_gestural_accidental() {
        use crate::model::note::FullNoteContent;
        use tusk_model::data::{
            DataAccidentalGestural, DataAccidentalGesturalBasic, DataDuration, DataDurationCmn,
            DataOctave, DataPitchname,
        };
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("b".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));
        // Set gestural accidental (sounding pitch is B flat)
        mei_note.note_ges.accid_ges = Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::F,
        ));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check pitch alter (gestural accidental → alter)
        if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
            assert_eq!(pitch.alter, Some(-1.0)); // Flat = -1
        } else {
            panic!("Expected pitched note");
        }
    }

    #[test]
    fn test_convert_mei_note_with_stem_direction() {
        use crate::model::note::StemValue;
        use tusk_model::data::{
            DataDuration, DataDurationCmn, DataOctave, DataPitchname, DataStemdirection,
            DataStemdirectionBasic,
        };
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("d".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(5u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));
        mei_note.note_vis.stem_dir = Some(DataStemdirection::MeiDataStemdirectionBasic(
            DataStemdirectionBasic::Down,
        ));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check stem direction
        assert!(mxml_note.stem.is_some());
        assert_eq!(mxml_note.stem.as_ref().unwrap().value, StemValue::Down);
    }

    #[test]
    fn test_convert_mei_note_grace() {
        use tusk_model::data::{
            DataDuration, DataDurationCmn, DataGrace, DataOctave, DataPitchname,
        };
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("a".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N8));
        mei_note.note_log.grace = Some(DataGrace::Unacc); // Unaccented/slashed grace note

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check grace note
        assert!(mxml_note.is_grace());
        assert!(mxml_note.grace.is_some());
        // Grace notes should not have duration
        assert!(mxml_note.duration.is_none());
        // Unaccented grace should have slash
        use crate::model::data::YesNo;
        assert_eq!(mxml_note.grace.as_ref().unwrap().slash, Some(YesNo::Yes));
    }

    #[test]
    fn test_convert_mei_note_cue() {
        use tusk_model::data::{
            DataBoolean, DataDuration, DataDurationCmn, DataOctave, DataPitchname,
        };
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.note_log.pname = Some(DataPitchname::from("c".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(5u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));
        mei_note.note_log.cue = Some(DataBoolean::True);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        assert!(mxml_note.is_cue());
    }

    #[test]
    fn test_convert_mei_note_with_id() {
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;

        let mut mei_note = MeiNote::default();
        mei_note.common.xml_id = Some("note-1".to_string());
        mei_note.note_log.pname = Some(DataPitchname::from("e".to_string()));
        mei_note.note_log.oct = Some(DataOctave::from(4u64));
        mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_note(&mei_note, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check ID mapping
        assert!(mxml_note.id.is_some());
        assert_eq!(mxml_note.id.as_ref().unwrap(), "note-1");
    }

    #[test]
    fn test_convert_mei_note_all_pitches() {
        use crate::model::data::Step;
        use crate::model::note::FullNoteContent;
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;

        let pitch_mappings = [
            ("c", Step::C),
            ("d", Step::D),
            ("e", Step::E),
            ("f", Step::F),
            ("g", Step::G),
            ("a", Step::A),
            ("b", Step::B),
        ];

        for (mei_pname, expected_step) in pitch_mappings {
            let mut mei_note = MeiNote::default();
            mei_note.note_log.pname = Some(DataPitchname::from(mei_pname.to_string()));
            mei_note.note_log.oct = Some(DataOctave::from(4u64));
            mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));

            let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
            ctx.set_divisions(4.0);

            let result = convert_mei_note(&mei_note, &mut ctx);
            assert!(result.is_ok(), "Failed for pitch {}", mei_pname);

            let mxml_note = result.unwrap();
            if let FullNoteContent::Pitch(pitch) = &mxml_note.content {
                assert_eq!(
                    pitch.step, expected_step,
                    "Pitch mismatch for {}",
                    mei_pname
                );
            } else {
                panic!("Expected pitched note for {}", mei_pname);
            }
        }
    }

    #[test]
    fn test_convert_mei_note_all_durations() {
        use crate::model::note::NoteTypeValue;
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::Note as MeiNote;

        let duration_mappings = [
            (DataDurationCmn::Breve, NoteTypeValue::Breve),
            (DataDurationCmn::N1, NoteTypeValue::Whole),
            (DataDurationCmn::N2, NoteTypeValue::Half),
            (DataDurationCmn::N4, NoteTypeValue::Quarter),
            (DataDurationCmn::N8, NoteTypeValue::Eighth),
            (DataDurationCmn::N16, NoteTypeValue::N16th),
            (DataDurationCmn::N32, NoteTypeValue::N32nd),
            (DataDurationCmn::N64, NoteTypeValue::N64th),
        ];

        for (mei_dur, expected_type) in duration_mappings {
            let mut mei_note = MeiNote::default();
            mei_note.note_log.pname = Some(DataPitchname::from("c".to_string()));
            mei_note.note_log.oct = Some(DataOctave::from(4u64));
            mei_note.note_log.dur = Some(DataDuration::MeiDataDurationCmn(mei_dur));

            let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
            ctx.set_divisions(4.0);

            let result = convert_mei_note(&mei_note, &mut ctx);
            assert!(result.is_ok(), "Failed for duration {:?}", mei_dur);

            let mxml_note = result.unwrap();
            assert!(
                mxml_note.note_type.is_some(),
                "No note type for {:?}",
                mei_dur
            );
            assert_eq!(
                mxml_note.note_type.as_ref().unwrap().value,
                expected_type,
                "Duration mismatch for {:?}",
                mei_dur
            );
        }
    }

    // ========================================================================
    // MEI Rest → MusicXML Rest Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_rest_basic() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check that it's a rest
        assert!(mxml_note.is_rest());
        // Check duration (quarter note = 1 quarter = 4 divisions)
        assert_eq!(mxml_note.duration, Some(4.0));
    }

    #[test]
    fn test_convert_mei_rest_with_duration_type() {
        use crate::model::note::NoteTypeValue;
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N2)); // Half rest

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check note type (graphical duration)
        assert!(mxml_note.note_type.is_some());
        assert_eq!(
            mxml_note.note_type.as_ref().unwrap().value,
            NoteTypeValue::Half
        );
        // Check duration in divisions (half rest = 2 quarters = 8 divisions)
        assert_eq!(mxml_note.duration, Some(8.0));
    }

    #[test]
    fn test_convert_mei_rest_with_dots() {
        use tusk_model::data::{DataAugmentdot, DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N4));
        mei_rest.rest_log.dots = Some(DataAugmentdot::from(1u64)); // Dotted quarter

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check dots
        assert_eq!(mxml_note.dots.len(), 1);
        // Dotted quarter = 1.5 quarters = 6 divisions
        assert_eq!(mxml_note.duration.unwrap(), 6.0);
    }

    #[test]
    fn test_convert_mei_rest_with_id() {
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.common.xml_id = Some("rest-1".to_string());
        mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N4));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check ID mapping
        assert!(mxml_note.id.is_some());
        assert_eq!(mxml_note.id.as_ref().unwrap(), "rest-1");
    }

    #[test]
    fn test_convert_mei_rest_cue() {
        use tusk_model::data::{DataBoolean, DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N4));
        mei_rest.rest_log.cue = Some(DataBoolean::True);

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        assert!(mxml_note.is_cue());
    }

    #[test]
    fn test_convert_mei_rest_with_dur_ppq() {
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        // Set gestural duration directly (12 ppq)
        mei_rest.rest_ges.dur_ppq = Some("12".to_string());

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0); // Even with divisions set, dur.ppq takes precedence

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Duration should be exactly the dur.ppq value
        assert_eq!(mxml_note.duration, Some(12.0));
    }

    #[test]
    fn test_convert_mei_rest_whole_rest() {
        use crate::model::note::NoteTypeValue;
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N1)); // Whole rest

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        assert!(mxml_note.is_rest());
        assert!(mxml_note.note_type.is_some());
        assert_eq!(
            mxml_note.note_type.as_ref().unwrap().value,
            NoteTypeValue::Whole
        );
        // Whole rest = 4 quarters = 16 divisions
        assert_eq!(mxml_note.duration, Some(16.0));
    }

    #[test]
    fn test_convert_mei_rest_all_durations() {
        use crate::model::note::NoteTypeValue;
        use tusk_model::data::{DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let duration_mappings = [
            (DataDurationCmn::Breve, NoteTypeValue::Breve),
            (DataDurationCmn::N1, NoteTypeValue::Whole),
            (DataDurationCmn::N2, NoteTypeValue::Half),
            (DataDurationCmn::N4, NoteTypeValue::Quarter),
            (DataDurationCmn::N8, NoteTypeValue::Eighth),
            (DataDurationCmn::N16, NoteTypeValue::N16th),
            (DataDurationCmn::N32, NoteTypeValue::N32nd),
            (DataDurationCmn::N64, NoteTypeValue::N64th),
        ];

        for (mei_dur, expected_type) in duration_mappings {
            let mut mei_rest = MeiRest::default();
            mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(mei_dur));

            let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
            ctx.set_divisions(4.0);

            let result = convert_mei_rest(&mei_rest, &mut ctx);
            assert!(result.is_ok(), "Failed for duration {:?}", mei_dur);

            let mxml_note = result.unwrap();
            assert!(
                mxml_note.note_type.is_some(),
                "No note type for {:?}",
                mei_dur
            );
            assert_eq!(
                mxml_note.note_type.as_ref().unwrap().value,
                expected_type,
                "Duration mismatch for {:?}",
                mei_dur
            );
        }
    }

    #[test]
    fn test_convert_mei_rest_double_dotted() {
        use tusk_model::data::{DataAugmentdot, DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N2));
        mei_rest.rest_log.dots = Some(DataAugmentdot::from(2u64)); // Double-dotted half

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Check dots
        assert_eq!(mxml_note.dots.len(), 2);
        // Double-dotted half = 2 + 1 + 0.5 = 3.5 quarters = 14 divisions
        assert_eq!(mxml_note.duration.unwrap(), 14.0);
    }

    #[test]
    fn test_convert_mei_rest_generates_warnings_for_timing() {
        use tusk_model::data::{DataBeat, DataDurationCmn, DataDurationrests};
        use tusk_model::elements::Rest as MeiRest;

        let mut mei_rest = MeiRest::default();
        mei_rest.rest_log.dur = Some(DataDurationrests::MeiDataDurationCmn(DataDurationCmn::N4));
        mei_rest.rest_log.tstamp = Some(DataBeat::from(1.0));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let _ = convert_mei_rest(&mei_rest, &mut ctx);

        // Should have warnings about timing attributes being lost
        assert!(ctx.has_warnings());
    }

    #[test]
    fn test_convert_mei_rest_default_duration() {
        use tusk_model::elements::Rest as MeiRest;

        // Rest with no duration specified
        let mei_rest = MeiRest::default();

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_rest(&mei_rest, &mut ctx);
        assert!(result.is_ok());

        let mxml_note = result.unwrap();
        // Should default to quarter note = 4 divisions
        assert_eq!(mxml_note.duration, Some(4.0));
    }

    // ========================================================================
    // MEI Chord to MusicXML Conversion Tests
    // ========================================================================

    #[test]
    fn test_convert_mei_chord_basic() {
        use tusk_model::data::{DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        // Create an MEI chord with two notes (C4, E4)
        let mut mei_chord = MeiChord::default();

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        let mut note2 = MeiNote::default();
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));
        mei_chord.children.push(ChordChild::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        assert!(result.is_ok());

        let mxml_notes = result.unwrap();
        // Should have two notes
        assert_eq!(mxml_notes.len(), 2);
        // First note should NOT have chord flag
        assert!(mxml_notes[0].chord.is_none());
        // Second note should have chord flag
        assert!(mxml_notes[1].chord.is_some());
    }

    #[test]
    fn test_convert_mei_chord_pitches_preserved() {
        use crate::model::data::Step;
        use crate::model::note::FullNoteContent;
        use tusk_model::data::{DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        // Create a C major chord (C4, E4, G4)
        let mut mei_chord = MeiChord::default();

        for pname in ["c", "e", "g"] {
            let mut note = MeiNote::default();
            note.note_log.pname = Some(DataPitchname::from(pname.to_string()));
            note.note_log.oct = Some(DataOctave::from(4u64));
            mei_chord.children.push(ChordChild::Note(Box::new(note)));
        }

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // Check pitches are preserved
        let steps: Vec<Step> = mxml_notes
            .iter()
            .filter_map(|n| {
                if let FullNoteContent::Pitch(pitch) = &n.content {
                    Some(pitch.step)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(steps, vec![Step::C, Step::E, Step::G]);
    }

    #[test]
    fn test_convert_mei_chord_duration() {
        use crate::model::note::NoteTypeValue;
        use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        // Create a chord with half note duration
        let mut mei_chord = MeiChord::default();
        mei_chord.chord_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N2)); // Half note

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        let mut note2 = MeiNote::default();
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));
        mei_chord.children.push(ChordChild::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // Both notes should have half note duration (2 * 4 = 8 divisions)
        for note in &mxml_notes {
            assert_eq!(note.duration, Some(8.0));
            assert_eq!(
                note.note_type.as_ref().map(|t| &t.value),
                Some(&NoteTypeValue::Half)
            );
        }
    }

    #[test]
    fn test_convert_mei_chord_dots() {
        use tusk_model::data::{
            DataAugmentdot, DataDuration, DataDurationCmn, DataOctave, DataPitchname,
        };
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        // Create a dotted quarter note chord
        let mut mei_chord = MeiChord::default();
        mei_chord.chord_log.dur = Some(DataDuration::MeiDataDurationCmn(DataDurationCmn::N4));
        mei_chord.chord_log.dots = Some(DataAugmentdot::from(1u64));

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        let mut note2 = MeiNote::default();
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));
        mei_chord.children.push(ChordChild::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // Both notes should have 1 dot
        for note in &mxml_notes {
            assert_eq!(note.dots.len(), 1);
        }

        // Duration should be dotted quarter = 1.5 * 4 = 6 divisions
        assert_eq!(mxml_notes[0].duration, Some(6.0));
    }

    #[test]
    fn test_convert_mei_chord_grace() {
        use tusk_model::data::{DataGrace, DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        // Create a grace chord
        let mut mei_chord = MeiChord::default();
        mei_chord.chord_log.grace = Some(DataGrace::Acc);

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        let mut note2 = MeiNote::default();
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));
        mei_chord.children.push(ChordChild::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // Both notes should be grace notes
        for note in &mxml_notes {
            assert!(note.is_grace());
            assert!(note.duration.is_none()); // Grace notes have no duration
        }
    }

    #[test]
    fn test_convert_mei_chord_cue() {
        use tusk_model::data::{DataBoolean, DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        // Create a cue chord
        let mut mei_chord = MeiChord::default();
        mei_chord.chord_log.cue = Some(DataBoolean::True);

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        let mut note2 = MeiNote::default();
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));
        mei_chord.children.push(ChordChild::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // Both notes should be cue notes
        for note in &mxml_notes {
            assert!(note.cue.is_some());
        }
    }

    #[test]
    fn test_convert_mei_chord_with_id() {
        use tusk_model::data::{DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        let mut mei_chord = MeiChord::default();
        mei_chord.common.xml_id = Some("chord1".to_string());

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let _ = convert_mei_chord(&mei_chord, &mut ctx);

        // Chord ID should be mapped in context
        assert!(ctx.get_mei_id("chord1").is_some());
    }

    #[test]
    fn test_convert_mei_chord_with_stem_direction() {
        use crate::model::note::StemValue;
        use tusk_model::data::{
            DataOctave, DataPitchname, DataStemdirection, DataStemdirectionBasic,
        };
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        let mut mei_chord = MeiChord::default();
        mei_chord.chord_vis.stem_dir = Some(DataStemdirection::MeiDataStemdirectionBasic(
            DataStemdirectionBasic::Up,
        ));

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        let mut note2 = MeiNote::default();
        note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
        note2.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));
        mei_chord.children.push(ChordChild::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // First note should have stem direction (chord stem applies to first note)
        assert_eq!(
            mxml_notes[0].stem.as_ref().map(|s| &s.value),
            Some(&StemValue::Up)
        );
    }

    #[test]
    fn test_convert_mei_chord_empty_returns_empty() {
        use tusk_model::elements::Chord as MeiChord;

        // Create an empty chord (no notes)
        let mei_chord = MeiChord::default();

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        assert!(result.is_ok());

        let mxml_notes = result.unwrap();
        assert!(mxml_notes.is_empty());
    }

    #[test]
    fn test_convert_mei_chord_single_note() {
        use tusk_model::data::{DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        // Create a chord with just one note (degenerate case)
        let mut mei_chord = MeiChord::default();

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // Should have one note without chord flag
        assert_eq!(mxml_notes.len(), 1);
        assert!(mxml_notes[0].chord.is_none());
    }

    #[test]
    fn test_convert_mei_chord_with_gestural_duration() {
        use tusk_model::data::{DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        let mut mei_chord = MeiChord::default();
        mei_chord.chord_ges.dur_ppq = Some("96".to_string()); // 96 ppq = quarter note at 96 ppq

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // Duration should come from gestural attribute
        assert_eq!(mxml_notes[0].duration, Some(96.0));
    }

    #[test]
    fn test_convert_mei_chord_with_accidentals() {
        use crate::model::note::FullNoteContent;
        use tusk_model::data::{
            DataAccidentalGestural, DataAccidentalGesturalBasic, DataOctave, DataPitchname,
        };
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        // Create a chord with C# and F# (C#4, F#4)
        let mut mei_chord = MeiChord::default();

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));
        note1.note_ges.accid_ges = Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::S,
        ));

        let mut note2 = MeiNote::default();
        note2.note_log.pname = Some(DataPitchname::from("f".to_string()));
        note2.note_log.oct = Some(DataOctave::from(4u64));
        note2.note_ges.accid_ges = Some(DataAccidentalGestural::MeiDataAccidentalGesturalBasic(
            DataAccidentalGesturalBasic::S,
        ));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));
        mei_chord.children.push(ChordChild::Note(Box::new(note2)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let result = convert_mei_chord(&mei_chord, &mut ctx);
        let mxml_notes = result.unwrap();

        // Both notes should have alter = 1 (sharp)
        for note in &mxml_notes {
            if let FullNoteContent::Pitch(pitch) = &note.content {
                assert_eq!(pitch.alter, Some(1.0));
            }
        }
    }

    #[test]
    fn test_convert_mei_chord_warnings_for_lossy_attributes() {
        use tusk_model::data::{DataBeat, DataOctave, DataPitchname};
        use tusk_model::elements::{Chord as MeiChord, ChordChild, Note as MeiNote};

        let mut mei_chord = MeiChord::default();
        mei_chord.chord_log.tstamp = Some(DataBeat::from(1.0));

        let mut note1 = MeiNote::default();
        note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
        note1.note_log.oct = Some(DataOctave::from(4u64));

        mei_chord.children.push(ChordChild::Note(Box::new(note1)));

        let mut ctx = ConversionContext::new(ConversionDirection::MeiToMusicXml);
        ctx.set_divisions(4.0);

        let _ = convert_mei_chord(&mei_chord, &mut ctx);

        // Should have warnings about timing attributes
        assert!(ctx.has_warnings());
    }
}

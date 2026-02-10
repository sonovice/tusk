//! Conversion from MEI to LilyPond AST.

use thiserror::Error;
use tusk_model::elements::{LayerChild, MeasureChild, Mei, MeiChild, ScoreChild, SectionChild};
use tusk_model::generated::data::{DataAccidentalGesturalBasic, DataDurationCmn};

use crate::model::pitch::Pitch;
use crate::model::{
    Duration, LilyPondFile, MultiMeasureRestEvent, Music, NoteEvent, RestEvent, ScoreBlock,
    ScoreItem, ToplevelExpression, Version,
};

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("LilyPond export is not yet implemented")]
    NotImplemented,
    #[error("no music found in MEI document")]
    NoMusic,
    #[error("export error: {0}")]
    Other(String),
}

/// Convert an MEI document to a LilyPond AST.
pub fn export(mei: &Mei) -> Result<LilyPondFile, ExportError> {
    // Find the Music → Body → Mdiv → Score path
    let score = find_score(mei).ok_or(ExportError::NoMusic)?;

    // Walk section → measures → staves → layers → notes/rests
    let mut staff_music: Vec<Vec<Vec<Music>>> = Vec::new(); // staff → layer → items

    for child in &score.children {
        if let ScoreChild::Section(section) = child {
            for section_child in &section.children {
                if let SectionChild::Measure(measure) = section_child {
                    for mc in &measure.children {
                        if let MeasureChild::Staff(staff) = mc {
                            let mut layers: Vec<Vec<Music>> = Vec::new();
                            for sc in &staff.children {
                                let tusk_model::elements::StaffChild::Layer(layer) = sc;
                                let mut items = Vec::new();
                                for lc in &layer.children {
                                    if let Some(m) = convert_layer_child(lc) {
                                        items.push(m);
                                    }
                                }
                                layers.push(items);
                            }
                            staff_music.push(layers);
                        }
                    }
                }
            }
        }
    }

    // Build music expression from collected layers
    let music = build_music_from_layers(staff_music);

    let score_block = ScoreBlock {
        items: vec![ScoreItem::Music(music)],
    };

    Ok(LilyPondFile {
        version: Some(Version {
            version: "2.24.0".to_string(),
        }),
        items: vec![ToplevelExpression::Score(score_block)],
    })
}

/// Build a Music expression from staff/layer structure.
///
/// Single layer → `{ notes... }` (sequential).
/// Multiple layers → `<< { voice1 } { voice2 } >>` (simultaneous).
fn build_music_from_layers(staff_music: Vec<Vec<Vec<Music>>>) -> Music {
    // Flatten all staves (for now we handle single-staff)
    let mut all_layers: Vec<Vec<Music>> = Vec::new();
    for layers in staff_music {
        all_layers.extend(layers);
    }

    // Filter out empty layers
    let non_empty: Vec<Vec<Music>> = all_layers.into_iter().filter(|l| !l.is_empty()).collect();

    match non_empty.len() {
        0 => Music::Sequential(Vec::new()),
        1 => Music::Sequential(non_empty.into_iter().next().unwrap()),
        _ => {
            // Multiple layers → simultaneous with each as sequential
            let voices: Vec<Music> = non_empty.into_iter().map(Music::Sequential).collect();
            Music::Simultaneous(voices)
        }
    }
}

/// Find the Score element in the MEI hierarchy.
fn find_score(mei: &Mei) -> Option<&tusk_model::elements::Score> {
    for child in &mei.children {
        if let MeiChild::Music(music) = child
            && let Some(tusk_model::elements::MusicChild::Body(body)) = music.children.first()
            && let Some(tusk_model::elements::BodyChild::Mdiv(mdiv)) = body.children.first()
            && let Some(tusk_model::elements::MdivChild::Score(score)) = mdiv.children.first()
        {
            return Some(score);
        }
    }
    None
}

/// Convert a single MEI LayerChild to a LilyPond Music expression.
fn convert_layer_child(child: &LayerChild) -> Option<Music> {
    match child {
        LayerChild::Note(note) => Some(convert_mei_note(note)),
        LayerChild::Rest(rest) => Some(convert_mei_rest(rest)),
        LayerChild::MRest(mrest) => Some(convert_mei_mrest(mrest)),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// MEI → LilyPond pitch/duration conversion
// ---------------------------------------------------------------------------

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

/// Convert an MEI Note to a LilyPond NoteEvent.
fn convert_mei_note(note: &tusk_model::elements::Note) -> Music {
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

    // Determine alter from gestural accidental
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

    // Check for written accidental (force/cautionary)
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

    let pitch = Pitch {
        step,
        alter,
        octave,
        force_accidental,
        cautionary,
    };

    let duration = extract_note_duration(note);

    Music::Note(NoteEvent {
        pitch,
        duration,
        pitched_rest: false,
    })
}

/// Convert an MEI Rest to a LilyPond RestEvent or pitched rest.
fn convert_mei_rest(rest: &tusk_model::elements::Rest) -> Music {
    // Check for pitched rest label
    if let Some(label) = &rest.common.label
        && let Some(pitch_str) = label.strip_prefix("lilypond:pitched-rest,")
        && let Some(note_event) = parse_pitched_rest_label(pitch_str, rest)
    {
        return Music::Note(note_event);
    }

    Music::Rest(RestEvent {
        duration: extract_rest_duration(rest),
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
        },
        duration: extract_rest_duration(rest),
        pitched_rest: true,
    })
}

/// Convert an MEI MRest to a LilyPond MultiMeasureRestEvent.
fn convert_mei_mrest(mrest: &tusk_model::elements::MRest) -> Music {
    // Restore duration from label
    let duration = mrest
        .common
        .label
        .as_ref()
        .and_then(|l| l.strip_prefix("lilypond:mrest,"))
        .and_then(parse_mrest_label);

    Music::MultiMeasureRest(MultiMeasureRestEvent { duration })
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import;
    use crate::parser::Parser;
    use crate::serializer;

    /// Parse LilyPond → import to MEI → export to LilyPond AST → serialize.
    fn roundtrip(src: &str) -> String {
        let file = Parser::new(src).unwrap().parse().unwrap();
        let mei = import::import(&file).unwrap();
        let exported = export(&mei).unwrap();
        serializer::serialize(&exported)
    }

    #[test]
    fn roundtrip_single_note() {
        let output = roundtrip("{ c'4 }");
        assert!(output.contains("c'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_note_with_accidental() {
        let output = roundtrip("{ cis''2 }");
        assert!(output.contains("cis''2"), "output: {output}");
    }

    #[test]
    fn roundtrip_rest() {
        let output = roundtrip("{ r4 }");
        assert!(output.contains("r4"), "output: {output}");
    }

    #[test]
    fn roundtrip_dotted() {
        let output = roundtrip("{ c'2. r8. }");
        assert!(output.contains("c'2."), "output: {output}");
        assert!(output.contains("r8."), "output: {output}");
    }

    #[test]
    fn roundtrip_flat() {
        let output = roundtrip("{ bes,16 }");
        assert!(output.contains("bes,16"), "output: {output}");
    }

    #[test]
    fn roundtrip_multiple_notes() {
        let output = roundtrip("{ c4 d8 e16 f2 }");
        assert!(output.contains("c4"), "output: {output}");
        assert!(output.contains("d8"), "output: {output}");
        assert!(output.contains("e16"), "output: {output}");
        assert!(output.contains("f2"), "output: {output}");
    }

    #[test]
    fn roundtrip_multi_measure_rest() {
        let output = roundtrip("{ R1*4 }");
        assert!(output.contains("R1*4"), "output: {output}");
    }

    #[test]
    fn roundtrip_pitched_rest() {
        let output = roundtrip("{ c4\\rest }");
        assert!(output.contains("c4\\rest"), "output: {output}");
    }

    #[test]
    fn roundtrip_force_accidental() {
        let output = roundtrip("{ cis'!4 }");
        assert!(output.contains("cis'!4"), "output: {output}");
    }

    #[test]
    fn roundtrip_cautionary_accidental() {
        let output = roundtrip("{ bes'?4 }");
        assert!(output.contains("bes'?4"), "output: {output}");
    }

    #[test]
    fn roundtrip_two_voices() {
        let output = roundtrip("<< { c'4 d'4 } { e'4 f'4 } >>");
        // Should produce simultaneous with two sequential voices
        assert!(output.contains("<<"), "output: {output}");
        assert!(output.contains(">>"), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
        assert!(output.contains("d'4"), "output: {output}");
        assert!(output.contains("e'4"), "output: {output}");
        assert!(output.contains("f'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_three_voices() {
        let output = roundtrip("<< { c'4 } { e'4 } { g'4 } >>");
        assert!(output.contains("<<"), "output: {output}");
        assert!(output.contains(">>"), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
        assert!(output.contains("e'4"), "output: {output}");
        assert!(output.contains("g'4"), "output: {output}");
    }

    #[test]
    fn roundtrip_sequential_preserved() {
        // Single voice should stay sequential, no << >>
        let output = roundtrip("{ c'4 d'4 e'4 }");
        assert!(!output.contains("<<"), "output: {output}");
        assert!(output.contains("c'4"), "output: {output}");
    }
}

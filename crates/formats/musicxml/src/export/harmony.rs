//! Harmony export from MEI to MusicXML.
//!
//! Converts MEI `<harm>` control events back to MusicXML `<harmony>` elements.
//! When the harm element has HarmonyData in ExtensionStore, the full MusicXML
//! Harmony struct is reconstructed from typed data. Otherwise, a simple
//! text-only harmony is created.

use crate::context::ConversionContext;
use crate::model::data::{AboveBelow, LeftRight, Step, YesNo};
use crate::model::direction::Offset;
use crate::model::elements::MeasureContent;
use crate::model::harmony::{
    Barre, Bass, BassStep, Degree, DegreeAlter, DegreeType, DegreeTypeValue, DegreeValue,
    FirstFret, Frame, FrameFingering, FrameNote, FrameString, Fret, Harmony, HarmonyAlter,
    HarmonyArrangement, HarmonyChord, HarmonyChordRoot, HarmonyType, Inversion, Kind, KindValue,
    Numeral, NumeralKey, NumeralMode, NumeralRoot, Root, RootStep, StyleText,
};
use tusk_model::elements::{Harm, HarmChild};
use tusk_model::musicxml_ext::HarmonyData;

/// Convert an MEI `<harm>` control event to a MusicXML `Harmony` measure content.
///
/// If the harm element has HarmonyData in ExtensionStore, the original `Harmony`
/// struct is reconstructed from typed data. The MusicXML staff number is set to
/// `local_staff_n` for correct within-part numbering.
///
/// Returns `None` if the harm element cannot be converted.
pub fn convert_mei_harm(
    harm: &Harm,
    local_staff_n: usize,
    ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    // Reconstruct from ExtensionStore typed data
    if let Some(id) = &harm.common.xml_id {
        if let Some(data) = ctx.ext_store().harmony(id) {
            let mut harmony = build_harmony_from_data(data);
            harmony.staff = Some(local_staff_n as u32);
            return Some(MeasureContent::Harmony(Box::new(harmony)));
        }
    }

    // Fallback: create a minimal harmony from text content
    let text = collect_harm_text(harm);
    if text.is_empty() {
        return None;
    }

    let harmony = create_text_harmony(&text, local_staff_n);
    Some(MeasureContent::Harmony(Box::new(harmony)))
}

/// Build a MusicXML `Harmony` from typed `HarmonyData`.
fn build_harmony_from_data(data: &HarmonyData) -> Harmony {
    Harmony {
        chords: data.chords.iter().map(build_chord_from_data).collect(),
        frame: data.frame.as_ref().map(build_frame_from_data),
        offset: data.offset.as_ref().map(|o| Offset {
            value: o.value,
            sound: o.sound.map(|b| if b { YesNo::Yes } else { YesNo::No }),
        }),
        footnote: None,
        level: None,
        staff: None,
        harmony_type: data.harmony_type.as_deref().and_then(parse_harmony_type),
        print_object: data
            .print_object
            .map(|b| if b { YesNo::Yes } else { YesNo::No }),
        print_frame: data
            .print_frame
            .map(|b| if b { YesNo::Yes } else { YesNo::No }),
        arrangement: data
            .arrangement
            .as_deref()
            .and_then(parse_harmony_arrangement),
        placement: data.placement.as_deref().and_then(parse_above_below),
        font_family: data.visual.as_ref().and_then(|v| v.font_family.clone()),
        font_size: data.visual.as_ref().and_then(|v| v.font_size),
        font_style: data.visual.as_ref().and_then(|v| v.font_style.clone()),
        font_weight: data.visual.as_ref().and_then(|v| v.font_weight.clone()),
        default_x: data.visual.as_ref().and_then(|v| v.default_x),
        default_y: data.visual.as_ref().and_then(|v| v.default_y),
        color: data.visual.as_ref().and_then(|v| v.color.clone()),
        id: data.id.clone(),
    }
}

fn build_chord_from_data(chord: &tusk_model::musicxml_ext::HarmonyChordData) -> HarmonyChord {
    let root_type = match chord.root_type.as_str() {
        "root" => {
            let step = chord
                .root_step
                .as_deref()
                .and_then(parse_step)
                .unwrap_or(Step::C);
            HarmonyChordRoot::Root(Root {
                root_step: RootStep {
                    value: step,
                    text: chord.root_text.clone(),
                },
                root_alter: chord.root_alter.map(|v| HarmonyAlter {
                    value: v,
                    print_object: None,
                    location: None,
                }),
            })
        }
        "numeral" => {
            let numeral_value = chord.numeral_value.unwrap_or(1);
            HarmonyChordRoot::Numeral(Numeral {
                numeral_root: NumeralRoot {
                    value: numeral_value,
                    text: chord.root_text.clone(),
                },
                numeral_alter: chord.root_alter.map(|v| HarmonyAlter {
                    value: v,
                    print_object: None,
                    location: None,
                }),
                numeral_key: chord.numeral_key.as_ref().map(|k| NumeralKey {
                    numeral_fifths: k.fifths,
                    numeral_mode: NumeralMode::from_str(&k.mode).unwrap_or(NumeralMode::Major),
                    print_object: None,
                }),
            })
        }
        _ => {
            // "function" or unknown
            HarmonyChordRoot::Function(StyleText {
                value: chord.function.clone().unwrap_or_default(),
                font_family: None,
                font_style: None,
                font_size: None,
                font_weight: None,
                color: None,
            })
        }
    };

    let kind_value = KindValue::from_str(&chord.kind.value).unwrap_or(KindValue::Other);

    HarmonyChord {
        root_type,
        kind: Kind {
            value: kind_value,
            text: chord.kind.text.clone(),
            use_symbols: chord
                .kind
                .use_symbols
                .map(|b| if b { YesNo::Yes } else { YesNo::No }),
            stack_degrees: chord
                .kind
                .stack_degrees
                .map(|b| if b { YesNo::Yes } else { YesNo::No }),
            parentheses_degrees: chord
                .kind
                .parentheses_degrees
                .map(|b| if b { YesNo::Yes } else { YesNo::No }),
            bracket_degrees: chord
                .kind
                .bracket_degrees
                .map(|b| if b { YesNo::Yes } else { YesNo::No }),
            halign: chord
                .kind
                .halign
                .as_deref()
                .and_then(parse_left_center_right),
            valign: None,
        },
        inversion: chord.inversion.map(|v| Inversion {
            value: v,
            text: None,
        }),
        bass: chord.bass.as_ref().map(build_bass_from_data),
        degrees: chord.degrees.iter().map(build_degree_from_data).collect(),
    }
}

fn build_bass_from_data(bass: &tusk_model::musicxml_ext::BassData) -> Bass {
    let step = parse_step(&bass.step).unwrap_or(Step::C);
    Bass {
        bass_separator: bass.separator.as_ref().map(|s| StyleText {
            value: s.clone(),
            font_family: None,
            font_style: None,
            font_size: None,
            font_weight: None,
            color: None,
        }),
        bass_step: BassStep {
            value: step,
            text: bass.text.clone(),
        },
        bass_alter: bass.alter.map(|v| HarmonyAlter {
            value: v,
            print_object: None,
            location: None,
        }),
        arrangement: bass
            .arrangement
            .as_deref()
            .and_then(parse_harmony_arrangement),
    }
}

fn build_degree_from_data(deg: &tusk_model::musicxml_ext::DegreeData) -> Degree {
    Degree {
        degree_value: DegreeValue {
            value: deg.value,
            symbol: deg
                .symbol
                .as_deref()
                .and_then(crate::model::harmony::DegreeSymbolValue::from_str),
            text: deg.value_text.clone(),
        },
        degree_alter: DegreeAlter {
            value: deg.alter,
            plus_minus: deg
                .plus_minus
                .map(|b| if b { YesNo::Yes } else { YesNo::No }),
        },
        degree_type: DegreeType {
            value: DegreeTypeValue::from_str(&deg.degree_type).unwrap_or(DegreeTypeValue::Add),
            text: None,
        },
        print_object: None,
    }
}

fn build_frame_from_data(frame: &tusk_model::musicxml_ext::FrameData) -> Frame {
    Frame {
        frame_strings: frame.strings,
        frame_frets: frame.frets,
        first_fret: frame.first_fret.as_ref().map(|ff| FirstFret {
            value: ff.value,
            text: ff.text.clone(),
            location: ff.location.as_deref().and_then(parse_left_right),
        }),
        frame_notes: frame.notes.iter().map(build_frame_note_from_data).collect(),
        height: None,
        width: None,
        default_x: frame.visual.as_ref().and_then(|v| v.default_x),
        default_y: frame.visual.as_ref().and_then(|v| v.default_y),
        halign: None,
        valign: None,
        unplayed: frame.unplayed.clone(),
        color: frame.visual.as_ref().and_then(|v| v.color.clone()),
        id: frame.id.clone(),
    }
}

fn build_frame_note_from_data(note: &tusk_model::musicxml_ext::FrameNoteData) -> FrameNote {
    use crate::model::data::StartStop;
    FrameNote {
        string: FrameString {
            value: note.string,
            placement: None,
        },
        fret: Fret { value: note.fret },
        fingering: note.fingering.as_ref().map(|f| FrameFingering {
            value: f.clone(),
            substitution: None,
            alternate: None,
        }),
        barre: note.barre.as_deref().map(|b| Barre {
            barre_type: if b == "stop" {
                StartStop::Stop
            } else {
                StartStop::Start
            },
            color: None,
        }),
    }
}

// --- String-to-enum parsing helpers ---

fn parse_step(s: &str) -> Option<Step> {
    match s {
        "A" => Some(Step::A),
        "B" => Some(Step::B),
        "C" => Some(Step::C),
        "D" => Some(Step::D),
        "E" => Some(Step::E),
        "F" => Some(Step::F),
        "G" => Some(Step::G),
        _ => None,
    }
}

fn parse_above_below(s: &str) -> Option<AboveBelow> {
    match s {
        "above" => Some(AboveBelow::Above),
        "below" => Some(AboveBelow::Below),
        _ => None,
    }
}

fn parse_harmony_type(s: &str) -> Option<HarmonyType> {
    match s {
        "explicit" => Some(HarmonyType::Explicit),
        "implied" => Some(HarmonyType::Implied),
        "alternate" => Some(HarmonyType::Alternate),
        _ => None,
    }
}

fn parse_harmony_arrangement(s: &str) -> Option<HarmonyArrangement> {
    match s {
        "vertical" => Some(HarmonyArrangement::Vertical),
        "horizontal" => Some(HarmonyArrangement::Horizontal),
        "diagonal" => Some(HarmonyArrangement::Diagonal),
        _ => None,
    }
}

fn parse_left_center_right(s: &str) -> Option<crate::model::data::LeftCenterRight> {
    use crate::model::data::LeftCenterRight;
    match s {
        "left" => Some(LeftCenterRight::Left),
        "center" => Some(LeftCenterRight::Center),
        "right" => Some(LeftCenterRight::Right),
        _ => None,
    }
}

fn parse_left_right(s: &str) -> Option<LeftRight> {
    match s {
        "left" => Some(LeftRight::Left),
        "right" => Some(LeftRight::Right),
        _ => None,
    }
}

/// Collect text content from harm children.
fn collect_harm_text(harm: &Harm) -> String {
    harm.children
        .iter()
        .map(|c| {
            let HarmChild::Text(t) = c;
            t.as_str()
        })
        .collect::<Vec<_>>()
        .join("")
}

/// Create a minimal MusicXML Harmony from plain text (function-based).
fn create_text_harmony(text: &str, local_staff_n: usize) -> Harmony {
    Harmony {
        chords: vec![HarmonyChord {
            root_type: HarmonyChordRoot::Function(StyleText {
                value: text.to_string(),
                font_family: None,
                font_style: None,
                font_size: None,
                font_weight: None,
                color: None,
            }),
            kind: Kind {
                value: KindValue::None,
                text: None,
                use_symbols: None,
                stack_degrees: None,
                parentheses_degrees: None,
                bracket_degrees: None,
                halign: None,
                valign: None,
            },
            inversion: None,
            bass: None,
            degrees: vec![],
        }],
        frame: None,
        offset: None,
        footnote: None,
        level: None,
        staff: Some(local_staff_n as u32),
        harmony_type: None,
        print_object: None,
        print_frame: None,
        arrangement: None,
        placement: None,
        font_family: None,
        font_size: None,
        font_style: None,
        font_weight: None,
        default_x: None,
        default_y: None,
        color: None,
        id: None,
    }
}

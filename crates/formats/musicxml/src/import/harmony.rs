//! Harmony conversion from MusicXML to MEI.
//!
//! Converts MusicXML `<harmony>` elements to MEI `<harm>` control events.
//! The full MusicXML Harmony struct is serialized as JSON in the `@label`
//! attribute for lossless roundtrip; a human-readable chord text summary
//! is stored as the `<harm>` text child.

use crate::context::ConversionContext;
use crate::model::data::{AboveBelow, Step};
use crate::model::harmony::{Harmony, HarmonyChord, HarmonyChordRoot, KindValue, Root};
use tusk_model::data::{DataBeat, DataStaffrel, DataStaffrelBasic};
use tusk_model::elements::{Harm, HarmChild};

/// Label prefix for MEI harm elements carrying roundtrip JSON data.
pub const HARM_LABEL_PREFIX: &str = "musicxml:harmony,";

/// Convert a MusicXML `<harmony>` element to an MEI `<harm>` control event.
///
/// The full `Harmony` struct is JSON-encoded in `@label` for lossless roundtrip.
/// A human-readable chord symbol (e.g. "Cmaj7", "Am/E") is stored as text.
pub fn convert_harmony(harmony: &Harmony, ctx: &mut ConversionContext) -> Harm {
    let tstamp = calculate_harmony_tstamp(harmony, ctx);
    let staff = ctx.current_staff();
    let place = harmony.placement.as_ref().map(convert_placement);

    let mut harm = Harm::default();

    // Generate unique ID
    let harm_id = ctx.generate_id_with_suffix("harm");
    harm.common.xml_id = Some(harm_id);

    // Encode full MusicXML Harmony as JSON in label for lossless roundtrip.
    // Normalize: clear `staff` (handled via MEI @staff), and canonicalize
    // `offset` to encode the absolute beat position in divisions. On export,
    // harmony elements are placed before notes (like directions), so
    // beat_position=0 on re-import — the offset ensures correct tstamp.
    let mut harmony_for_json = harmony.clone();
    harmony_for_json.staff = None;
    // Compute absolute position in divisions = current_beat_position + existing_offset
    let abs_position =
        ctx.beat_position() + harmony.offset.as_ref().map(|o| o.value).unwrap_or(0.0);
    if abs_position != 0.0 || harmony.offset.is_some() {
        harmony_for_json.offset = Some(crate::model::direction::Offset {
            value: abs_position,
            sound: harmony.offset.as_ref().and_then(|o| o.sound),
        });
    } else {
        harmony_for_json.offset = None;
    }
    if let Ok(json) = serde_json::to_string(&harmony_for_json) {
        harm.common.label = Some(format!("{}{}", HARM_LABEL_PREFIX, json));
    }

    // Set tstamp and staff
    harm.harm_log.tstamp = Some(tstamp);
    harm.harm_log.staff = Some((staff as u64).to_string());

    // Set placement
    if let Some(place) = place {
        harm.harm_vis.place = Some(place);
    }

    // Human-readable text summary
    let text = harmony_to_text(harmony);
    if !text.is_empty() {
        harm.children.push(HarmChild::Text(text));
    }

    harm
}

/// Calculate the MEI tstamp for a harmony element.
///
/// Uses the current beat position from context, adjusted by any offset.
/// MEI tstamp is 1-based (beat 1 is the first beat).
fn calculate_harmony_tstamp(harmony: &Harmony, ctx: &ConversionContext) -> DataBeat {
    let mut beat_position = ctx.beat_position_in_beats();

    // Apply offset if present (offset is in divisions)
    if let Some(ref offset) = harmony.offset {
        beat_position += ctx.divisions_to_beats(offset.value);
    }

    // MEI tstamp is 1-based
    DataBeat::from(beat_position + 1.0)
}

/// Convert MusicXML AboveBelow placement to MEI DataStaffrel.
fn convert_placement(p: &AboveBelow) -> DataStaffrel {
    DataStaffrel::MeiDataStaffrelBasic(match p {
        AboveBelow::Above => DataStaffrelBasic::Above,
        AboveBelow::Below => DataStaffrelBasic::Below,
    })
}

/// Generate a human-readable text summary of a harmony element.
///
/// Produces concise chord symbol text like "C", "Am7", "Dm/F", "bIII".
fn harmony_to_text(harmony: &Harmony) -> String {
    harmony
        .chords
        .iter()
        .map(chord_to_text)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Generate text for a single chord within a harmony.
fn chord_to_text(chord: &HarmonyChord) -> String {
    let mut text = String::new();

    // Root identifier
    match &chord.root_type {
        HarmonyChordRoot::Root(root) => {
            text.push_str(&root_to_text(root));
        }
        HarmonyChordRoot::Numeral(numeral) => {
            if let Some(ref alter) = numeral.numeral_alter {
                if alter.value < 0.0 {
                    text.push('b');
                } else if alter.value > 0.0 {
                    text.push('#');
                }
            }
            text.push_str(
                numeral
                    .numeral_root
                    .text
                    .as_deref()
                    .unwrap_or(&numeral.numeral_root.value.to_string()),
            );
        }
        HarmonyChordRoot::Function(func) => {
            text.push_str(&func.value);
        }
    }

    // Kind abbreviation
    text.push_str(kind_abbreviation(chord.kind.value));

    // Bass note (slash chord)
    if let Some(ref bass) = chord.bass {
        text.push('/');
        text.push_str(&step_to_str(bass.bass_step.value));
        if let Some(ref alter) = bass.bass_alter {
            if alter.value < 0.0 {
                text.push('b');
            } else if alter.value > 0.0 {
                text.push('#');
            }
        }
    }

    text
}

/// Convert a Root to its text representation.
fn root_to_text(root: &Root) -> String {
    let mut s = step_to_str(root.root_step.value);
    if let Some(ref alter) = root.root_alter {
        if alter.value <= -1.0 {
            s.push('b');
        } else if alter.value >= 1.0 {
            s.push('#');
        }
    }
    s
}

/// Convert a Step enum to string.
fn step_to_str(step: Step) -> String {
    match step {
        Step::A => "A".into(),
        Step::B => "B".into(),
        Step::C => "C".into(),
        Step::D => "D".into(),
        Step::E => "E".into(),
        Step::F => "F".into(),
        Step::G => "G".into(),
    }
}

/// Get a short abbreviation for a chord kind.
fn kind_abbreviation(kind: KindValue) -> &'static str {
    match kind {
        KindValue::Major => "",
        KindValue::Minor => "m",
        KindValue::Augmented => "aug",
        KindValue::Diminished => "dim",
        KindValue::Dominant => "7",
        KindValue::MajorSeventh => "maj7",
        KindValue::MinorSeventh => "m7",
        KindValue::DiminishedSeventh => "dim7",
        KindValue::AugmentedSeventh => "aug7",
        KindValue::HalfDiminished => "m7b5",
        KindValue::MajorMinor => "mMaj7",
        KindValue::MajorSixth => "6",
        KindValue::MinorSixth => "m6",
        KindValue::DominantNinth => "9",
        KindValue::MajorNinth => "maj9",
        KindValue::MinorNinth => "m9",
        KindValue::Dominant11th => "11",
        KindValue::Major11th => "maj11",
        KindValue::Minor11th => "m11",
        KindValue::Dominant13th => "13",
        KindValue::Major13th => "maj13",
        KindValue::Minor13th => "m13",
        KindValue::SuspendedSecond => "sus2",
        KindValue::SuspendedFourth => "sus4",
        KindValue::Neapolitan => "N",
        KindValue::Italian => "It",
        KindValue::French => "Fr",
        KindValue::German => "Ger",
        KindValue::Pedal => "ped",
        KindValue::Power => "5",
        KindValue::Tristan => "Tris",
        KindValue::Other => "other",
        KindValue::None => "",
    }
}

/// Reconstruct a MusicXML `Harmony` from the `@label` JSON data.
///
/// Returns `None` if the label doesn't contain valid harmony JSON data.
pub fn harmony_from_label(label: &str) -> Option<Harmony> {
    let json = label.strip_prefix(HARM_LABEL_PREFIX)?;
    serde_json::from_str(json).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::harmony::{
        Bass, BassStep, HarmonyAlter, HarmonyChord, HarmonyChordRoot, Kind, Root, RootStep,
    };

    #[test]
    fn test_harmony_to_text_major() {
        let harmony = Harmony {
            chords: vec![HarmonyChord {
                root_type: HarmonyChordRoot::Root(Root {
                    root_step: RootStep {
                        value: Step::C,
                        text: None,
                    },
                    root_alter: None,
                }),
                kind: Kind {
                    value: KindValue::Major,
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
            staff: None,
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
        };
        assert_eq!(harmony_to_text(&harmony), "C");
    }

    #[test]
    fn test_harmony_to_text_minor_seventh_with_bass() {
        let harmony = Harmony {
            chords: vec![HarmonyChord {
                root_type: HarmonyChordRoot::Root(Root {
                    root_step: RootStep {
                        value: Step::A,
                        text: None,
                    },
                    root_alter: None,
                }),
                kind: Kind {
                    value: KindValue::MinorSeventh,
                    text: None,
                    use_symbols: None,
                    stack_degrees: None,
                    parentheses_degrees: None,
                    bracket_degrees: None,
                    halign: None,
                    valign: None,
                },
                inversion: None,
                bass: Some(Bass {
                    bass_separator: None,
                    bass_step: BassStep {
                        value: Step::E,
                        text: None,
                    },
                    bass_alter: None,
                    arrangement: None,
                }),
                degrees: vec![],
            }],
            frame: None,
            offset: None,
            staff: None,
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
        };
        assert_eq!(harmony_to_text(&harmony), "Am7/E");
    }

    #[test]
    fn test_harmony_to_text_sharp_root() {
        let harmony = Harmony {
            chords: vec![HarmonyChord {
                root_type: HarmonyChordRoot::Root(Root {
                    root_step: RootStep {
                        value: Step::F,
                        text: None,
                    },
                    root_alter: Some(HarmonyAlter {
                        value: 1.0,
                        print_object: None,
                        location: None,
                    }),
                }),
                kind: Kind {
                    value: KindValue::Minor,
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
            staff: None,
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
        };
        assert_eq!(harmony_to_text(&harmony), "F#m");
    }

    #[test]
    fn test_harmony_json_roundtrip() {
        let harmony = Harmony {
            chords: vec![HarmonyChord {
                root_type: HarmonyChordRoot::Root(Root {
                    root_step: RootStep {
                        value: Step::C,
                        text: None,
                    },
                    root_alter: None,
                }),
                kind: Kind {
                    value: KindValue::MajorSeventh,
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
            staff: None,
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
        };

        let json = serde_json::to_string(&harmony).unwrap();
        let label = format!("{}{}", HARM_LABEL_PREFIX, json);
        let recovered = harmony_from_label(&label).unwrap();
        assert_eq!(harmony, recovered);
    }
}

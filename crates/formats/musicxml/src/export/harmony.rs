//! Harmony export from MEI to MusicXML.
//!
//! Converts MEI `<harm>` control events back to MusicXML `<harmony>` elements.
//! When the MEI harm element carries a `musicxml:harmony,` label prefix, the
//! full MusicXML Harmony struct is deserialized from the JSON payload for
//! lossless roundtrip. Otherwise, a simple text-only harmony is created.

use crate::context::ConversionContext;
use crate::import::harmony::harmony_from_label;
use crate::model::elements::MeasureContent;
use crate::model::harmony::Harmony;
use tusk_model::elements::{Harm, HarmChild};

/// Convert an MEI `<harm>` control event to a MusicXML `Harmony` measure content.
///
/// If the harm element carries a roundtrip label (`musicxml:harmony,{json}`),
/// the original `Harmony` struct is deserialized from JSON. The MusicXML staff
/// number is overridden with `local_staff_n` to maintain correct within-part
/// numbering.
///
/// Returns `None` if the harm element cannot be converted.
pub fn convert_mei_harm(
    harm: &Harm,
    local_staff_n: usize,
    ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    // Preferred: reconstruct from ExtensionStore mxml_json
    if let Some(id) = &harm.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref val) = ext.mxml_json {
                if let Ok(mut harmony) = serde_json::from_value::<Harmony>(val.clone()) {
                    harmony.staff = Some(local_staff_n as u32);
                    return Some(MeasureContent::Harmony(Box::new(harmony)));
                }
            }
        }
    }

    // Fallback: reconstruct from roundtrip label
    if let Some(label) = harm.common.label.as_deref() {
        if let Some(mut harmony) = harmony_from_label(label) {
            harmony.staff = Some(local_staff_n as u32);
            return Some(MeasureContent::Harmony(Box::new(harmony)));
        }
    }

    // Fallback: create a minimal harmony from text content
    let text = collect_harm_text(harm);
    if text.is_empty() {
        return None;
    }

    // For non-roundtrip harm elements, create a function-based harmony
    // (simplest MusicXML representation for plain text harmony)
    let harmony = create_text_harmony(&text, local_staff_n);
    Some(MeasureContent::Harmony(Box::new(harmony)))
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
///
/// Used when an MEI harm has no roundtrip JSON label — creates a
/// `<harmony><function>text</function><kind>none</kind></harmony>`.
fn create_text_harmony(text: &str, local_staff_n: usize) -> Harmony {
    use crate::model::harmony::{HarmonyChord, HarmonyChordRoot, Kind, KindValue, StyleText};

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

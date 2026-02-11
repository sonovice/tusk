//! Figured bass export from MEI to MusicXML.
//!
//! Converts MEI `<fb>` elements back to MusicXML `<figured-bass>` elements.
//! When the MEI fb element carries a `musicxml:figured-bass,` label prefix,
//! the full MusicXML FiguredBass struct is deserialized from the JSON payload
//! for lossless roundtrip.

use crate::context::ConversionContext;
use crate::import::figured_bass::figured_bass_from_label;
use crate::model::elements::MeasureContent;
use crate::model::figured_bass::FiguredBass;
use tusk_model::elements::Fb;

/// Convert an MEI `<fb>` element to a MusicXML `FiguredBass` measure content.
///
/// If the fb element carries a roundtrip label (`musicxml:figured-bass,{json}`),
/// the original `FiguredBass` struct is deserialized from JSON. The MusicXML
/// staff number is overridden with `local_staff_n`.
///
/// Returns `None` if the fb element cannot be converted.
pub fn convert_mei_fb(
    fb: &Fb,
    local_staff_n: usize,
    ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    // Preferred: reconstruct from ExtensionStore mxml_json
    if let Some(id) = &fb.common.xml_id {
        if let Some(ext) = ctx.ext_store().get(id) {
            if let Some(ref val) = ext.mxml_json {
                if let Ok(mut figured_bass) = serde_json::from_value::<FiguredBass>(val.clone()) {
                    figured_bass.staff = Some(local_staff_n as u32);
                    return Some(MeasureContent::FiguredBass(Box::new(figured_bass)));
                }
            }
        }
    }

    // Fallback: reconstruct from roundtrip label
    if let Some(label) = fb.common.label.as_deref() {
        if let Some(mut figured_bass) = figured_bass_from_label(label) {
            figured_bass.staff = Some(local_staff_n as u32);
            return Some(MeasureContent::FiguredBass(Box::new(figured_bass)));
        }
    }

    // Fallback: create a minimal figured-bass from fb children
    let figured_bass = create_fallback_figured_bass(fb, local_staff_n);
    Some(MeasureContent::FiguredBass(Box::new(figured_bass)))
}

/// Create a minimal MusicXML FiguredBass from an MEI fb with no roundtrip data.
fn create_fallback_figured_bass(fb: &Fb, local_staff_n: usize) -> FiguredBass {
    use crate::model::figured_bass::Figure;
    use crate::model::harmony::StyleText;
    use tusk_model::elements::{FChild, FbChild};

    let figures = fb
        .children
        .iter()
        .map(|child| {
            let FbChild::F(f) = child;
            let text: String = f
                .children
                .iter()
                .map(|c| {
                    let FChild::Text(t) = c;
                    t.as_str()
                })
                .collect();
            // Best-effort: put the text as figure-number
            Figure {
                prefix: None,
                figure_number: if text.is_empty() {
                    None
                } else {
                    Some(StyleText {
                        value: text,
                        font_family: None,
                        font_style: None,
                        font_size: None,
                        font_weight: None,
                        color: None,
                    })
                },
                suffix: None,
                extend: None,
            }
        })
        .collect();

    FiguredBass {
        figures,
        duration: None,
        offset: None,
        staff: Some(local_staff_n as u32),
        parentheses: None,
        placement: None,
        print_object: None,
        default_x: None,
        default_y: None,
        relative_x: None,
        relative_y: None,
        font_family: None,
        font_style: None,
        font_size: None,
        font_weight: None,
        color: None,
        halign: None,
        valign: None,
        id: None,
    }
}

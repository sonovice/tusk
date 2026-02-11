//! Figured bass conversion from MusicXML to MEI.
//!
//! Converts MusicXML `<figured-bass>` elements to MEI `<fb>` measure-level
//! elements with `<f>` children. Full MusicXML data is stored in ExtensionStore
//! for lossless roundtrip; human-readable figure text is stored in `<f>` children.

use crate::context::ConversionContext;
use crate::model::figured_bass::FiguredBass;
use tusk_model::elements::{F, FChild, Fb, FbChild};

/// Label marker for MEI fb elements carrying figured-bass data (via ExtensionStore).
pub const FB_LABEL_PREFIX: &str = "musicxml:figured-bass";

/// Convert a MusicXML `<figured-bass>` element to an MEI `<fb>` element.
///
/// Data is stored in ExtensionStore for lossless roundtrip.
/// Human-readable figure text (e.g. "b7", "6", "#") is stored in `<f>` children.
pub fn convert_figured_bass(fb: &FiguredBass, ctx: &mut ConversionContext) -> Fb {
    let mut mei_fb = Fb::default();

    let fb_id = ctx.generate_id_with_suffix("fb");
    mei_fb.common.xml_id = Some(fb_id);

    // Normalize: clear staff (handled via context), canonicalize offset.
    let mut fb_for_json = fb.clone();
    fb_for_json.staff = None;
    let abs_position = ctx.beat_position() + fb.offset.as_ref().map(|o| o.value).unwrap_or(0.0);
    if abs_position != 0.0 || fb.offset.is_some() {
        fb_for_json.offset = Some(crate::model::direction::Offset {
            value: abs_position,
            sound: fb.offset.as_ref().and_then(|o| o.sound),
        });
    } else {
        fb_for_json.offset = None;
    }

    // Short marker label for identification
    mei_fb.common.label = Some(FB_LABEL_PREFIX.to_string());

    // Store raw MusicXML JSON in ExtensionStore for direct roundtrip
    if let Some(ref id) = mei_fb.common.xml_id {
        ctx.ext_store_mut().entry(id.clone()).mxml_json = serde_json::to_value(&fb_for_json).ok();
    }

    // Create <f> children with human-readable text
    for figure in &fb.figures {
        let mut mei_f = F::default();
        let text = figure_to_text(figure);
        if !text.is_empty() {
            mei_f.children.push(FChild::Text(text));
        }
        mei_fb.children.push(FbChild::F(Box::new(mei_f)));
    }

    mei_fb
}

/// Reconstruct a MusicXML `FiguredBass` from the `@label` JSON data.
///
/// Returns `None` if the label doesn't contain valid figured-bass JSON data.
/// Deserialize a FiguredBass from a legacy JSON roundtrip label.
pub fn figured_bass_from_label(label: &str) -> Option<FiguredBass> {
    if label == FB_LABEL_PREFIX {
        return None;
    }
    let json = label.strip_prefix("musicxml:figured-bass,")?;
    serde_json::from_str(json).ok()
}

/// Generate human-readable text for a single figure.
///
/// Combines prefix, figure-number, and suffix into a compact string.
/// Examples: "b7", "6", "#5", "6n" (n=natural).
fn figure_to_text(figure: &crate::model::figured_bass::Figure) -> String {
    let mut text = String::new();
    if let Some(ref prefix) = figure.prefix {
        text.push_str(&accidental_abbrev(&prefix.value));
    }
    if let Some(ref num) = figure.figure_number {
        text.push_str(&num.value);
    }
    if let Some(ref suffix) = figure.suffix {
        text.push_str(&accidental_abbrev(&suffix.value));
    }
    text
}

/// Abbreviate accidental names for compact display.
fn accidental_abbrev(s: &str) -> String {
    match s {
        "flat" => "b".to_string(),
        "sharp" => "#".to_string(),
        "natural" => "n".to_string(),
        "double-sharp" => "x".to_string(),
        "flat-flat" => "bb".to_string(),
        "sharp-sharp" => "##".to_string(),
        "plus" => "+".to_string(),
        "slash" => "/".to_string(),
        "back-slash" => "\\".to_string(),
        "vertical" => "|".to_string(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::figured_bass::{Figure, FiguredBass};
    use crate::model::harmony::StyleText;

    fn simple_style_text(value: &str) -> StyleText {
        StyleText {
            value: value.to_string(),
            font_family: None,
            font_style: None,
            font_size: None,
            font_weight: None,
            color: None,
        }
    }

    #[test]
    fn test_figured_bass_json_roundtrip() {
        let fb = FiguredBass {
            figures: vec![Figure {
                prefix: Some(simple_style_text("flat")),
                figure_number: Some(simple_style_text("7")),
                suffix: None,
                extend: None,
            }],
            duration: None,
            offset: None,
            staff: None,
            parentheses: None,
            placement: None,
            print_object: None,
            default_x: None,
            default_y: Some(-80.0),
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
        };

        // Test legacy format label roundtrip
        let json = serde_json::to_string(&fb).unwrap();
        let label = format!("musicxml:figured-bass,{}", json);
        let recovered = figured_bass_from_label(&label).unwrap();
        assert_eq!(fb, recovered);

        // New marker label returns None (data is in ExtensionStore)
        assert!(figured_bass_from_label(FB_LABEL_PREFIX).is_none());
    }

    #[test]
    fn test_figure_to_text() {
        let fig = Figure {
            prefix: Some(simple_style_text("flat")),
            figure_number: Some(simple_style_text("7")),
            suffix: None,
            extend: None,
        };
        assert_eq!(figure_to_text(&fig), "b7");

        let fig2 = Figure {
            prefix: None,
            figure_number: Some(simple_style_text("6")),
            suffix: Some(simple_style_text("natural")),
            extend: None,
        };
        assert_eq!(figure_to_text(&fig2), "6n");
    }
}

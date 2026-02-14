//! Figured bass conversion from MusicXML to MEI.
//!
//! Converts MusicXML `<figured-bass>` elements to MEI `<fb>` measure-level
//! elements with `<f>` children. Full MusicXML data is stored in ExtensionStore
//! for lossless roundtrip; human-readable figure text is stored in `<f>` children.

use crate::context::ConversionContext;
use crate::model::figured_bass::FiguredBass;
use tusk_model::elements::{F, FChild, Fb, FbChild};
use tusk_model::musicxml_ext::{
    FigureData, FigureExtendData, FiguredBassData, OffsetData, StyleTextData, VisualAttrs,
};

/// Convert a MusicXML `<figured-bass>` element to an MEI `<fb>` element.
///
/// Data is stored in ExtensionStore for lossless roundtrip.
/// Human-readable figure text (e.g. "b7", "6", "#") is stored in `<f>` children.
pub fn convert_figured_bass(fb: &FiguredBass, ctx: &mut ConversionContext) -> Fb {
    let mut mei_fb = Fb::default();

    let fb_id = ctx.generate_id_with_suffix("fb");
    mei_fb.common.xml_id = Some(fb_id);

    // Canonicalize offset to absolute position
    let abs_position = ctx.beat_position() + fb.offset.as_ref().map(|o| o.value).unwrap_or(0.0);
    let offset = if abs_position != 0.0 || fb.offset.is_some() {
        Some(OffsetData {
            value: abs_position,
            sound: fb
                .offset
                .as_ref()
                .and_then(|o| o.sound.map(|s| s == crate::model::data::YesNo::Yes)),
        })
    } else {
        None
    };

    // Build typed FiguredBassData
    let data = build_figured_bass_data(fb, offset);

    // Store in ExtensionStore
    if let Some(ref id) = mei_fb.common.xml_id {
        ctx.ext_store_mut().insert_figured_bass(id.clone(), data);
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

/// Build a `FiguredBassData` from a MusicXML `FiguredBass`.
fn build_figured_bass_data(fb: &FiguredBass, offset: Option<OffsetData>) -> FiguredBassData {
    let figures = fb.figures.iter().map(build_figure_data).collect();

    let visual = build_visual_attrs(fb);

    FiguredBassData {
        figures,
        duration: fb.duration,
        footnote: fb.footnote.clone(),
        level: fb.level.clone(),
        offset,
        parentheses: fb.parentheses.map(|p| p == crate::model::data::YesNo::Yes),
        placement: fb.placement.map(|p| format!("{:?}", p).to_lowercase()),
        print_object: fb.print_object.map(|p| p == crate::model::data::YesNo::Yes),
        visual: if visual != VisualAttrs::default() {
            Some(visual)
        } else {
            None
        },
        halign: fb.halign.clone(),
        valign: fb.valign.clone(),
        id: fb.id.clone(),
    }
}

/// Build a `FigureData` from a MusicXML `Figure`.
fn build_figure_data(fig: &crate::model::figured_bass::Figure) -> FigureData {
    FigureData {
        prefix: fig.prefix.as_ref().map(build_style_text_data),
        figure_number: fig.figure_number.as_ref().map(build_style_text_data),
        suffix: fig.suffix.as_ref().map(build_style_text_data),
        extend: fig.extend.as_ref().map(build_figure_extend_data),
    }
}

/// Build a `StyleTextData` from a MusicXML `StyleText`.
fn build_style_text_data(st: &crate::model::harmony::StyleText) -> StyleTextData {
    let visual = VisualAttrs {
        font_family: st.font_family.clone(),
        font_size: st.font_size,
        font_style: st.font_style.clone(),
        font_weight: st.font_weight.clone(),
        color: st.color.clone(),
        ..Default::default()
    };
    StyleTextData {
        value: st.value.clone(),
        visual: if visual != VisualAttrs::default() {
            Some(visual)
        } else {
            None
        },
    }
}

/// Build a `FigureExtendData` from a MusicXML `FigureExtend`.
fn build_figure_extend_data(ext: &crate::model::figured_bass::FigureExtend) -> FigureExtendData {
    let visual = VisualAttrs {
        default_x: ext.default_x,
        default_y: ext.default_y,
        relative_x: ext.relative_x,
        relative_y: ext.relative_y,
        color: ext.color.clone(),
        ..Default::default()
    };
    FigureExtendData {
        extend_type: ext.extend_type.map(|t| format!("{:?}", t).to_lowercase()),
        visual: if visual != VisualAttrs::default() {
            Some(visual)
        } else {
            None
        },
    }
}

/// Build visual attributes from FiguredBass position/font/color fields.
fn build_visual_attrs(fb: &FiguredBass) -> VisualAttrs {
    VisualAttrs {
        font_family: fb.font_family.clone(),
        font_size: fb.font_size,
        font_style: fb.font_style.clone(),
        font_weight: fb.font_weight.clone(),
        color: fb.color.clone(),
        default_x: fb.default_x,
        default_y: fb.default_y,
        relative_x: fb.relative_x,
        relative_y: fb.relative_y,
    }
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
    use crate::model::figured_bass::Figure;
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

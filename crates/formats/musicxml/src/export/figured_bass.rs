//! Figured bass export from MEI to MusicXML.
//!
//! Converts MEI `<fb>` elements back to MusicXML `<figured-bass>` elements.
//! Reads typed `FiguredBassData` from `ExtensionStore` for lossless roundtrip.

use crate::context::ConversionContext;
use crate::model::data::{AboveBelow, StartStopContinue, YesNo};
use crate::model::direction::Offset;
use crate::model::elements::MeasureContent;
use crate::model::figured_bass::{Figure, FigureExtend, FiguredBass};
use crate::model::harmony::StyleText;
use tusk_model::elements::Fb;
use tusk_model::musicxml_ext::{FigureData, FiguredBassData, StyleTextData};

/// Convert an MEI `<fb>` element to a MusicXML `FiguredBass` measure content.
///
/// Reads typed data from ExtensionStore. Falls back to constructing minimal
/// FiguredBass from MEI `<f>` children text if no extension data is found.
pub fn convert_mei_fb(
    fb: &Fb,
    local_staff_n: usize,
    ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    // Preferred: reconstruct from typed ExtensionStore data
    if let Some(id) = &fb.common.xml_id {
        if let Some(data) = ctx.ext_store().figured_bass_data(id) {
            let data = data.clone();
            let mut figured_bass = build_figured_bass_from_data(&data);
            figured_bass.staff = Some(local_staff_n as u32);
            return Some(MeasureContent::FiguredBass(Box::new(figured_bass)));
        }
    }

    // Fallback: create a minimal figured-bass from fb children
    let figured_bass = create_fallback_figured_bass(fb, local_staff_n);
    Some(MeasureContent::FiguredBass(Box::new(figured_bass)))
}

/// Reconstruct a MusicXML `FiguredBass` from typed `FiguredBassData`.
fn build_figured_bass_from_data(data: &FiguredBassData) -> FiguredBass {
    let figures = data.figures.iter().map(build_figure_from_data).collect();

    let offset = data.offset.as_ref().map(|o| Offset {
        value: o.value,
        sound: o.sound.map(|b| if b { YesNo::Yes } else { YesNo::No }),
    });

    let footnote = data.footnote.clone();

    let level = data.level.clone();

    let parentheses = data
        .parentheses
        .map(|b| if b { YesNo::Yes } else { YesNo::No });
    let print_object = data
        .print_object
        .map(|b| if b { YesNo::Yes } else { YesNo::No });
    let placement = data.placement.as_deref().and_then(parse_above_below);

    let (
        default_x,
        default_y,
        relative_x,
        relative_y,
        font_family,
        font_style,
        font_size,
        font_weight,
        color,
    ) = if let Some(ref vis) = data.visual {
        (
            vis.default_x,
            vis.default_y,
            vis.relative_x,
            vis.relative_y,
            vis.font_family.clone(),
            vis.font_style.clone(),
            vis.font_size,
            vis.font_weight.clone(),
            vis.color.clone(),
        )
    } else {
        (None, None, None, None, None, None, None, None, None)
    };

    FiguredBass {
        figures,
        duration: data.duration,
        footnote,
        level,
        offset,
        staff: None, // set by caller
        parentheses,
        placement,
        print_object,
        default_x,
        default_y,
        relative_x,
        relative_y,
        font_family,
        font_style,
        font_size,
        font_weight,
        color,
        halign: data.halign.clone(),
        valign: data.valign.clone(),
        id: data.id.clone(),
    }
}

/// Build a MusicXML Figure from typed FigureData.
fn build_figure_from_data(fig: &FigureData) -> Figure {
    Figure {
        prefix: fig.prefix.as_ref().map(build_style_text),
        figure_number: fig.figure_number.as_ref().map(build_style_text),
        suffix: fig.suffix.as_ref().map(build_style_text),
        extend: fig.extend.as_ref().map(build_figure_extend),
    }
}

/// Build a MusicXML StyleText from StyleTextData.
fn build_style_text(st: &StyleTextData) -> StyleText {
    let (font_family, font_style, font_size, font_weight, color) = if let Some(ref vis) = st.visual
    {
        (
            vis.font_family.clone(),
            vis.font_style.clone(),
            vis.font_size,
            vis.font_weight.clone(),
            vis.color.clone(),
        )
    } else {
        (None, None, None, None, None)
    };
    StyleText {
        value: st.value.clone(),
        font_family,
        font_style,
        font_size,
        font_weight,
        color,
    }
}

/// Build a MusicXML FigureExtend from FigureExtendData.
fn build_figure_extend(ext: &tusk_model::musicxml_ext::FigureExtendData) -> FigureExtend {
    let extend_type = ext
        .extend_type
        .as_deref()
        .and_then(parse_start_stop_continue);
    let (default_x, default_y, relative_x, relative_y, color) = if let Some(ref vis) = ext.visual {
        (
            vis.default_x,
            vis.default_y,
            vis.relative_x,
            vis.relative_y,
            vis.color.clone(),
        )
    } else {
        (None, None, None, None, None)
    };
    FigureExtend {
        extend_type,
        default_x,
        default_y,
        relative_x,
        relative_y,
        color,
    }
}

fn parse_above_below(s: &str) -> Option<AboveBelow> {
    match s {
        "above" => Some(AboveBelow::Above),
        "below" => Some(AboveBelow::Below),
        _ => None,
    }
}

fn parse_start_stop_continue(s: &str) -> Option<StartStopContinue> {
    match s {
        "start" => Some(StartStopContinue::Start),
        "stop" => Some(StartStopContinue::Stop),
        "continue" => Some(StartStopContinue::Continue),
        _ => None,
    }
}

/// Create a minimal MusicXML FiguredBass from an MEI fb with no roundtrip data.
fn create_fallback_figured_bass(fb: &Fb, local_staff_n: usize) -> FiguredBass {
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
        footnote: None,
        level: None,
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

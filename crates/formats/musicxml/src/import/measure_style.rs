//! Measure-style conversion from MusicXML to MEI.
//!
//! Converts MusicXML `<measure-style>` elements (inside `<attributes>`) to MEI
//! `<dir>` control events with typed data in ExtensionStore for lossless roundtrip.

use crate::context::ConversionContext;
use crate::model::attributes::MeasureStyle;
use tusk_model::elements::{Dir, DirChild, MeasureChild};
use tusk_model::musicxml_ext::{MeasureStyleContentData, MeasureStyleData};

/// Convert MusicXML `<measure-style>` elements to MEI `<dir>` measure children.
///
/// Each measure-style becomes a dir element with typed MeasureStyleData stored
/// in `ExtensionStore.measure_styles` keyed by the dir's xml:id.
pub fn convert_measure_styles(
    styles: &[MeasureStyle],
    ctx: &mut ConversionContext,
) -> Vec<MeasureChild> {
    styles
        .iter()
        .filter_map(|ms| convert_one(ms, ctx))
        .collect()
}

fn convert_one(ms: &MeasureStyle, ctx: &mut ConversionContext) -> Option<MeasureChild> {
    let mut dir = Dir::default();
    dir.common.xml_id = Some(ctx.generate_id_with_suffix("mstyle"));

    // Store typed MeasureStyleData in ExtensionStore (no label, no mxml_json)
    if let Some(ref id) = dir.common.xml_id {
        ctx.ext_store_mut()
            .insert_measure_style(id.clone(), build_measure_style_data(ms));
    }

    // Human-readable summary as text child
    let summary = measure_style_summary(ms);
    if !summary.is_empty() {
        dir.children.push(DirChild::Text(summary));
    }

    // tstamp = 1 (measure-style applies to whole measure, not a specific beat;
    // using a fixed tstamp ensures stable roundtrip regardless of where the
    // <attributes> block appears in the MusicXML measure content)
    dir.dir_log.tstamp = Some(tusk_model::data::DataBeat::from(1.0));

    // Staff
    dir.dir_log.staff = Some(ctx.current_staff().to_string());

    Some(MeasureChild::Dir(Box::new(dir)))
}

fn measure_style_summary(ms: &MeasureStyle) -> String {
    use crate::model::attributes::MeasureStyleContent;
    match &ms.content {
        MeasureStyleContent::MultipleRest(mr) => format!("multiple-rest: {}", mr.value),
        MeasureStyleContent::MeasureRepeat(mr) => {
            format!("measure-repeat: {}", mr.repeat_type)
        }
        MeasureStyleContent::BeatRepeat(br) => {
            format!("beat-repeat: {}", br.repeat_type)
        }
        MeasureStyleContent::Slash(sl) => {
            format!("slash: {}", sl.slash_type)
        }
    }
}

fn build_measure_style_data(ms: &MeasureStyle) -> MeasureStyleData {
    use crate::model::attributes::MeasureStyleContent;
    use crate::model::data::YesNo;

    let content = match &ms.content {
        MeasureStyleContent::MultipleRest(mr) => MeasureStyleContentData::MultipleRest {
            value: mr.value,
            use_symbols: mr.use_symbols.map(|v| matches!(v, YesNo::Yes)),
        },
        MeasureStyleContent::MeasureRepeat(mr) => MeasureStyleContentData::MeasureRepeat {
            value: mr.value,
            repeat_type: mr.repeat_type.to_string(),
            slashes: mr.slashes,
        },
        MeasureStyleContent::BeatRepeat(br) => MeasureStyleContentData::BeatRepeat {
            repeat_type: br.repeat_type.to_string(),
            slashes: br.slashes,
            use_dots: br.use_dots.map(|v| matches!(v, YesNo::Yes)),
        },
        MeasureStyleContent::Slash(sl) => MeasureStyleContentData::Slash {
            slash_type: sl.slash_type.to_string(),
            use_stems: sl.use_stems.map(|v| matches!(v, YesNo::Yes)),
            use_dots: sl.use_dots.map(|v| matches!(v, YesNo::Yes)),
        },
    };

    MeasureStyleData {
        number: ms.number,
        content,
    }
}

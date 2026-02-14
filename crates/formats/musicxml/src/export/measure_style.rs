//! Measure-style export from MEI to MusicXML.
//!
//! Converts MEI `<dir>` elements with typed MeasureStyleData in ExtensionStore
//! back to MusicXML `<measure-style>` inside `<attributes>`.

use crate::context::ConversionContext;
use crate::model::attributes::{
    Attributes, BeatRepeat, MeasureRepeat, MeasureStyle, MeasureStyleContent, MultipleRest, Slash,
};
use crate::model::data::YesNo;
use crate::model::elements::MeasureContent;
use tusk_model::elements::Dir;
use tusk_model::musicxml_ext::{MeasureStyleContentData, MeasureStyleData};

/// Convert an MEI `<dir>` with measure-style data to MusicXML `<attributes>`.
///
/// Reads typed MeasureStyleData from ExtensionStore and reconstructs the
/// MusicXML MeasureStyle struct.
pub fn convert_mei_measure_style_dir(
    dir: &Dir,
    ctx: &mut ConversionContext,
) -> Option<MeasureContent> {
    let id = dir.common.xml_id.as_ref()?;
    let data = ctx.ext_store().measure_style(id)?;
    let ms = build_measure_style_from_data(data);
    let attrs = Attributes {
        measure_styles: vec![ms],
        ..Default::default()
    };
    Some(MeasureContent::Attributes(Box::new(attrs)))
}

/// Build a MusicXML `MeasureStyle` from typed `MeasureStyleData`.
fn build_measure_style_from_data(data: &MeasureStyleData) -> MeasureStyle {
    let bool_to_yesno = |b| if b { YesNo::Yes } else { YesNo::No };

    let content = match &data.content {
        MeasureStyleContentData::MultipleRest { value, use_symbols } => {
            MeasureStyleContent::MultipleRest(MultipleRest {
                value: *value,
                use_symbols: use_symbols.map(bool_to_yesno),
            })
        }
        MeasureStyleContentData::MeasureRepeat {
            value,
            repeat_type,
            slashes,
        } => MeasureStyleContent::MeasureRepeat(MeasureRepeat {
            value: *value,
            repeat_type: repeat_type
                .parse()
                .unwrap_or(crate::model::data::StartStop::Start),
            slashes: *slashes,
        }),
        MeasureStyleContentData::BeatRepeat {
            repeat_type,
            slashes,
            use_dots,
        } => MeasureStyleContent::BeatRepeat(BeatRepeat {
            repeat_type: repeat_type
                .parse()
                .unwrap_or(crate::model::data::StartStopContinue::Start),
            slashes: *slashes,
            use_dots: use_dots.map(bool_to_yesno),
            slash_type: None,
            slash_dots: vec![],
            except_voices: vec![],
        }),
        MeasureStyleContentData::Slash {
            slash_type,
            use_stems,
            use_dots,
        } => MeasureStyleContent::Slash(Slash {
            slash_type: slash_type
                .parse()
                .unwrap_or(crate::model::data::StartStop::Start),
            use_stems: use_stems.map(bool_to_yesno),
            use_dots: use_dots.map(bool_to_yesno),
            slash_type_element: None,
            slash_dots: vec![],
            except_voices: vec![],
        }),
    };

    MeasureStyle {
        number: data.number,
        content,
    }
}

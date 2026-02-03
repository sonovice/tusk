//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe default typography of lyrics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLyricStyle {
    ///Describes the alignment of lyric syllables associated with a note or chord.
    #[serde(rename = "@lyric.align", skip_serializing_if = "Option::is_none")]
    pub lyric_align: Option<crate::generated::data::DataMeasurementsigned>,
    ///Sets the font family default value for lyrics.
    #[serde(rename = "@lyric.fam", skip_serializing_if = "Option::is_none")]
    pub lyric_fam: Option<crate::generated::data::DataFontfamily>,
    ///Sets the font name default value for lyrics.
    #[serde(rename = "@lyric.name", skip_serializing_if = "Option::is_none")]
    pub lyric_name: Option<crate::generated::data::DataFontname>,
    ///Sets the default font size value for lyrics.
    #[serde(rename = "@lyric.size", skip_serializing_if = "Option::is_none")]
    pub lyric_size: Option<crate::generated::data::DataFontsize>,
    ///Sets the default font style value for lyrics.
    #[serde(rename = "@lyric.style", skip_serializing_if = "Option::is_none")]
    pub lyric_style: Option<crate::generated::data::DataFontstyle>,
    ///Sets the default font weight value for lyrics.
    #[serde(rename = "@lyric.weight", skip_serializing_if = "Option::is_none")]
    pub lyric_weight: Option<crate::generated::data::DataFontweight>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that capture music font name and size.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNotationStyle {
    ///Sets the default music font name.
    #[serde(rename = "@music.name", skip_serializing_if = "Option::is_none")]
    pub music_name: Option<crate::generated::data::DataMusicfont>,
    ///Sets the default music font size.
    #[serde(rename = "@music.size", skip_serializing_if = "Option::is_none")]
    pub music_size: Option<crate::generated::data::DataFontsize>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that specify names or values taken from an external symbol authority.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttExtSymNames {
    ///Glyph name.
    #[serde(rename = "@glyph.name", skip_serializing_if = "Option::is_none")]
    pub glyph_name: Option<String>,
    /**Numeric glyph reference in hexadecimal notation,e.g., "#xE000" or "U+E000". N.B. SMuFL
    version 1.18 uses the range U+E000 - U+ECBF.*/
    #[serde(rename = "@glyph.num", skip_serializing_if = "Option::is_none")]
    pub glyph_num: Option<crate::generated::data::DataHexnum>,
}

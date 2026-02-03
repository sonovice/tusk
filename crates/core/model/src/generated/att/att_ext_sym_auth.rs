//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttExtSymAuthGlyphAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
///Attributes that point to an external symbol authority.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttExtSymAuth {
    ///A name or label associated with the controlled vocabulary from which the value ofglyph.nameorglyph.numis taken, or the textual content of the element.
    #[serde(rename = "@glyph.auth", skip_serializing_if = "Option::is_none")]
    pub glyph_auth: Option<AttExtSymAuthGlyphAuth>,
    ///The web-accessible location of the controlled vocabulary from which the value ofglyph.nameorglyph.numis taken, or the textual content of the element.
    #[serde(rename = "@glyph.uri", skip_serializing_if = "Option::is_none")]
    pub glyph_uri: Option<crate::generated::data::DataUri>,
}

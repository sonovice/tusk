//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttBarLineVisGlyphAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBarLineVis {
    /**Provides a way of pointing to a user-defined symbol. It must contain a reference to an
    ID of asymbolDefelement elsewhere in the document.*/
    #[serde(rename = "@altsym", skip_serializing_if = "Option::is_none")]
    pub altsym: Option<crate::generated::data::DataUri>,
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
    as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
    ///A name or label associated with the controlled vocabulary from which the value ofglyph.nameorglyph.numis taken, or the textual content of the element.
    #[serde(rename = "@glyph.auth", skip_serializing_if = "Option::is_none")]
    pub glyph_auth: Option<AttBarLineVisGlyphAuth>,
    ///The web-accessible location of the controlled vocabulary from which the value ofglyph.nameorglyph.numis taken, or the textual content of the element.
    #[serde(rename = "@glyph.uri", skip_serializing_if = "Option::is_none")]
    pub glyph_uri: Option<crate::generated::data::DataUri>,
    ///Glyph name.
    #[serde(rename = "@glyph.name", skip_serializing_if = "Option::is_none")]
    pub glyph_name: Option<String>,
    /**Numeric glyph reference in hexadecimal notation,e.g., "#xE000" or "U+E000". N.B. SMuFL
    version 1.18 uses the range U+E000 - U+ECBF.*/
    #[serde(rename = "@glyph.num", skip_serializing_if = "Option::is_none")]
    pub glyph_num: Option<crate::generated::data::DataHexnum>,
    /**Indicates if a feature should be rendered when the notation is presented graphically
    or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
    ///Measurement of the horizontal dimension of an entity.
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<crate::generated::data::DataMeasurementunsigned>,
    /**States the length of bar lines in virtual units. The value must be greater than 0 and
    is typically equal to 2 times (the number of staff lines - 1);e.g., a value of8for a
    5-line staff.*/
    #[serde(rename = "@len", skip_serializing_if = "Option::is_none")]
    pub len: Option<f64>,
    ///Records the method of barring.
    #[serde(rename = "@method", skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::generated::data::DataBarmethod>,
    ///Denotes the staff location of the bar line if its length is non-standard.
    #[serde(rename = "@place", skip_serializing_if = "Option::is_none")]
    pub place: Option<crate::generated::data::DataStaffloc>,
}

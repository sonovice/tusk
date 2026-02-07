//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttStemVisGlyphAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
///Visual domain attributes that describe the properties of a stem in the mensural repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStemVis {
    /**Encodes an x coordinate for a feature in an output coordinate system. When it is
          necessary to record the placement of a feature in a facsimile image, use the facs
          attribute.*/
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /**Encodes a y coordinate for a feature in an output coordinate system. When it is
          necessary to record the placement of a feature in a facsimile image, use the facs
          attribute.*/
    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
          as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
    ///A name or label associated with the controlled vocabulary from which the value ofglyph.nameorglyph.numis taken, or the textual content of the element.
    #[serde(rename = "@glyph.auth", skip_serializing_if = "Option::is_none")]
    pub glyph_auth: Option<AttStemVisGlyphAuth>,
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
    ///Records the position of the stem in relation to the note head(s).
    #[serde(rename = "@pos", skip_serializing_if = "Option::is_none")]
    pub pos: Option<crate::generated::data::DataStemposition>,
    ///Encodes the stem length.
    #[serde(rename = "@len", skip_serializing_if = "Option::is_none")]
    pub len: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Encodes the form of the stem using the values provided by the data.STEMFORM.mensural datatype.
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<crate::generated::data::DataStemformMensural>,
    ///Describes the direction of a stem.
    #[serde(rename = "@dir", skip_serializing_if = "Option::is_none")]
    pub dir: Option<crate::generated::data::DataStemdirection>,
    ///Records the position of the flag using the values provided by the data.FLAGPOS.mensural datatype.
    #[serde(rename = "@flag.pos", skip_serializing_if = "Option::is_none")]
    pub flag_pos: Option<crate::generated::data::DataFlagposMensural>,
    ///Encodes the form of the flag using the values provided by the data.FLAGFORM.mensural datatype.
    #[serde(rename = "@flag.form", skip_serializing_if = "Option::is_none")]
    pub flag_form: Option<crate::generated::data::DataFlagformMensural>,
}

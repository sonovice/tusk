//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMensurVisGlyphAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMensurVisForm {
    ///Horizontally oriented.
    #[serde(rename = "horizontal")]
    Horizontal,
    ///Vertically oriented.
    #[serde(rename = "vertical")]
    Vertical,
}
/**Visual domain attributes. These attributes describe the physical appearance of the
mensuration sign/time signature of mensural notation.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMensurVis {
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
    pub glyph_auth: Option<AttMensurVisGlyphAuth>,
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
    ///Holds the staff location of the feature.
    #[serde(rename = "@loc", skip_serializing_if = "Option::is_none")]
    pub loc: Option<crate::generated::data::DataStaffloc>,
    ///Contains the name of a font-family.
    #[serde(rename = "@fontfam", skip_serializing_if = "Option::is_none")]
    pub fontfam: Option<crate::generated::data::DataFontfamily>,
    ///Holds the name of a font.
    #[serde(rename = "@fontname", skip_serializing_if = "Option::is_none")]
    pub fontname: Option<crate::generated::data::DataFontname>,
    /**Indicates the size of a font expressed in printers' points,i.e., 1/72nd of an inch,
    relative terms,e.g.,small,larger,etc., or percentage values relative tonormalsize,e.g.,125%.*/
    #[serde(rename = "@fontsize", skip_serializing_if = "Option::is_none")]
    pub fontsize: Option<crate::generated::data::DataFontsize>,
    ///Records the style of a font,i.e.,italic,oblique, ornormal.
    #[serde(rename = "@fontstyle", skip_serializing_if = "Option::is_none")]
    pub fontstyle: Option<crate::generated::data::DataFontstyle>,
    ///Used to indicate bold type.
    #[serde(rename = "@fontweight", skip_serializing_if = "Option::is_none")]
    pub fontweight: Option<crate::generated::data::DataFontweight>,
    /**Indicates letter spacing (aka tracking) in analogy to the CSS letter-spacing
    property.*/
    #[serde(rename = "@letterspacing", skip_serializing_if = "Option::is_none")]
    pub letterspacing: Option<crate::generated::data::DataMeasurementtypographysigned>,
    ///Indicates line height in analogy to the CSS line-height property.
    #[serde(rename = "@lineheight", skip_serializing_if = "Option::is_none")]
    pub lineheight: Option<crate::generated::data::DataPercent>,
    ///Indicates the number of slashes present.
    #[serde(rename = "@slash", skip_serializing_if = "Option::is_none")]
    pub slash: Option<crate::generated::data::DataSlash>,
    ///Specifies whether a dot is to be added to the base symbol.
    #[serde(rename = "@dot", skip_serializing_if = "Option::is_none")]
    pub dot: Option<crate::generated::data::DataBoolean>,
    ///Indicates whether the base symbol is written vertically or horizontally.
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<AttMensurVisForm>,
    ///Describes the rotation or reflection of the base symbol.
    #[serde(rename = "@orient", skip_serializing_if = "Option::is_none")]
    pub orient: Option<crate::generated::data::DataOrientation>,
    ///The base symbol in the mensuration sign/time signature of mensural notation.
    #[serde(rename = "@sign", skip_serializing_if = "Option::is_none")]
    pub sign: Option<crate::generated::data::DataMensurationsign>,
}

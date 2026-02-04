//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMultiRestVisGlyphAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMultiRestVis {
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
    pub glyph_auth: Option<AttMultiRestVisGlyphAuth>,
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
    ///States where the number will be placed in relation to the notational feature.
    #[serde(rename = "@num.place", skip_serializing_if = "Option::is_none")]
    pub num_place: Option<crate::generated::data::DataStaffrelBasic>,
    ///Determines if the number is visible.
    #[serde(rename = "@num.visible", skip_serializing_if = "Option::is_none")]
    pub num_visible: Option<crate::generated::data::DataBoolean>,
    ///Holds the staff location of the feature.
    #[serde(rename = "@loc", skip_serializing_if = "Option::is_none")]
    pub loc: Option<crate::generated::data::DataStaffloc>,
    ///Captures staff location in terms of written pitch name.
    #[serde(rename = "@ploc", skip_serializing_if = "Option::is_none")]
    pub ploc: Option<crate::generated::data::DataPitchname>,
    ///Records staff location in terms of written octave.
    #[serde(rename = "@oloc", skip_serializing_if = "Option::is_none")]
    pub oloc: Option<crate::generated::data::DataOctave>,
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
    /**Indicates if a feature should be rendered when the notation is presented graphically
    or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
    ///Measurement of the horizontal dimension of an entity.
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<crate::generated::data::DataMeasurementunsigned>,
    /**The block attribute controls whether the multimeasure rest should be rendered as a block rest
    or as church rests ("Kirchenpausen"), that are combinations of longa, breve and semibreve rests.*/
    #[serde(rename = "@block", skip_serializing_if = "Option::is_none")]
    pub block: Option<crate::generated::data::DataBoolean>,
}

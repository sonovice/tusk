//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttArpegVisGlyphAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttArpegVis {
    /**Provides a way of pointing to a user-defined symbol. It must contain a reference to an
          ID of asymbolDefelement elsewhere in the document.*/
    #[serde(rename = "@altsym", skip_serializing_if = "Option::is_none")]
    pub altsym: Option<crate::generated::data::DataUri>,
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
          as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
    /**Records the characters often used to mark accidentals, articulations, and sometimes
          notes as having a cautionary or editorial function. For an example of cautionary
          accidentals enclosed in parentheses, see Read, p. 131, ex. 9-14.*/
    #[serde(rename = "@enclose", skip_serializing_if = "Option::is_none")]
    pub enclose: Option<crate::generated::data::DataEnclosure>,
    ///A name or label associated with the controlled vocabulary from which the value ofglyph.nameorglyph.numis taken, or the textual content of the element.
    #[serde(rename = "@glyph.auth", skip_serializing_if = "Option::is_none")]
    pub glyph_auth: Option<AttArpegVisGlyphAuth>,
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
    ///Describes the style of a line.
    #[serde(rename = "@lform", skip_serializing_if = "Option::is_none")]
    pub lform: Option<crate::generated::data::DataLineform>,
    ///Width of a line.
    #[serde(rename = "@lwidth", skip_serializing_if = "Option::is_none")]
    pub lwidth: Option<crate::generated::data::DataLinewidth>,
    /**Describes the number of segments into which a dashed or dotted line may be divided, or
          the number of "peaks" of a wavy line; a pair of space-separated values (minimum and
          maximum, respectively) provides a range between which a rendering system-supplied value
          may fall, while a single value indicates a fixed amount of space; that is, the minimum and
          maximum values are equal.*/
    #[serde(rename = "@lsegs", skip_serializing_if = "Option::is_none")]
    pub lsegs: Option<u64>,
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
    /**Records a horizontal adjustment to a feature’s programmatically-determined location in
          terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
          staff lines.*/
    #[serde(rename = "@ho", skip_serializing_if = "Option::is_none")]
    pub ho: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records a timestamp adjustment of a feature’s programmatically-determined location in
          terms of musical time; that is, beats.*/
    #[serde(rename = "@to", skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::generated::data::DataTstampoffset>,
    /**Records the vertical adjustment of a feature’s programmatically-determined location in
          terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
          staff lines.*/
    #[serde(rename = "@vo", skip_serializing_if = "Option::is_none")]
    pub vo: Option<crate::generated::data::DataMeasurementsigned>,
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
    ///Indicates if an arrowhead is to be drawn as part of the arpeggiation symbol.
    #[serde(rename = "@arrow", skip_serializing_if = "Option::is_none")]
    pub arrow: Option<crate::generated::data::DataBoolean>,
    ///Symbol rendered at end of the line.
    #[serde(rename = "@arrow.shape", skip_serializing_if = "Option::is_none")]
    pub arrow_shape: Option<crate::generated::data::DataLinestartendsymbol>,
    ///Holds the relative size of the arrow symbol.
    #[serde(rename = "@arrow.size", skip_serializing_if = "Option::is_none")]
    pub arrow_size: Option<crate::generated::data::DataFontsizescale>,
    ///Captures the overall color of the arrow.
    #[serde(rename = "@arrow.color", skip_serializing_if = "Option::is_none")]
    pub arrow_color: Option<crate::generated::data::DataColor>,
    ///Captures the fill color of the arrow if different from the line color.
    #[serde(rename = "@arrow.fillcolor", skip_serializing_if = "Option::is_none")]
    pub arrow_fillcolor: Option<crate::generated::data::DataColor>,
}

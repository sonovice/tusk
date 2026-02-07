//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNcVisGlyphAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNcVisCurve {
    ///Anti-clockwise curvature.
    #[serde(rename = "a")]
    A,
    ///Clockwise curvature.
    #[serde(rename = "c")]
    C,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNcVisCon {
    ///Gapped; not connected.
    #[serde(rename = "g")]
    G,
    ///Looped.
    #[serde(rename = "l")]
    L,
    ///Extended.
    #[serde(rename = "e")]
    E,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNcVisRellen {
    ///Longer.
    #[serde(rename = "l")]
    L,
    ///Shorter.
    #[serde(rename = "s")]
    S,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNcVis {
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
    pub glyph_auth: Option<AttNcVisGlyphAuth>,
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
    ///Records direction of curvature.
    #[serde(rename = "@curve", skip_serializing_if = "Option::is_none")]
    pub curve: Option<AttNcVisCurve>,
    ///
    #[serde(rename = "@angled", skip_serializing_if = "Option::is_none")]
    pub angled: Option<crate::generated::data::DataBoolean>,
    /**Connection to the previous component within the same neume; this attribute should not
          be used for the first component of a neume.*/
    #[serde(rename = "@con", skip_serializing_if = "Option::is_none")]
    pub con: Option<AttNcVisCon>,
    ///Pen stroke has an extension; specific to Hispanic notation.
    #[serde(rename = "@hooked", skip_serializing_if = "Option::is_none")]
    pub hooked: Option<crate::generated::data::DataBoolean>,
    ///Indicates participation in a ligature.
    #[serde(rename = "@ligated", skip_serializing_if = "Option::is_none")]
    pub ligated: Option<crate::generated::data::DataBoolean>,
    /**Length of the pen stroke relative to the previous component within the same neume;
          this attribute should not be used for the first component of a neume.*/
    #[serde(rename = "@rellen", skip_serializing_if = "Option::is_none")]
    pub rellen: Option<AttNcVisRellen>,
    /**Direction of the initial direction for an s-shaped pen stroke;i.e., "w" for the
          standard letter S, "e" for its mirror image, "s" for the letter S turned 90-degrees
          anti-clockwise, and "n" for its mirror image.*/
    #[serde(rename = "@s-shape", skip_serializing_if = "Option::is_none")]
    pub s_shape: Option<crate::generated::data::DataCompassdirectionBasic>,
    ///Direction of the pen stroke.
    #[serde(rename = "@tilt", skip_serializing_if = "Option::is_none")]
    pub tilt: Option<crate::generated::data::DataCompassdirection>,
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
    /**Indicates if a feature should be rendered when the notation is presented graphically
          or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
    /**Records a horizontal adjustment to a feature’s programmatically-determined location in
          terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
          staff lines.*/
    #[serde(rename = "@ho", skip_serializing_if = "Option::is_none")]
    pub ho: Option<crate::generated::data::DataMeasurementsigned>,
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
}

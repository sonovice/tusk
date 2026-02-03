//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNoteVisGlyphAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNoteVisHeadAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteVis {
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
    pub glyph_auth: Option<AttNoteVisGlyphAuth>,
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
    /**Provides a way of pointing to a user-defined symbol. It must contain a reference to an
    ID of asymbolDefelement elsewhere in the document.*/
    #[serde(rename = "@head.altsym", skip_serializing_if = "Option::is_none")]
    pub head_altsym: Option<crate::generated::data::DataUri>,
    /**A name or label associated with the controlled vocabulary from which a numerical value
    ofhead.shapeis taken.*/
    #[serde(rename = "@head.auth", skip_serializing_if = "Option::is_none")]
    pub head_auth: Option<AttNoteVisHeadAuth>,
    ///Captures the overall color of a notehead.
    #[serde(rename = "@head.color", skip_serializing_if = "Option::is_none")]
    pub head_color: Option<crate::generated::data::DataColor>,
    ///Describes how/if the notehead is filled.
    #[serde(rename = "@head.fill", skip_serializing_if = "Option::is_none")]
    pub head_fill: Option<crate::generated::data::DataFill>,
    ///Captures the fill color of a notehead if different from the overall note color.
    #[serde(rename = "@head.fillcolor", skip_serializing_if = "Option::is_none")]
    pub head_fillcolor: Option<crate::generated::data::DataColor>,
    ///Records any additional symbols applied to the notehead.
    #[serde(rename = "@head.mod", default, skip_serializing_if = "Vec::is_empty")]
    pub head_mod: Vec<crate::generated::data::DataNoteheadmodifier>,
    /**Describes rotation applied to the basic notehead shape. A positive value rotates the
    notehead in a counter-clockwise fashion, while negative values produce clockwise
    rotation.*/
    #[serde(rename = "@head.rotation", skip_serializing_if = "Option::is_none")]
    pub head_rotation: Option<crate::generated::data::DataRotation>,
    ///Used to override the head shape normally used for the given duration.
    #[serde(rename = "@head.shape", skip_serializing_if = "Option::is_none")]
    pub head_shape: Option<crate::generated::data::DataHeadshape>,
    /**Indicates if a feature should be rendered when the notation is presented graphically
    or sounded when it is presented in an aural form.*/
    #[serde(rename = "@head.visible", skip_serializing_if = "Option::is_none")]
    pub head_visible: Option<crate::generated::data::DataBoolean>,
    ///Holds the staff location of the feature.
    #[serde(rename = "@loc", skip_serializing_if = "Option::is_none")]
    pub loc: Option<crate::generated::data::DataStaffloc>,
    /**Contains an indication of which staff a note or chord that logically belongs to the
    current staff should be visually placed on; that is, the one above or the one
    below.*/
    #[serde(rename = "@stem.with", skip_serializing_if = "Option::is_none")]
    pub stem_with: Option<crate::generated::data::DataNeighboringlayer>,
    ///Records the form of the stem.
    #[serde(rename = "@stem.form", skip_serializing_if = "Option::is_none")]
    pub stem_form: Option<crate::generated::data::DataStemformMensural>,
    ///Describes the direction of a stem.
    #[serde(rename = "@stem.dir", skip_serializing_if = "Option::is_none")]
    pub stem_dir: Option<crate::generated::data::DataStemdirection>,
    ///Encodes the stem length.
    #[serde(rename = "@stem.len", skip_serializing_if = "Option::is_none")]
    pub stem_len: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Encodes any stem "modifiers"; that is, symbols rendered on the stem, such as tremolo
    or Sprechstimme indicators.*/
    #[serde(rename = "@stem.mod", skip_serializing_if = "Option::is_none")]
    pub stem_mod: Option<crate::generated::data::DataStemmodifier>,
    ///Records the position of the stem in relation to the note head(s).
    #[serde(rename = "@stem.pos", skip_serializing_if = "Option::is_none")]
    pub stem_pos: Option<crate::generated::data::DataStemposition>,
    /**Points to a note element in a different layer whose stem is shared.
    The linked notes should be rendered like a chord though they are part of different layers.*/
    #[serde(rename = "@stem.sameas", skip_serializing_if = "Option::is_none")]
    pub stem_sameas: Option<crate::generated::data::DataUri>,
    ///Determines whether a stem should be displayed.
    #[serde(rename = "@stem.visible", skip_serializing_if = "Option::is_none")]
    pub stem_visible: Option<crate::generated::data::DataBoolean>,
    ///Records the output x coordinate of the stem’s attachment point.
    #[serde(rename = "@stem.x", skip_serializing_if = "Option::is_none")]
    pub stem_x: Option<f64>,
    ///Records the output y coordinate of the stem’s attachment point.
    #[serde(rename = "@stem.y", skip_serializing_if = "Option::is_none")]
    pub stem_y: Option<f64>,
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
    /**Records a timestamp adjustment of a feature’s programmatically-determined location in
    terms of musical time; that is, beats.*/
    #[serde(rename = "@to", skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::generated::data::DataTstampoffset>,
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
    /**Presence of this attribute indicates that the secondary beam should be broken
    following this note/chord. The value of the attribute records the number of beams which
    should remain unbroken.*/
    #[serde(rename = "@breaksec", skip_serializing_if = "Option::is_none")]
    pub breaksec: Option<u64>,
    ///Indicates this element’s participation in a ligature.
    #[serde(rename = "@lig", skip_serializing_if = "Option::is_none")]
    pub lig: Option<crate::generated::data::DataLigatureform>,
}

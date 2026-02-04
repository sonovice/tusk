//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMensuralVisMensurForm {
    ///Horizontally oriented.
    #[serde(rename = "horizontal")]
    Horizontal,
    ///Vertically oriented.
    #[serde(rename = "vertical")]
    Vertical,
}
/**Used by staffDef and scoreDef to provide default values for attributes in the visual
      domain related to mensuration.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMensuralVis {
    /**Records the color of the mensuration sign. Do not confuse this with the musical term
          'color' as used in pre-CMN notation.*/
    #[serde(rename = "@mensur.color", skip_serializing_if = "Option::is_none")]
    pub mensur_color: Option<crate::generated::data::DataColor>,
    ///Determines if a dot is to be added to the base symbol.
    #[serde(rename = "@mensur.dot", skip_serializing_if = "Option::is_none")]
    pub mensur_dot: Option<crate::generated::data::DataBoolean>,
    ///Indicates whether the base symbol is written vertically or horizontally.
    #[serde(rename = "@mensur.form", skip_serializing_if = "Option::is_none")]
    pub mensur_form: Option<AttMensuralVisMensurForm>,
    ///Holds the staff location of the mensuration sign.
    #[serde(rename = "@mensur.loc", skip_serializing_if = "Option::is_none")]
    pub mensur_loc: Option<crate::generated::data::DataStaffloc>,
    ///Describes the rotation or reflection of the base symbol.
    #[serde(rename = "@mensur.orient", skip_serializing_if = "Option::is_none")]
    pub mensur_orient: Option<crate::generated::data::DataOrientation>,
    ///The base symbol in the mensuration sign/time signature of mensural notation.
    #[serde(rename = "@mensur.sign", skip_serializing_if = "Option::is_none")]
    pub mensur_sign: Option<crate::generated::data::DataMensurationsign>,
    ///Describes the relative size of the mensuration sign.
    #[serde(rename = "@mensur.size", skip_serializing_if = "Option::is_none")]
    pub mensur_size: Option<crate::generated::data::DataFontsize>,
    /**Indicates the number lines added to the mensuration sign. For example, one slash is
          added for what we now call 'alla breve'.*/
    #[serde(rename = "@mensur.slash", skip_serializing_if = "Option::is_none")]
    pub mensur_slash: Option<u64>,
}

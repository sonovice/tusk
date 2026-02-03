//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNcFormCurve {
    ///Anti-clockwise curvature.
    #[serde(rename = "a")]
    A,
    ///Clockwise curvature.
    #[serde(rename = "c")]
    C,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNcFormCon {
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
pub enum AttNcFormRellen {
    ///Longer.
    #[serde(rename = "l")]
    L,
    ///Shorter.
    #[serde(rename = "s")]
    S,
}
///Attributes that record visual details of neume notation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNcForm {
    ///Records direction of curvature.
    #[serde(rename = "@curve", skip_serializing_if = "Option::is_none")]
    pub curve: Option<AttNcFormCurve>,
    ///
    #[serde(rename = "@angled", skip_serializing_if = "Option::is_none")]
    pub angled: Option<crate::generated::data::DataBoolean>,
    /**Connection to the previous component within the same neume; this attribute should not
    be used for the first component of a neume.*/
    #[serde(rename = "@con", skip_serializing_if = "Option::is_none")]
    pub con: Option<AttNcFormCon>,
    ///Pen stroke has an extension; specific to Hispanic notation.
    #[serde(rename = "@hooked", skip_serializing_if = "Option::is_none")]
    pub hooked: Option<crate::generated::data::DataBoolean>,
    ///Indicates participation in a ligature.
    #[serde(rename = "@ligated", skip_serializing_if = "Option::is_none")]
    pub ligated: Option<crate::generated::data::DataBoolean>,
    /**Length of the pen stroke relative to the previous component within the same neume;
    this attribute should not be used for the first component of a neume.*/
    #[serde(rename = "@rellen", skip_serializing_if = "Option::is_none")]
    pub rellen: Option<AttNcFormRellen>,
    /**Direction of the initial direction for an s-shaped pen stroke;i.e., "w" for the
    standard letter S, "e" for its mirror image, "s" for the letter S turned 90-degrees
    anti-clockwise, and "n" for its mirror image.*/
    #[serde(rename = "@s-shape", skip_serializing_if = "Option::is_none")]
    pub s_shape: Option<crate::generated::data::DataCompassdirectionBasic>,
    ///Direction of the pen stroke.
    #[serde(rename = "@tilt", skip_serializing_if = "Option::is_none")]
    pub tilt: Option<crate::generated::data::DataCompassdirection>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttCutoutCutout {
    ///The staff lines should not be drawn.
    #[serde(rename = "cutout")]
    Cutout,
}
/**Attributes that indicate how to render the staff lines of the measure containing an
      element belonging to this attribute class.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCutout {
    ///"Cut-out" style.
    #[serde(rename = "@cutout", skip_serializing_if = "Option::is_none")]
    pub cutout: Option<AttCutoutCutout>,
}

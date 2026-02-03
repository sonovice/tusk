//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttCurvatureDirectionCurve {
    ///Anti-clockwise curvature.
    #[serde(rename = "a")]
    A,
    ///Clockwise curvature.
    #[serde(rename = "c")]
    C,
}
///Attributes describing the direction of curvature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCurvatureDirection {
    ///Records direction of curvature.
    #[serde(rename = "@curve", skip_serializing_if = "Option::is_none")]
    pub curve: Option<AttCurvatureDirectionCurve>,
}

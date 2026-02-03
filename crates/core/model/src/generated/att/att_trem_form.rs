//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttTremFormForm {
    ///Measured tremolo.
    #[serde(rename = "meas")]
    Meas,
    ///Unmeasured tremolo.
    #[serde(rename = "unmeas")]
    Unmeas,
}
///Attributes describing the form of a tremolo.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTremForm {
    ///Describes the style of the tremolo.
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<AttTremFormForm>,
}

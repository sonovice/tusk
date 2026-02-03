//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttFTremVis {
    ///Indicates the number of beams present.
    #[serde(rename = "@beams", skip_serializing_if = "Option::is_none")]
    pub beams: Option<u64>,
    ///Captures the number of "floating" beams,i.e., those not attached to stems.
    #[serde(rename = "@beams.float", skip_serializing_if = "Option::is_none")]
    pub beams_float: Option<u64>,
    ///Records the amount of separation between floating beams and stems.
    #[serde(rename = "@float.gap", skip_serializing_if = "Option::is_none")]
    pub float_gap: Option<crate::generated::data::DataMeasurementunsigned>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttCurveLogFunc {
    ///The function of the curve is unknown.
    #[serde(rename = "unknown")]
    Unknown,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCurveLog {
    /**Holds a reference to the first element in a sequence of events to which the feature
    applies.*/
    #[serde(rename = "@startid", skip_serializing_if = "Option::is_none")]
    pub startid: Option<crate::generated::data::DataUri>,
    /**Indicates the final element in a sequence of events to which the feature
    applies.*/
    #[serde(rename = "@endid", skip_serializing_if = "Option::is_none")]
    pub endid: Option<crate::generated::data::DataUri>,
    ///Indicates the function of the curve.
    #[serde(rename = "@func", skip_serializing_if = "Option::is_none")]
    pub func: Option<AttCurveLogFunc>,
}

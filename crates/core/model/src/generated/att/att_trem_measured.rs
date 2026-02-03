//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe measured tremolandi.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTremMeasured {
    ///The performed duration of an individual note in a measured tremolo.
    #[serde(rename = "@unitdur", skip_serializing_if = "Option::is_none")]
    pub unitdur: Option<crate::generated::data::DataDurationCmn>,
}

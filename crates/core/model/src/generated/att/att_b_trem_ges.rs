//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBTremGes {
    ///The performed duration of an individual note in a measured tremolo.
    #[serde(rename = "@unitdur", skip_serializing_if = "Option::is_none")]
    pub unitdur: Option<crate::generated::data::DataDurationCmn>,
}

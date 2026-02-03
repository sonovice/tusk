//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that locate a sound source within 3-D space.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSoundLocation {
    ///The lateral or left-to-right plane.
    #[serde(rename = "@azimuth", skip_serializing_if = "Option::is_none")]
    pub azimuth: Option<crate::generated::data::DataDegrees>,
    ///The above-to-below axis.
    #[serde(rename = "@elevation", skip_serializing_if = "Option::is_none")]
    pub elevation: Option<crate::generated::data::DataDegrees>,
}

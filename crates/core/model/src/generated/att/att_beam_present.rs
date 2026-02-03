//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that indicate whether an event lies under a beam.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBeamPresent {
    ///Indicates that this event is "under a beam".
    #[serde(rename = "@beam", default, skip_serializing_if = "Vec::is_empty")]
    pub beam: Vec<crate::generated::data::DataBeam>,
}

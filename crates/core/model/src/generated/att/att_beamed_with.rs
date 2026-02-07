//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes indicating cross-staff beaming.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBeamedWith {
    /**In the case of cross-staff beams, the beam.with attribute is used to indicate which
          staff the beam is connected to; that is, the staff above or the staff below.*/
    #[serde(rename = "@beam.with", skip_serializing_if = "Option::is_none")]
    pub beam_with: Option<crate::generated::data::DataNeighboringlayer>,
}

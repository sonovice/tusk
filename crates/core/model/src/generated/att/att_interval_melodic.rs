//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that provide for description of intervallic content.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttIntervalMelodic {
    /**Encodes the melodic interval from the previous pitch. The value may be a general
    directional indication (u, d, s, etc.), an indication of diatonic interval direction,
    quality, and size, or a precise numeric value in half steps.*/
    #[serde(rename = "@intm", skip_serializing_if = "Option::is_none")]
    pub intm: Option<crate::generated::data::DataIntervalMelodic>,
}

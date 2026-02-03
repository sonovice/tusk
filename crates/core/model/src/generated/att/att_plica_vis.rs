//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes that describe the properties of a plica stem in the mensural repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPlicaVis {
    ///Describes the direction of a stem.
    #[serde(rename = "@dir", skip_serializing_if = "Option::is_none")]
    pub dir: Option<crate::generated::data::DataStemdirectionBasic>,
    ///Encodes the stem length.
    #[serde(rename = "@len", skip_serializing_if = "Option::is_none")]
    pub len: Option<crate::generated::data::DataMeasurementunsigned>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe vertical size.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttHeight {
    ///Measurement of the vertical dimension of an entity.
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<crate::generated::data::DataMeasurementunsigned>,
}

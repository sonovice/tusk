//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that capture the dimensions of an entity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDimensions {
    ///Measurement of the vertical dimension of an entity.
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Measurement of the horizontal dimension of an entity.
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<crate::generated::data::DataMeasurementunsigned>,
}

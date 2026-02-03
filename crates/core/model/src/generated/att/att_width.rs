//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe horizontal size.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttWidth {
    ///Measurement of the horizontal dimension of an entity.
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<crate::generated::data::DataMeasurementunsigned>,
}

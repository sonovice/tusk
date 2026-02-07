//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe foliation schemes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttFoliationScheme {
    /**Identifies the foliation scheme in terms of which the location is being specified by
          pointing to some foliation element defining it, or to some other equivalent
          resource.*/
    #[serde(rename = "@scheme", skip_serializing_if = "Option::is_none")]
    pub scheme: Option<crate::generated::data::DataUri>,
}

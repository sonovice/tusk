//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for linking metadata to data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDataPointing {
    ///Used to link metadata elements to one or more data-containing elements.
    #[serde(rename = "@data", default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<crate::generated::data::DataUri>,
}

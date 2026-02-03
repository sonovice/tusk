//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record the shape of a clef.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttClefShape {
    ///Describes a clef’s shape.
    #[serde(rename = "@shape", skip_serializing_if = "Option::is_none")]
    pub shape: Option<crate::generated::data::DataClefshape>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAnnotVis {
    ///Location of the annotation.
    #[serde(rename = "@place", default, skip_serializing_if = "Vec::is_empty")]
    pub place: Vec<crate::generated::data::DataPlacement>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that identify the layer to which a feature applies.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLayerIdent {
    ///Identifies the layer to which a feature applies.
    #[serde(rename = "@layer", default, skip_serializing_if = "Vec::is_empty")]
    pub layer: Vec<u64>,
}

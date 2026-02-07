//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by staffDef and scoreDef to provide default values for attributes in the logical
      domain that are related to key signatures.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttKeySigDefaultLog {
    ///Written key signature.
    #[serde(rename = "@keysig", default, skip_serializing_if = "Vec::is_empty")]
    pub keysig: Vec<crate::generated::data::DataKeyfifths>,
}

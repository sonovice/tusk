//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttRestVisMensural {
    ///States how many spaces are covered by the rest.
    #[serde(rename = "@spaces", skip_serializing_if = "Option::is_none")]
    pub spaces: Option<u64>,
}

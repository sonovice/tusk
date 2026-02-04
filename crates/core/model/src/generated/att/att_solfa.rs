//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that specify pitch using sol-fa.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSolfa {
    /**Contains sol-fa designation,e.g., do, re, mi, etc., in either a fixed or movable Do
          system.*/
    #[serde(rename = "@psolfa", skip_serializing_if = "Option::is_none")]
    pub psolfa: Option<String>,
}

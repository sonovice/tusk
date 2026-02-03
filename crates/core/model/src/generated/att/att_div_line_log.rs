//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttDivLineLogForm {
    ///
    #[serde(rename = "caesura")]
    Caesura,
    ///
    #[serde(rename = "finalis")]
    Finalis,
    ///
    #[serde(rename = "maior")]
    Maior,
    ///
    #[serde(rename = "maxima")]
    Maxima,
    ///
    #[serde(rename = "minima")]
    Minima,
    ///
    #[serde(rename = "virgula")]
    Virgula,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDivLineLog {
    ///Identifies the different kinds of division.
    #[serde(rename = "@form", default, skip_serializing_if = "Vec::is_empty")]
    pub form: Vec<AttDivLineLogForm>,
}

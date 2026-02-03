//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record numbers to be displayed with a feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNumbered {
    ///Records a number or count accompanying a notational feature.
    #[serde(rename = "@num", skip_serializing_if = "Option::is_none")]
    pub num: Option<u64>,
}

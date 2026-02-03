//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSymbolLog {
    /**Holds a reference to the first element in a sequence of events to which the feature
    applies.*/
    #[serde(rename = "@startid", skip_serializing_if = "Option::is_none")]
    pub startid: Option<crate::generated::data::DataUri>,
}

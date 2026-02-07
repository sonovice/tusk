//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttAnchoredTextLogFunc {
    ///The function of the text is unknown.
    #[serde(rename = "unknown")]
    Unknown,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAnchoredTextLog {
    /**Holds a reference to the first element in a sequence of events to which the feature
          applies.*/
    #[serde(rename = "@startid", skip_serializing_if = "Option::is_none")]
    pub startid: Option<crate::generated::data::DataUri>,
    ///Indicates the function of the text.
    #[serde(rename = "@func", skip_serializing_if = "Option::is_none")]
    pub func: Option<AttAnchoredTextLogFunc>,
}

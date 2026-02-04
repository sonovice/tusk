//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe order within a collection of features.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSequence {
    /**Used to assign a sequence number related to the order in which the encoded features
          carrying this attribute are believed to have occurred.*/
    #[serde(rename = "@seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
}

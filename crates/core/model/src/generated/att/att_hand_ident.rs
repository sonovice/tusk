//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes which identify a document hand.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttHandIdent {
    ///Signifies the hand responsible for an action. The value must be the ID of ahandelement declared in the header.
    #[serde(rename = "@hand", skip_serializing_if = "Option::is_none")]
    pub hand: Option<crate::generated::data::DataUri>,
}

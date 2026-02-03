//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes supplying pointers to user-defined symbols.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAltSym {
    /**Provides a way of pointing to a user-defined symbol. It must contain a reference to an
    ID of asymbolDefelement elsewhere in the document.*/
    #[serde(rename = "@altsym", skip_serializing_if = "Option::is_none")]
    pub altsym: Option<crate::generated::data::DataUri>,
}

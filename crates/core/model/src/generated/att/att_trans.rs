//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes for elements encoding authorial or scribal intervention when transcribing
manuscript or similar sources.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTrans {
    /**The @instant attribute is syntactic sugar for classifying a scribal intervention as an
    ad-hoc modification; that is, one which does not interrupt the writing process.*/
    #[serde(rename = "@instant", skip_serializing_if = "Option::is_none")]
    pub instant: Option<crate::generated::data::DataBoolean>,
    ///Points to the genetic state that results from this modification.
    #[serde(rename = "@state", default, skip_serializing_if = "Vec::is_empty")]
    pub state: Vec<crate::generated::data::DataUri>,
    ///Signifies the hand responsible for an action. The value must be the ID of ahandelement declared in the header.
    #[serde(rename = "@hand", skip_serializing_if = "Option::is_none")]
    pub hand: Option<crate::generated::data::DataUri>,
    /**Identifies one or more metadata elements (other than classification terms) within the
    header, which are understood to apply to the element bearing this attribute and its
    content.*/
    #[serde(rename = "@decls", default, skip_serializing_if = "Vec::is_empty")]
    pub decls: Vec<crate::generated::data::DataUri>,
    /**Used to assign a sequence number related to the order in which the encoded features
    carrying this attribute are believed to have occurred.*/
    #[serde(rename = "@seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
}

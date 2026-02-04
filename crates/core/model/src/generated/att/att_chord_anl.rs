//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Analytical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttChordAnl {
    ///Indicates that this event is "under a beam".
    #[serde(rename = "@beam", default, skip_serializing_if = "Vec::is_empty")]
    pub beam: Vec<crate::generated::data::DataBeam>,
    /**Indicates the attachment of a fermata to this element. If visual information about the
          fermata needs to be recorded, then afermataelement should be
          employed instead.*/
    #[serde(rename = "@fermata", skip_serializing_if = "Option::is_none")]
    pub fermata: Option<crate::generated::data::DataStaffrelBasic>,
    ///Indicates the attachment of an l.v. (laissez vibrer) sign to this element.
    #[serde(rename = "@lv", skip_serializing_if = "Option::is_none")]
    pub lv: Option<crate::generated::data::DataBoolean>,
    /**Indicates that this element has an attached ornament. If visual information about the
          ornament is needed, then one of the elements that represents an ornament (mordent, trill,
          or turn) should be employed.*/
    #[serde(rename = "@ornam", default, skip_serializing_if = "Vec::is_empty")]
    pub ornam: Vec<crate::generated::data::DataOrnamCmn>,
    /**Indicates that this element participates in a slur. If visual information about the
          slur needs to be recorded, then aslurelement should be
          employed.*/
    #[serde(rename = "@slur", default, skip_serializing_if = "Vec::is_empty")]
    pub slur: Vec<crate::generated::data::DataSlur>,
    /**Indicates that this element participates in a tie. If visual information about the tie
          needs to be recorded, then atieelement should be employed.*/
    #[serde(rename = "@tie", default, skip_serializing_if = "Vec::is_empty")]
    pub tie: Vec<crate::generated::data::DataTie>,
    /**Indicates that this feature participates in a tuplet. If visual information about the
          tuplet needs to be recorded, then atupletelement should be
          employed.*/
    #[serde(rename = "@tuplet", default, skip_serializing_if = "Vec::is_empty")]
    pub tuplet: Vec<crate::generated::data::DataTuplet>,
}

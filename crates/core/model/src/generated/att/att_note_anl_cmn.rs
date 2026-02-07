//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Analytical domain attributes in the CMN repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteAnlCmn {
    ///Indicates that this event is "under a beam".
    #[serde(rename = "@beam", default, skip_serializing_if = "Vec::is_empty")]
    pub beam: Vec<crate::generated::data::DataBeam>,
    /**Indicates that this element participates in a glissando. If visual information about
          the glissando needs to be recorded, then aglisselement should be
          employed instead.*/
    #[serde(rename = "@gliss", skip_serializing_if = "Option::is_none")]
    pub gliss: Option<crate::generated::data::DataGlissando>,
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
    ///Holds an associated sung text syllable.
    #[serde(rename = "@syl", skip_serializing_if = "Option::is_none")]
    pub syl: Option<String>,
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

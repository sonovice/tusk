//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttHispanTickLogEvaluate {
    /**If an element pointed to is itself a pointer, then the target of that pointer will
    be taken, and so on, until an element is found which is not a pointer.*/
    #[serde(rename = "all")]
    All,
    /**If an element pointed to is itself a pointer, then its target (whether a pointer
    or not) is taken as the target of this pointer.*/
    #[serde(rename = "one")]
    One,
    /**No further evaluation of targets is carried out beyond that needed to find the
    element(s) specified in plist or target attribute.*/
    #[serde(rename = "none")]
    None,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttHispanTickLog {
    /**Indicates the point of occurrence of this feature along a time line. Its value must be
    the ID of awhenelement elsewhere in the document.*/
    #[serde(rename = "@when", skip_serializing_if = "Option::is_none")]
    pub when: Option<crate::generated::data::DataUri>,
    ///Identifies the layer to which a feature applies.
    #[serde(rename = "@layer", default, skip_serializing_if = "Vec::is_empty")]
    pub layer: Vec<u64>,
    /**Indicates the part in which the current feature should appear. Use '%all' when the
    feature should occur in every part.*/
    #[serde(rename = "@part", default, skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<String>,
    /**Signifies the part staff on which a notated feature occurs. Use '%all' when the
    feature should occur on every staff.*/
    #[serde(rename = "@partstaff", default, skip_serializing_if = "Vec::is_empty")]
    pub partstaff: Vec<String>,
    /**When the target attribute is present, plist identifies the active participants; that
    is, those entities pointed "from", in a relationship with the specified target(s). When
    the target attribute is not present, it identifies participants in a mutual
    relationship.*/
    #[serde(rename = "@plist", default, skip_serializing_if = "Vec::is_empty")]
    pub plist: Vec<crate::generated::data::DataUri>,
    /**Signifies the staff on which a notated event occurs or to which a control event
    applies. Mandatory when applicable.*/
    #[serde(rename = "@staff", default, skip_serializing_if = "Vec::is_empty")]
    pub staff: Vec<u64>,
    /**Specifies the intended meaning when a participant in a relationship is itself a
    pointer.*/
    #[serde(rename = "@evaluate", skip_serializing_if = "Option::is_none")]
    pub evaluate: Option<AttHispanTickLogEvaluate>,
}

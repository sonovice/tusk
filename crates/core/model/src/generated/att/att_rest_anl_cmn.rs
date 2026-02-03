//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Analytical domain attributes in the CMN repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttRestAnlCmn {
    ///Indicates that this event is "under a beam".
    #[serde(rename = "@beam", default, skip_serializing_if = "Vec::is_empty")]
    pub beam: Vec<crate::generated::data::DataBeam>,
    /**Indicates the attachment of a fermata to this element. If visual information about the
    fermata needs to be recorded, then afermataelement should be
    employed instead.*/
    #[serde(rename = "@fermata", skip_serializing_if = "Option::is_none")]
    pub fermata: Option<crate::generated::data::DataStaffrelBasic>,
    /**Indicates that this feature participates in a tuplet. If visual information about the
    tuplet needs to be recorded, then atupletelement should be
    employed.*/
    #[serde(rename = "@tuplet", default, skip_serializing_if = "Vec::is_empty")]
    pub tuplet: Vec<crate::generated::data::DataTuplet>,
}

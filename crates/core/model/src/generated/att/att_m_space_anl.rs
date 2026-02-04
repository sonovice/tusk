//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Analytical domain attributes in the CMN repertoire. Use the n attribute to explicitly
encode this measure’s position in a string of measures containing onlymRestelements.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMSpaceAnl {
    /**Indicates the attachment of a fermata to this element. If visual information about the
    fermata needs to be recorded, then afermataelement should be
    employed instead.*/
    #[serde(rename = "@fermata", skip_serializing_if = "Option::is_none")]
    pub fermata: Option<crate::generated::data::DataStaffrelBasic>,
}

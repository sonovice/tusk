//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes indicating the attachment of a fermata to the feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttFermataPresent {
    /**Indicates the attachment of a fermata to this element. If visual information about the
          fermata needs to be recorded, then afermataelement should be
          employed instead.*/
    #[serde(rename = "@fermata", skip_serializing_if = "Option::is_none")]
    pub fermata: Option<crate::generated::data::DataStaffrelBasic>,
}

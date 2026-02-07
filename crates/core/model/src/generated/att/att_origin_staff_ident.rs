//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for identifying the staff associated with a distant feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOriginStaffIdent {
    /**signifies the staff on which referenced notation occurs. Defaults to the same value as
          the local staff. Mandatory when applicable.*/
    #[serde(rename = "@origin.staff", skip_serializing_if = "Option::is_none")]
    pub origin_staff: Option<crate::generated::SpaceSeparated<u64>>,
}

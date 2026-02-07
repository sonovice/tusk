//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for identifying the staff associated with the current feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffIdent {
    /**Signifies the staff on which a notated event occurs or to which a control event
          applies. Mandatory when applicable.*/
    #[serde(rename = "@staff", default, skip_serializing_if = "Vec::is_empty")]
    pub staff: Vec<u64>,
}

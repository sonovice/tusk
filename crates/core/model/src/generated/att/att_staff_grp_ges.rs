//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffGrpGes {
    /**Provides a way of pointing to a MIDI instrument definition. It must contain the ID of
          aninstrDefelement elsewhere in the document.*/
    #[serde(rename = "@instr", skip_serializing_if = "Option::is_none")]
    pub instr: Option<crate::generated::data::DataUri>,
}

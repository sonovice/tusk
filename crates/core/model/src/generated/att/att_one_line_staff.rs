//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record placement of notes on a single-line staff.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOneLineStaff {
    /**Determines the placement of notes on a 1-line staff. A value oftrueplaces all
          notes on the line, while a value offalseplaces stems-up notes above the line and
          stems-down notes below the line.*/
    #[serde(rename = "@ontheline", skip_serializing_if = "Option::is_none")]
    pub ontheline: Option<crate::generated::data::DataBoolean>,
}

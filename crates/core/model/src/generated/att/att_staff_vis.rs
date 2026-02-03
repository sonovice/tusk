//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffVis {
    /**Indicates if a feature should be rendered when the notation is presented graphically
    or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
}

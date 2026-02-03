//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes specifying whether a feature is contemporary or historical.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttContemporary {
    ///
    #[serde(rename = "@contemporary", skip_serializing_if = "Option::is_none")]
    pub contemporary: Option<crate::generated::data::DataBoolean>,
}

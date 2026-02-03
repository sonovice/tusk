//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes for recording the number of slashes that accompany a feature.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSlashCount {
    ///Indicates the number of slashes present.
    #[serde(rename = "@slash", skip_serializing_if = "Option::is_none")]
    pub slash: Option<crate::generated::data::DataSlash>,
}

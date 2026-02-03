//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes for tuning.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTuningLog {
    ///Describes the tuning standard used.
    #[serde(rename = "@tuning.standard", skip_serializing_if = "Option::is_none")]
    pub tuning_standard: Option<crate::generated::data::DataCoursetuning>,
}

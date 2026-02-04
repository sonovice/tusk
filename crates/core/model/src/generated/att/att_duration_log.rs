//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that express duration in musical terms.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDurationLog {
    /**Records the duration of a feature using the relative durational values provided by the
    data.DURATION datatype.*/
    #[serde(rename = "@dur", skip_serializing_if = "Option::is_none")]
    pub dur: Option<crate::generated::data::DataDuration>,
}

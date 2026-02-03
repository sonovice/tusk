//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that permit total duration to be represented by multiple values.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDurationAdditive {
    /**When a duration cannot be represented as a single power-of-two value, multiple
    space-separated values that add up to the total duration may be used.*/
    #[serde(rename = "@dur", default, skip_serializing_if = "Vec::is_empty")]
    pub dur: Vec<crate::generated::data::DataDuration>,
}

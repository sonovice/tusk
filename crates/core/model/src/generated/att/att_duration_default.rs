//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that provide a durational default value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDurationDefault {
    /**Contains a default duration in those situations when the first note, rest, chord, etc.
          in a measure does not have a duration specified.*/
    #[serde(rename = "@dur.default", skip_serializing_if = "Option::is_none")]
    pub dur_default: Option<crate::generated::data::DataDuration>,
    /**Along with numbase.default, describes the default duration as a ratio. num.default is
          the first value in the ratio.*/
    #[serde(rename = "@num.default", skip_serializing_if = "Option::is_none")]
    pub num_default: Option<u64>,
    /**Along with num.default, describes the default duration as a ratio. numbase.default is
          the second value in the ratio.*/
    #[serde(rename = "@numbase.default", skip_serializing_if = "Option::is_none")]
    pub numbase_default: Option<u64>,
}

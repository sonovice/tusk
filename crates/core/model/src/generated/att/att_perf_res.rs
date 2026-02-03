//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that define the characteristics and components of the performance resource.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPerfRes {
    ///Marks a performance resource as ad libitum (optional).
    #[serde(rename = "@adlib", skip_serializing_if = "Option::is_none")]
    pub adlib: Option<crate::generated::data::DataBoolean>,
    ///Indicates the number of performers.
    #[serde(rename = "@count", skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    /**Records the amount of diatonic pitch shift,e.g., C to C♯ = 0, C to D♭ = 1, necessary
    to calculate the sounded pitch from the written one.*/
    #[serde(rename = "@trans.diat", skip_serializing_if = "Option::is_none")]
    pub trans_diat: Option<i64>,
    /**Records the amount of pitch shift in semitones,e.g., C to C♯ = 1, C to D♭ = 1,
    necessary to calculate the sounded pitch from the written one.*/
    #[serde(rename = "@trans.semi", skip_serializing_if = "Option::is_none")]
    pub trans_semi: Option<i64>,
    ///Use this attribute to identify the performance resource as a soloist especially in an accompanied work, such as a concerto or vocal solo.
    #[serde(rename = "@solo", skip_serializing_if = "Option::is_none")]
    pub solo: Option<crate::generated::data::DataBoolean>,
}

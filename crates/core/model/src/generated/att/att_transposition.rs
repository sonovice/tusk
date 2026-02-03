//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe transposition.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTransposition {
    /**Records the amount of diatonic pitch shift,e.g., C to C♯ = 0, C to D♭ = 1, necessary
    to calculate the sounded pitch from the written one.*/
    #[serde(rename = "@trans.diat", skip_serializing_if = "Option::is_none")]
    pub trans_diat: Option<i64>,
    /**Records the amount of pitch shift in semitones,e.g., C to C♯ = 1, C to D♭ = 1,
    necessary to calculate the sounded pitch from the written one.*/
    #[serde(rename = "@trans.semi", skip_serializing_if = "Option::is_none")]
    pub trans_semi: Option<i64>,
}

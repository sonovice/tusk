//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttScoreDefLogCmn {
    /**Provides an example of how automated beaming (including secondary beams) is to be
    performed.*/
    #[serde(rename = "@beam.group", skip_serializing_if = "Option::is_none")]
    pub beam_group: Option<String>,
    /**Indicates whether automatically-drawn beams should include rests shorter than a
    quarter note duration.*/
    #[serde(rename = "@beam.rests", skip_serializing_if = "Option::is_none")]
    pub beam_rests: Option<crate::generated::data::DataBoolean>,
}

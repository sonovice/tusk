//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLayerDefLog {
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
    /**Provides an example of how automated beaming (including secondary beams) is to be
          performed.*/
    #[serde(rename = "@beam.group", skip_serializing_if = "Option::is_none")]
    pub beam_group: Option<String>,
    /**Indicates whether automatically-drawn beams should include rests shorter than a
          quarter note duration.*/
    #[serde(rename = "@beam.rests", skip_serializing_if = "Option::is_none")]
    pub beam_rests: Option<crate::generated::data::DataBoolean>,
    /**Contains a default octave specification for use when the first note, rest, chord, etc.
          in a measure does not have an octave value specified.*/
    #[serde(rename = "@oct.default", skip_serializing_if = "Option::is_none")]
    pub oct_default: Option<crate::generated::data::DataOctave>,
    /**Records the amount of diatonic pitch shift,e.g., C to C♯ = 0, C to D♭ = 1, necessary
          to calculate the sounded pitch from the written one.*/
    #[serde(rename = "@trans.diat", skip_serializing_if = "Option::is_none")]
    pub trans_diat: Option<i64>,
    /**Records the amount of pitch shift in semitones,e.g., C to C♯ = 1, C to D♭ = 1,
          necessary to calculate the sounded pitch from the written one.*/
    #[serde(rename = "@trans.semi", skip_serializing_if = "Option::is_none")]
    pub trans_semi: Option<i64>,
}

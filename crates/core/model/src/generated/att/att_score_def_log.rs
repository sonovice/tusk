//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Logical domain attributes for scoreDef in the CMN repertoire. The values set in these
      attributes act as score-wide defaults for attributes that are not set in descendant
      elements.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttScoreDefLog {
    ///Encodes a value for the clef symbol.
    #[serde(rename = "@clef.shape", skip_serializing_if = "Option::is_none")]
    pub clef_shape: Option<crate::generated::data::DataClefshape>,
    /**Contains a default value for the position of the clef. The value must be in the range
          between 1 and the number of lines on the staff. The numbering of lines starts with the
          lowest line of the staff.*/
    #[serde(rename = "@clef.line", skip_serializing_if = "Option::is_none")]
    pub clef_line: Option<crate::generated::data::DataClefline>,
    ///Records the amount of octave displacement to be applied to the clef.
    #[serde(rename = "@clef.dis", skip_serializing_if = "Option::is_none")]
    pub clef_dis: Option<crate::generated::data::DataOctaveDis>,
    ///Records the direction of octave displacement to be applied to the clef.
    #[serde(rename = "@clef.dis.place", skip_serializing_if = "Option::is_none")]
    pub clef_dis_place: Option<crate::generated::data::DataStaffrelBasic>,
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
    ///Written key signature.
    #[serde(rename = "@keysig", default, skip_serializing_if = "Vec::is_empty")]
    pub keysig: Vec<crate::generated::data::DataKeyfifths>,
    /**Captures the number of beats in a measure, that is, the top number of the meter
          signature. It must contain a decimal number or an expression that evaluates to a
          decimal number, such as 2+3 or 3*2.*/
    #[serde(rename = "@meter.count", skip_serializing_if = "Option::is_none")]
    pub meter_count: Option<String>,
    /**Contains the number indicating the beat unit, that is, the bottom number of the meter
          signature.*/
    #[serde(rename = "@meter.unit", skip_serializing_if = "Option::is_none")]
    pub meter_unit: Option<f64>,
    /**Indicates the use of a meter symbol instead of a numeric meter signature, that is, 'C'
          for common time or 'C' with a slash for cut time.*/
    #[serde(rename = "@meter.sym", skip_serializing_if = "Option::is_none")]
    pub meter_sym: Option<crate::generated::data::DataMetersign>,
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
    /**Provides an example of how automated beaming (including secondary beams) is to be
          performed.*/
    #[serde(rename = "@beam.group", skip_serializing_if = "Option::is_none")]
    pub beam_group: Option<String>,
    /**Indicates whether automatically-drawn beams should include rests shorter than a
          quarter note duration.*/
    #[serde(rename = "@beam.rests", skip_serializing_if = "Option::is_none")]
    pub beam_rests: Option<crate::generated::data::DataBoolean>,
    ///Describes the maxima-long relationship.
    #[serde(rename = "@modusmaior", skip_serializing_if = "Option::is_none")]
    pub modusmaior: Option<crate::generated::data::DataModusmaior>,
    ///Describes the long-breve relationship.
    #[serde(rename = "@modusminor", skip_serializing_if = "Option::is_none")]
    pub modusminor: Option<crate::generated::data::DataModusminor>,
    ///Describes the semibreve-minim relationship.
    #[serde(rename = "@prolatio", skip_serializing_if = "Option::is_none")]
    pub prolatio: Option<crate::generated::data::DataProlatio>,
    ///Describes the breve-semibreve relationship.
    #[serde(rename = "@tempus", skip_serializing_if = "Option::is_none")]
    pub tempus: Option<crate::generated::data::DataTempus>,
    ///Describes the divisions of the breve in use in 14th-century Italy.
    #[serde(rename = "@divisio", skip_serializing_if = "Option::is_none")]
    pub divisio: Option<crate::generated::data::DataDivisio>,
    ///Together, proport.num and proport.numbase specify a proportional change as a ratio,e.g., 1:3. Proport.num is for the first value in the ratio.
    #[serde(rename = "@proport.num", skip_serializing_if = "Option::is_none")]
    pub proport_num: Option<u64>,
    ///Together, proport.num and proport.numbase specify a proportional change as a ratio,e.g., 1:3. Proport.numbase is for the second value in the ratio.
    #[serde(rename = "@proport.numbase", skip_serializing_if = "Option::is_none")]
    pub proport_numbase: Option<u64>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record a default value for octave.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOctaveDefault {
    /**Contains a default octave specification for use when the first note, rest, chord, etc.
          in a measure does not have an octave value specified.*/
    #[serde(rename = "@oct.default", skip_serializing_if = "Option::is_none")]
    pub oct_default: Option<crate::generated::data::DataOctave>,
}

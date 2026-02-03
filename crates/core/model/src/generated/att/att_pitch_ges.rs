//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural attributes about pitch.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPitchGes {
    ///Records performed octave information that differs from the written value.
    #[serde(rename = "@oct.ges", skip_serializing_if = "Option::is_none")]
    pub oct_ges: Option<crate::generated::data::DataOctave>,
    ///Contains a performed pitch name that differs from the written value.
    #[serde(rename = "@pname.ges", skip_serializing_if = "Option::is_none")]
    pub pname_ges: Option<crate::generated::data::DataPitchnameGestural>,
    ///Holds a pitch-to-number mapping, a base-40 or MIDI note number, for example.
    #[serde(rename = "@pnum", skip_serializing_if = "Option::is_none")]
    pub pnum: Option<crate::generated::data::DataPitchnumber>,
}

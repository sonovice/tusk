//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttHarmAnlForm {
    /**The notation contains all the notes necessary for the harmonic label,e.g., the
    notes "D F♯ A" for the harmonic label "D".*/
    #[serde(rename = "explicit")]
    Explicit,
    /**The harmonic label relies on notes implied, but not actually present, in the
    notation,e.g., the notes "D F♯ C" for the harmonic label "D7". The note "A" is
    missing from the notation, but can be implied.*/
    #[serde(rename = "implied")]
    Implied,
}
///Analytical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttHarmAnl {
    ///Encodes the harmonic interval between pitches occurring at the same time.
    #[serde(rename = "@inth", default, skip_serializing_if = "Vec::is_empty")]
    pub inth: Vec<crate::generated::data::DataIntervalHarmonic>,
    ///Indicates to what degree the harmonic label is supported by the notation.
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<AttHarmAnlForm>,
}

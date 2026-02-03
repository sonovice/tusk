//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttSylLogCon {
    ///Space (word separator).
    #[serde(rename = "s")]
    S,
    ///Dash (syllable separator).
    #[serde(rename = "d")]
    D,
    ///Underscore (syllable extension).
    #[serde(rename = "u")]
    U,
    ///Tilde (syllable elision).
    #[serde(rename = "t")]
    T,
    ///Circumflex [angled line above] (syllable elision).
    #[serde(rename = "c")]
    C,
    ///Caron [angled line below] (syllable elision).
    #[serde(rename = "v")]
    V,
    ///Inverted breve [curved line above] (syllable elision).
    #[serde(rename = "i")]
    I,
    ///Breve [curved line below] (syllable elision).
    #[serde(rename = "b")]
    B,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttSylLogWordpos {
    ///(initial) first syllable.
    #[serde(rename = "i")]
    I,
    ///(medial) neither first nor last syllable.
    #[serde(rename = "m")]
    M,
    ///(single) single syllable.
    #[serde(rename = "s")]
    S,
    ///(terminal) last syllable.
    #[serde(rename = "t")]
    T,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSylLog {
    /**Describes the symbols typically used to indicate breaks between syllables and their
    functions.*/
    #[serde(rename = "@con", skip_serializing_if = "Option::is_none")]
    pub con: Option<AttSylLogCon>,
    ///Records the position of a syllable within a word.
    #[serde(rename = "@wordpos", skip_serializing_if = "Option::is_none")]
    pub wordpos: Option<AttSylLogWordpos>,
}

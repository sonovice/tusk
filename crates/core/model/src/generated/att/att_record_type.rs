//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttRecordTypeRecordtype {
    ///Language material.
    #[serde(rename = "a")]
    A,
    ///Notated music.
    #[serde(rename = "c")]
    C,
    ///Manuscript notated music.
    #[serde(rename = "d")]
    D,
    ///Non-manuscript cartographic material.
    #[serde(rename = "e")]
    E,
    ///Manuscript cartographic material.
    #[serde(rename = "f")]
    F,
    ///Projected medium.
    #[serde(rename = "g")]
    G,
    ///Nonmusical sound recording.
    #[serde(rename = "i")]
    I,
    ///Musical sound recording.
    #[serde(rename = "j")]
    J,
    ///Two-dimensional nonprojectable graphic.
    #[serde(rename = "k")]
    K,
    ///Computer file.
    #[serde(rename = "m")]
    M,
    ///Kit.
    #[serde(rename = "o")]
    O,
    ///Mixed materials.
    #[serde(rename = "p")]
    P,
    ///Three-dimensional artifact or naturally occurring object.
    #[serde(rename = "r")]
    R,
    ///Manuscript language material.
    #[serde(rename = "t")]
    T,
}
/**Attributes that define the characteristics and components of the bibliographic
description.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttRecordType {
    ///
    #[serde(rename = "@recordtype", skip_serializing_if = "Option::is_none")]
    pub recordtype: Option<AttRecordTypeRecordtype>,
}

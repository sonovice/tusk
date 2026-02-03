//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Analytical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAmbitusAnl {
    ///Encodes the harmonic interval between pitches occurring at the same time.
    #[serde(rename = "@inth", default, skip_serializing_if = "Vec::is_empty")]
    pub inth: Vec<crate::generated::data::DataIntervalHarmonic>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttArticGes {
    ///Records performed articulation that differs from the written value.
    #[serde(rename = "@artic.ges", default, skip_serializing_if = "Vec::is_empty")]
    pub artic_ges: Vec<crate::generated::data::DataArticulation>,
}

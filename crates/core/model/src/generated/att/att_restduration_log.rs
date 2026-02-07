//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that express duration of rests in musical terms.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttRestdurationLog {
    /**Records the duration of a rest using the relative durational values provided by the
          data.DURATIONRESTS datatype.*/
    #[serde(rename = "@dur", skip_serializing_if = "Option::is_none")]
    pub dur: Option<crate::generated::data::DataDurationrests>,
}

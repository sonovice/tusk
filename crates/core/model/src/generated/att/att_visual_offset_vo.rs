//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Vertical offset attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVisualOffsetVo {
    /**Records the vertical adjustment of a feature’s programmatically-determined location in
          terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
          staff lines.*/
    #[serde(rename = "@vo", skip_serializing_if = "Option::is_none")]
    pub vo: Option<crate::generated::data::DataMeasurementsigned>,
}

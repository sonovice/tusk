//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Horizontal offset attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVisualOffsetHo {
    /**Records a horizontal adjustment to a feature’s programmatically-determined location in
          terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
          staff lines.*/
    #[serde(rename = "@ho", skip_serializing_if = "Option::is_none")]
    pub ho: Option<crate::generated::data::DataMeasurementsigned>,
}

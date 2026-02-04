//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Horizontal offset requiring a pair of attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVisualOffset2Ho {
    /**Records the horizontal adjustment of a feature’s programmatically-determined start
          point.*/
    #[serde(rename = "@startho", skip_serializing_if = "Option::is_none")]
    pub startho: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records the horizontal adjustment of a feature’s programmatically-determined end
          point.*/
    #[serde(rename = "@endho", skip_serializing_if = "Option::is_none")]
    pub endho: Option<crate::generated::data::DataMeasurementsigned>,
}

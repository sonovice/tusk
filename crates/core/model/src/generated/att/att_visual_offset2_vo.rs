//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Vertical offset attributes requiring a pair of attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVisualOffset2Vo {
    /**Records a vertical adjustment of a feature’s programmatically-determined start
          point.*/
    #[serde(rename = "@startvo", skip_serializing_if = "Option::is_none")]
    pub startvo: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records a vertical adjustment of a feature’s programmatically-determined end
          point.*/
    #[serde(rename = "@endvo", skip_serializing_if = "Option::is_none")]
    pub endvo: Option<crate::generated::data::DataMeasurementsigned>,
}

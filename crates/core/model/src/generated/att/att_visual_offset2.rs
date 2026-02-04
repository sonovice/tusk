//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Visual offset attributes. Some items may have their location recorded in terms of pairs of
      offsets from their programmatically-determined location. The startho and endho attributes
      record the horizontal offsets of the start and end points of the item, respectively.
      Similarly, the startvo and endvo attributes record the vertical offsets of the start and end
      points of the item. The startto and endto attributes hold timestamp offsets, the most common
      use of which is as alternatives to the ho attributes.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVisualOffset2 {
    /**Records the horizontal adjustment of a feature’s programmatically-determined start
          point.*/
    #[serde(rename = "@startho", skip_serializing_if = "Option::is_none")]
    pub startho: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records the horizontal adjustment of a feature’s programmatically-determined end
          point.*/
    #[serde(rename = "@endho", skip_serializing_if = "Option::is_none")]
    pub endho: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records a timestamp adjustment of a feature’s programmatically-determined start
          point.*/
    #[serde(rename = "@startto", skip_serializing_if = "Option::is_none")]
    pub startto: Option<crate::generated::data::DataTstampoffset>,
    /**Records a timestamp adjustment of a feature’s programmatically-determined end
          point.*/
    #[serde(rename = "@endto", skip_serializing_if = "Option::is_none")]
    pub endto: Option<crate::generated::data::DataTstampoffset>,
    /**Records a vertical adjustment of a feature’s programmatically-determined start
          point.*/
    #[serde(rename = "@startvo", skip_serializing_if = "Option::is_none")]
    pub startvo: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records a vertical adjustment of a feature’s programmatically-determined end
          point.*/
    #[serde(rename = "@endvo", skip_serializing_if = "Option::is_none")]
    pub endvo: Option<crate::generated::data::DataMeasurementsigned>,
}

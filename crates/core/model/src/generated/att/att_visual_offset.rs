//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Visual offset attributes. Some items may have their location recorded in terms of offsets
      from their programmatically-determined location. The ho attribute records the horizontal
      offset while vo records the vertical. The to attribute holds a timestamp offset, the most
      common use of which is as an alternative to the ho attribute.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVisualOffset {
    /**Records a horizontal adjustment to a feature’s programmatically-determined location in
          terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
          staff lines.*/
    #[serde(rename = "@ho", skip_serializing_if = "Option::is_none")]
    pub ho: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records a timestamp adjustment of a feature’s programmatically-determined location in
          terms of musical time; that is, beats.*/
    #[serde(rename = "@to", skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::generated::data::DataTstampoffset>,
    /**Records the vertical adjustment of a feature’s programmatically-determined location in
          terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
          staff lines.*/
    #[serde(rename = "@vo", skip_serializing_if = "Option::is_none")]
    pub vo: Option<crate::generated::data::DataMeasurementsigned>,
}

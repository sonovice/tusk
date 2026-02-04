//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Horizontal offset attributes requiring a pair of attributes specified in terms of
      time.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVisualOffset2To {
    /**Records a timestamp adjustment of a feature’s programmatically-determined start
          point.*/
    #[serde(rename = "@startto", skip_serializing_if = "Option::is_none")]
    pub startto: Option<crate::generated::data::DataTstampoffset>,
    /**Records a timestamp adjustment of a feature’s programmatically-determined end
          point.*/
    #[serde(rename = "@endto", skip_serializing_if = "Option::is_none")]
    pub endto: Option<crate::generated::data::DataTstampoffset>,
}

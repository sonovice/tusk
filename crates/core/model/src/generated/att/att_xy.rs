//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Output coordinate attributes. Some elements may have their exact rendered *output*
      coordinates recorded. x and y attributes indicate where to place the rendered output.
      Recording the coordinates of a feature in a facsimile requires the use of the facs
      attribute.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttXy {
    /**Encodes an x coordinate for a feature in an output coordinate system. When it is
          necessary to record the placement of a feature in a facsimile image, use the facs
          attribute.*/
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /**Encodes a y coordinate for a feature in an output coordinate system. When it is
          necessary to record the placement of a feature in a facsimile image, use the facs
          attribute.*/
    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

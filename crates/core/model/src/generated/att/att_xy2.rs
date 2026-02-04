//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Output coordinate attributes. Some elements may need 2 coordinate pairs to record their
rendered *output* coordinates. The attributes indicate where to place the rendered output.
Recording the coordinates of a feature in a facsimile requires the use of the facs
attribute.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttXy2 {
    ///Encodes the optional 2nd x coordinate.
    #[serde(rename = "@x2", skip_serializing_if = "Option::is_none")]
    pub x2: Option<f64>,
    ///Encodes the optional 2nd y coordinate.
    #[serde(rename = "@y2", skip_serializing_if = "Option::is_none")]
    pub y2: Option<f64>,
}

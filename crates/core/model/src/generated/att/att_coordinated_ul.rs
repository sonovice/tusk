//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**This attribute class records the upper left position of a feature within a two-dimensional coordinate
      system.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCoordinatedUl {
    ///Indicates the upper-left corner x coordinate.
    #[serde(rename = "@ulx", skip_serializing_if = "Option::is_none")]
    pub ulx: Option<u64>,
    ///Indicates the upper-left corner y coordinate.
    #[serde(rename = "@uly", skip_serializing_if = "Option::is_none")]
    pub uly: Option<u64>,
}

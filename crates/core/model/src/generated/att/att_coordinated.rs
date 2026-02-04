//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**This attribute class records the position of a feature within a two-dimensional coordinate
system.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCoordinated {
    ///Indicates the upper-left corner x coordinate.
    #[serde(rename = "@ulx", skip_serializing_if = "Option::is_none")]
    pub ulx: Option<u64>,
    ///Indicates the upper-left corner y coordinate.
    #[serde(rename = "@uly", skip_serializing_if = "Option::is_none")]
    pub uly: Option<u64>,
    ///Indicates the lower-right corner x coordinate.
    #[serde(rename = "@lrx", skip_serializing_if = "Option::is_none")]
    pub lrx: Option<u64>,
    ///Indicates the lower-right corner y coordinate.
    #[serde(rename = "@lry", skip_serializing_if = "Option::is_none")]
    pub lry: Option<u64>,
    /**Indicates the amount by which the contents of this element have been rotated clockwise or, if applicable, how the orientation of
    the element self should be interpreted, with respect to the normal orientation of the parent surface.
    The orientation is expressed in arc degrees.*/
    #[serde(rename = "@rotate", skip_serializing_if = "Option::is_none")]
    pub rotate: Option<crate::generated::data::DataDegrees>,
}

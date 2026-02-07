//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record the basic visual rendition of lines.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLineRendBase {
    ///Describes the style of a line.
    #[serde(rename = "@lform", skip_serializing_if = "Option::is_none")]
    pub lform: Option<crate::generated::data::DataLineform>,
    ///Width of a line.
    #[serde(rename = "@lwidth", skip_serializing_if = "Option::is_none")]
    pub lwidth: Option<crate::generated::data::DataLinewidth>,
    /**Describes the number of segments into which a dashed or dotted line may be divided, or
          the number of "peaks" of a wavy line; a pair of space-separated values (minimum and
          maximum, respectively) provides a range between which a rendering system-supplied value
          may fall, while a single value indicates a fixed amount of space; that is, the minimum and
          maximum values are equal.*/
    #[serde(rename = "@lsegs", skip_serializing_if = "Option::is_none")]
    pub lsegs: Option<u64>,
}

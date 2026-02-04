//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttEndingVis {
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
    ///Symbol rendered at end of line.
    #[serde(rename = "@lendsym", skip_serializing_if = "Option::is_none")]
    pub lendsym: Option<crate::generated::data::DataLinestartendsymbol>,
    ///Holds the relative size of the line-end symbol.
    #[serde(rename = "@lendsym.size", skip_serializing_if = "Option::is_none")]
    pub lendsym_size: Option<crate::generated::data::DataFontsizescale>,
    ///Symbol rendered at start of line.
    #[serde(rename = "@lstartsym", skip_serializing_if = "Option::is_none")]
    pub lstartsym: Option<crate::generated::data::DataLinestartendsymbol>,
    ///Holds the relative size of the line-start symbol.
    #[serde(rename = "@lstartsym.size", skip_serializing_if = "Option::is_none")]
    pub lstartsym_size: Option<crate::generated::data::DataFontsizescale>,
}

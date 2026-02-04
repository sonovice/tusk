//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttGrpSymLogSymbol {
    ///Curved symbol,i.e., {.
    #[serde(rename = "brace")]
    Brace,
    ///Square symbol,i.e., [, but with curved/angled top and bottom segments.
    #[serde(rename = "bracket")]
    Bracket,
    ///Square symbol,i.e., [, with horizontal top and bottom segments.
    #[serde(rename = "bracketsq")]
    Bracketsq,
    /**Line symbol,i.e., |, (wide) line without top and bottom curved/horizontal
              segments.*/
    #[serde(rename = "line")]
    Line,
    ///Grouping symbol missing.
    #[serde(rename = "none")]
    None,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttGrpSymLog {
    ///Specifies the symbol used to group a set of staves.
    #[serde(rename = "@symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<AttGrpSymLogSymbol>,
    /**Holds a reference to the first element in a sequence of events to which the feature
          applies.*/
    #[serde(rename = "@startid", skip_serializing_if = "Option::is_none")]
    pub startid: Option<crate::generated::data::DataUri>,
    /**Indicates the final element in a sequence of events to which the feature
          applies.*/
    #[serde(rename = "@endid", skip_serializing_if = "Option::is_none")]
    pub endid: Option<crate::generated::data::DataUri>,
    ///Indicates the nesting level of staff grouping symbols.
    #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
    pub level: Option<u64>,
}

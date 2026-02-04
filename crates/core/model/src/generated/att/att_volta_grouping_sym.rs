//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttVoltaGroupingSymVoltasym {
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
///Attributes that describe the symbol used to group volta elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVoltaGroupingSym {
    ///Specifies the symbol used to group lyrics.
    #[serde(rename = "@voltasym", skip_serializing_if = "Option::is_none")]
    pub voltasym: Option<AttVoltaGroupingSymVoltasym>,
}

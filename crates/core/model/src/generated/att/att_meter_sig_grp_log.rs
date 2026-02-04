//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMeterSigGrpLogFunc {
    ///Meter signatures apply to alternating measures.
    #[serde(rename = "alternating")]
    Alternating,
    ///Meter signatures are interchangeable,e.g., 3/4 and 6/8.
    #[serde(rename = "interchanging")]
    Interchanging,
    /**Meter signatures with different unit values are used to express a complex metrical
    pattern that is not expressible using traditional means, such as 2/4+1/8.*/
    #[serde(rename = "mixed")]
    Mixed,
    ///Meter signatures in a relationship not covered by the values alternating, interchanging or mixed.
    #[serde(rename = "other")]
    Other,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeterSigGrpLog {
    ///Function of the meter signature group.
    #[serde(rename = "@func", skip_serializing_if = "Option::is_none")]
    pub func: Option<AttMeterSigGrpLogFunc>,
}

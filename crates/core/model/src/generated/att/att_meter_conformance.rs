//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMeterConformanceMetcon {
    ///Complete;i.e., conformant with the prevailing meter.
    #[serde(rename = "c")]
    C,
    ///Incomplete;i.e., not enough beats.
    #[serde(rename = "i")]
    I,
    ///Overfull;i.e., too many beats.
    #[serde(rename = "o")]
    O,
}
/**Attributes that provide information about a structure’s conformance to the prevailing
      meter.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeterConformance {
    /**Indicates the relationship between the content of a staff or layer and the prevailing
          meter.*/
    #[serde(rename = "@metcon", skip_serializing_if = "Option::is_none")]
    pub metcon: Option<AttMeterConformanceMetcon>,
}

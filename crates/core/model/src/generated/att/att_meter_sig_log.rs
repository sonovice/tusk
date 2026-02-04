//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeterSigLog {
    /**Captures the number of beats in a measure, that is, the top number of the meter
          signature. It must contain a decimal number or an expression that evaluates to a
          decimal number, such as 2+3 or 3*2.*/
    #[serde(rename = "@count", skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /**Indicates the use of a meter symbol instead of a numeric meter signature, that is, 'C'
          for common time or 'C' with a slash for cut time.*/
    #[serde(rename = "@sym", skip_serializing_if = "Option::is_none")]
    pub sym: Option<crate::generated::data::DataMetersign>,
    /**Contains the number indicating the beat unit, that is, the bottom number of the meter
          signature.*/
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<f64>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by staffDef and scoreDef to provide default values for attributes in the logical
domain related to meter signature.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeterSigDefaultLog {
    /**Captures the number of beats in a measure, that is, the top number of the meter
    signature. It must contain a decimal number or an expression that evaluates to a
    decimal number, such as 2+3 or 3*2.*/
    #[serde(rename = "@meter.count", skip_serializing_if = "Option::is_none")]
    pub meter_count: Option<String>,
    /**Contains the number indicating the beat unit, that is, the bottom number of the meter
    signature.*/
    #[serde(rename = "@meter.unit", skip_serializing_if = "Option::is_none")]
    pub meter_unit: Option<f64>,
    /**Indicates the use of a meter symbol instead of a numeric meter signature, that is, 'C'
    for common time or 'C' with a slash for cut time.*/
    #[serde(rename = "@meter.sym", skip_serializing_if = "Option::is_none")]
    pub meter_sym: Option<crate::generated::data::DataMetersign>,
}

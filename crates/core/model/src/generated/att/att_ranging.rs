//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Groups attributes that describe a numerical range.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttRanging {
    ///Gives a minimum estimated value for an approximate measurement.
    #[serde(rename = "@atleast", skip_serializing_if = "Option::is_none")]
    pub atleast: Option<f64>,
    ///Gives a maximum estimated value for an approximate measurement.
    #[serde(rename = "@atmost", skip_serializing_if = "Option::is_none")]
    pub atmost: Option<f64>,
    /**Where the measurement summarizes more than one observation or a range of values,
    supplies the minimum value observed.*/
    #[serde(rename = "@min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /**Where the measurement summarizes more than one observation or a range of values,
    supplies the maximum value observed.*/
    #[serde(rename = "@max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /**Specifies the degree of statistical confidence (between zero and one) that a value
    falls within the range specified by min and max, or the proportion of observed values that
    fall within that range.*/
    #[serde(rename = "@confidence", skip_serializing_if = "Option::is_none")]
    pub confidence: Option<crate::generated::data::DataConfidence>,
}

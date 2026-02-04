//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that identify a musical range in terms of musical time.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOriginTimestampLog {
    /**encodes the starting point of musical material in terms of musical time,i.e., a
          (potentially negative) count of measures plus a beat location.*/
    #[serde(rename = "@origin.tstamp", skip_serializing_if = "Option::is_none")]
    pub origin_tstamp: Option<crate::generated::data::DataMeasurebeatoffset>,
    /**encodes the ending point of musical material in terms of musical time,i.e., a count
          of measures plus a beat location. The values are relative to the measure identified byorigin.tstamp.*/
    #[serde(rename = "@origin.tstamp2", skip_serializing_if = "Option::is_none")]
    pub origin_tstamp2: Option<crate::generated::data::DataMeasurebeat>,
}

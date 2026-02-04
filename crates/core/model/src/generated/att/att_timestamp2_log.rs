//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that record a time stamp for the end of an event in terms of musical
time.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTimestamp2Log {
    /**Encodes the ending point of an event,i.e., a count of measures plus a beat location
    in the ending measure.*/
    #[serde(rename = "@tstamp2", skip_serializing_if = "Option::is_none")]
    pub tstamp2: Option<crate::generated::data::DataMeasurebeat>,
}

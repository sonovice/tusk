//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that record a performed (as opposed to notated) time stamp for the end of an
      event.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTimestamp2Ges {
    /**Encodes the ending point of an event,i.e., a count of measures plus a beat location
          in the ending measure.*/
    #[serde(rename = "@tstamp2.ges", skip_serializing_if = "Option::is_none")]
    pub tstamp2_ges: Option<crate::generated::data::DataMeasurebeat>,
    ///Records the ending point of an event in terms of ISO time.
    #[serde(rename = "@tstamp2.real", skip_serializing_if = "Option::is_none")]
    pub tstamp2_real: Option<crate::generated::data::DataIsotime>,
}

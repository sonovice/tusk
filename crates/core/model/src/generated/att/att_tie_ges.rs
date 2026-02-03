//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTieGes {
    /**Encodes the ending point of an event,i.e., a count of measures plus a beat location
    in the ending measure.*/
    #[serde(rename = "@tstamp2.ges", skip_serializing_if = "Option::is_none")]
    pub tstamp2_ges: Option<crate::generated::data::DataMeasurebeat>,
    ///Records the ending point of an event in terms of ISO time.
    #[serde(rename = "@tstamp2.real", skip_serializing_if = "Option::is_none")]
    pub tstamp2_real: Option<crate::generated::data::DataIsotime>,
}

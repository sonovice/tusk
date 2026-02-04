//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Gestural domain attributes. The tstamp.ges and tstamp.real attributes encode the onset
      time of the measure. In reality, this is usually the same as the onset time of the first event
      in the measure.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeasureGes {
    /**Encodes the onset time in terms of musical time,i.e., beats[.fractional beat part],
          as expressed in the written time signature.*/
    #[serde(rename = "@tstamp.ges", skip_serializing_if = "Option::is_none")]
    pub tstamp_ges: Option<crate::generated::data::DataBeat>,
    ///Records the onset time in terms of ISO time.
    #[serde(rename = "@tstamp.real", skip_serializing_if = "Option::is_none")]
    pub tstamp_real: Option<crate::generated::data::DataIsotime>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record a performed (as opposed to notated) time stamp.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTimestampGes {
    /**Encodes the onset time in terms of musical time,i.e., beats[.fractional beat part],
          as expressed in the written time signature.*/
    #[serde(rename = "@tstamp.ges", skip_serializing_if = "Option::is_none")]
    pub tstamp_ges: Option<crate::generated::data::DataBeat>,
    ///Records the onset time in terms of ISO time.
    #[serde(rename = "@tstamp.real", skip_serializing_if = "Option::is_none")]
    pub tstamp_real: Option<crate::generated::data::DataIsotime>,
}

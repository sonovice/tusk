//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that record a time stamp in terms of musical time,i.e., beats[.fractional beat
part].*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTimestampLog {
    /**Encodes the onset time in terms of musical time,i.e., beats[.fractional beat part],
    as expressed in the written time signature.*/
    #[serde(rename = "@tstamp", skip_serializing_if = "Option::is_none")]
    pub tstamp: Option<crate::generated::data::DataBeat>,
}

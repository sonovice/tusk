//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record time-base information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTimeBase {
    /**Indicates the number of pulses (sometimes referred to as ticks or divisions) per
    quarter note. Unlike MIDI, MEI permits different values for a score and individual
    staves.*/
    #[serde(rename = "@ppq", skip_serializing_if = "Option::is_none")]
    pub ppq: Option<u64>,
}

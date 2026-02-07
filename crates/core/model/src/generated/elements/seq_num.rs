//!Element: `<seqNum>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///sequence number - MIDI sequence number.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "seqNum")]
pub struct SeqNum {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    ///Number in the range 0-65535.
    #[serde(rename = "@num", skip_serializing_if = "Option::is_none")]
    pub num: Option<u64>,
}
impl Validate for SeqNum {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

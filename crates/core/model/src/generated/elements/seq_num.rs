//!Element: `<seqNum>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///sequence number - Number in the range 0-65535.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "seqNum")]
pub struct SeqNum {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
}
impl Validate for SeqNum {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

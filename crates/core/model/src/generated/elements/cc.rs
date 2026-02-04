//!Element: `<cc>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///control change - MIDI parameter/control change.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "cc")]
pub struct Cc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    #[serde(flatten)]
    pub midi_number: crate::generated::att::AttMidiNumber,
    #[serde(flatten)]
    pub midi_value: crate::generated::att::AttMidiValue,
}
impl Validate for Cc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

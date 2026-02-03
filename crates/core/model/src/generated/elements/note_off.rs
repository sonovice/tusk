//!Element: `<noteOff>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///MIDI note-off event.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "noteOff")]
pub struct NoteOff {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    #[serde(flatten)]
    pub midi_number: crate::generated::att::AttMidiNumber,
}
impl Validate for NoteOff {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

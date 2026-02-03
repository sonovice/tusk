//!Element: `<noteOn>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///MIDI note-on event.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "noteOn")]
pub struct NoteOn {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    #[serde(flatten)]
    pub midi_number: crate::generated::att::AttMidiNumber,
}
impl Validate for NoteOn {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

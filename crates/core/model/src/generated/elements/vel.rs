//!Element: `<vel>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///velocity - MIDI Note-on/off velocity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "vel")]
pub struct Vel {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    #[serde(flatten)]
    pub midi_number: crate::generated::att::AttMidiNumber,
    ///Indicates whether this is note-on or note-off velocity data.
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
}
impl Validate for Vel {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

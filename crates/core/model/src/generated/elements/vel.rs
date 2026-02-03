//!Element: `<vel>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///velocity - Note-off velocity.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "vel")]
pub struct Vel {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    #[serde(flatten)]
    pub midi_number: crate::generated::att::AttMidiNumber,
}
impl Validate for Vel {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

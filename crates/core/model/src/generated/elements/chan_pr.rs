//!Element: `<chanPr>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///channel pressure - MIDI channel pressure/after touch.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "chanPr")]
pub struct ChanPr {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    #[serde(flatten)]
    pub midi_number: crate::generated::att::AttMidiNumber,
}
impl Validate for ChanPr {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

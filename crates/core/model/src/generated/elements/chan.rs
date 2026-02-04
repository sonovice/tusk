//!Element: `<chan>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///channel - MIDI number in the range set by data.MIDICHANNEL.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "chan")]
pub struct Chan {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
}
impl Validate for Chan {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

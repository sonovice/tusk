//!Element: `<chan>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///channel - MIDI channel assignment.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "chan")]
pub struct Chan {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub midi_event: crate::generated::att::AttMidiEvent,
    ///MIDI number in the range set by data.MIDICHANNEL.
    #[serde(rename = "@num", skip_serializing_if = "Option::is_none")]
    pub num: Option<crate::generated::data::DataMidichannel>,
}
impl Validate for Chan {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

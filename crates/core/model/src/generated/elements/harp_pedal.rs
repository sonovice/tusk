//!Element: `<harpPedal>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///harp pedal - Harp pedal diagram.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "harpPedal")]
pub struct HarpPedal {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub harp_pedal_log: crate::generated::att::AttHarpPedalLog,
    #[serde(flatten)]
    pub harp_pedal_vis: crate::generated::att::AttHarpPedalVis,
    #[serde(flatten)]
    pub harp_pedal_ges: crate::generated::att::AttHarpPedalGes,
    #[serde(flatten)]
    pub harp_pedal_anl: crate::generated::att::AttHarpPedalAnl,
}
impl crate::generated::model::ModelControlEventLikeCmn for HarpPedal {}
impl Validate for HarpPedal {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

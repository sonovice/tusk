//!Element: `<hispanTick>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Hispanic tick.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hispanTick")]
pub struct HispanTick {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub hispan_tick_anl: crate::generated::att::AttHispanTickAnl,
    #[serde(flatten)]
    pub hispan_tick_ges: crate::generated::att::AttHispanTickGes,
    #[serde(flatten)]
    pub hispan_tick_log: crate::generated::att::AttHispanTickLog,
    #[serde(flatten)]
    pub hispan_tick_vis: crate::generated::att::AttHispanTickVis,
}
impl crate::generated::model::ModelNeumeModifierLike for HispanTick {}
impl Validate for HispanTick {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

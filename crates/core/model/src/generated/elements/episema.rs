//!Element: `<episema>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Episema.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "episema")]
pub struct Episema {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub episema_anl: crate::generated::att::AttEpisemaAnl,
    #[serde(flatten)]
    pub episema_ges: crate::generated::att::AttEpisemaGes,
    #[serde(flatten)]
    pub episema_log: crate::generated::att::AttEpisemaLog,
    #[serde(flatten)]
    pub episema_vis: crate::generated::att::AttEpisemaVis,
}
impl crate::generated::model::ModelNeumeModifierLike for Episema {}
impl Validate for Episema {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

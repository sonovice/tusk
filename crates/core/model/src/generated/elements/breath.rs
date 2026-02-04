//!Element: `<breath>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
/**breath mark - An indication of a point at which the performer on an instrument requiring
      breath (including the voice) may breathe.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "breath")]
pub struct Breath {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub breath_log: crate::generated::att::AttBreathLog,
    #[serde(flatten)]
    pub breath_vis: crate::generated::att::AttBreathVis,
    #[serde(flatten)]
    pub breath_ges: crate::generated::att::AttBreathGes,
    #[serde(flatten)]
    pub breath_anl: crate::generated::att::AttBreathAnl,
}
impl crate::generated::model::ModelControlEventLikeCmn for Breath {}
impl Validate for Breath {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

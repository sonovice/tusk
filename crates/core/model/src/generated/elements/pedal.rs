//!Element: `<pedal>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Piano pedal mark.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "pedal")]
pub struct Pedal {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub pedal_log: crate::generated::att::AttPedalLog,
    #[serde(flatten)]
    pub pedal_vis: crate::generated::att::AttPedalVis,
    #[serde(flatten)]
    pub pedal_ges: crate::generated::att::AttPedalGes,
    #[serde(flatten)]
    pub pedal_anl: crate::generated::att::AttPedalAnl,
}
impl crate::generated::model::ModelControlEventLikeCmn for Pedal {}
impl Validate for Pedal {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<mordent>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
/**An ornament indicating rapid alternation of the main note with a secondary note, usually a
      step below, but sometimes a step above.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mordent")]
pub struct Mordent {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub mordent_anl: crate::generated::att::AttMordentAnl,
    #[serde(flatten)]
    pub mordent_ges: crate::generated::att::AttMordentGes,
    #[serde(flatten)]
    pub mordent_log: crate::generated::att::AttMordentLog,
    #[serde(flatten)]
    pub mordent_vis: crate::generated::att::AttMordentVis,
}
impl crate::generated::model::ModelOrnamentLikeCmn for Mordent {}
impl Validate for Mordent {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<caesura>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Break, pause, or interruption in the normal tempo of a composition. Typically indicated by
"railroad tracks",i.e., two diagonal slashes.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "caesura")]
pub struct Caesura {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub caesura_anl: crate::generated::att::AttCaesuraAnl,
    #[serde(flatten)]
    pub caesura_ges: crate::generated::att::AttCaesuraGes,
    #[serde(flatten)]
    pub caesura_log: crate::generated::att::AttCaesuraLog,
    #[serde(flatten)]
    pub caesura_vis: crate::generated::att::AttCaesuraVis,
}
impl crate::generated::model::ModelControlEventLike for Caesura {}
impl Validate for Caesura {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

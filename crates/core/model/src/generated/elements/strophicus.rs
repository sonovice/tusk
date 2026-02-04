//!Element: `<strophicus>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Strophicus.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "strophicus")]
pub struct Strophicus {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub strophicus_anl: crate::generated::att::AttStrophicusAnl,
    #[serde(flatten)]
    pub strophicus_ges: crate::generated::att::AttStrophicusGes,
    #[serde(flatten)]
    pub strophicus_log: crate::generated::att::AttStrophicusLog,
    #[serde(flatten)]
    pub strophicus_vis: crate::generated::att::AttStrophicusVis,
}
impl crate::generated::model::ModelNeumeComponentModifierLike for Strophicus {}
impl Validate for Strophicus {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

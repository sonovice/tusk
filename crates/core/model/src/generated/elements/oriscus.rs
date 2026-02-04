//!Element: `<oriscus>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Oriscus.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "oriscus")]
pub struct Oriscus {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub oriscus_anl: crate::generated::att::AttOriscusAnl,
    #[serde(flatten)]
    pub oriscus_ges: crate::generated::att::AttOriscusGes,
    #[serde(flatten)]
    pub oriscus_log: crate::generated::att::AttOriscusLog,
    #[serde(flatten)]
    pub oriscus_vis: crate::generated::att::AttOriscusVis,
}
impl crate::generated::model::ModelNeumeComponentModifierLike for Oriscus {}
impl Validate for Oriscus {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

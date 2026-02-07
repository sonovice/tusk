//!Element: `<liquescent>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Liquescent.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "liquescent")]
pub struct Liquescent {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub liquescent_anl: crate::generated::att::AttLiquescentAnl,
    #[serde(flatten)]
    pub liquescent_ges: crate::generated::att::AttLiquescentGes,
    #[serde(flatten)]
    pub liquescent_log: crate::generated::att::AttLiquescentLog,
    #[serde(flatten)]
    pub liquescent_vis: crate::generated::att::AttLiquescentVis,
}
impl crate::generated::model::ModelNeumeComponentModifierLike for Liquescent {}
impl Validate for Liquescent {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

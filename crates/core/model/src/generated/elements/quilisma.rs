//!Element: `<quilisma>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Quilisma.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "quilisma")]
pub struct Quilisma {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub quilisma_anl: crate::generated::att::AttQuilismaAnl,
    #[serde(flatten)]
    pub quilisma_ges: crate::generated::att::AttQuilismaGes,
    #[serde(flatten)]
    pub quilisma_log: crate::generated::att::AttQuilismaLog,
    #[serde(flatten)]
    pub quilisma_vis: crate::generated::att::AttQuilismaVis,
}
impl crate::generated::model::ModelNeumeComponentModifierLike for Quilisma {}
impl Validate for Quilisma {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

//!Element: `<plica>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Plica
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "plica")]
pub struct Plica {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub plica_log: crate::generated::att::AttPlicaLog,
    #[serde(flatten)]
    pub plica_vis: crate::generated::att::AttPlicaVis,
    #[serde(flatten)]
    pub plica_ges: crate::generated::att::AttPlicaGes,
    #[serde(flatten)]
    pub plica_anl: crate::generated::att::AttPlicaAnl,
}
impl Validate for Plica {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

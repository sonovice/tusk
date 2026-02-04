//!Element: `<tabDurSym>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///A visual indication of the duration of atabGrp.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "tabDurSym")]
pub struct TabDurSym {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub stringtab: crate::generated::att::AttStringtab,
    #[serde(flatten)]
    pub tab_dur_sym_log: crate::generated::att::AttTabDurSymLog,
    #[serde(flatten)]
    pub tab_dur_sym_vis: crate::generated::att::AttTabDurSymVis,
    #[serde(flatten)]
    pub tab_dur_sym_ges: crate::generated::att::AttTabDurSymGes,
    #[serde(flatten)]
    pub tab_dur_sym_anl: crate::generated::att::AttTabDurSymAnl,
}
impl crate::generated::model::ModelEventLike for TabDurSym {}
impl Validate for TabDurSym {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

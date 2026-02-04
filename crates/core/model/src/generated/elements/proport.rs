//!Element: `<proport>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///proportion - Description of note duration as arithmetic ratio.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "proport")]
pub struct Proport {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub proport_log: crate::generated::att::AttProportLog,
    #[serde(flatten)]
    pub proport_vis: crate::generated::att::AttProportVis,
    #[serde(flatten)]
    pub proport_ges: crate::generated::att::AttProportGes,
    #[serde(flatten)]
    pub proport_anl: crate::generated::att::AttProportAnl,
}
impl crate::generated::model::ModelEventLikeMensural for Proport {}
impl crate::generated::model::ModelStaffDefPartMensural for Proport {}
impl Validate for Proport {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

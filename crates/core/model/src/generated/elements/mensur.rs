//!Element: `<mensur>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**mensuration - Collects information about the metrical relationship between a note value
and the next smaller value; that is, either triple or duple.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mensur")]
pub struct Mensur {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub mensur_log: crate::generated::att::AttMensurLog,
    #[serde(flatten)]
    pub mensur_vis: crate::generated::att::AttMensurVis,
    #[serde(flatten)]
    pub mensur_ges: crate::generated::att::AttMensurGes,
    #[serde(flatten)]
    pub mensur_anl: crate::generated::att::AttMensurAnl,
}
impl crate::generated::model::ModelEventLikeMensural for Mensur {}
impl crate::generated::model::ModelStaffDefPartMensural for Mensur {}
impl Validate for Mensur {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

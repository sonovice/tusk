//!Element: `<pb>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**page beginning - An empty formatting element that forces text to begin on a new
page.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "pb")]
pub struct Pb {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
    #[serde(flatten)]
    pub pb_anl: crate::generated::att::AttPbAnl,
    #[serde(flatten)]
    pub pb_ges: crate::generated::att::AttPbGes,
    #[serde(flatten)]
    pub pb_log: crate::generated::att::AttPbLog,
    #[serde(flatten)]
    pub pb_vis: crate::generated::att::AttPbVis,
}
impl crate::generated::model::ModelPbLike for Pb {}
impl Validate for Pb {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

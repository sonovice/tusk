//!Element: `<sb>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
/**system beginning - An empty formatting element that forces musical notation to begin on
      a new line.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "sb")]
pub struct Sb {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
    #[serde(flatten)]
    pub sb_anl: crate::generated::att::AttSbAnl,
    #[serde(flatten)]
    pub sb_ges: crate::generated::att::AttSbGes,
    #[serde(flatten)]
    pub sb_log: crate::generated::att::AttSbLog,
    #[serde(flatten)]
    pub sb_vis: crate::generated::att::AttSbVis,
}
impl crate::generated::model::ModelMilestoneLikeMusic for Sb {}
impl Validate for Sb {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

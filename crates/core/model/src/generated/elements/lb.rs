//!Element: `<lb>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
/**line beginning - An empty formatting element that forces text to begin on a new
      line.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lb")]
pub struct Lb {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
}
impl crate::generated::model::ModelLbLike for Lb {}
impl Validate for Lb {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

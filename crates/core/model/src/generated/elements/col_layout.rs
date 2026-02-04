//!Element: `<colLayout>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///column layout - Records the number of columns.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "colLayout")]
pub struct ColLayout {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
}
impl crate::generated::model::ModelMilestoneLikeMusic for ColLayout {}
impl crate::generated::model::ModelMilestoneLikeText for ColLayout {}
impl Validate for ColLayout {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

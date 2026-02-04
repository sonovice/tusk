//!Element: `<colLayout>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**column layout - An empty formatting element that signals the start of columnar
layout.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "colLayout")]
pub struct ColLayout {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
    ///Records the number of columns.
    #[serde(rename = "@cols", skip_serializing_if = "Option::is_none")]
    pub cols: Option<u64>,
}
impl crate::generated::model::ModelMilestoneLikeMusic for ColLayout {}
impl crate::generated::model::ModelMilestoneLikeText for ColLayout {}
impl Validate for ColLayout {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

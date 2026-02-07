//!Element: `<space>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
/**A placeholder used to fill an incomplete measure, layer, etc. most often so that the
      combined duration of the events equals the number of beats in the measure.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "space")]
pub struct Space {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub duration_quality: crate::generated::att::AttDurationQuality,
    #[serde(flatten)]
    pub space_log: crate::generated::att::AttSpaceLog,
    #[serde(flatten)]
    pub space_vis: crate::generated::att::AttSpaceVis,
    #[serde(flatten)]
    pub space_ges: crate::generated::att::AttSpaceGes,
    #[serde(flatten)]
    pub space_anl: crate::generated::att::AttSpaceAnl,
}
impl crate::generated::model::ModelEventLike for Space {}
impl Validate for Space {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

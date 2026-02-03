//!Element: `<hairpin>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Indicates continuous dynamics expressed on the score as wedge-shaped graphics,e.g., <
and >.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "hairpin")]
pub struct Hairpin {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub hairpin_log: crate::generated::att::AttHairpinLog,
    #[serde(flatten)]
    pub hairpin_vis: crate::generated::att::AttHairpinVis,
    #[serde(flatten)]
    pub hairpin_ges: crate::generated::att::AttHairpinGes,
    #[serde(flatten)]
    pub hairpin_anl: crate::generated::att::AttHairpinAnl,
}
impl crate::generated::model::ModelControlEventLikeCmn for Hairpin {}
impl Validate for Hairpin {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

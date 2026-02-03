//!Element: `<trill>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Rapid alternation of a note with another (usually at the interval of a second
above).*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "trill")]
pub struct Trill {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub trill_anl: crate::generated::att::AttTrillAnl,
    #[serde(flatten)]
    pub trill_ges: crate::generated::att::AttTrillGes,
    #[serde(flatten)]
    pub trill_log: crate::generated::att::AttTrillLog,
    #[serde(flatten)]
    pub trill_vis: crate::generated::att::AttTrillVis,
}
impl crate::generated::model::ModelOrnamentLikeCmn for Trill {}
impl Validate for Trill {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

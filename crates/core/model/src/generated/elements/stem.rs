//!Element: `<stem>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///A stem element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "stem")]
pub struct Stem {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub stem_log: crate::generated::att::AttStemLog,
    #[serde(flatten)]
    pub stem_vis: crate::generated::att::AttStemVis,
    #[serde(flatten)]
    pub stem_ges: crate::generated::att::AttStemGes,
    #[serde(flatten)]
    pub stem_anl: crate::generated::att::AttStemAnl,
}
impl Validate for Stem {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

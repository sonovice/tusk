//!Element: `<expansion>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**Indicates how a section may be programmatically expanded into its 'through-composed'
form.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "expansion")]
pub struct Expansion {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub plist: crate::generated::att::AttPlist,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
}
impl Validate for Expansion {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

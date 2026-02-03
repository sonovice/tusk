//!Element: `<context>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
/**The historical, social, intellectual, artistic, or other context within which the work was
originally conceived (e.g., the 17th century restoration of the monarchy in England, the
aesthetic movement of the late 19th century, etc.) or the historical, social, intellectual,
artistic, or other context within which the expression was realized.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "context")]
pub struct Context {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
}
impl Validate for Context {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

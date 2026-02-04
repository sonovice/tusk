//!Element: `<domainsDecl>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///domains declaration - Indicates which domains are included in the encoding.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "domainsDecl")]
pub struct DomainsDecl {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
}
impl Validate for DomainsDecl {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

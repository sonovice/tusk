//!Element: `<domainsDecl>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///domains declaration - Indicates which domains are included in the encoding.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "domainsDecl")]
pub struct DomainsDecl {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    ///
    #[serde(rename = "@anl", skip_serializing_if = "Option::is_none")]
    pub anl: Option<crate::generated::data::DataBoolean>,
    ///
    #[serde(rename = "@ges", skip_serializing_if = "Option::is_none")]
    pub ges: Option<crate::generated::data::DataBoolean>,
    ///
    #[serde(rename = "@vis", skip_serializing_if = "Option::is_none")]
    pub vis: Option<crate::generated::data::DataBoolean>,
}
impl Validate for DomainsDecl {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

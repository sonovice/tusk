//!Element: `<castGrp>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<castGrp>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CastGrpChild {
    #[serde(rename = "roleDesc")]
    RoleDesc(Box<crate::generated::elements::RoleDesc>),
    #[serde(rename = "castItem")]
    CastItem(Box<crate::generated::elements::CastItem>),
    #[serde(rename = "castGrp")]
    CastGrp(Box<crate::generated::elements::CastGrp>),
}
impl CastGrpChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            CastGrpChild::RoleDesc(elem) => {
                ctx.enter("roleDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CastGrpChild::CastItem(elem) => {
                ctx.enter("castItem", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CastGrpChild::CastGrp(elem) => {
                ctx.enter("castGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///cast group - Groups one or more individual castItem elements within a cast list.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "castGrp")]
pub struct CastGrp {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<CastGrpChild>,
}
impl Validate for CastGrp {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

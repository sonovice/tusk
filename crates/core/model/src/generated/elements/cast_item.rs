//!Element: `<castItem>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<castItem>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CastItemChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "actor")]
    Actor(Box<crate::generated::elements::Actor>),
    #[serde(rename = "role")]
    Role(Box<crate::generated::elements::Role>),
    #[serde(rename = "roleDesc")]
    RoleDesc(Box<crate::generated::elements::RoleDesc>),
    #[serde(rename = "perfRes")]
    PerfRes(Box<crate::generated::elements::PerfRes>),
}
impl CastItemChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            CastItemChild::Text(_) => {}
            CastItemChild::Actor(elem) => {
                ctx.enter("actor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CastItemChild::Role(elem) => {
                ctx.enter("role", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CastItemChild::RoleDesc(elem) => {
                ctx.enter("roleDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            CastItemChild::PerfRes(elem) => {
                ctx.enter("perfRes", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Contains a single entry within a cast list, describing either a single role or a list of
non-speaking roles.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "castItem")]
pub struct CastItem {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<CastItemChild>,
}
impl Validate for CastItem {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

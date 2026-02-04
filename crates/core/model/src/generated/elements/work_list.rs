//!Element: `<workList>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<workList>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkListChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "work")]
    Work(Box<crate::generated::elements::Work>),
}
impl WorkListChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            WorkListChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkListChild::Work(elem) => {
                ctx.enter("work", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**work list - Grouping mechanism for information describing non-bibliographic aspects of a
text.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "workList")]
pub struct WorkList {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<WorkListChild>,
}
impl crate::generated::model::ModelHeaderPart for WorkList {}
impl Validate for WorkList {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

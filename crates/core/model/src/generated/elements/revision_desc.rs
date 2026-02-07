//!Element: `<revisionDesc>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<revisionDesc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RevisionDescChild {
    #[serde(rename = "change")]
    Change(Box<crate::generated::elements::Change>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
impl RevisionDescChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            RevisionDescChild::Change(elem) => {
                ctx.enter("change", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RevisionDescChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**revision description - Container for information about alterations that have been made
      to an MEI file.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "revisionDesc")]
pub struct RevisionDesc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<RevisionDescChild>,
}
impl Validate for RevisionDesc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

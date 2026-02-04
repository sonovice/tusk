//!Element: `<genDesc>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<genDesc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GenDescChild {
    #[serde(rename = "genDesc")]
    GenDesc(Box<crate::generated::elements::GenDesc>),
    #[serde(rename = "genState")]
    GenState(Box<crate::generated::elements::GenState>),
}
impl GenDescChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            GenDescChild::GenDesc(elem) => {
                ctx.enter("genDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            GenDescChild::GenState(elem) => {
                ctx.enter("genState", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**genetic description - When set to "true" the child elements are known to be in chronological order. When set
to "false" or when not provided, the order of child elements is unknown.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "genDesc")]
pub struct GenDesc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<GenDescChild>,
}
impl crate::generated::model::ModelResourceLike for GenDesc {}
impl Validate for GenDesc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

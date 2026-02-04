//!Element: `<physLoc>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<physLoc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PhysLocChild {
    #[serde(rename = "repository")]
    Repository(Box<crate::generated::elements::Repository>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "history")]
    History(Box<crate::generated::elements::History>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
}
impl PhysLocChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PhysLocChild::Repository(elem) => {
                ctx.enter("repository", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysLocChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysLocChild::History(elem) => {
                ctx.enter("history", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PhysLocChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**physical location - Groups information about the current physical location of a
bibliographic item, such as the repository in which it is located and its shelf mark(s), and
its previous locations.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "physLoc")]
pub struct PhysLoc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PhysLocChild>,
}
impl crate::generated::model::ModelBiblPart for PhysLoc {}
impl Validate for PhysLoc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

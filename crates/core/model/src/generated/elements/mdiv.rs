//!Element: `<mdiv>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<mdiv>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MdivChild {
    #[serde(rename = "parts")]
    Parts(Box<crate::generated::elements::Parts>),
    #[serde(rename = "score")]
    Score(Box<crate::generated::elements::Score>),
    #[serde(rename = "mdiv")]
    Mdiv(Box<crate::generated::elements::Mdiv>),
}
impl MdivChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MdivChild::Parts(elem) => {
                ctx.enter("parts", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MdivChild::Score(elem) => {
                ctx.enter("score", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MdivChild::Mdiv(elem) => {
                ctx.enter("mdiv", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///musical division - Contains a subdivision of the body of a musical text.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "mdiv")]
pub struct Mdiv {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub mdiv_anl: crate::generated::att::AttMdivAnl,
    #[serde(flatten)]
    pub mdiv_ges: crate::generated::att::AttMdivGes,
    #[serde(flatten)]
    pub mdiv_log: crate::generated::att::AttMdivLog,
    #[serde(flatten)]
    pub mdiv_vis: crate::generated::att::AttMdivVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MdivChild>,
}
impl crate::generated::model::ModelMdivLike for Mdiv {}
impl Validate for Mdiv {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

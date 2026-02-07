//!Element: `<fig>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<fig>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FigChild {
    #[serde(rename = "graphic")]
    Graphic(Box<crate::generated::elements::Graphic>),
    #[serde(rename = "caption")]
    Caption(Box<crate::generated::elements::Caption>),
    #[serde(rename = "score")]
    Score(Box<crate::generated::elements::Score>),
    #[serde(rename = "figDesc")]
    FigDesc(Box<crate::generated::elements::FigDesc>),
}
impl FigChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            FigChild::Graphic(elem) => {
                ctx.enter("graphic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FigChild::Caption(elem) => {
                ctx.enter("caption", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FigChild::Score(elem) => {
                ctx.enter("score", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FigChild::FigDesc(elem) => {
                ctx.enter("figDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**figure - Groups elements representing or containing graphic information such as an
      illustration or figure.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "fig")]
pub struct Fig {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub horizontal_align: crate::generated::att::AttHorizontalAlign,
    #[serde(flatten)]
    pub vertical_align: crate::generated::att::AttVerticalAlign,
    #[serde(flatten)]
    pub xy: crate::generated::att::AttXy,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<FigChild>,
}
impl crate::generated::model::ModelFigureLike for Fig {}
impl Validate for Fig {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

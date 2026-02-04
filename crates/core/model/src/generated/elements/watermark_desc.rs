//!Element: `<watermarkDesc>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<watermarkDesc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WatermarkDescChild {
    #[serde(rename = "watermarkList")]
    WatermarkList(Box<crate::generated::elements::WatermarkList>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "watermark")]
    Watermark(Box<crate::generated::elements::Watermark>),
}
impl WatermarkDescChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            WatermarkDescChild::WatermarkList(elem) => {
                ctx.enter("watermarkList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WatermarkDescChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WatermarkDescChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WatermarkDescChild::Watermark(elem) => {
                ctx.enter("watermark", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///watermark description - Contains a description of the watermark(s) of an item.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "watermarkDesc")]
pub struct WatermarkDesc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<WatermarkDescChild>,
}
impl crate::generated::model::ModelPhysDescPart for WatermarkDesc {}
impl Validate for WatermarkDesc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

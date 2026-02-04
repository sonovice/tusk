//!Element: `<ossia>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<ossia>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OssiaChild {
    #[serde(rename = "layer")]
    Layer(Box<crate::generated::elements::Layer>),
    #[serde(rename = "oLayer")]
    OLayer(Box<crate::generated::elements::OLayer>),
    #[serde(rename = "oStaff")]
    OStaff(Box<crate::generated::elements::OStaff>),
    #[serde(rename = "staff")]
    Staff(Box<crate::generated::elements::Staff>),
}
impl OssiaChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            OssiaChild::Layer(elem) => {
                ctx.enter("layer", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            OssiaChild::OLayer(elem) => {
                ctx.enter("oLayer", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            OssiaChild::OStaff(elem) => {
                ctx.enter("oStaff", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            OssiaChild::Staff(elem) => {
                ctx.enter("staff", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Captures original notation and a differently notated version*present in
      the source being transcribed*.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ossia")]
pub struct Ossia {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub ossia_log: crate::generated::att::AttOssiaLog,
    #[serde(flatten)]
    pub ossia_vis: crate::generated::att::AttOssiaVis,
    #[serde(flatten)]
    pub ossia_ges: crate::generated::att::AttOssiaGes,
    #[serde(flatten)]
    pub ossia_anl: crate::generated::att::AttOssiaAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<OssiaChild>,
}
impl crate::generated::model::ModelOssiaLike for Ossia {}
impl Validate for Ossia {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

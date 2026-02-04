//!Element: `<manifestationList>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<manifestationList>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ManifestationListChild {
    #[serde(rename = "manifestation")]
    Manifestation(Box<crate::generated::elements::Manifestation>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
impl ManifestationListChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ManifestationListChild::Manifestation(elem) => {
                ctx.enter("manifestation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ManifestationListChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**A container for the descriptions of physical embodiments of an expression of a
work.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "manifestationList")]
pub struct ManifestationList {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ManifestationListChild>,
}
impl Validate for ManifestationList {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

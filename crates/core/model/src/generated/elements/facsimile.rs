//!Element: `<facsimile>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<facsimile>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FacsimileChild {
    #[serde(rename = "surface")]
    Surface(Box<crate::generated::elements::Surface>),
    #[serde(rename = "graphic")]
    Graphic(Box<crate::generated::elements::Graphic>),
}
impl FacsimileChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            FacsimileChild::Surface(elem) => {
                ctx.enter("surface", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            FacsimileChild::Graphic(elem) => {
                ctx.enter("graphic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Contains a representation of a written source in the form of a set of images rather than
as transcribed or encoded text.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "facsimile")]
pub struct Facsimile {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<FacsimileChild>,
}
impl crate::generated::model::ModelResourceLike for Facsimile {}
impl Validate for Facsimile {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

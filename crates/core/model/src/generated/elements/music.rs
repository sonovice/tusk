//!Element: `<music>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<music>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MusicChild {
    #[serde(rename = "group")]
    Group(Box<crate::generated::elements::Group>),
    #[serde(rename = "back")]
    Back(Box<crate::generated::elements::Back>),
    #[serde(rename = "performance")]
    Performance(Box<crate::generated::elements::Performance>),
    #[serde(rename = "facsimile")]
    Facsimile(Box<crate::generated::elements::Facsimile>),
    #[serde(rename = "genDesc")]
    GenDesc(Box<crate::generated::elements::GenDesc>),
    #[serde(rename = "front")]
    Front(Box<crate::generated::elements::Front>),
    #[serde(rename = "body")]
    Body(Box<crate::generated::elements::Body>),
}
impl MusicChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MusicChild::Group(elem) => {
                ctx.enter("group", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MusicChild::Back(elem) => {
                ctx.enter("back", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MusicChild::Performance(elem) => {
                ctx.enter("performance", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MusicChild::Facsimile(elem) => {
                ctx.enter("facsimile", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MusicChild::GenDesc(elem) => {
                ctx.enter("genDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MusicChild::Front(elem) => {
                ctx.enter("front", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MusicChild::Body(elem) => {
                ctx.enter("body", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Contains a single musical text of any kind, whether unitary or composite, for example, an
      etude, opera, song cycle, symphony, or anthology of piano solos.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "music")]
pub struct Music {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub mei_version: crate::generated::att::AttMeiVersion,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MusicChild>,
}
impl Validate for Music {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

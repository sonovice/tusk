//!Element: `<meiHead>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<meiHead>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MeiHeadChild {
    #[serde(rename = "encodingDesc")]
    EncodingDesc(Box<crate::generated::elements::EncodingDesc>),
    #[serde(rename = "workList")]
    WorkList(Box<crate::generated::elements::WorkList>),
    #[serde(rename = "manifestationList")]
    ManifestationList(Box<crate::generated::elements::ManifestationList>),
    #[serde(rename = "extMeta")]
    ExtMeta(Box<crate::generated::elements::ExtMeta>),
    #[serde(rename = "fileDesc")]
    FileDesc(Box<crate::generated::elements::FileDesc>),
    #[serde(rename = "revisionDesc")]
    RevisionDesc(Box<crate::generated::elements::RevisionDesc>),
    #[serde(rename = "altId")]
    AltId(Box<crate::generated::elements::AltId>),
}
impl MeiHeadChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MeiHeadChild::EncodingDesc(elem) => {
                ctx.enter("encodingDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeiHeadChild::WorkList(elem) => {
                ctx.enter("workList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeiHeadChild::ManifestationList(elem) => {
                ctx.enter("manifestationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeiHeadChild::ExtMeta(elem) => {
                ctx.enter("extMeta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeiHeadChild::FileDesc(elem) => {
                ctx.enter("fileDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeiHeadChild::RevisionDesc(elem) => {
                ctx.enter("revisionDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MeiHeadChild::AltId(elem) => {
                ctx.enter("altId", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**MEI header - Supplies the descriptive and declarative metadata prefixed to every
MEI-conformant text.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "meiHead")]
pub struct MeiHead {
    #[serde(flatten)]
    pub basic: crate::generated::att::AttBasic,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub labelled: crate::generated::att::AttLabelled,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub mei_version: crate::generated::att::AttMeiVersion,
    #[serde(flatten)]
    pub responsibility: crate::generated::att::AttResponsibility,
    /**Specifies the kind of document to which the header is attached, for example whether it
    is a corpus or individual text.*/
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MeiHeadChild>,
}
impl crate::generated::model::ModelStartLikeHeader for MeiHead {}
impl Validate for MeiHead {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

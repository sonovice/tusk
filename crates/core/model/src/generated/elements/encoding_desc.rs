//!Element: `<encodingDesc>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<encodingDesc>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EncodingDescChild {
    #[serde(rename = "tagsDecl")]
    TagsDecl(Box<crate::generated::elements::TagsDecl>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "projectDesc")]
    ProjectDesc(Box<crate::generated::elements::ProjectDesc>),
    #[serde(rename = "editorialDecl")]
    EditorialDecl(Box<crate::generated::elements::EditorialDecl>),
    #[serde(rename = "appInfo")]
    AppInfo(Box<crate::generated::elements::AppInfo>),
    #[serde(rename = "samplingDecl")]
    SamplingDecl(Box<crate::generated::elements::SamplingDecl>),
    #[serde(rename = "domainsDecl")]
    DomainsDecl(Box<crate::generated::elements::DomainsDecl>),
    #[serde(rename = "classDecls")]
    ClassDecls(Box<crate::generated::elements::ClassDecls>),
}
impl EncodingDescChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            EncodingDescChild::TagsDecl(elem) => {
                ctx.enter("tagsDecl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EncodingDescChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EncodingDescChild::ProjectDesc(elem) => {
                ctx.enter("projectDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EncodingDescChild::EditorialDecl(elem) => {
                ctx.enter("editorialDecl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EncodingDescChild::AppInfo(elem) => {
                ctx.enter("appInfo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EncodingDescChild::SamplingDecl(elem) => {
                ctx.enter("samplingDecl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EncodingDescChild::DomainsDecl(elem) => {
                ctx.enter("domainsDecl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EncodingDescChild::ClassDecls(elem) => {
                ctx.enter("classDecls", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**encoding description - Documents the relationship between an electronic file and the
      source or sources from which it was derived as well as applications used in the
      encoding/editing process.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "encodingDesc")]
pub struct EncodingDesc {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<EncodingDescChild>,
}
impl crate::generated::model::ModelHeaderPart for EncodingDesc {}
impl Validate for EncodingDesc {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

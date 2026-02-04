//!Element: `<monogr>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<monogr>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MonogrChild {
    #[serde(rename = "editor")]
    Editor(Box<crate::generated::elements::Editor>),
    #[serde(rename = "corpName")]
    CorpName(Box<crate::generated::elements::CorpName>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "funder")]
    Funder(Box<crate::generated::elements::Funder>),
    #[serde(rename = "sponsor")]
    Sponsor(Box<crate::generated::elements::Sponsor>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
    #[serde(rename = "creator")]
    Creator(Box<crate::generated::elements::Creator>),
    #[serde(rename = "edition")]
    Edition(Box<crate::generated::elements::Edition>),
    #[serde(rename = "imprint")]
    Imprint(Box<crate::generated::elements::Imprint>),
    #[serde(rename = "contributor")]
    Contributor(Box<crate::generated::elements::Contributor>),
    #[serde(rename = "respStmt")]
    RespStmt(Box<crate::generated::elements::RespStmt>),
}
impl MonogrChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            MonogrChild::Editor(elem) => {
                ctx.enter("editor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::CorpName(elem) => {
                ctx.enter("corpName", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Funder(elem) => {
                ctx.enter("funder", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Sponsor(elem) => {
                ctx.enter("sponsor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Creator(elem) => {
                ctx.enter("creator", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Edition(elem) => {
                ctx.enter("edition", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Imprint(elem) => {
                ctx.enter("imprint", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::Contributor(elem) => {
                ctx.enter("contributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            MonogrChild::RespStmt(elem) => {
                ctx.enter("respStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**monograph level - Contains bibliographic elements describing an item, for example, a
published book or journal, score, recording, or an unpublished manuscript.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "monogr")]
pub struct Monogr {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub record_type: crate::generated::att::AttRecordType,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<MonogrChild>,
}
impl Validate for Monogr {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

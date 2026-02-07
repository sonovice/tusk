//!Element: `<editionStmt>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<editionStmt>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EditionStmtChild {
    #[serde(rename = "respStmt")]
    RespStmt(Box<crate::generated::elements::RespStmt>),
    #[serde(rename = "sponsor")]
    Sponsor(Box<crate::generated::elements::Sponsor>),
    #[serde(rename = "funder")]
    Funder(Box<crate::generated::elements::Funder>),
    #[serde(rename = "contributor")]
    Contributor(Box<crate::generated::elements::Contributor>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "editor")]
    Editor(Box<crate::generated::elements::Editor>),
    #[serde(rename = "creator")]
    Creator(Box<crate::generated::elements::Creator>),
    #[serde(rename = "edition")]
    Edition(Box<crate::generated::elements::Edition>),
}
impl EditionStmtChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            EditionStmtChild::RespStmt(elem) => {
                ctx.enter("respStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditionStmtChild::Sponsor(elem) => {
                ctx.enter("sponsor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditionStmtChild::Funder(elem) => {
                ctx.enter("funder", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditionStmtChild::Contributor(elem) => {
                ctx.enter("contributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditionStmtChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditionStmtChild::Editor(elem) => {
                ctx.enter("editor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditionStmtChild::Creator(elem) => {
                ctx.enter("creator", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            EditionStmtChild::Edition(elem) => {
                ctx.enter("edition", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**edition statement - Container for meta-data pertaining to a particular edition of the
material being described.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "editionStmt")]
pub struct EditionStmt {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<EditionStmtChild>,
}
impl Validate for EditionStmt {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

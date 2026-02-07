//!Element: `<titleStmt>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<titleStmt>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TitleStmtChild {
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "creator")]
    Creator(Box<crate::generated::elements::Creator>),
    #[serde(rename = "respStmt")]
    RespStmt(Box<crate::generated::elements::RespStmt>),
    #[serde(rename = "editor")]
    Editor(Box<crate::generated::elements::Editor>),
    #[serde(rename = "funder")]
    Funder(Box<crate::generated::elements::Funder>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "sponsor")]
    Sponsor(Box<crate::generated::elements::Sponsor>),
    #[serde(rename = "contributor")]
    Contributor(Box<crate::generated::elements::Contributor>),
}
impl TitleStmtChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            TitleStmtChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitleStmtChild::Creator(elem) => {
                ctx.enter("creator", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitleStmtChild::RespStmt(elem) => {
                ctx.enter("respStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitleStmtChild::Editor(elem) => {
                ctx.enter("editor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitleStmtChild::Funder(elem) => {
                ctx.enter("funder", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitleStmtChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitleStmtChild::Sponsor(elem) => {
                ctx.enter("sponsor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            TitleStmtChild::Contributor(elem) => {
                ctx.enter("contributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///title statement - Container for title and responsibility meta-data.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "titleStmt")]
pub struct TitleStmt {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<TitleStmtChild>,
}
impl Validate for TitleStmt {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

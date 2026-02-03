//!Element: `<expression>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<expression>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExpressionChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "otherChar")]
    OtherChar(Box<crate::generated::elements::OtherChar>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "key")]
    Key(Box<crate::generated::elements::Key>),
    #[serde(rename = "history")]
    History(Box<crate::generated::elements::History>),
    #[serde(rename = "mensuration")]
    Mensuration(Box<crate::generated::elements::Mensuration>),
    #[serde(rename = "langUsage")]
    LangUsage(Box<crate::generated::elements::LangUsage>),
    #[serde(rename = "incip")]
    Incip(Box<crate::generated::elements::Incip>),
    #[serde(rename = "perfDuration")]
    PerfDuration(Box<crate::generated::elements::PerfDuration>),
    #[serde(rename = "extent")]
    Extent(Box<crate::generated::elements::Extent>),
    #[serde(rename = "scoreFormat")]
    ScoreFormat(Box<crate::generated::elements::ScoreFormat>),
    #[serde(rename = "contents")]
    Contents(Box<crate::generated::elements::Contents>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "notesStmt")]
    NotesStmt(Box<crate::generated::elements::NotesStmt>),
    #[serde(rename = "classification")]
    Classification(Box<crate::generated::elements::Classification>),
    #[serde(rename = "creation")]
    Creation(Box<crate::generated::elements::Creation>),
    #[serde(rename = "context")]
    Context(Box<crate::generated::elements::Context>),
    #[serde(rename = "componentList")]
    ComponentList(Box<crate::generated::elements::ComponentList>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
    #[serde(rename = "dedication")]
    Dedication(Box<crate::generated::elements::Dedication>),
    #[serde(rename = "meter")]
    Meter(Box<crate::generated::elements::Meter>),
    #[serde(rename = "perfMedium")]
    PerfMedium(Box<crate::generated::elements::PerfMedium>),
    #[serde(rename = "extMeta")]
    ExtMeta(Box<crate::generated::elements::ExtMeta>),
    #[serde(rename = "respStmt")]
    RespStmt(Box<crate::generated::elements::RespStmt>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
}
impl ExpressionChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ExpressionChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::OtherChar(elem) => {
                ctx.enter("otherChar", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Key(elem) => {
                ctx.enter("key", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::History(elem) => {
                ctx.enter("history", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Mensuration(elem) => {
                ctx.enter("mensuration", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::LangUsage(elem) => {
                ctx.enter("langUsage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Incip(elem) => {
                ctx.enter("incip", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::PerfDuration(elem) => {
                ctx.enter("perfDuration", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Extent(elem) => {
                ctx.enter("extent", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::ScoreFormat(elem) => {
                ctx.enter("scoreFormat", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Contents(elem) => {
                ctx.enter("contents", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::NotesStmt(elem) => {
                ctx.enter("notesStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Classification(elem) => {
                ctx.enter("classification", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Creation(elem) => {
                ctx.enter("creation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Context(elem) => {
                ctx.enter("context", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::ComponentList(elem) => {
                ctx.enter("componentList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Dedication(elem) => {
                ctx.enter("dedication", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Meter(elem) => {
                ctx.enter("meter", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::PerfMedium(elem) => {
                ctx.enter("perfMedium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::ExtMeta(elem) => {
                ctx.enter("extMeta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::RespStmt(elem) => {
                ctx.enter("respStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ExpressionChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Intellectual or artistic realization of a work.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "expression")]
pub struct Expression {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ExpressionChild>,
}
impl crate::generated::model::ModelExpressionLike for Expression {}
impl Validate for Expression {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

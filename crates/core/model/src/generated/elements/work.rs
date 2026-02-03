//!Element: `<work>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<work>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorkChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "componentList")]
    ComponentList(Box<crate::generated::elements::ComponentList>),
    #[serde(rename = "key")]
    Key(Box<crate::generated::elements::Key>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
    #[serde(rename = "extMeta")]
    ExtMeta(Box<crate::generated::elements::ExtMeta>),
    #[serde(rename = "audience")]
    Audience(Box<crate::generated::elements::Audience>),
    #[serde(rename = "incip")]
    Incip(Box<crate::generated::elements::Incip>),
    #[serde(rename = "otherChar")]
    OtherChar(Box<crate::generated::elements::OtherChar>),
    #[serde(rename = "perfMedium")]
    PerfMedium(Box<crate::generated::elements::PerfMedium>),
    #[serde(rename = "dedication")]
    Dedication(Box<crate::generated::elements::Dedication>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "respStmt")]
    RespStmt(Box<crate::generated::elements::RespStmt>),
    #[serde(rename = "meter")]
    Meter(Box<crate::generated::elements::Meter>),
    #[serde(rename = "langUsage")]
    LangUsage(Box<crate::generated::elements::LangUsage>),
    #[serde(rename = "contents")]
    Contents(Box<crate::generated::elements::Contents>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "mensuration")]
    Mensuration(Box<crate::generated::elements::Mensuration>),
    #[serde(rename = "history")]
    History(Box<crate::generated::elements::History>),
    #[serde(rename = "creation")]
    Creation(Box<crate::generated::elements::Creation>),
    #[serde(rename = "perfDuration")]
    PerfDuration(Box<crate::generated::elements::PerfDuration>),
    #[serde(rename = "context")]
    Context(Box<crate::generated::elements::Context>),
    #[serde(rename = "notesStmt")]
    NotesStmt(Box<crate::generated::elements::NotesStmt>),
    #[serde(rename = "classification")]
    Classification(Box<crate::generated::elements::Classification>),
    #[serde(rename = "expressionList")]
    ExpressionList(Box<crate::generated::elements::ExpressionList>),
    #[serde(rename = "title")]
    Title(Box<crate::generated::elements::Title>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
}
impl WorkChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            WorkChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::ComponentList(elem) => {
                ctx.enter("componentList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Key(elem) => {
                ctx.enter("key", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::ExtMeta(elem) => {
                ctx.enter("extMeta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Audience(elem) => {
                ctx.enter("audience", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Incip(elem) => {
                ctx.enter("incip", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::OtherChar(elem) => {
                ctx.enter("otherChar", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::PerfMedium(elem) => {
                ctx.enter("perfMedium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Dedication(elem) => {
                ctx.enter("dedication", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::RespStmt(elem) => {
                ctx.enter("respStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Meter(elem) => {
                ctx.enter("meter", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::LangUsage(elem) => {
                ctx.enter("langUsage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Contents(elem) => {
                ctx.enter("contents", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Mensuration(elem) => {
                ctx.enter("mensuration", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::History(elem) => {
                ctx.enter("history", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Creation(elem) => {
                ctx.enter("creation", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::PerfDuration(elem) => {
                ctx.enter("perfDuration", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Context(elem) => {
                ctx.enter("context", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::NotesStmt(elem) => {
                ctx.enter("notesStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Classification(elem) => {
                ctx.enter("classification", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::ExpressionList(elem) => {
                ctx.enter("expressionList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::Title(elem) => {
                ctx.enter("title", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            WorkChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Provides a detailed description of a work — a distinct intellectual or artistic creation —
specifically its history, language use, and high-level musical attributes (e.g., key, tempo,
meter, medium of performance, and intended duration).*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "work")]
pub struct Work {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<WorkChild>,
}
impl crate::generated::model::ModelWorkLike for Work {}
impl Validate for Work {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<item>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<item>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ItemChild {
    #[serde(rename = "extMeta")]
    ExtMeta(Box<crate::generated::elements::ExtMeta>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "dedication")]
    Dedication(Box<crate::generated::elements::Dedication>),
    #[serde(rename = "componentList")]
    ComponentList(Box<crate::generated::elements::ComponentList>),
    #[serde(rename = "classification")]
    Classification(Box<crate::generated::elements::Classification>),
    #[serde(rename = "physLoc")]
    PhysLoc(Box<crate::generated::elements::PhysLoc>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "physDesc")]
    PhysDesc(Box<crate::generated::elements::PhysDesc>),
    #[serde(rename = "notesStmt")]
    NotesStmt(Box<crate::generated::elements::NotesStmt>),
    #[serde(rename = "availability")]
    Availability(Box<crate::generated::elements::Availability>),
    #[serde(rename = "history")]
    History(Box<crate::generated::elements::History>),
    #[serde(rename = "relationList")]
    RelationList(Box<crate::generated::elements::RelationList>),
}
impl ItemChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ItemChild::ExtMeta(elem) => {
                ctx.enter("extMeta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::Dedication(elem) => {
                ctx.enter("dedication", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::ComponentList(elem) => {
                ctx.enter("componentList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::Classification(elem) => {
                ctx.enter("classification", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::PhysLoc(elem) => {
                ctx.enter("physLoc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::PhysDesc(elem) => {
                ctx.enter("physDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::NotesStmt(elem) => {
                ctx.enter("notesStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::Availability(elem) => {
                ctx.enter("availability", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::History(elem) => {
                ctx.enter("history", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ItemChild::RelationList(elem) => {
                ctx.enter("relationList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Single instance or exemplar of a source/manifestation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "item")]
pub struct Item {
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
    pub children: Vec<ItemChild>,
}
impl crate::generated::model::ModelItemLike for Item {}
impl Validate for Item {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

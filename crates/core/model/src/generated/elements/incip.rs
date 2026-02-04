//!Element: `<incip>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<incip>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IncipChild {
    #[serde(rename = "meter")]
    Meter(Box<crate::generated::elements::Meter>),
    #[serde(rename = "perfResList")]
    PerfResList(Box<crate::generated::elements::PerfResList>),
    #[serde(rename = "incipCode")]
    IncipCode(Box<crate::generated::elements::IncipCode>),
    #[serde(rename = "incipText")]
    IncipText(Box<crate::generated::elements::IncipText>),
    #[serde(rename = "key")]
    Key(Box<crate::generated::elements::Key>),
    #[serde(rename = "role")]
    Role(Box<crate::generated::elements::Role>),
    #[serde(rename = "graphic")]
    Graphic(Box<crate::generated::elements::Graphic>),
    #[serde(rename = "clefGrp")]
    ClefGrp(Box<crate::generated::elements::ClefGrp>),
    #[serde(rename = "score")]
    Score(Box<crate::generated::elements::Score>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "mensuration")]
    Mensuration(Box<crate::generated::elements::Mensuration>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
    #[serde(rename = "perfRes")]
    PerfRes(Box<crate::generated::elements::PerfRes>),
    #[serde(rename = "clef")]
    Clef(Box<crate::generated::elements::Clef>),
    #[serde(rename = "annot")]
    Annot(Box<crate::generated::elements::Annot>),
}
impl IncipChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            IncipChild::Meter(elem) => {
                ctx.enter("meter", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::PerfResList(elem) => {
                ctx.enter("perfResList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::IncipCode(elem) => {
                ctx.enter("incipCode", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::IncipText(elem) => {
                ctx.enter("incipText", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Key(elem) => {
                ctx.enter("key", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Role(elem) => {
                ctx.enter("role", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Graphic(elem) => {
                ctx.enter("graphic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::ClefGrp(elem) => {
                ctx.enter("clefGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Score(elem) => {
                ctx.enter("score", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Mensuration(elem) => {
                ctx.enter("mensuration", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::PerfRes(elem) => {
                ctx.enter("perfRes", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Clef(elem) => {
                ctx.enter("clef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            IncipChild::Annot(elem) => {
                ctx.enter("annot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///incipit - The opening music and/or words of a musical or textual work.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "incip")]
pub struct Incip {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<IncipChild>,
}
impl crate::generated::model::ModelIncipLike for Incip {}
impl Validate for Incip {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<refrain>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<refrain>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RefrainChild {
    #[serde(rename = "dynam")]
    Dynam(Box<crate::generated::elements::Dynam>),
    #[serde(rename = "space")]
    Space(Box<crate::generated::elements::Space>),
    #[serde(rename = "volta")]
    Volta(Box<crate::generated::elements::Volta>),
    #[serde(rename = "syl")]
    Syl(Box<crate::generated::elements::Syl>),
    #[serde(rename = "app")]
    App(Box<crate::generated::elements::App>),
    #[serde(rename = "subst")]
    Subst(Box<crate::generated::elements::Subst>),
    #[serde(rename = "lb")]
    Lb(Box<crate::generated::elements::Lb>),
    #[serde(rename = "dir")]
    Dir(Box<crate::generated::elements::Dir>),
    #[serde(rename = "choice")]
    Choice(Box<crate::generated::elements::Choice>),
    #[serde(rename = "tempo")]
    Tempo(Box<crate::generated::elements::Tempo>),
}
impl RefrainChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            RefrainChild::Dynam(elem) => {
                ctx.enter("dynam", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::Space(elem) => {
                ctx.enter("space", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::Volta(elem) => {
                ctx.enter("volta", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::Syl(elem) => {
                ctx.enter("syl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::App(elem) => {
                ctx.enter("app", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::Subst(elem) => {
                ctx.enter("subst", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::Lb(elem) => {
                ctx.enter("lb", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::Dir(elem) => {
                ctx.enter("dir", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::Choice(elem) => {
                ctx.enter("choice", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RefrainChild::Tempo(elem) => {
                ctx.enter("tempo", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Recurring lyrics, especially at the end of each verse or stanza of a poem or song lyrics;
a chorus.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "refrain")]
pub struct Refrain {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub refrain_log: crate::generated::att::AttRefrainLog,
    #[serde(flatten)]
    pub refrain_vis: crate::generated::att::AttRefrainVis,
    #[serde(flatten)]
    pub refrain_ges: crate::generated::att::AttRefrainGes,
    #[serde(flatten)]
    pub refrain_anl: crate::generated::att::AttRefrainAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<RefrainChild>,
}
impl crate::generated::model::ModelVerseLike for Refrain {}
impl Validate for Refrain {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<history>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<history>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HistoryChild {
    #[serde(rename = "list")]
    List(Box<crate::generated::elements::List>),
    #[serde(rename = "p")]
    P(Box<crate::generated::elements::P>),
    #[serde(rename = "biblList")]
    BiblList(Box<crate::generated::elements::BiblList>),
    #[serde(rename = "castList")]
    CastList(Box<crate::generated::elements::CastList>),
    #[serde(rename = "quote")]
    Quote(Box<crate::generated::elements::Quote>),
    #[serde(rename = "treatSched")]
    TreatSched(Box<crate::generated::elements::TreatSched>),
    #[serde(rename = "table")]
    Table(Box<crate::generated::elements::Table>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "treatHist")]
    TreatHist(Box<crate::generated::elements::TreatHist>),
    #[serde(rename = "provenance")]
    Provenance(Box<crate::generated::elements::Provenance>),
    #[serde(rename = "div")]
    Div(Box<crate::generated::elements::Div>),
    #[serde(rename = "lg")]
    Lg(Box<crate::generated::elements::Lg>),
    #[serde(rename = "acquisition")]
    Acquisition(Box<crate::generated::elements::Acquisition>),
    #[serde(rename = "eventList")]
    EventList(Box<crate::generated::elements::EventList>),
    #[serde(rename = "exhibHist")]
    ExhibHist(Box<crate::generated::elements::ExhibHist>),
}
impl HistoryChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            HistoryChild::List(elem) => {
                ctx.enter("list", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::P(elem) => {
                ctx.enter("p", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::BiblList(elem) => {
                ctx.enter("biblList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::CastList(elem) => {
                ctx.enter("castList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::Quote(elem) => {
                ctx.enter("quote", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::TreatSched(elem) => {
                ctx.enter("treatSched", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::Table(elem) => {
                ctx.enter("table", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::TreatHist(elem) => {
                ctx.enter("treatHist", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::Provenance(elem) => {
                ctx.enter("provenance", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::Div(elem) => {
                ctx.enter("div", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::Lg(elem) => {
                ctx.enter("lg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::Acquisition(elem) => {
                ctx.enter("acquisition", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::EventList(elem) => {
                ctx.enter("eventList", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            HistoryChild::ExhibHist(elem) => {
                ctx.enter("exhibHist", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Provides a container for information about the history of a resource other than the
      circumstances of its creation.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "history")]
pub struct History {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<HistoryChild>,
}
impl crate::generated::model::ModelPhysDescPart for History {}
impl Validate for History {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

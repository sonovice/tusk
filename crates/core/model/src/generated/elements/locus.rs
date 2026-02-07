//!Element: `<locus>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<locus>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocusChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "symbol")]
    Symbol(Box<crate::generated::elements::Symbol>),
    #[serde(rename = "rend")]
    Rend(Box<crate::generated::elements::Rend>),
    #[serde(rename = "locus")]
    Locus(Box<crate::generated::elements::Locus>),
}
impl LocusChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            LocusChild::Text(_) => {}
            LocusChild::Symbol(elem) => {
                ctx.enter("symbol", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LocusChild::Rend(elem) => {
                ctx.enter("rend", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LocusChild::Locus(elem) => {
                ctx.enter("locus", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Defines a location within a manuscript or manuscript component, usually as a (possibly
      discontinuous) sequence of folio references.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "locus")]
pub struct Locus {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub foliation_scheme: crate::generated::att::AttFoliationScheme,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    ///Specifies the starting point of the location in a normalized form.
    #[serde(rename = "@from", skip_serializing_if = "Option::is_none")]
    pub from: Option<crate::generated::data::DataWord>,
    ///Specifies the end-point of the location in a normalized form.
    #[serde(rename = "@to", skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::generated::data::DataWord>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<LocusChild>,
}
impl crate::generated::model::ModelMsInline for Locus {}
impl Validate for Locus {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

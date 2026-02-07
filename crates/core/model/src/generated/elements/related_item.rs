//!Element: `<relatedItem>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<relatedItem>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RelatedItemChild {
    #[serde(rename = "bibl")]
    Bibl(Box<crate::generated::elements::Bibl>),
    #[serde(rename = "biblStruct")]
    BiblStruct(Box<crate::generated::elements::BiblStruct>),
}
impl RelatedItemChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            RelatedItemChild::Bibl(elem) => {
                ctx.enter("bibl", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            RelatedItemChild::BiblStruct(elem) => {
                ctx.enter("biblStruct", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**related item - Contains or references another bibliographic item which is related to the
present one.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "relatedItem")]
pub struct RelatedItem {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
    ///Describes the relationship between the entity identified by therelatedItemelement and the resource described in the parent element,i.e.,bibl,sourceorrelatedItem.
    #[serde(rename = "@rel", skip_serializing_if = "Option::is_none")]
    pub rel: Option<crate::generated::data::DataModsrelationship>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<RelatedItemChild>,
}
impl crate::generated::model::ModelBiblPart for RelatedItem {}
impl Validate for RelatedItem {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

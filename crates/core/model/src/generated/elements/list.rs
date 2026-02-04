//!Element: `<list>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<list>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "li")]
    Li(Box<crate::generated::elements::Li>),
    #[serde(rename = "label")]
    Label(Box<crate::generated::elements::Label>),
}
impl ListChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ListChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ListChild::Li(elem) => {
                ctx.enter("li", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ListChild::Label(elem) => {
                ctx.enter("label", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**A formatting element that contains a series of items separated from one another and
arranged in a linear, often vertical, sequence.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "list")]
pub struct List {
    #[serde(flatten)]
    pub basic: crate::generated::att::AttBasic,
    #[serde(flatten)]
    pub classed: crate::generated::att::AttClassed,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub labelled: crate::generated::att::AttLabelled,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub linking: crate::generated::att::AttLinking,
    #[serde(flatten)]
    pub n_number_like: crate::generated::att::AttNNumberLike,
    #[serde(flatten)]
    pub responsibility: crate::generated::att::AttResponsibility,
    #[serde(flatten)]
    pub xy: crate::generated::att::AttXy,
    /**Used to indicate the format of a list. In asimplelist,lielements are not numbered or bulleted. In amarkedlist, the sequence of the list items
    is not critical, and a bullet, box, dash, or other character is displayed at the start of
    eachitem. In anorderedlist, the sequence of the items is
    important, and eachliis lettered or numbered. Style sheet
    functions should be used to specify the mark or numeration system for eachli.*/
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<String>,
    ///Captures the nature of the content of a list.
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ListChild>,
}
impl crate::generated::model::ModelListLike for List {}
impl Validate for List {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = None;
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

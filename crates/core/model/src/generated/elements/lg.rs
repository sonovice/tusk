//!Element: `<lg>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<lg>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LgChild {
    #[serde(rename = "l")]
    L(Box<crate::generated::elements::L>),
    #[serde(rename = "lg")]
    Lg(Box<crate::generated::elements::Lg>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
impl LgChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            LgChild::L(elem) => {
                ctx.enter("l", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LgChild::Lg(elem) => {
                ctx.enter("lg", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            LgChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**line group - May be used for any section of text that is organized as a group of lines;
      however, it is most often used for a group of verse lines functioning as a formal unit,e.g., a
      stanza, refrain, verse paragraph, etc.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "lg")]
pub struct Lg {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub lang: crate::generated::att::AttLang,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub xy: crate::generated::att::AttXy,
    #[serde(flatten)]
    pub lyrics_anl: crate::generated::att::AttLyricsAnl,
    #[serde(flatten)]
    pub lyrics_ges: crate::generated::att::AttLyricsGes,
    #[serde(flatten)]
    pub lyrics_log: crate::generated::att::AttLyricsLog,
    #[serde(flatten)]
    pub lyrics_vis: crate::generated::att::AttLyricsVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<LgChild>,
}
impl crate::generated::model::ModelLgLike for Lg {}
impl Validate for Lg {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

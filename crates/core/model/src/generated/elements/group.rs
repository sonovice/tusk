//!Element: `<group>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<group>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupChild {
    #[serde(rename = "group")]
    Group(Box<crate::generated::elements::Group>),
    #[serde(rename = "music")]
    Music(Box<crate::generated::elements::Music>),
}
impl GroupChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            GroupChild::Group(elem) => {
                ctx.enter("group", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            GroupChild::Music(elem) => {
                ctx.enter("music", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Contains a composite musical text, grouping together a sequence of distinct musical texts
(or groups of such musical texts) which are regarded as a unit for some purpose, for example,
the collected works of a composer.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "group")]
pub struct Group {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<GroupChild>,
}
impl Validate for Group {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

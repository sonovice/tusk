//!Element: `<appInfo>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<appInfo>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AppInfoChild {
    #[serde(rename = "application")]
    Application(Box<crate::generated::elements::Application>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
}
impl AppInfoChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            AppInfoChild::Application(elem) => {
                ctx.enter("application", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AppInfoChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**application information - Groups information about applications which have acted upon
the MEI file.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "appInfo")]
pub struct AppInfo {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<AppInfoChild>,
}
impl crate::generated::model::ModelEncodingPart for AppInfo {}
impl Validate for AppInfo {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

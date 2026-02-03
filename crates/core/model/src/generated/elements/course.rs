//!Element: `<course>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<course>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CourseChild {
    #[serde(rename = "string")]
    String(Box<crate::generated::elements::String>),
}
impl CourseChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            CourseChild::String(elem) => {
                ctx.enter("string", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///individual course tuning information - Describes the tuning of a course on a stringed instrument (e.g., guitar, lute).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "course")]
pub struct Course {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub course_log: crate::generated::att::AttCourseLog,
    #[serde(flatten)]
    pub course_vis: crate::generated::att::AttCourseVis,
    #[serde(flatten)]
    pub course_ges: crate::generated::att::AttCourseGes,
    #[serde(flatten)]
    pub course_anl: crate::generated::att::AttCourseAnl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<CourseChild>,
}
impl crate::generated::model::ModelTuningPart for Course {}
impl Validate for Course {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

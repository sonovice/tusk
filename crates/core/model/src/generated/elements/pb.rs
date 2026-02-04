//!Element: `<pb>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<pb>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PbChild {
    #[serde(rename = "pgFoot")]
    PgFoot(Box<crate::generated::elements::PgFoot>),
    #[serde(rename = "pgDesc")]
    PgDesc(Box<crate::generated::elements::PgDesc>),
    #[serde(rename = "pgHead")]
    PgHead(Box<crate::generated::elements::PgHead>),
}
impl PbChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PbChild::PgFoot(elem) => {
                ctx.enter("pgFoot", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PbChild::PgDesc(elem) => {
                ctx.enter("pgDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PbChild::PgHead(elem) => {
                ctx.enter("pgHead", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**page beginning - An empty formatting element that forces text to begin on a new
page.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "pb")]
pub struct Pb {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub source: crate::generated::att::AttSource,
    #[serde(flatten)]
    pub pb_anl: crate::generated::att::AttPbAnl,
    #[serde(flatten)]
    pub pb_ges: crate::generated::att::AttPbGes,
    #[serde(flatten)]
    pub pb_log: crate::generated::att::AttPbLog,
    #[serde(flatten)]
    pub pb_vis: crate::generated::att::AttPbVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PbChild>,
}
impl crate::generated::model::ModelPbLike for Pb {}
impl Validate for Pb {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

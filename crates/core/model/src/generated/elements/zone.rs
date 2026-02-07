//!Element: `<zone>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<zone>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ZoneChild {
    #[serde(rename = "figDesc")]
    FigDesc(Box<crate::generated::elements::FigDesc>),
    #[serde(rename = "graphic")]
    Graphic(Box<crate::generated::elements::Graphic>),
}
impl ZoneChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            ZoneChild::FigDesc(elem) => {
                ctx.enter("figDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            ZoneChild::Graphic(elem) => {
                ctx.enter("graphic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Defines an area of interest within asurfaceor graphic file.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "zone")]
pub struct Zone {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub coordinated: crate::generated::att::AttCoordinated,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<ZoneChild>,
}
impl Validate for Zone {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

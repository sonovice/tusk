//!Element: `<surface>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<surface>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SurfaceChild {
    #[serde(rename = "zone")]
    Zone(Box<crate::generated::elements::Zone>),
    #[serde(rename = "figDesc")]
    FigDesc(Box<crate::generated::elements::FigDesc>),
    #[serde(rename = "graphic")]
    Graphic(Box<crate::generated::elements::Graphic>),
}
impl SurfaceChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            SurfaceChild::Zone(elem) => {
                ctx.enter("zone", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SurfaceChild::FigDesc(elem) => {
                ctx.enter("figDesc", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            SurfaceChild::Graphic(elem) => {
                ctx.enter("graphic", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Defines a writing surface in terms of a rectangular coordinate space, optionally grouping
      one or more graphic representations of that space, and rectangular zones of interest within
      it.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "surface")]
pub struct Surface {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub coordinated: crate::generated::att::AttCoordinated,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub start_id: crate::generated::att::AttStartId,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<SurfaceChild>,
}
impl Validate for Surface {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

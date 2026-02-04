//!Element: `<bifolium>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<bifolium>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BifoliumChild {
    #[serde(rename = "restore")]
    Restore(Box<crate::generated::elements::Restore>),
    #[serde(rename = "damage")]
    Damage(Box<crate::generated::elements::Damage>),
    #[serde(rename = "bifolium")]
    Bifolium(Box<crate::generated::elements::Bifolium>),
    #[serde(rename = "patch")]
    Patch(Box<crate::generated::elements::Patch>),
    #[serde(rename = "del")]
    Del(Box<crate::generated::elements::Del>),
    #[serde(rename = "folium")]
    Folium(Box<crate::generated::elements::Folium>),
    #[serde(rename = "add")]
    Add(Box<crate::generated::elements::Add>),
    #[serde(rename = "gap")]
    Gap(Box<crate::generated::elements::Gap>),
    #[serde(rename = "cutout")]
    Cutout(Box<crate::generated::elements::Cutout>),
}
impl BifoliumChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            BifoliumChild::Restore(elem) => {
                ctx.enter("restore", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BifoliumChild::Damage(elem) => {
                ctx.enter("damage", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BifoliumChild::Bifolium(elem) => {
                ctx.enter("bifolium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BifoliumChild::Patch(elem) => {
                ctx.enter("patch", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BifoliumChild::Del(elem) => {
                ctx.enter("del", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BifoliumChild::Folium(elem) => {
                ctx.enter("folium", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BifoliumChild::Add(elem) => {
                ctx.enter("add", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BifoliumChild::Gap(elem) => {
                ctx.enter("gap", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            BifoliumChild::Cutout(elem) => {
                ctx.enter("cutout", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Describes a folded sheet of paper.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "bifolium")]
pub struct Bifolium {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub dimensions: crate::generated::att::AttDimensions,
    #[serde(flatten)]
    pub measurement: crate::generated::att::AttMeasurement,
    #[serde(flatten)]
    pub bifolium_surfaces: crate::generated::att::AttBifoliumSurfaces,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<BifoliumChild>,
}
impl crate::generated::model::ModelBifoliumLike for Bifolium {}
impl Validate for Bifolium {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

//!Element: `<ambitus>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<ambitus>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AmbitusChild {
    #[serde(rename = "ambNote")]
    AmbNote(Box<crate::generated::elements::AmbNote>),
}
impl AmbitusChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            AmbitusChild::AmbNote(elem) => {
                ctx.enter("ambNote", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///Range of a voice, instrument or piece.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "ambitus")]
pub struct Ambitus {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub ambitus_anl: crate::generated::att::AttAmbitusAnl,
    #[serde(flatten)]
    pub ambitus_ges: crate::generated::att::AttAmbitusGes,
    #[serde(flatten)]
    pub ambitus_log: crate::generated::att::AttAmbitusLog,
    #[serde(flatten)]
    pub ambitus_vis: crate::generated::att::AttAmbitusVis,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<AmbitusChild>,
}
impl Validate for Ambitus {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

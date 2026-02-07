//!Element: `<staffGrp>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<staffGrp>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StaffGrpChild {
    #[serde(rename = "grpSym")]
    GrpSym(Box<crate::generated::elements::GrpSym>),
    #[serde(rename = "staffDef")]
    StaffDef(Box<crate::generated::elements::StaffDef>),
    #[serde(rename = "staffGrp")]
    StaffGrp(Box<crate::generated::elements::StaffGrp>),
    #[serde(rename = "labelAbbr")]
    LabelAbbr(Box<crate::generated::elements::LabelAbbr>),
    #[serde(rename = "label")]
    Label(Box<crate::generated::elements::Label>),
    #[serde(rename = "instrDef")]
    InstrDef(Box<crate::generated::elements::InstrDef>),
}
impl StaffGrpChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            StaffGrpChild::GrpSym(elem) => {
                ctx.enter("grpSym", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffGrpChild::StaffDef(elem) => {
                ctx.enter("staffDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffGrpChild::StaffGrp(elem) => {
                ctx.enter("staffGrp", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffGrpChild::LabelAbbr(elem) => {
                ctx.enter("labelAbbr", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffGrpChild::Label(elem) => {
                ctx.enter("label", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            StaffGrpChild::InstrDef(elem) => {
                ctx.enter("instrDef", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
///staff group - A group of bracketed or braced staves.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "staffGrp")]
pub struct StaffGrp {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub facsimile: crate::generated::att::AttFacsimile,
    #[serde(flatten)]
    pub metadata_pointing: crate::generated::att::AttMetadataPointing,
    #[serde(flatten)]
    pub staff_grp_anl: crate::generated::att::AttStaffGrpAnl,
    #[serde(flatten)]
    pub staff_grp_ges: crate::generated::att::AttStaffGrpGes,
    #[serde(flatten)]
    pub staff_grp_log: crate::generated::att::AttStaffGrpLog,
    #[serde(flatten)]
    pub staff_grp_vis: crate::generated::att::AttStaffGrpVis,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<StaffGrpChild>,
}
impl crate::generated::model::ModelStaffGrpLike for StaffGrp {}
impl Validate for StaffGrp {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

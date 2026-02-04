//!Element: `<availability>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Child content for `<availability>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AvailabilityChild {
    /// Text content.
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "sysReq")]
    SysReq(Box<crate::generated::elements::SysReq>),
    #[serde(rename = "distributor")]
    Distributor(Box<crate::generated::elements::Distributor>),
    #[serde(rename = "price")]
    Price(Box<crate::generated::elements::Price>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "accessRestrict")]
    AccessRestrict(Box<crate::generated::elements::AccessRestrict>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "useRestrict")]
    UseRestrict(Box<crate::generated::elements::UseRestrict>),
}
impl AvailabilityChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            AvailabilityChild::Text(_) => {}
            AvailabilityChild::SysReq(elem) => {
                ctx.enter("sysReq", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AvailabilityChild::Distributor(elem) => {
                ctx.enter("distributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AvailabilityChild::Price(elem) => {
                ctx.enter("price", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AvailabilityChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AvailabilityChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AvailabilityChild::AccessRestrict(elem) => {
                ctx.enter("accessRestrict", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AvailabilityChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AvailabilityChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            AvailabilityChild::UseRestrict(elem) => {
                ctx.enter("useRestrict", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Groups elements that describe the availability of and access to a bibliographic item,
including an MEI-encoded document.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "availability")]
pub struct Availability {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<AvailabilityChild>,
}
impl crate::generated::model::ModelPubStmtPart for Availability {}
impl crate::generated::model::ModelImprintPart for Availability {}
impl Validate for Availability {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

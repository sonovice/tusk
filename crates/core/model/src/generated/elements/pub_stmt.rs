//!Element: `<pubStmt>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<pubStmt>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PubStmtChild {
    #[serde(rename = "head")]
    Head(Box<crate::generated::elements::Head>),
    #[serde(rename = "respStmt")]
    RespStmt(Box<crate::generated::elements::RespStmt>),
    #[serde(rename = "unpub")]
    Unpub(Box<crate::generated::elements::Unpub>),
    #[serde(rename = "address")]
    Address(Box<crate::generated::elements::Address>),
    #[serde(rename = "pubPlace")]
    PubPlace(Box<crate::generated::elements::PubPlace>),
    #[serde(rename = "publisher")]
    Publisher(Box<crate::generated::elements::Publisher>),
    #[serde(rename = "identifier")]
    Identifier(Box<crate::generated::elements::Identifier>),
    #[serde(rename = "availability")]
    Availability(Box<crate::generated::elements::Availability>),
    #[serde(rename = "date")]
    Date(Box<crate::generated::elements::Date>),
    #[serde(rename = "distributor")]
    Distributor(Box<crate::generated::elements::Distributor>),
}
impl PubStmtChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            PubStmtChild::Head(elem) => {
                ctx.enter("head", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::RespStmt(elem) => {
                ctx.enter("respStmt", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::Unpub(elem) => {
                ctx.enter("unpub", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::Address(elem) => {
                ctx.enter("address", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::PubPlace(elem) => {
                ctx.enter("pubPlace", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::Publisher(elem) => {
                ctx.enter("publisher", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::Identifier(elem) => {
                ctx.enter("identifier", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::Availability(elem) => {
                ctx.enter("availability", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::Date(elem) => {
                ctx.enter("date", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
            PubStmtChild::Distributor(elem) => {
                ctx.enter("distributor", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**publication statement - Container for information regarding the publication or
      distribution of a bibliographic item, including the publisher’s name and address, the date of
      publication, and other relevant details.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "pubStmt")]
pub struct PubStmt {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<PubStmtChild>,
}
impl Validate for PubStmt {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

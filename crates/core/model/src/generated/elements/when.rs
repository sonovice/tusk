//!Element: `<when>`
use serde::{Deserialize, Serialize};
use crate::generated::validation::{ValidationContext, Validate};
///Child content for `<when>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WhenChild {
    #[serde(rename = "extData")]
    ExtData(Box<crate::generated::elements::ExtData>),
}
impl WhenChild {
    /// Validate this child element.
    pub fn validate_with_context(&self, ctx: &mut ValidationContext, index: usize) {
        match self {
            WhenChild::ExtData(elem) => {
                ctx.enter("extData", index);
                elem.validate_with_context(ctx);
                ctx.exit();
            }
        }
    }
}
/**Indicates a point in time either absolutely (using the absolute attribute), or relative to
      another when element (using the since, interval and inttype attributes).*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "when")]
pub struct When {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub data_pointing: crate::generated::att::AttDataPointing,
    ///Provides an absolute value for the time point.
    #[serde(rename = "@absolute", skip_serializing_if = "Option::is_none")]
    pub absolute: Option<String>,
    /**Specifies the time interval between this time point and the one designated by the
          since attribute. This attribute can only be interpreted meaningfully in conjunction with
          the inttype attribute.*/
    #[serde(rename = "@interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    ///Specifies the kind of values used in the absolute attribute.
    #[serde(rename = "@abstype", skip_serializing_if = "Option::is_none")]
    pub abstype: Option<crate::generated::data::DataBetype>,
    ///Specifies the kind of values used in the interval attribute.
    #[serde(rename = "@inttype", skip_serializing_if = "Option::is_none")]
    pub inttype: Option<crate::generated::data::DataBetype>,
    /**Identifies the reference point for determining the time of the current when element,
          which is obtained by adding the interval to the time of the reference point. The value
          should be the ID of another when element within the same parent element. If the since
          attribute is omitted and the absolute attribute is not specified, then the reference point
          is understood to be the immediately preceding when element.*/
    #[serde(rename = "@since", skip_serializing_if = "Option::is_none")]
    pub since: Option<crate::generated::data::DataUri>,
    /// Child elements.
    #[serde(default, rename = "$value")]
    pub children: Vec<WhenChild>,
}
impl Validate for When {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
        for (i, child) in self.children.iter().enumerate() {
            child.validate_with_context(ctx, i);
        }
    }
}

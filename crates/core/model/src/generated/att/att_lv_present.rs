//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that indicate the presence of an l.v. (laissez vibrer) marking attached to a
      feature. If visual information about the lv sign needs to be recorded, then anlvelement should be employed.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLvPresent {
    ///Indicates the attachment of an l.v. (laissez vibrer) sign to this element.
    #[serde(rename = "@lv", skip_serializing_if = "Option::is_none")]
    pub lv: Option<crate::generated::data::DataBoolean>,
}

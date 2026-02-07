//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that indicate whether to render a repeat symbol or the source material to which
      it refers.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttExpandable {
    /**Indicates whether to render a repeat symbol or the source material to which it refers.
          A value of 'true' renders the source material, while 'false' displays the repeat
          symbol.*/
    #[serde(rename = "@expand", skip_serializing_if = "Option::is_none")]
    pub expand: Option<crate::generated::data::DataBoolean>,
}

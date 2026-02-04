//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record renditional characteristics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTextRendition {
    ///Used to extend the values of the rend attribute.
    #[serde(rename = "@altrend", default, skip_serializing_if = "Vec::is_empty")]
    pub altrend: Vec<String>,
    /**Captures the appearance of the element’s contents using MEI-defined
          descriptors.*/
    #[serde(rename = "@rend", default, skip_serializing_if = "Vec::is_empty")]
    pub rend: Vec<crate::generated::data::DataTextrendition>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes which can be used to classify features.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTyped {
    /**Contains one or more URIs which denote classification terms that apply to the entity
          bearing this attribute.*/
    #[serde(rename = "@class", default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<crate::generated::data::DataUri>,
    /**Designation which characterizes the element in some sense, using any convenient
          classification scheme or typology that employs single-token labels.*/
    #[serde(rename = "@type", default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<String>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that form the basis of the att.common class.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBasic {
    /**Regularizes the naming of an element and thus facilitates building links between it
    and other resources. Each id attribute within a document must have a unique value.*/
    #[serde(rename = "xml:id", skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,
    /**Provides a base URI reference with which applications can resolve relative URI
    references into absolute URI references.*/
    #[serde(rename = "xml:base", skip_serializing_if = "Option::is_none")]
    pub xml_base: Option<crate::generated::data::DataUri>,
}

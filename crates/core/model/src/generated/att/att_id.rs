//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that uniquely identify an element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttId {
    /**Regularizes the naming of an element and thus facilitates building links between it
          and other resources. Each id attribute within a document must have a unique value.*/
    #[serde(rename = "xml:id", skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,
}

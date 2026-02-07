//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that provide for classification of notation.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNotationType {
    /**Contains classification of the notation contained or described by the element bearing
          this attribute.*/
    #[serde(rename = "@notationtype", skip_serializing_if = "Option::is_none")]
    pub notationtype: Option<crate::generated::data::DataNotationtype>,
    /**Provides any sub-classification of the notation contained or described by the element,
          additional to that given by its notationtype attribute.*/
    #[serde(rename = "@notationsubtype", skip_serializing_if = "Option::is_none")]
    pub notationsubtype: Option<String>,
}

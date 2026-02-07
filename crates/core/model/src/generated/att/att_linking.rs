//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that specify element-to-element relationships.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLinking {
    ///Points to an element of which the current element is a copy.
    #[serde(rename = "@copyof", skip_serializing_if = "Option::is_none")]
    pub copyof: Option<crate::generated::data::DataUri>,
    /**Used to point to other elements that correspond to this one in a generic
          fashion.*/
    #[serde(rename = "@corresp", default, skip_serializing_if = "Vec::is_empty")]
    pub corresp: Vec<crate::generated::data::DataUri>,
    /**points to one or more events in a user-defined collection that are known to be
          predecessors of the current element.*/
    #[serde(rename = "@follows", default, skip_serializing_if = "Vec::is_empty")]
    pub follows: Vec<crate::generated::data::DataUri>,
    ///Used to point to the next event(s) in a user-defined collection.
    #[serde(rename = "@next", default, skip_serializing_if = "Vec::is_empty")]
    pub next: Vec<crate::generated::data::DataUri>,
    /**Points to one or more events in a user-defined collection that are known to be
          successors of the current element.*/
    #[serde(rename = "@precedes", default, skip_serializing_if = "Vec::is_empty")]
    pub precedes: Vec<crate::generated::data::DataUri>,
    ///Points to the previous event(s) in a user-defined collection.
    #[serde(rename = "@prev", default, skip_serializing_if = "Vec::is_empty")]
    pub prev: Vec<crate::generated::data::DataUri>,
    /**Points to an element that is the same as the current element but is not a literal copy
          of the current element.*/
    #[serde(rename = "@sameas", default, skip_serializing_if = "Vec::is_empty")]
    pub sameas: Vec<crate::generated::data::DataUri>,
    ///Points to elements that are synchronous with the current element.
    #[serde(rename = "@synch", default, skip_serializing_if = "Vec::is_empty")]
    pub synch: Vec<crate::generated::data::DataUri>,
}

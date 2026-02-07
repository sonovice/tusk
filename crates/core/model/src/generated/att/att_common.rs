//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes common to many elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCommon {
    /**Regularizes the naming of an element and thus facilitates building links between it
          and other resources. Each id attribute within a document must have a unique value.*/
    #[serde(rename = "xml:id", skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<String>,
    /**Provides a base URI reference with which applications can resolve relative URI
          references into absolute URI references.*/
    #[serde(rename = "xml:base", skip_serializing_if = "Option::is_none")]
    pub xml_base: Option<crate::generated::data::DataUri>,
    /**Captures text to be used to generate a label for the element to which it’s attached, a
          "tool tip" or prefatory text, for example. Should not be used to record document
          content.*/
    #[serde(rename = "@label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
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
    /**Provides a number-like designation that indicates an element’s position in a sequence
          of similar elements. May not contain space characters.*/
    #[serde(rename = "@n", skip_serializing_if = "Option::is_none")]
    pub n: Option<crate::generated::data::DataWord>,
    /**Indicates the agent(s) responsible for some aspect of the text’s transcription,
          editing, or encoding. Its value must point to one or more identifiers declared in the
          document header.*/
    #[serde(rename = "@resp", default, skip_serializing_if = "Vec::is_empty")]
    pub resp: Vec<crate::generated::data::DataUri>,
    /**Contains one or more URIs which denote classification terms that apply to the entity
          bearing this attribute.*/
    #[serde(rename = "@class", default, skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<crate::generated::data::DataUri>,
    /**Designation which characterizes the element in some sense, using any convenient
          classification scheme or typology that employs single-token labels.*/
    #[serde(rename = "@type", default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<String>,
}

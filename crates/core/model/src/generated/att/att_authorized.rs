//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe the source of a controlled value.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAuthorized {
    /**A value that represents or identifies other data. Often, it is a primary key in the
          database or a unique value in the coded list identified by theauthorauth.uriattributes.*/
    #[serde(rename = "@codedval", default, skip_serializing_if = "Vec::is_empty")]
    pub codedval: Vec<String>,
    /**A name or label associated with a controlled vocabulary or other authoritative source
          for this element or its content.*/
    #[serde(rename = "@auth", skip_serializing_if = "Option::is_none")]
    pub auth: Option<String>,
    /**A web-accessible location of the controlled vocabulary or other authoritative source
          of identification or definition for this element or its content. This attribute may
          contain a complete URI or a partial URI which is completed by the value of the codedval
          attribute.*/
    #[serde(rename = "@auth.uri", skip_serializing_if = "Option::is_none")]
    pub auth_uri: Option<crate::generated::data::DataUri>,
}

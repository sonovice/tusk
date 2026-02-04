//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes shared by names.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttName {
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
    ///Contains the end point of a date range in standard ISO form.
    #[serde(rename = "@enddate", skip_serializing_if = "Option::is_none")]
    pub enddate: Option<crate::generated::data::DataIsodate>,
    ///Provides the value of a textual date in standard ISO form.
    #[serde(rename = "@isodate", skip_serializing_if = "Option::is_none")]
    pub isodate: Option<crate::generated::data::DataIsodate>,
    ///Contains an upper boundary for an uncertain date in standard ISO form.
    #[serde(rename = "@notafter", skip_serializing_if = "Option::is_none")]
    pub notafter: Option<crate::generated::data::DataIsodate>,
    ///Contains a lower boundary, in standard ISO form, for an uncertain date.
    #[serde(rename = "@notbefore", skip_serializing_if = "Option::is_none")]
    pub notbefore: Option<crate::generated::data::DataIsodate>,
    ///Contains the starting point of a date range in standard ISO form.
    #[serde(rename = "@startdate", skip_serializing_if = "Option::is_none")]
    pub startdate: Option<crate::generated::data::DataIsodate>,
    /**Holds the number of initial characters (such as those constituting an article or
    preposition) that should not be used for sorting a title or name.*/
    #[serde(rename = "@nonfiling", skip_serializing_if = "Option::is_none")]
    pub nonfiling: Option<u64>,
    /**Used to record a pointer to the regularized form of the name elsewhere in the
    document.*/
    #[serde(rename = "@nymref", skip_serializing_if = "Option::is_none")]
    pub nymref: Option<crate::generated::data::DataUri>,
    /**Used to specify further information about the entity referenced by this name, for
    example, the occupation of a person or the status of a place.*/
    #[serde(rename = "@role", default, skip_serializing_if = "Vec::is_empty")]
    pub role: Vec<crate::generated::data::DataRelators>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttPointingXlinkActuate {
    ///Load the target resource(s) immediately.
    #[serde(rename = "onLoad")]
    OnLoad,
    ///Load the target resource(s) upon user request.
    #[serde(rename = "onRequest")]
    OnRequest,
    ///Do not permit loading of the target resource(s).
    #[serde(rename = "none")]
    None,
    ///Behavior other than allowed by the other values of this attribute.
    #[serde(rename = "other")]
    Other,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttPointingXlinkShow {
    ///Open in a new window.
    #[serde(rename = "new")]
    New,
    ///Load the referenced resource in the same window.
    #[serde(rename = "replace")]
    Replace,
    ///Embed the referenced resource at the point of the link.
    #[serde(rename = "embed")]
    Embed,
    ///Do not permit traversal to the referenced resource.
    #[serde(rename = "none")]
    None,
    ///Behavior other than permitted by the other values of this attribute.
    #[serde(rename = "other")]
    Other,
}
///Attributes common to all pointing/linking elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPointing {
    ///Defines whether a link occurs automatically or must be requested by the user.
    #[serde(rename = "@xlink:actuate", skip_serializing_if = "Option::is_none")]
    pub xlink_actuate: Option<AttPointingXlinkActuate>,
    /**Characterization of the relationship between resources. The value of the role
    attribute must be a URI.*/
    #[serde(rename = "@xlink:role", skip_serializing_if = "Option::is_none")]
    pub xlink_role: Option<crate::generated::data::DataUri>,
    ///Defines how a remote resource is rendered.
    #[serde(rename = "@xlink:show", skip_serializing_if = "Option::is_none")]
    pub xlink_show: Option<AttPointingXlinkShow>,
    /**Identifies passive participants in a relationship; that is, the entities pointed
    "to".*/
    #[serde(rename = "@target", default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<crate::generated::data::DataUri>,
    /**Characterization of target resource(s) using any convenient classification scheme or
    typology.*/
    #[serde(rename = "@targettype", skip_serializing_if = "Option::is_none")]
    pub targettype: Option<String>,
}

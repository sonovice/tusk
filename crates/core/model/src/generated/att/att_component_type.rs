//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttComponentTypeComptype {
    ///A physical and logical part of entity.
    #[serde(rename = "constituent")]
    Constituent,
    /**A physical, but not logical component of the entity, usually included as part of
    the binding process.*/
    #[serde(rename = "boundwith")]
    Boundwith,
    ///A logical component of the entity physically held elsewhere.
    #[serde(rename = "separated")]
    Separated,
}
///Attributes that express the relationship between a component and its host.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttComponentType {
    ///
    #[serde(rename = "@comptype", skip_serializing_if = "Option::is_none")]
    pub comptype: Option<AttComponentTypeComptype>,
}

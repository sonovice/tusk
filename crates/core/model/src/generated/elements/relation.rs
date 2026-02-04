//!Element: `<relation>`
use crate::generated::validation::{Validate, ValidationContext};
use serde::{Deserialize, Serialize};
///Describes a relationship or linkage amongst entities.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename = "relation")]
pub struct Relation {
    #[serde(flatten)]
    pub common: crate::generated::att::AttCommon,
    #[serde(flatten)]
    pub authorized: crate::generated::att::AttAuthorized,
    #[serde(flatten)]
    pub bibl: crate::generated::att::AttBibl,
    #[serde(flatten)]
    pub datable: crate::generated::att::AttDatable,
    #[serde(flatten)]
    pub evidence: crate::generated::att::AttEvidence,
    #[serde(flatten)]
    pub plist: crate::generated::att::AttPlist,
    #[serde(flatten)]
    pub pointing: crate::generated::att::AttPointing,
    #[serde(flatten)]
    pub target_eval: crate::generated::att::AttTargetEval,
    /**Describes the relationship between the entities identified by the plist and target
    attributes.*/
    #[serde(rename = "@rel", skip_serializing_if = "Option::is_none")]
    pub rel: Option<crate::generated::data::DataRelationship>,
}
impl crate::generated::model::ModelRelationLike for Relation {}
impl Validate for Relation {
    fn validate_with_context(&self, ctx: &mut ValidationContext) {
        let _xml_id: Option<&str> = self.common.xml_id.as_deref();
    }
}

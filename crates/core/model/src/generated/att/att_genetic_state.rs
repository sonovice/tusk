//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that pertain to a genetic state.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttGeneticState {
    /**The @instant attribute is syntactic sugar for classifying a scribal intervention as an
    ad-hoc modification; that is, one which does not interrupt the writing process.*/
    #[serde(rename = "@instant", skip_serializing_if = "Option::is_none")]
    pub instant: Option<crate::generated::data::DataBoolean>,
    ///Points to the genetic state that results from this modification.
    #[serde(rename = "@state", default, skip_serializing_if = "Vec::is_empty")]
    pub state: Vec<crate::generated::data::DataUri>,
}

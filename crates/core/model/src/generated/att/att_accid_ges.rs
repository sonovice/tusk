//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttAccidGes {
    ///Records the performed pitch inflection.
    #[serde(rename = "@accid.ges", skip_serializing_if = "Option::is_none")]
    pub accid_ges: Option<crate::generated::data::DataAccidentalGestural>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes in the CMN repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttChordLogCmn {
    /**Marks a note or chord as a "grace" (without a definite performed duration) and records
    from which other note/chord it should "steal" time.*/
    #[serde(rename = "@grace", skip_serializing_if = "Option::is_none")]
    pub grace: Option<crate::generated::data::DataGrace>,
    ///Records the amount of time to be "stolen" from a non-grace note/chord.
    #[serde(rename = "@grace.time", skip_serializing_if = "Option::is_none")]
    pub grace_time: Option<crate::generated::data::DataPercent>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that indicate programmatic numbering.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMultinumMeasures {
    /**Indicates whether programmatically calculated counts of multiple measures of rest
          (mRest) and whole measure repeats (mRpt) in parts should be rendered.*/
    #[serde(rename = "@multi.number", skip_serializing_if = "Option::is_none")]
    pub multi_number: Option<crate::generated::data::DataBoolean>,
}

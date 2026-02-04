//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Attributes that provide information about a measure’s conformance to the prevailing
      meter.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeterConformanceBar {
    /**Indicates the relationship between the content of a measure and the prevailing
          meter.*/
    #[serde(rename = "@metcon", skip_serializing_if = "Option::is_none")]
    pub metcon: Option<crate::generated::data::DataBoolean>,
    /**Indicates whether or not a bar line is "controlling"; that is, if it indicates a point
          of alignment across all the parts. Bar lines within a score are usually controlling; that
          is, they "line up". Bar lines within parts may or may not be controlling. When applied tomeasure, this attribute indicates the nature of the right bar line
          but not the left.*/
    #[serde(rename = "@control", skip_serializing_if = "Option::is_none")]
    pub control: Option<crate::generated::data::DataBoolean>,
}

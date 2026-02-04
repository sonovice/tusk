//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeasureVis {
    /**States the length of bar lines in virtual units. The value must be greater than 0 and
          is typically equal to 2 times (the number of staff lines - 1);e.g., a value of8for a
          5-line staff.*/
    #[serde(rename = "@bar.len", skip_serializing_if = "Option::is_none")]
    pub bar_len: Option<f64>,
    ///Records the method of barring.
    #[serde(rename = "@bar.method", skip_serializing_if = "Option::is_none")]
    pub bar_method: Option<crate::generated::data::DataBarmethod>,
    /**Denotes the staff location of bar lines, if the length is non-standard; that is, not
          equal to 2 times (the number of staff lines - 1).*/
    #[serde(rename = "@bar.place", skip_serializing_if = "Option::is_none")]
    pub bar_place: Option<crate::generated::data::DataStaffloc>,
    ///Measurement of the horizontal dimension of an entity.
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<crate::generated::data::DataMeasurementunsigned>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that indicate the calendar system of a date or other datable element.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCalendared {
    /**Indicates the calendar system to which a date belongs, for example, Gregorian, Julian,
          Roman, Mosaic, Revolutionary, Islamic, etc.*/
    #[serde(rename = "@calendar", skip_serializing_if = "Option::is_none")]
    pub calendar: Option<String>,
}

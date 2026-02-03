//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttStaffGrpVisSymbol {
    ///Curved symbol,i.e., {.
    #[serde(rename = "brace")]
    Brace,
    ///Square symbol,i.e., [, but with curved/angled top and bottom segments.
    #[serde(rename = "bracket")]
    Bracket,
    ///Square symbol,i.e., [, with horizontal top and bottom segments.
    #[serde(rename = "bracketsq")]
    Bracketsq,
    /**Line symbol,i.e., |, (wide) line without top and bottom curved/horizontal
    segments.*/
    #[serde(rename = "line")]
    Line,
    ///Grouping symbol missing.
    #[serde(rename = "none")]
    None,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffGrpVis {
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
    ///Specifies the symbol used to group a set of staves.
    #[serde(rename = "@symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<AttStaffGrpVisSymbol>,
    /**Indicates if a feature should be rendered when the notation is presented graphically
    or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
    /**Indicates whether bar lines go across the space between staves (true) or are only
    drawn across the lines of each staff (false).*/
    #[serde(rename = "@bar.thru", skip_serializing_if = "Option::is_none")]
    pub bar_thru: Option<crate::generated::data::DataBoolean>,
}

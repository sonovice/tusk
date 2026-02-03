//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttTupletSpanVisNumFormat {
    ///Only the num attribute is displayed,e.g., '7'.
    #[serde(rename = "count")]
    Count,
    ///Both the num and numbase attributes are displayed,e.g., '7:4'.
    #[serde(rename = "ratio")]
    Ratio,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTupletSpanVis {
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
    as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
    ///States where the number will be placed in relation to the notational feature.
    #[serde(rename = "@num.place", skip_serializing_if = "Option::is_none")]
    pub num_place: Option<crate::generated::data::DataStaffrelBasic>,
    ///Determines if the number is visible.
    #[serde(rename = "@num.visible", skip_serializing_if = "Option::is_none")]
    pub num_visible: Option<crate::generated::data::DataBoolean>,
    /**Used to state where a tuplet bracket will be placed in relation to the note
    heads.*/
    #[serde(rename = "@bracket.place", skip_serializing_if = "Option::is_none")]
    pub bracket_place: Option<crate::generated::data::DataStaffrelBasic>,
    ///States whether a bracket should be rendered with a tuplet.
    #[serde(rename = "@bracket.visible", skip_serializing_if = "Option::is_none")]
    pub bracket_visible: Option<crate::generated::data::DataBoolean>,
    ///Controls how the num:numbase ratio is to be displayed.
    #[serde(rename = "@num.format", skip_serializing_if = "Option::is_none")]
    pub num_format: Option<AttTupletSpanVisNumFormat>,
}

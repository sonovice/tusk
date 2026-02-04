//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMetaMarkVis {
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
    as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
    /**Captures the placement of the item with respect to the staff with which it is
    associated.*/
    #[serde(rename = "@place", skip_serializing_if = "Option::is_none")]
    pub place: Option<crate::generated::data::DataStaffrel>,
    ///Provides a label for members of a vertically aligned group.
    #[serde(rename = "@vgrp", skip_serializing_if = "Option::is_none")]
    pub vgrp: Option<u64>,
}

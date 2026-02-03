//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record grouping of vertically aligned elements.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttVerticalGroup {
    ///Provides a label for members of a vertically aligned group.
    #[serde(rename = "@vgrp", skip_serializing_if = "Option::is_none")]
    pub vgrp: Option<u64>,
}

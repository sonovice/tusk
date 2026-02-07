//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttChordDefLog {
    ///Records fret position.
    #[serde(rename = "@tab.pos", skip_serializing_if = "Option::is_none")]
    pub tab_pos: Option<u64>,
    /**This attribute is deprecated in favor of the newtuningelement and will be removed in a future version. Provides a *written* pitch and octave for each open string or course of
          strings.*/
    #[serde(rename = "@tab.strings", skip_serializing_if = "Option::is_none")]
    pub tab_strings: Option<crate::generated::SpaceSeparated<String>>,
    ///This attribute is deprecated in favor of the newtuningelement and will be removed in a future version. Provides a *written* pitch and octave for each open string or course of strings.
    #[serde(rename = "@tab.courses", skip_serializing_if = "Option::is_none")]
    pub tab_courses: Option<crate::generated::SpaceSeparated<String>>,
}

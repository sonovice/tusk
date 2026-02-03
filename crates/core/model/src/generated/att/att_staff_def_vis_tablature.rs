//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes for staffDef in the tablature repertoire.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStaffDefVisTablature {
    ///Attribute that describes the vertical alignment of tablature symbols. Only applicable in cases where the symbols' vertical position does not communicate other information, such as courses (i.e., only in German lute tablature). Typical values aretopandbottom.
    #[serde(rename = "@tab.align", skip_serializing_if = "Option::is_none")]
    pub tab_align: Option<crate::generated::data::DataVerticalalignment>,
    ///Used in German lute tablature where the vertical alignment of tab notes is consistent but cannot be identified using a typical value oftab.align(i.e.,toporbottom). Specifies the horizontal strand corresponding to thelinesattribute onstaffDefthat anchors the vertical position of tab notes. This anchorline is used as the vertical starting position when stacking tab notes into chords. Single tab notes simply occupy this position. Chordsgrow upwardsfrom this position. If the chord extends further than the number of available horizontal strands (lines) above the anchorline, the entire chord is shifted downward until its top tab note is positioned on the top-most line. (Note that in German lute tablature, the lines are conceptual rather than visible).
    #[serde(rename = "@tab.anchorline", skip_serializing_if = "Option::is_none")]
    pub tab_anchorline: Option<crate::generated::data::DataClefline>,
}

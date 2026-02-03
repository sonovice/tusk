//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///String tablature string and fret information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStringtab {
    ///This attribute is deprecated and will be removed in a future version. Indicates which finger, if any, should be used to play an individual string. The index, middle, ring, and little fingers are represented by the values 1-4, whiletis for the thumb. The valuesxandoindicate muffled and open strings, respectively.
    #[serde(rename = "@tab.fing", skip_serializing_if = "Option::is_none")]
    pub tab_fing: Option<crate::generated::data::DataFingerFret>,
    ///Records the location at which a string should be stopped against a fret.
    #[serde(rename = "@tab.fret", skip_serializing_if = "Option::is_none")]
    pub tab_fret: Option<crate::generated::data::DataFretnumber>,
    ///Used in German lute tablature in cases where vertical positioning deviates from the norm which can be specified bytab.align. Indicates the position of the tab note on one of the horizontal strands corresponding to thelinesattribute onstaffDef. (Note that in this case, the lines are conceptual rather than visible).
    #[serde(rename = "@tab.line", skip_serializing_if = "Option::is_none")]
    pub tab_line: Option<crate::generated::data::DataClefline>,
    ///This attribute is deprecated in favor oftab.courseand will be removed in a future version. Records which string is to be played.
    #[serde(rename = "@tab.string", skip_serializing_if = "Option::is_none")]
    pub tab_string: Option<crate::generated::data::DataStringnumber>,
    ///Records which course is to be played.
    #[serde(rename = "@tab.course", skip_serializing_if = "Option::is_none")]
    pub tab_course: Option<crate::generated::data::DataCoursenumber>,
}

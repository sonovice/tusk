//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttRehLog {
    /**Indicates the point of occurrence of this feature along a time line. Its value must be
    the ID of awhenelement elsewhere in the document.*/
    #[serde(rename = "@when", skip_serializing_if = "Option::is_none")]
    pub when: Option<crate::generated::data::DataUri>,
    /**Indicates the part in which the current feature should appear. Use '%all' when the
    feature should occur in every part.*/
    #[serde(rename = "@part", default, skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<String>,
    /**Signifies the part staff on which a notated feature occurs. Use '%all' when the
    feature should occur on every staff.*/
    #[serde(rename = "@partstaff", default, skip_serializing_if = "Vec::is_empty")]
    pub partstaff: Vec<String>,
    /**Signifies the staff on which a notated event occurs or to which a control event
    applies. Mandatory when applicable.*/
    #[serde(rename = "@staff", default, skip_serializing_if = "Vec::is_empty")]
    pub staff: Vec<u64>,
    /**Holds a reference to the first element in a sequence of events to which the feature
    applies.*/
    #[serde(rename = "@startid", skip_serializing_if = "Option::is_none")]
    pub startid: Option<crate::generated::data::DataUri>,
    /**Encodes the onset time in terms of musical time,i.e., beats[.fractional beat part],
    as expressed in the written time signature.*/
    #[serde(rename = "@tstamp", skip_serializing_if = "Option::is_none")]
    pub tstamp: Option<crate::generated::data::DataBeat>,
    /**Encodes the onset time in terms of musical time,i.e., beats[.fractional beat part],
    as expressed in the written time signature.*/
    #[serde(rename = "@tstamp.ges", skip_serializing_if = "Option::is_none")]
    pub tstamp_ges: Option<crate::generated::data::DataBeat>,
    ///Records the onset time in terms of ISO time.
    #[serde(rename = "@tstamp.real", skip_serializing_if = "Option::is_none")]
    pub tstamp_real: Option<crate::generated::data::DataIsotime>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttGraceGrpLogAttach {
    ///Attached to the preceding event.
    #[serde(rename = "pre")]
    Pre,
    ///Attached to the following event.
    #[serde(rename = "post")]
    Post,
    ///Attachment is ambiguous.
    #[serde(rename = "unknown")]
    Unknown,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttGraceGrpLog {
    /**Indicates the point of occurrence of this feature along a time line. Its value must be
          the ID of awhenelement elsewhere in the document.*/
    #[serde(rename = "@when", skip_serializing_if = "Option::is_none")]
    pub when: Option<crate::generated::data::DataUri>,
    ///Identifies the layer to which a feature applies.
    #[serde(rename = "@layer", default, skip_serializing_if = "Vec::is_empty")]
    pub layer: Vec<u64>,
    /**Signifies the staff on which a notated event occurs or to which a control event
          applies. Mandatory when applicable.*/
    #[serde(rename = "@staff", default, skip_serializing_if = "Vec::is_empty")]
    pub staff: Vec<u64>,
    /**Encodes the onset time in terms of musical time,i.e., beats[.fractional beat part],
          as expressed in the written time signature.*/
    #[serde(rename = "@tstamp.ges", skip_serializing_if = "Option::is_none")]
    pub tstamp_ges: Option<crate::generated::data::DataBeat>,
    ///Records the onset time in terms of ISO time.
    #[serde(rename = "@tstamp.real", skip_serializing_if = "Option::is_none")]
    pub tstamp_real: Option<crate::generated::data::DataIsotime>,
    /**Encodes the onset time in terms of musical time,i.e., beats[.fractional beat part],
          as expressed in the written time signature.*/
    #[serde(rename = "@tstamp", skip_serializing_if = "Option::is_none")]
    pub tstamp: Option<crate::generated::data::DataBeat>,
    /**Marks a note or chord as a "grace" (without a definite performed duration) and records
          from which other note/chord it should "steal" time.*/
    #[serde(rename = "@grace", skip_serializing_if = "Option::is_none")]
    pub grace: Option<crate::generated::data::DataGrace>,
    ///Records the amount of time to be "stolen" from a non-grace note/chord.
    #[serde(rename = "@grace.time", skip_serializing_if = "Option::is_none")]
    pub grace_time: Option<crate::generated::data::DataPercent>,
    /**Records whether the grace note group is attached to the following event or to the
          preceding one. The usual name for the latter is "Nachschlag".*/
    #[serde(rename = "@attach", skip_serializing_if = "Option::is_none")]
    pub attach: Option<AttGraceGrpLogAttach>,
}

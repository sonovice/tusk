//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTupletLog {
    /**In the case of cross-staff beams, the beam.with attribute is used to indicate which
          staff the beam is connected to; that is, the staff above or the staff below.*/
    #[serde(rename = "@beam.with", skip_serializing_if = "Option::is_none")]
    pub beam_with: Option<crate::generated::data::DataNeighboringlayer>,
    /**When a duration cannot be represented as a single power-of-two value, multiple
          space-separated values that add up to the total duration may be used.*/
    #[serde(rename = "@dur", default, skip_serializing_if = "Vec::is_empty")]
    pub dur: Vec<crate::generated::data::DataDuration>,
    /**Along with numbase, describes duration as a ratio. num is the first value in the
          ratio, while numbase is the second.*/
    #[serde(rename = "@num", skip_serializing_if = "Option::is_none")]
    pub num: Option<u64>,
    /**Along with num, describes duration as a ratio. num is the first value in the ratio,
          while numbase is the second.*/
    #[serde(rename = "@numbase", skip_serializing_if = "Option::is_none")]
    pub numbase: Option<u64>,
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
    /**Holds a reference to the first element in a sequence of events to which the feature
          applies.*/
    #[serde(rename = "@startid", skip_serializing_if = "Option::is_none")]
    pub startid: Option<crate::generated::data::DataUri>,
    /**Indicates the final element in a sequence of events to which the feature
          applies.*/
    #[serde(rename = "@endid", skip_serializing_if = "Option::is_none")]
    pub endid: Option<crate::generated::data::DataUri>,
}

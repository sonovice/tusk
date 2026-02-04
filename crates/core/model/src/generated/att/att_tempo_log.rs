//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttTempoLogEvaluate {
    /**If an element pointed to is itself a pointer, then the target of that pointer will
    be taken, and so on, until an element is found which is not a pointer.*/
    #[serde(rename = "all")]
    All,
    /**If an element pointed to is itself a pointer, then its target (whether a pointer
    or not) is taken as the target of this pointer.*/
    #[serde(rename = "one")]
    One,
    /**No further evaluation of targets is carried out beyond that needed to find the
    element(s) specified in plist or target attribute.*/
    #[serde(rename = "none")]
    None,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttTempoLogFunc {
    ///Marks a gradual change of tempo, such as "accel." or "rit."
    #[serde(rename = "continuous")]
    Continuous,
    /**Represents a static tempo instruction, such as a textual term like "Adagio", a
    metronome marking like "♩=70", or a combination of text and metronome
    indication.*/
    #[serde(rename = "instantaneous")]
    Instantaneous,
    /**Captures a change in pulse rate (tempo) and/or pulse grouping (subdivision) in an
    "equation" of the form [tempo before change] = [tempo after change].*/
    #[serde(rename = "metricmod")]
    Metricmod,
    /**Indicates a change in pulse rate (tempo) and/or pulse grouping (subdivision) in an
    "equation" of the form [tempo after change] = [tempo before change]. The term
    "precedente" often appears following the "equation" to distinguish this kind of
    historical usage from the modern metric modulation form.*/
    #[serde(rename = "precedente")]
    Precedente,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTempoLog {
    /**Indicates the point of occurrence of this feature along a time line. Its value must be
    the ID of awhenelement elsewhere in the document.*/
    #[serde(rename = "@when", skip_serializing_if = "Option::is_none")]
    pub when: Option<crate::generated::data::DataUri>,
    ///Identifies the layer to which a feature applies.
    #[serde(rename = "@layer", default, skip_serializing_if = "Vec::is_empty")]
    pub layer: Vec<u64>,
    /**Indicates the part in which the current feature should appear. Use '%all' when the
    feature should occur in every part.*/
    #[serde(rename = "@part", default, skip_serializing_if = "Vec::is_empty")]
    pub part: Vec<String>,
    /**Signifies the part staff on which a notated feature occurs. Use '%all' when the
    feature should occur on every staff.*/
    #[serde(rename = "@partstaff", default, skip_serializing_if = "Vec::is_empty")]
    pub partstaff: Vec<String>,
    /**When the target attribute is present, plist identifies the active participants; that
    is, those entities pointed "from", in a relationship with the specified target(s). When
    the target attribute is not present, it identifies participants in a mutual
    relationship.*/
    #[serde(rename = "@plist", default, skip_serializing_if = "Vec::is_empty")]
    pub plist: Vec<crate::generated::data::DataUri>,
    /**Signifies the staff on which a notated event occurs or to which a control event
    applies. Mandatory when applicable.*/
    #[serde(rename = "@staff", default, skip_serializing_if = "Vec::is_empty")]
    pub staff: Vec<u64>,
    /**Specifies the intended meaning when a participant in a relationship is itself a
    pointer.*/
    #[serde(rename = "@evaluate", skip_serializing_if = "Option::is_none")]
    pub evaluate: Option<AttTempoLogEvaluate>,
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
    /**Used to describe tempo in terms of beats (often the meter signature denominator) per
    minute, ala M.M. (Maelzel’s Metronome). Do not confuse this attribute with midi.bpm or
    midi.mspb. In MIDI, a beat is always defined as a quarter note, *not the numerator of the
    time signature or the metronomic indication*.*/
    #[serde(rename = "@mm", skip_serializing_if = "Option::is_none")]
    pub mm: Option<crate::generated::data::DataTempovalue>,
    ///Captures the metronomic unit.
    #[serde(rename = "@mm.unit", skip_serializing_if = "Option::is_none")]
    pub mm_unit: Option<crate::generated::data::DataDuration>,
    ///Records the number of augmentation dots required by a dotted metronome unit.
    #[serde(rename = "@mm.dots", skip_serializing_if = "Option::is_none")]
    pub mm_dots: Option<crate::generated::data::DataAugmentdot>,
    /**Holds a reference to the first element in a sequence of events to which the feature
    applies.*/
    #[serde(rename = "@startid", skip_serializing_if = "Option::is_none")]
    pub startid: Option<crate::generated::data::DataUri>,
    /**Indicates the final element in a sequence of events to which the feature
    applies.*/
    #[serde(rename = "@endid", skip_serializing_if = "Option::is_none")]
    pub endid: Option<crate::generated::data::DataUri>,
    /**Encodes the ending point of an event,i.e., a count of measures plus a beat location
    in the ending measure.*/
    #[serde(rename = "@tstamp2", skip_serializing_if = "Option::is_none")]
    pub tstamp2: Option<crate::generated::data::DataMeasurebeat>,
    ///Records the function of a tempo indication.
    #[serde(rename = "@func", skip_serializing_if = "Option::is_none")]
    pub func: Option<AttTempoLogFunc>,
}

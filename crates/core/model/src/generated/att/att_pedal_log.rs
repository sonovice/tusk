//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttPedalLogEvaluate {
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
pub enum AttPedalLogDir {
    ///Depress the pedal.
    #[serde(rename = "down")]
    Down,
    ///Release the pedal.
    #[serde(rename = "up")]
    Up,
    ///Half pedal.
    #[serde(rename = "half")]
    Half,
    ///Release then immediately depress the pedal.
    #[serde(rename = "bounce")]
    Bounce,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttPedalLogFunc {
    /**The sustain pedal, also referred to as the "damper" pedal, allows the piano
    strings to vibrate sympathetically with the struck strings. It is the right-most and
    the most frequently used pedal on modern pianos.*/
    #[serde(rename = "sustain")]
    Sustain,
    /**The soft pedal, sometimes called the "una corda", "piano", or "half-blow" pedal,
    reduces the volume and modifies the timbre of the piano. On the modern piano, it is
    the left-most pedal.*/
    #[serde(rename = "soft")]
    Soft,
    /**The sostenuto or tone-sustaining pedal allows notes already undamped to continue
    to ring while other notes are damped normally; that is, on their release by the
    fingers. This is usually the center pedal of the modern piano.*/
    #[serde(rename = "sostenuto")]
    Sostenuto,
    /**The silent or practice pedal mutes the volume of the piano so that one may
    practice quietly. It is sometimes a replacement for the sostenuto pedal, especially on
    an upright or vertical instrument.*/
    #[serde(rename = "silent")]
    Silent,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPedalLog {
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
    pub evaluate: Option<AttPedalLogEvaluate>,
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
    ///Records the position of the piano damper pedal.
    #[serde(rename = "@dir", skip_serializing_if = "Option::is_none")]
    pub dir: Option<AttPedalLogDir>,
    /**Indicates the function of the depressed pedal, but not necessarily the text associated
    with its use. Use thedirelement for such text.*/
    #[serde(rename = "@func", skip_serializing_if = "Option::is_none")]
    pub func: Option<AttPedalLogFunc>,
}

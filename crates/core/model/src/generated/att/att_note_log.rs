//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteLog {
    ///Records the number of augmentation dots required by a written dotted duration.
    #[serde(rename = "@dots", skip_serializing_if = "Option::is_none")]
    pub dots: Option<crate::generated::data::DataAugmentdot>,
    /**Indicates this feature is 'colored'; that is, it is a participant in a change in
          rhythmic values. In mensural notation, coloration is indicated by colored notes (red,
          black, etc.) where void notes would otherwise occur. In CMN, coloration is indicated by an
          inverse color; that is, the note head is void when it would otherwise be filled and vice
          versa.*/
    #[serde(rename = "@colored", skip_serializing_if = "Option::is_none")]
    pub colored: Option<crate::generated::data::DataBoolean>,
    ///
    #[serde(rename = "@cue", skip_serializing_if = "Option::is_none")]
    pub cue: Option<crate::generated::data::DataBoolean>,
    /**Records the duration of a feature using the relative durational values provided by the
          data.DURATION datatype.*/
    #[serde(rename = "@dur", skip_serializing_if = "Option::is_none")]
    pub dur: Option<crate::generated::data::DataDuration>,
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
    ///Contains a written pitch name.
    #[serde(rename = "@pname", skip_serializing_if = "Option::is_none")]
    pub pname: Option<crate::generated::data::DataPitchname>,
    ///Captures written octave information.
    #[serde(rename = "@oct", skip_serializing_if = "Option::is_none")]
    pub oct: Option<crate::generated::data::DataOctave>,
    ///Encodes the durational quality of a mensural note using the values provided by the data.DURQUALITY.mensural datatype (i.e., the perfect / imperfect / altered / major / minor / duplex quality of a note).
    #[serde(rename = "@dur.quality", skip_serializing_if = "Option::is_none")]
    pub dur_quality: Option<crate::generated::data::DataDurqualityMensural>,
}

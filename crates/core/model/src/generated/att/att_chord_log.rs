//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Logical domain attributes for chord. The artic, dots, and dur attributes encode the
      written articulations, augmentation dots, and duration values. The beam, fermata, lv, slur,
      syl, tie, and tuplet attributes may be used to indicate the attachment of these things to this
      chord. If visual information about these things needs to be recorded, then either the elements
      corresponding to these attributes or the attributes available in the att.vis.chord class
      should be employed.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttChordLog {
    /**Encodes the written articulation(s). Articulations are normally encoded in order from
          the note head outward; that is, away from the stem. See additional notes at att.vis.note.
          Only articulations should be encoded in the artic attribute; for example, fingerings
          should be encoded using thefingelement.*/
    #[serde(rename = "@artic", default, skip_serializing_if = "Vec::is_empty")]
    pub artic: Vec<crate::generated::data::DataArticulation>,
    ///Records the number of augmentation dots required by a written dotted duration.
    #[serde(rename = "@dots", skip_serializing_if = "Option::is_none")]
    pub dots: Option<crate::generated::data::DataAugmentdot>,
    /**Marks a note or chord as a "grace" (without a definite performed duration) and records
          from which other note/chord it should "steal" time.*/
    #[serde(rename = "@grace", skip_serializing_if = "Option::is_none")]
    pub grace: Option<crate::generated::data::DataGrace>,
    ///Records the amount of time to be "stolen" from a non-grace note/chord.
    #[serde(rename = "@grace.time", skip_serializing_if = "Option::is_none")]
    pub grace_time: Option<crate::generated::data::DataPercent>,
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
    ///Holds an associated sung text syllable.
    #[serde(rename = "@syl", skip_serializing_if = "Option::is_none")]
    pub syl: Option<String>,
}

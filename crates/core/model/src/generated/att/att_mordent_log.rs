//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMordentLogEvaluate {
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
pub enum AttMordentLogForm {
    /**Starts with the written note, followed by its lower neighbor, with a return to the
              written note. In modern practice, this is called an "inverted mordent" and indicated
              by a short wavy line with a vertical line through it.*/
    #[serde(rename = "lower")]
    Lower,
    /**Starts with the written note, followed by its upper neighbor, with a return to the
              principal note. In modern practice, the symbol lacks the vertical line used for the
              inverted form.*/
    #[serde(rename = "upper")]
    Upper,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMordentLog {
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
    pub evaluate: Option<AttMordentLogEvaluate>,
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
    ///Records the sounding accidental associated with an upper neighboring note.
    #[serde(rename = "@accidupper.ges", skip_serializing_if = "Option::is_none")]
    pub accidupper_ges: Option<crate::generated::data::DataAccidentalGestural>,
    ///Records the sounding accidental associated with a lower neighboring note.
    #[serde(rename = "@accidlower.ges", skip_serializing_if = "Option::is_none")]
    pub accidlower_ges: Option<crate::generated::data::DataAccidentalGestural>,
    ///Records the written accidental associated with an upper neighboring note.
    #[serde(rename = "@accidupper", skip_serializing_if = "Option::is_none")]
    pub accidupper: Option<crate::generated::data::DataAccidentalWritten>,
    ///Records the written accidental associated with a lower neighboring note.
    #[serde(rename = "@accidlower", skip_serializing_if = "Option::is_none")]
    pub accidlower: Option<crate::generated::data::DataAccidentalWritten>,
    /**Records semantic meaning,i.e., intended performance, of the mordent. Thealtsym,glyph.name, orglyph.numattributes may be used
          to specify the appropriate symbol.*/
    #[serde(rename = "@form", skip_serializing_if = "Option::is_none")]
    pub form: Option<AttMordentLogForm>,
    /**When set to 'true', a double or long mordent, sometimes called a "pincé double",
          consisting of 5 notes, is indicated.*/
    #[serde(rename = "@long", skip_serializing_if = "Option::is_none")]
    pub long: Option<crate::generated::data::DataBoolean>,
}

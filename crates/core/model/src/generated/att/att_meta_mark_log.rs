//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMetaMarkLogEvaluate {
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
pub enum AttMetaMarkLogEvidence {
    ///There is evidence within the document to support the intervention.
    #[serde(rename = "internal")]
    Internal,
    ///There is evidence outside the document to support the intervention.
    #[serde(rename = "external")]
    External,
    /**The assertion has been made by the editor, cataloguer, or scholar on the basis of
              their expertise.*/
    #[serde(rename = "conjecture")]
    Conjecture,
}
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMetaMarkLog {
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
    pub evaluate: Option<AttMetaMarkLogEvaluate>,
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
    /**Contains a list of one or more pointers indicating the sources which attest to a given
          reading. Each value should correspond to the ID of asourceormanifestationelement located in the document header.*/
    #[serde(rename = "@source", default, skip_serializing_if = "Vec::is_empty")]
    pub source: Vec<crate::generated::data::DataUri>,
    ///Signifies the degree of certainty or precision associated with a feature.
    #[serde(rename = "@cert", skip_serializing_if = "Option::is_none")]
    pub cert: Option<crate::generated::data::DataCertainty>,
    /**Indicates the nature of the evidence supporting the reliability or accuracy of the
          intervention or interpretation.*/
    #[serde(rename = "@evidence", skip_serializing_if = "Option::is_none")]
    pub evidence: Option<AttMetaMarkLogEvidence>,
    /**The @instant attribute is syntactic sugar for classifying a scribal intervention as an
          ad-hoc modification; that is, one which does not interrupt the writing process.*/
    #[serde(rename = "@instant", skip_serializing_if = "Option::is_none")]
    pub instant: Option<crate::generated::data::DataBoolean>,
    ///Points to the genetic state that results from this modification.
    #[serde(rename = "@state", default, skip_serializing_if = "Vec::is_empty")]
    pub state: Vec<crate::generated::data::DataUri>,
    ///Signifies the hand responsible for an action. The value must be the ID of ahandelement declared in the header.
    #[serde(rename = "@hand", skip_serializing_if = "Option::is_none")]
    pub hand: Option<crate::generated::data::DataUri>,
    /**Identifies one or more metadata elements (other than classification terms) within the
          header, which are understood to apply to the element bearing this attribute and its
          content.*/
    #[serde(rename = "@decls", default, skip_serializing_if = "Vec::is_empty")]
    pub decls: Vec<crate::generated::data::DataUri>,
    /**Used to assign a sequence number related to the order in which the encoded features
          carrying this attribute are believed to have occurred.*/
    #[serde(rename = "@seq", skip_serializing_if = "Option::is_none")]
    pub seq: Option<u64>,
}

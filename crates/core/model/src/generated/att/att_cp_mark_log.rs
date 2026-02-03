//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttCpMarkLogEvaluate {
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
///Logical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttCpMarkLog {
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
    pub evaluate: Option<AttCpMarkLogEvaluate>,
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
    /**encodes the starting point of musical material in terms of musical time,i.e., a
    (potentially negative) count of measures plus a beat location.*/
    #[serde(rename = "@origin.tstamp", skip_serializing_if = "Option::is_none")]
    pub origin_tstamp: Option<crate::generated::data::DataMeasurebeatoffset>,
    /**encodes the ending point of musical material in terms of musical time,i.e., a count
    of measures plus a beat location. The values are relative to the measure identified byorigin.tstamp.*/
    #[serde(rename = "@origin.tstamp2", skip_serializing_if = "Option::is_none")]
    pub origin_tstamp2: Option<crate::generated::data::DataMeasurebeat>,
    /**signifies the staff on which referenced notation occurs. Defaults to the same value as
    the local staff. Mandatory when applicable.*/
    #[serde(rename = "@origin.staff", skip_serializing_if = "Option::is_none")]
    pub origin_staff: Option<crate::generated::SpaceSeparated<u64>>,
    ///identifies the layer on which referenced notation occurs.
    #[serde(rename = "@origin.layer", skip_serializing_if = "Option::is_none")]
    pub origin_layer: Option<crate::generated::SpaceSeparated<u64>>,
    ///indicates the first element in a sequence of events.
    #[serde(rename = "@origin.startid", skip_serializing_if = "Option::is_none")]
    pub origin_startid: Option<crate::generated::data::DataUri>,
    ///indicates the final element in a sequence of events.
    #[serde(rename = "@origin.endid", skip_serializing_if = "Option::is_none")]
    pub origin_endid: Option<crate::generated::data::DataUri>,
    ///Records the amount of octave displacement.
    #[serde(rename = "@dis", skip_serializing_if = "Option::is_none")]
    pub dis: Option<crate::generated::data::DataOctaveDis>,
    ///Records the direction of octave displacement.
    #[serde(rename = "@dis.place", skip_serializing_if = "Option::is_none")]
    pub dis_place: Option<crate::generated::data::DataStaffrelBasic>,
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
}

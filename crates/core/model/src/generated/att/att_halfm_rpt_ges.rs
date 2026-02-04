//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttHalfmRptGes {
    ///Records performed duration information that differs from the written duration; @dur.ges allows the same datatypes as @dur. Values of @dur.ges that require dots should also use @dots.ges.
    #[serde(rename = "@dur.ges", skip_serializing_if = "Option::is_none")]
    pub dur_ges: Option<crate::generated::data::DataDurationGestural>,
    /**Number of dots required for a gestural duration when different from that of the
    written duration.*/
    #[serde(rename = "@dots.ges", skip_serializing_if = "Option::is_none")]
    pub dots_ges: Option<crate::generated::data::DataAugmentdot>,
    ///Duration as a count of units provided in the time signature denominator.
    #[serde(rename = "@dur.metrical", skip_serializing_if = "Option::is_none")]
    pub dur_metrical: Option<f64>,
    /**Duration recorded as pulses-per-quarter note,e.g., MIDI clicks or MusicXML
    divisions.*/
    #[serde(rename = "@dur.ppq", skip_serializing_if = "Option::is_none")]
    pub dur_ppq: Option<u64>,
    ///Duration in seconds,e.g.,1.732.
    #[serde(rename = "@dur.real", skip_serializing_if = "Option::is_none")]
    pub dur_real: Option<f64>,
    ///Duration as an optionally dottedHumdrum **recip value.
    #[serde(rename = "@dur.recip", skip_serializing_if = "Option::is_none")]
    pub dur_recip: Option<String>,
}

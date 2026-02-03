//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNoteGesExtremis {
    ///Highest note the performer can play.
    #[serde(rename = "highest")]
    Highest,
    ///Lowest note the performer can play.
    #[serde(rename = "lowest")]
    Lowest,
}
///Gestural domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteGes {
    ///Records the performed pitch inflection.
    #[serde(rename = "@accid.ges", skip_serializing_if = "Option::is_none")]
    pub accid_ges: Option<crate::generated::data::DataAccidentalGestural>,
    ///Records performed articulation that differs from the written value.
    #[serde(rename = "@artic.ges", default, skip_serializing_if = "Vec::is_empty")]
    pub artic_ges: Vec<crate::generated::data::DataArticulation>,
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
    /**Provides a way of pointing to a MIDI instrument definition. It must contain the ID of
    aninstrDefelement elsewhere in the document.*/
    #[serde(rename = "@instr", skip_serializing_if = "Option::is_none")]
    pub instr: Option<crate::generated::data::DataUri>,
    ///MIDI Note-on/off velocity.
    #[serde(rename = "@vel", skip_serializing_if = "Option::is_none")]
    pub vel: Option<crate::generated::data::DataMidivalue>,
    /**Along with numbase, describes duration as a ratio. num is the first value in the
    ratio, while numbase is the second.*/
    #[serde(rename = "@num", skip_serializing_if = "Option::is_none")]
    pub num: Option<u64>,
    /**Along with num, describes duration as a ratio. num is the first value in the ratio,
    while numbase is the second.*/
    #[serde(rename = "@numbase", skip_serializing_if = "Option::is_none")]
    pub numbase: Option<u64>,
    ///Records performed octave information that differs from the written value.
    #[serde(rename = "@oct.ges", skip_serializing_if = "Option::is_none")]
    pub oct_ges: Option<crate::generated::data::DataOctave>,
    ///Contains a performed pitch name that differs from the written value.
    #[serde(rename = "@pname.ges", skip_serializing_if = "Option::is_none")]
    pub pname_ges: Option<crate::generated::data::DataPitchnameGestural>,
    ///Holds a pitch-to-number mapping, a base-40 or MIDI note number, for example.
    #[serde(rename = "@pnum", skip_serializing_if = "Option::is_none")]
    pub pnum: Option<crate::generated::data::DataPitchnumber>,
    ///This attribute is deprecated and will be removed in a future version. Indicates which finger, if any, should be used to play an individual string. The index, middle, ring, and little fingers are represented by the values 1-4, whiletis for the thumb. The valuesxandoindicate muffled and open strings, respectively.
    #[serde(rename = "@tab.fing", skip_serializing_if = "Option::is_none")]
    pub tab_fing: Option<crate::generated::data::DataFingerFret>,
    ///Records the location at which a string should be stopped against a fret.
    #[serde(rename = "@tab.fret", skip_serializing_if = "Option::is_none")]
    pub tab_fret: Option<crate::generated::data::DataFretnumber>,
    ///Used in German lute tablature in cases where vertical positioning deviates from the norm which can be specified bytab.align. Indicates the position of the tab note on one of the horizontal strands corresponding to thelinesattribute onstaffDef. (Note that in this case, the lines are conceptual rather than visible).
    #[serde(rename = "@tab.line", skip_serializing_if = "Option::is_none")]
    pub tab_line: Option<crate::generated::data::DataClefline>,
    ///This attribute is deprecated in favor oftab.courseand will be removed in a future version. Records which string is to be played.
    #[serde(rename = "@tab.string", skip_serializing_if = "Option::is_none")]
    pub tab_string: Option<crate::generated::data::DataStringnumber>,
    ///Records which course is to be played.
    #[serde(rename = "@tab.course", skip_serializing_if = "Option::is_none")]
    pub tab_course: Option<crate::generated::data::DataCoursenumber>,
    ///Indicates an extreme, indefinite performed pitch.
    #[serde(rename = "@extremis", skip_serializing_if = "Option::is_none")]
    pub extremis: Option<AttNoteGesExtremis>,
}

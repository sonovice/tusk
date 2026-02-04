//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Analytical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteAnl {
    ///Captures a written accidental.
    #[serde(rename = "@accid", skip_serializing_if = "Option::is_none")]
    pub accid: Option<crate::generated::data::DataAccidentalWritten>,
    /**Encodes the written articulation(s). Articulations are normally encoded in order from
    the note head outward; that is, away from the stem. See additional notes at att.vis.note.
    Only articulations should be encoded in the artic attribute; for example, fingerings
    should be encoded using thefingelement.*/
    #[serde(rename = "@artic", default, skip_serializing_if = "Vec::is_empty")]
    pub artic: Vec<crate::generated::data::DataArticulation>,
    /**Captures scale degree information usingHumdrum **deg syntax-- an optional indicator
    of melodic approach (^ = ascending approach, v = descending approach), a scale degree
    value (1 = tonic ... 7 = leading tone), and an optional indication of chromatic
    alteration,1,v7,^1, orv5+, for example.
    The amount of chromatic alternation is not indicated.*/
    #[serde(rename = "@deg", skip_serializing_if = "Option::is_none")]
    pub deg: Option<crate::generated::data::DataScaledegree>,
    /**Encodes the melodic interval from the previous pitch. The value may be a general
    directional indication (u, d, s, etc.), an indication of diatonic interval direction,
    quality, and size, or a precise numeric value in half steps.*/
    #[serde(rename = "@intm", skip_serializing_if = "Option::is_none")]
    pub intm: Option<crate::generated::data::DataIntervalMelodic>,
    ///Describes melodic function usingHumdrum **embel syntax.
    #[serde(rename = "@mfunc", skip_serializing_if = "Option::is_none")]
    pub mfunc: Option<crate::generated::data::DataMelodicfunction>,
    ///Indicates that this event is "under a beam".
    #[serde(rename = "@beam", default, skip_serializing_if = "Vec::is_empty")]
    pub beam: Vec<crate::generated::data::DataBeam>,
    /**Indicates that this element participates in a glissando. If visual information about
    the glissando needs to be recorded, then aglisselement should be
    employed instead.*/
    #[serde(rename = "@gliss", skip_serializing_if = "Option::is_none")]
    pub gliss: Option<crate::generated::data::DataGlissando>,
    ///Indicates the attachment of an l.v. (laissez vibrer) sign to this element.
    #[serde(rename = "@lv", skip_serializing_if = "Option::is_none")]
    pub lv: Option<crate::generated::data::DataBoolean>,
    /**Indicates that this element has an attached ornament. If visual information about the
    ornament is needed, then one of the elements that represents an ornament (mordent, trill,
    or turn) should be employed.*/
    #[serde(rename = "@ornam", default, skip_serializing_if = "Vec::is_empty")]
    pub ornam: Vec<crate::generated::data::DataOrnamCmn>,
    /**Indicates that this element participates in a slur. If visual information about the
    slur needs to be recorded, then aslurelement should be
    employed.*/
    #[serde(rename = "@slur", default, skip_serializing_if = "Vec::is_empty")]
    pub slur: Vec<crate::generated::data::DataSlur>,
    ///Holds an associated sung text syllable.
    #[serde(rename = "@syl", skip_serializing_if = "Option::is_none")]
    pub syl: Option<String>,
    /**Indicates that this element participates in a tie. If visual information about the tie
    needs to be recorded, then atieelement should be employed.*/
    #[serde(rename = "@tie", default, skip_serializing_if = "Vec::is_empty")]
    pub tie: Vec<crate::generated::data::DataTie>,
    /**Indicates that this feature participates in a tuplet. If visual information about the
    tuplet needs to be recorded, then atupletelement should be
    employed.*/
    #[serde(rename = "@tuplet", default, skip_serializing_if = "Vec::is_empty")]
    pub tuplet: Vec<crate::generated::data::DataTuplet>,
    /**Indicates the attachment of a fermata to this element. If visual information about the
    fermata needs to be recorded, then afermataelement should be
    employed instead.*/
    #[serde(rename = "@fermata", skip_serializing_if = "Option::is_none")]
    pub fermata: Option<crate::generated::data::DataStaffrelBasic>,
    ///Holds pitch class information.
    #[serde(rename = "@pclass", skip_serializing_if = "Option::is_none")]
    pub pclass: Option<crate::generated::data::DataPitchclass>,
    /**Contains sol-fa designation,e.g., do, re, mi, etc., in either a fixed or movable Do
    system.*/
    #[serde(rename = "@psolfa", skip_serializing_if = "Option::is_none")]
    pub psolfa: Option<String>,
}

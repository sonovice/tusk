//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Accidentals associated with ornaments.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOrnamentAccid {
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
}

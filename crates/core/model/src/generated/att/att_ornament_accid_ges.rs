//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Gestural accidentals associated with ornaments.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttOrnamentAccidGes {
    ///Records the sounding accidental associated with an upper neighboring note.
    #[serde(rename = "@accidupper.ges", skip_serializing_if = "Option::is_none")]
    pub accidupper_ges: Option<crate::generated::data::DataAccidentalGestural>,
    ///Records the sounding accidental associated with a lower neighboring note.
    #[serde(rename = "@accidlower.ges", skip_serializing_if = "Option::is_none")]
    pub accidlower_ges: Option<crate::generated::data::DataAccidentalGestural>,
}

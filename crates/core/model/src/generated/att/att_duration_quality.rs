//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attribute that expresses duration for a given mensural note symbol.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDurationQuality {
    ///Encodes the durational quality of a mensural note using the values provided by the data.DURQUALITY.mensural datatype (i.e., the perfect / imperfect / altered / major / minor / duplex quality of a note).
    #[serde(rename = "@dur.quality", skip_serializing_if = "Option::is_none")]
    pub dur_quality: Option<crate::generated::data::DataDurqualityMensural>,
}

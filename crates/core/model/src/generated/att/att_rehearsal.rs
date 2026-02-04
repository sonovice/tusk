//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttRehearsalRehEnclose {
    ///Enclosed by box.
    #[serde(rename = "box")]
    Box,
    ///Enclosed by circle.
    #[serde(rename = "circle")]
    Circle,
    ///No enclosing shape.
    #[serde(rename = "none")]
    None,
}
/**Attributes used by scoreDef and staffDef to provide default information about rehearsal
      numbers/letters.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttRehearsal {
    ///Describes the enclosing shape for rehearsal marks.
    #[serde(rename = "@reh.enclose", skip_serializing_if = "Option::is_none")]
    pub reh_enclose: Option<AttRehearsalRehEnclose>,
}

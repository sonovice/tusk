//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Logical domain attributes. These attributes describe augmentation or diminution of the
normal value of the notes in mensural notation as a ratio.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttProportLog {
    /**Along with numbase, describes duration as a ratio. num is the first value in the
    ratio, while numbase is the second.*/
    #[serde(rename = "@num", skip_serializing_if = "Option::is_none")]
    pub num: Option<u64>,
    /**Along with num, describes duration as a ratio. num is the first value in the ratio,
    while numbase is the second.*/
    #[serde(rename = "@numbase", skip_serializing_if = "Option::is_none")]
    pub numbase: Option<u64>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMeiVersionMeiversion {
    ///Version of MEI
    #[serde(rename = "6.0-dev")]
    N60Dev,
    ///Version of MEI All AnyStart customization
    #[serde(rename = "6.0-dev+anyStart")]
    N60DevAnyStart,
    ///Version of MEI Basic customization
    #[serde(rename = "6.0-dev+basic")]
    N60DevBasic,
    ///Version of MEI CMN customization
    #[serde(rename = "6.0-dev+cmn")]
    N60DevCmn,
    ///Version of MEI Mensural customization
    #[serde(rename = "6.0-dev+mensural")]
    N60DevMensural,
    ///Version of MEI Neumes customization
    #[serde(rename = "6.0-dev+neumes")]
    N60DevNeumes,
}
///Attributes that record the version of MEI in use.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeiVersion {
    ///Specifies a generic MEI version label.
    #[serde(rename = "@meiversion", skip_serializing_if = "Option::is_none")]
    pub meiversion: Option<AttMeiVersionMeiversion>,
}

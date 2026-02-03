//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNeumeAnlType {
    ///
    #[serde(rename = "apostropha")]
    Apostropha,
    ///
    #[serde(rename = "bistropha")]
    Bistropha,
    ///
    #[serde(rename = "cephalicus")]
    Cephalicus,
    ///
    #[serde(rename = "climacus")]
    Climacus,
    ///
    #[serde(rename = "clivis")]
    Clivis,
    ///
    #[serde(rename = "epiphonus")]
    Epiphonus,
    ///
    #[serde(rename = "oriscus")]
    Oriscus,
    ///
    #[serde(rename = "pes")]
    Pes,
    ///
    #[serde(rename = "pessubpunctis")]
    Pessubpunctis,
    ///
    #[serde(rename = "porrectus")]
    Porrectus,
    ///
    #[serde(rename = "porrectusflexus")]
    Porrectusflexus,
    ///
    #[serde(rename = "pressusmaior")]
    Pressusmaior,
    ///
    #[serde(rename = "pressusminor")]
    Pressusminor,
    ///
    #[serde(rename = "punctum")]
    Punctum,
    ///
    #[serde(rename = "quilisma")]
    Quilisma,
    ///
    #[serde(rename = "scandicus")]
    Scandicus,
    ///
    #[serde(rename = "strophicus")]
    Strophicus,
    ///
    #[serde(rename = "torculus")]
    Torculus,
    ///
    #[serde(rename = "torculusresupinus")]
    Torculusresupinus,
    ///
    #[serde(rename = "tristropha")]
    Tristropha,
    ///
    #[serde(rename = "virga")]
    Virga,
    ///
    #[serde(rename = "virgastrata")]
    Virgastrata,
}
///Analytical domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNeumeAnl {
    /**Designation which characterizes the element in some sense, using any convenient
    classification scheme or typology that employs single-token labels.*/
    #[serde(rename = "@type", default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<AttNeumeAnlType>,
}

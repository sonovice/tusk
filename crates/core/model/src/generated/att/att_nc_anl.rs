//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNcAnlType {
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
pub struct AttNcAnl {
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
    /**Designation which characterizes the element in some sense, using any convenient
    classification scheme or typology that employs single-token labels.*/
    #[serde(rename = "@type", default, skip_serializing_if = "Vec::is_empty")]
    pub r#type: Vec<AttNcAnlType>,
    ///Holds pitch class information.
    #[serde(rename = "@pclass", skip_serializing_if = "Option::is_none")]
    pub pclass: Option<crate::generated::data::DataPitchclass>,
    /**Contains sol-fa designation,e.g., do, re, mi, etc., in either a fixed or movable Do
    system.*/
    #[serde(rename = "@psolfa", skip_serializing_if = "Option::is_none")]
    pub psolfa: Option<String>,
}

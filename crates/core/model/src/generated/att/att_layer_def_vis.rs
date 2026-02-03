//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttLayerDefVisBeamRend {
    ///Beam lines grow farther apart from left to right.
    #[serde(rename = "acc")]
    Acc,
    ///Beam lines grow closer together from left to right.
    #[serde(rename = "rit")]
    Rit,
    ///Beam lines are equally-spaced over the entire length of the beam.
    #[serde(rename = "norm")]
    Norm,
}
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttLayerDefVis {
    ///Color of beams, including those associated with tuplets.
    #[serde(rename = "@beam.color", skip_serializing_if = "Option::is_none")]
    pub beam_color: Option<crate::generated::data::DataColor>,
    ///Encodes whether a beam is "feathered" and in which direction.
    #[serde(rename = "@beam.rend", skip_serializing_if = "Option::is_none")]
    pub beam_rend: Option<AttLayerDefVisBeamRend>,
    ///Captures beam slope.
    #[serde(rename = "@beam.slope", skip_serializing_if = "Option::is_none")]
    pub beam_slope: Option<f64>,
    /**Provides a default value for the font family name of text (other than lyrics) when
    this information is not provided on the individual elements.*/
    #[serde(rename = "@text.fam", skip_serializing_if = "Option::is_none")]
    pub text_fam: Option<crate::generated::data::DataFontfamily>,
    /**Provides a default value for the font name of text (other than lyrics) when this
    information is not provided on the individual elements.*/
    #[serde(rename = "@text.name", skip_serializing_if = "Option::is_none")]
    pub text_name: Option<crate::generated::data::DataFontname>,
    /**Provides a default value for the font size of text (other than lyrics) when this
    information is not provided on the individual elements.*/
    #[serde(rename = "@text.size", skip_serializing_if = "Option::is_none")]
    pub text_size: Option<crate::generated::data::DataFontsize>,
    /**Provides a default value for the font style of text (other than lyrics) when this
    information is not provided on the individual elements.*/
    #[serde(rename = "@text.style", skip_serializing_if = "Option::is_none")]
    pub text_style: Option<crate::generated::data::DataFontstyle>,
    /**Provides a default value for the font weight for text (other than lyrics) when this
    information is not provided on the individual elements.*/
    #[serde(rename = "@text.weight", skip_serializing_if = "Option::is_none")]
    pub text_weight: Option<crate::generated::data::DataFontweight>,
    /**Indicates if a feature should be rendered when the notation is presented graphically
    or sounded when it is presented in an aural form.*/
    #[serde(rename = "@visible", skip_serializing_if = "Option::is_none")]
    pub visible: Option<crate::generated::data::DataBoolean>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttScoreDefVisCmnBeamRend {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttScoreDefVisCmnRehEnclose {
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
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttScoreDefVisCmn {
    ///Color of beams, including those associated with tuplets.
    #[serde(rename = "@beam.color", skip_serializing_if = "Option::is_none")]
    pub beam_color: Option<crate::generated::data::DataColor>,
    ///Encodes whether a beam is "feathered" and in which direction.
    #[serde(rename = "@beam.rend", skip_serializing_if = "Option::is_none")]
    pub beam_rend: Option<AttScoreDefVisCmnBeamRend>,
    ///Captures beam slope.
    #[serde(rename = "@beam.slope", skip_serializing_if = "Option::is_none")]
    pub beam_slope: Option<f64>,
    ///Determines whether to display guitar chord grids.
    #[serde(rename = "@grid.show", skip_serializing_if = "Option::is_none")]
    pub grid_show: Option<crate::generated::data::DataBoolean>,
    ///Determines whether piano pedal marks should be rendered as lines or as terms.
    #[serde(rename = "@pedal.style", skip_serializing_if = "Option::is_none")]
    pub pedal_style: Option<crate::generated::data::DataPedalstyle>,
    ///Describes the enclosing shape for rehearsal marks.
    #[serde(rename = "@reh.enclose", skip_serializing_if = "Option::is_none")]
    pub reh_enclose: Option<AttScoreDefVisCmnRehEnclose>,
    ///
    #[serde(rename = "@slur.lform", skip_serializing_if = "Option::is_none")]
    pub slur_lform: Option<crate::generated::data::DataLineform>,
    ///
    #[serde(rename = "@slur.lwidth", skip_serializing_if = "Option::is_none")]
    pub slur_lwidth: Option<crate::generated::data::DataLinewidth>,
    ///
    #[serde(rename = "@tie.lform", skip_serializing_if = "Option::is_none")]
    pub tie_lform: Option<crate::generated::data::DataLineform>,
    ///
    #[serde(rename = "@tie.lwidth", skip_serializing_if = "Option::is_none")]
    pub tie_lwidth: Option<crate::generated::data::DataLinewidth>,
}

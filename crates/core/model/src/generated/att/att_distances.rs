//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe distance from the staff.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttDistances {
    ///Records the default distance from the staff for directives.
    #[serde(rename = "@dir.dist", skip_serializing_if = "Option::is_none")]
    pub dir_dist: Option<crate::generated::data::DataMeasurementsigned>,
    ///Records the default distance from the staff for dynamic marks.
    #[serde(rename = "@dynam.dist", skip_serializing_if = "Option::is_none")]
    pub dynam_dist: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records the default distance from the staff of harmonic indications, such as guitar
          chord grids or functional labels.*/
    #[serde(rename = "@harm.dist", skip_serializing_if = "Option::is_none")]
    pub harm_dist: Option<crate::generated::data::DataMeasurementsigned>,
    ///Records the default distance from the staff for rehearsal marks.
    #[serde(rename = "@reh.dist", skip_serializing_if = "Option::is_none")]
    pub reh_dist: Option<crate::generated::data::DataMeasurementsigned>,
    ///Records the default distance from the staff for tempo marks.
    #[serde(rename = "@tempo.dist", skip_serializing_if = "Option::is_none")]
    pub tempo_dist: Option<crate::generated::data::DataMeasurementsigned>,
}

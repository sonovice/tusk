//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttStageDirVis {
    /**Captures the placement of the item with respect to the staff with which it is
    associated.*/
    #[serde(rename = "@place", skip_serializing_if = "Option::is_none")]
    pub place: Option<crate::generated::data::DataStaffrel>,
    /**Records a horizontal adjustment to a feature’s programmatically-determined location in
    terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
    staff lines.*/
    #[serde(rename = "@ho", skip_serializing_if = "Option::is_none")]
    pub ho: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records a timestamp adjustment of a feature’s programmatically-determined location in
    terms of musical time; that is, beats.*/
    #[serde(rename = "@to", skip_serializing_if = "Option::is_none")]
    pub to: Option<crate::generated::data::DataTstampoffset>,
    /**Records the vertical adjustment of a feature’s programmatically-determined location in
    terms of staff interline distance; that is, in units of 1/2 the distance between adjacent
    staff lines.*/
    #[serde(rename = "@vo", skip_serializing_if = "Option::is_none")]
    pub vo: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records the horizontal adjustment of a feature’s programmatically-determined start
    point.*/
    #[serde(rename = "@startho", skip_serializing_if = "Option::is_none")]
    pub startho: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records the horizontal adjustment of a feature’s programmatically-determined end
    point.*/
    #[serde(rename = "@endho", skip_serializing_if = "Option::is_none")]
    pub endho: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records a timestamp adjustment of a feature’s programmatically-determined start
    point.*/
    #[serde(rename = "@startto", skip_serializing_if = "Option::is_none")]
    pub startto: Option<crate::generated::data::DataTstampoffset>,
    /**Records a timestamp adjustment of a feature’s programmatically-determined end
    point.*/
    #[serde(rename = "@endto", skip_serializing_if = "Option::is_none")]
    pub endto: Option<crate::generated::data::DataTstampoffset>,
    /**Encodes an x coordinate for a feature in an output coordinate system. When it is
    necessary to record the placement of a feature in a facsimile image, use the facs
    attribute.*/
    #[serde(rename = "@x", skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /**Encodes a y coordinate for a feature in an output coordinate system. When it is
    necessary to record the placement of a feature in a facsimile image, use the facs
    attribute.*/
    #[serde(rename = "@y", skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
}

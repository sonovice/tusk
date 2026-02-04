//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttBendVisCurvedir {
    ///Upward curve.
    #[serde(rename = "above")]
    Above,
    ///Downward curve.
    #[serde(rename = "below")]
    Below,
    ///A "meandering" curve, both above and below the items it pertains to.
    #[serde(rename = "mixed")]
    Mixed,
}
/**Visual domain attributes. If the bulge or bezier attributes are present, the bend should
be rendered as a curve. Otherwise, it should be rendered using lines. The ho and vo attributes
describe the visual offset of the entire rendered bend. The endho, endvo and startho, startvo
attribute pairs may be used to encode start and end points relative to their programmatic
placement. For exact placement of the endpoints of the bend, use the x and y
attributes.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttBendVis {
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
    as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
    /**Records the placement of Bezier control points as a series of pairs of space-separated
    values;e.g.,19 45 -32 118.*/
    #[serde(rename = "@bezier", skip_serializing_if = "Option::is_none")]
    pub bezier: Option<crate::generated::SpaceSeparated<f64>>,
    /**Describes a curve as one or more pairs of values with respect to an imaginary line
    connecting the starting and ending points of the curve. The first value captures a
    distance to the left (positive value) or right (negative value) of the line, expressed in
    virtual units. The second value of each pair represents a point along the line, expressed
    as a percentage of the line’s length. N.B. An MEI virtual unit (vu) is half the distance
    between adjacent staff lines where the interline space is measured from the middle of a
    staff line.*/
    #[serde(rename = "@bulge", skip_serializing_if = "Option::is_none")]
    pub bulge: Option<crate::generated::SpaceSeparated<crate::generated::data::DataPercent>>,
    ///Describes a curve with a generic term indicating the direction of curvature.
    #[serde(rename = "@curvedir", skip_serializing_if = "Option::is_none")]
    pub curvedir: Option<AttBendVisCurvedir>,
    ///Describes the style of a line.
    #[serde(rename = "@lform", skip_serializing_if = "Option::is_none")]
    pub lform: Option<crate::generated::data::DataLineform>,
    ///Width of a line.
    #[serde(rename = "@lwidth", skip_serializing_if = "Option::is_none")]
    pub lwidth: Option<crate::generated::data::DataLinewidth>,
    /**Describes the number of segments into which a dashed or dotted line may be divided, or
    the number of "peaks" of a wavy line; a pair of space-separated values (minimum and
    maximum, respectively) provides a range between which a rendering system-supplied value
    may fall, while a single value indicates a fixed amount of space; that is, the minimum and
    maximum values are equal.*/
    #[serde(rename = "@lsegs", skip_serializing_if = "Option::is_none")]
    pub lsegs: Option<u64>,
    ///Provides a label for members of a vertically aligned group.
    #[serde(rename = "@vgrp", skip_serializing_if = "Option::is_none")]
    pub vgrp: Option<u64>,
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
    /**Records a vertical adjustment of a feature’s programmatically-determined start
    point.*/
    #[serde(rename = "@startvo", skip_serializing_if = "Option::is_none")]
    pub startvo: Option<crate::generated::data::DataMeasurementsigned>,
    /**Records a vertical adjustment of a feature’s programmatically-determined end
    point.*/
    #[serde(rename = "@endvo", skip_serializing_if = "Option::is_none")]
    pub endvo: Option<crate::generated::data::DataMeasurementsigned>,
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
    ///Encodes the optional 2nd x coordinate.
    #[serde(rename = "@x2", skip_serializing_if = "Option::is_none")]
    pub x2: Option<f64>,
    ///Encodes the optional 2nd y coordinate.
    #[serde(rename = "@y2", skip_serializing_if = "Option::is_none")]
    pub y2: Option<f64>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttPhraseVisCmnCurvedir {
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
///Visual domain attributes.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPhraseVisCmn {
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
    pub curvedir: Option<AttPhraseVisCmnCurvedir>,
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
}

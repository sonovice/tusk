//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Visual domain attributes. The startho and startvo attributes record the horizontal and
vertical offsets of the left end, endho and endvo record the horizontal and vertical offsets
of the right end, and the opening attribute records the width of the opening in staff
inter-line units. The x and y attributes give the absolute coordinates of the left end point,
and x2 and y2 the right end point, of an imaginary line that defines the length of the hairpin
and horizontally bifurcates it. The so-called "pitch" of hairpin may be controlled by use of
the startho, endho, startvo, and endvo attributes, while the placement of the entire rendered
mark may be controlled by use of the ho and vo attributes.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttHairpinVis {
    /**Used to indicate visual appearance. Do not confuse this with the musical term 'color'
    as used in pre-CMN notation.*/
    #[serde(rename = "@color", skip_serializing_if = "Option::is_none")]
    pub color: Option<crate::generated::data::DataColor>,
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
    /**Captures the placement of the item with respect to the staff with which it is
    associated.*/
    #[serde(rename = "@place", skip_serializing_if = "Option::is_none")]
    pub place: Option<crate::generated::data::DataStaffrel>,
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
    /**Specifies the distance between the lines at the open end of a hairpin dynamic
    mark.*/
    #[serde(rename = "@opening", skip_serializing_if = "Option::is_none")]
    pub opening: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Applies to a "Rossini" hairpin, i.e., one where the normally open side is closed by a connecting line.
    #[serde(rename = "@closed", skip_serializing_if = "Option::is_none")]
    pub closed: Option<crate::generated::data::DataBoolean>,
    ///Indicates that the opening points are aligned with an imaginary line that is always 90° perpendicular to the horizontal plane, regardless of any angle or start/end adjustments, including when the hairpin is angled with @angle.optimize or through @endvo/@startvo adjustments.
    #[serde(rename = "@opening.vertical", skip_serializing_if = "Option::is_none")]
    pub opening_vertical: Option<crate::generated::data::DataBoolean>,
    ///Indicates that the slope of the hairpin can be adjusted to follow the content in order to optimize spacing.
    #[serde(rename = "@angle.optimize", skip_serializing_if = "Option::is_none")]
    pub angle_optimize: Option<crate::generated::data::DataBoolean>,
}

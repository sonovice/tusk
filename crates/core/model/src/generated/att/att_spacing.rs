//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that capture notation spacing information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttSpacing {
    ///Describes a note’s spacing relative to its time value.
    #[serde(rename = "@spacing.packexp", skip_serializing_if = "Option::is_none")]
    pub spacing_packexp: Option<f64>,
    ///Describes the note spacing of output.
    #[serde(rename = "@spacing.packfact", skip_serializing_if = "Option::is_none")]
    pub spacing_packfact: Option<f64>,
    /**Specifies the minimum amount of space between adjacent staves in the same system;
    measured from the bottom line of the staff above to the top line of the staff
    below.*/
    #[serde(rename = "@spacing.staff", skip_serializing_if = "Option::is_none")]
    pub spacing_staff: Option<crate::generated::data::DataMeasurementsigned>,
    /**Describes the space between adjacent systems; a pair of space-separated values
    (minimum and maximum, respectively) provides a range between which a rendering
    system-supplied value may fall, while a single value indicates a fixed amount of space;
    that is, the minimum and maximum values are equal.*/
    #[serde(rename = "@spacing.system", skip_serializing_if = "Option::is_none")]
    pub spacing_system: Option<crate::generated::data::DataMeasurementsigned>,
}

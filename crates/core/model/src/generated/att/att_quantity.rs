//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttQuantityUnit {
    ///Byte.
    #[serde(rename = "byte")]
    Byte,
    ///Character.
    #[serde(rename = "char")]
    Char,
    ///Centimeter.
    #[serde(rename = "cm")]
    Cm,
    ///Degree.
    #[serde(rename = "deg")]
    Deg,
    ///Inch.
    #[serde(rename = "in")]
    In,
    ///Serial issue.
    #[serde(rename = "issue")]
    Issue,
    ///Foot.
    #[serde(rename = "ft")]
    Ft,
    ///Meter.
    #[serde(rename = "m")]
    M,
    ///Millimeter.
    #[serde(rename = "mm")]
    Mm,
    ///Page.
    #[serde(rename = "page")]
    Page,
    ///Pica.
    #[serde(rename = "pc")]
    Pc,
    ///Point.
    #[serde(rename = "pt")]
    Pt,
    ///Pixel.
    #[serde(rename = "px")]
    Px,
    ///Radian.
    #[serde(rename = "rad")]
    Rad,
    ///Record.
    #[serde(rename = "record")]
    Record,
    ///Serial volume.
    #[serde(rename = "vol")]
    Vol,
    ///MEI virtual unit.
    #[serde(rename = "vu")]
    Vu,
}
///Attributes that specify a measurement in numerical terms.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttQuantity {
    ///Indicates the unit of measurement.
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<AttQuantityUnit>,
    ///Gives a minimum estimated value for an approximate measurement.
    #[serde(rename = "@atleast", skip_serializing_if = "Option::is_none")]
    pub atleast: Option<f64>,
    ///Gives a maximum estimated value for an approximate measurement.
    #[serde(rename = "@atmost", skip_serializing_if = "Option::is_none")]
    pub atmost: Option<f64>,
    /**Where the measurement summarizes more than one observation or a range of values,
          supplies the minimum value observed.*/
    #[serde(rename = "@min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    /**Where the measurement summarizes more than one observation or a range of values,
          supplies the maximum value observed.*/
    #[serde(rename = "@max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    /**Specifies the degree of statistical confidence (between zero and one) that a value
          falls within the range specified by min and max, or the proportion of observed values that
          fall within that range.*/
    #[serde(rename = "@confidence", skip_serializing_if = "Option::is_none")]
    pub confidence: Option<crate::generated::data::DataConfidence>,
    /**Numeric value capturing a measurement or count. Can only be interpreted in combination
          with the unit attribute.*/
    #[serde(rename = "@quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
}

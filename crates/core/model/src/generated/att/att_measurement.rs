//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttMeasurementUnit {
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
///Attributes that record the unit of measurement in which a value is expressed.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeasurement {
    ///Indicates the unit of measurement.
    #[serde(rename = "@unit", skip_serializing_if = "Option::is_none")]
    pub unit: Option<AttMeasurementUnit>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that record page-level layout information.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttPages {
    /**Specifies the height of the page; may be expressed in real-world units or staff
          steps.*/
    #[serde(rename = "@page.height", skip_serializing_if = "Option::is_none")]
    pub page_height: Option<crate::generated::data::DataMeasurementunsigned>,
    /**Describes the width of the page; may be expressed in real-world units or staff
          steps.*/
    #[serde(rename = "@page.width", skip_serializing_if = "Option::is_none")]
    pub page_width: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the amount of whitespace at the top of a page.
    #[serde(rename = "@page.topmar", skip_serializing_if = "Option::is_none")]
    pub page_topmar: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the amount of whitespace at the bottom of a page.
    #[serde(rename = "@page.botmar", skip_serializing_if = "Option::is_none")]
    pub page_botmar: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the amount of whitespace at the left side of a page.
    #[serde(rename = "@page.leftmar", skip_serializing_if = "Option::is_none")]
    pub page_leftmar: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the amount of whitespace at the right side of a page.
    #[serde(rename = "@page.rightmar", skip_serializing_if = "Option::is_none")]
    pub page_rightmar: Option<crate::generated::data::DataMeasurementunsigned>,
    ///Indicates the number of logical pages to be rendered on a single physical page.
    #[serde(rename = "@page.panels", skip_serializing_if = "Option::is_none")]
    pub page_panels: Option<crate::generated::data::DataPagePanels>,
    ///Indicates how the page should be scaled when rendered.
    #[serde(rename = "@page.scale", skip_serializing_if = "Option::is_none")]
    pub page_scale: Option<crate::generated::data::DataPgscale>,
}

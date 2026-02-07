//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
///Attributes that describe default text typography.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttTextStyle {
    /**Provides a default value for the font family name of text (other than lyrics) when
          this information is not provided on the individual elements.*/
    #[serde(rename = "@text.fam", skip_serializing_if = "Option::is_none")]
    pub text_fam: Option<crate::generated::data::DataFontfamily>,
    /**Provides a default value for the font name of text (other than lyrics) when this
          information is not provided on the individual elements.*/
    #[serde(rename = "@text.name", skip_serializing_if = "Option::is_none")]
    pub text_name: Option<crate::generated::data::DataFontname>,
    /**Provides a default value for the font size of text (other than lyrics) when this
          information is not provided on the individual elements.*/
    #[serde(rename = "@text.size", skip_serializing_if = "Option::is_none")]
    pub text_size: Option<crate::generated::data::DataFontsize>,
    /**Provides a default value for the font style of text (other than lyrics) when this
          information is not provided on the individual elements.*/
    #[serde(rename = "@text.style", skip_serializing_if = "Option::is_none")]
    pub text_style: Option<crate::generated::data::DataFontstyle>,
    /**Provides a default value for the font weight for text (other than lyrics) when this
          information is not provided on the individual elements.*/
    #[serde(rename = "@text.weight", skip_serializing_if = "Option::is_none")]
    pub text_weight: Option<crate::generated::data::DataFontweight>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Used by staffDef and scoreDef to provide default values for attributes in the visual
      domain related to meter signature.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeterSigDefaultVis {
    ///Contains an indication of how the meter signature should be rendered.
    #[serde(rename = "@meter.form", skip_serializing_if = "Option::is_none")]
    pub meter_form: Option<crate::generated::data::DataMeterform>,
    /**Determines whether the old meter signature should be displayed when the meter
          signature changes.*/
    #[serde(rename = "@meter.showchange", skip_serializing_if = "Option::is_none")]
    pub meter_showchange: Option<crate::generated::data::DataBoolean>,
    ///Determines whether the meter signature is to be displayed.
    #[serde(rename = "@meter.visible", skip_serializing_if = "Option::is_none")]
    pub meter_visible: Option<crate::generated::data::DataBoolean>,
}

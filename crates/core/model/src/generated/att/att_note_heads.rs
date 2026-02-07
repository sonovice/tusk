//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttNoteHeadsHeadAuth {
    ///Standard Music Font Layout.
    #[serde(rename = "smufl")]
    Smufl,
}
///Attributes pertaining to the notehead part of a note.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttNoteHeads {
    /**Provides a way of pointing to a user-defined symbol. It must contain a reference to an
          ID of asymbolDefelement elsewhere in the document.*/
    #[serde(rename = "@head.altsym", skip_serializing_if = "Option::is_none")]
    pub head_altsym: Option<crate::generated::data::DataUri>,
    /**A name or label associated with the controlled vocabulary from which a numerical value
          ofhead.shapeis taken.*/
    #[serde(rename = "@head.auth", skip_serializing_if = "Option::is_none")]
    pub head_auth: Option<AttNoteHeadsHeadAuth>,
    ///Captures the overall color of a notehead.
    #[serde(rename = "@head.color", skip_serializing_if = "Option::is_none")]
    pub head_color: Option<crate::generated::data::DataColor>,
    ///Describes how/if the notehead is filled.
    #[serde(rename = "@head.fill", skip_serializing_if = "Option::is_none")]
    pub head_fill: Option<crate::generated::data::DataFill>,
    ///Captures the fill color of a notehead if different from the overall note color.
    #[serde(rename = "@head.fillcolor", skip_serializing_if = "Option::is_none")]
    pub head_fillcolor: Option<crate::generated::data::DataColor>,
    ///Records any additional symbols applied to the notehead.
    #[serde(rename = "@head.mod", default, skip_serializing_if = "Vec::is_empty")]
    pub head_mod: Vec<crate::generated::data::DataNoteheadmodifier>,
    /**Describes rotation applied to the basic notehead shape. A positive value rotates the
          notehead in a counter-clockwise fashion, while negative values produce clockwise
          rotation.*/
    #[serde(rename = "@head.rotation", skip_serializing_if = "Option::is_none")]
    pub head_rotation: Option<crate::generated::data::DataRotation>,
    ///Used to override the head shape normally used for the given duration.
    #[serde(rename = "@head.shape", skip_serializing_if = "Option::is_none")]
    pub head_shape: Option<crate::generated::data::DataHeadshape>,
    /**Indicates if a feature should be rendered when the notation is presented graphically
          or sounded when it is presented in an aural form.*/
    #[serde(rename = "@head.visible", skip_serializing_if = "Option::is_none")]
    pub head_visible: Option<crate::generated::data::DataBoolean>,
}

//! Attribute class: `#ac.ident`
use serde::{Deserialize, Serialize};
/**Logical domain attributes. The n attribute contains a name or number associated with the
      measure (Read, p. 445). Often, this is an integer, but not always. For example, some measures,
      especially incomplete measures or those under an ending mark, may have labels that contain an
      integer plus a suffix, such as '12a'. Measures may even have labels, especially in editorial
      or analytical uses of MEI, that are entirely non-numeric strings. Measure numbers may be
      machine-generated instead of encoding them in the markup. However, an explicit measure number
      should restart numbering with the given value. The join attribute may be used to indicate
      another measure which metrically completes the current, incomplete one.*/
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AttMeasureLog {
    /**Indicates the point of occurrence of this feature along a time line. Its value must be
          the ID of awhenelement elsewhere in the document.*/
    #[serde(rename = "@when", skip_serializing_if = "Option::is_none")]
    pub when: Option<crate::generated::data::DataUri>,
    /**Indicates the relationship between the content of a measure and the prevailing
          meter.*/
    #[serde(rename = "@metcon", skip_serializing_if = "Option::is_none")]
    pub metcon: Option<crate::generated::data::DataBoolean>,
    /**Indicates whether or not a bar line is "controlling"; that is, if it indicates a point
          of alignment across all the parts. Bar lines within a score are usually controlling; that
          is, they "line up". Bar lines within parts may or may not be controlling. When applied tomeasure, this attribute indicates the nature of the right bar line
          but not the left.*/
    #[serde(rename = "@control", skip_serializing_if = "Option::is_none")]
    pub control: Option<crate::generated::data::DataBoolean>,
    /**Indicates the visual rendition of the left bar line. It is present here only for
          facilitation of translation from legacy encodings which use it. Usually, it can be safely
          ignored.*/
    #[serde(rename = "@left", skip_serializing_if = "Option::is_none")]
    pub left: Option<crate::generated::data::DataBarrendition>,
    ///Indicates the function of the right bar line and is structurally important.
    #[serde(rename = "@right", skip_serializing_if = "Option::is_none")]
    pub right: Option<crate::generated::data::DataBarrendition>,
}

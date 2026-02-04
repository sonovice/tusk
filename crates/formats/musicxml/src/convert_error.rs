//! Error types for MusicXML ↔ MEI conversion.

use thiserror::Error;

/// Errors that can occur during conversion.
#[derive(Debug, Error)]
pub enum ConversionError {
    /// An element reference (ID) could not be resolved.
    #[error("unresolved reference: {id} at {location}")]
    UnresolvedReference {
        /// The ID that could not be resolved.
        id: String,
        /// Location where the reference occurred.
        location: String,
    },

    /// A required attribute or element was missing.
    #[error("missing required {kind}: {name} at {location}")]
    MissingRequired {
        /// What kind of thing is missing (e.g., "attribute", "element").
        kind: String,
        /// Name of the missing item.
        name: String,
        /// Location in the document.
        location: String,
    },

    /// An invalid value was encountered.
    #[error("invalid {kind} value: {value} at {location}")]
    InvalidValue {
        /// What kind of value is invalid (e.g., "duration", "pitch", "octave").
        kind: String,
        /// The invalid value (as string).
        value: String,
        /// Location in the document.
        location: String,
    },

    /// An unsupported feature was encountered during conversion.
    #[error("unsupported feature: {feature} at {location}")]
    UnsupportedFeature {
        /// Description of the unsupported feature.
        feature: String,
        /// Location in the document.
        location: String,
    },

    /// The document structure is invalid or unexpected.
    #[error("invalid structure: {message} at {location}")]
    InvalidStructure {
        /// Description of the structural problem.
        message: String,
        /// Location in the document.
        location: String,
    },

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// An XML parsing error occurred.
    #[error("XML error: {0}")]
    Xml(String),
}

impl ConversionError {
    /// Create an unresolved reference error.
    pub fn unresolved_reference(id: impl Into<String>, location: impl Into<String>) -> Self {
        Self::UnresolvedReference {
            id: id.into(),
            location: location.into(),
        }
    }

    /// Create a missing required element/attribute error.
    pub fn missing_required(
        kind: impl Into<String>,
        name: impl Into<String>,
        location: impl Into<String>,
    ) -> Self {
        Self::MissingRequired {
            kind: kind.into(),
            name: name.into(),
            location: location.into(),
        }
    }

    /// Create an invalid value error.
    pub fn invalid_value(
        kind: impl Into<String>,
        value: impl Into<String>,
        location: impl Into<String>,
    ) -> Self {
        Self::InvalidValue {
            kind: kind.into(),
            value: value.into(),
            location: location.into(),
        }
    }

    /// Create an unsupported feature error.
    pub fn unsupported_feature(feature: impl Into<String>, location: impl Into<String>) -> Self {
        Self::UnsupportedFeature {
            feature: feature.into(),
            location: location.into(),
        }
    }

    /// Create an invalid structure error.
    pub fn invalid_structure(message: impl Into<String>, location: impl Into<String>) -> Self {
        Self::InvalidStructure {
            message: message.into(),
            location: location.into(),
        }
    }

    /// Create an XML error.
    pub fn xml(message: impl Into<String>) -> Self {
        Self::Xml(message.into())
    }
}

/// Result type for conversion operations.
pub type ConversionResult<T> = Result<T, ConversionError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unresolved_reference_error() {
        let err = ConversionError::unresolved_reference("note-1", "measure 5");
        let msg = err.to_string();
        assert!(msg.contains("note-1"));
        assert!(msg.contains("measure 5"));
    }

    #[test]
    fn test_missing_required_error() {
        let err = ConversionError::missing_required("attribute", "dur", "note element");
        let msg = err.to_string();
        assert!(msg.contains("attribute"));
        assert!(msg.contains("dur"));
    }

    #[test]
    fn test_invalid_value_error() {
        let err = ConversionError::invalid_value("duration", "invalid", "note element");
        let msg = err.to_string();
        assert!(msg.contains("duration"));
        assert!(msg.contains("invalid"));
    }

    #[test]
    fn test_unsupported_feature_error() {
        let err = ConversionError::unsupported_feature("editorial markup", "apparatus element");
        let msg = err.to_string();
        assert!(msg.contains("editorial markup"));
    }

    #[test]
    fn test_invalid_structure_error() {
        let err = ConversionError::invalid_structure("missing measure", "part P1");
        let msg = err.to_string();
        assert!(msg.contains("missing measure"));
    }

    #[test]
    fn test_xml_error() {
        let err = ConversionError::xml("unexpected end of file");
        let msg = err.to_string();
        assert!(msg.contains("unexpected end of file"));
    }
}

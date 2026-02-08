//! Conversion between versioned MEI types and the **internal** model (tusk_model).
//!
//! The internal model is its own type, not "latest MEI". Versioned MEI (e.g. v6_0_dev)
//! converts **into internal** for import; export is **internal →** chosen versioned type.
//! Today internal is structurally aligned with 6.0-dev, but it can diverge later without
//! being tied to any single MEI version. Currently only v6_0_dev is supported for the
//! versioned side; other versions will be added when versioned parsing is in place.

use thiserror::Error;

use crate::versions::v6_0_dev;

/// Errors that can occur when converting between versioned MEI and internal model.
#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("serde conversion failed: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("unsupported target version for export: {0}")]
    UnsupportedTargetVersion(String),
}

/// A versioned MEI document. Currently only v6_0_dev is supported; other versions
/// (v2_1_1 … v5_1) will be added when their root element and att mapping are generated.
#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum VersionedMei {
    V6_0Dev(v6_0_dev::Mei),
}

/// Convert a versioned MEI document into the internal (tusk_model) representation.
pub fn to_internal(versioned: VersionedMei) -> Result<tusk_model::elements::Mei, ConversionError> {
    match versioned {
        VersionedMei::V6_0Dev(mei) => versioned_to_internal_via_serde(&mei).map_err(ConversionError::Serde),
    }
}

/// Convert the internal MEI document to a specific versioned representation for export.
/// Currently only "v6_0_dev" is supported.
pub fn from_internal(
    mei: &tusk_model::elements::Mei,
    target_version: &str,
) -> Result<VersionedMei, ConversionError> {
    match target_version {
        "v6_0_dev" => {
            let value = serde_json::to_value(mei).map_err(ConversionError::Serde)?;
            let v: v6_0_dev::Mei = serde_json::from_value(value).map_err(ConversionError::Serde)?;
            Ok(VersionedMei::V6_0Dev(v))
        }
        other => Err(ConversionError::UnsupportedTargetVersion(other.to_string())),
    }
}

/// Default versioned target for export: internal is converted to this version when serializing to MEI.
pub const DEFAULT_EXPORT_VERSION: &str = "v6_0_dev";

/// Helper: convert v6_0_dev type into internal via serde roundtrip (today internal is aligned with 6.0-dev shape).
fn versioned_to_internal_via_serde(
    versioned: &v6_0_dev::Mei,
) -> Result<tusk_model::elements::Mei, serde_json::Error> {
    let value = serde_json::to_value(versioned)?;
    let internal: tusk_model::elements::Mei = serde_json::from_value(value)?;
    Ok(internal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal_to_versioned_to_internal_roundtrip() {
        let internal = tusk_model::elements::Mei::default();
        let versioned = from_internal(&internal, DEFAULT_EXPORT_VERSION).expect("from_internal");
        let back = to_internal(versioned).expect("to_internal");
        let value_internal = serde_json::to_value(&internal).unwrap();
        let value_back = serde_json::to_value(&back).unwrap();
        assert_eq!(value_internal, value_back, "internal → versioned → internal roundtrip");
    }

    #[test]
    fn from_internal_unsupported_version_errors() {
        let internal = tusk_model::elements::Mei::default();
        let err = from_internal(&internal, "v99").unwrap_err();
        assert!(matches!(err, ConversionError::UnsupportedTargetVersion(_)));
    }
}

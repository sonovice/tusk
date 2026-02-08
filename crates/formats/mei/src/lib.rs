// Allow lints in generated serializer/deserializer impls
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    clippy::box_default,
    clippy::collapsible_if,
    clippy::match_single_binding,
    clippy::needless_borrow,
    clippy::redundant_closure,
    clippy::single_match
)]

//! MEI parsing and serialization for Tusk.
//!
//! This crate handles reading and writing MEI (Music Encoding Initiative) files.
//!
//! # Supported Versions
//!
//! - MEI 5.1 (primary target)
//! - MEI 5.0 (with migration to 5.1)
//! - MEI 4.0.1 (with migration to 5.1)
//! - MEI 3.0.0 (with migration to 5.1)
//!
//! # Streaming Support
//!
//! For large files (100+ MB operas), use `MeiReader` for chunked processing
//! by `<mdiv>` elements to maintain constant memory usage.
//!
//! # Import/Export
//!
//! This crate provides `import()` and `export()` functions for reading/writing
//! MEI documents. For conversion between MEI and MusicXML formats, see the
//! `tusk-musicxml` crate which contains the conversion logic.
//!
//! ```ignore
//! use tusk_mei::{import, export};
//!
//! // Parse MEI from XML string
//! let xml = r#"<mei xmlns="http://www.music-encoding.org/ns/mei">...</mei>"#;
//! let mei = import(xml)?;
//!
//! // Write MEI to XML string
//! let xml_output = export(&mei)?;
//! ```

pub mod deserializer;
pub mod serializer;
pub mod versions;
pub mod xml_compare;

pub use deserializer::{
    DeserializeConfig, DeserializeError, DeserializeResult, ExtractAttributes, MEI_NAMESPACE,
    MeiDeserialize, MeiReader, extract_namespaces, is_namespace_declaration,
    strip_namespace_prefix,
};
pub use serializer::{
    CollectAttributes, IndentConfig, MeiSerialize, MeiWriter, NamespaceDecl, SerializeConfig,
    SerializeError, SerializeResult, namespaces,
};

/// Import (parse) an MEI document from an XML string.
///
/// This parses an MEI XML string and returns the MEI document.
///
/// # Example
///
/// ```ignore
/// use tusk_mei::import;
///
/// let xml = r#"<mei xmlns="http://www.music-encoding.org/ns/mei">...</mei>"#;
/// let mei = import(xml)?;
/// ```
pub fn import(xml: &str) -> DeserializeResult<tusk_model::elements::Mei> {
    // Note: MeiDeserialize trait must be in scope for from_mei_str()
    <tusk_model::elements::Mei as MeiDeserialize>::from_mei_str(xml)
}

/// Export (serialize) an MEI document to an XML string.
///
/// This serializes an MEI document to an XML string.
///
/// # Example
///
/// ```ignore
/// use tusk_mei::export;
/// use tusk_model::elements::Mei;
///
/// let mei = Mei::default();
/// let xml = export(&mei)?;
/// ```
pub fn export(mei: &tusk_model::elements::Mei) -> SerializeResult<String> {
    // Note: MeiSerialize trait must be in scope for to_mei_string()
    <tusk_model::elements::Mei as MeiSerialize>::to_mei_string(mei)
}

// ---------------------------------------------------------------------------
// Unified format trait implementations
// ---------------------------------------------------------------------------

/// MEI format handler.
///
/// Implements the unified [`tusk_format`] traits so that MEI can be
/// used interchangeably with other formats through the [`FormatRegistry`].
///
/// Since MEI *is* the canonical internal model, import and export are
/// essentially just XML deserialization / serialization — no semantic
/// conversion step is needed.
///
/// [`FormatRegistry`]: tusk_format::FormatRegistry
pub struct MeiFormat;

impl tusk_format::Format for MeiFormat {
    fn id(&self) -> &'static str {
        "mei"
    }

    fn name(&self) -> &'static str {
        "MEI"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &["mei"]
    }

    fn detect(&self, content: &[u8]) -> bool {
        // Only check the first 4 KB for efficiency with large files.
        let prefix = &content[..content.len().min(4096)];
        let s = std::str::from_utf8(prefix).unwrap_or("");
        s.contains("<mei") || s.contains("music-encoding.org")
    }
}

impl tusk_format::Importer for MeiFormat {
    fn import_from_str(&self, input: &str) -> tusk_format::FormatResult<tusk_format::Mei> {
        crate::import(input).map_err(tusk_format::FormatError::parse)
    }
}

impl tusk_format::Exporter for MeiFormat {
    fn export_to_string(&self, mei: &tusk_format::Mei) -> tusk_format::FormatResult<String> {
        crate::export(mei).map_err(tusk_format::FormatError::serialize)
    }
}

#[cfg(all(test, feature = "roundtrip_tests"))]
mod roundtrip_tests;

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;
    use serde::{Deserialize, Serialize};
    use tusk_model::data::DataDurationCmn;

    /// Test struct wrapping a duration enum value (simulates MEI attribute)
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestDuration {
        #[serde(rename = "@dur")]
        dur: DataDurationCmn,
    }

    #[test]
    fn quick_xml_serialize_enum_as_attribute() {
        // MEI uses enums as attribute values, not element content
        let test = TestDuration {
            dur: DataDurationCmn::N4,
        };
        let xml = to_string(&test).expect("should serialize");
        assert!(
            xml.contains("dur=\"4\""),
            "should serialize duration as attribute: {}",
            xml
        );
    }

    #[test]
    fn quick_xml_deserialize_enum_as_attribute() {
        let xml = r#"<TestDuration dur="4"/>"#;
        let parsed: TestDuration = from_str(xml).expect("should deserialize");
        assert_eq!(parsed.dur, DataDurationCmn::N4);
    }

    #[test]
    fn quick_xml_roundtrip_enum_attribute() {
        let original = TestDuration {
            dur: DataDurationCmn::Breve,
        };
        let xml = to_string(&original).expect("should serialize");
        let parsed: TestDuration = from_str(&xml).expect("should deserialize");
        assert_eq!(original, parsed);
    }

    /// Test struct simulating an MEI element with multiple attributes
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "note")]
    struct TestNote {
        #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
        xml_id: Option<String>,
        #[serde(rename = "@dur", skip_serializing_if = "Option::is_none")]
        dur: Option<DataDurationCmn>,
        #[serde(rename = "@label", skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    }

    #[test]
    fn quick_xml_serialize_mei_like_element() {
        let note = TestNote {
            xml_id: Some("n1".to_string()),
            dur: Some(DataDurationCmn::N4),
            label: None,
        };
        let xml = to_string(&note).expect("should serialize");
        assert!(xml.contains("<note"), "should have note element: {}", xml);
        assert!(xml.contains("xml:id=\"n1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("dur=\"4\""), "should have dur: {}", xml);
    }

    #[test]
    fn quick_xml_deserialize_mei_like_element() {
        let xml = r#"<note xml:id="n1" dur="4" label="test"/>"#;
        let note: TestNote = from_str(xml).expect("should deserialize");
        assert_eq!(note.xml_id, Some("n1".to_string()));
        assert_eq!(note.dur, Some(DataDurationCmn::N4));
        assert_eq!(note.label, Some("test".to_string()));
    }

    #[test]
    fn quick_xml_roundtrip_mei_like_element() {
        let original = TestNote {
            xml_id: Some("n1".to_string()),
            dur: Some(DataDurationCmn::N8),
            label: Some("eighth".to_string()),
        };
        let xml = to_string(&original).expect("should serialize");
        let parsed: TestNote = from_str(&xml).expect("should deserialize");
        assert_eq!(original, parsed);
    }

    /// Test struct with child elements (simulates MEI element hierarchy)
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename = "measure")]
    struct TestMeasure {
        #[serde(rename = "@n", skip_serializing_if = "Option::is_none")]
        n: Option<String>,
        #[serde(rename = "note", default)]
        notes: Vec<TestNote>,
    }

    #[test]
    fn quick_xml_serialize_with_children() {
        let measure = TestMeasure {
            n: Some("1".to_string()),
            notes: vec![
                TestNote {
                    xml_id: Some("n1".to_string()),
                    dur: Some(DataDurationCmn::N4),
                    label: None,
                },
                TestNote {
                    xml_id: Some("n2".to_string()),
                    dur: Some(DataDurationCmn::N2),
                    label: None,
                },
            ],
        };
        let xml = to_string(&measure).expect("should serialize");
        assert!(xml.contains("<measure"), "should have measure: {}", xml);
        assert!(xml.contains("<note"), "should have note children: {}", xml);
    }

    #[test]
    fn quick_xml_deserialize_with_children() {
        let xml =
            r#"<measure n="1"><note xml:id="n1" dur="4"/><note xml:id="n2" dur="2"/></measure>"#;
        let measure: TestMeasure = from_str(xml).expect("should deserialize");
        assert_eq!(measure.n, Some("1".to_string()));
        assert_eq!(measure.notes.len(), 2);
        assert_eq!(measure.notes[0].xml_id, Some("n1".to_string()));
        assert_eq!(measure.notes[1].dur, Some(DataDurationCmn::N2));
    }

    // NOTE: The generated MEI model types use #[serde(flatten)] for attribute classes,
    // which requires custom serialization logic (task 1.4.2). The tests above verify
    // that quick-xml with the 'serialize' feature is properly configured and functional.
}

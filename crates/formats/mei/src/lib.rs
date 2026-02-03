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

pub mod serializer;
pub mod versions;

pub use serializer::{MeiSerialize, MeiWriter, SerializeConfig, SerializeError, SerializeResult};

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;
    use serde::{Deserialize, Serialize};
    use tusk_model::data::DataDurationCmn;

    #[test]
    fn crate_compiles() {
        assert!(true);
    }

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

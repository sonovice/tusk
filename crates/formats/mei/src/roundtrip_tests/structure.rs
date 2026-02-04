//! Round-trip serialization tests for structural MEI elements.
//!
//! Tests for Measure, Staff, Layer, Section, Mdiv elements and their hierarchical relationships.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Measure Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_measure() {
    use tusk_model::elements::Measure;

    let original = Measure::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.measure_log.right.is_none());
    assert!(parsed.measure_log.left.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_measure_with_xml_id() {
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"m1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
}

#[test]
fn roundtrip_measure_with_n_attribute() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());
    original.common.n = Some(DataWord("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("n=\"1\""), "xml should contain n: {}", xml);

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));
}

#[test]
fn roundtrip_measure_with_barline_right() {
    use tusk_model::data::DataBarrendition;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());
    original.measure_log.right = Some(DataBarrendition::Dbl);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("right=\"dbl\""),
        "xml should contain right: {}",
        xml
    );

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.measure_log.right, Some(DataBarrendition::Dbl));
}

#[test]
fn roundtrip_measure_with_barline_left() {
    use tusk_model::data::DataBarrendition;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_log.left = Some(DataBarrendition::Rptstart);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.measure_log.left, Some(DataBarrendition::Rptstart));
}

#[test]
fn roundtrip_measure_with_metcon() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_log.metcon = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.measure_log.metcon, Some(DataBoolean::True));
}

#[test]
fn roundtrip_measure_with_control() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_log.control = Some(DataBoolean::False);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.measure_log.control, Some(DataBoolean::False));
}

#[test]
fn roundtrip_measure_with_visual_width() {
    use tusk_model::data::DataMeasurementunsigned;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_vis.width = Some(DataMeasurementunsigned("100vu".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.measure_vis.width,
        Some(DataMeasurementunsigned("100vu".to_string()))
    );
}

#[test]
fn roundtrip_measure_with_bar_len() {
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_vis.bar_len = Some(8.0);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.measure_vis.bar_len, Some(8.0));
}

#[test]
fn roundtrip_measure_with_gestural_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Measure;

    let mut original = Measure::default();
    original.measure_ges.tstamp_ges = Some(DataBeat(0.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.measure_ges.tstamp_ges.is_some());
}

#[test]
fn roundtrip_measure_with_staff_child() {
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("s1".to_string());
    staff.n_integer.n = Some(1);

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());
    original.children.push(MeasureChild::Staff(Box::new(staff)));

    let xml = original.to_mei_string().expect("serialize");

    // Verify the serialized XML contains the staff child
    assert!(
        xml.contains("<staff"),
        "should contain staff element: {}",
        xml
    );
    assert!(
        xml.contains("</measure>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MeasureChild::Staff(staff) => {
            assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn roundtrip_measure_with_multiple_staff_children() {
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff1 = Staff::default();
    staff1.basic.xml_id = Some("s1".to_string());
    staff1.n_integer.n = Some(1);

    let mut staff2 = Staff::default();
    staff2.basic.xml_id = Some("s2".to_string());
    staff2.n_integer.n = Some(2);

    let mut original = Measure::default();
    original.common.xml_id = Some("m1".to_string());
    original
        .children
        .push(MeasureChild::Staff(Box::new(staff1)));
    original
        .children
        .push(MeasureChild::Staff(Box::new(staff2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);
}

#[test]
fn roundtrip_measure_complete_cmn() {
    use tusk_model::data::{DataBarrendition, DataBoolean, DataWord};
    use tusk_model::elements::Measure;

    // Common Music Notation measure with all typical attributes
    let mut original = Measure::default();
    original.common.xml_id = Some("m42".to_string());
    original.common.n = Some(DataWord("42".to_string()));
    original.measure_log.right = Some(DataBarrendition::Single);
    original.measure_log.metcon = Some(DataBoolean::True);
    original.measure_log.control = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.common.n, original.common.n);
    assert_eq!(parsed.measure_log.right, original.measure_log.right);
    assert_eq!(parsed.measure_log.metcon, original.measure_log.metcon);
    assert_eq!(parsed.measure_log.control, original.measure_log.control);
}

#[test]
fn measure_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Measure;

    let xml = r#"<measure xml:id="m1" unknown="value" n="1"/>"#;
    let measure = Measure::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
}

#[test]
fn measure_ignores_unknown_child_elements() {
    use tusk_model::elements::Measure;

    let xml = r#"<measure xml:id="m1"><unknownElement/></measure>"#;
    let measure = Measure::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
    assert!(measure.children.is_empty());
}

#[test]
fn measure_deserializes_with_xml_declaration() {
    use tusk_model::elements::Measure;

    let xml = r#"<?xml version="1.0"?><measure xml:id="m1" n="1"/>"#;
    let measure = Measure::from_mei_str(xml).expect("should deserialize");

    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
}

// ============================================================================
// Staff Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_staff() {
    use tusk_model::elements::Staff;

    let original = Staff::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n_integer.n.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_staff_with_xml_id() {
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.basic.xml_id = Some("s1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"s1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Staff::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.basic.xml_id, Some("s1".to_string()));
}

#[test]
fn roundtrip_staff_with_n_attribute() {
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.basic.xml_id = Some("s1".to_string());
    original.n_integer.n = Some(1);

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("n=\"1\""), "xml should contain n: {}", xml);

    let parsed = Staff::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn roundtrip_staff_with_label() {
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.labelled.label = Some("Violin I".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.labelled.label, Some("Violin I".to_string()));
}

#[test]
fn roundtrip_staff_with_visible_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.staff_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.staff_vis.visible, Some(DataBoolean::True));
}

#[test]
fn roundtrip_staff_with_def_attribute() {
    use tusk_model::elements::Staff;

    let mut original = Staff::default();
    original.staff_log.def = Some(tusk_model::data::DataUri("staffdef1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.staff_log.def.is_some());
}

#[test]
fn roundtrip_staff_with_metcon() {
    use tusk_model::elements::Staff;

    // Parse from XML to test metcon attribute deserialization
    let xml = r#"<staff n="1" metcon="c" />"#;
    let parsed = Staff::from_mei_str(xml).expect("deserialize");

    assert!(parsed.staff_log.metcon.is_some());

    // Serialize and verify round-trip
    let reserialized = parsed.to_mei_string().expect("serialize");
    assert!(
        reserialized.contains("metcon=\"c\""),
        "metcon should be preserved: {}",
        reserialized
    );
}

#[test]
fn roundtrip_staff_complete_cmn() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Staff;

    // Common Music Notation staff with all typical attributes
    let mut original = Staff::default();
    original.basic.xml_id = Some("s1".to_string());
    original.n_integer.n = Some(1);
    original.labelled.label = Some("Piano".to_string());
    original.staff_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, original.basic.xml_id);
    assert_eq!(parsed.n_integer.n, original.n_integer.n);
    assert_eq!(parsed.labelled.label, original.labelled.label);
    assert_eq!(parsed.staff_vis.visible, original.staff_vis.visible);
}

#[test]
fn staff_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff xml:id="s1" unknown="value" n="1"/>"#;
    let staff = Staff::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
    assert_eq!(staff.n_integer.n, Some(1));
}

#[test]
fn staff_deserializes_with_xml_declaration() {
    use tusk_model::elements::Staff;

    let xml = r#"<?xml version="1.0"?><staff xml:id="s1" n="1"/>"#;
    let staff = Staff::from_mei_str(xml).expect("should deserialize");

    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
}

#[test]
fn staff_ignores_unknown_child_elements() {
    use tusk_model::elements::Staff;

    // Staff with unknown child element should parse gracefully
    let xml = r#"<staff xml:id="s1"><unknownElement/></staff>"#;
    let staff = Staff::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
    // Children should be empty since we skip unknown children
    assert!(staff.children.is_empty());
}

// ============================================================================
// Staff External XML Parsing Tests
// ============================================================================

#[test]
fn parse_external_staff_minimal() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff/>"#;
    let parsed = Staff::from_mei_str(xml).expect("deserialize");

    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Staff::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.basic.xml_id.is_none());
}

#[test]
fn parse_external_staff_with_attributes() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff xml:id="s1" n="1"/>"#;
    let parsed = Staff::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("s1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Staff::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.basic.xml_id, Some("s1".to_string()));
    assert_eq!(reparsed.n_integer.n, Some(1));
}

#[test]
fn parse_external_staff_various_n_values() {
    use tusk_model::elements::Staff;

    // Test various staff numbers
    for n in 1u64..=10 {
        let xml = format!(r#"<staff n="{}"/>"#, n);
        let parsed = Staff::from_mei_str(&xml).expect("should parse");
        assert_eq!(parsed.n_integer.n, Some(n));

        let reserialized = parsed.to_mei_string().expect("re-serialize");
        let reparsed = Staff::from_mei_str(&reserialized).expect("re-deserialize");
        assert_eq!(reparsed.n_integer.n, Some(n));
    }
}

// ============================================================================
// Tests from MEI Example Files
// ============================================================================

/// Staff from tempo-01.mei
#[test]
fn mei_example_tempo01_staff() {
    use tusk_model::elements::Staff;

    // From specs/mei/examples/verovio/tempo-01.mei
    // Note: Layer children are not yet parsed (next task), so we just verify attributes
    let xml = r#"<staff n="1">
                <layer n="1">
                  <note dots="1" dur="4" oct="5" pname="g" />
                  <note dur="8" oct="5" pname="g" />
                </layer>
              </staff>"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert_eq!(staff.n_integer.n, Some(1));
    // Children are skipped for now until layer parsing is implemented
}

/// Staff with multiple layers (from Tchaikovsky example pattern)
#[test]
fn mei_example_staff_structure() {
    use tusk_model::elements::Staff;

    // Structure from typical CMN MEI files
    let xml = r#"<staff xml:id="s1" n="1" label="Piano Right Hand">
        <layer n="1">
            <note dur="4" pname="c" oct="5"/>
        </layer>
    </staff>"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
    assert_eq!(staff.n_integer.n, Some(1));
    assert_eq!(staff.labelled.label, Some("Piano Right Hand".to_string()));
}

/// Self-closing staff element
#[test]
fn mei_example_staff_self_closing() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff xml:id="s1" n="1" />"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
    assert_eq!(staff.n_integer.n, Some(1));
    assert!(staff.children.is_empty());
}

/// Staff without xml:id (common pattern)
#[test]
fn mei_example_staff_without_id() {
    use tusk_model::elements::Staff;

    let xml = r#"<staff n="2" />"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert!(staff.basic.xml_id.is_none());
    assert_eq!(staff.n_integer.n, Some(2));
}

/// Staff visibility attribute
#[test]
fn mei_example_staff_hidden() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Staff;

    let xml = r#"<staff n="1" visible="false" />"#;

    let staff = Staff::from_mei_str(xml).expect("should parse");

    assert_eq!(staff.staff_vis.visible, Some(DataBoolean::False));
}

// ============================================================================
// Staff in Measure Context Tests
// ============================================================================

/// Test that Staff parsed as child of Measure round-trips correctly
#[test]
fn roundtrip_staff_in_measure_context() {
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("s1".to_string());
    staff.n_integer.n = Some(1);
    staff.labelled.label = Some("Violin".to_string());

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let xml = measure.to_mei_string().expect("serialize");

    // Verify structure
    assert!(xml.contains("<measure"), "should have measure: {}", xml);
    assert!(xml.contains("<staff"), "should have staff: {}", xml);
    assert!(
        xml.contains("label=\"Violin\""),
        "should have label: {}",
        xml
    );

    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MeasureChild::Staff(staff) => {
            assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
            assert_eq!(staff.n_integer.n, Some(1));
            assert_eq!(staff.labelled.label, Some("Violin".to_string()));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

/// Multiple staves in a measure
#[test]
fn roundtrip_multiple_staves_in_measure() {
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff1 = Staff::default();
    staff1.basic.xml_id = Some("s1".to_string());
    staff1.n_integer.n = Some(1);
    staff1.labelled.label = Some("Violin I".to_string());

    let mut staff2 = Staff::default();
    staff2.basic.xml_id = Some("s2".to_string());
    staff2.n_integer.n = Some(2);
    staff2.labelled.label = Some("Violin II".to_string());

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.children.push(MeasureChild::Staff(Box::new(staff1)));
    measure.children.push(MeasureChild::Staff(Box::new(staff2)));

    let xml = measure.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // First staff
    match &parsed.children[0] {
        MeasureChild::Staff(staff) => {
            assert_eq!(staff.n_integer.n, Some(1));
            assert_eq!(staff.labelled.label, Some("Violin I".to_string()));
        }
        other => panic!("Expected Staff 1, got {:?}", other),
    }

    // Second staff
    match &parsed.children[1] {
        MeasureChild::Staff(staff) => {
            assert_eq!(staff.n_integer.n, Some(2));
            assert_eq!(staff.labelled.label, Some("Violin II".to_string()));
        }
        other => panic!("Expected Staff 2, got {:?}", other),
    }
}

// ============================================================================
// Layer Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_layer() {
    use tusk_model::elements::Layer;

    let original = Layer::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n_integer.n.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_layer_with_xml_id() {
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.basic.xml_id = Some("l1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"l1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Layer::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.basic.xml_id, Some("l1".to_string()));
}

#[test]
fn roundtrip_layer_with_n_attribute() {
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.basic.xml_id = Some("l1".to_string());
    original.n_integer.n = Some(1);

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("n=\"1\""), "xml should contain n: {}", xml);

    let parsed = Layer::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn roundtrip_layer_with_label() {
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.labelled.label = Some("Voice 1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.labelled.label, Some("Voice 1".to_string()));
}

#[test]
fn roundtrip_layer_with_visible_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.layer_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.layer_vis.visible, Some(DataBoolean::True));
}

#[test]
fn roundtrip_layer_with_def_attribute() {
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.layer_log.def = Some(tusk_model::data::DataUri("layerdef1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.layer_log.def.is_some());
}

#[test]
fn roundtrip_layer_with_metcon() {
    use tusk_model::att::AttLayerLogMetcon;
    use tusk_model::elements::Layer;

    // Parse from XML to test metcon attribute deserialization
    let xml = r#"<layer n="1" metcon="c" />"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.layer_log.metcon, Some(AttLayerLogMetcon::C));

    // Serialize and verify round-trip
    let reserialized = parsed.to_mei_string().expect("serialize");
    assert!(
        reserialized.contains("metcon=\"c\""),
        "metcon should be preserved: {}",
        reserialized
    );
}

#[test]
fn roundtrip_layer_with_cue() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Layer;

    let mut original = Layer::default();
    original.layer_log.cue = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("cue=\"true\""),
        "should contain cue attribute: {}",
        xml
    );

    let parsed = Layer::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.layer_log.cue, Some(DataBoolean::True));
}

#[test]
fn roundtrip_layer_complete_cmn() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Layer;

    // Common Music Notation layer with all typical attributes
    let mut original = Layer::default();
    original.basic.xml_id = Some("l1".to_string());
    original.n_integer.n = Some(1);
    original.labelled.label = Some("Voice 1".to_string());
    original.layer_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, original.basic.xml_id);
    assert_eq!(parsed.n_integer.n, original.n_integer.n);
    assert_eq!(parsed.labelled.label, original.labelled.label);
    assert_eq!(parsed.layer_vis.visible, original.layer_vis.visible);
}

#[test]
fn layer_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer xml:id="l1" unknown="value" n="1"/>"#;
    let layer = Layer::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(layer.basic.xml_id, Some("l1".to_string()));
    assert_eq!(layer.n_integer.n, Some(1));
}

#[test]
fn layer_deserializes_with_xml_declaration() {
    use tusk_model::elements::Layer;

    let xml = r#"<?xml version="1.0"?><layer xml:id="l1" n="1"/>"#;
    let layer = Layer::from_mei_str(xml).expect("should deserialize");

    assert_eq!(layer.basic.xml_id, Some("l1".to_string()));
}

#[test]
fn layer_ignores_unknown_child_elements() {
    use tusk_model::elements::Layer;

    // Layer with unknown child element should parse gracefully
    let xml = r#"<layer xml:id="l1"><unknownElement/></layer>"#;
    let layer = Layer::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(layer.basic.xml_id, Some("l1".to_string()));
    // Children should be empty since we skip unknown children
    assert!(layer.children.is_empty());
}

// ============================================================================
// Layer with Child Elements Tests
// ============================================================================

#[test]
fn roundtrip_layer_with_note_child() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note};

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));

    let mut original = Layer::default();
    original.basic.xml_id = Some("l1".to_string());
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Note(Box::new(note)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("l1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Note(note) => {
            assert_eq!(note.common.xml_id, Some("n1".to_string()));
            assert_eq!(
                note.note_log.dur,
                Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
            );
        }
        other => panic!("Expected Note child, got {:?}", other),
    }
}

#[test]
fn roundtrip_layer_with_rest_child() {
    use tusk_model::data::{DataDurationCmn, DataDurationrests};
    use tusk_model::elements::{Layer, LayerChild, Rest};

    let mut rest = Rest::default();
    rest.common.xml_id = Some("r1".to_string());
    rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));

    let mut original = Layer::default();
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Rest(Box::new(rest)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Rest(rest) => {
            assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        }
        other => panic!("Expected Rest child, got {:?}", other),
    }
}

#[test]
fn roundtrip_layer_with_chord_child() {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_model::elements::{Chord, Layer, LayerChild};

    let mut chord = Chord::default();
    chord.common.xml_id = Some("c1".to_string());
    chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut original = Layer::default();
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Chord(Box::new(chord)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Chord(chord) => {
            assert_eq!(chord.common.xml_id, Some("c1".to_string()));
        }
        other => panic!("Expected Chord child, got {:?}", other),
    }
}

#[test]
fn roundtrip_layer_with_space_child() {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_model::elements::{Layer, LayerChild, Space};

    let mut space = Space::default();
    space.common.xml_id = Some("s1".to_string());
    space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut original = Layer::default();
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Space(Box::new(space)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Space(space) => {
            assert_eq!(space.common.xml_id, Some("s1".to_string()));
        }
        other => panic!("Expected Space child, got {:?}", other),
    }
}

#[test]
fn roundtrip_layer_with_multiple_children() {
    use tusk_model::data::{
        DataDuration, DataDurationCmn, DataDurationrests, DataOctave, DataPitchname,
    };
    use tusk_model::elements::{Layer, LayerChild, Note, Rest};

    let mut note1 = Note::default();
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));

    let mut note2 = Note::default();
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note2.note_log.pname = Some(DataPitchname::from("d".to_string()));
    note2.note_log.oct = Some(DataOctave(4));

    let mut rest = Rest::default();
    rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N2));

    let mut original = Layer::default();
    original.n_integer.n = Some(1);
    original.children.push(LayerChild::Note(Box::new(note1)));
    original.children.push(LayerChild::Note(Box::new(note2)));
    original.children.push(LayerChild::Rest(Box::new(rest)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 3);

    // Verify order is preserved
    match &parsed.children[0] {
        LayerChild::Note(note) => {
            assert_eq!(
                note.note_log.pname,
                Some(DataPitchname::from("c".to_string()))
            );
        }
        other => panic!("Expected Note 1, got {:?}", other),
    }

    match &parsed.children[1] {
        LayerChild::Note(note) => {
            assert_eq!(
                note.note_log.pname,
                Some(DataPitchname::from("d".to_string()))
            );
        }
        other => panic!("Expected Note 2, got {:?}", other),
    }

    match &parsed.children[2] {
        LayerChild::Rest(_) => {}
        other => panic!("Expected Rest, got {:?}", other),
    }
}

// ============================================================================
// Layer External XML Parsing Tests
// ============================================================================

#[test]
fn parse_external_layer_minimal() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer/>"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Layer::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.basic.xml_id.is_none());
}

#[test]
fn parse_external_layer_with_attributes() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer xml:id="l1" n="1"/>"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("l1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Layer::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.basic.xml_id, Some("l1".to_string()));
    assert_eq!(reparsed.n_integer.n, Some(1));
}

#[test]
fn parse_external_layer_various_n_values() {
    use tusk_model::elements::Layer;

    for n in [1, 2, 3, 10] {
        let xml = format!(r#"<layer n="{}"/>"#, n);
        let parsed = Layer::from_mei_str(&xml).expect("deserialize");
        assert_eq!(parsed.n_integer.n, Some(n));
    }
}

#[test]
fn mei_example_layer_structure() {
    use tusk_model::elements::{Layer, LayerChild};

    // Based on specs/mei/examples/usersymbols/usersymbols-sample347.txt
    let xml = r#"<layer n="1">
        <rest dur="4" xml:id="r1"/>
        <note dur="8" oct="4" pname="c" xml:id="n1"/>
    </layer>"#;

    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 2);

    // First child should be rest
    match &parsed.children[0] {
        LayerChild::Rest(rest) => {
            assert_eq!(rest.common.xml_id, Some("r1".to_string()));
        }
        other => panic!("Expected Rest, got {:?}", other),
    }

    // Second child should be note
    match &parsed.children[1] {
        LayerChild::Note(note) => {
            assert_eq!(note.common.xml_id, Some("n1".to_string()));
        }
        other => panic!("Expected Note, got {:?}", other),
    }
}

#[test]
fn mei_example_layer_self_closing() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer n="1"/>"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
    assert!(parsed.children.is_empty());
}

#[test]
fn mei_example_layer_without_id() {
    use tusk_model::elements::Layer;

    let xml = r#"<layer n="2"><rest dur="4"/></layer>"#;
    let parsed = Layer::from_mei_str(xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert_eq!(parsed.n_integer.n, Some(2));
    assert_eq!(parsed.children.len(), 1);
}

#[test]
fn roundtrip_layer_in_staff_context() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note, Staff, StaffChild};

    // Create a note
    let mut note = Note::default();
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));

    // Create a layer containing the note
    let mut layer = Layer::default();
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    // Create a staff containing the layer
    let mut staff = Staff::default();
    staff.basic.xml_id = Some("s1".to_string());
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let xml = staff.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("s1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        StaffChild::Layer(layer) => {
            assert_eq!(layer.n_integer.n, Some(1));
            assert_eq!(layer.children.len(), 1);

            match &layer.children[0] {
                LayerChild::Note(note) => {
                    assert_eq!(
                        note.note_log.pname,
                        Some(DataPitchname::from("c".to_string()))
                    );
                }
                other => panic!("Expected Note, got {:?}", other),
            }
        }
        other => panic!("Expected Layer, got {:?}", other),
    }
}

#[test]
fn roundtrip_multiple_layers_in_staff() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note, Staff, StaffChild};

    // First layer with note
    let mut note1 = Note::default();
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));

    let mut layer1 = Layer::default();
    layer1.n_integer.n = Some(1);
    layer1.children.push(LayerChild::Note(Box::new(note1)));

    // Second layer with different note
    let mut note2 = Note::default();
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2));
    note2.note_log.pname = Some(DataPitchname::from("g".to_string()));
    note2.note_log.oct = Some(DataOctave(3));

    let mut layer2 = Layer::default();
    layer2.n_integer.n = Some(2);
    layer2.children.push(LayerChild::Note(Box::new(note2)));

    // Staff with both layers
    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer1)));
    staff.children.push(StaffChild::Layer(Box::new(layer2)));

    let xml = staff.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // First layer
    match &parsed.children[0] {
        StaffChild::Layer(layer) => {
            assert_eq!(layer.n_integer.n, Some(1));
        }
        other => panic!("Expected Layer 1, got {:?}", other),
    }

    // Second layer
    match &parsed.children[1] {
        StaffChild::Layer(layer) => {
            assert_eq!(layer.n_integer.n, Some(2));
        }
        other => panic!("Expected Layer 2, got {:?}", other),
    }
}

// ============================================================================
// Section Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_section() {
    use tusk_model::elements::Section;

    let original = Section::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_section_with_xml_id() {
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"sec1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Section::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
}

#[test]
fn roundtrip_section_with_n_attribute() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original.common.n = Some(DataWord("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains("n=\"1\""), "xml should contain n: {}", xml);

    let parsed = Section::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));
}

#[test]
fn roundtrip_section_with_label() {
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.common.label = Some("Introduction".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("Introduction".to_string()));
}

#[test]
fn roundtrip_section_with_restart_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.section_vis.restart = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.section_vis.restart, Some(DataBoolean::True));
}

#[test]
fn roundtrip_section_with_attacca_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Section;

    let mut original = Section::default();
    original.section_ges.attacca = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.section_ges.attacca, Some(DataBoolean::True));
}

#[test]
fn roundtrip_section_with_measure_child() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, Section, SectionChild};

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    let xml = original.to_mei_string().expect("serialize");

    assert!(xml.contains("<section"), "should have section: {}", xml);
    assert!(xml.contains("<measure"), "should have measure: {}", xml);
    assert!(
        xml.contains("</section>"),
        "should have closing tag: {}",
        xml
    );

    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Measure(measure) => {
            assert_eq!(measure.common.xml_id, Some("m1".to_string()));
        }
        other => panic!("Expected Measure, got {:?}", other),
    }
}

#[test]
fn roundtrip_section_with_multiple_measure_children() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, Section, SectionChild};

    let mut measure1 = Measure::default();
    measure1.common.xml_id = Some("m1".to_string());
    measure1.common.n = Some(DataWord("1".to_string()));

    let mut measure2 = Measure::default();
    measure2.common.xml_id = Some("m2".to_string());
    measure2.common.n = Some(DataWord("2".to_string()));

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original
        .children
        .push(SectionChild::Measure(Box::new(measure1)));
    original
        .children
        .push(SectionChild::Measure(Box::new(measure2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);
}

#[test]
fn roundtrip_section_with_staff_child() {
    use tusk_model::elements::{Section, SectionChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("s1".to_string());
    staff.n_integer.n = Some(1);

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original.children.push(SectionChild::Staff(Box::new(staff)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Staff(staff) => {
            assert_eq!(staff.basic.xml_id, Some("s1".to_string()));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn roundtrip_section_with_nested_section() {
    use tusk_model::elements::{Section, SectionChild};

    let mut inner_section = Section::default();
    inner_section.common.xml_id = Some("sec2".to_string());

    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original
        .children
        .push(SectionChild::Section(Box::new(inner_section)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Section(section) => {
            assert_eq!(section.common.xml_id, Some("sec2".to_string()));
        }
        other => panic!("Expected nested Section, got {:?}", other),
    }
}

#[test]
fn roundtrip_section_complete_cmn() {
    use tusk_model::data::{DataBoolean, DataWord};
    use tusk_model::elements::Section;

    // Common Music Notation section with all typical attributes
    let mut original = Section::default();
    original.common.xml_id = Some("sec1".to_string());
    original.common.n = Some(DataWord("1".to_string()));
    original.common.label = Some("First Section".to_string());
    original.section_vis.restart = Some(DataBoolean::False);
    original.section_ges.attacca = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.common.n, original.common.n);
    assert_eq!(parsed.common.label, original.common.label);
    assert_eq!(parsed.section_vis.restart, original.section_vis.restart);
    assert_eq!(parsed.section_ges.attacca, original.section_ges.attacca);
}

#[test]
fn section_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Section;

    let xml = r#"<section xml:id="sec1" unknown="value" n="1"/>"#;
    let section = Section::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
}

#[test]
fn section_ignores_unknown_child_elements() {
    use tusk_model::elements::Section;

    let xml = r#"<section xml:id="sec1"><unknownElement/></section>"#;
    let section = Section::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
    assert!(section.children.is_empty());
}

#[test]
fn section_deserializes_with_xml_declaration() {
    use tusk_model::elements::Section;

    let xml = r#"<?xml version="1.0"?><section xml:id="sec1" n="1"/>"#;
    let section = Section::from_mei_str(xml).expect("should deserialize");

    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
}

// ============================================================================
// Section External XML Parsing Tests
// ============================================================================

#[test]
fn parse_external_section_minimal() {
    use tusk_model::elements::Section;

    let xml = r#"<section/>"#;
    let parsed = Section::from_mei_str(xml).expect("deserialize");

    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Section::from_mei_str(&reserialized).expect("re-deserialize");

    assert!(reparsed.common.xml_id.is_none());
}

#[test]
fn parse_external_section_with_attributes() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Section;

    let xml = r#"<section xml:id="sec1" n="1"/>"#;
    let parsed = Section::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));

    // Verify round-trip preserves values
    let reserialized = parsed.to_mei_string().expect("re-serialize");
    let reparsed = Section::from_mei_str(&reserialized).expect("re-deserialize");

    assert_eq!(reparsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(reparsed.common.n, Some(DataWord("1".to_string())));
}

#[test]
fn mei_example_section_structure() {
    use tusk_model::elements::{Section, SectionChild};

    // Basic section structure
    let xml = r#"<section xml:id="section1" label="Movement I">
        <measure xml:id="m1" n="1"/>
        <measure xml:id="m2" n="2"/>
    </section>"#;

    let parsed = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(parsed.common.xml_id, Some("section1".to_string()));
    assert_eq!(parsed.common.label, Some("Movement I".to_string()));
    assert_eq!(parsed.children.len(), 2);

    // First measure
    match &parsed.children[0] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m1".to_string()));
        }
        other => panic!("Expected Measure 1, got {:?}", other),
    }

    // Second measure
    match &parsed.children[1] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m2".to_string()));
        }
        other => panic!("Expected Measure 2, got {:?}", other),
    }
}

#[test]
fn mei_example_section_self_closing() {
    use tusk_model::elements::Section;

    let xml = r#"<section xml:id="sec1" n="1" />"#;
    let section = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
    assert!(section.children.is_empty());
}

#[test]
fn mei_example_section_without_id() {
    use tusk_model::elements::Section;

    let xml = r#"<section n="2" />"#;
    let section = Section::from_mei_str(xml).expect("should parse");

    assert!(section.common.xml_id.is_none());
}

#[test]
fn mei_example_section_with_restart() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Section;

    // Section with restart attribute (indicates staves restart)
    let xml = r#"<section xml:id="sec1" restart="true" />"#;
    let section = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(section.section_vis.restart, Some(DataBoolean::True));
}

#[test]
fn mei_example_section_with_attacca() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Section;

    // Section with attacca attribute (indicates next section should begin immediately)
    let xml = r#"<section xml:id="sec1" attacca="true" />"#;
    let section = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(section.section_ges.attacca, Some(DataBoolean::True));
}

// ============================================================================
// Mdiv Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_mdiv() {
    use tusk_model::elements::Mdiv;

    let original = Mdiv::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_mdiv_with_xml_id() {
    use tusk_model::elements::Mdiv;

    let mut original = Mdiv::default();
    original.common.xml_id = Some("mdiv1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"mdiv1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("mdiv1".to_string()));
}

#[test]
fn roundtrip_mdiv_with_n_attribute() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Mdiv;

    let mut original = Mdiv::default();
    original.common.n = Some(DataWord("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));
}

#[test]
fn roundtrip_mdiv_with_label() {
    use tusk_model::elements::Mdiv;

    let mut original = Mdiv::default();
    original.common.label = Some("Movement I".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.label, Some("Movement I".to_string()));
}

#[test]
fn roundtrip_mdiv_with_attacca_attribute() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Mdiv;

    let mut original = Mdiv::default();
    original.mdiv_ges.attacca = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.mdiv_ges.attacca, Some(DataBoolean::True));
}

#[test]
fn roundtrip_mdiv_with_nested_mdiv() {
    use tusk_model::elements::{Mdiv, MdivChild};

    let mut inner_mdiv = Mdiv::default();
    inner_mdiv.common.xml_id = Some("mdiv2".to_string());
    inner_mdiv.common.label = Some("Movement I-A".to_string());

    let mut original = Mdiv::default();
    original.common.xml_id = Some("mdiv1".to_string());
    original
        .children
        .push(MdivChild::Mdiv(Box::new(inner_mdiv)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mdiv1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MdivChild::Mdiv(nested) => {
            assert_eq!(nested.common.xml_id, Some("mdiv2".to_string()));
            assert_eq!(nested.common.label, Some("Movement I-A".to_string()));
        }
        other => panic!("Expected nested Mdiv, got {:?}", other),
    }
}

#[test]
fn roundtrip_mdiv_with_multiple_nested_mdivs() {
    use tusk_model::elements::{Mdiv, MdivChild};

    let mut mdiv_a = Mdiv::default();
    mdiv_a.common.xml_id = Some("mdiv-a".to_string());

    let mut mdiv_b = Mdiv::default();
    mdiv_b.common.xml_id = Some("mdiv-b".to_string());

    let mut original = Mdiv::default();
    original.common.xml_id = Some("mdiv-root".to_string());
    original.children.push(MdivChild::Mdiv(Box::new(mdiv_a)));
    original.children.push(MdivChild::Mdiv(Box::new(mdiv_b)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        MdivChild::Mdiv(nested) => {
            assert_eq!(nested.common.xml_id, Some("mdiv-a".to_string()));
        }
        other => panic!("Expected Mdiv, got {:?}", other),
    }

    match &parsed.children[1] {
        MdivChild::Mdiv(nested) => {
            assert_eq!(nested.common.xml_id, Some("mdiv-b".to_string()));
        }
        other => panic!("Expected Mdiv, got {:?}", other),
    }
}

#[test]
fn roundtrip_mdiv_complete_cmn() {
    use tusk_model::data::{DataBoolean, DataWord};
    use tusk_model::elements::Mdiv;

    // Common Music Notation mdiv with all typical attributes
    let mut original = Mdiv::default();
    original.common.xml_id = Some("mdiv1".to_string());
    original.common.n = Some(DataWord("1".to_string()));
    original.common.label = Some("Allegro".to_string());
    original.mdiv_ges.attacca = Some(DataBoolean::False);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.common.n, original.common.n);
    assert_eq!(parsed.common.label, original.common.label);
    assert_eq!(parsed.mdiv_ges.attacca, original.mdiv_ges.attacca);
}

#[test]
fn mdiv_handles_unknown_attributes_leniently() {
    use tusk_model::elements::Mdiv;

    let xml = r#"<mdiv xml:id="mdiv1" unknown="value" n="1"/>"#;
    let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(mdiv.common.xml_id, Some("mdiv1".to_string()));
}

#[test]
fn mdiv_ignores_unknown_child_elements() {
    use tusk_model::elements::Mdiv;

    let xml = r#"<mdiv xml:id="mdiv1"><unknownElement/></mdiv>"#;
    let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

    // Unknown child should be skipped
    assert_eq!(mdiv.common.xml_id, Some("mdiv1".to_string()));
    assert!(mdiv.children.is_empty());
}

#[test]
fn mdiv_deserializes_with_xml_declaration() {
    use tusk_model::elements::Mdiv;

    let xml = r#"<?xml version="1.0"?><mdiv xml:id="mdiv1" n="1"/>"#;
    let mdiv = Mdiv::from_mei_str(xml).expect("should deserialize");

    assert_eq!(mdiv.common.xml_id, Some("mdiv1".to_string()));
}

// ============================================================================
// Structural Hierarchy Tests
// ============================================================================
// These tests verify the complete structural hierarchy:
// mdiv → section → measure → staff → layer → note/rest/chord/space

#[test]
fn hierarchy_layer_contains_note() {
    use tusk_model::data::{DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note};

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(tusk_model::data::DataDuration::DataDurationCmn(
        DataDurationCmn::N4,
    ));

    let mut layer = Layer::default();
    layer.basic.xml_id = Some("layer1".to_string());
    layer.children.push(LayerChild::Note(Box::new(note)));

    let xml = layer.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("layer1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerChild::Note(n) => {
            assert_eq!(n.common.xml_id, Some("n1".to_string()));
            assert_eq!(n.note_log.pname, Some(DataPitchname::from("c".to_string())));
            assert_eq!(n.note_log.oct, Some(DataOctave(4)));
        }
        other => panic!("Expected Note, got {:?}", other),
    }
}

#[test]
fn hierarchy_layer_contains_mixed_children() {
    use tusk_model::data::{
        DataDuration, DataDurationCmn, DataDurationrests, DataOctave, DataPitchname,
    };
    use tusk_model::elements::{Chord, Layer, LayerChild, Note, Rest, Space};

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut rest = Rest::default();
    rest.common.xml_id = Some("r1".to_string());
    rest.rest_log.dur = Some(DataDurationrests::DataDurationCmn(DataDurationCmn::N4));

    let mut chord = Chord::default();
    chord.common.xml_id = Some("c1".to_string());
    chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2));

    let mut space = Space::default();
    space.common.xml_id = Some("s1".to_string());
    space.space_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut layer = Layer::default();
    layer.basic.xml_id = Some("layer1".to_string());
    layer.children.push(LayerChild::Note(Box::new(note)));
    layer.children.push(LayerChild::Rest(Box::new(rest)));
    layer.children.push(LayerChild::Chord(Box::new(chord)));
    layer.children.push(LayerChild::Space(Box::new(space)));

    let xml = layer.to_mei_string().expect("serialize");
    let parsed = Layer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 4);

    // Verify order and types preserved
    assert!(matches!(&parsed.children[0], LayerChild::Note(_)));
    assert!(matches!(&parsed.children[1], LayerChild::Rest(_)));
    assert!(matches!(&parsed.children[2], LayerChild::Chord(_)));
    assert!(matches!(&parsed.children[3], LayerChild::Space(_)));
}

#[test]
fn hierarchy_staff_contains_layer() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note, Staff, StaffChild};

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut layer = Layer::default();
    layer.basic.xml_id = Some("layer1".to_string());
    layer.n_integer.n = Some(1);
    layer.children.push(LayerChild::Note(Box::new(note)));

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("staff1".to_string());
    staff.n_integer.n = Some(1);
    staff.children.push(StaffChild::Layer(Box::new(layer)));

    let xml = staff.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("staff1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        StaffChild::Layer(l) => {
            assert_eq!(l.basic.xml_id, Some("layer1".to_string()));
            assert_eq!(l.children.len(), 1);
            match &l.children[0] {
                LayerChild::Note(n) => {
                    assert_eq!(n.common.xml_id, Some("n1".to_string()));
                }
                other => panic!("Expected Note, got {:?}", other),
            }
        }
        other => panic!("Expected Layer, got {:?}", other),
    }
}

#[test]
fn hierarchy_staff_contains_multiple_layers() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{Layer, LayerChild, Note, Staff, StaffChild};

    // Create two layers for polyphony (e.g., soprano and alto in one staff)
    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    note1.note_log.pname = Some(DataPitchname::from("g".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    note1.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut layer1 = Layer::default();
    layer1.basic.xml_id = Some("layer1".to_string());
    layer1.n_integer.n = Some(1);
    layer1.children.push(LayerChild::Note(Box::new(note1)));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    note2.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let mut layer2 = Layer::default();
    layer2.basic.xml_id = Some("layer2".to_string());
    layer2.n_integer.n = Some(2);
    layer2.children.push(LayerChild::Note(Box::new(note2)));

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("staff1".to_string());
    staff.children.push(StaffChild::Layer(Box::new(layer1)));
    staff.children.push(StaffChild::Layer(Box::new(layer2)));

    let xml = staff.to_mei_string().expect("serialize");
    let parsed = Staff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // Verify layer order and content
    match &parsed.children[0] {
        StaffChild::Layer(l) => {
            assert_eq!(l.n_integer.n, Some(1));
        }
        other => panic!("Expected Layer, got {:?}", other),
    }

    match &parsed.children[1] {
        StaffChild::Layer(l) => {
            assert_eq!(l.n_integer.n, Some(2));
        }
        other => panic!("Expected Layer, got {:?}", other),
    }
}

#[test]
fn hierarchy_measure_contains_staff() {
    // Note: Currently, Measure's deserializer uses read_children_raw + parse_staff_from_raw
    // which only parses staff attributes, not staff's children (layers).
    // This test documents the current behavior - staff children are preserved in serialization
    // but not fully parsed in deserialization (layers within staff not parsed).
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("staff1".to_string());
    staff.n_integer.n = Some(1);
    // Note: even if we added layer children here, they wouldn't be parsed back

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let xml = measure.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
    assert_eq!(parsed.common.n, Some(DataWord("1".to_string())));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.basic.xml_id, Some("staff1".to_string()));
            assert_eq!(s.n_integer.n, Some(1));
            // Staff children (layers) are not parsed from within Measure - known limitation
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn hierarchy_measure_contains_multiple_staves() {
    // Test that measure can contain multiple staff elements
    // Note: Staff children (layers) are not parsed - known limitation
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut staff_rh = Staff::default();
    staff_rh.basic.xml_id = Some("staff1".to_string());
    staff_rh.n_integer.n = Some(1);

    let mut staff_lh = Staff::default();
    staff_lh.basic.xml_id = Some("staff2".to_string());
    staff_lh.n_integer.n = Some(2);

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure
        .children
        .push(MeasureChild::Staff(Box::new(staff_rh)));
    measure
        .children
        .push(MeasureChild::Staff(Box::new(staff_lh)));

    let xml = measure.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);

    // Verify staff order preserved
    match &parsed.children[0] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.n_integer.n, Some(1));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }

    match &parsed.children[1] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.n_integer.n, Some(2));
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn hierarchy_section_contains_measure() {
    // Test section → measure hierarchy
    // Note: Measure's staff children are not parsed with full hierarchy
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Section, SectionChild, Staff};

    let mut staff = Staff::default();
    staff.n_integer.n = Some(1);

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let mut section = Section::default();
    section.common.xml_id = Some("sec1".to_string());
    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    let xml = section.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m1".to_string()));
            assert_eq!(m.children.len(), 1); // Staff is parsed
        }
        other => panic!("Expected Measure, got {:?}", other),
    }
}

#[test]
fn hierarchy_section_contains_multiple_measures() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, Section, SectionChild};

    let mut measure1 = Measure::default();
    measure1.common.xml_id = Some("m1".to_string());
    measure1.common.n = Some(DataWord("1".to_string()));

    let mut measure2 = Measure::default();
    measure2.common.xml_id = Some("m2".to_string());
    measure2.common.n = Some(DataWord("2".to_string()));

    let mut measure3 = Measure::default();
    measure3.common.xml_id = Some("m3".to_string());
    measure3.common.n = Some(DataWord("3".to_string()));

    let mut section = Section::default();
    section.common.xml_id = Some("sec1".to_string());
    section
        .children
        .push(SectionChild::Measure(Box::new(measure1)));
    section
        .children
        .push(SectionChild::Measure(Box::new(measure2)));
    section
        .children
        .push(SectionChild::Measure(Box::new(measure3)));

    let xml = section.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 3);

    // Verify measure order preserved
    for (i, child) in parsed.children.iter().enumerate() {
        match child {
            SectionChild::Measure(m) => {
                let expected_n = DataWord(format!("{}", i + 1));
                assert_eq!(m.common.n, Some(expected_n));
            }
            other => panic!("Expected Measure, got {:?}", other),
        }
    }
}

#[test]
fn hierarchy_section_contains_nested_sections() {
    use tusk_model::elements::{Section, SectionChild};

    let mut inner_section = Section::default();
    inner_section.common.xml_id = Some("sec2".to_string());
    inner_section.common.label = Some("Coda".to_string());

    let mut outer_section = Section::default();
    outer_section.common.xml_id = Some("sec1".to_string());
    outer_section.common.label = Some("Movement I".to_string());
    outer_section
        .children
        .push(SectionChild::Section(Box::new(inner_section)));

    let xml = outer_section.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SectionChild::Section(s) => {
            assert_eq!(s.common.xml_id, Some("sec2".to_string()));
            assert_eq!(s.common.label, Some("Coda".to_string()));
        }
        other => panic!("Expected Section, got {:?}", other),
    }
}

#[test]
fn hierarchy_mdiv_contains_nested_mdiv() {
    use tusk_model::elements::{Mdiv, MdivChild};

    let mut inner_mdiv = Mdiv::default();
    inner_mdiv.common.xml_id = Some("mdiv2".to_string());
    inner_mdiv.common.label = Some("Movement I".to_string());

    let mut outer_mdiv = Mdiv::default();
    outer_mdiv.common.xml_id = Some("mdiv1".to_string());
    outer_mdiv.common.label = Some("Symphony No. 1".to_string());
    outer_mdiv
        .children
        .push(MdivChild::Mdiv(Box::new(inner_mdiv)));

    let xml = outer_mdiv.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mdiv1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MdivChild::Mdiv(m) => {
            assert_eq!(m.common.xml_id, Some("mdiv2".to_string()));
        }
        other => panic!("Expected Mdiv, got {:?}", other),
    }
}

#[test]
fn hierarchy_full_cmn_structure() {
    // Test section → measure → staff structure
    // Note: Staff's children (layers) are not parsed when staff is a child of measure
    // This tests the current behavior; full recursive parsing is a future enhancement
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Section, SectionChild, Staff};

    let mut staff = Staff::default();
    staff.basic.xml_id = Some("staff1".to_string());
    staff.n_integer.n = Some(1);
    // Staff children won't be parsed when inside measure

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure.children.push(MeasureChild::Staff(Box::new(staff)));

    let mut section = Section::default();
    section.common.xml_id = Some("sec1".to_string());
    section.common.label = Some("Movement I".to_string());
    section
        .children
        .push(SectionChild::Measure(Box::new(measure)));

    // Serialize and parse
    let xml = section.to_mei_string().expect("serialize");
    let parsed = Section::from_mei_str(&xml).expect("deserialize");

    // Traverse the hierarchy to verify structure preserved
    assert_eq!(parsed.common.xml_id, Some("sec1".to_string()));
    assert_eq!(parsed.common.label, Some("Movement I".to_string()));
    assert_eq!(parsed.children.len(), 1);

    let measure = match &parsed.children[0] {
        SectionChild::Measure(m) => m,
        other => panic!("Expected Measure, got {:?}", other),
    };
    assert_eq!(measure.common.xml_id, Some("m1".to_string()));
    assert_eq!(measure.children.len(), 1);

    let staff = match &measure.children[0] {
        MeasureChild::Staff(s) => s,
        other => panic!("Expected Staff, got {:?}", other),
    };
    assert_eq!(staff.basic.xml_id, Some("staff1".to_string()));
    assert_eq!(staff.n_integer.n, Some(1));
    // Staff children are not parsed - this is the current limitation
}

#[test]
fn hierarchy_realistic_piano_measure() {
    // Test a realistic piano measure with two staves
    // Note: Staff children (layers) are not parsed when inside measure
    use tusk_model::data::DataWord;
    use tusk_model::elements::{Measure, MeasureChild, Staff};

    let mut rh_staff = Staff::default();
    rh_staff.basic.xml_id = Some("rh".to_string());
    rh_staff.n_integer.n = Some(1);
    // Layers won't be parsed from within measure

    let mut lh_staff = Staff::default();
    lh_staff.basic.xml_id = Some("lh".to_string());
    lh_staff.n_integer.n = Some(2);
    // Layers won't be parsed from within measure

    let mut measure = Measure::default();
    measure.common.xml_id = Some("m1".to_string());
    measure.common.n = Some(DataWord("1".to_string()));
    measure
        .children
        .push(MeasureChild::Staff(Box::new(rh_staff)));
    measure
        .children
        .push(MeasureChild::Staff(Box::new(lh_staff)));

    let xml = measure.to_mei_string().expect("serialize");
    let parsed = Measure::from_mei_str(&xml).expect("deserialize");

    // Verify structure - both staves should be parsed
    assert_eq!(parsed.children.len(), 2);

    // RH staff
    match &parsed.children[0] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.n_integer.n, Some(1));
            assert_eq!(s.basic.xml_id, Some("rh".to_string()));
            // Staff children are not parsed from within measure
        }
        other => panic!("Expected Staff, got {:?}", other),
    }

    // LH staff
    match &parsed.children[1] {
        MeasureChild::Staff(s) => {
            assert_eq!(s.n_integer.n, Some(2));
            assert_eq!(s.basic.xml_id, Some("lh".to_string()));
            // Staff children are not parsed from within measure
        }
        other => panic!("Expected Staff, got {:?}", other),
    }
}

#[test]
fn hierarchy_from_external_xml() {
    // Parse a multi-level structure from external XML string
    // Note: Measure's staff parsing doesn't recursively parse staff children (layers)
    use tusk_model::elements::{MeasureChild, Section, SectionChild};

    let xml = r#"<section xml:id="sec1" label="Introduction">
        <measure xml:id="m1" n="1">
            <staff xml:id="s1" n="1">
                <layer xml:id="l1" n="1">
                    <note xml:id="n1" pname="c" oct="4" dur="4"/>
                </layer>
            </staff>
        </measure>
        <measure xml:id="m2" n="2">
            <staff xml:id="s2" n="1"/>
        </measure>
    </section>"#;

    let section = Section::from_mei_str(xml).expect("should parse");

    assert_eq!(section.common.xml_id, Some("sec1".to_string()));
    assert_eq!(section.common.label, Some("Introduction".to_string()));
    assert_eq!(section.children.len(), 2);

    // First measure - verify measure and staff parsing
    match &section.children[0] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m1".to_string()));
            assert_eq!(m.children.len(), 1);
            match &m.children[0] {
                MeasureChild::Staff(s) => {
                    assert_eq!(s.basic.xml_id, Some("s1".to_string()));
                    assert_eq!(s.n_integer.n, Some(1));
                    // Staff children (layers) are not parsed when inside measure
                }
                other => panic!("Expected Staff, got {:?}", other),
            }
        }
        other => panic!("Expected Measure, got {:?}", other),
    }

    // Second measure
    match &section.children[1] {
        SectionChild::Measure(m) => {
            assert_eq!(m.common.xml_id, Some("m2".to_string()));
            assert_eq!(m.children.len(), 1);
            match &m.children[0] {
                MeasureChild::Staff(s) => {
                    assert_eq!(s.basic.xml_id, Some("s2".to_string()));
                }
                other => panic!("Expected Staff, got {:?}", other),
            }
        }
        other => panic!("Expected Measure, got {:?}", other),
    }
}

#[test]
fn hierarchy_deep_nesting_preserved() {
    // Test deeply nested mdiv structure preserves all IDs through serialization
    use tusk_model::elements::{Mdiv, MdivChild};

    // Build nested mdiv hierarchy
    let mut inner_inner_mdiv = Mdiv::default();
    inner_inner_mdiv.common.xml_id = Some("mdiv-inner-inner".to_string());
    inner_inner_mdiv.common.label = Some("Third Level".to_string());

    let mut inner_mdiv = Mdiv::default();
    inner_mdiv.common.xml_id = Some("mdiv-inner".to_string());
    inner_mdiv.common.label = Some("Second Level".to_string());
    inner_mdiv
        .children
        .push(MdivChild::Mdiv(Box::new(inner_inner_mdiv)));

    let mut outer_mdiv = Mdiv::default();
    outer_mdiv.common.xml_id = Some("mdiv-outer".to_string());
    outer_mdiv.common.label = Some("First Level".to_string());
    outer_mdiv
        .children
        .push(MdivChild::Mdiv(Box::new(inner_mdiv)));

    let xml = outer_mdiv.to_mei_string().expect("serialize");
    let parsed = Mdiv::from_mei_str(&xml).expect("deserialize");

    // Verify three-level nesting is preserved
    assert_eq!(parsed.common.xml_id, Some("mdiv-outer".to_string()));
    assert_eq!(parsed.common.label, Some("First Level".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MdivChild::Mdiv(inner) => {
            assert_eq!(inner.common.xml_id, Some("mdiv-inner".to_string()));
            assert_eq!(inner.common.label, Some("Second Level".to_string()));
            assert_eq!(inner.children.len(), 1);

            match &inner.children[0] {
                MdivChild::Mdiv(inner_inner) => {
                    assert_eq!(
                        inner_inner.common.xml_id,
                        Some("mdiv-inner-inner".to_string())
                    );
                    assert_eq!(inner_inner.common.label, Some("Third Level".to_string()));
                }
                other => panic!("Expected Mdiv, got {:?}", other),
            }
        }
        other => panic!("Expected Mdiv, got {:?}", other),
    }
}

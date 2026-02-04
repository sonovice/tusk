//! Round-trip serialization tests for score definition elements.
//!
//! Tests for ScoreDef, StaffDef, LayerDef, StaffGrp elements and their relationships.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// ScoreDef Element Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_empty_scoredef() {
    use tusk_model::elements::ScoreDef;

    let original = ScoreDef::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    // All fields should remain None/empty
    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.score_def_log.meter_count.is_none());
    assert!(parsed.score_def_log.meter_unit.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_scoredef_with_xml_id() {
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.common.xml_id = Some("sd1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("xml:id=\"sd1\""),
        "xml should contain id: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.common.xml_id, Some("sd1".to_string()));
}

#[test]
fn roundtrip_scoredef_with_meter_attributes() {
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_log.meter_count = Some("4".to_string());
    original.score_def_log.meter_unit = Some(4.0);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("meter.count=\"4\""),
        "xml should contain meter.count: {}",
        xml
    );
    assert!(
        xml.contains("meter.unit=\"4\""),
        "xml should contain meter.unit: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
}

#[test]
fn roundtrip_scoredef_with_meter_sym() {
    use tusk_model::data::DataMetersign;
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_log.meter_count = Some("4".to_string());
    original.score_def_log.meter_unit = Some(4.0);
    original.score_def_log.meter_sym = Some(DataMetersign::Common);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("meter.sym=\"common\""),
        "xml should contain meter.sym: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_log.meter_sym, Some(DataMetersign::Common));
}

#[test]
fn roundtrip_scoredef_with_keysig() {
    use tusk_model::data::DataKeyfifths;
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    // 3 flats (e.g., E-flat major or C minor)
    original.score_def_log.keysig = vec![DataKeyfifths("-3".to_string())];

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("keysig=\"-3\"") || xml.contains("keysig=\"3f\""),
        "xml should contain keysig: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert!(!parsed.score_def_log.keysig.is_empty());
}

#[test]
fn roundtrip_scoredef_with_clef_attributes() {
    use tusk_model::data::{DataClefline, DataClefshape};
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_log.clef_shape = Some(DataClefshape::G);
    original.score_def_log.clef_line = Some(DataClefline(2));

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("clef.shape=\"G\""),
        "xml should contain clef.shape: {}",
        xml
    );
    assert!(
        xml.contains("clef.line=\"2\""),
        "xml should contain clef.line: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_log.clef_shape, Some(DataClefshape::G));
    assert_eq!(parsed.score_def_log.clef_line, Some(DataClefline(2)));
}

#[test]
fn roundtrip_scoredef_with_gestural_ppq() {
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_ges.ppq = Some(480);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("ppq=\"480\""),
        "xml should contain ppq: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_ges.ppq, Some(480));
}

#[test]
fn roundtrip_scoredef_with_midi_bpm() {
    use tusk_model::data::DataMidibpm;
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_ges.midi_bpm = Some(DataMidibpm(120.0));

    let xml = original.to_mei_string().expect("serialize");
    // midi.bpm may serialize as "120" or "120.0" depending on float representation
    assert!(
        xml.contains("midi.bpm=\"120") && xml.contains('"'),
        "xml should contain midi.bpm: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert!(parsed.score_def_ges.midi_bpm.is_some());
}

#[test]
fn roundtrip_scoredef_with_analytical_key() {
    use tusk_model::data::{DataMode, DataModeCmn, DataPitchname};
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_anl.key_pname = Some(DataPitchname::from("c".to_string()));
    original.score_def_anl.key_mode = Some(DataMode::DataModeCmn(DataModeCmn::Major));

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("key.pname=\"c\""),
        "xml should contain key.pname: {}",
        xml
    );
    assert!(
        xml.contains("key.mode=\"major\""),
        "xml should contain key.mode: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(
        parsed.score_def_anl.key_pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(
        parsed.score_def_anl.key_mode,
        Some(DataMode::DataModeCmn(DataModeCmn::Major))
    );
}

#[test]
fn roundtrip_scoredef_with_tune_hz() {
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_ges.tune_hz = Some(440.0);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("tune.Hz=\"440\""),
        "xml should contain tune.Hz: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_ges.tune_hz, Some(440.0));
}

#[test]
fn roundtrip_scoredef_with_visual_meter_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::ScoreDef;

    let mut original = ScoreDef::default();
    original.score_def_vis.meter_visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    assert!(
        xml.contains("meter.visible=\"true\""),
        "xml should contain meter.visible: {}",
        xml
    );

    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");
    assert_eq!(parsed.score_def_vis.meter_visible, Some(DataBoolean::True));
}

#[test]
fn roundtrip_scoredef_comprehensive() {
    use tusk_model::data::{DataClefline, DataClefshape, DataMode, DataModeCmn, DataPitchname};
    use tusk_model::elements::ScoreDef;

    // A realistic scoreDef with common attributes
    let mut original = ScoreDef::default();
    original.common.xml_id = Some("sd1".to_string());
    original.score_def_log.clef_shape = Some(DataClefshape::G);
    original.score_def_log.clef_line = Some(DataClefline(2));
    original.score_def_log.meter_count = Some("4".to_string());
    original.score_def_log.meter_unit = Some(4.0);
    original.score_def_anl.key_pname = Some(DataPitchname::from("c".to_string()));
    original.score_def_anl.key_mode = Some(DataMode::DataModeCmn(DataModeCmn::Major));
    original.score_def_ges.ppq = Some(960);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.score_def_log.clef_shape, Some(DataClefshape::G));
    assert_eq!(parsed.score_def_log.clef_line, Some(DataClefline(2)));
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
    assert_eq!(
        parsed.score_def_anl.key_pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(
        parsed.score_def_anl.key_mode,
        Some(DataMode::DataModeCmn(DataModeCmn::Major))
    );
    assert_eq!(parsed.score_def_ges.ppq, Some(960));
}

// External XML parsing tests for scoreDef

#[test]
fn parse_external_scoredef_minimal() {
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef/>"#;
    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn parse_external_scoredef_with_meter() {
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef meter.count="4" meter.unit="4"/>"#;
    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
}

#[test]
fn parse_external_scoredef_with_keysig_fifths() {
    use tusk_model::elements::ScoreDef;

    // keysig="3f" means 3 flats
    let xml = r#"<scoreDef keysig="3f" meter.count="4" meter.sym="common" meter.unit="4"/>"#;
    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    // keysig should be parsed
    assert!(!parsed.score_def_log.keysig.is_empty());
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
}

#[test]
fn parse_external_scoredef_with_staffgrp_child() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild};

    let xml = r#"<scoreDef>
        <staffGrp xml:id="sg1">
            <staffDef xml:id="sd1" n="1" lines="5" clef.shape="G" clef.line="2"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.children.len(), 1);
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn parse_external_scoredef_mensural_example() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    // From specs/mei/examples/verovio/notes_rests.mei
    let xml = r#"<scoreDef>
        <staffGrp>
            <staffDef label="notes" n="1" notationtype="mensural.white" lines="5" clef.shape="G" clef.line="2"/>
            <staffDef label="rests" n="2" notationtype="mensural.white" lines="5" clef.shape="G" clef.line="2"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.children.len(), 2);
            // Check first staffDef - label is in labelled.label for StaffDef
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.labelled.label, Some("notes".to_string()));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
            // Check second staffDef
            match &sg.children[1] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.labelled.label, Some("rests".to_string()));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn roundtrip_scoredef_with_staffgrp_child() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffDef, StaffGrp, StaffGrpChild};

    let mut staff_def = StaffDef::default();
    staff_def.basic.xml_id = Some("staff1".to_string());
    staff_def.n_integer.n = Some(1);

    let mut staff_grp = StaffGrp::default();
    staff_grp.common.xml_id = Some("sg1".to_string());
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let mut original = ScoreDef::default();
    original.common.xml_id = Some("sd1".to_string());
    original
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(staff_grp)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.children.len(), 1);
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.basic.xml_id, Some("staff1".to_string()));
                    assert_eq!(sd.n_integer.n, Some(1));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn roundtrip_scoredef_with_keysig_child() {
    use tusk_model::elements::{KeySig, ScoreDef, ScoreDefChild};

    let mut keysig = KeySig::default();
    keysig.common.xml_id = Some("ks1".to_string());

    let mut original = ScoreDef::default();
    original.common.xml_id = Some("sd1".to_string());
    original
        .children
        .push(ScoreDefChild::KeySig(Box::new(keysig)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::KeySig(ks) => {
            assert_eq!(ks.common.xml_id, Some("ks1".to_string()));
        }
        other => panic!("Expected KeySig, got {:?}", other),
    }
}

#[test]
fn roundtrip_scoredef_with_metersig_child() {
    use tusk_model::elements::{MeterSig, ScoreDef, ScoreDefChild};

    let mut metersig = MeterSig::default();
    metersig.common.xml_id = Some("ms1".to_string());

    let mut original = ScoreDef::default();
    original
        .children
        .push(ScoreDefChild::MeterSig(Box::new(metersig)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::MeterSig(ms) => {
            assert_eq!(ms.common.xml_id, Some("ms1".to_string()));
        }
        other => panic!("Expected MeterSig, got {:?}", other),
    }
}

// ============================================================================
// StaffDef parsing tests
// ============================================================================

#[test]
fn staffdef_deserializes_from_empty_element() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n_integer.n.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn staffdef_deserializes_xml_id() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef xml:id="sd1"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
}

#[test]
fn staffdef_deserializes_n_attribute() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef n="1"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn staffdef_deserializes_label() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef label="Soprano"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.labelled.label, Some("Soprano".to_string()));
}

#[test]
fn staffdef_deserializes_lines() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef lines="5"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.lines, Some(5));
}

#[test]
fn staffdef_deserializes_clef_attributes() {
    use tusk_model::data::{DataClefline, DataClefshape};
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef clef.shape="G" clef.line="2"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.clef_shape, Some(DataClefshape::G));
    assert_eq!(parsed.staff_def_log.clef_line, Some(DataClefline(2)));
}

#[test]
fn staffdef_deserializes_clef_dis_attributes() {
    use tusk_model::data::{DataOctaveDis, DataStaffrelBasic};
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef clef.dis="8" clef.dis.place="below"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.clef_dis, Some(DataOctaveDis(8)));
    assert_eq!(
        parsed.staff_def_log.clef_dis_place,
        Some(DataStaffrelBasic::Below)
    );
}

#[test]
fn staffdef_deserializes_notationtype() {
    use tusk_model::data::DataNotationtype;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef notationtype="mensural.white"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.staff_def_log.notationtype,
        Some(DataNotationtype::MensuralWhite)
    );
}

#[test]
fn staffdef_deserializes_meter_attributes() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef meter.count="4" meter.unit="4"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.staff_def_log.meter_unit, Some(4.0));
}

#[test]
fn staffdef_deserializes_meter_sym() {
    use tusk_model::data::DataMetersign;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef meter.sym="common"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.meter_sym, Some(DataMetersign::Common));
}

#[test]
fn staffdef_deserializes_transposition() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef trans.diat="-2" trans.semi="-3"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_log.trans_diat, Some(-2));
    assert_eq!(parsed.staff_def_log.trans_semi, Some(-3));
}

#[test]
fn staffdef_deserializes_keysig() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef keysig="2s"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert!(!parsed.staff_def_log.keysig.is_empty());
}

#[test]
fn staffdef_deserializes_ppq() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef ppq="960"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_ges.ppq, Some(960));
}

#[test]
fn staffdef_deserializes_tuning_attributes() {
    use tusk_model::data::DataPitchname;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef tune.Hz="440" tune.pname="a"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_ges.tune_hz, Some(440.0));
    assert_eq!(
        parsed.staff_def_ges.tune_pname,
        Some(DataPitchname::from("a".to_string()))
    );
}

#[test]
fn staffdef_deserializes_key_attributes() {
    use tusk_model::data::{DataMode, DataModeCmn, DataPitchname};
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef key.pname="c" key.mode="major"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.staff_def_anl.key_pname,
        Some(DataPitchname::from("c".to_string()))
    );
    assert_eq!(
        parsed.staff_def_anl.key_mode,
        Some(DataMode::DataModeCmn(DataModeCmn::Major))
    );
}

#[test]
fn staffdef_deserializes_visual_attributes() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef clef.visible="true" lines.visible="true"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.staff_def_vis.clef_visible, Some(DataBoolean::True));
    assert_eq!(parsed.staff_def_vis.lines_visible, Some(DataBoolean::True));
}

#[test]
fn staffdef_deserializes_scale() {
    use tusk_model::data::DataPercent;
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef scale="75%"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.staff_def_vis.scale,
        Some(DataPercent("75%".to_string()))
    );
}

#[test]
fn staffdef_deserializes_full_common_staff_attributes() {
    use tusk_model::data::{DataClefline, DataClefshape, DataNotationtype};
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef xml:id="sd1" n="1" label="Tenor" lines="5" clef.shape="C" clef.line="4" notationtype="cmn"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.labelled.label, Some("Tenor".to_string()));
    assert_eq!(parsed.staff_def_log.lines, Some(5));
    assert_eq!(parsed.staff_def_log.clef_shape, Some(DataClefshape::C));
    assert_eq!(parsed.staff_def_log.clef_line, Some(DataClefline(4)));
    assert_eq!(
        parsed.staff_def_log.notationtype,
        Some(DataNotationtype::Cmn)
    );
}

#[test]
fn staffdef_deserializes_with_clef_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><clef xml:id="c1" shape="G" line="2"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::Clef(clef) => {
            assert_eq!(clef.common.xml_id, Some("c1".to_string()));
        }
        other => panic!("Expected Clef, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_with_keysig_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><keySig xml:id="ks1"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::KeySig(ks) => {
            assert_eq!(ks.common.xml_id, Some("ks1".to_string()));
        }
        other => panic!("Expected KeySig, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_with_metersig_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><meterSig xml:id="ms1" count="4" unit="4"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::MeterSig(ms) => {
            assert_eq!(ms.common.xml_id, Some("ms1".to_string()));
        }
        other => panic!("Expected MeterSig, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_with_label_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><label xml:id="l1">Violin I</label></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::Label(label) => {
            assert_eq!(label.common.xml_id, Some("l1".to_string()));
        }
        other => panic!("Expected Label, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_with_layerdef_child() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1"><layerDef xml:id="ld1" n="1"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::LayerDef(ld) => {
            assert_eq!(ld.basic.xml_id, Some("ld1".to_string()));
        }
        other => panic!("Expected LayerDef, got {:?}", other),
    }
}

#[test]
fn staffdef_deserializes_multiple_children() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef n="1">
        <label>Violin</label>
        <clef shape="G" line="2"/>
        <keySig/>
    </staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 3);
    assert!(matches!(&parsed.children[0], StaffDefChild::Label(_)));
    assert!(matches!(&parsed.children[1], StaffDefChild::Clef(_)));
    assert!(matches!(&parsed.children[2], StaffDefChild::KeySig(_)));
}

#[test]
fn staffdef_handles_unknown_attributes_leniently() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef xml:id="sd1" unknown="value" n="1"/>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn staffdef_roundtrip_basic() {
    use tusk_model::elements::StaffDef;

    let mut original = StaffDef::default();
    original.basic.xml_id = Some("sd1".to_string());
    original.n_integer.n = Some(1);
    original.staff_def_log.lines = Some(5);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = StaffDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.staff_def_log.lines, Some(5));
}

// Note: Full roundtrip with children requires serialization implementation.
// Testing deserialization from manually constructed XML instead.
#[test]
fn staffdef_parses_with_clef_from_xml() {
    use tusk_model::data::{DataClefline, DataClefshape};
    use tusk_model::elements::{StaffDef, StaffDefChild};

    // Use manually constructed XML to verify deserialization
    let xml = r#"<staffDef xml:id="sd1"><clef xml:id="c1" shape="G" line="2"/></staffDef>"#;
    let parsed = StaffDef::from_mei_str(xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StaffDefChild::Clef(c) => {
            assert_eq!(c.common.xml_id, Some("c1".to_string()));
            assert_eq!(c.clef_log.shape, Some(DataClefshape::G));
            assert_eq!(c.clef_log.line, Some(DataClefline(2)));
        }
        other => panic!("Expected Clef, got {:?}", other),
    }
}

// ============================================================================
// LayerDef parsing tests
// ============================================================================

#[test]
fn layerdef_deserializes_from_empty_element() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.n_integer.n.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn layerdef_deserializes_xml_id() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef xml:id="ld1"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
}

#[test]
fn layerdef_deserializes_n_attribute() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef n="1"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn layerdef_deserializes_label() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef label="Voice 1"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.labelled.label, Some("Voice 1".to_string()));
}

#[test]
fn layerdef_deserializes_dur_default() {
    use tusk_model::data::{DataDuration, DataDurationCmn};
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef dur.default="4"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.layer_def_log.dur_default,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn layerdef_deserializes_oct_default() {
    use tusk_model::data::DataOctave;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef oct.default="4"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.oct_default, Some(DataOctave(4)));
}

#[test]
fn layerdef_deserializes_num_default_and_numbase_default() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef num.default="3" numbase.default="2"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.num_default, Some(3));
    assert_eq!(parsed.layer_def_log.numbase_default, Some(2));
}

#[test]
fn layerdef_deserializes_beam_group() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef beam.group="4,4,4,4"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.beam_group, Some("4,4,4,4".to_string()));
}

#[test]
fn layerdef_deserializes_beam_rests() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef beam.rests="true"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.beam_rests, Some(DataBoolean::True));
}

#[test]
fn layerdef_deserializes_transposition() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef trans.diat="-1" trans.semi="-2"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_log.trans_diat, Some(-1));
    assert_eq!(parsed.layer_def_log.trans_semi, Some(-2));
}

#[test]
fn layerdef_deserializes_gestural_instr() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::LayerDef;

    let xml = r##"<layerDef instr="#instr1"/>"##;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.layer_def_ges.instr,
        Some(DataUri("#instr1".to_string()))
    );
}

#[test]
fn layerdef_deserializes_tuning_attributes() {
    use tusk_model::data::DataPitchname;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef tune.Hz="442" tune.pname="a"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_ges.tune_hz, Some(442.0));
    assert_eq!(
        parsed.layer_def_ges.tune_pname,
        Some(DataPitchname::from("a".to_string()))
    );
}

#[test]
fn layerdef_deserializes_visual_beam_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef beam.color="red"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.layer_def_vis.beam_color,
        Some(DataColor::DataColornames(DataColornames::Red))
    );
}

#[test]
fn layerdef_deserializes_beam_rend() {
    use tusk_model::att::AttLayerDefVisBeamRend;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef beam.rend="acc"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(
        parsed.layer_def_vis.beam_rend,
        Some(AttLayerDefVisBeamRend::Acc)
    );
}

#[test]
fn layerdef_deserializes_visible() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef visible="false"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.layer_def_vis.visible, Some(DataBoolean::False));
}

#[test]
fn layerdef_deserializes_full_common_attributes() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave};
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef xml:id="ld1" n="1" label="Melody" dur.default="8" oct.default="5"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.labelled.label, Some("Melody".to_string()));
    assert_eq!(
        parsed.layer_def_log.dur_default,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N8))
    );
    assert_eq!(parsed.layer_def_log.oct_default, Some(DataOctave(5)));
}

#[test]
fn layerdef_deserializes_with_label_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1"><label xml:id="l1">Voice I</label></layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LayerDefChild::Label(label) => {
            assert_eq!(label.common.xml_id, Some("l1".to_string()));
        }
        other => panic!("Expected Label, got {:?}", other),
    }
}

#[test]
fn layerdef_deserializes_with_labelabbr_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1"><labelAbbr xml:id="la1">V.I</labelAbbr></layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LayerDefChild::LabelAbbr(label_abbr) => {
            assert_eq!(label_abbr.common.xml_id, Some("la1".to_string()));
        }
        other => panic!("Expected LabelAbbr, got {:?}", other),
    }
}

#[test]
fn layerdef_deserializes_with_instrdef_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1"><instrDef xml:id="id1"/></layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LayerDefChild::InstrDef(instr_def) => {
            assert_eq!(instr_def.basic.xml_id, Some("id1".to_string()));
        }
        other => panic!("Expected InstrDef, got {:?}", other),
    }
}

#[test]
fn layerdef_deserializes_with_metersig_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1"><meterSig xml:id="ms1" count="4" unit="4"/></layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        LayerDefChild::MeterSig(meter_sig) => {
            assert_eq!(meter_sig.common.xml_id, Some("ms1".to_string()));
        }
        other => panic!("Expected MeterSig, got {:?}", other),
    }
}

#[test]
fn layerdef_deserializes_multiple_children() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef n="1">
        <label>Voice</label>
        <labelAbbr>V.</labelAbbr>
        <instrDef/>
    </layerDef>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize");

    assert_eq!(parsed.children.len(), 3);
    assert!(matches!(&parsed.children[0], LayerDefChild::Label(_)));
    assert!(matches!(&parsed.children[1], LayerDefChild::LabelAbbr(_)));
    assert!(matches!(&parsed.children[2], LayerDefChild::InstrDef(_)));
}

#[test]
fn layerdef_handles_unknown_attributes_leniently() {
    use tusk_model::elements::LayerDef;

    let xml = r#"<layerDef xml:id="ld1" unknown="value" n="1"/>"#;
    let parsed = LayerDef::from_mei_str(xml).expect("should deserialize in lenient mode");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
}

#[test]
fn layerdef_roundtrip_basic() {
    use tusk_model::elements::LayerDef;

    let mut original = LayerDef::default();
    original.basic.xml_id = Some("ld1".to_string());
    original.n_integer.n = Some(1);
    original.labelled.label = Some("Voice 1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = LayerDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.labelled.label, Some("Voice 1".to_string()));
}

#[test]
fn layerdef_roundtrip_with_log_attributes() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave};
    use tusk_model::elements::LayerDef;

    let mut original = LayerDef::default();
    original.layer_def_log.dur_default = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.layer_def_log.oct_default = Some(DataOctave(4));
    original.layer_def_log.beam_group = Some("8,8,8,8".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = LayerDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.layer_def_log.dur_default,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
    assert_eq!(parsed.layer_def_log.oct_default, Some(DataOctave(4)));
    assert_eq!(parsed.layer_def_log.beam_group, Some("8,8,8,8".to_string()));
}

#[test]
fn layerdef_roundtrip_with_ges_attributes() {
    use tusk_model::data::{DataPitchname, DataUri};
    use tusk_model::elements::LayerDef;

    let mut original = LayerDef::default();
    original.layer_def_ges.instr = Some(DataUri("#piano".to_string()));
    original.layer_def_ges.tune_hz = Some(440.0);
    original.layer_def_ges.tune_pname = Some(DataPitchname::from("a".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = LayerDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.layer_def_ges.instr,
        Some(DataUri("#piano".to_string()))
    );
    assert_eq!(parsed.layer_def_ges.tune_hz, Some(440.0));
    assert_eq!(
        parsed.layer_def_ges.tune_pname,
        Some(DataPitchname::from("a".to_string()))
    );
}

#[test]
fn layerdef_roundtrip_with_vis_attributes() {
    use tusk_model::att::AttLayerDefVisBeamRend;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::LayerDef;

    let mut original = LayerDef::default();
    original.layer_def_vis.beam_rend = Some(AttLayerDefVisBeamRend::Rit);
    original.layer_def_vis.beam_slope = Some(0.5);
    original.layer_def_vis.visible = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = LayerDef::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.layer_def_vis.beam_rend,
        Some(AttLayerDefVisBeamRend::Rit)
    );
    assert_eq!(parsed.layer_def_vis.beam_slope, Some(0.5));
    assert_eq!(parsed.layer_def_vis.visible, Some(DataBoolean::True));
}

// ============================================================================
// StaffGrp tests with full attribute support
// ============================================================================

#[test]
fn staffgrp_parse_with_vis_attributes() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::{DataBarmethod, DataBoolean};
    use tusk_model::elements::{ScoreDef, ScoreDefChild};

    let xml = r#"<scoreDef>
        <staffGrp xml:id="sg1" symbol="bracket" bar.thru="true" bar.method="mensur" bar.len="8" visible="true">
            <staffDef n="1" lines="5"/>
            <staffDef n="2" lines="5"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.staff_grp_vis.symbol, Some(AttStaffGrpVisSymbol::Bracket));
            assert_eq!(sg.staff_grp_vis.bar_thru, Some(DataBoolean::True));
            assert_eq!(sg.staff_grp_vis.bar_method, Some(DataBarmethod::Mensur));
            assert_eq!(sg.staff_grp_vis.bar_len, Some(8.0));
            assert_eq!(sg.staff_grp_vis.visible, Some(DataBoolean::True));
            assert_eq!(sg.children.len(), 2);
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn staffgrp_parse_with_ges_attributes() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{ScoreDef, ScoreDefChild};

    let xml = r##"<scoreDef>
        <staffGrp xml:id="sg1" instr="#piano">
            <staffDef n="1" lines="5"/>
        </staffGrp>
    </scoreDef>"##;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.staff_grp_ges.instr, Some(DataUri("#piano".to_string())));
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

#[test]
fn staffgrp_roundtrip_with_vis_attributes() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::{DataBarmethod, DataBoolean, DataStaffloc};
    use tusk_model::elements::{StaffDef, StaffGrp, StaffGrpChild};

    let mut staff_grp = StaffGrp::default();
    staff_grp.common.xml_id = Some("sg1".to_string());
    staff_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Brace);
    staff_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);
    staff_grp.staff_grp_vis.bar_method = Some(DataBarmethod::Mensur);
    staff_grp.staff_grp_vis.bar_len = Some(8.0);
    staff_grp.staff_grp_vis.bar_place = Some(DataStaffloc::from(0));
    staff_grp.staff_grp_vis.visible = Some(DataBoolean::True);

    let mut staff_def = StaffDef::default();
    staff_def.n_integer.n = Some(1);
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let xml = staff_grp.to_mei_string().expect("serialize");
    let parsed = StaffGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sg1".to_string()));
    assert_eq!(
        parsed.staff_grp_vis.symbol,
        Some(AttStaffGrpVisSymbol::Brace)
    );
    assert_eq!(parsed.staff_grp_vis.bar_thru, Some(DataBoolean::True));
    assert_eq!(parsed.staff_grp_vis.bar_method, Some(DataBarmethod::Mensur));
    assert_eq!(parsed.staff_grp_vis.bar_len, Some(8.0));
    assert_eq!(parsed.staff_grp_vis.bar_place, Some(DataStaffloc::from(0)));
    assert_eq!(parsed.staff_grp_vis.visible, Some(DataBoolean::True));
}

#[test]
fn staffgrp_roundtrip_with_ges_attributes() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{StaffDef, StaffGrp, StaffGrpChild};

    let mut staff_grp = StaffGrp::default();
    staff_grp.common.xml_id = Some("sg1".to_string());
    staff_grp.staff_grp_ges.instr = Some(DataUri("#strings".to_string()));

    let mut staff_def = StaffDef::default();
    staff_def.n_integer.n = Some(1);
    staff_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(staff_def)));

    let xml = staff_grp.to_mei_string().expect("serialize");
    let parsed = StaffGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sg1".to_string()));
    assert_eq!(
        parsed.staff_grp_ges.instr,
        Some(DataUri("#strings".to_string()))
    );
}

#[test]
fn staffgrp_nested_with_attributes() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef>
        <staffGrp xml:id="outer" symbol="bracket" bar.thru="true">
            <staffGrp xml:id="inner" symbol="brace">
                <staffDef n="1" lines="5"/>
                <staffDef n="2" lines="5"/>
            </staffGrp>
            <staffDef n="3" lines="5"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(outer) => {
            assert_eq!(outer.common.xml_id, Some("outer".to_string()));
            assert_eq!(
                outer.staff_grp_vis.symbol,
                Some(AttStaffGrpVisSymbol::Bracket)
            );
            assert_eq!(outer.staff_grp_vis.bar_thru, Some(DataBoolean::True));
            assert_eq!(outer.children.len(), 2);

            // Check nested staffGrp
            match &outer.children[0] {
                StaffGrpChild::StaffGrp(inner) => {
                    assert_eq!(inner.common.xml_id, Some("inner".to_string()));
                    assert_eq!(
                        inner.staff_grp_vis.symbol,
                        Some(AttStaffGrpVisSymbol::Brace)
                    );
                    assert_eq!(inner.children.len(), 2);
                }
                other => panic!("Expected nested StaffGrp, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

// ============================================================================
// Score Definition Integration Tests
// ============================================================================
//
// These tests verify that scoreDef, staffDef, layerDef, and staffGrp elements
// work together correctly as complete score definition structures. Test cases
// include:
//
// - Complete score definitions from real MEI example files
// - Score redefinition scenarios (key/meter changes mid-score)
// - Complex staffGrp hierarchies with multiple staves
// - Round-trip serialization of complete score definitions
// ============================================================================

// ----------------------------------------------------------------------------
// Tests from specs/mei/examples/verovio/04-score-redefinition.mei
// Score definition with key signature and meter changes
// ----------------------------------------------------------------------------

/// Initial scoreDef with keysig="4f" and meter.sym="common" from 04-score-redefinition.mei
#[test]
fn scoredef_example_key_changes_initial() {
    use tusk_model::data::{DataClefline, DataClefshape, DataMetersign};
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef keysig="4f" meter.sym="common">
        <staffGrp>
            <staffDef n="1" lines="5" clef.shape="G" clef.line="2" />
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    // Check keysig attribute - should contain "4f"
    assert!(
        !parsed.score_def_log.keysig.is_empty(),
        "keysig should be set"
    );
    // Check meter symbol
    assert_eq!(parsed.score_def_log.meter_sym, Some(DataMetersign::Common));

    // Verify staffGrp structure
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.children.len(), 1);
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.n_integer.n, Some(1));
                    assert_eq!(sd.staff_def_log.lines, Some(5));
                    assert_eq!(sd.staff_def_log.clef_shape, Some(DataClefshape::G));
                    assert_eq!(sd.staff_def_log.clef_line, Some(DataClefline(2)));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Score redefinition mid-score: keysig="0" keysig.cancelaccid="none"
#[test]
fn scoredef_example_key_changes_cancel_none() {
    use tusk_model::data::DataCancelaccid;
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef keysig="0" keysig.cancelaccid="none" />"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    // keysig="0" means no key signature
    assert!(!parsed.score_def_log.keysig.is_empty());
    // Verify cancel accidental attribute
    assert_eq!(
        parsed.score_def_vis.keysig_cancelaccid,
        Some(DataCancelaccid::None)
    );
}

/// Score redefinition with new key and meter: keysig="2s" meter.sym="cut"
#[test]
fn scoredef_example_key_changes_new_key_and_meter() {
    use tusk_model::data::{DataCancelaccid, DataMetersign};
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef keysig="2s" keysig.cancelaccid="before" meter.sym="cut" />"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert!(!parsed.score_def_log.keysig.is_empty());
    assert_eq!(
        parsed.score_def_vis.keysig_cancelaccid,
        Some(DataCancelaccid::Before)
    );
    assert_eq!(parsed.score_def_log.meter_sym, Some(DataMetersign::Cut));
}

/// Score redefinition with keysig.visible="false"
#[test]
fn scoredef_example_key_changes_invisible_keysig() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef keysig.visible="false" keysig="5f" meter.count="4" meter.unit="4" />"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.score_def_vis.keysig_visible,
        Some(DataBoolean::False)
    );
    assert!(!parsed.score_def_log.keysig.is_empty());
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
}

/// Score redefinition with keysig.cancelaccid="before-bar"
#[test]
fn scoredef_example_key_changes_cancel_before_bar() {
    use tusk_model::data::DataCancelaccid;
    use tusk_model::elements::ScoreDef;

    let xml = r#"<scoreDef keysig="2s" keysig.cancelaccid="before-bar" />"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert!(!parsed.score_def_log.keysig.is_empty());
    assert_eq!(
        parsed.score_def_vis.keysig_cancelaccid,
        Some(DataCancelaccid::BeforeBar)
    );
}

// ----------------------------------------------------------------------------
// Tests from specs/mei/examples/verovio/tchaikovsky_scherzo.mei
// String quartet score definition with multiple staves
// ----------------------------------------------------------------------------

/// Complete scoreDef from Tchaikovsky string quartet with 4 staves
#[test]
fn scoredef_example_tchaikovsky_string_quartet() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::{DataClefline, DataClefshape};
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffDefChild, StaffGrpChild};

    let xml = r#"<scoreDef>
        <staffGrp symbol="bracket">
            <staffDef n="1" lines="5" keysig="5f">
                <clef shape="G" line="2" />
            </staffDef>
            <staffDef n="2" lines="5" keysig="5f">
                <clef shape="G" line="2" />
            </staffDef>
            <staffDef n="3" lines="5" keysig="5f">
                <clef shape="C" line="3" />
            </staffDef>
            <staffDef n="4" lines="5" keysig="5f">
                <clef shape="F" line="4" />
            </staffDef>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    // Verify staffGrp with bracket symbol
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.staff_grp_vis.symbol, Some(AttStaffGrpVisSymbol::Bracket));
            assert_eq!(sg.children.len(), 4);

            // Check each staff definition
            for (i, child) in sg.children.iter().enumerate() {
                match child {
                    StaffGrpChild::StaffDef(sd) => {
                        assert_eq!(sd.n_integer.n, Some((i + 1) as u64));
                        assert_eq!(sd.staff_def_log.lines, Some(5));
                        assert!(!sd.staff_def_log.keysig.is_empty(), "keysig should be set");

                        // Check clef child element
                        assert_eq!(sd.children.len(), 1);
                        match &sd.children[0] {
                            StaffDefChild::Clef(clef) => {
                                match i {
                                    0 | 1 => {
                                        // Violin I & II: treble clef
                                        assert_eq!(clef.clef_log.shape, Some(DataClefshape::G));
                                        assert_eq!(clef.clef_log.line, Some(DataClefline(2)));
                                    }
                                    2 => {
                                        // Viola: alto clef
                                        assert_eq!(clef.clef_log.shape, Some(DataClefshape::C));
                                        assert_eq!(clef.clef_log.line, Some(DataClefline(3)));
                                    }
                                    3 => {
                                        // Cello: bass clef
                                        assert_eq!(clef.clef_log.shape, Some(DataClefshape::F));
                                        assert_eq!(clef.clef_log.line, Some(DataClefline(4)));
                                    }
                                    _ => unreachable!(),
                                }
                            }
                            other => panic!("Expected Clef, got {:?}", other),
                        }
                    }
                    other => panic!("Expected StaffDef, got {:?}", other),
                }
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

// ----------------------------------------------------------------------------
// Score Definition Round-Trip Tests
// Verify complete score definition structures serialize and deserialize correctly
// ----------------------------------------------------------------------------

/// Round-trip test for complete orchestral score definition
#[test]
fn scoredef_roundtrip_complete_orchestral() {
    use tusk_model::att::AttStaffGrpVisSymbol;
    use tusk_model::data::{
        DataBoolean, DataClefline, DataClefshape, DataMetersign, DataNotationtype,
    };
    use tusk_model::elements::{Clef, ScoreDef, ScoreDefChild, StaffDef, StaffGrp, StaffGrpChild};

    // Build a complete orchestral-style score definition
    let mut score_def = ScoreDef::default();
    score_def.common.xml_id = Some("sd1".to_string());
    score_def.score_def_log.meter_count = Some("4".to_string());
    score_def.score_def_log.meter_unit = Some(4.0);
    score_def.score_def_log.meter_sym = Some(DataMetersign::Common);
    score_def.score_def_ges.ppq = Some(480);

    // Create outer bracket group
    let mut outer_grp = StaffGrp::default();
    outer_grp.common.xml_id = Some("sg-outer".to_string());
    outer_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Bracket);
    outer_grp.staff_grp_vis.bar_thru = Some(DataBoolean::True);

    // Create inner brace group for piano grand staff
    let mut piano_grp = StaffGrp::default();
    piano_grp.common.xml_id = Some("sg-piano".to_string());
    // Note: StaffGrp uses child <label> elements for labelling, not an attribute
    piano_grp.staff_grp_vis.symbol = Some(AttStaffGrpVisSymbol::Brace);

    // Piano right hand staff
    let mut rh_staff = StaffDef::default();
    rh_staff.basic.xml_id = Some("sd-rh".to_string());
    rh_staff.n_integer.n = Some(1);
    rh_staff.staff_def_log.lines = Some(5);
    rh_staff.staff_def_log.notationtype = Some(DataNotationtype::Cmn);
    let mut rh_clef = Clef::default();
    rh_clef.clef_log.shape = Some(DataClefshape::G);
    rh_clef.clef_log.line = Some(DataClefline(2));
    rh_staff
        .children
        .push(tusk_model::elements::StaffDefChild::Clef(Box::new(rh_clef)));

    // Piano left hand staff
    let mut lh_staff = StaffDef::default();
    lh_staff.basic.xml_id = Some("sd-lh".to_string());
    lh_staff.n_integer.n = Some(2);
    lh_staff.staff_def_log.lines = Some(5);
    lh_staff.staff_def_log.notationtype = Some(DataNotationtype::Cmn);
    let mut lh_clef = Clef::default();
    lh_clef.clef_log.shape = Some(DataClefshape::F);
    lh_clef.clef_log.line = Some(DataClefline(4));
    lh_staff
        .children
        .push(tusk_model::elements::StaffDefChild::Clef(Box::new(lh_clef)));

    piano_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(rh_staff)));
    piano_grp
        .children
        .push(StaffGrpChild::StaffDef(Box::new(lh_staff)));

    outer_grp
        .children
        .push(StaffGrpChild::StaffGrp(Box::new(piano_grp)));

    score_def
        .children
        .push(ScoreDefChild::StaffGrp(Box::new(outer_grp)));

    // Round-trip
    let xml = score_def.to_mei_string().expect("serialize");
    let parsed = ScoreDef::from_mei_str(&xml).expect("deserialize");

    // Verify top-level attributes
    assert_eq!(parsed.common.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.score_def_log.meter_count, Some("4".to_string()));
    assert_eq!(parsed.score_def_log.meter_unit, Some(4.0));
    assert_eq!(parsed.score_def_log.meter_sym, Some(DataMetersign::Common));
    assert_eq!(parsed.score_def_ges.ppq, Some(480));

    // Verify nested structure
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(outer) => {
            assert_eq!(outer.common.xml_id, Some("sg-outer".to_string()));
            assert_eq!(
                outer.staff_grp_vis.symbol,
                Some(AttStaffGrpVisSymbol::Bracket)
            );
            assert_eq!(outer.staff_grp_vis.bar_thru, Some(DataBoolean::True));
            assert_eq!(outer.children.len(), 1);

            match &outer.children[0] {
                StaffGrpChild::StaffGrp(piano) => {
                    assert_eq!(piano.common.xml_id, Some("sg-piano".to_string()));
                    // StaffGrp uses child <label> elements, not an attribute
                    assert_eq!(
                        piano.staff_grp_vis.symbol,
                        Some(AttStaffGrpVisSymbol::Brace)
                    );
                    assert_eq!(piano.children.len(), 2);

                    // Check staff definitions
                    match &piano.children[0] {
                        StaffGrpChild::StaffDef(sd) => {
                            assert_eq!(sd.basic.xml_id, Some("sd-rh".to_string()));
                            assert_eq!(sd.n_integer.n, Some(1));
                        }
                        other => panic!("Expected StaffDef, got {:?}", other),
                    }
                    match &piano.children[1] {
                        StaffGrpChild::StaffDef(sd) => {
                            assert_eq!(sd.basic.xml_id, Some("sd-lh".to_string()));
                            assert_eq!(sd.n_integer.n, Some(2));
                        }
                        other => panic!("Expected StaffDef, got {:?}", other),
                    }
                }
                other => panic!("Expected nested StaffGrp, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Parse test for staffDef with layerDef children
/// Note: StaffDef child serialization not yet implemented, so this is a parse test
#[test]
fn staffdef_parse_with_layerdef_children() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave};
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef xml:id="sd1" n="1" lines="5">
        <layerDef xml:id="ld1" n="1" label="Soprano" dur.default="4" oct.default="5"/>
        <layerDef xml:id="ld2" n="2" label="Alto" dur.default="4" oct.default="4"/>
    </staffDef>"#;

    let parsed = StaffDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 2);

    // Verify layer definitions
    match &parsed.children[0] {
        StaffDefChild::LayerDef(ld) => {
            assert_eq!(ld.basic.xml_id, Some("ld1".to_string()));
            assert_eq!(ld.n_integer.n, Some(1));
            assert_eq!(ld.labelled.label, Some("Soprano".to_string()));
            assert_eq!(
                ld.layer_def_log.dur_default,
                Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
            );
            assert_eq!(ld.layer_def_log.oct_default, Some(DataOctave(5)));
        }
        other => panic!("Expected LayerDef, got {:?}", other),
    }

    match &parsed.children[1] {
        StaffDefChild::LayerDef(ld) => {
            assert_eq!(ld.basic.xml_id, Some("ld2".to_string()));
            assert_eq!(ld.n_integer.n, Some(2));
            assert_eq!(ld.labelled.label, Some("Alto".to_string()));
            assert_eq!(ld.layer_def_log.oct_default, Some(DataOctave(4)));
        }
        other => panic!("Expected LayerDef, got {:?}", other),
    }
}

/// Parse test for score definition with transposing instruments
/// Note: This is a parse test, not round-trip, because StaffDef serialization
/// doesn't yet include trans.diat/trans.semi attributes
#[test]
fn scoredef_parse_transposing_instruments() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef xml:id="sd-trans">
        <staffGrp>
            <staffDef xml:id="sd-clarinet" label="Clarinet in Bb" n="1" lines="5"
                      clef.shape="G" clef.line="2" trans.diat="-1" trans.semi="-2"/>
            <staffDef xml:id="sd-horn" label="Horn in F" n="2" lines="5"
                      clef.shape="G" clef.line="2" trans.diat="-4" trans.semi="-7"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("sd-trans".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.children.len(), 2);

            // Check clarinet transposition
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.labelled.label, Some("Clarinet in Bb".to_string()));
                    assert_eq!(sd.staff_def_log.trans_diat, Some(-1));
                    assert_eq!(sd.staff_def_log.trans_semi, Some(-2));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }

            // Check horn transposition
            match &sg.children[1] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.labelled.label, Some("Horn in F".to_string()));
                    assert_eq!(sd.staff_def_log.trans_diat, Some(-4));
                    assert_eq!(sd.staff_def_log.trans_semi, Some(-7));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Test parsing scoreDef with multiple child types (keySig, meterSig, staffGrp)
/// Verifies that different child element types are correctly identified and ordered
#[test]
fn scoredef_parse_mixed_children() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef xml:id="sd-mixed">
        <keySig xml:id="ks1"/>
        <meterSig xml:id="ms1"/>
        <staffGrp xml:id="sg1">
            <staffDef n="1" lines="5"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("sd-mixed".to_string()));
    assert_eq!(parsed.children.len(), 3);

    // Verify child order preserved
    match &parsed.children[0] {
        ScoreDefChild::KeySig(ks) => {
            assert_eq!(ks.common.xml_id, Some("ks1".to_string()));
        }
        other => panic!("Expected KeySig, got {:?}", other),
    }

    match &parsed.children[1] {
        ScoreDefChild::MeterSig(ms) => {
            // Note: MeterSig-specific attributes (count, unit) not yet parsed
            assert_eq!(ms.common.xml_id, Some("ms1".to_string()));
        }
        other => panic!("Expected MeterSig, got {:?}", other),
    }

    match &parsed.children[2] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.children.len(), 1);
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.n_integer.n, Some(1));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Test staffGrp structure with nested staffDef children
/// Note: grpSym parsing not yet implemented, so testing staffDef children only
#[test]
fn staffgrp_parse_with_staffdef_children() {
    use tusk_model::elements::{ScoreDef, ScoreDefChild, StaffGrpChild};

    let xml = r#"<scoreDef>
        <staffGrp xml:id="sg1">
            <staffDef n="1" lines="5"/>
            <staffDef n="2" lines="5"/>
        </staffGrp>
    </scoreDef>"#;

    let parsed = ScoreDef::from_mei_str(xml).expect("parse");

    match &parsed.children[0] {
        ScoreDefChild::StaffGrp(sg) => {
            assert_eq!(sg.common.xml_id, Some("sg1".to_string()));
            assert_eq!(sg.children.len(), 2);

            // First child should be staffDef n="1"
            match &sg.children[0] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.n_integer.n, Some(1));
                    assert_eq!(sd.staff_def_log.lines, Some(5));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }

            // Second child should be staffDef n="2"
            match &sg.children[1] {
                StaffGrpChild::StaffDef(sd) => {
                    assert_eq!(sd.n_integer.n, Some(2));
                }
                other => panic!("Expected StaffDef, got {:?}", other),
            }
        }
        other => panic!("Expected StaffGrp, got {:?}", other),
    }
}

/// Test staffDef with instrDef child
#[test]
fn staffdef_parse_with_instrdef_child() {
    use tusk_model::elements::StaffDef;

    let xml = r#"<staffDef xml:id="sd1" n="1" lines="5">
        <instrDef xml:id="id1" midi.instrnum="1" />
    </staffDef>"#;

    let parsed = StaffDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.basic.xml_id, Some("sd1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        tusk_model::elements::StaffDefChild::InstrDef(id) => {
            assert_eq!(id.basic.xml_id, Some("id1".to_string()));
            // Note: InstrDef midi_instrnum not yet parsed from attributes
        }
        other => panic!("Expected InstrDef, got {:?}", other),
    }
}

/// Test staffDef with label and labelAbbr children
#[test]
fn staffdef_parse_with_label_children() {
    use tusk_model::elements::{StaffDef, StaffDefChild};

    let xml = r#"<staffDef xml:id="sd1" n="1" lines="5">
        <label xml:id="l1">Violin I</label>
        <labelAbbr xml:id="la1">Vln. I</labelAbbr>
    </staffDef>"#;

    let parsed = StaffDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        StaffDefChild::Label(label) => {
            assert_eq!(label.common.xml_id, Some("l1".to_string()));
            // Label content should be in text children
        }
        other => panic!("Expected Label, got {:?}", other),
    }

    match &parsed.children[1] {
        StaffDefChild::LabelAbbr(la) => {
            assert_eq!(la.common.xml_id, Some("la1".to_string()));
        }
        other => panic!("Expected LabelAbbr, got {:?}", other),
    }
}

/// Test layerDef with instrDef child
#[test]
fn layerdef_parse_with_instrdef_child() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef xml:id="ld1" n="1">
        <instrDef xml:id="id1" />
    </layerDef>"#;

    let parsed = LayerDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LayerDefChild::InstrDef(id) => {
            assert_eq!(id.basic.xml_id, Some("id1".to_string()));
            // Note: InstrDef midi attributes not yet parsed
        }
        other => panic!("Expected InstrDef, got {:?}", other),
    }
}

/// Test parsing layerDef with multiple children
/// Note: LayerDef child serialization not yet implemented, so this is a parse test
#[test]
fn layerdef_parse_with_multiple_children() {
    use tusk_model::elements::{LayerDef, LayerDefChild};

    let xml = r#"<layerDef xml:id="ld1" n="1">
        <label xml:id="l1">Voice I</label>
        <labelAbbr xml:id="la1">V.I</labelAbbr>
        <instrDef xml:id="id1"/>
        <meterSig xml:id="ms1"/>
    </layerDef>"#;

    let parsed = LayerDef::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.basic.xml_id, Some("ld1".to_string()));
    assert_eq!(parsed.children.len(), 4);

    // Verify child types preserved in order
    assert!(matches!(&parsed.children[0], LayerDefChild::Label(_)));
    assert!(matches!(&parsed.children[1], LayerDefChild::LabelAbbr(_)));
    assert!(matches!(&parsed.children[2], LayerDefChild::InstrDef(_)));
    assert!(matches!(&parsed.children[3], LayerDefChild::MeterSig(_)));
}

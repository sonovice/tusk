//! Round-trip serialization tests for control event elements.
//!
//! Tests for Slur, Tie, Dynam, Hairpin, Dir, Tempo, Fermata elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Control Event Tests - Slur
// ============================================================================

#[test]
fn slur_parse_empty() {
    use tusk_model::elements::Slur;

    let xml = r#"<slur/>"#;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.slur_log.startid.is_none());
    assert!(parsed.slur_log.endid.is_none());
}

#[test]
fn slur_parse_with_id() {
    use tusk_model::elements::Slur;

    let xml = r#"<slur xml:id="slur-1"/>"#;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("slur-1".to_string()));
}

#[test]
fn slur_parse_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Slur;

    let xml = r##"<slur xml:id="slur-1" startid="#note1" endid="#note2"/>"##;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("slur-1".to_string()));
    assert_eq!(parsed.slur_log.startid, Some(DataUri("#note1".to_string())));
    assert_eq!(parsed.slur_log.endid, Some(DataUri("#note2".to_string())));
}

#[test]
fn slur_parse_with_staff_layer() {
    use tusk_model::elements::Slur;

    let xml = r#"<slur staff="1" layer="1"/>"#;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.slur_log.staff, vec![1]);
    assert_eq!(parsed.slur_log.layer, vec![1]);
}

#[test]
fn slur_parse_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Slur;

    let xml = r#"<slur tstamp="1" tstamp2="0m+4"/>"#;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.slur_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.slur_log.tstamp2,
        Some(DataMeasurebeat("0m+4".to_string()))
    );
}

#[test]
fn slur_parse_complete() {
    use tusk_model::elements::Slur;

    let xml = r##"<slur xml:id="slur1" startid="#n1" endid="#n4" staff="1" layer="1"/>"##;
    let parsed = Slur::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("slur1".to_string()));
    assert!(parsed.slur_log.startid.is_some());
    assert!(parsed.slur_log.endid.is_some());
    assert_eq!(parsed.slur_log.staff, vec![1]);
    assert_eq!(parsed.slur_log.layer, vec![1]);
}

// ============================================================================
// Control Event Tests - Tie
// ============================================================================

#[test]
fn tie_parse_empty() {
    use tusk_model::elements::Tie;

    let xml = r#"<tie/>"#;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.tie_log.startid.is_none());
    assert!(parsed.tie_log.endid.is_none());
}

#[test]
fn tie_parse_with_id() {
    use tusk_model::elements::Tie;

    let xml = r#"<tie xml:id="tie-1"/>"#;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tie-1".to_string()));
}

#[test]
fn tie_parse_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Tie;

    let xml = r##"<tie xml:id="tie-1" startid="#note1" endid="#note2"/>"##;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tie-1".to_string()));
    assert_eq!(parsed.tie_log.startid, Some(DataUri("#note1".to_string())));
    assert_eq!(parsed.tie_log.endid, Some(DataUri("#note2".to_string())));
}

#[test]
fn tie_parse_with_staff_layer() {
    use tusk_model::elements::Tie;

    let xml = r#"<tie staff="1" layer="1"/>"#;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tie_log.staff, vec![1]);
    assert_eq!(parsed.tie_log.layer, vec![1]);
}

#[test]
fn tie_parse_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Tie;

    let xml = r#"<tie tstamp="2.5" tstamp2="1m+1"/>"#;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tie_log.tstamp, Some(DataBeat(2.5)));
    assert_eq!(
        parsed.tie_log.tstamp2,
        Some(DataMeasurebeat("1m+1".to_string()))
    );
}

#[test]
fn tie_parse_complete() {
    use tusk_model::elements::Tie;

    let xml = r##"<tie xml:id="tie1" startid="#n1" endid="#n2" staff="1"/>"##;
    let parsed = Tie::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tie1".to_string()));
    assert!(parsed.tie_log.startid.is_some());
    assert!(parsed.tie_log.endid.is_some());
    assert_eq!(parsed.tie_log.staff, vec![1]);
}

// ============================================================================
// Control Event Tests - Dynam
// ============================================================================

#[test]
fn dynam_parse_empty() {
    use tusk_model::elements::Dynam;

    let xml = r#"<dynam/>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn dynam_parse_with_id() {
    use tusk_model::elements::Dynam;

    let xml = r#"<dynam xml:id="dyn-1"/>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("dyn-1".to_string()));
}

#[test]
fn dynam_parse_with_text() {
    use tusk_model::elements::{Dynam, DynamChild};

    let xml = r#"<dynam xml:id="dyn-1">ff</dynam>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("dyn-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DynamChild::Text(t) => assert_eq!(t, "ff"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn dynam_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Dynam;

    let xml = r#"<dynam staff="1" tstamp="1"/>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.dynam_log.staff, vec![1]);
    assert_eq!(parsed.dynam_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn dynam_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Dynam;

    let xml = r##"<dynam startid="#note1"/>"##;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.dynam_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn dynam_parse_complete() {
    use tusk_model::elements::{Dynam, DynamChild};

    let xml = r#"<dynam xml:id="d1" staff="1" tstamp="1">p</dynam>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("d1".to_string()));
    assert_eq!(parsed.dynam_log.staff, vec![1]);
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DynamChild::Text(t) => assert_eq!(t, "p"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn dynam_parse_with_tstamp2() {
    use tusk_model::elements::{Dynam, DynamChild};

    // Test a dynamic with crescendo text and tstamp2
    let xml = r#"<dynam xml:id="d1" staff="1" tstamp="1" tstamp2="0m+4">cresc.</dynam>"#;
    let parsed = Dynam::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("d1".to_string()));
    assert!(parsed.dynam_log.tstamp.is_some());
    assert!(parsed.dynam_log.tstamp2.is_some());
    match &parsed.children[0] {
        DynamChild::Text(t) => assert_eq!(t, "cresc."),
        _ => panic!("Expected text child"),
    }
}

// ============================================================================
// Control Event Tests - Hairpin
// ============================================================================

#[test]
fn hairpin_parse_empty() {
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.hairpin_log.form.is_none());
}

#[test]
fn hairpin_parse_with_id() {
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin xml:id="hp-1"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("hp-1".to_string()));
}

#[test]
fn hairpin_parse_crescendo() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin xml:id="hp-1" form="cres"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("hp-1".to_string()));
    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Cres));
}

#[test]
fn hairpin_parse_diminuendo() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin xml:id="hp-1" form="dim"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("hp-1".to_string()));
    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Dim));
}

#[test]
fn hairpin_parse_with_niente() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin form="dim" niente="true"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Dim));
    assert_eq!(parsed.hairpin_log.niente, Some(DataBoolean::True));
}

#[test]
fn hairpin_parse_with_staff_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin staff="1" tstamp="1" tstamp2="0m+3"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.hairpin_log.staff, vec![1]);
    assert_eq!(parsed.hairpin_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.hairpin_log.tstamp2,
        Some(DataMeasurebeat("0m+3".to_string()))
    );
}

#[test]
fn hairpin_parse_complete() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let xml = r#"<hairpin xml:id="hp1" form="cres" staff="1" tstamp="1" tstamp2="0m+4"/>"#;
    let parsed = Hairpin::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("hp1".to_string()));
    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Cres));
    assert_eq!(parsed.hairpin_log.staff, vec![1]);
    assert!(parsed.hairpin_log.tstamp.is_some());
    assert!(parsed.hairpin_log.tstamp2.is_some());
}

// ============================================================================
// Control Event Tests - Dir
// ============================================================================

#[test]
fn dir_parse_empty() {
    use tusk_model::elements::Dir;

    let xml = r#"<dir/>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn dir_parse_with_id() {
    use tusk_model::elements::Dir;

    let xml = r#"<dir xml:id="dir-1"/>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("dir-1".to_string()));
}

#[test]
fn dir_parse_with_text() {
    use tusk_model::elements::{Dir, DirChild};

    let xml = r#"<dir xml:id="dir-1">legato</dir>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("dir-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DirChild::Text(t) => assert_eq!(t, "legato"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn dir_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Dir;

    let xml = r#"<dir staff="1" tstamp="1"/>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.dir_log.staff, vec![1]);
    assert_eq!(parsed.dir_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn dir_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Dir;

    let xml = r##"<dir startid="#note1"/>"##;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.dir_log.startid, Some(DataUri("#note1".to_string())));
}

#[test]
fn dir_parse_complete() {
    use tusk_model::elements::{Dir, DirChild};

    let xml = r#"<dir xml:id="d1" staff="1" tstamp="1">dolce</dir>"#;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("d1".to_string()));
    assert_eq!(parsed.dir_log.staff, vec![1]);
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DirChild::Text(t) => assert_eq!(t, "dolce"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn dir_parse_with_endid() {
    use tusk_model::elements::{Dir, DirChild};

    // Test a directive with extended duration
    let xml = r##"<dir xml:id="d1" staff="1" tstamp="1" endid="#n4">sempre legato</dir>"##;
    let parsed = Dir::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("d1".to_string()));
    assert!(parsed.dir_log.tstamp.is_some());
    assert!(parsed.dir_log.endid.is_some());
    match &parsed.children[0] {
        DirChild::Text(t) => assert_eq!(t, "sempre legato"),
        _ => panic!("Expected text child"),
    }
}

// ============================================================================
// Control Event Tests - Tempo
// ============================================================================

#[test]
fn tempo_parse_empty() {
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn tempo_parse_with_id() {
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo xml:id="tempo-1"/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tempo-1".to_string()));
}

#[test]
fn tempo_parse_with_text() {
    use tusk_model::elements::{Tempo, TempoChild};

    let xml = r#"<tempo xml:id="tempo-1">Allegro</tempo>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tempo-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        TempoChild::Text(t) => assert_eq!(t, "Allegro"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn tempo_parse_with_mm() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataTempovalue};
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo xml:id="tempo-1" mm="120" mm.unit="4"/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("tempo-1".to_string()));
    assert_eq!(parsed.tempo_log.mm, Some(DataTempovalue(120.0)));
    assert_eq!(
        parsed.tempo_log.mm_unit,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn tempo_parse_with_func() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo func="instantaneous"/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tempo_log.func, Some(AttTempoLogFunc::Instantaneous));
}

#[test]
fn tempo_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Tempo;

    let xml = r#"<tempo staff="1" tstamp="1"/>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tempo_log.staff, vec![1]);
    assert_eq!(parsed.tempo_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn tempo_parse_complete() {
    use tusk_model::elements::{Tempo, TempoChild};

    let xml = r#"<tempo xml:id="t1" staff="1" tstamp="1" mm="120" mm.unit="4">Allegro</tempo>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("t1".to_string()));
    assert_eq!(parsed.tempo_log.staff, vec![1]);
    assert!(parsed.tempo_log.mm.is_some());
    assert!(parsed.tempo_log.mm_unit.is_some());
    match &parsed.children[0] {
        TempoChild::Text(t) => assert_eq!(t, "Allegro"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn tempo_parse_continuous_func() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::elements::{Tempo, TempoChild};

    // Test a tempo marking with continuous function (like rit. or accel.)
    let xml =
        r#"<tempo xml:id="t1" staff="1" tstamp="1" tstamp2="0m+4" func="continuous">rit.</tempo>"#;
    let parsed = Tempo::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.tempo_log.func, Some(AttTempoLogFunc::Continuous));
    assert!(parsed.tempo_log.tstamp2.is_some());
    match &parsed.children[0] {
        TempoChild::Text(t) => assert_eq!(t, "rit."),
        _ => panic!("Expected text child"),
    }
}

// ============================================================================
// Control Event Tests - Fermata
// ============================================================================

#[test]
fn fermata_parse_empty() {
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.fermata_vis.form.is_none());
    assert!(parsed.fermata_vis.shape.is_none());
}

#[test]
fn fermata_parse_with_id() {
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata xml:id="ferm-1"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("ferm-1".to_string()));
}

#[test]
fn fermata_parse_with_form_norm() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata xml:id="ferm-1" form="norm"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("ferm-1".to_string()));
    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Norm));
}

#[test]
fn fermata_parse_with_form_inv() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata form="inv"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Inv));
}

#[test]
fn fermata_parse_with_shape_curved() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata shape="curved"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Curved));
}

#[test]
fn fermata_parse_with_shape_square() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata shape="square"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Square));
}

#[test]
fn fermata_parse_with_shape_angular() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata shape="angular"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Angular));
}

#[test]
fn fermata_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata staff="1" tstamp="4"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.fermata_log.staff, vec![1]);
    assert_eq!(parsed.fermata_log.tstamp, Some(DataBeat(4.0)));
}

#[test]
fn fermata_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Fermata;

    let xml = r##"<fermata startid="#note1"/>"##;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.fermata_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn fermata_parse_complete() {
    use tusk_model::att::{AttFermataVisForm, AttFermataVisShape};
    use tusk_model::elements::Fermata;

    let xml = r#"<fermata xml:id="f1" staff="1" tstamp="4" form="norm" shape="curved"/>"#;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("f1".to_string()));
    assert_eq!(parsed.fermata_log.staff, vec![1]);
    assert!(parsed.fermata_log.tstamp.is_some());
    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Norm));
    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Curved));
}

#[test]
fn fermata_parse_inverted() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let xml = r##"<fermata xml:id="f1" startid="#n1" form="inv"/>"##;
    let parsed = Fermata::from_mei_str(xml).expect("parse");

    assert!(parsed.fermata_log.startid.is_some());
    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Inv));
}

// ============================================================================
// Slur Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_slur_empty() {
    use tusk_model::elements::Slur;

    let original = Slur::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.slur_log.startid.is_none());
    assert!(parsed.slur_log.endid.is_none());
}

#[test]
fn roundtrip_slur_with_xml_id() {
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.common.xml_id = Some("slur-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("slur-1".to_string()));
}

#[test]
fn roundtrip_slur_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.common.xml_id = Some("s1".to_string());
    original.slur_log.startid = Some(DataUri("#n1".to_string()));
    original.slur_log.endid = Some(DataUri("#n4".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.slur_log.startid, Some(DataUri("#n1".to_string())));
    assert_eq!(parsed.slur_log.endid, Some(DataUri("#n4".to_string())));
}

#[test]
fn roundtrip_slur_with_staff_layer() {
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.slur_log.staff = vec![1];
    original.slur_log.layer = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.slur_log.staff, vec![1]);
    assert_eq!(parsed.slur_log.layer, vec![1]);
}

#[test]
fn roundtrip_slur_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.slur_log.tstamp = Some(DataBeat(1.0));
    original.slur_log.tstamp2 = Some(DataMeasurebeat("0m+4".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.slur_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.slur_log.tstamp2,
        Some(DataMeasurebeat("0m+4".to_string()))
    );
}

#[test]
fn roundtrip_slur_with_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.slur_vis.color = Some(DataColor::DataColornames(DataColornames::Blue));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.slur_vis.color,
        Some(DataColor::DataColornames(DataColornames::Blue))
    );
}

#[test]
fn roundtrip_slur_complete() {
    use tusk_model::data::{DataBeat, DataColor, DataColornames, DataMeasurebeat, DataUri};
    use tusk_model::elements::Slur;

    let mut original = Slur::default();
    original.common.xml_id = Some("slur-complete".to_string());
    original.slur_log.startid = Some(DataUri("#n1".to_string()));
    original.slur_log.endid = Some(DataUri("#n8".to_string()));
    original.slur_log.staff = vec![1];
    original.slur_log.layer = vec![1];
    original.slur_log.tstamp = Some(DataBeat(1.0));
    original.slur_log.tstamp2 = Some(DataMeasurebeat("1m+1".to_string()));
    original.slur_vis.color = Some(DataColor::DataColornames(DataColornames::Red));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Slur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.slur_log.startid, original.slur_log.startid);
    assert_eq!(parsed.slur_log.endid, original.slur_log.endid);
    assert_eq!(parsed.slur_log.staff, original.slur_log.staff);
    assert_eq!(parsed.slur_log.layer, original.slur_log.layer);
    assert_eq!(parsed.slur_log.tstamp, original.slur_log.tstamp);
    assert_eq!(parsed.slur_log.tstamp2, original.slur_log.tstamp2);
    assert_eq!(parsed.slur_vis.color, original.slur_vis.color);
}

// ============================================================================
// Tie Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_tie_empty() {
    use tusk_model::elements::Tie;

    let original = Tie::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.tie_log.startid.is_none());
    assert!(parsed.tie_log.endid.is_none());
}

#[test]
fn roundtrip_tie_with_xml_id() {
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.common.xml_id = Some("tie-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tie-1".to_string()));
}

#[test]
fn roundtrip_tie_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.tie_log.startid = Some(DataUri("#n1".to_string()));
    original.tie_log.endid = Some(DataUri("#n2".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tie_log.startid, Some(DataUri("#n1".to_string())));
    assert_eq!(parsed.tie_log.endid, Some(DataUri("#n2".to_string())));
}

#[test]
fn roundtrip_tie_with_staff() {
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.tie_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tie_log.staff, vec![1]);
}

#[test]
fn roundtrip_tie_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.tie_log.tstamp = Some(DataBeat(2.5));
    original.tie_log.tstamp2 = Some(DataMeasurebeat("1m+1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tie_log.tstamp, Some(DataBeat(2.5)));
    assert_eq!(
        parsed.tie_log.tstamp2,
        Some(DataMeasurebeat("1m+1".to_string()))
    );
}

#[test]
fn roundtrip_tie_with_color() {
    use tusk_model::data::{DataColor, DataColornames};
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.tie_vis.color = Some(DataColor::DataColornames(DataColornames::Blue));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.tie_vis.color,
        Some(DataColor::DataColornames(DataColornames::Blue))
    );
}

#[test]
fn roundtrip_tie_complete() {
    use tusk_model::data::{DataBeat, DataColor, DataColornames, DataMeasurebeat, DataUri};
    use tusk_model::elements::Tie;

    let mut original = Tie::default();
    original.common.xml_id = Some("tie-complete".to_string());
    original.tie_log.startid = Some(DataUri("#n1".to_string()));
    original.tie_log.endid = Some(DataUri("#n2".to_string()));
    original.tie_log.staff = vec![1];
    original.tie_log.layer = vec![1];
    original.tie_log.tstamp = Some(DataBeat(4.0));
    original.tie_log.tstamp2 = Some(DataMeasurebeat("1m+1".to_string()));
    original.tie_vis.color = Some(DataColor::DataColornames(DataColornames::Red));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tie::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.tie_log.startid, original.tie_log.startid);
    assert_eq!(parsed.tie_log.endid, original.tie_log.endid);
    assert_eq!(parsed.tie_log.staff, original.tie_log.staff);
    assert_eq!(parsed.tie_log.layer, original.tie_log.layer);
    assert_eq!(parsed.tie_log.tstamp, original.tie_log.tstamp);
    assert_eq!(parsed.tie_log.tstamp2, original.tie_log.tstamp2);
    assert_eq!(parsed.tie_vis.color, original.tie_vis.color);
}

// ============================================================================
// Dynam Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_dynam_empty() {
    use tusk_model::elements::Dynam;

    let original = Dynam::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_dynam_with_xml_id() {
    use tusk_model::elements::Dynam;

    let mut original = Dynam::default();
    original.common.xml_id = Some("dyn-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("dyn-1".to_string()));
}

#[test]
fn roundtrip_dynam_with_text() {
    use tusk_model::elements::{Dynam, DynamChild};

    let mut original = Dynam::default();
    original.common.xml_id = Some("d1".to_string());
    original.children.push(DynamChild::Text("ff".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DynamChild::Text(t) => assert_eq!(t, "ff"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn roundtrip_dynam_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Dynam;

    let mut original = Dynam::default();
    original.dynam_log.staff = vec![1];
    original.dynam_log.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dynam_log.staff, vec![1]);
    assert_eq!(parsed.dynam_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn roundtrip_dynam_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Dynam;

    let mut original = Dynam::default();
    original.dynam_log.startid = Some(DataUri("#note1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.dynam_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn roundtrip_dynam_with_tstamp2() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::{Dynam, DynamChild};

    let mut original = Dynam::default();
    original.dynam_log.tstamp = Some(DataBeat(1.0));
    original.dynam_log.tstamp2 = Some(DataMeasurebeat("0m+4".to_string()));
    original
        .children
        .push(DynamChild::Text("cresc.".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dynam_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.dynam_log.tstamp2,
        Some(DataMeasurebeat("0m+4".to_string()))
    );
}

#[test]
fn roundtrip_dynam_complete() {
    use tusk_model::data::{DataBeat, DataMeasurebeat, DataUri};
    use tusk_model::elements::{Dynam, DynamChild};

    let mut original = Dynam::default();
    original.common.xml_id = Some("dyn-complete".to_string());
    original.dynam_log.staff = vec![1, 2];
    original.dynam_log.layer = vec![1];
    original.dynam_log.tstamp = Some(DataBeat(1.0));
    original.dynam_log.tstamp2 = Some(DataMeasurebeat("2m+1".to_string()));
    original.dynam_log.startid = Some(DataUri("#n1".to_string()));
    original.children.push(DynamChild::Text("sfz".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dynam::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.dynam_log.staff, original.dynam_log.staff);
    assert_eq!(parsed.dynam_log.layer, original.dynam_log.layer);
    assert_eq!(parsed.dynam_log.tstamp, original.dynam_log.tstamp);
    assert_eq!(parsed.dynam_log.tstamp2, original.dynam_log.tstamp2);
    assert_eq!(parsed.dynam_log.startid, original.dynam_log.startid);
    assert_eq!(parsed.children.len(), 1);
}

// ============================================================================
// Hairpin Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_hairpin_empty() {
    use tusk_model::elements::Hairpin;

    let original = Hairpin::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.hairpin_log.form.is_none());
}

#[test]
fn roundtrip_hairpin_with_xml_id() {
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.common.xml_id = Some("hp-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("hp-1".to_string()));
}

#[test]
fn roundtrip_hairpin_crescendo() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.hairpin_log.form = Some(AttHairpinLogForm::Cres);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Cres));
}

#[test]
fn roundtrip_hairpin_diminuendo() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.hairpin_log.form = Some(AttHairpinLogForm::Dim);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Dim));
}

#[test]
fn roundtrip_hairpin_with_niente() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.hairpin_log.form = Some(AttHairpinLogForm::Dim);
    original.hairpin_log.niente = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.hairpin_log.form, Some(AttHairpinLogForm::Dim));
    assert_eq!(parsed.hairpin_log.niente, Some(DataBoolean::True));
}

#[test]
fn roundtrip_hairpin_with_staff_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.hairpin_log.staff = vec![1];
    original.hairpin_log.tstamp = Some(DataBeat(1.0));
    original.hairpin_log.tstamp2 = Some(DataMeasurebeat("0m+3".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.hairpin_log.staff, vec![1]);
    assert_eq!(parsed.hairpin_log.tstamp, Some(DataBeat(1.0)));
    assert_eq!(
        parsed.hairpin_log.tstamp2,
        Some(DataMeasurebeat("0m+3".to_string()))
    );
}

#[test]
fn roundtrip_hairpin_complete() {
    use tusk_model::att::AttHairpinLogForm;
    use tusk_model::data::{DataBeat, DataBoolean, DataMeasurebeat, DataUri};
    use tusk_model::elements::Hairpin;

    let mut original = Hairpin::default();
    original.common.xml_id = Some("hp-complete".to_string());
    original.hairpin_log.form = Some(AttHairpinLogForm::Cres);
    original.hairpin_log.niente = Some(DataBoolean::False);
    original.hairpin_log.staff = vec![1];
    original.hairpin_log.layer = vec![1];
    original.hairpin_log.tstamp = Some(DataBeat(1.0));
    original.hairpin_log.tstamp2 = Some(DataMeasurebeat("0m+4".to_string()));
    original.hairpin_log.startid = Some(DataUri("#n1".to_string()));
    original.hairpin_log.endid = Some(DataUri("#n4".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Hairpin::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.hairpin_log.form, original.hairpin_log.form);
    assert_eq!(parsed.hairpin_log.niente, original.hairpin_log.niente);
    assert_eq!(parsed.hairpin_log.staff, original.hairpin_log.staff);
    assert_eq!(parsed.hairpin_log.layer, original.hairpin_log.layer);
    assert_eq!(parsed.hairpin_log.tstamp, original.hairpin_log.tstamp);
    assert_eq!(parsed.hairpin_log.tstamp2, original.hairpin_log.tstamp2);
    assert_eq!(parsed.hairpin_log.startid, original.hairpin_log.startid);
    assert_eq!(parsed.hairpin_log.endid, original.hairpin_log.endid);
}

// ============================================================================
// Dir Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_dir_empty() {
    use tusk_model::elements::Dir;

    let original = Dir::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_dir_with_xml_id() {
    use tusk_model::elements::Dir;

    let mut original = Dir::default();
    original.common.xml_id = Some("dir-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("dir-1".to_string()));
}

#[test]
fn roundtrip_dir_with_text() {
    use tusk_model::elements::{Dir, DirChild};

    let mut original = Dir::default();
    original.children.push(DirChild::Text("legato".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        DirChild::Text(t) => assert_eq!(t, "legato"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn roundtrip_dir_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Dir;

    let mut original = Dir::default();
    original.dir_log.staff = vec![1];
    original.dir_log.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dir_log.staff, vec![1]);
    assert_eq!(parsed.dir_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn roundtrip_dir_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Dir;

    let mut original = Dir::default();
    original.dir_log.startid = Some(DataUri("#note1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dir_log.startid, Some(DataUri("#note1".to_string())));
}

#[test]
fn roundtrip_dir_with_endid() {
    use tusk_model::data::{DataBeat, DataUri};
    use tusk_model::elements::{Dir, DirChild};

    let mut original = Dir::default();
    original.dir_log.tstamp = Some(DataBeat(1.0));
    original.dir_log.endid = Some(DataUri("#n4".to_string()));
    original
        .children
        .push(DirChild::Text("sempre legato".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.dir_log.endid, Some(DataUri("#n4".to_string())));
}

#[test]
fn roundtrip_dir_complete() {
    use tusk_model::data::{DataBeat, DataMeasurebeat, DataUri};
    use tusk_model::elements::{Dir, DirChild};

    let mut original = Dir::default();
    original.common.xml_id = Some("dir-complete".to_string());
    original.dir_log.staff = vec![1];
    original.dir_log.layer = vec![1];
    original.dir_log.tstamp = Some(DataBeat(1.0));
    original.dir_log.tstamp2 = Some(DataMeasurebeat("2m+1".to_string()));
    original.dir_log.startid = Some(DataUri("#n1".to_string()));
    original.dir_log.endid = Some(DataUri("#n8".to_string()));
    original
        .children
        .push(DirChild::Text("dolce espressivo".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Dir::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.dir_log.staff, original.dir_log.staff);
    assert_eq!(parsed.dir_log.layer, original.dir_log.layer);
    assert_eq!(parsed.dir_log.tstamp, original.dir_log.tstamp);
    assert_eq!(parsed.dir_log.tstamp2, original.dir_log.tstamp2);
    assert_eq!(parsed.dir_log.startid, original.dir_log.startid);
    assert_eq!(parsed.dir_log.endid, original.dir_log.endid);
    assert_eq!(parsed.children.len(), 1);
}

// ============================================================================
// Tempo Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_tempo_empty() {
    use tusk_model::elements::Tempo;

    let original = Tempo::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_tempo_with_xml_id() {
    use tusk_model::elements::Tempo;

    let mut original = Tempo::default();
    original.common.xml_id = Some("tempo-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("tempo-1".to_string()));
}

#[test]
fn roundtrip_tempo_with_text() {
    use tusk_model::elements::{Tempo, TempoChild};

    let mut original = Tempo::default();
    original
        .children
        .push(TempoChild::Text("Allegro".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        TempoChild::Text(t) => assert_eq!(t, "Allegro"),
        _ => panic!("Expected text child"),
    }
}

#[test]
fn roundtrip_tempo_with_mm() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataTempovalue};
    use tusk_model::elements::Tempo;

    let mut original = Tempo::default();
    original.tempo_log.mm = Some(DataTempovalue(120.0));
    original.tempo_log.mm_unit = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tempo_log.mm, Some(DataTempovalue(120.0)));
    assert_eq!(
        parsed.tempo_log.mm_unit,
        Some(DataDuration::DataDurationCmn(DataDurationCmn::N4))
    );
}

#[test]
fn roundtrip_tempo_with_func() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::elements::Tempo;

    let mut original = Tempo::default();
    original.tempo_log.func = Some(AttTempoLogFunc::Instantaneous);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tempo_log.func, Some(AttTempoLogFunc::Instantaneous));
}

#[test]
fn roundtrip_tempo_continuous_func() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::data::{DataBeat, DataMeasurebeat};
    use tusk_model::elements::{Tempo, TempoChild};

    let mut original = Tempo::default();
    original.tempo_log.func = Some(AttTempoLogFunc::Continuous);
    original.tempo_log.tstamp = Some(DataBeat(1.0));
    original.tempo_log.tstamp2 = Some(DataMeasurebeat("0m+4".to_string()));
    original.children.push(TempoChild::Text("rit.".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tempo_log.func, Some(AttTempoLogFunc::Continuous));
    assert_eq!(
        parsed.tempo_log.tstamp2,
        Some(DataMeasurebeat("0m+4".to_string()))
    );
}

#[test]
fn roundtrip_tempo_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Tempo;

    let mut original = Tempo::default();
    original.tempo_log.staff = vec![1];
    original.tempo_log.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.tempo_log.staff, vec![1]);
    assert_eq!(parsed.tempo_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn roundtrip_tempo_complete() {
    use tusk_model::att::AttTempoLogFunc;
    use tusk_model::data::{DataBeat, DataDuration, DataDurationCmn, DataTempovalue, DataUri};
    use tusk_model::elements::{Tempo, TempoChild};

    let mut original = Tempo::default();
    original.common.xml_id = Some("tempo-complete".to_string());
    original.tempo_log.staff = vec![1];
    original.tempo_log.layer = vec![1];
    original.tempo_log.tstamp = Some(DataBeat(1.0));
    original.tempo_log.mm = Some(DataTempovalue(120.0));
    original.tempo_log.mm_unit = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.tempo_log.func = Some(AttTempoLogFunc::Instantaneous);
    original.tempo_log.startid = Some(DataUri("#n1".to_string()));
    original
        .children
        .push(TempoChild::Text("Allegro moderato".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Tempo::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.tempo_log.staff, original.tempo_log.staff);
    assert_eq!(parsed.tempo_log.layer, original.tempo_log.layer);
    assert_eq!(parsed.tempo_log.tstamp, original.tempo_log.tstamp);
    assert_eq!(parsed.tempo_log.mm, original.tempo_log.mm);
    assert_eq!(parsed.tempo_log.mm_unit, original.tempo_log.mm_unit);
    assert_eq!(parsed.tempo_log.func, original.tempo_log.func);
    assert_eq!(parsed.tempo_log.startid, original.tempo_log.startid);
    assert_eq!(parsed.children.len(), 1);
}

// ============================================================================
// Fermata Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_fermata_empty() {
    use tusk_model::elements::Fermata;

    let original = Fermata::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.fermata_vis.form.is_none());
    assert!(parsed.fermata_vis.shape.is_none());
}

#[test]
fn roundtrip_fermata_with_xml_id() {
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.common.xml_id = Some("ferm-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ferm-1".to_string()));
}

#[test]
fn roundtrip_fermata_with_form_norm() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.form = Some(AttFermataVisForm::Norm);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Norm));
}

#[test]
fn roundtrip_fermata_with_form_inv() {
    use tusk_model::att::AttFermataVisForm;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.form = Some(AttFermataVisForm::Inv);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.form, Some(AttFermataVisForm::Inv));
}

#[test]
fn roundtrip_fermata_with_shape_curved() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.shape = Some(AttFermataVisShape::Curved);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Curved));
}

#[test]
fn roundtrip_fermata_with_shape_square() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.shape = Some(AttFermataVisShape::Square);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Square));
}

#[test]
fn roundtrip_fermata_with_shape_angular() {
    use tusk_model::att::AttFermataVisShape;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_vis.shape = Some(AttFermataVisShape::Angular);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_vis.shape, Some(AttFermataVisShape::Angular));
}

#[test]
fn roundtrip_fermata_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_log.staff = vec![1];
    original.fermata_log.tstamp = Some(DataBeat(4.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.fermata_log.staff, vec![1]);
    assert_eq!(parsed.fermata_log.tstamp, Some(DataBeat(4.0)));
}

#[test]
fn roundtrip_fermata_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.fermata_log.startid = Some(DataUri("#note1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(
        parsed.fermata_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn roundtrip_fermata_complete() {
    use tusk_model::att::{AttFermataVisForm, AttFermataVisShape};
    use tusk_model::data::{DataBeat, DataUri};
    use tusk_model::elements::Fermata;

    let mut original = Fermata::default();
    original.common.xml_id = Some("ferm-complete".to_string());
    original.fermata_log.staff = vec![1];
    original.fermata_log.layer = vec![1];
    original.fermata_log.tstamp = Some(DataBeat(4.0));
    original.fermata_log.startid = Some(DataUri("#n4".to_string()));
    original.fermata_vis.form = Some(AttFermataVisForm::Norm);
    original.fermata_vis.shape = Some(AttFermataVisShape::Curved);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fermata::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.fermata_log.staff, original.fermata_log.staff);
    assert_eq!(parsed.fermata_log.layer, original.fermata_log.layer);
    assert_eq!(parsed.fermata_log.tstamp, original.fermata_log.tstamp);
    assert_eq!(parsed.fermata_log.startid, original.fermata_log.startid);
    assert_eq!(parsed.fermata_vis.form, original.fermata_vis.form);
    assert_eq!(parsed.fermata_vis.shape, original.fermata_vis.shape);
}

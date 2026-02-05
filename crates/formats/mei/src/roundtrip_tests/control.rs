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

// ============================================================================
// Turn Parse Tests
// ============================================================================

#[test]
fn turn_parse_empty() {
    use tusk_model::elements::Turn;

    let xml = r#"<turn/>"#;
    let parsed = Turn::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.turn_log.form.is_none());
}

#[test]
fn turn_parse_with_id() {
    use tusk_model::elements::Turn;

    let xml = r#"<turn xml:id="turn-1"/>"#;
    let parsed = Turn::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("turn-1".to_string()));
}

#[test]
fn turn_parse_with_form() {
    use tusk_model::att::AttTurnLogForm;
    use tusk_model::elements::Turn;

    let xml = r#"<turn form="upper"/>"#;
    let parsed = Turn::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.turn_log.form, Some(AttTurnLogForm::Upper));
}

#[test]
fn turn_parse_with_delayed() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Turn;

    let xml = r#"<turn delayed="true"/>"#;
    let parsed = Turn::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.turn_log.delayed, Some(DataBoolean::True));
}

#[test]
fn turn_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Turn;

    let xml = r#"<turn staff="1" tstamp="2.5"/>"#;
    let parsed = Turn::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.turn_log.staff, vec![1]);
    assert_eq!(parsed.turn_log.tstamp, Some(DataBeat(2.5)));
}

#[test]
fn turn_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Turn;

    let xml = r##"<turn startid="#note1"/>"##;
    let parsed = Turn::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.turn_log.startid, Some(DataUri("#note1".to_string())));
}

#[test]
fn turn_parse_with_accidentals() {
    use tusk_model::data::{DataAccidentalWritten, DataAccidentalWrittenBasic};
    use tusk_model::elements::Turn;

    let xml = r#"<turn accidupper="s" accidlower="f"/>"#;
    let parsed = Turn::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.turn_log.accidupper,
        Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::S
        ))
    );
    assert_eq!(
        parsed.turn_log.accidlower,
        Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
            DataAccidentalWrittenBasic::F
        ))
    );
}

// ============================================================================
// Turn Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_turn_empty() {
    use tusk_model::elements::Turn;

    let original = Turn::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Turn::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.turn_log.form.is_none());
}

#[test]
fn roundtrip_turn_with_xml_id() {
    use tusk_model::elements::Turn;

    let mut original = Turn::default();
    original.common.xml_id = Some("turn-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Turn::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("turn-1".to_string()));
}

#[test]
fn roundtrip_turn_with_form() {
    use tusk_model::att::AttTurnLogForm;
    use tusk_model::elements::Turn;

    let mut original = Turn::default();
    original.turn_log.form = Some(AttTurnLogForm::Lower);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Turn::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.turn_log.form, Some(AttTurnLogForm::Lower));
}

#[test]
fn roundtrip_turn_with_delayed() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::Turn;

    let mut original = Turn::default();
    original.turn_log.delayed = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Turn::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.turn_log.delayed, Some(DataBoolean::True));
}

#[test]
fn roundtrip_turn_complete() {
    use tusk_model::att::AttTurnLogForm;
    use tusk_model::data::{
        DataAccidentalWritten, DataAccidentalWrittenBasic, DataBeat, DataBoolean, DataUri,
    };
    use tusk_model::elements::Turn;

    let mut original = Turn::default();
    original.common.xml_id = Some("turn-complete".to_string());
    original.turn_log.staff = vec![1];
    original.turn_log.layer = vec![1];
    original.turn_log.tstamp = Some(DataBeat(2.0));
    original.turn_log.startid = Some(DataUri("#n1".to_string()));
    original.turn_log.form = Some(AttTurnLogForm::Upper);
    original.turn_log.delayed = Some(DataBoolean::False);
    original.turn_log.accidupper = Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
        DataAccidentalWrittenBasic::S,
    ));
    original.turn_log.accidlower = Some(DataAccidentalWritten::DataAccidentalWrittenBasic(
        DataAccidentalWrittenBasic::F,
    ));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Turn::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.turn_log.staff, original.turn_log.staff);
    assert_eq!(parsed.turn_log.layer, original.turn_log.layer);
    assert_eq!(parsed.turn_log.tstamp, original.turn_log.tstamp);
    assert_eq!(parsed.turn_log.startid, original.turn_log.startid);
    assert_eq!(parsed.turn_log.form, original.turn_log.form);
    assert_eq!(parsed.turn_log.delayed, original.turn_log.delayed);
    assert_eq!(parsed.turn_log.accidupper, original.turn_log.accidupper);
    assert_eq!(parsed.turn_log.accidlower, original.turn_log.accidlower);
}

// ============================================================================
// Breath Parse Tests
// ============================================================================

#[test]
fn breath_parse_empty() {
    use tusk_model::elements::Breath;

    let xml = r#"<breath/>"#;
    let parsed = Breath::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.breath_log.startid.is_none());
}

#[test]
fn breath_parse_with_id() {
    use tusk_model::elements::Breath;

    let xml = r#"<breath xml:id="breath-1"/>"#;
    let parsed = Breath::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("breath-1".to_string()));
}

#[test]
fn breath_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Breath;

    let xml = r#"<breath staff="1" tstamp="4"/>"#;
    let parsed = Breath::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.breath_log.staff, vec![1]);
    assert_eq!(parsed.breath_log.tstamp, Some(DataBeat(4.0)));
}

#[test]
fn breath_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Breath;

    let xml = r##"<breath startid="#note1"/>"##;
    let parsed = Breath::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.breath_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn breath_parse_with_place() {
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic};
    use tusk_model::elements::Breath;

    let xml = r#"<breath place="above"/>"#;
    let parsed = Breath::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.breath_vis.place,
        Some(DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above))
    );
}

// ============================================================================
// Breath Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_breath_empty() {
    use tusk_model::elements::Breath;

    let original = Breath::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Breath::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.breath_log.startid.is_none());
}

#[test]
fn roundtrip_breath_with_xml_id() {
    use tusk_model::elements::Breath;

    let mut original = Breath::default();
    original.common.xml_id = Some("breath-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Breath::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("breath-1".to_string()));
}

#[test]
fn roundtrip_breath_complete() {
    use tusk_model::data::{DataBeat, DataStaffrel, DataStaffrelBasic, DataUri};
    use tusk_model::elements::Breath;

    let mut original = Breath::default();
    original.common.xml_id = Some("breath-complete".to_string());
    original.breath_log.staff = vec![1];
    original.breath_log.layer = vec![1];
    original.breath_log.tstamp = Some(DataBeat(4.0));
    original.breath_log.startid = Some(DataUri("#n4".to_string()));
    original.breath_vis.place = Some(DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Breath::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.breath_log.staff, original.breath_log.staff);
    assert_eq!(parsed.breath_log.layer, original.breath_log.layer);
    assert_eq!(parsed.breath_log.tstamp, original.breath_log.tstamp);
    assert_eq!(parsed.breath_log.startid, original.breath_log.startid);
    assert_eq!(parsed.breath_vis.place, original.breath_vis.place);
}

// ============================================================================
// Bend Parse Tests
// ============================================================================

#[test]
fn bend_parse_empty() {
    use tusk_model::elements::Bend;

    let xml = r#"<bend/>"#;
    let parsed = Bend::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.bend_log.startid.is_none());
}

#[test]
fn bend_parse_with_id() {
    use tusk_model::elements::Bend;

    let xml = r#"<bend xml:id="bend-1"/>"#;
    let parsed = Bend::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("bend-1".to_string()));
}

#[test]
fn bend_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Bend;

    let xml = r#"<bend staff="1" tstamp="1"/>"#;
    let parsed = Bend::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.bend_log.staff, vec![1]);
    assert_eq!(parsed.bend_log.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn bend_parse_with_startid_endid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Bend;

    let xml = r##"<bend startid="#note1" endid="#note2"/>"##;
    let parsed = Bend::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.bend_log.startid, Some(DataUri("#note1".to_string())));
    assert_eq!(parsed.bend_log.endid, Some(DataUri("#note2".to_string())));
}

#[test]
fn bend_parse_with_amount() {
    use tusk_model::data::DataBendAmount;
    use tusk_model::elements::Bend;

    let xml = r#"<bend amount="1"/>"#;
    let parsed = Bend::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.bend_ges.amount,
        Some(DataBendAmount("1".to_string()))
    );
}

#[test]
fn bend_parse_with_curvedir() {
    use tusk_model::att::AttBendVisCurvedir;
    use tusk_model::elements::Bend;

    let xml = r#"<bend curvedir="above"/>"#;
    let parsed = Bend::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.bend_vis.curvedir, Some(AttBendVisCurvedir::Above));
}

// ============================================================================
// Bend Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_bend_empty() {
    use tusk_model::elements::Bend;

    let original = Bend::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Bend::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.bend_log.startid.is_none());
}

#[test]
fn roundtrip_bend_with_xml_id() {
    use tusk_model::elements::Bend;

    let mut original = Bend::default();
    original.common.xml_id = Some("bend-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Bend::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("bend-1".to_string()));
}

#[test]
fn roundtrip_bend_complete() {
    use tusk_model::att::AttBendVisCurvedir;
    use tusk_model::data::{DataBeat, DataBendAmount, DataMeasurebeat, DataUri};
    use tusk_model::elements::Bend;

    let mut original = Bend::default();
    original.common.xml_id = Some("bend-complete".to_string());
    original.bend_log.staff = vec![1];
    original.bend_log.layer = vec![1];
    original.bend_log.tstamp = Some(DataBeat(1.0));
    original.bend_log.tstamp2 = Some(DataMeasurebeat("0m+2".to_string()));
    original.bend_log.startid = Some(DataUri("#n1".to_string()));
    original.bend_log.endid = Some(DataUri("#n2".to_string()));
    original.bend_ges.amount = Some(DataBendAmount("0.5".to_string()));
    original.bend_vis.curvedir = Some(AttBendVisCurvedir::Above);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Bend::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.bend_log.staff, original.bend_log.staff);
    assert_eq!(parsed.bend_log.layer, original.bend_log.layer);
    assert_eq!(parsed.bend_log.tstamp, original.bend_log.tstamp);
    assert_eq!(parsed.bend_log.tstamp2, original.bend_log.tstamp2);
    assert_eq!(parsed.bend_log.startid, original.bend_log.startid);
    assert_eq!(parsed.bend_log.endid, original.bend_log.endid);
    assert_eq!(parsed.bend_ges.amount, original.bend_ges.amount);
    assert_eq!(parsed.bend_vis.curvedir, original.bend_vis.curvedir);
}

// ============================================================================
// Caesura Parse Tests
// ============================================================================

#[test]
fn caesura_parse_empty() {
    use tusk_model::elements::Caesura;

    let xml = r#"<caesura/>"#;
    let parsed = Caesura::from_mei_str(xml).expect("parse");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.caesura_log.startid.is_none());
}

#[test]
fn caesura_parse_with_id() {
    use tusk_model::elements::Caesura;

    let xml = r#"<caesura xml:id="caesura-1"/>"#;
    let parsed = Caesura::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.common.xml_id, Some("caesura-1".to_string()));
}

#[test]
fn caesura_parse_with_staff_tstamp() {
    use tusk_model::data::DataBeat;
    use tusk_model::elements::Caesura;

    let xml = r#"<caesura staff="1" tstamp="4"/>"#;
    let parsed = Caesura::from_mei_str(xml).expect("parse");

    assert_eq!(parsed.caesura_log.staff, vec![1]);
    assert_eq!(parsed.caesura_log.tstamp, Some(DataBeat(4.0)));
}

#[test]
fn caesura_parse_with_startid() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Caesura;

    let xml = r##"<caesura startid="#note1"/>"##;
    let parsed = Caesura::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.caesura_log.startid,
        Some(DataUri("#note1".to_string()))
    );
}

#[test]
fn caesura_parse_with_place() {
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic};
    use tusk_model::elements::Caesura;

    let xml = r#"<caesura place="above"/>"#;
    let parsed = Caesura::from_mei_str(xml).expect("parse");

    assert_eq!(
        parsed.caesura_vis.place,
        Some(DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above))
    );
}

// ============================================================================
// Caesura Round-Trip Tests
// ============================================================================

#[test]
fn roundtrip_caesura_empty() {
    use tusk_model::elements::Caesura;

    let original = Caesura::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Caesura::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.caesura_log.startid.is_none());
}

#[test]
fn roundtrip_caesura_with_xml_id() {
    use tusk_model::elements::Caesura;

    let mut original = Caesura::default();
    original.common.xml_id = Some("caesura-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Caesura::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("caesura-1".to_string()));
}

#[test]
fn roundtrip_caesura_complete() {
    use tusk_model::data::{DataBeat, DataStaffrel, DataStaffrelBasic, DataUri};
    use tusk_model::elements::Caesura;

    let mut original = Caesura::default();
    original.common.xml_id = Some("caesura-complete".to_string());
    original.caesura_log.staff = vec![1];
    original.caesura_log.layer = vec![1];
    original.caesura_log.tstamp = Some(DataBeat(4.0));
    original.caesura_log.startid = Some(DataUri("#n4".to_string()));
    original.caesura_vis.place = Some(DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Caesura::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.caesura_log.staff, original.caesura_log.staff);
    assert_eq!(parsed.caesura_log.layer, original.caesura_log.layer);
    assert_eq!(parsed.caesura_log.tstamp, original.caesura_log.tstamp);
    assert_eq!(parsed.caesura_log.startid, original.caesura_log.startid);
    assert_eq!(parsed.caesura_vis.place, original.caesura_vis.place);
}

// ============================================================================
// RepeatMark Tests
// ============================================================================

#[test]
fn roundtrip_repeat_mark_empty() {
    use tusk_model::elements::RepeatMark;

    let original = RepeatMark::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = RepeatMark::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_repeat_mark_with_func() {
    use tusk_model::att::AttRepeatMarkLogFunc;
    use tusk_model::elements::RepeatMark;

    let mut original = RepeatMark::default();
    original.common.xml_id = Some("rm1".to_string());
    original.repeat_mark_log.func = Some(AttRepeatMarkLogFunc::DalSegno);
    original.repeat_mark_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = RepeatMark::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.repeat_mark_log.func, original.repeat_mark_log.func);
    assert_eq!(parsed.repeat_mark_log.staff, original.repeat_mark_log.staff);
}

#[test]
fn roundtrip_repeat_mark_with_text_child() {
    use tusk_model::elements::{RepeatMark, RepeatMarkChild};

    let mut original = RepeatMark::default();
    original.common.xml_id = Some("rm2".to_string());
    original
        .children
        .push(RepeatMarkChild::Text("D.C.".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = RepeatMark::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        RepeatMarkChild::Text(text) => assert_eq!(text, "D.C."),
        _ => panic!("Expected Text child"),
    }
}

// ============================================================================
// Volta Tests
// ============================================================================

#[test]
fn roundtrip_volta_empty() {
    use tusk_model::elements::Volta;

    let original = Volta::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Volta::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_volta_with_n() {
    use tusk_model::data::DataWord;
    use tusk_model::elements::Volta;

    let mut original = Volta::default();
    original.common.xml_id = Some("v1".to_string());
    original.common.n = Some(DataWord("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Volta::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.common.n, original.common.n);
}

// ============================================================================
// MRpt Tests
// ============================================================================

#[test]
fn roundtrip_m_rpt_empty() {
    use tusk_model::elements::MRpt;

    let original = MRpt::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MRpt::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_m_rpt_with_num() {
    use tusk_model::elements::MRpt;

    let mut original = MRpt::default();
    original.common.xml_id = Some("mr1".to_string());
    original.m_rpt_log.num = Some(2);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MRpt::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.m_rpt_log.num, original.m_rpt_log.num);
}

// ============================================================================
// MRpt2 Tests
// ============================================================================

#[test]
fn roundtrip_m_rpt2_empty() {
    use tusk_model::elements::MRpt2;

    let original = MRpt2::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MRpt2::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_m_rpt2_with_staff() {
    use tusk_model::elements::MRpt2;

    let mut original = MRpt2::default();
    original.common.xml_id = Some("mr2_1".to_string());
    original.m_rpt2_log.staff = vec![1, 2];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MRpt2::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.m_rpt2_log.staff, original.m_rpt2_log.staff);
}

// ============================================================================
// BeatRpt Tests
// ============================================================================

#[test]
fn roundtrip_beat_rpt_empty() {
    use tusk_model::elements::BeatRpt;

    let original = BeatRpt::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = BeatRpt::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_beat_rpt_with_beatdef() {
    use tusk_model::elements::BeatRpt;

    let mut original = BeatRpt::default();
    original.common.xml_id = Some("br1".to_string());
    original.beat_rpt_log.beatdef = Some(1.0);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = BeatRpt::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.beat_rpt_log.beatdef, original.beat_rpt_log.beatdef);
}

#[test]
fn roundtrip_beat_rpt_with_slash() {
    use tusk_model::data::DataBeatrptRend;
    use tusk_model::elements::BeatRpt;

    let mut original = BeatRpt::default();
    original.common.xml_id = Some("br2".to_string());
    original.beat_rpt_vis.slash = Some(DataBeatrptRend("2".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = BeatRpt::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.beat_rpt_vis.slash, original.beat_rpt_vis.slash);
}

// ============================================================================
// HalfmRpt Tests
// ============================================================================

#[test]
fn roundtrip_halfm_rpt_empty() {
    use tusk_model::elements::HalfmRpt;

    let original = HalfmRpt::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = HalfmRpt::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_halfm_rpt_with_staff_layer() {
    use tusk_model::elements::HalfmRpt;

    let mut original = HalfmRpt::default();
    original.common.xml_id = Some("hmr1".to_string());
    original.halfm_rpt_log.staff = vec![1];
    original.halfm_rpt_log.layer = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = HalfmRpt::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.halfm_rpt_log.staff, original.halfm_rpt_log.staff);
    assert_eq!(parsed.halfm_rpt_log.layer, original.halfm_rpt_log.layer);
}

// ============================================================================
// MultiRpt Tests
// ============================================================================

#[test]
fn roundtrip_multi_rpt_empty() {
    use tusk_model::elements::MultiRpt;

    let original = MultiRpt::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MultiRpt::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_multi_rpt_with_num() {
    use tusk_model::elements::MultiRpt;

    let mut original = MultiRpt::default();
    original.common.xml_id = Some("mulrpt1".to_string());
    original.multi_rpt_log.num = Some(4);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MultiRpt::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.multi_rpt_log.num, original.multi_rpt_log.num);
}

// ============================================================================
// MultiRest Tests
// ============================================================================

#[test]
fn roundtrip_multi_rest_empty() {
    use tusk_model::elements::MultiRest;

    let original = MultiRest::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MultiRest::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_multi_rest_with_num() {
    use tusk_model::elements::MultiRest;

    let mut original = MultiRest::default();
    original.common.xml_id = Some("mulrest1".to_string());
    original.multi_rest_log.num = Some(8);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MultiRest::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.multi_rest_log.num, original.multi_rest_log.num);
}

#[test]
fn roundtrip_multi_rest_with_block() {
    use tusk_model::data::DataBoolean;
    use tusk_model::elements::MultiRest;

    let mut original = MultiRest::default();
    original.common.xml_id = Some("mulrest2".to_string());
    original.multi_rest_log.num = Some(4);
    original.multi_rest_vis.block = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MultiRest::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.multi_rest_log.num, original.multi_rest_log.num);
    assert_eq!(parsed.multi_rest_vis.block, original.multi_rest_vis.block);
}

// ============================================================================
// MSpace Tests
// ============================================================================

#[test]
fn roundtrip_m_space_empty() {
    use tusk_model::elements::MSpace;

    let original = MSpace::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MSpace::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_m_space_with_staff_layer() {
    use tusk_model::elements::MSpace;

    let mut original = MSpace::default();
    original.common.xml_id = Some("ms1".to_string());
    original.m_space_log.staff = vec![1];
    original.m_space_log.layer = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MSpace::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.m_space_log.staff, original.m_space_log.staff);
    assert_eq!(parsed.m_space_log.layer, original.m_space_log.layer);
}

#[test]
fn roundtrip_m_space_with_fermata() {
    use tusk_model::data::DataStaffrelBasic;
    use tusk_model::elements::MSpace;

    let mut original = MSpace::default();
    original.common.xml_id = Some("ms2".to_string());
    original.m_space_anl.fermata = Some(DataStaffrelBasic::Above);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MSpace::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.m_space_anl.fermata, original.m_space_anl.fermata);
}

// ============================================================================
// MNum Tests
// ============================================================================

#[test]
fn roundtrip_m_num_empty() {
    use tusk_model::elements::MNum;

    let original = MNum::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MNum::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_m_num_with_text() {
    use tusk_model::elements::{MNum, MNumChild};

    let mut original = MNum::default();
    original.common.xml_id = Some("mn1".to_string());
    original.children.push(MNumChild::Text("42".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MNum::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MNumChild::Text(text) => assert_eq!(text, "42"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn roundtrip_m_num_with_place() {
    use tusk_model::data::{DataStaffrel, DataStaffrelBasic};
    use tusk_model::elements::{MNum, MNumChild};

    let mut original = MNum::default();
    original.common.xml_id = Some("mn2".to_string());
    original.m_num_vis.place = Some(DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above));
    original.children.push(MNumChild::Text("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MNum::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.m_num_vis.place, original.m_num_vis.place);
}

// ============================================================================
// BeamSpan Tests
// ============================================================================

#[test]
fn roundtrip_beam_span_empty() {
    use tusk_model::elements::BeamSpan;

    let original = BeamSpan::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = BeamSpan::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_beam_span_with_refs() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::BeamSpan;

    let mut original = BeamSpan::default();
    original.common.xml_id = Some("bs1".to_string());
    original.beam_span_log.startid = Some(DataUri("#n1".to_string()));
    original.beam_span_log.endid = Some(DataUri("#n4".to_string()));
    original.beam_span_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = BeamSpan::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.beam_span_log.startid, original.beam_span_log.startid);
    assert_eq!(parsed.beam_span_log.endid, original.beam_span_log.endid);
    assert_eq!(parsed.beam_span_log.staff, original.beam_span_log.staff);
}

#[test]
fn roundtrip_beam_span_with_plist() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::BeamSpan;

    let mut original = BeamSpan::default();
    original.common.xml_id = Some("bs2".to_string());
    original.beam_span_log.plist = vec![
        DataUri("#n1".to_string()),
        DataUri("#n2".to_string()),
        DataUri("#n3".to_string()),
    ];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = BeamSpan::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.beam_span_log.plist, original.beam_span_log.plist);
}

// ============================================================================
// Octave Tests
// ============================================================================

#[test]
fn roundtrip_octave_empty() {
    use tusk_model::elements::Octave;

    let original = Octave::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Octave::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_octave_with_dis() {
    use tusk_model::data::{DataOctaveDis, DataStaffrelBasic, DataUri};
    use tusk_model::elements::Octave;

    let mut original = Octave::default();
    original.common.xml_id = Some("oct1".to_string());
    original.octave_log.dis = Some(DataOctaveDis(8));
    original.octave_log.dis_place = Some(DataStaffrelBasic::Above);
    original.octave_log.startid = Some(DataUri("#n1".to_string()));
    original.octave_log.endid = Some(DataUri("#n8".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Octave::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.octave_log.dis, original.octave_log.dis);
    assert_eq!(parsed.octave_log.dis_place, original.octave_log.dis_place);
    assert_eq!(parsed.octave_log.startid, original.octave_log.startid);
    assert_eq!(parsed.octave_log.endid, original.octave_log.endid);
}

#[test]
fn roundtrip_octave_with_tstamp() {
    use tusk_model::data::{DataBeat, DataMeasurebeat, DataOctaveDis, DataStaffrelBasic};
    use tusk_model::elements::Octave;

    let mut original = Octave::default();
    original.common.xml_id = Some("oct2".to_string());
    original.octave_log.dis = Some(DataOctaveDis(15));
    original.octave_log.dis_place = Some(DataStaffrelBasic::Below);
    original.octave_log.tstamp = Some(DataBeat(1.0));
    original.octave_log.tstamp2 = Some(DataMeasurebeat("2m+1".to_string()));
    original.octave_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Octave::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.octave_log.dis, original.octave_log.dis);
    assert_eq!(parsed.octave_log.dis_place, original.octave_log.dis_place);
    assert_eq!(parsed.octave_log.tstamp, original.octave_log.tstamp);
    assert_eq!(parsed.octave_log.tstamp2, original.octave_log.tstamp2);
    assert_eq!(parsed.octave_log.staff, original.octave_log.staff);
}

// ============================================================================
// Gliss Tests
// ============================================================================

#[test]
fn roundtrip_gliss_empty() {
    use tusk_model::elements::Gliss;

    let original = Gliss::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Gliss::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_gliss_with_refs() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Gliss;

    let mut original = Gliss::default();
    original.common.xml_id = Some("gliss1".to_string());
    original.gliss_log.startid = Some(DataUri("#n1".to_string()));
    original.gliss_log.endid = Some(DataUri("#n2".to_string()));
    original.gliss_log.staff = vec![1];
    original.gliss_log.layer = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Gliss::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.gliss_log.startid, original.gliss_log.startid);
    assert_eq!(parsed.gliss_log.endid, original.gliss_log.endid);
    assert_eq!(parsed.gliss_log.staff, original.gliss_log.staff);
    assert_eq!(parsed.gliss_log.layer, original.gliss_log.layer);
}

#[test]
fn roundtrip_gliss_with_line_style() {
    use tusk_model::data::{DataLineform, DataUri};
    use tusk_model::elements::Gliss;

    let mut original = Gliss::default();
    original.common.xml_id = Some("gliss2".to_string());
    original.gliss_log.startid = Some(DataUri("#n1".to_string()));
    original.gliss_log.endid = Some(DataUri("#n2".to_string()));
    original.gliss_vis.lform = Some(DataLineform::Wavy);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Gliss::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.gliss_vis.lform, original.gliss_vis.lform);
}

// ============================================================================
// Lv (laissez vibrer) Tests
// ============================================================================

#[test]
fn roundtrip_lv_empty() {
    use tusk_model::elements::Lv;

    let original = Lv::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Lv::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_lv_with_refs() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Lv;

    let mut original = Lv::default();
    original.common.xml_id = Some("lv1".to_string());
    original.lv_log.startid = Some(DataUri("#n1".to_string()));
    original.lv_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Lv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.lv_log.startid, original.lv_log.startid);
    assert_eq!(parsed.lv_log.staff, original.lv_log.staff);
}

#[test]
fn roundtrip_lv_with_curvedir() {
    use tusk_model::att::AttLvVisCurvedir;
    use tusk_model::data::DataUri;
    use tusk_model::elements::Lv;

    let mut original = Lv::default();
    original.common.xml_id = Some("lv2".to_string());
    original.lv_log.startid = Some(DataUri("#n1".to_string()));
    original.lv_vis.curvedir = Some(AttLvVisCurvedir::Above);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Lv::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.lv_vis.curvedir, original.lv_vis.curvedir);
}

// ============================================================================
// BracketSpan Tests
// ============================================================================

#[test]
fn roundtrip_bracket_span_empty() {
    use tusk_model::elements::BracketSpan;

    let original = BracketSpan::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = BracketSpan::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn roundtrip_bracket_span_with_refs() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::BracketSpan;

    let mut original = BracketSpan::default();
    original.common.xml_id = Some("brspan1".to_string());
    original.bracket_span_log.startid = Some(DataUri("#n1".to_string()));
    original.bracket_span_log.endid = Some(DataUri("#n4".to_string()));
    original.bracket_span_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = BracketSpan::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(
        parsed.bracket_span_log.startid,
        original.bracket_span_log.startid
    );
    assert_eq!(
        parsed.bracket_span_log.endid,
        original.bracket_span_log.endid
    );
    assert_eq!(
        parsed.bracket_span_log.staff,
        original.bracket_span_log.staff
    );
}

#[test]
fn roundtrip_bracket_span_with_line_style() {
    use tusk_model::data::{DataLineform, DataUri};
    use tusk_model::elements::BracketSpan;

    let mut original = BracketSpan::default();
    original.common.xml_id = Some("brspan2".to_string());
    original.bracket_span_log.startid = Some(DataUri("#n1".to_string()));
    original.bracket_span_log.endid = Some(DataUri("#n4".to_string()));
    original.bracket_span_vis.lform = Some(DataLineform::Solid);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = BracketSpan::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(
        parsed.bracket_span_vis.lform,
        original.bracket_span_vis.lform
    );
}

// ============================================================================
// BTrem (bowed tremolo) Tests
// ============================================================================

#[test]
fn roundtrip_b_trem_empty() {
    use tusk_model::elements::BTrem;

    let original = BTrem::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = BTrem::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_b_trem_with_note() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{BTrem, BTremChild, Note};

    let mut note = Note::default();
    note.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note.note_log.oct = Some(DataOctave(4));
    note.note_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N8));

    let mut original = BTrem::default();
    original.common.xml_id = Some("btrem1".to_string());
    original.b_trem_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.children.push(BTremChild::Note(Box::new(note)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = BTrem::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.b_trem_log.dur, original.b_trem_log.dur);
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        BTremChild::Note(n) => {
            assert_eq!(n.note_log.pname, Some(DataPitchname::from("c".to_string())));
            assert_eq!(n.note_log.oct, Some(DataOctave(4)));
        }
        _ => panic!("Expected Note child"),
    }
}

#[test]
fn roundtrip_b_trem_with_chord() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{BTrem, BTremChild, Chord, ChordChild, Note};

    let mut note1 = Note::default();
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));

    let mut note2 = Note::default();
    note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note2.note_log.oct = Some(DataOctave(4));

    let mut chord = Chord::default();
    chord.chord_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    chord.children.push(ChordChild::Note(Box::new(note1)));
    chord.children.push(ChordChild::Note(Box::new(note2)));

    let mut original = BTrem::default();
    original.common.xml_id = Some("btrem2".to_string());
    original.children.push(BTremChild::Chord(Box::new(chord)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = BTrem::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        BTremChild::Chord(c) => {
            assert_eq!(c.children.len(), 2);
        }
        _ => panic!("Expected Chord child"),
    }
}

// ============================================================================
// FTrem (fingered tremolo) Tests
// ============================================================================

#[test]
fn roundtrip_f_trem_empty() {
    use tusk_model::elements::FTrem;

    let original = FTrem::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = FTrem::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn roundtrip_f_trem_with_notes() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{FTrem, FTremChild, Note};

    let mut note1 = Note::default();
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));

    let mut note2 = Note::default();
    note2.note_log.pname = Some(DataPitchname::from("g".to_string()));
    note2.note_log.oct = Some(DataOctave(4));

    let mut original = FTrem::default();
    original.common.xml_id = Some("ftrem1".to_string());
    original.f_trem_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N2));
    original.children.push(FTremChild::Note(Box::new(note1)));
    original.children.push(FTremChild::Note(Box::new(note2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FTrem::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.f_trem_log.dur, original.f_trem_log.dur);
    assert_eq!(parsed.children.len(), 2);
}

#[test]
fn roundtrip_f_trem_with_beams() {
    use tusk_model::data::{DataDuration, DataDurationCmn, DataOctave, DataPitchname};
    use tusk_model::elements::{FTrem, FTremChild, Note};

    let mut note1 = Note::default();
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));

    let mut note2 = Note::default();
    note2.note_log.pname = Some(DataPitchname::from("e".to_string()));
    note2.note_log.oct = Some(DataOctave(4));

    let mut original = FTrem::default();
    original.common.xml_id = Some("ftrem2".to_string());
    original.f_trem_log.dur = Some(DataDuration::DataDurationCmn(DataDurationCmn::N4));
    original.f_trem_vis.beams = Some(3);
    original.children.push(FTremChild::Note(Box::new(note1)));
    original.children.push(FTremChild::Note(Box::new(note2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FTrem::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, original.common.xml_id);
    assert_eq!(parsed.f_trem_vis.beams, original.f_trem_vis.beams);
}

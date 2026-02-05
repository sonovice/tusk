//! Round-trip serialization tests for mensural notation MEI elements.
//!
//! Tests for Mensur, Mensuration, Proport, and Ligature elements used
//! in early music (mensural) notation.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Mensur Tests
// ============================================================================

#[test]
fn mensur_roundtrip_empty() {
    use tusk_model::elements::Mensur;

    let original = Mensur::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mensur::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn mensur_roundtrip_with_xml_id() {
    use tusk_model::elements::Mensur;

    let mut original = Mensur::default();
    original.common.xml_id = Some("m1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mensur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
}

#[test]
fn mensur_roundtrip_with_ratio() {
    use tusk_model::elements::Mensur;

    let mut original = Mensur::default();
    original.common.xml_id = Some("m1".to_string());
    original.mensur_log.num = Some(3);
    original.mensur_log.numbase = Some(2);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mensur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
    assert_eq!(parsed.mensur_log.num, Some(3));
    assert_eq!(parsed.mensur_log.numbase, Some(2));
}

#[test]
fn mensur_roundtrip_with_mensuration_attributes() {
    use tusk_model::data::{DataProlatio, DataTempus};
    use tusk_model::elements::Mensur;

    let mut original = Mensur::default();
    original.common.xml_id = Some("m1".to_string());
    original.mensur_log.tempus = Some(DataTempus(3));
    original.mensur_log.prolatio = Some(DataProlatio(2));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mensur::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("m1".to_string()));
    assert_eq!(parsed.mensur_log.tempus, Some(DataTempus(3)));
    assert_eq!(parsed.mensur_log.prolatio, Some(DataProlatio(2)));
}

// ============================================================================
// Mensuration Tests
// ============================================================================

#[test]
fn mensuration_roundtrip_empty() {
    use tusk_model::elements::Mensuration;

    let original = Mensuration::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mensuration::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn mensuration_roundtrip_with_xml_id() {
    use tusk_model::elements::Mensuration;

    let mut original = Mensuration::default();
    original.common.xml_id = Some("mns1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mensuration::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mns1".to_string()));
}

#[test]
fn mensuration_roundtrip_with_mensur_attributes() {
    use tusk_model::data::DataTempus;
    use tusk_model::elements::Mensuration;

    let mut original = Mensuration::default();
    original.common.xml_id = Some("mns1".to_string());
    original.mensur_log.tempus = Some(DataTempus(3));
    original.mensur_log.num = Some(3);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Mensuration::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mns1".to_string()));
    assert_eq!(parsed.mensur_log.tempus, Some(DataTempus(3)));
    assert_eq!(parsed.mensur_log.num, Some(3));
}

// ============================================================================
// Proport Tests
// ============================================================================

#[test]
fn proport_roundtrip_empty() {
    use tusk_model::elements::Proport;

    let original = Proport::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Proport::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn proport_roundtrip_with_xml_id() {
    use tusk_model::elements::Proport;

    let mut original = Proport::default();
    original.common.xml_id = Some("p1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Proport::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("p1".to_string()));
}

#[test]
fn proport_roundtrip_with_ratio() {
    use tusk_model::elements::Proport;

    let mut original = Proport::default();
    original.common.xml_id = Some("p1".to_string());
    original.proport_log.num = Some(3);
    original.proport_log.numbase = Some(2);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Proport::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("p1".to_string()));
    assert_eq!(parsed.proport_log.num, Some(3));
    assert_eq!(parsed.proport_log.numbase, Some(2));
}

// ============================================================================
// Ligature Tests
// ============================================================================

#[test]
fn ligature_roundtrip_empty() {
    use tusk_model::elements::Ligature;

    let original = Ligature::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ligature::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn ligature_roundtrip_with_xml_id() {
    use tusk_model::elements::Ligature;

    let mut original = Ligature::default();
    original.common.xml_id = Some("lig1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ligature::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("lig1".to_string()));
}

#[test]
fn ligature_roundtrip_with_form_attribute() {
    use tusk_model::data::DataLigatureform;
    use tusk_model::elements::Ligature;

    let mut original = Ligature::default();
    original.common.xml_id = Some("lig1".to_string());
    original.ligature_vis.form = Some(DataLigatureform::Recta);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ligature::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("lig1".to_string()));
    assert_eq!(parsed.ligature_vis.form, Some(DataLigatureform::Recta));
}

#[test]
fn ligature_roundtrip_with_note_children() {
    use tusk_model::data::{DataOctave, DataPitchname};
    use tusk_model::elements::{Ligature, LigatureChild, Note};

    let mut original = Ligature::default();
    original.common.xml_id = Some("lig1".to_string());

    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    note1.note_log.pname = Some(DataPitchname::from("c".to_string()));
    note1.note_log.oct = Some(DataOctave(4));
    original.children.push(LigatureChild::Note(Box::new(note1)));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    note2.note_log.pname = Some(DataPitchname::from("d".to_string()));
    note2.note_log.oct = Some(DataOctave(4));
    original.children.push(LigatureChild::Note(Box::new(note2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ligature::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("lig1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        LigatureChild::Note(n) => {
            assert_eq!(n.common.xml_id, Some("n1".to_string()));
            assert_eq!(n.note_log.pname, Some(DataPitchname::from("c".to_string())));
        }
        _ => panic!("Expected Note child"),
    }

    match &parsed.children[1] {
        LigatureChild::Note(n) => {
            assert_eq!(n.common.xml_id, Some("n2".to_string()));
            assert_eq!(n.note_log.pname, Some(DataPitchname::from("d".to_string())));
        }
        _ => panic!("Expected Note child"),
    }
}

#[test]
fn ligature_roundtrip_with_nested_ligature() {
    use tusk_model::elements::{Ligature, LigatureChild, Note};

    let mut original = Ligature::default();
    original.common.xml_id = Some("lig1".to_string());

    let mut inner_lig = Ligature::default();
    inner_lig.common.xml_id = Some("lig2".to_string());

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    inner_lig.children.push(LigatureChild::Note(Box::new(note)));

    original
        .children
        .push(LigatureChild::Ligature(Box::new(inner_lig)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ligature::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("lig1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        LigatureChild::Ligature(inner) => {
            assert_eq!(inner.common.xml_id, Some("lig2".to_string()));
            assert_eq!(inner.children.len(), 1);
        }
        _ => panic!("Expected Ligature child"),
    }
}

#[test]
fn ligature_roundtrip_with_mensur_child() {
    use tusk_model::elements::{Ligature, LigatureChild, Mensur, Note};

    let mut original = Ligature::default();
    original.common.xml_id = Some("lig1".to_string());

    let mut mensur = Mensur::default();
    mensur.common.xml_id = Some("m1".to_string());
    mensur.mensur_log.num = Some(3);
    original
        .children
        .push(LigatureChild::Mensur(Box::new(mensur)));

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    original.children.push(LigatureChild::Note(Box::new(note)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ligature::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("lig1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        LigatureChild::Mensur(m) => {
            assert_eq!(m.common.xml_id, Some("m1".to_string()));
            assert_eq!(m.mensur_log.num, Some(3));
        }
        _ => panic!("Expected Mensur child"),
    }
}

#[test]
fn ligature_roundtrip_with_proport_child() {
    use tusk_model::elements::{Ligature, LigatureChild, Note, Proport};

    let mut original = Ligature::default();
    original.common.xml_id = Some("lig1".to_string());

    let mut proport = Proport::default();
    proport.common.xml_id = Some("p1".to_string());
    proport.proport_log.num = Some(2);
    proport.proport_log.numbase = Some(1);
    original
        .children
        .push(LigatureChild::Proport(Box::new(proport)));

    let mut note = Note::default();
    note.common.xml_id = Some("n1".to_string());
    original.children.push(LigatureChild::Note(Box::new(note)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ligature::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("lig1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        LigatureChild::Proport(p) => {
            assert_eq!(p.common.xml_id, Some("p1".to_string()));
            assert_eq!(p.proport_log.num, Some(2));
            assert_eq!(p.proport_log.numbase, Some(1));
        }
        _ => panic!("Expected Proport child"),
    }
}

#[test]
fn ligature_roundtrip_with_mixed_children() {
    use tusk_model::elements::{Dot, Ligature, LigatureChild, Mensur, Note, Rest};

    let mut original = Ligature::default();
    original.common.xml_id = Some("lig1".to_string());

    let mut note1 = Note::default();
    note1.common.xml_id = Some("n1".to_string());
    original.children.push(LigatureChild::Note(Box::new(note1)));

    let mut dot = Dot::default();
    dot.common.xml_id = Some("d1".to_string());
    original.children.push(LigatureChild::Dot(Box::new(dot)));

    let mut note2 = Note::default();
    note2.common.xml_id = Some("n2".to_string());
    original.children.push(LigatureChild::Note(Box::new(note2)));

    let mut rest = Rest::default();
    rest.common.xml_id = Some("r1".to_string());
    original.children.push(LigatureChild::Rest(Box::new(rest)));

    let mut mensur = Mensur::default();
    mensur.common.xml_id = Some("m1".to_string());
    original
        .children
        .push(LigatureChild::Mensur(Box::new(mensur)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ligature::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 5);
    assert!(matches!(parsed.children[0], LigatureChild::Note(_)));
    assert!(matches!(parsed.children[1], LigatureChild::Dot(_)));
    assert!(matches!(parsed.children[2], LigatureChild::Note(_)));
    assert!(matches!(parsed.children[3], LigatureChild::Rest(_)));
    assert!(matches!(parsed.children[4], LigatureChild::Mensur(_)));
}

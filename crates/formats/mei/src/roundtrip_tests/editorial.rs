//! Round-trip serialization tests for editorial MEI elements.
//!
//! Tests for App, Lem, Rdg, Choice, Corr, Sic, Add, Del elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// App (Apparatus) Tests
// ============================================================================

#[test]
fn app_roundtrip_empty() {
    use tusk_model::elements::App;

    let original = App::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = App::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn app_roundtrip_with_xml_id() {
    use tusk_model::elements::App;

    let mut original = App::default();
    original.common.xml_id = Some("app-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = App::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("app-1".to_string()));
}

#[test]
fn app_roundtrip_with_lem() {
    use tusk_model::elements::{App, AppChild, Lem};

    let mut original = App::default();
    original.common.xml_id = Some("app-1".to_string());

    let mut lem = Lem::default();
    lem.common.xml_id = Some("lem-1".to_string());
    original.children.push(AppChild::Lem(Box::new(lem)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = App::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        AppChild::Lem(l) => assert_eq!(l.common.xml_id, Some("lem-1".to_string())),
        _ => panic!("Expected Lem child"),
    }
}

#[test]
fn app_roundtrip_with_rdg() {
    use tusk_model::elements::{App, AppChild, Rdg};

    let mut original = App::default();
    original.common.xml_id = Some("app-1".to_string());

    let mut rdg = Rdg::default();
    rdg.common.xml_id = Some("rdg-1".to_string());
    original.children.push(AppChild::Rdg(Box::new(rdg)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = App::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        AppChild::Rdg(r) => assert_eq!(r.common.xml_id, Some("rdg-1".to_string())),
        _ => panic!("Expected Rdg child"),
    }
}

#[test]
fn app_roundtrip_with_lem_and_rdg() {
    use tusk_model::elements::{App, AppChild, Lem, Rdg};

    let mut original = App::default();
    original.common.xml_id = Some("app-1".to_string());

    let mut lem = Lem::default();
    lem.common.xml_id = Some("lem-1".to_string());
    original.children.push(AppChild::Lem(Box::new(lem)));

    let mut rdg1 = Rdg::default();
    rdg1.common.xml_id = Some("rdg-1".to_string());
    original.children.push(AppChild::Rdg(Box::new(rdg1)));

    let mut rdg2 = Rdg::default();
    rdg2.common.xml_id = Some("rdg-2".to_string());
    original.children.push(AppChild::Rdg(Box::new(rdg2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = App::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 3);
}

// ============================================================================
// Lem (Lemma) Tests
// ============================================================================

#[test]
fn lem_roundtrip_empty() {
    use tusk_model::elements::Lem;

    let original = Lem::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Lem::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn lem_roundtrip_with_xml_id() {
    use tusk_model::elements::Lem;

    let mut original = Lem::default();
    original.common.xml_id = Some("lem-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Lem::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("lem-1".to_string()));
}

#[test]
fn lem_roundtrip_with_source() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Lem;

    let mut original = Lem::default();
    original.common.xml_id = Some("lem-1".to_string());
    original.crit.source = vec![DataUri("#source1".to_string())];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Lem::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.crit.source.len(), 1);
    assert_eq!(parsed.crit.source[0], DataUri("#source1".to_string()));
}

#[test]
fn lem_serialize_with_text() {
    use tusk_model::elements::{Lem, LemChild};

    let mut original = Lem::default();
    original
        .children
        .push(LemChild::Text("lemma text".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    // Verify the text is in the serialized output
    assert!(xml.contains("lemma text"));
    assert!(xml.contains("<lem>"));
    assert!(xml.contains("</lem>"));
}

// ============================================================================
// Rdg (Reading) Tests
// ============================================================================

#[test]
fn rdg_roundtrip_empty() {
    use tusk_model::elements::Rdg;

    let original = Rdg::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rdg::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn rdg_roundtrip_with_xml_id() {
    use tusk_model::elements::Rdg;

    let mut original = Rdg::default();
    original.common.xml_id = Some("rdg-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rdg::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("rdg-1".to_string()));
}

#[test]
fn rdg_roundtrip_with_source() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Rdg;

    let mut original = Rdg::default();
    original.common.xml_id = Some("rdg-1".to_string());
    original.crit.source = vec![DataUri("#ms-A".to_string()), DataUri("#ms-B".to_string())];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Rdg::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.crit.source.len(), 2);
}

#[test]
fn rdg_serialize_with_text() {
    use tusk_model::elements::{Rdg, RdgChild};

    let mut original = Rdg::default();
    original
        .children
        .push(RdgChild::Text("variant reading".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    // Verify the text is in the serialized output
    assert!(xml.contains("variant reading"));
    assert!(xml.contains("<rdg>"));
    assert!(xml.contains("</rdg>"));
}

// ============================================================================
// Choice Tests
// ============================================================================

#[test]
fn choice_roundtrip_empty() {
    use tusk_model::elements::Choice;

    let original = Choice::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Choice::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn choice_roundtrip_with_xml_id() {
    use tusk_model::elements::Choice;

    let mut original = Choice::default();
    original.common.xml_id = Some("choice-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Choice::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("choice-1".to_string()));
}

#[test]
fn choice_roundtrip_with_sic_corr() {
    use tusk_model::elements::{Choice, ChoiceChild, Corr, Sic};

    let mut original = Choice::default();
    original.common.xml_id = Some("choice-1".to_string());

    let mut sic = Sic::default();
    sic.common.xml_id = Some("sic-1".to_string());
    original.children.push(ChoiceChild::Sic(Box::new(sic)));

    let mut corr = Corr::default();
    corr.common.xml_id = Some("corr-1".to_string());
    original.children.push(ChoiceChild::Corr(Box::new(corr)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Choice::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);
}

// ============================================================================
// Corr (Correction) Tests
// ============================================================================

#[test]
fn corr_roundtrip_empty() {
    use tusk_model::elements::Corr;

    let original = Corr::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Corr::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn corr_roundtrip_with_xml_id() {
    use tusk_model::elements::Corr;

    let mut original = Corr::default();
    original.common.xml_id = Some("corr-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Corr::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("corr-1".to_string()));
}

#[test]
fn corr_serialize_with_text() {
    use tusk_model::elements::{Corr, CorrChild};

    let mut original = Corr::default();
    original
        .children
        .push(CorrChild::Text("corrected text".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    // Verify the text is in the serialized output
    assert!(xml.contains("corrected text"));
    assert!(xml.contains("<corr>"));
    assert!(xml.contains("</corr>"));
}

// ============================================================================
// Sic Tests
// ============================================================================

#[test]
fn sic_roundtrip_empty() {
    use tusk_model::elements::Sic;

    let original = Sic::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Sic::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn sic_roundtrip_with_xml_id() {
    use tusk_model::elements::Sic;

    let mut original = Sic::default();
    original.common.xml_id = Some("sic-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Sic::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("sic-1".to_string()));
}

#[test]
fn sic_serialize_with_text() {
    use tusk_model::elements::{Sic, SicChild};

    let mut original = Sic::default();
    original
        .children
        .push(SicChild::Text("erroneous text".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    // Verify the text is in the serialized output
    assert!(xml.contains("erroneous text"));
    assert!(xml.contains("<sic>"));
    assert!(xml.contains("</sic>"));
}

// ============================================================================
// Add (Addition) Tests
// ============================================================================

#[test]
fn add_roundtrip_empty() {
    use tusk_model::elements::Add;

    let original = Add::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Add::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn add_roundtrip_with_xml_id() {
    use tusk_model::elements::Add;

    let mut original = Add::default();
    original.common.xml_id = Some("add-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Add::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("add-1".to_string()));
}

#[test]
fn add_serialize_with_place() {
    use tusk_model::data::{DataPlacement, DataStaffrel, DataStaffrelBasic};
    use tusk_model::elements::Add;

    let mut original = Add::default();
    original.common.xml_id = Some("add-1".to_string());
    original.place = vec![DataPlacement::DataStaffrel(
        DataStaffrel::DataStaffrelBasic(DataStaffrelBasic::Above),
    )];

    let xml = original.to_mei_string().expect("serialize");
    // Verify the place attribute is in the serialized output
    assert!(xml.contains(r#"place="above""#));
}

#[test]
fn add_serialize_with_text() {
    use tusk_model::elements::{Add, AddChild};

    let mut original = Add::default();
    original
        .children
        .push(AddChild::Text("added text".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    // Verify the text is in the serialized output
    assert!(xml.contains("added text"));
    assert!(xml.contains("<add>"));
    assert!(xml.contains("</add>"));
}

// ============================================================================
// Del (Deletion) Tests
// ============================================================================

#[test]
fn del_roundtrip_empty() {
    use tusk_model::elements::Del;

    let original = Del::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Del::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn del_roundtrip_with_xml_id() {
    use tusk_model::elements::Del;

    let mut original = Del::default();
    original.common.xml_id = Some("del-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Del::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("del-1".to_string()));
}

#[test]
fn del_serialize_with_text() {
    use tusk_model::elements::{Del, DelChild};

    let mut original = Del::default();
    original
        .children
        .push(DelChild::Text("deleted text".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    // Verify the text is in the serialized output
    assert!(xml.contains("deleted text"));
    assert!(xml.contains("<del>"));
    assert!(xml.contains("</del>"));
}

// ============================================================================
// Fig (Figure) Tests
// ============================================================================

#[test]
fn fig_roundtrip_empty() {
    use tusk_model::elements::Fig;

    let original = Fig::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fig::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn fig_roundtrip_with_xml_id() {
    use tusk_model::elements::Fig;

    let mut original = Fig::default();
    original.common.xml_id = Some("fig-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fig::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("fig-1".to_string()));
}

#[test]
fn fig_roundtrip_with_fig_desc() {
    use tusk_model::elements::{Fig, FigChild, FigDesc, FigDescChild};

    let mut original = Fig::default();
    original.common.xml_id = Some("fig-1".to_string());

    let mut fig_desc = FigDesc::default();
    fig_desc.common.xml_id = Some("figDesc-1".to_string());
    fig_desc
        .children
        .push(FigDescChild::Text("A musical figure".to_string()));
    original
        .children
        .push(FigChild::FigDesc(Box::new(fig_desc)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Fig::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        FigChild::FigDesc(fd) => {
            assert_eq!(fd.common.xml_id, Some("figDesc-1".to_string()));
            assert!(!fd.children.is_empty());
        }
        _ => panic!("Expected FigDesc child"),
    }
}

#[test]
fn fig_serialize_with_halign() {
    use tusk_model::data::DataHorizontalalignment;
    use tusk_model::elements::Fig;

    let mut original = Fig::default();
    original.horizontal_align.halign = Some(DataHorizontalalignment::Center);

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains(r#"halign="center""#));
}

#[test]
fn fig_serialize_with_valign() {
    use tusk_model::data::DataVerticalalignment;
    use tusk_model::elements::Fig;

    let mut original = Fig::default();
    original.vertical_align.valign = Some(DataVerticalalignment::Middle);

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains(r#"valign="middle""#));
}

// ============================================================================
// FigDesc (Figure Description) Tests
// ============================================================================

#[test]
fn fig_desc_roundtrip_empty() {
    use tusk_model::elements::FigDesc;

    let original = FigDesc::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = FigDesc::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn fig_desc_roundtrip_with_xml_id() {
    use tusk_model::elements::FigDesc;

    let mut original = FigDesc::default();
    original.common.xml_id = Some("figDesc-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FigDesc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("figDesc-1".to_string()));
}

#[test]
fn fig_desc_roundtrip_with_text() {
    use tusk_model::elements::{FigDesc, FigDescChild};

    let mut original = FigDesc::default();
    original.common.xml_id = Some("figDesc-1".to_string());
    original.children.push(FigDescChild::Text(
        "Description of a musical example.".to_string(),
    ));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FigDesc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        FigDescChild::Text(text) => {
            assert_eq!(text, "Description of a musical example.");
        }
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn fig_desc_serialize_with_lang() {
    use tusk_model::elements::FigDesc;

    let mut original = FigDesc::default();
    original.lang.xml_lang = Some("en".to_string());

    let xml = original.to_mei_string().expect("serialize");
    assert!(xml.contains(r#"xml:lang="en""#));
}

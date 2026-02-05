//! Round-trip serialization tests for name-related MEI elements.
//!
//! Tests for ForeName, FamName, AddName, GenName, NameLink, PeriodName, StyleName elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// ForeName Tests
// ============================================================================

#[test]
fn fore_name_roundtrip_empty() {
    use tusk_model::elements::ForeName;

    let original = ForeName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = ForeName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn fore_name_roundtrip_with_text() {
    use tusk_model::elements::{ForeName, ForeNameChild};

    let mut original = ForeName::default();
    original.common.xml_id = Some("foreName-1".to_string());
    original
        .children
        .push(ForeNameChild::Text("Johann".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ForeName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("foreName-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ForeNameChild::Text(t) => assert_eq!(t, "Johann"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn fore_name_roundtrip_with_attributes() {
    use tusk_model::elements::ForeName;

    let mut original = ForeName::default();
    original.common.xml_id = Some("foreName-1".to_string());
    original.name.auth = Some("viaf".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ForeName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("foreName-1".to_string()));
    assert_eq!(parsed.name.auth, Some("viaf".to_string()));
}

// ============================================================================
// FamName Tests
// ============================================================================

#[test]
fn fam_name_roundtrip_empty() {
    use tusk_model::elements::FamName;

    let original = FamName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = FamName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn fam_name_roundtrip_with_text() {
    use tusk_model::elements::{FamName, FamNameChild};

    let mut original = FamName::default();
    original.common.xml_id = Some("famName-1".to_string());
    original
        .children
        .push(FamNameChild::Text("Bach".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FamName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("famName-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        FamNameChild::Text(t) => assert_eq!(t, "Bach"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn fam_name_roundtrip_with_lang() {
    use tusk_model::elements::FamName;

    let mut original = FamName::default();
    original.common.xml_id = Some("famName-1".to_string());
    original.lang.xml_lang = Some("de".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = FamName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("famName-1".to_string()));
    assert_eq!(parsed.lang.xml_lang, Some("de".to_string()));
}

// ============================================================================
// AddName Tests
// ============================================================================

#[test]
fn add_name_roundtrip_empty() {
    use tusk_model::elements::AddName;

    let original = AddName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = AddName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn add_name_roundtrip_with_text() {
    use tusk_model::elements::{AddName, AddNameChild};

    let mut original = AddName::default();
    original.common.xml_id = Some("addName-1".to_string());
    original
        .children
        .push(AddNameChild::Text("the Great".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = AddName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("addName-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        AddNameChild::Text(t) => assert_eq!(t, "the Great"),
        _ => panic!("Expected Text child"),
    }
}

// ============================================================================
// GenName Tests
// ============================================================================

#[test]
fn gen_name_roundtrip_empty() {
    use tusk_model::elements::GenName;

    let original = GenName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = GenName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn gen_name_roundtrip_with_text() {
    use tusk_model::elements::{GenName, GenNameChild};

    let mut original = GenName::default();
    original.common.xml_id = Some("genName-1".to_string());
    original
        .children
        .push(GenNameChild::Text("Jr.".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GenName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("genName-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GenNameChild::Text(t) => assert_eq!(t, "Jr."),
        _ => panic!("Expected Text child"),
    }
}

// ============================================================================
// NameLink Tests
// ============================================================================

#[test]
fn name_link_roundtrip_empty() {
    use tusk_model::elements::NameLink;

    let original = NameLink::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = NameLink::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn name_link_roundtrip_with_text() {
    use tusk_model::elements::{NameLink, NameLinkChild};

    let mut original = NameLink::default();
    original.common.xml_id = Some("nameLink-1".to_string());
    original
        .children
        .push(NameLinkChild::Text("van der".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = NameLink::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("nameLink-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        NameLinkChild::Text(t) => assert_eq!(t, "van der"),
        _ => panic!("Expected Text child"),
    }
}

// ============================================================================
// PeriodName Tests
// ============================================================================

#[test]
fn period_name_roundtrip_empty() {
    use tusk_model::elements::PeriodName;

    let original = PeriodName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = PeriodName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn period_name_roundtrip_with_text() {
    use tusk_model::elements::{PeriodName, PeriodNameChild};

    let mut original = PeriodName::default();
    original.common.xml_id = Some("periodName-1".to_string());
    original
        .children
        .push(PeriodNameChild::Text("Baroque".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = PeriodName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("periodName-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        PeriodNameChild::Text(t) => assert_eq!(t, "Baroque"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn period_name_roundtrip_with_name_attributes() {
    use tusk_model::elements::PeriodName;

    let mut original = PeriodName::default();
    original.common.xml_id = Some("periodName-1".to_string());
    original.name.auth = Some("lcsh".to_string());
    original.name.codedval = vec!["sh85088762".to_string()];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = PeriodName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("periodName-1".to_string()));
    assert_eq!(parsed.name.auth, Some("lcsh".to_string()));
    assert_eq!(parsed.name.codedval, vec!["sh85088762".to_string()]);
}

// ============================================================================
// StyleName Tests
// ============================================================================

#[test]
fn style_name_roundtrip_empty() {
    use tusk_model::elements::StyleName;

    let original = StyleName::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = StyleName::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn style_name_roundtrip_with_text() {
    use tusk_model::elements::{StyleName, StyleNameChild};

    let mut original = StyleName::default();
    original.common.xml_id = Some("styleName-1".to_string());
    original
        .children
        .push(StyleNameChild::Text("bebop".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = StyleName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("styleName-1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        StyleNameChild::Text(t) => assert_eq!(t, "bebop"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn style_name_roundtrip_with_attributes() {
    use tusk_model::elements::StyleName;

    let mut original = StyleName::default();
    original.common.xml_id = Some("styleName-1".to_string());
    original.bibl.analog = Some("dcterms:subject".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = StyleName::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("styleName-1".to_string()));
    assert_eq!(parsed.bibl.analog, Some("dcterms:subject".to_string()));
}

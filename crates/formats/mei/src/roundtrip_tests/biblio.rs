//! Round-trip serialization tests for bibliographic and codicological MEI elements.
//!
//! Tests for ExtData, AvFile, Cutout, Bifolium, Folium, Patch, Analytic, Monogr, Series.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// ExtData Tests
// ============================================================================

#[test]
fn ext_data_roundtrip_empty() {
    use tusk_model::elements::ExtData;

    let original = ExtData::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = ExtData::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn ext_data_roundtrip_with_text() {
    use tusk_model::elements::{ExtData, ExtDataChild};

    let mut original = ExtData::default();
    original.basic.xml_id = Some("ed1".to_string());
    original.internet_media.mimetype = Some("text/plain".to_string());
    original
        .children
        .push(ExtDataChild::Text("External data content".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ExtData::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("ed1".to_string()));
    assert_eq!(parsed.internet_media.mimetype, Some("text/plain".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ExtDataChild::Text(t) => assert_eq!(t, "External data content"),
    }
}

// ============================================================================
// AvFile Tests
// ============================================================================

#[test]
fn av_file_roundtrip_empty() {
    use tusk_model::elements::AvFile;

    let original = AvFile::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = AvFile::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn av_file_roundtrip_with_attrs() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::AvFile;

    let mut original = AvFile::default();
    original.common.xml_id = Some("av1".to_string());
    original.internet_media.mimetype = Some("audio/mpeg".to_string());
    original.pointing.target = vec![DataUri("audio.mp3".to_string())];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = AvFile::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("av1".to_string()));
    assert_eq!(
        parsed.internet_media.mimetype,
        Some("audio/mpeg".to_string())
    );
    assert_eq!(parsed.pointing.target.len(), 1);
    assert_eq!(parsed.pointing.target[0].0, "audio.mp3");
}

// ============================================================================
// Patch Tests
// ============================================================================

#[test]
fn patch_roundtrip_empty() {
    use tusk_model::elements::Patch;

    let original = Patch::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Patch::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn patch_roundtrip_with_attrs() {
    use tusk_model::elements::Patch;

    let mut original = Patch::default();
    original.common.xml_id = Some("patch1".to_string());
    original.attached_to = Some("top-left".to_string());
    original.attached_by = Some("glue".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Patch::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("patch1".to_string()));
    assert_eq!(parsed.attached_to, Some("top-left".to_string()));
    assert_eq!(parsed.attached_by, Some("glue".to_string()));
}

// ============================================================================
// Cutout Tests
// ============================================================================

#[test]
fn cutout_roundtrip_empty() {
    use tusk_model::elements::Cutout;

    let original = Cutout::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cutout::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn cutout_roundtrip_with_attrs() {
    use tusk_model::elements::Cutout;

    let mut original = Cutout::default();
    original.common.xml_id = Some("c1".to_string());
    original.removed_from = Some("top".to_string());
    original.removed_by = Some("scissors".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cutout::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("c1".to_string()));
    assert_eq!(parsed.removed_from, Some("top".to_string()));
    assert_eq!(parsed.removed_by, Some("scissors".to_string()));
}

// ============================================================================
// Folium Tests
// ============================================================================

#[test]
fn folium_roundtrip_empty() {
    use tusk_model::elements::Folium;

    let original = Folium::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Folium::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn folium_roundtrip_with_xml_id() {
    use tusk_model::elements::Folium;

    let mut original = Folium::default();
    original.common.xml_id = Some("fol1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Folium::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("fol1".to_string()));
}

#[test]
fn folium_roundtrip_with_patch() {
    use tusk_model::elements::{Folium, FoliumChild, Patch};

    let mut original = Folium::default();
    original.common.xml_id = Some("fol1".to_string());

    let mut patch = Patch::default();
    patch.common.xml_id = Some("patch1".to_string());
    original.children.push(FoliumChild::Patch(Box::new(patch)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Folium::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        FoliumChild::Patch(p) => {
            assert_eq!(p.common.xml_id, Some("patch1".to_string()));
        }
        _ => panic!("Expected Patch child"),
    }
}

// ============================================================================
// Bifolium Tests
// ============================================================================

#[test]
fn bifolium_roundtrip_empty() {
    use tusk_model::elements::Bifolium;

    let original = Bifolium::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Bifolium::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn bifolium_roundtrip_with_xml_id() {
    use tusk_model::elements::Bifolium;

    let mut original = Bifolium::default();
    original.common.xml_id = Some("bif1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Bifolium::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("bif1".to_string()));
}

#[test]
fn bifolium_roundtrip_with_folium() {
    use tusk_model::elements::{Bifolium, BifoliumChild, Folium};

    let mut original = Bifolium::default();
    original.common.xml_id = Some("bif1".to_string());

    let mut folium = Folium::default();
    folium.common.xml_id = Some("fol1".to_string());
    original
        .children
        .push(BifoliumChild::Folium(Box::new(folium)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Bifolium::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        BifoliumChild::Folium(f) => {
            assert_eq!(f.common.xml_id, Some("fol1".to_string()));
        }
        _ => panic!("Expected Folium child"),
    }
}

#[test]
fn bifolium_roundtrip_with_nested_bifolium() {
    use tusk_model::elements::{Bifolium, BifoliumChild};

    let mut original = Bifolium::default();
    original.common.xml_id = Some("bif1".to_string());

    let mut nested = Bifolium::default();
    nested.common.xml_id = Some("bif2".to_string());
    original
        .children
        .push(BifoliumChild::Bifolium(Box::new(nested)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Bifolium::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        BifoliumChild::Bifolium(b) => {
            assert_eq!(b.common.xml_id, Some("bif2".to_string()));
        }
        _ => panic!("Expected Bifolium child"),
    }
}

// ============================================================================
// Analytic Tests
// ============================================================================

#[test]
fn analytic_roundtrip_empty() {
    use tusk_model::elements::Analytic;

    let original = Analytic::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Analytic::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn analytic_roundtrip_with_xml_id() {
    use tusk_model::elements::Analytic;

    let mut original = Analytic::default();
    original.common.xml_id = Some("ana1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Analytic::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ana1".to_string()));
}

#[test]
fn analytic_roundtrip_with_title() {
    use tusk_model::elements::{Analytic, AnalyticChild, Title, TitleChild};

    let mut original = Analytic::default();
    original.common.xml_id = Some("ana1".to_string());

    let mut title = Title::default();
    title.basic.xml_id = Some("title1".to_string());
    title
        .children
        .push(TitleChild::Text("Article Title".to_string()));
    original.children.push(AnalyticChild::Title(Box::new(title)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Analytic::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        AnalyticChild::Title(t) => {
            assert_eq!(t.basic.xml_id, Some("title1".to_string()));
        }
        _ => panic!("Expected Title child"),
    }
}

// ============================================================================
// Monogr Tests
// ============================================================================

#[test]
fn monogr_roundtrip_empty() {
    use tusk_model::elements::Monogr;

    let original = Monogr::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Monogr::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn monogr_roundtrip_with_xml_id() {
    use tusk_model::elements::Monogr;

    let mut original = Monogr::default();
    original.common.xml_id = Some("mon1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Monogr::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mon1".to_string()));
}

#[test]
fn monogr_roundtrip_with_title() {
    use tusk_model::elements::{Monogr, MonogrChild, Title, TitleChild};

    let mut original = Monogr::default();
    original.common.xml_id = Some("mon1".to_string());

    let mut title = Title::default();
    title.basic.xml_id = Some("title1".to_string());
    title
        .children
        .push(TitleChild::Text("Book Title".to_string()));
    original.children.push(MonogrChild::Title(Box::new(title)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Monogr::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MonogrChild::Title(t) => {
            assert_eq!(t.basic.xml_id, Some("title1".to_string()));
        }
        _ => panic!("Expected Title child"),
    }
}

// ============================================================================
// Series Tests
// ============================================================================

#[test]
fn series_roundtrip_empty() {
    use tusk_model::elements::Series;

    let original = Series::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Series::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn series_roundtrip_with_xml_id() {
    use tusk_model::elements::Series;

    let mut original = Series::default();
    original.common.xml_id = Some("ser1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Series::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ser1".to_string()));
}

#[test]
fn series_roundtrip_with_text() {
    use tusk_model::elements::{Series, SeriesChild};

    let mut original = Series::default();
    original.common.xml_id = Some("ser1".to_string());
    original
        .children
        .push(SeriesChild::Text("Series Name".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Series::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ser1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SeriesChild::Text(t) => assert_eq!(t, "Series Name"),
        _ => panic!("Expected Text child"),
    }
}

#[test]
fn series_roundtrip_with_title() {
    use tusk_model::elements::{Series, SeriesChild, Title, TitleChild};

    let mut original = Series::default();
    original.common.xml_id = Some("ser1".to_string());

    let mut title = Title::default();
    title.basic.xml_id = Some("title1".to_string());
    title
        .children
        .push(TitleChild::Text("Series Title".to_string()));
    original.children.push(SeriesChild::Title(Box::new(title)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Series::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SeriesChild::Title(t) => {
            assert_eq!(t.basic.xml_id, Some("title1".to_string()));
        }
        _ => panic!("Expected Title child"),
    }
}

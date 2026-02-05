//! Round-trip serialization tests for neume notation MEI elements.
//!
//! Tests for Syllable, Neume, Nc, NcGrp, and neume component elements
//! (Oriscus, Quilisma, Liquescent, Strophicus, Plica, Episema, HispanTick, AmbNote).

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Oriscus Tests
// ============================================================================

#[test]
fn oriscus_roundtrip_empty() {
    use tusk_model::elements::Oriscus;

    let original = Oriscus::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Oriscus::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn oriscus_roundtrip_with_xml_id() {
    use tusk_model::elements::Oriscus;

    let mut original = Oriscus::default();
    original.common.xml_id = Some("or1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Oriscus::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("or1".to_string()));
}

// ============================================================================
// Quilisma Tests
// ============================================================================

#[test]
fn quilisma_roundtrip_empty() {
    use tusk_model::elements::Quilisma;

    let original = Quilisma::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Quilisma::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn quilisma_roundtrip_with_xml_id() {
    use tusk_model::elements::Quilisma;

    let mut original = Quilisma::default();
    original.common.xml_id = Some("q1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Quilisma::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("q1".to_string()));
}

// ============================================================================
// Liquescent Tests
// ============================================================================

#[test]
fn liquescent_roundtrip_empty() {
    use tusk_model::elements::Liquescent;

    let original = Liquescent::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Liquescent::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn liquescent_roundtrip_with_xml_id() {
    use tusk_model::elements::Liquescent;

    let mut original = Liquescent::default();
    original.common.xml_id = Some("liq1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Liquescent::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("liq1".to_string()));
}

// ============================================================================
// Strophicus Tests
// ============================================================================

#[test]
fn strophicus_roundtrip_empty() {
    use tusk_model::elements::Strophicus;

    let original = Strophicus::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Strophicus::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn strophicus_roundtrip_with_xml_id() {
    use tusk_model::elements::Strophicus;

    let mut original = Strophicus::default();
    original.common.xml_id = Some("str1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Strophicus::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("str1".to_string()));
}

// ============================================================================
// Plica Tests
// ============================================================================

#[test]
fn plica_roundtrip_empty() {
    use tusk_model::elements::Plica;

    let original = Plica::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Plica::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn plica_roundtrip_with_xml_id() {
    use tusk_model::elements::Plica;

    let mut original = Plica::default();
    original.common.xml_id = Some("pl1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Plica::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("pl1".to_string()));
}

// ============================================================================
// Episema Tests
// ============================================================================

#[test]
fn episema_roundtrip_empty() {
    use tusk_model::elements::Episema;

    let original = Episema::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Episema::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn episema_roundtrip_with_xml_id() {
    use tusk_model::elements::Episema;

    let mut original = Episema::default();
    original.common.xml_id = Some("ep1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Episema::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ep1".to_string()));
}

#[test]
fn episema_roundtrip_with_staff() {
    use tusk_model::elements::Episema;

    let mut original = Episema::default();
    original.common.xml_id = Some("ep1".to_string());
    original.episema_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Episema::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ep1".to_string()));
    assert_eq!(parsed.episema_log.staff, vec![1]);
}

// ============================================================================
// HispanTick Tests
// ============================================================================

#[test]
fn hispan_tick_roundtrip_empty() {
    use tusk_model::elements::HispanTick;

    let original = HispanTick::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = HispanTick::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn hispan_tick_roundtrip_with_xml_id() {
    use tusk_model::elements::HispanTick;

    let mut original = HispanTick::default();
    original.common.xml_id = Some("ht1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = HispanTick::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ht1".to_string()));
}

#[test]
fn hispan_tick_roundtrip_with_staff() {
    use tusk_model::elements::HispanTick;

    let mut original = HispanTick::default();
    original.common.xml_id = Some("ht1".to_string());
    original.hispan_tick_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = HispanTick::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ht1".to_string()));
    assert_eq!(parsed.hispan_tick_log.staff, vec![1]);
}

// ============================================================================
// AmbNote Tests
// ============================================================================

#[test]
fn amb_note_roundtrip_empty() {
    use tusk_model::elements::AmbNote;

    let original = AmbNote::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = AmbNote::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn amb_note_roundtrip_with_xml_id() {
    use tusk_model::elements::AmbNote;

    let mut original = AmbNote::default();
    original.common.xml_id = Some("an1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = AmbNote::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("an1".to_string()));
}

// ============================================================================
// Nc Tests
// ============================================================================

#[test]
fn nc_roundtrip_empty() {
    use tusk_model::elements::Nc;

    let original = Nc::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Nc::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn nc_roundtrip_with_xml_id() {
    use tusk_model::elements::Nc;

    let mut original = Nc::default();
    original.basic.xml_id = Some("nc1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Nc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("nc1".to_string()));
}

#[test]
fn nc_roundtrip_with_pitch() {
    use tusk_model::elements::Nc;

    let mut original = Nc::default();
    original.basic.xml_id = Some("nc1".to_string());
    original.nc_log.pname = Some("c".to_string());
    original.nc_log.oct = Some("4".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Nc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("nc1".to_string()));
    assert_eq!(parsed.nc_log.pname, Some("c".to_string()));
    assert_eq!(parsed.nc_log.oct, Some("4".to_string()));
}

#[test]
fn nc_roundtrip_with_quilisma_child() {
    use tusk_model::elements::{Nc, NcChild, Quilisma};

    let mut original = Nc::default();
    original.basic.xml_id = Some("nc1".to_string());

    let mut quilisma = Quilisma::default();
    quilisma.common.xml_id = Some("q1".to_string());
    original
        .children
        .push(NcChild::Quilisma(Box::new(quilisma)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Nc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("nc1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        NcChild::Quilisma(q) => {
            assert_eq!(q.common.xml_id, Some("q1".to_string()));
        }
        _ => panic!("Expected Quilisma child"),
    }
}

#[test]
fn nc_roundtrip_with_multiple_children() {
    use tusk_model::elements::{Episema, Nc, NcChild, Oriscus, Quilisma};

    let mut original = Nc::default();
    original.basic.xml_id = Some("nc1".to_string());

    let mut quilisma = Quilisma::default();
    quilisma.common.xml_id = Some("q1".to_string());
    original
        .children
        .push(NcChild::Quilisma(Box::new(quilisma)));

    let mut oriscus = Oriscus::default();
    oriscus.common.xml_id = Some("or1".to_string());
    original.children.push(NcChild::Oriscus(Box::new(oriscus)));

    let mut episema = Episema::default();
    episema.common.xml_id = Some("ep1".to_string());
    original.children.push(NcChild::Episema(Box::new(episema)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Nc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 3);
    assert!(matches!(parsed.children[0], NcChild::Quilisma(_)));
    assert!(matches!(parsed.children[1], NcChild::Oriscus(_)));
    assert!(matches!(parsed.children[2], NcChild::Episema(_)));
}

// ============================================================================
// NcGrp Tests
// ============================================================================

#[test]
fn nc_grp_roundtrip_empty() {
    use tusk_model::elements::NcGrp;

    let original = NcGrp::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = NcGrp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn nc_grp_roundtrip_with_xml_id() {
    use tusk_model::elements::NcGrp;

    let mut original = NcGrp::default();
    original.common.xml_id = Some("ncg1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = NcGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ncg1".to_string()));
}

#[test]
fn nc_grp_roundtrip_with_nc_child() {
    use tusk_model::elements::{Nc, NcGrp, NcGrpChild};

    let mut original = NcGrp::default();
    original.common.xml_id = Some("ncg1".to_string());

    let mut nc = Nc::default();
    nc.basic.xml_id = Some("nc1".to_string());
    original.children.push(NcGrpChild::Nc(Box::new(nc)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = NcGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ncg1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        NcGrpChild::Nc(n) => {
            assert_eq!(n.basic.xml_id, Some("nc1".to_string()));
        }
        _ => panic!("Expected Nc child"),
    }
}

#[test]
fn nc_grp_roundtrip_with_nested_nc_grp() {
    use tusk_model::elements::{Nc, NcGrp, NcGrpChild};

    let mut original = NcGrp::default();
    original.common.xml_id = Some("ncg1".to_string());

    let mut inner_grp = NcGrp::default();
    inner_grp.common.xml_id = Some("ncg2".to_string());

    let mut nc = Nc::default();
    nc.basic.xml_id = Some("nc1".to_string());
    inner_grp.children.push(NcGrpChild::Nc(Box::new(nc)));

    original
        .children
        .push(NcGrpChild::NcGrp(Box::new(inner_grp)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = NcGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ncg1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        NcGrpChild::NcGrp(inner) => {
            assert_eq!(inner.common.xml_id, Some("ncg2".to_string()));
            assert_eq!(inner.children.len(), 1);
        }
        _ => panic!("Expected NcGrp child"),
    }
}

// ============================================================================
// Neume Tests
// ============================================================================

#[test]
fn neume_roundtrip_empty() {
    use tusk_model::elements::Neume;

    let original = Neume::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Neume::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn neume_roundtrip_with_xml_id() {
    use tusk_model::elements::Neume;

    let mut original = Neume::default();
    original.basic.xml_id = Some("n1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Neume::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("n1".to_string()));
}

#[test]
fn neume_roundtrip_with_nc_children() {
    use tusk_model::elements::{Nc, Neume, NeumeChild};

    let mut original = Neume::default();
    original.basic.xml_id = Some("n1".to_string());

    let mut nc1 = Nc::default();
    nc1.basic.xml_id = Some("nc1".to_string());
    nc1.nc_log.pname = Some("c".to_string());
    nc1.nc_log.oct = Some("4".to_string());
    original.children.push(NeumeChild::Nc(Box::new(nc1)));

    let mut nc2 = Nc::default();
    nc2.basic.xml_id = Some("nc2".to_string());
    nc2.nc_log.pname = Some("d".to_string());
    nc2.nc_log.oct = Some("4".to_string());
    original.children.push(NeumeChild::Nc(Box::new(nc2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Neume::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("n1".to_string()));
    assert_eq!(parsed.children.len(), 2);

    match &parsed.children[0] {
        NeumeChild::Nc(n) => {
            assert_eq!(n.basic.xml_id, Some("nc1".to_string()));
            assert_eq!(n.nc_log.pname, Some("c".to_string()));
        }
        _ => panic!("Expected Nc child"),
    }

    match &parsed.children[1] {
        NeumeChild::Nc(n) => {
            assert_eq!(n.basic.xml_id, Some("nc2".to_string()));
            assert_eq!(n.nc_log.pname, Some("d".to_string()));
        }
        _ => panic!("Expected Nc child"),
    }
}

#[test]
fn neume_roundtrip_with_mixed_children() {
    use tusk_model::elements::{Episema, Nc, Neume, NeumeChild};

    let mut original = Neume::default();
    original.basic.xml_id = Some("n1".to_string());

    let mut nc = Nc::default();
    nc.basic.xml_id = Some("nc1".to_string());
    original.children.push(NeumeChild::Nc(Box::new(nc)));

    let mut episema = Episema::default();
    episema.common.xml_id = Some("ep1".to_string());
    original
        .children
        .push(NeumeChild::Episema(Box::new(episema)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Neume::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 2);
    assert!(matches!(parsed.children[0], NeumeChild::Nc(_)));
    assert!(matches!(parsed.children[1], NeumeChild::Episema(_)));
}

// ============================================================================
// Syllable Tests
// ============================================================================

#[test]
fn syllable_roundtrip_empty() {
    use tusk_model::elements::Syllable;

    let original = Syllable::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Syllable::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn syllable_roundtrip_with_xml_id() {
    use tusk_model::elements::Syllable;

    let mut original = Syllable::default();
    original.common.xml_id = Some("syl1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Syllable::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("syl1".to_string()));
}

#[test]
fn syllable_roundtrip_with_neume_child() {
    use tusk_model::elements::{Neume, Syllable, SyllableChild};

    let mut original = Syllable::default();
    original.common.xml_id = Some("syl1".to_string());

    let mut neume = Neume::default();
    neume.basic.xml_id = Some("n1".to_string());
    original
        .children
        .push(SyllableChild::Neume(Box::new(neume)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Syllable::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("syl1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SyllableChild::Neume(n) => {
            assert_eq!(n.basic.xml_id, Some("n1".to_string()));
        }
        _ => panic!("Expected Neume child"),
    }
}

#[test]
fn syllable_roundtrip_with_full_neume_structure() {
    use tusk_model::elements::{Nc, NcChild, Neume, NeumeChild, Quilisma, Syllable, SyllableChild};

    let mut original = Syllable::default();
    original.common.xml_id = Some("syl1".to_string());

    // Create neume with nc children
    let mut neume = Neume::default();
    neume.basic.xml_id = Some("n1".to_string());

    let mut nc1 = Nc::default();
    nc1.basic.xml_id = Some("nc1".to_string());
    nc1.nc_log.pname = Some("c".to_string());
    nc1.nc_log.oct = Some("4".to_string());

    // Add quilisma to nc
    let mut quilisma = Quilisma::default();
    quilisma.common.xml_id = Some("q1".to_string());
    nc1.children.push(NcChild::Quilisma(Box::new(quilisma)));

    neume.children.push(NeumeChild::Nc(Box::new(nc1)));

    let mut nc2 = Nc::default();
    nc2.basic.xml_id = Some("nc2".to_string());
    nc2.nc_log.pname = Some("d".to_string());
    nc2.nc_log.oct = Some("4".to_string());
    neume.children.push(NeumeChild::Nc(Box::new(nc2)));

    original
        .children
        .push(SyllableChild::Neume(Box::new(neume)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Syllable::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("syl1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        SyllableChild::Neume(n) => {
            assert_eq!(n.basic.xml_id, Some("n1".to_string()));
            assert_eq!(n.children.len(), 2);

            match &n.children[0] {
                NeumeChild::Nc(nc) => {
                    assert_eq!(nc.basic.xml_id, Some("nc1".to_string()));
                    assert_eq!(nc.children.len(), 1);
                    assert!(matches!(nc.children[0], NcChild::Quilisma(_)));
                }
                _ => panic!("Expected Nc child"),
            }
        }
        _ => panic!("Expected Neume child"),
    }
}

#[test]
fn syllable_roundtrip_with_multiple_neumes() {
    use tusk_model::elements::{Episema, Neume, Syllable, SyllableChild};

    let mut original = Syllable::default();
    original.common.xml_id = Some("syl1".to_string());

    let mut neume1 = Neume::default();
    neume1.basic.xml_id = Some("n1".to_string());
    original
        .children
        .push(SyllableChild::Neume(Box::new(neume1)));

    let mut episema = Episema::default();
    episema.common.xml_id = Some("ep1".to_string());
    original
        .children
        .push(SyllableChild::Episema(Box::new(episema)));

    let mut neume2 = Neume::default();
    neume2.basic.xml_id = Some("n2".to_string());
    original
        .children
        .push(SyllableChild::Neume(Box::new(neume2)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Syllable::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 3);
    assert!(matches!(parsed.children[0], SyllableChild::Neume(_)));
    assert!(matches!(parsed.children[1], SyllableChild::Episema(_)));
    assert!(matches!(parsed.children[2], SyllableChild::Neume(_)));
}

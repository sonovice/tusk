//! Round-trip serialization tests for analysis, gestural, and linkage MEI elements.
//!
//! Tests for Phase 12 elements:
//! - Ambitus, AmbNote
//! - OStaff, OLayer
//! - Attacca
//! - When, Clip
//! - Expansion
//! - CpMark
//! - GenDesc, GenState
//! - MetaMark

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Ambitus Tests
// ============================================================================

#[test]
fn ambitus_roundtrip_empty() {
    use tusk_model::elements::Ambitus;

    let original = Ambitus::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ambitus::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn ambitus_roundtrip_with_xml_id() {
    use tusk_model::elements::Ambitus;

    let mut original = Ambitus::default();
    original.common.xml_id = Some("ambitus-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ambitus::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ambitus-1".to_string()));
}

#[test]
fn ambitus_roundtrip_with_amb_note() {
    use tusk_model::elements::{AmbNote, Ambitus, AmbitusChild};

    let mut original = Ambitus::default();
    original.common.xml_id = Some("ambitus-1".to_string());

    let mut amb_note = AmbNote::default();
    amb_note.common.xml_id = Some("ambnote-1".to_string());
    original
        .children
        .push(AmbitusChild::AmbNote(Box::new(amb_note)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Ambitus::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        AmbitusChild::AmbNote(an) => {
            assert_eq!(an.common.xml_id, Some("ambnote-1".to_string()));
        }
    }
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
    original.common.xml_id = Some("ambnote-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = AmbNote::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ambnote-1".to_string()));
}

// ============================================================================
// OStaff Tests
// ============================================================================

#[test]
fn o_staff_roundtrip_empty() {
    use tusk_model::elements::OStaff;

    let original = OStaff::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = OStaff::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn o_staff_roundtrip_with_n() {
    use tusk_model::elements::OStaff;

    let mut original = OStaff::default();
    original.basic.xml_id = Some("ostaff-1".to_string());
    original.n_integer.n = Some(1);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = OStaff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("ostaff-1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
}

// ============================================================================
// OLayer Tests
// ============================================================================

#[test]
fn o_layer_roundtrip_empty() {
    use tusk_model::elements::OLayer;

    let original = OLayer::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = OLayer::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.basic.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn o_layer_roundtrip_with_n() {
    use tusk_model::elements::OLayer;

    let mut original = OLayer::default();
    original.basic.xml_id = Some("olayer-1".to_string());
    original.n_integer.n = Some(1);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = OLayer::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.basic.xml_id, Some("olayer-1".to_string()));
    assert_eq!(parsed.n_integer.n, Some(1));
}

// ============================================================================
// Attacca Tests
// ============================================================================

#[test]
fn attacca_roundtrip_empty() {
    use tusk_model::elements::Attacca;

    let original = Attacca::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Attacca::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
#[ignore = "RNG model has no AttaccaChild content model"]
fn attacca_roundtrip_with_text() {
    // Skipped: internal RNG model does not define AttaccaChild.
}

// ============================================================================
// When Tests
// ============================================================================

#[test]
fn when_roundtrip_empty() {
    use tusk_model::elements::When;

    let original = When::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = When::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.absolute.is_none());
    assert!(parsed.interval.is_none());
}

#[test]
fn when_roundtrip_with_absolute() {
    use tusk_model::elements::When;

    let mut original = When::default();
    original.common.xml_id = Some("when-1".to_string());
    original.absolute = Some("00:01:30".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = When::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("when-1".to_string()));
    assert_eq!(parsed.absolute, Some("00:01:30".to_string()));
}

#[test]
fn when_roundtrip_with_interval() {
    use tusk_model::elements::When;

    let mut original = When::default();
    original.common.xml_id = Some("when-2".to_string());
    original.interval = Some("5.0".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = When::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("when-2".to_string()));
    assert_eq!(parsed.interval, Some("5.0".to_string()));
}

// ============================================================================
// Clip Tests
// ============================================================================

#[test]
fn clip_roundtrip_empty() {
    use tusk_model::elements::Clip;

    let original = Clip::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Clip::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn clip_roundtrip_with_bounds() {
    use tusk_model::elements::Clip;

    let mut original = Clip::default();
    original.common.xml_id = Some("clip-1".to_string());
    original.media_bounds.begin = Some("00:00:30".to_string());
    original.media_bounds.end = Some("00:01:00".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Clip::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("clip-1".to_string()));
    assert_eq!(parsed.media_bounds.begin, Some("00:00:30".to_string()));
    assert_eq!(parsed.media_bounds.end, Some("00:01:00".to_string()));
}

#[test]
fn clip_roundtrip_with_when() {
    use tusk_model::elements::{Clip, ClipChild, When};

    let mut original = Clip::default();
    original.common.xml_id = Some("clip-1".to_string());

    let mut when = When::default();
    when.common.xml_id = Some("when-1".to_string());
    when.absolute = Some("00:00:15".to_string());
    original.children.push(ClipChild::When(Box::new(when)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Clip::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        ClipChild::When(w) => {
            assert_eq!(w.common.xml_id, Some("when-1".to_string()));
            assert_eq!(w.absolute, Some("00:00:15".to_string()));
        }
        _ => panic!("Expected When child"),
    }
}

// ============================================================================
// Expansion Tests
// ============================================================================

#[test]
fn expansion_roundtrip_empty() {
    use tusk_model::elements::Expansion;

    let original = Expansion::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Expansion::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
}

#[test]
fn expansion_roundtrip_with_xml_id() {
    use tusk_model::elements::Expansion;

    let mut original = Expansion::default();
    original.common.xml_id = Some("expansion-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Expansion::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("expansion-1".to_string()));
}

// ============================================================================
// CpMark Tests
// ============================================================================

#[test]
fn cp_mark_roundtrip_empty() {
    use tusk_model::elements::CpMark;

    let original = CpMark::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = CpMark::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
#[ignore = "RNG model has no CpMarkChild content model"]
fn cp_mark_roundtrip_with_text() {
    // Skipped: internal RNG model does not define CpMarkChild.
}

// ============================================================================
// GenDesc Tests
// ============================================================================

#[test]
fn gen_desc_roundtrip_empty() {
    use tusk_model::elements::GenDesc;

    let original = GenDesc::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = GenDesc::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn gen_desc_roundtrip_with_ordered() {
    use tusk_model::elements::GenDesc;
    use tusk_model::generated::data::DataBoolean;

    let mut original = GenDesc::default();
    original.common.xml_id = Some("gendesc-1".to_string());
    original.ordered = Some(DataBoolean::True);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GenDesc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("gendesc-1".to_string()));
    assert_eq!(parsed.ordered, Some(DataBoolean::True));
}

#[test]
fn gen_desc_roundtrip_with_nested() {
    use tusk_model::elements::{GenDesc, GenDescChild};

    let mut original = GenDesc::default();
    original.common.xml_id = Some("gendesc-1".to_string());

    let mut nested = GenDesc::default();
    nested.common.xml_id = Some("gendesc-nested".to_string());
    original
        .children
        .push(GenDescChild::GenDesc(Box::new(nested)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GenDesc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GenDescChild::GenDesc(gd) => {
            assert_eq!(gd.common.xml_id, Some("gendesc-nested".to_string()));
        }
        _ => panic!("Expected GenDesc child"),
    }
}

#[test]
fn gen_desc_roundtrip_with_gen_state() {
    use tusk_model::elements::{GenDesc, GenDescChild, GenState};

    let mut original = GenDesc::default();
    original.common.xml_id = Some("gendesc-1".to_string());

    let mut gen_state = GenState::default();
    gen_state.common.xml_id = Some("genstate-1".to_string());
    original
        .children
        .push(GenDescChild::GenState(Box::new(gen_state)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GenDesc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        GenDescChild::GenState(gs) => {
            assert_eq!(gs.common.xml_id, Some("genstate-1".to_string()));
        }
        _ => panic!("Expected GenState child"),
    }
}

// ============================================================================
// GenState Tests
// ============================================================================

#[test]
fn gen_state_roundtrip_empty() {
    use tusk_model::elements::GenState;

    let original = GenState::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = GenState::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn gen_state_roundtrip_with_xml_id() {
    use tusk_model::elements::GenState;

    let mut original = GenState::default();
    original.common.xml_id = Some("genstate-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = GenState::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("genstate-1".to_string()));
}

// ============================================================================
// MetaMark Tests
// ============================================================================

#[test]
fn meta_mark_roundtrip_empty() {
    use tusk_model::elements::MetaMark;

    let original = MetaMark::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = MetaMark::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn meta_mark_roundtrip_with_function() {
    use tusk_model::elements::MetaMark;

    let mut original = MetaMark::default();
    original.common.xml_id = Some("metamark-1".to_string());
    original.function = Some("confirmation".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = MetaMark::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("metamark-1".to_string()));
    assert_eq!(parsed.function, Some("confirmation".to_string()));
}

#[test]
#[ignore = "RNG model has no MetaMarkChild content model"]
fn meta_mark_roundtrip_with_text() {
    // Skipped: internal RNG model does not define MetaMarkChild.
}

//! Round-trip serialization tests for MIDI-related MEI elements.
//!
//! Tests for Midi and InstrGrp elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Midi Tests
// ============================================================================

#[test]
fn midi_roundtrip_empty() {
    use tusk_model::elements::Midi;

    let original = Midi::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn midi_roundtrip_with_xml_id() {
    use tusk_model::elements::Midi;

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("midi1".to_string()));
}

#[test]
fn midi_roundtrip_with_staff() {
    use tusk_model::elements::Midi;

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());
    original.midi_log.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("midi1".to_string()));
    assert_eq!(parsed.midi_log.staff, vec![1]);
}

#[test]
fn midi_roundtrip_with_multiple_staff() {
    use tusk_model::elements::Midi;

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());
    original.midi_log.staff = vec![1, 2, 3];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.midi_log.staff, vec![1, 2, 3]);
}

#[test]
fn midi_roundtrip_with_layer() {
    use tusk_model::elements::Midi;

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());
    original.midi_log.layer = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.midi_log.layer, vec![1]);
}

#[test]
fn midi_roundtrip_with_part() {
    use tusk_model::elements::Midi;

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());
    original.midi_log.part = vec!["P1".to_string()];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.midi_log.part, vec!["P1".to_string()]);
}

#[test]
fn midi_roundtrip_with_all_attributes() {
    use tusk_model::elements::Midi;

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());
    original.common.label = Some("MIDI data".to_string());
    original.midi_log.staff = vec![1, 2];
    original.midi_log.layer = vec![1];
    original.midi_log.part = vec!["P1".to_string(), "P2".to_string()];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("midi1".to_string()));
    assert_eq!(parsed.common.label, Some("MIDI data".to_string()));
    assert_eq!(parsed.midi_log.staff, vec![1, 2]);
    assert_eq!(parsed.midi_log.layer, vec![1]);
    assert_eq!(
        parsed.midi_log.part,
        vec!["P1".to_string(), "P2".to_string()]
    );
}

// ============================================================================
// InstrGrp Tests
// ============================================================================

#[test]
fn instr_grp_roundtrip_empty() {
    use tusk_model::elements::InstrGrp;

    let original = InstrGrp::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = InstrGrp::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn instr_grp_roundtrip_with_xml_id() {
    use tusk_model::elements::InstrGrp;

    let mut original = InstrGrp::default();
    original.common.xml_id = Some("ig1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = InstrGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ig1".to_string()));
}

#[test]
fn instr_grp_roundtrip_with_instr_def() {
    use tusk_model::elements::{InstrDef, InstrGrp, InstrGrpChild};

    let mut original = InstrGrp::default();
    original.common.xml_id = Some("ig1".to_string());

    let mut instr_def = InstrDef::default();
    instr_def.basic.xml_id = Some("id1".to_string());
    instr_def.labelled.label = Some("Piano".to_string());
    instr_def.n_integer.n = Some(1);
    original
        .children
        .push(InstrGrpChild::InstrDef(Box::new(instr_def)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = InstrGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ig1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        InstrGrpChild::InstrDef(id) => {
            assert_eq!(id.basic.xml_id, Some("id1".to_string()));
            assert_eq!(id.labelled.label, Some("Piano".to_string()));
            assert_eq!(id.n_integer.n, Some(1));
        }
    }
}

#[test]
fn instr_grp_roundtrip_with_multiple_instr_defs() {
    use tusk_model::elements::{InstrDef, InstrGrp, InstrGrpChild};

    let mut original = InstrGrp::default();
    original.common.xml_id = Some("strings".to_string());

    // Add violin
    let mut violin = InstrDef::default();
    violin.basic.xml_id = Some("id-vln".to_string());
    violin.labelled.label = Some("Violin".to_string());
    violin.n_integer.n = Some(1);
    original
        .children
        .push(InstrGrpChild::InstrDef(Box::new(violin)));

    // Add viola
    let mut viola = InstrDef::default();
    viola.basic.xml_id = Some("id-vla".to_string());
    viola.labelled.label = Some("Viola".to_string());
    viola.n_integer.n = Some(2);
    original
        .children
        .push(InstrGrpChild::InstrDef(Box::new(viola)));

    // Add cello
    let mut cello = InstrDef::default();
    cello.basic.xml_id = Some("id-vc".to_string());
    cello.labelled.label = Some("Cello".to_string());
    cello.n_integer.n = Some(3);
    original
        .children
        .push(InstrGrpChild::InstrDef(Box::new(cello)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = InstrGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("strings".to_string()));
    assert_eq!(parsed.children.len(), 3);

    // Verify all children
    let labels: Vec<_> = parsed
        .children
        .iter()
        .map(|c| match c {
            InstrGrpChild::InstrDef(id) => id.labelled.label.clone(),
        })
        .collect();

    assert_eq!(
        labels,
        vec![
            Some("Violin".to_string()),
            Some("Viola".to_string()),
            Some("Cello".to_string())
        ]
    );
}

#[test]
fn instr_grp_roundtrip_with_label() {
    use tusk_model::elements::InstrGrp;

    let mut original = InstrGrp::default();
    original.common.xml_id = Some("ig1".to_string());
    original.common.label = Some("String Section".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = InstrGrp::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("ig1".to_string()));
    assert_eq!(parsed.common.label, Some("String Section".to_string()));
}

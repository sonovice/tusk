//! Round-trip serialization tests for MIDI-related MEI elements.
//!
//! Tests for Midi, InstrGrp, and MIDI control elements (Cc, Chan, ChanPr, Port, Prog, Vel).

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;
use tusk_model::generated::data::{DataBeat, DataMidichannel, DataMidivalue};

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

// ============================================================================
// Cc (Control Change) Tests
// ============================================================================

#[test]
fn cc_roundtrip_empty() {
    use tusk_model::elements::Cc;

    let original = Cc::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cc::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.midi_number.num.is_none());
    assert!(parsed.midi_value.val.is_none());
}

#[test]
fn cc_roundtrip_with_attributes() {
    use tusk_model::elements::Cc;

    let mut original = Cc::default();
    original.common.xml_id = Some("cc1".to_string());
    original.midi_number.num = Some(DataMidivalue("64".to_string()));
    original.midi_value.val = Some(DataMidivalue("127".to_string()));
    original.midi_event.staff = vec![1];
    original.midi_event.layer = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cc::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("cc1".to_string()));
    assert_eq!(
        parsed.midi_number.num,
        Some(DataMidivalue("64".to_string()))
    );
    assert_eq!(
        parsed.midi_value.val,
        Some(DataMidivalue("127".to_string()))
    );
    assert_eq!(parsed.midi_event.staff, vec![1]);
    assert_eq!(parsed.midi_event.layer, vec![1]);
}

// ============================================================================
// Chan (Channel) Tests
// ============================================================================

#[test]
fn chan_roundtrip_empty() {
    use tusk_model::elements::Chan;

    let original = Chan::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Chan::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.num.is_none());
}

#[test]
fn chan_roundtrip_with_attributes() {
    use tusk_model::elements::Chan;

    let mut original = Chan::default();
    original.common.xml_id = Some("chan1".to_string());
    original.num = Some(DataMidichannel("10".to_string()));
    original.midi_event.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Chan::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("chan1".to_string()));
    assert_eq!(parsed.num, Some(DataMidichannel("10".to_string())));
    assert_eq!(parsed.midi_event.staff, vec![1]);
}

// ============================================================================
// ChanPr (Channel Pressure) Tests
// ============================================================================

#[test]
fn chan_pr_roundtrip_empty() {
    use tusk_model::elements::ChanPr;

    let original = ChanPr::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = ChanPr::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.midi_number.num.is_none());
}

#[test]
fn chan_pr_roundtrip_with_attributes() {
    use tusk_model::elements::ChanPr;

    let mut original = ChanPr::default();
    original.common.xml_id = Some("cp1".to_string());
    original.midi_number.num = Some(DataMidivalue("64".to_string()));
    original.midi_event.staff = vec![1, 2];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = ChanPr::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("cp1".to_string()));
    assert_eq!(
        parsed.midi_number.num,
        Some(DataMidivalue("64".to_string()))
    );
    assert_eq!(parsed.midi_event.staff, vec![1, 2]);
}

// ============================================================================
// Port Tests
// ============================================================================

#[test]
fn port_roundtrip_empty() {
    use tusk_model::elements::Port;

    let original = Port::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Port::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.midi_number.num.is_none());
}

#[test]
fn port_roundtrip_with_attributes() {
    use tusk_model::elements::Port;

    let mut original = Port::default();
    original.common.xml_id = Some("port1".to_string());
    original.midi_number.num = Some(DataMidivalue("1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Port::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("port1".to_string()));
    assert_eq!(parsed.midi_number.num, Some(DataMidivalue("1".to_string())));
}

// ============================================================================
// Prog (Program Change) Tests
// ============================================================================

#[test]
fn prog_roundtrip_empty() {
    use tusk_model::elements::Prog;

    let original = Prog::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Prog::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.midi_number.num.is_none());
}

#[test]
fn prog_roundtrip_with_attributes() {
    use tusk_model::elements::Prog;

    let mut original = Prog::default();
    original.common.xml_id = Some("prog1".to_string());
    original.midi_number.num = Some(DataMidivalue("1".to_string()));
    original.midi_event.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Prog::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("prog1".to_string()));
    assert_eq!(parsed.midi_number.num, Some(DataMidivalue("1".to_string())));
    assert_eq!(parsed.midi_event.staff, vec![1]);
}

// ============================================================================
// Vel (Velocity) Tests
// ============================================================================

#[test]
fn vel_roundtrip_empty() {
    use tusk_model::elements::Vel;

    let original = Vel::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Vel::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.midi_number.num.is_none());
    assert!(parsed.form.is_none());
}

#[test]
fn vel_roundtrip_with_attributes() {
    use tusk_model::elements::Vel;

    let mut original = Vel::default();
    original.common.xml_id = Some("vel1".to_string());
    original.midi_number.num = Some(DataMidivalue("90".to_string()));
    original.form = Some("noteOn".to_string());
    original.midi_event.staff = vec![1];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Vel::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("vel1".to_string()));
    assert_eq!(
        parsed.midi_number.num,
        Some(DataMidivalue("90".to_string()))
    );
    assert_eq!(parsed.form, Some("noteOn".to_string()));
    assert_eq!(parsed.midi_event.staff, vec![1]);
}

// ============================================================================
// Midi with control element children Tests
// ============================================================================

#[test]
fn midi_roundtrip_with_cc_child() {
    use tusk_model::elements::{Cc, Midi, MidiChild};

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());

    let mut cc = Cc::default();
    cc.common.xml_id = Some("cc1".to_string());
    cc.midi_number.num = Some(DataMidivalue("64".to_string()));
    cc.midi_value.val = Some(DataMidivalue("127".to_string()));
    original.children.push(MidiChild::Cc(Box::new(cc)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("midi1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        MidiChild::Cc(cc) => {
            assert_eq!(cc.common.xml_id, Some("cc1".to_string()));
            assert_eq!(cc.midi_number.num, Some(DataMidivalue("64".to_string())));
            assert_eq!(cc.midi_value.val, Some(DataMidivalue("127".to_string())));
        }
        _ => panic!("Expected Cc child"),
    }
}

#[test]
fn midi_roundtrip_with_multiple_control_children() {
    use tusk_model::elements::{Cc, Chan, Midi, MidiChild, Prog, Vel};

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());

    // Add prog
    let mut prog = Prog::default();
    prog.common.xml_id = Some("prog1".to_string());
    prog.midi_number.num = Some(DataMidivalue("1".to_string()));
    original.children.push(MidiChild::Prog(Box::new(prog)));

    // Add chan
    let mut chan = Chan::default();
    chan.common.xml_id = Some("chan1".to_string());
    chan.num = Some(DataMidichannel("1".to_string()));
    original.children.push(MidiChild::Chan(Box::new(chan)));

    // Add cc
    let mut cc = Cc::default();
    cc.common.xml_id = Some("cc1".to_string());
    cc.midi_number.num = Some(DataMidivalue("7".to_string()));
    cc.midi_value.val = Some(DataMidivalue("100".to_string()));
    original.children.push(MidiChild::Cc(Box::new(cc)));

    // Add vel
    let mut vel = Vel::default();
    vel.common.xml_id = Some("vel1".to_string());
    vel.midi_number.num = Some(DataMidivalue("64".to_string()));
    vel.form = Some("noteOn".to_string());
    original.children.push(MidiChild::Vel(Box::new(vel)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("midi1".to_string()));
    assert_eq!(parsed.children.len(), 4);

    // Check order and types
    assert!(matches!(parsed.children[0], MidiChild::Prog(_)));
    assert!(matches!(parsed.children[1], MidiChild::Chan(_)));
    assert!(matches!(parsed.children[2], MidiChild::Cc(_)));
    assert!(matches!(parsed.children[3], MidiChild::Vel(_)));

    // Check specific values
    match &parsed.children[0] {
        MidiChild::Prog(p) => {
            assert_eq!(p.common.xml_id, Some("prog1".to_string()));
        }
        _ => panic!("Expected Prog"),
    }
    match &parsed.children[1] {
        MidiChild::Chan(c) => {
            assert_eq!(c.num, Some(DataMidichannel("1".to_string())));
        }
        _ => panic!("Expected Chan"),
    }
    match &parsed.children[2] {
        MidiChild::Cc(cc) => {
            assert_eq!(cc.midi_value.val, Some(DataMidivalue("100".to_string())));
        }
        _ => panic!("Expected Cc"),
    }
    match &parsed.children[3] {
        MidiChild::Vel(v) => {
            assert_eq!(v.form, Some("noteOn".to_string()));
        }
        _ => panic!("Expected Vel"),
    }
}

// ============================================================================
// NoteOn Tests
// ============================================================================

#[test]
fn note_on_roundtrip_empty() {
    use tusk_model::elements::NoteOn;

    let original = NoteOn::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = NoteOn::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.midi_number.num.is_none());
}

#[test]
fn note_on_roundtrip_with_attributes() {
    use tusk_model::elements::NoteOn;

    let mut original = NoteOn::default();
    original.common.xml_id = Some("non1".to_string());
    original.midi_number.num = Some(DataMidivalue("60".to_string()));
    original.midi_event.staff = vec![1];
    original.midi_event.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = NoteOn::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("non1".to_string()));
    assert_eq!(
        parsed.midi_number.num,
        Some(DataMidivalue("60".to_string()))
    );
    assert_eq!(parsed.midi_event.staff, vec![1]);
    assert_eq!(parsed.midi_event.tstamp, Some(DataBeat(1.0)));
}

// ============================================================================
// NoteOff Tests
// ============================================================================

#[test]
fn note_off_roundtrip_empty() {
    use tusk_model::elements::NoteOff;

    let original = NoteOff::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = NoteOff::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.midi_number.num.is_none());
}

#[test]
fn note_off_roundtrip_with_attributes() {
    use tusk_model::elements::NoteOff;

    let mut original = NoteOff::default();
    original.common.xml_id = Some("nof1".to_string());
    original.midi_number.num = Some(DataMidivalue("60".to_string()));
    original.midi_event.staff = vec![1];
    original.midi_event.tstamp = Some(DataBeat(2.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = NoteOff::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("nof1".to_string()));
    assert_eq!(
        parsed.midi_number.num,
        Some(DataMidivalue("60".to_string()))
    );
    assert_eq!(parsed.midi_event.staff, vec![1]);
    assert_eq!(parsed.midi_event.tstamp, Some(DataBeat(2.0)));
}

// ============================================================================
// Cue Tests
// ============================================================================

#[test]
fn cue_roundtrip_empty() {
    use tusk_model::elements::Cue;

    let original = Cue::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cue::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn cue_roundtrip_with_attributes() {
    use tusk_model::elements::Cue;

    let mut original = Cue::default();
    original.common.xml_id = Some("cue1".to_string());
    original.midi_event.staff = vec![1];
    original.midi_event.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cue::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("cue1".to_string()));
    assert_eq!(parsed.midi_event.staff, vec![1]);
    assert_eq!(parsed.midi_event.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn cue_roundtrip_with_text_content() {
    use tusk_model::elements::{Cue, CueChild};

    let mut original = Cue::default();
    original.common.xml_id = Some("cue1".to_string());
    original
        .children
        .push(CueChild::Text("Verse 1".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Cue::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("cue1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        CueChild::Text(text) => assert_eq!(text, "Verse 1"),
    }
}

// ============================================================================
// Marker Tests
// ============================================================================

#[test]
fn marker_roundtrip_empty() {
    use tusk_model::elements::Marker;

    let original = Marker::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Marker::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn marker_roundtrip_with_attributes() {
    use tusk_model::elements::Marker;

    let mut original = Marker::default();
    original.common.xml_id = Some("mrk1".to_string());
    original.midi_event.staff = vec![1];
    original.midi_event.tstamp = Some(DataBeat(1.0));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Marker::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mrk1".to_string()));
    assert_eq!(parsed.midi_event.staff, vec![1]);
    assert_eq!(parsed.midi_event.tstamp, Some(DataBeat(1.0)));
}

#[test]
fn marker_roundtrip_with_text_content() {
    use tusk_model::elements::{Marker, MarkerChild};

    let mut original = Marker::default();
    original.common.xml_id = Some("mrk1".to_string());
    original
        .children
        .push(MarkerChild::Text("Chorus".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Marker::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("mrk1".to_string()));
    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MarkerChild::Text(text) => assert_eq!(text, "Chorus"),
    }
}

// ============================================================================
// Midi with new event children Tests
// ============================================================================

#[test]
fn midi_roundtrip_with_note_on_child() {
    use tusk_model::elements::{Midi, MidiChild, NoteOn};

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());

    let mut note_on = NoteOn::default();
    note_on.common.xml_id = Some("non1".to_string());
    note_on.midi_number.num = Some(DataMidivalue("60".to_string()));
    original.children.push(MidiChild::NoteOn(Box::new(note_on)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MidiChild::NoteOn(n) => {
            assert_eq!(n.common.xml_id, Some("non1".to_string()));
            assert_eq!(n.midi_number.num, Some(DataMidivalue("60".to_string())));
        }
        _ => panic!("Expected NoteOn child"),
    }
}

#[test]
fn midi_roundtrip_with_note_off_child() {
    use tusk_model::elements::{Midi, MidiChild, NoteOff};

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());

    let mut note_off = NoteOff::default();
    note_off.common.xml_id = Some("nof1".to_string());
    note_off.midi_number.num = Some(DataMidivalue("60".to_string()));
    original
        .children
        .push(MidiChild::NoteOff(Box::new(note_off)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MidiChild::NoteOff(n) => {
            assert_eq!(n.common.xml_id, Some("nof1".to_string()));
            assert_eq!(n.midi_number.num, Some(DataMidivalue("60".to_string())));
        }
        _ => panic!("Expected NoteOff child"),
    }
}

#[test]
fn midi_roundtrip_with_cue_child() {
    use tusk_model::elements::{Cue, CueChild, Midi, MidiChild};

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());

    let mut cue = Cue::default();
    cue.common.xml_id = Some("cue1".to_string());
    cue.children.push(CueChild::Text("Intro".to_string()));
    original.children.push(MidiChild::Cue(Box::new(cue)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MidiChild::Cue(c) => {
            assert_eq!(c.common.xml_id, Some("cue1".to_string()));
            assert_eq!(c.children.len(), 1);
            match &c.children[0] {
                CueChild::Text(text) => assert_eq!(text, "Intro"),
            }
        }
        _ => panic!("Expected Cue child"),
    }
}

#[test]
fn midi_roundtrip_with_marker_child() {
    use tusk_model::elements::{Marker, MarkerChild, Midi, MidiChild};

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());

    let mut marker = Marker::default();
    marker.common.xml_id = Some("mrk1".to_string());
    marker
        .children
        .push(MarkerChild::Text("Bridge".to_string()));
    original.children.push(MidiChild::Marker(Box::new(marker)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        MidiChild::Marker(m) => {
            assert_eq!(m.common.xml_id, Some("mrk1".to_string()));
            assert_eq!(m.children.len(), 1);
            match &m.children[0] {
                MarkerChild::Text(text) => assert_eq!(text, "Bridge"),
            }
        }
        _ => panic!("Expected Marker child"),
    }
}

#[test]
fn midi_roundtrip_with_mixed_event_children() {
    use tusk_model::elements::{
        Cue, CueChild, Marker, MarkerChild, Midi, MidiChild, NoteOff, NoteOn, Prog,
    };

    let mut original = Midi::default();
    original.common.xml_id = Some("midi1".to_string());

    // Add prog
    let mut prog = Prog::default();
    prog.midi_number.num = Some(DataMidivalue("1".to_string()));
    original.children.push(MidiChild::Prog(Box::new(prog)));

    // Add noteOn
    let mut note_on = NoteOn::default();
    note_on.midi_number.num = Some(DataMidivalue("60".to_string()));
    note_on.midi_event.tstamp = Some(DataBeat(1.0));
    original.children.push(MidiChild::NoteOn(Box::new(note_on)));

    // Add noteOff
    let mut note_off = NoteOff::default();
    note_off.midi_number.num = Some(DataMidivalue("60".to_string()));
    note_off.midi_event.tstamp = Some(DataBeat(2.0));
    original
        .children
        .push(MidiChild::NoteOff(Box::new(note_off)));

    // Add cue
    let mut cue = Cue::default();
    cue.children.push(CueChild::Text("Verse".to_string()));
    original.children.push(MidiChild::Cue(Box::new(cue)));

    // Add marker
    let mut marker = Marker::default();
    marker
        .children
        .push(MarkerChild::Text("Chorus".to_string()));
    original.children.push(MidiChild::Marker(Box::new(marker)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Midi::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 5);
    assert!(matches!(parsed.children[0], MidiChild::Prog(_)));
    assert!(matches!(parsed.children[1], MidiChild::NoteOn(_)));
    assert!(matches!(parsed.children[2], MidiChild::NoteOff(_)));
    assert!(matches!(parsed.children[3], MidiChild::Cue(_)));
    assert!(matches!(parsed.children[4], MidiChild::Marker(_)));
}

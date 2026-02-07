//! Deserializer implementations for MIDI-related MEI elements.
//!
//! This module contains implementations for Midi, InstrGrp, and MIDI control elements
//! (Cc, Chan, ChanPr, Port, Prog, Vel).

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::att::{
    AttMidiAnl, AttMidiEvent, AttMidiGes, AttMidiLog, AttMidiNumber, AttMidiValue,
};
use tusk_model::elements::{
    Cc, Chan, ChanPr, Cue, CueChild, Hex, HexChild, InstrDef, InstrGrp, InstrGrpChild, Marker,
    MarkerChild, MetaText, MetaTextChild, Midi, MidiChild, NoteOff, NoteOn, Port, Prog, SeqNum,
    TrkName, TrkNameChild, Vel,
};

use super::{extract_attr, from_attr_string};

// ============================================================================
// Midi attribute class implementations
// ============================================================================

// ============================================================================
// MeiDeserialize implementations
// ============================================================================

impl MeiDeserialize for Midi {
    fn element_name() -> &'static str {
        "midi"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut midi = Midi::default();

        // Extract attributes into each attribute class
        midi.common.extract_attributes(&mut attrs)?;
        midi.midi_log.extract_attributes(&mut attrs)?;
        midi.midi_ges.extract_attributes(&mut attrs)?;
        midi.midi_anl.extract_attributes(&mut attrs)?;

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("midi")?
            {
                match name.as_str() {
                    "cc" => {
                        let cc = Cc::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::Cc(Box::new(cc)));
                    }
                    "chan" => {
                        let chan = Chan::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::Chan(Box::new(chan)));
                    }
                    "chanPr" => {
                        let chan_pr = ChanPr::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::ChanPr(Box::new(chan_pr)));
                    }
                    "port" => {
                        let port = Port::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::Port(Box::new(port)));
                    }
                    "prog" => {
                        let prog = Prog::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::Prog(Box::new(prog)));
                    }
                    "vel" => {
                        let vel = Vel::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::Vel(Box::new(vel)));
                    }
                    "noteOn" => {
                        let note_on = NoteOn::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::NoteOn(Box::new(note_on)));
                    }
                    "noteOff" => {
                        let note_off = NoteOff::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::NoteOff(Box::new(note_off)));
                    }
                    "cue" => {
                        let cue = Cue::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::Cue(Box::new(cue)));
                    }
                    "marker" => {
                        let marker = Marker::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::Marker(Box::new(marker)));
                    }
                    "metaText" => {
                        let meta_text = MetaText::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::MetaText(Box::new(meta_text)));
                    }
                    "seqNum" => {
                        let seq_num = SeqNum::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::SeqNum(Box::new(seq_num)));
                    }
                    "trkName" => {
                        let trk_name = TrkName::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::TrkName(Box::new(trk_name)));
                    }
                    "hex" => {
                        let hex = Hex::from_mei_event(reader, child_attrs, child_empty)?;
                        midi.children.push(MidiChild::Hex(Box::new(hex)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "midi", child_empty)?;
                    }
                }
            }
        }

        Ok(midi)
    }
}

impl MeiDeserialize for InstrGrp {
    fn element_name() -> &'static str {
        "instrGrp"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut instr_grp = InstrGrp::default();

        // Extract attributes into common attribute class
        instr_grp.common.extract_attributes(&mut attrs)?;

        // Read children if not empty
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("instrGrp")?
            {
                match name.as_str() {
                    "instrDef" => {
                        let instr_def = InstrDef::from_mei_event(reader, child_attrs, child_empty)?;
                        instr_grp
                            .children
                            .push(InstrGrpChild::InstrDef(Box::new(instr_def)));
                    }
                    // Unknown children are skipped (lenient mode)
                    _ => {
                        reader.skip_unknown_child(&name, "instrGrp", child_empty)?;
                    }
                }
            }
        }

        Ok(instr_grp)
    }
}

// ============================================================================
// MIDI Control Element implementations
// ============================================================================

impl MeiDeserialize for Cc {
    fn element_name() -> &'static str {
        "cc"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut cc = Cc::default();

        cc.common.extract_attributes(&mut attrs)?;
        cc.midi_event.extract_attributes(&mut attrs)?;
        cc.midi_number.extract_attributes(&mut attrs)?;
        cc.midi_value.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("cc")?;
        }

        Ok(cc)
    }
}

impl MeiDeserialize for Chan {
    fn element_name() -> &'static str {
        "chan"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut chan = Chan::default();

        chan.common.extract_attributes(&mut attrs)?;
        chan.midi_event.extract_attributes(&mut attrs)?;
        // Chan has its own `num` attribute (DataMidichannel type)
        extract_attr!(attrs, "num", chan.num);

        if !is_empty {
            reader.skip_to_end("chan")?;
        }

        Ok(chan)
    }
}

impl MeiDeserialize for ChanPr {
    fn element_name() -> &'static str {
        "chanPr"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut chan_pr = ChanPr::default();

        chan_pr.common.extract_attributes(&mut attrs)?;
        chan_pr.midi_event.extract_attributes(&mut attrs)?;
        chan_pr.midi_number.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("chanPr")?;
        }

        Ok(chan_pr)
    }
}

impl MeiDeserialize for Port {
    fn element_name() -> &'static str {
        "port"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut port = Port::default();

        port.common.extract_attributes(&mut attrs)?;
        port.midi_event.extract_attributes(&mut attrs)?;
        port.midi_number.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("port")?;
        }

        Ok(port)
    }
}

impl MeiDeserialize for Prog {
    fn element_name() -> &'static str {
        "prog"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut prog = Prog::default();

        prog.common.extract_attributes(&mut attrs)?;
        prog.midi_event.extract_attributes(&mut attrs)?;
        prog.midi_number.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("prog")?;
        }

        Ok(prog)
    }
}

impl MeiDeserialize for Vel {
    fn element_name() -> &'static str {
        "vel"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut vel = Vel::default();

        vel.common.extract_attributes(&mut attrs)?;
        vel.midi_event.extract_attributes(&mut attrs)?;
        vel.midi_number.extract_attributes(&mut attrs)?;
        // Vel has its own `form` attribute
        extract_attr!(attrs, "form", string vel.form);

        if !is_empty {
            reader.skip_to_end("vel")?;
        }

        Ok(vel)
    }
}

// ============================================================================
// MIDI Event Element implementations (NoteOn, NoteOff, Cue, Marker)
// ============================================================================

impl MeiDeserialize for NoteOn {
    fn element_name() -> &'static str {
        "noteOn"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut note_on = NoteOn::default();

        note_on.common.extract_attributes(&mut attrs)?;
        note_on.midi_event.extract_attributes(&mut attrs)?;
        note_on.midi_number.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("noteOn")?;
        }

        Ok(note_on)
    }
}

impl MeiDeserialize for NoteOff {
    fn element_name() -> &'static str {
        "noteOff"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut note_off = NoteOff::default();

        note_off.common.extract_attributes(&mut attrs)?;
        note_off.midi_event.extract_attributes(&mut attrs)?;
        note_off.midi_number.extract_attributes(&mut attrs)?;

        if !is_empty {
            reader.skip_to_end("noteOff")?;
        }

        Ok(note_off)
    }
}

impl MeiDeserialize for Cue {
    fn element_name() -> &'static str {
        "cue"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut cue = Cue::default();

        cue.common.extract_attributes(&mut attrs)?;
        cue.lang.extract_attributes(&mut attrs)?;
        cue.midi_event.extract_attributes(&mut attrs)?;

        if !is_empty {
            // Read text content
            if let Some(text) = reader.read_text_until_end("cue")?
                && !text.is_empty()
            {
                cue.children.push(CueChild::Text(text));
            }
        }

        Ok(cue)
    }
}

impl MeiDeserialize for Marker {
    fn element_name() -> &'static str {
        "marker"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut marker = Marker::default();

        marker.common.extract_attributes(&mut attrs)?;
        marker.lang.extract_attributes(&mut attrs)?;
        marker.midi_event.extract_attributes(&mut attrs)?;

        if !is_empty {
            // Read text content
            if let Some(text) = reader.read_text_until_end("marker")?
                && !text.is_empty()
            {
                marker.children.push(MarkerChild::Text(text));
            }
        }

        Ok(marker)
    }
}

// ============================================================================
// MIDI Meta Element implementations (MetaText, SeqNum, TrkName, Hex)
// ============================================================================

impl MeiDeserialize for MetaText {
    fn element_name() -> &'static str {
        "metaText"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut meta_text = MetaText::default();

        meta_text.common.extract_attributes(&mut attrs)?;
        meta_text.lang.extract_attributes(&mut attrs)?;
        meta_text.midi_event.extract_attributes(&mut attrs)?;

        if !is_empty {
            // Read text content
            if let Some(text) = reader.read_text_until_end("metaText")?
                && !text.is_empty()
            {
                meta_text.children.push(MetaTextChild::Text(text));
            }
        }

        Ok(meta_text)
    }
}

impl MeiDeserialize for SeqNum {
    fn element_name() -> &'static str {
        "seqNum"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut seq_num = SeqNum::default();

        seq_num.common.extract_attributes(&mut attrs)?;
        seq_num.midi_event.extract_attributes(&mut attrs)?;
        // SeqNum has its own `num` attribute (u64 in range 0-65535)
        extract_attr!(attrs, "num", seq_num.num);

        if !is_empty {
            reader.skip_to_end("seqNum")?;
        }

        Ok(seq_num)
    }
}

impl MeiDeserialize for TrkName {
    fn element_name() -> &'static str {
        "trkName"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut trk_name = TrkName::default();

        trk_name.common.extract_attributes(&mut attrs)?;
        trk_name.lang.extract_attributes(&mut attrs)?;
        trk_name.midi_event.extract_attributes(&mut attrs)?;

        if !is_empty {
            // Read text content
            if let Some(text) = reader.read_text_until_end("trkName")?
                && !text.is_empty()
            {
                trk_name.children.push(TrkNameChild::Text(text));
            }
        }

        Ok(trk_name)
    }
}

impl MeiDeserialize for Hex {
    fn element_name() -> &'static str {
        "hex"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut hex = Hex::default();

        hex.common.extract_attributes(&mut attrs)?;
        hex.midi_event.extract_attributes(&mut attrs)?;

        if !is_empty {
            // Read text content (hex data)
            if let Some(text) = reader.read_text_until_end("hex")?
                && !text.is_empty()
            {
                hex.children.push(HexChild::Text(text));
            }
        }

        Ok(hex)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use crate::deserializer::MeiDeserialize;
    use tusk_model::elements::{
        Cc, Chan, ChanPr, Cue, CueChild, Hex, HexChild, InstrGrp, InstrGrpChild, Marker,
        MarkerChild, MetaText, MetaTextChild, Midi, MidiChild, NoteOff, NoteOn, Port, Prog, SeqNum,
        TrkName, TrkNameChild, Vel,
    };

    // ============================================================================
    // Midi tests
    // ============================================================================

    #[test]
    fn midi_deserializes_from_empty_element() {
        let xml = r#"<midi/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert!(midi.common.xml_id.is_none());
        assert!(midi.children.is_empty());
    }

    #[test]
    fn midi_deserializes_xml_id() {
        let xml = r#"<midi xml:id="midi1"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.common.xml_id, Some("midi1".to_string()));
    }

    #[test]
    fn midi_deserializes_staff_attribute() {
        let xml = r#"<midi staff="1"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.midi_log.staff, vec![1]);
    }

    #[test]
    fn midi_deserializes_layer_attribute() {
        let xml = r#"<midi layer="1"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.midi_log.layer, vec![1]);
    }

    #[test]
    fn midi_deserializes_multiple_staff_values() {
        let xml = r#"<midi staff="1 2"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.midi_log.staff, vec![1, 2]);
    }

    #[test]
    fn midi_deserializes_part_attribute() {
        let xml = r#"<midi part="P1"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.midi_log.part, vec!["P1".to_string()]);
    }

    #[test]
    fn midi_handles_unknown_attributes_leniently() {
        let xml = r#"<midi xml:id="midi1" unknown="value"/>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(midi.common.xml_id, Some("midi1".to_string()));
    }

    #[test]
    fn midi_handles_unknown_children_leniently() {
        let xml = r#"<midi xml:id="midi1">
            <unknownElement>ignored</unknownElement>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(midi.common.xml_id, Some("midi1".to_string()));
        assert!(midi.children.is_empty()); // unknown element was skipped
    }

    // ============================================================================
    // InstrGrp tests
    // ============================================================================

    #[test]
    fn instr_grp_deserializes_from_empty_element() {
        let xml = r#"<instrGrp/>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize");

        assert!(instr_grp.common.xml_id.is_none());
        assert!(instr_grp.children.is_empty());
    }

    #[test]
    fn instr_grp_deserializes_xml_id() {
        let xml = r#"<instrGrp xml:id="ig1"/>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
    }

    #[test]
    fn instr_grp_deserializes_with_instr_def_child() {
        let xml = r#"<instrGrp xml:id="ig1">
            <instrDef xml:id="id1" n="1" label="Piano"/>
        </instrGrp>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
        assert_eq!(instr_grp.children.len(), 1);

        match &instr_grp.children[0] {
            InstrGrpChild::InstrDef(instr_def) => {
                assert_eq!(instr_def.basic.xml_id, Some("id1".to_string()));
                assert_eq!(instr_def.labelled.label, Some("Piano".to_string()));
            }
        }
    }

    #[test]
    fn instr_grp_deserializes_with_multiple_instr_def_children() {
        let xml = r#"<instrGrp xml:id="ig1">
            <instrDef xml:id="id1" n="1" label="Violin"/>
            <instrDef xml:id="id2" n="2" label="Viola"/>
            <instrDef xml:id="id3" n="3" label="Cello"/>
        </instrGrp>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
        assert_eq!(instr_grp.children.len(), 3);

        // Check each child
        let labels: Vec<_> = instr_grp
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
    fn instr_grp_handles_unknown_attributes_leniently() {
        let xml = r#"<instrGrp xml:id="ig1" unknown="value"/>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
    }

    #[test]
    fn instr_grp_handles_unknown_children_leniently() {
        let xml = r#"<instrGrp xml:id="ig1">
            <unknownElement>ignored</unknownElement>
            <instrDef xml:id="id1"/>
        </instrGrp>"#;
        let instr_grp = InstrGrp::from_mei_str(xml).expect("should deserialize in lenient mode");

        assert_eq!(instr_grp.common.xml_id, Some("ig1".to_string()));
        assert_eq!(instr_grp.children.len(), 1); // unknown element was skipped
    }

    // ============================================================================
    // Cc tests
    // ============================================================================

    #[test]
    fn cc_deserializes_from_empty_element() {
        let xml = r#"<cc/>"#;
        let cc = Cc::from_mei_str(xml).expect("should deserialize");
        assert!(cc.common.xml_id.is_none());
    }

    #[test]
    fn cc_deserializes_with_attributes() {
        let xml = r#"<cc xml:id="cc1" num="64" val="127" staff="1" tstamp="1"/>"#;
        let cc = Cc::from_mei_str(xml).expect("should deserialize");

        assert_eq!(cc.common.xml_id, Some("cc1".to_string()));
        assert!(cc.midi_number.num.is_some());
        assert!(cc.midi_value.val.is_some());
        assert_eq!(cc.midi_event.staff, vec![1]);
    }

    // ============================================================================
    // Chan tests
    // ============================================================================

    #[test]
    fn chan_deserializes_from_empty_element() {
        let xml = r#"<chan/>"#;
        let chan = Chan::from_mei_str(xml).expect("should deserialize");
        assert!(chan.common.xml_id.is_none());
    }

    #[test]
    fn chan_deserializes_with_attributes() {
        let xml = r#"<chan xml:id="chan1" num="1" staff="1"/>"#;
        let chan = Chan::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chan.common.xml_id, Some("chan1".to_string()));
        assert!(chan.num.is_some());
        assert_eq!(chan.midi_event.staff, vec![1]);
    }

    // ============================================================================
    // ChanPr tests
    // ============================================================================

    #[test]
    fn chan_pr_deserializes_from_empty_element() {
        let xml = r#"<chanPr/>"#;
        let chan_pr = ChanPr::from_mei_str(xml).expect("should deserialize");
        assert!(chan_pr.common.xml_id.is_none());
    }

    #[test]
    fn chan_pr_deserializes_with_attributes() {
        let xml = r#"<chanPr xml:id="cp1" num="64" staff="1"/>"#;
        let chan_pr = ChanPr::from_mei_str(xml).expect("should deserialize");

        assert_eq!(chan_pr.common.xml_id, Some("cp1".to_string()));
        assert!(chan_pr.midi_number.num.is_some());
        assert_eq!(chan_pr.midi_event.staff, vec![1]);
    }

    // ============================================================================
    // Port tests
    // ============================================================================

    #[test]
    fn port_deserializes_from_empty_element() {
        let xml = r#"<port/>"#;
        let port = Port::from_mei_str(xml).expect("should deserialize");
        assert!(port.common.xml_id.is_none());
    }

    #[test]
    fn port_deserializes_with_attributes() {
        let xml = r#"<port xml:id="port1" num="1"/>"#;
        let port = Port::from_mei_str(xml).expect("should deserialize");

        assert_eq!(port.common.xml_id, Some("port1".to_string()));
        assert!(port.midi_number.num.is_some());
    }

    // ============================================================================
    // Prog tests
    // ============================================================================

    #[test]
    fn prog_deserializes_from_empty_element() {
        let xml = r#"<prog/>"#;
        let prog = Prog::from_mei_str(xml).expect("should deserialize");
        assert!(prog.common.xml_id.is_none());
    }

    #[test]
    fn prog_deserializes_with_attributes() {
        let xml = r#"<prog xml:id="prog1" num="1" staff="1"/>"#;
        let prog = Prog::from_mei_str(xml).expect("should deserialize");

        assert_eq!(prog.common.xml_id, Some("prog1".to_string()));
        assert!(prog.midi_number.num.is_some());
        assert_eq!(prog.midi_event.staff, vec![1]);
    }

    // ============================================================================
    // Vel tests
    // ============================================================================

    #[test]
    fn vel_deserializes_from_empty_element() {
        let xml = r#"<vel/>"#;
        let vel = Vel::from_mei_str(xml).expect("should deserialize");
        assert!(vel.common.xml_id.is_none());
    }

    #[test]
    fn vel_deserializes_with_attributes() {
        let xml = r#"<vel xml:id="vel1" num="90" form="noteOn" staff="1"/>"#;
        let vel = Vel::from_mei_str(xml).expect("should deserialize");

        assert_eq!(vel.common.xml_id, Some("vel1".to_string()));
        assert!(vel.midi_number.num.is_some());
        assert_eq!(vel.form, Some("noteOn".to_string()));
        assert_eq!(vel.midi_event.staff, vec![1]);
    }

    // ============================================================================
    // Midi with children tests
    // ============================================================================

    #[test]
    fn midi_deserializes_with_cc_child() {
        let xml = r#"<midi xml:id="midi1">
            <cc xml:id="cc1" num="64" val="127"/>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.common.xml_id, Some("midi1".to_string()));
        assert_eq!(midi.children.len(), 1);

        match &midi.children[0] {
            MidiChild::Cc(cc) => {
                assert_eq!(cc.common.xml_id, Some("cc1".to_string()));
            }
            _ => panic!("Expected Cc child"),
        }
    }

    #[test]
    fn midi_deserializes_with_multiple_control_children() {
        let xml = r#"<midi xml:id="midi1">
            <prog num="1"/>
            <chan num="1"/>
            <cc num="7" val="100"/>
            <vel num="64" form="noteOn"/>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.children.len(), 4);

        // Check order and types
        assert!(matches!(midi.children[0], MidiChild::Prog(_)));
        assert!(matches!(midi.children[1], MidiChild::Chan(_)));
        assert!(matches!(midi.children[2], MidiChild::Cc(_)));
        assert!(matches!(midi.children[3], MidiChild::Vel(_)));
    }

    // ============================================================================
    // NoteOn tests
    // ============================================================================

    #[test]
    fn note_on_deserializes_from_empty_element() {
        let xml = r#"<noteOn/>"#;
        let note_on = NoteOn::from_mei_str(xml).expect("should deserialize");
        assert!(note_on.common.xml_id.is_none());
    }

    #[test]
    fn note_on_deserializes_with_attributes() {
        let xml = r#"<noteOn xml:id="non1" num="60" staff="1" tstamp="1"/>"#;
        let note_on = NoteOn::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note_on.common.xml_id, Some("non1".to_string()));
        assert!(note_on.midi_number.num.is_some());
        assert_eq!(note_on.midi_event.staff, vec![1]);
    }

    // ============================================================================
    // NoteOff tests
    // ============================================================================

    #[test]
    fn note_off_deserializes_from_empty_element() {
        let xml = r#"<noteOff/>"#;
        let note_off = NoteOff::from_mei_str(xml).expect("should deserialize");
        assert!(note_off.common.xml_id.is_none());
    }

    #[test]
    fn note_off_deserializes_with_attributes() {
        let xml = r#"<noteOff xml:id="nof1" num="60" staff="1" tstamp="2"/>"#;
        let note_off = NoteOff::from_mei_str(xml).expect("should deserialize");

        assert_eq!(note_off.common.xml_id, Some("nof1".to_string()));
        assert!(note_off.midi_number.num.is_some());
        assert_eq!(note_off.midi_event.staff, vec![1]);
    }

    // ============================================================================
    // Cue tests
    // ============================================================================

    #[test]
    fn cue_deserializes_from_empty_element() {
        let xml = r#"<cue/>"#;
        let cue = Cue::from_mei_str(xml).expect("should deserialize");
        assert!(cue.common.xml_id.is_none());
        assert!(cue.children.is_empty());
    }

    #[test]
    fn cue_deserializes_with_attributes() {
        let xml = r#"<cue xml:id="cue1" staff="1" tstamp="1"/>"#;
        let cue = Cue::from_mei_str(xml).expect("should deserialize");

        assert_eq!(cue.common.xml_id, Some("cue1".to_string()));
        assert_eq!(cue.midi_event.staff, vec![1]);
    }

    #[test]
    fn cue_deserializes_with_text_content() {
        let xml = r#"<cue xml:id="cue1">Verse 1</cue>"#;
        let cue = Cue::from_mei_str(xml).expect("should deserialize");

        assert_eq!(cue.common.xml_id, Some("cue1".to_string()));
        assert_eq!(cue.children.len(), 1);
        match &cue.children[0] {
            CueChild::Text(text) => assert_eq!(text, "Verse 1"),
        }
    }

    // ============================================================================
    // Marker tests
    // ============================================================================

    #[test]
    fn marker_deserializes_from_empty_element() {
        let xml = r#"<marker/>"#;
        let marker = Marker::from_mei_str(xml).expect("should deserialize");
        assert!(marker.common.xml_id.is_none());
        assert!(marker.children.is_empty());
    }

    #[test]
    fn marker_deserializes_with_attributes() {
        let xml = r#"<marker xml:id="mrk1" staff="1" tstamp="1"/>"#;
        let marker = Marker::from_mei_str(xml).expect("should deserialize");

        assert_eq!(marker.common.xml_id, Some("mrk1".to_string()));
        assert_eq!(marker.midi_event.staff, vec![1]);
    }

    #[test]
    fn marker_deserializes_with_text_content() {
        let xml = r#"<marker xml:id="mrk1">Chorus</marker>"#;
        let marker = Marker::from_mei_str(xml).expect("should deserialize");

        assert_eq!(marker.common.xml_id, Some("mrk1".to_string()));
        assert_eq!(marker.children.len(), 1);
        match &marker.children[0] {
            MarkerChild::Text(text) => assert_eq!(text, "Chorus"),
        }
    }

    // ============================================================================
    // Midi with new event children tests
    // ============================================================================

    #[test]
    fn midi_deserializes_with_note_on_child() {
        let xml = r#"<midi xml:id="midi1">
            <noteOn xml:id="non1" num="60"/>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.children.len(), 1);
        match &midi.children[0] {
            MidiChild::NoteOn(note_on) => {
                assert_eq!(note_on.common.xml_id, Some("non1".to_string()));
            }
            _ => panic!("Expected NoteOn child"),
        }
    }

    #[test]
    fn midi_deserializes_with_note_off_child() {
        let xml = r#"<midi xml:id="midi1">
            <noteOff xml:id="nof1" num="60"/>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.children.len(), 1);
        match &midi.children[0] {
            MidiChild::NoteOff(note_off) => {
                assert_eq!(note_off.common.xml_id, Some("nof1".to_string()));
            }
            _ => panic!("Expected NoteOff child"),
        }
    }

    #[test]
    fn midi_deserializes_with_cue_child() {
        let xml = r#"<midi xml:id="midi1">
            <cue xml:id="cue1">Intro</cue>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.children.len(), 1);
        match &midi.children[0] {
            MidiChild::Cue(cue) => {
                assert_eq!(cue.common.xml_id, Some("cue1".to_string()));
            }
            _ => panic!("Expected Cue child"),
        }
    }

    #[test]
    fn midi_deserializes_with_marker_child() {
        let xml = r#"<midi xml:id="midi1">
            <marker xml:id="mrk1">Bridge</marker>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.children.len(), 1);
        match &midi.children[0] {
            MidiChild::Marker(marker) => {
                assert_eq!(marker.common.xml_id, Some("mrk1".to_string()));
            }
            _ => panic!("Expected Marker child"),
        }
    }

    #[test]
    fn midi_deserializes_with_mixed_event_children() {
        let xml = r#"<midi xml:id="midi1">
            <prog num="1"/>
            <noteOn num="60" tstamp="1"/>
            <noteOff num="60" tstamp="2"/>
            <cue>Verse</cue>
            <marker>Chorus</marker>
        </midi>"#;
        let midi = Midi::from_mei_str(xml).expect("should deserialize");

        assert_eq!(midi.children.len(), 5);
        assert!(matches!(midi.children[0], MidiChild::Prog(_)));
        assert!(matches!(midi.children[1], MidiChild::NoteOn(_)));
        assert!(matches!(midi.children[2], MidiChild::NoteOff(_)));
        assert!(matches!(midi.children[3], MidiChild::Cue(_)));
        assert!(matches!(midi.children[4], MidiChild::Marker(_)));
    }
}

//! Serializer implementations for MIDI-related MEI elements.
//!
//! This module contains implementations for Midi, InstrGrp, and MIDI control elements
//! (Cc, Chan, ChanPr, Port, Prog, Vel).

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttMidiAnl, AttMidiEvent, AttMidiGes, AttMidiLog, AttMidiNumber, AttMidiValue,
};
use tusk_model::elements::{
    Cc, Chan, ChanPr, Cue, CueChild, Hex, HexChild, InstrGrp, InstrGrpChild, Marker, MarkerChild,
    MetaText, MetaTextChild, Midi, MidiChild, NoteOff, NoteOn, Port, Prog, SeqNum, TrkName,
    TrkNameChild, Vel,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Midi attribute class implementations
// ============================================================================

// ============================================================================
// Element implementations
// ============================================================================

impl MeiSerialize for Midi {
    fn element_name(&self) -> &'static str {
        "midi"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_log.collect_attributes());
        attrs.extend(self.midi_ges.collect_attributes());
        attrs.extend(self.midi_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for MidiChild {
    fn element_name(&self) -> &'static str {
        match self {
            MidiChild::TrkName(_) => "trkName",
            MidiChild::Vel(v) => v.element_name(),
            MidiChild::ChanPr(v) => v.element_name(),
            MidiChild::Marker(v) => v.element_name(),
            MidiChild::Prog(v) => v.element_name(),
            MidiChild::Cue(v) => v.element_name(),
            MidiChild::Cc(v) => v.element_name(),
            MidiChild::Chan(v) => v.element_name(),
            MidiChild::MetaText(_) => "metaText",
            MidiChild::NoteOff(v) => v.element_name(),
            MidiChild::Hex(_) => "hex",
            MidiChild::NoteOn(v) => v.element_name(),
            MidiChild::Port(v) => v.element_name(),
            MidiChild::SeqNum(_) => "seqNum",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            MidiChild::Cc(v) => v.collect_all_attributes(),
            MidiChild::Chan(v) => v.collect_all_attributes(),
            MidiChild::ChanPr(v) => v.collect_all_attributes(),
            MidiChild::Port(v) => v.collect_all_attributes(),
            MidiChild::Prog(v) => v.collect_all_attributes(),
            MidiChild::Vel(v) => v.collect_all_attributes(),
            MidiChild::NoteOn(v) => v.collect_all_attributes(),
            MidiChild::NoteOff(v) => v.collect_all_attributes(),
            MidiChild::Cue(v) => v.collect_all_attributes(),
            MidiChild::Marker(v) => v.collect_all_attributes(),
            MidiChild::MetaText(v) => v.collect_all_attributes(),
            MidiChild::SeqNum(v) => v.collect_all_attributes(),
            MidiChild::TrkName(v) => v.collect_all_attributes(),
            MidiChild::Hex(v) => v.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MidiChild::Cue(v) => v.has_children(),
            MidiChild::Marker(v) => v.has_children(),
            MidiChild::MetaText(v) => v.has_children(),
            MidiChild::TrkName(v) => v.has_children(),
            MidiChild::Hex(v) => v.has_children(),
            // Other MIDI elements have no children
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MidiChild::Cue(v) => v.serialize_children(writer),
            MidiChild::Marker(v) => v.serialize_children(writer),
            MidiChild::MetaText(v) => v.serialize_children(writer),
            MidiChild::TrkName(v) => v.serialize_children(writer),
            MidiChild::Hex(v) => v.serialize_children(writer),
            // Other MIDI elements have no children
            _ => Ok(()),
        }
    }
}

// ============================================================================
// MIDI Control Element implementations
// ============================================================================

impl MeiSerialize for Cc {
    fn element_name(&self) -> &'static str {
        "cc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs.extend(self.midi_number.collect_attributes());
        attrs.extend(self.midi_value.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Chan {
    fn element_name(&self) -> &'static str {
        "chan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        // Chan has its own `num` attribute (DataMidichannel type)
        if let Some(ref v) = self.num {
            attrs.push(("num", v.to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for ChanPr {
    fn element_name(&self) -> &'static str {
        "chanPr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs.extend(self.midi_number.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Port {
    fn element_name(&self) -> &'static str {
        "port"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs.extend(self.midi_number.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Prog {
    fn element_name(&self) -> &'static str {
        "prog"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs.extend(self.midi_number.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Vel {
    fn element_name(&self) -> &'static str {
        "vel"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs.extend(self.midi_number.collect_attributes());
        // Vel has its own `form` attribute
        if let Some(ref v) = self.form {
            attrs.push(("form", v.clone()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// MIDI Event Element implementations (NoteOn, NoteOff, Cue, Marker)
// ============================================================================

impl MeiSerialize for NoteOn {
    fn element_name(&self) -> &'static str {
        "noteOn"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs.extend(self.midi_number.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for NoteOff {
    fn element_name(&self) -> &'static str {
        "noteOff"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs.extend(self.midi_number.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Cue {
    fn element_name(&self) -> &'static str {
        "cue"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                CueChild::Text(text) => writer.write_text(text)?,
            }
        }
        Ok(())
    }
}

impl MeiSerialize for Marker {
    fn element_name(&self) -> &'static str {
        "marker"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                MarkerChild::Text(text) => writer.write_text(text)?,
            }
        }
        Ok(())
    }
}

// ============================================================================
// MIDI Meta Element implementations (MetaText, SeqNum, TrkName, Hex)
// ============================================================================

impl MeiSerialize for MetaText {
    fn element_name(&self) -> &'static str {
        "metaText"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                MetaTextChild::Text(text) => writer.write_text(text)?,
            }
        }
        Ok(())
    }
}

impl MeiSerialize for SeqNum {
    fn element_name(&self) -> &'static str {
        "seqNum"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        // SeqNum has its own `num` attribute
        if let Some(ref v) = self.num {
            attrs.push(("num", v.to_string()));
        }
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for TrkName {
    fn element_name(&self) -> &'static str {
        "trkName"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                TrkNameChild::Text(text) => writer.write_text(text)?,
            }
        }
        Ok(())
    }
}

impl MeiSerialize for Hex {
    fn element_name(&self) -> &'static str {
        "hex"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.midi_event.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                HexChild::Text(text) => writer.write_text(text)?,
            }
        }
        Ok(())
    }
}

impl MeiSerialize for InstrGrp {
    fn element_name(&self) -> &'static str {
        "instrGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for InstrGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            InstrGrpChild::InstrDef(_) => "instrDef",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            InstrGrpChild::InstrDef(instr_def) => instr_def.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            InstrGrpChild::InstrDef(_) => false, // InstrDef has no children
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            InstrGrpChild::InstrDef(instr_def) => instr_def.serialize_children(writer),
        }
    }
}

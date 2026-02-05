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
    Cc, Chan, ChanPr, InstrGrp, InstrGrpChild, Midi, MidiChild, Port, Prog, Vel,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Midi attribute class implementations
// ============================================================================

impl CollectAttributes for AttMidiLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        if let Some(v) = serialize_vec_serde(&self.layer) {
            attrs.push(("layer", v));
        }
        if !self.part.is_empty() {
            attrs.push(("part", self.part.join(" ")));
        }
        if !self.partstaff.is_empty() {
            attrs.push(("partstaff", self.partstaff.join(" ")));
        }
        if let Some(v) = serialize_vec_serde(&self.staff) {
            attrs.push(("staff", v));
        }

        attrs
    }
}

impl CollectAttributes for AttMidiGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMidiGes is empty - no attributes to collect
        Vec::new()
    }
}

impl CollectAttributes for AttMidiAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttMidiAnl is empty - no attributes to collect
        Vec::new()
    }
}

impl CollectAttributes for AttMidiEvent {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "layer", vec self.layer);
        if !self.part.is_empty() {
            attrs.push(("part", self.part.join(" ")));
        }
        if !self.partstaff.is_empty() {
            attrs.push(("partstaff", self.partstaff.join(" ")));
        }
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        attrs
    }
}

impl CollectAttributes for AttMidiNumber {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "num", self.num);
        attrs
    }
}

impl CollectAttributes for AttMidiValue {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "val", self.val);
        attrs
    }
}

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
            MidiChild::Marker(_) => "marker",
            MidiChild::Prog(v) => v.element_name(),
            MidiChild::Cue(_) => "cue",
            MidiChild::Cc(v) => v.element_name(),
            MidiChild::Chan(v) => v.element_name(),
            MidiChild::MetaText(_) => "metaText",
            MidiChild::NoteOff(_) => "noteOff",
            MidiChild::Hex(_) => "hex",
            MidiChild::NoteOn(_) => "noteOn",
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
            // Other MIDI child elements not yet implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        // MIDI control elements have no children
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // MIDI control elements have no children
        Ok(())
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

//! Serializer implementations for MIDI-related MEI elements.
//!
//! This module contains implementations for Midi and InstrGrp elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{AttMidiAnl, AttMidiGes, AttMidiLog};
use tusk_model::elements::{InstrGrp, InstrGrpChild, Midi, MidiChild};

use super::serialize_vec_serde;

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
            MidiChild::Vel(_) => "vel",
            MidiChild::ChanPr(_) => "chanPr",
            MidiChild::Marker(_) => "marker",
            MidiChild::Prog(_) => "prog",
            MidiChild::Cue(_) => "cue",
            MidiChild::Cc(_) => "cc",
            MidiChild::Chan(_) => "chan",
            MidiChild::MetaText(_) => "metaText",
            MidiChild::NoteOff(_) => "noteOff",
            MidiChild::Hex(_) => "hex",
            MidiChild::NoteOn(_) => "noteOn",
            MidiChild::Port(_) => "port",
            MidiChild::SeqNum(_) => "seqNum",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        // Child elements not yet implemented - return empty for now
        // Will be implemented when MIDI child elements are added in Phase 7
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Child elements not yet implemented
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

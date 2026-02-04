//! Serializer implementations for MEI definition elements.
//!
//! This module contains implementations for ScoreDef, StaffDef, LayerDef, StaffGrp,
//! and their related attribute classes and child elements (KeySig, MeterSig).

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttScoreDefAnl, AttScoreDefGes, AttScoreDefLog, AttScoreDefVis, AttStaffGrpAnl, AttStaffGrpGes,
    AttStaffGrpLog, AttStaffGrpVis,
};
use tusk_model::elements::{ScoreDef, ScoreDefChild};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// ScoreDef attribute class implementations
// ============================================================================

impl CollectAttributes for AttScoreDefLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // Clef attributes
        if let Some(v) = &self.clef_shape {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.shape", s));
            }
        }
        if let Some(v) = &self.clef_line {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.line", s));
            }
        }
        if let Some(v) = &self.clef_dis {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.dis", s));
            }
        }
        if let Some(v) = &self.clef_dis_place {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.dis.place", s));
            }
        }

        // Duration defaults
        if let Some(v) = &self.dur_default {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("dur.default", s));
            }
        }
        if let Some(v) = &self.num_default {
            attrs.push(("num.default", v.to_string()));
        }
        if let Some(v) = &self.numbase_default {
            attrs.push(("numbase.default", v.to_string()));
        }

        // Key signature
        if let Some(s) = serialize_vec_serde(&self.keysig) {
            attrs.push(("keysig", s));
        }

        // Meter attributes
        if let Some(v) = &self.meter_count {
            attrs.push(("meter.count", v.clone()));
        }
        if let Some(v) = &self.meter_unit {
            attrs.push(("meter.unit", v.to_string()));
        }
        if let Some(v) = &self.meter_sym {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("meter.sym", s));
            }
        }

        // Octave default
        if let Some(v) = &self.oct_default {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("oct.default", s));
            }
        }

        // Transposition
        if let Some(v) = &self.trans_diat {
            attrs.push(("trans.diat", v.to_string()));
        }
        if let Some(v) = &self.trans_semi {
            attrs.push(("trans.semi", v.to_string()));
        }

        // Beam attributes
        if let Some(v) = &self.beam_group {
            attrs.push(("beam.group", v.clone()));
        }
        if let Some(v) = &self.beam_rests {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("beam.rests", s));
            }
        }

        // Mensural attributes
        if let Some(v) = &self.modusmaior {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("modusmaior", s));
            }
        }
        if let Some(v) = &self.modusminor {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("modusminor", s));
            }
        }
        if let Some(v) = &self.prolatio {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("prolatio", s));
            }
        }
        if let Some(v) = &self.tempus {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("tempus", s));
            }
        }
        if let Some(v) = &self.divisio {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("divisio", s));
            }
        }
        if let Some(v) = &self.proport_num {
            attrs.push(("proport.num", v.to_string()));
        }
        if let Some(v) = &self.proport_numbase {
            attrs.push(("proport.numbase", v.to_string()));
        }

        attrs
    }
}

impl CollectAttributes for AttScoreDefGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // MIDI attributes
        if let Some(v) = &self.midi_channel {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.channel", s));
            }
        }
        if let Some(v) = &self.midi_duty {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.duty", s));
            }
        }
        if let Some(v) = &self.midi_port {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.port", s));
            }
        }
        if let Some(v) = &self.midi_track {
            attrs.push(("midi.track", v.to_string()));
        }
        if let Some(v) = &self.ppq {
            attrs.push(("ppq", v.to_string()));
        }
        if let Some(v) = &self.midi_bpm {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.bpm", s));
            }
        }
        if let Some(v) = &self.midi_mspb {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.mspb", s));
            }
        }

        // Tuning attributes
        if let Some(v) = &self.tune_hz {
            attrs.push(("tune.Hz", v.to_string()));
        }
        if let Some(v) = &self.tune_pname {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("tune.pname", s));
            }
        }
        if let Some(v) = &self.tune_temper {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("tune.temper", s));
            }
        }

        // Metronome attributes
        if let Some(v) = &self.mm {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mm", s));
            }
        }
        if let Some(v) = &self.mm_unit {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mm.unit", s));
            }
        }
        if let Some(v) = &self.mm_dots {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mm.dots", s));
            }
        }

        attrs
    }
}

impl CollectAttributes for AttScoreDefVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // Bar attributes
        if let Some(v) = &self.bar_len {
            attrs.push(("bar.len", v.to_string()));
        }
        if let Some(v) = &self.bar_method {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("bar.method", s));
            }
        }
        if let Some(v) = &self.bar_place {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("bar.place", s));
            }
        }

        // Clef visual
        if let Some(v) = &self.clef_color {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.color", s));
            }
        }
        if let Some(v) = &self.clef_visible {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.visible", s));
            }
        }

        // Meter visual attributes
        if let Some(v) = &self.meter_form {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("meter.form", s));
            }
        }
        if let Some(v) = &self.meter_showchange {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("meter.showchange", s));
            }
        }
        if let Some(v) = &self.meter_visible {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("meter.visible", s));
            }
        }

        // Key signature visual
        if let Some(v) = &self.keysig_cancelaccid {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("keysig.cancelaccid", s));
            }
        }
        if let Some(v) = &self.keysig_visible {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("keysig.visible", s));
            }
        }

        // Page attributes (commonly used)
        if let Some(v) = &self.page_height {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("page.height", s));
            }
        }
        if let Some(v) = &self.page_width {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("page.width", s));
            }
        }

        // Spacing attributes (commonly used)
        if let Some(v) = &self.spacing_packexp {
            attrs.push(("spacing.packexp", v.to_string()));
        }
        if let Some(v) = &self.spacing_packfact {
            attrs.push(("spacing.packfact", v.to_string()));
        }
        if let Some(v) = &self.spacing_staff {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("spacing.staff", s));
            }
        }
        if let Some(v) = &self.spacing_system {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("spacing.system", s));
            }
        }

        // Note: For brevity, not all visual attributes are serialized here.
        // Additional attributes can be added as needed.

        attrs
    }
}

impl CollectAttributes for AttScoreDefAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // Key analytical attributes
        if let Some(v) = &self.key_accid {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("key.accid", s));
            }
        }
        if let Some(v) = &self.key_mode {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("key.mode", s));
            }
        }
        if let Some(v) = &self.key_pname {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("key.pname", s));
            }
        }

        attrs
    }
}

// ============================================================================
// StaffGrp attribute class implementations
// ============================================================================

impl CollectAttributes for AttStaffGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttStaffGrpLog is empty - no attributes to collect
        Vec::new()
    }
}

impl CollectAttributes for AttStaffGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "instr", self.instr);
        attrs
    }
}

impl CollectAttributes for AttStaffGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // Bar line attributes
        if let Some(v) = &self.bar_len {
            attrs.push(("bar.len", v.to_string()));
        }
        push_attr!(attrs, "bar.method", self.bar_method);
        push_attr!(attrs, "bar.place", self.bar_place);
        push_attr!(attrs, "bar.thru", self.bar_thru);

        // Grouping symbol
        push_attr!(attrs, "symbol", self.symbol);

        // Visibility
        push_attr!(attrs, "visible", self.visible);

        attrs
    }
}

impl CollectAttributes for AttStaffGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        // AttStaffGrpAnl is empty - no attributes to collect
        Vec::new()
    }
}

// ============================================================================
// Element implementations
// ============================================================================

impl MeiSerialize for ScoreDef {
    fn element_name(&self) -> &'static str {
        "scoreDef"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.score_def_log.collect_attributes());
        attrs.extend(self.score_def_ges.collect_attributes());
        attrs.extend(self.score_def_vis.collect_attributes());
        attrs.extend(self.score_def_anl.collect_attributes());
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

impl MeiSerialize for ScoreDefChild {
    fn element_name(&self) -> &'static str {
        match self {
            ScoreDefChild::GrpSym(_) => "grpSym",
            ScoreDefChild::InstrGrp(_) => "instrGrp",
            ScoreDefChild::Ambitus(_) => "ambitus",
            ScoreDefChild::PgFoot(_) => "pgFoot",
            ScoreDefChild::SymbolTable(_) => "symbolTable",
            ScoreDefChild::KeySig(_) => "keySig",
            ScoreDefChild::PgHead(_) => "pgHead",
            ScoreDefChild::MeterSig(_) => "meterSig",
            ScoreDefChild::MeterSigGrp(_) => "meterSigGrp",
            ScoreDefChild::StaffGrp(_) => "staffGrp",
            ScoreDefChild::ChordTable(_) => "chordTable",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ScoreDefChild::StaffGrp(sg) => sg.collect_all_attributes(),
            ScoreDefChild::KeySig(ks) => ks.collect_all_attributes(),
            ScoreDefChild::MeterSig(ms) => ms.collect_all_attributes(),
            // Other children - return common attributes only for now
            ScoreDefChild::GrpSym(_)
            | ScoreDefChild::InstrGrp(_)
            | ScoreDefChild::Ambitus(_)
            | ScoreDefChild::PgFoot(_)
            | ScoreDefChild::SymbolTable(_)
            | ScoreDefChild::PgHead(_)
            | ScoreDefChild::MeterSigGrp(_)
            | ScoreDefChild::ChordTable(_) => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ScoreDefChild::StaffGrp(sg) => !sg.children.is_empty(),
            // Most scoreDef children may have children
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ScoreDefChild::StaffGrp(sg) => sg.serialize_children(writer),
            // Other children not yet fully implemented
            _ => Ok(()),
        }
    }
}

// StaffGrp serialization
impl MeiSerialize for tusk_model::elements::StaffGrp {
    fn element_name(&self) -> &'static str {
        "staffGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.staff_grp_log.collect_attributes());
        attrs.extend(self.staff_grp_ges.collect_attributes());
        attrs.extend(self.staff_grp_vis.collect_attributes());
        attrs.extend(self.staff_grp_anl.collect_attributes());
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

impl MeiSerialize for tusk_model::elements::StaffGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            tusk_model::elements::StaffGrpChild::Label(_) => "label",
            tusk_model::elements::StaffGrpChild::StaffDef(_) => "staffDef",
            tusk_model::elements::StaffGrpChild::InstrDef(_) => "instrDef",
            tusk_model::elements::StaffGrpChild::StaffGrp(_) => "staffGrp",
            tusk_model::elements::StaffGrpChild::LabelAbbr(_) => "labelAbbr",
            tusk_model::elements::StaffGrpChild::GrpSym(_) => "grpSym",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            tusk_model::elements::StaffGrpChild::StaffDef(sd) => sd.collect_all_attributes(),
            tusk_model::elements::StaffGrpChild::StaffGrp(sg) => sg.collect_all_attributes(),
            // Other children not yet fully implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            tusk_model::elements::StaffGrpChild::StaffGrp(sg) => !sg.children.is_empty(),
            tusk_model::elements::StaffGrpChild::StaffDef(sd) => !sd.children.is_empty(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            tusk_model::elements::StaffGrpChild::StaffGrp(sg) => sg.serialize_children(writer),
            tusk_model::elements::StaffGrpChild::StaffDef(sd) => sd.serialize_children(writer),
            _ => Ok(()),
        }
    }
}

// StaffDef serialization
impl MeiSerialize for tusk_model::elements::StaffDef {
    fn element_name(&self) -> &'static str {
        "staffDef"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // Basic attributes (xml:id)
        if let Some(id) = &self.basic.xml_id {
            attrs.push(("xml:id", id.clone()));
        }

        // Labelled attributes
        if let Some(label) = &self.labelled.label {
            attrs.push(("label", label.clone()));
        }

        // N integer
        if let Some(n) = &self.n_integer.n {
            attrs.push(("n", n.to_string()));
        }

        // Lines (visual)
        if let Some(lines) = &self.staff_def_log.lines {
            attrs.push(("lines", lines.to_string()));
        }

        // Clef attributes (logical)
        if let Some(v) = &self.staff_def_log.clef_shape {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.shape", s));
            }
        }
        if let Some(v) = &self.staff_def_log.clef_line {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.line", s));
            }
        }
        if let Some(v) = &self.staff_def_log.clef_dis {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.dis", s));
            }
        }
        if let Some(v) = &self.staff_def_log.clef_dis_place {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("clef.dis.place", s));
            }
        }

        // Key signature
        if !self.staff_def_log.keysig.is_empty() {
            // Join multiple keysig values with space (rare but possible)
            let keysig_str = self
                .staff_def_log
                .keysig
                .iter()
                .filter_map(|k| to_attr_string(k))
                .collect::<Vec<_>>()
                .join(" ");
            if !keysig_str.is_empty() {
                attrs.push(("keysig", keysig_str));
            }
        }

        // Time signature (meter)
        if let Some(ref count) = self.staff_def_log.meter_count {
            attrs.push(("meter.count", count.clone()));
        }
        if let Some(unit) = self.staff_def_log.meter_unit {
            attrs.push(("meter.unit", unit.to_string()));
        }
        if let Some(v) = &self.staff_def_log.meter_sym {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("meter.sym", s));
            }
        }

        // Notation type
        if let Some(v) = &self.staff_def_log.notationtype {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("notationtype", s));
            }
        }

        // PPQ (gestural - pulses per quarter note)
        if let Some(ppq) = self.staff_def_ges.ppq {
            attrs.push(("ppq", ppq.to_string()));
        }

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

impl MeiSerialize for tusk_model::elements::StaffDefChild {
    fn element_name(&self) -> &'static str {
        match self {
            tusk_model::elements::StaffDefChild::Label(_) => "label",
            tusk_model::elements::StaffDefChild::LabelAbbr(_) => "labelAbbr",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            tusk_model::elements::StaffDefChild::Label(label) => {
                for child in &label.children {
                    match child {
                        tusk_model::elements::LabelChild::Text(text) => {
                            writer.write_text(text)?;
                        }
                        _ => {}
                    }
                }
            }
            tusk_model::elements::StaffDefChild::LabelAbbr(abbr) => {
                for child in &abbr.children {
                    match child {
                        tusk_model::elements::LabelAbbrChild::Text(text) => {
                            writer.write_text(text)?;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// KeySig serialization
impl MeiSerialize for tusk_model::elements::KeySig {
    fn element_name(&self) -> &'static str {
        "keySig"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        // KeySig-specific attributes not yet implemented
        attrs
    }

    fn has_children(&self) -> bool {
        false // KeySig children not yet implemented
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// MeterSig serialization
impl MeiSerialize for tusk_model::elements::MeterSig {
    fn element_name(&self) -> &'static str {
        "meterSig"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        // MeterSig-specific attributes not yet implemented
        attrs
    }

    fn has_children(&self) -> bool {
        false // MeterSig has no children
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// LayerDef serialization
impl MeiSerialize for tusk_model::elements::LayerDef {
    fn element_name(&self) -> &'static str {
        "layerDef"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // Basic attributes (xml:id)
        if let Some(id) = &self.basic.xml_id {
            attrs.push(("xml:id", id.clone()));
        }

        // Labelled attributes
        if let Some(label) = &self.labelled.label {
            attrs.push(("label", label.clone()));
        }

        // N integer
        if let Some(n) = &self.n_integer.n {
            attrs.push(("n", n.to_string()));
        }

        // LayerDefLog attributes
        if let Some(v) = &self.layer_def_log.dur_default {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("dur.default", s));
            }
        }
        if let Some(v) = &self.layer_def_log.num_default {
            attrs.push(("num.default", v.to_string()));
        }
        if let Some(v) = &self.layer_def_log.numbase_default {
            attrs.push(("numbase.default", v.to_string()));
        }
        if let Some(v) = &self.layer_def_log.beam_group {
            attrs.push(("beam.group", v.clone()));
        }
        if let Some(v) = &self.layer_def_log.beam_rests {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("beam.rests", s));
            }
        }
        if let Some(v) = &self.layer_def_log.oct_default {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("oct.default", s));
            }
        }
        if let Some(v) = &self.layer_def_log.trans_diat {
            attrs.push(("trans.diat", v.to_string()));
        }
        if let Some(v) = &self.layer_def_log.trans_semi {
            attrs.push(("trans.semi", v.to_string()));
        }

        // LayerDefGes attributes
        if let Some(v) = &self.layer_def_ges.instr {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("instr", s));
            }
        }
        if let Some(v) = &self.layer_def_ges.tune_hz {
            attrs.push(("tune.Hz", v.to_string()));
        }
        if let Some(v) = &self.layer_def_ges.tune_pname {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("tune.pname", s));
            }
        }
        if let Some(v) = &self.layer_def_ges.tune_temper {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("tune.temper", s));
            }
        }

        // LayerDefVis attributes
        if let Some(v) = &self.layer_def_vis.beam_color {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("beam.color", s));
            }
        }
        if let Some(v) = &self.layer_def_vis.beam_rend {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("beam.rend", s));
            }
        }
        if let Some(v) = &self.layer_def_vis.beam_slope {
            attrs.push(("beam.slope", v.to_string()));
        }
        if let Some(v) = &self.layer_def_vis.text_fam {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.fam", s));
            }
        }
        if let Some(v) = &self.layer_def_vis.text_name {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.name", s));
            }
        }
        if let Some(v) = &self.layer_def_vis.text_size {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.size", s));
            }
        }
        if let Some(v) = &self.layer_def_vis.text_style {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.style", s));
            }
        }
        if let Some(v) = &self.layer_def_vis.text_weight {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.weight", s));
            }
        }
        if let Some(v) = &self.layer_def_vis.visible {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("visible", s));
            }
        }

        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // LayerDef children serialization not yet fully implemented
        Ok(())
    }
}

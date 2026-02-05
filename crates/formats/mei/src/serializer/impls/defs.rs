//! Serializer implementations for MEI definition elements.
//!
//! This module contains implementations for ScoreDef, StaffDef, LayerDef, StaffGrp,
//! PgHead, PgFoot, and their related attribute classes and child elements (KeySig, MeterSig).

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttFormework, AttScoreDefAnl, AttScoreDefGes, AttScoreDefLog, AttScoreDefVis, AttStaffGrpAnl,
    AttStaffGrpGes, AttStaffGrpLog, AttStaffGrpVis,
};
use tusk_model::elements::{PgFoot, PgFootChild, PgHead, PgHeadChild, ScoreDef, ScoreDefChild};

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

        // Music font attributes
        if let Some(v) = &self.music_name {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("music.name", s));
            }
        }
        if let Some(v) = &self.music_size {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("music.size", s));
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

        // System attributes
        if let Some(v) = &self.system_leftline {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("system.leftline", s));
            }
        }
        if let Some(v) = &self.system_leftmar {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("system.leftmar", s));
            }
        }
        if let Some(v) = &self.system_rightmar {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("system.rightmar", s));
            }
        }
        if let Some(v) = &self.system_topmar {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("system.topmar", s));
            }
        }

        // Distance attributes
        if let Some(v) = &self.dir_dist {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("dir.dist", s));
            }
        }
        if let Some(v) = &self.dynam_dist {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("dynam.dist", s));
            }
        }
        if let Some(v) = &self.harm_dist {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("harm.dist", s));
            }
        }
        if let Some(v) = &self.reh_dist {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("reh.dist", s));
            }
        }
        if let Some(v) = &self.tempo_dist {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("tempo.dist", s));
            }
        }

        // Ending rendering
        if let Some(v) = &self.ending_rend {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("ending.rend", s));
            }
        }

        // Lyric attributes
        if let Some(v) = &self.lyric_align {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("lyric.align", s));
            }
        }
        if let Some(v) = &self.lyric_fam {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("lyric.fam", s));
            }
        }
        if let Some(v) = &self.lyric_name {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("lyric.name", s));
            }
        }
        if let Some(v) = &self.lyric_size {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("lyric.size", s));
            }
        }
        if let Some(v) = &self.lyric_style {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("lyric.style", s));
            }
        }
        if let Some(v) = &self.lyric_weight {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("lyric.weight", s));
            }
        }

        // Measure number visibility
        if let Some(v) = &self.mnum_visible {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mnum.visible", s));
            }
        }

        // Multi-measure number
        if let Some(v) = &self.multi_number {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("multi.number", s));
            }
        }

        // One-line staff placement
        if let Some(v) = &self.ontheline {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("ontheline", s));
            }
        }

        // Optimize (hide empty staves)
        if let Some(v) = &self.optimize {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("optimize", s));
            }
        }

        // Page margins
        if let Some(v) = &self.page_topmar {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("page.topmar", s));
            }
        }
        if let Some(v) = &self.page_botmar {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("page.botmar", s));
            }
        }
        if let Some(v) = &self.page_leftmar {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("page.leftmar", s));
            }
        }
        if let Some(v) = &self.page_rightmar {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("page.rightmar", s));
            }
        }
        if let Some(v) = &self.page_panels {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("page.panels", s));
            }
        }
        if let Some(v) = &self.page_scale {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("page.scale", s));
            }
        }

        // Vertical order attributes
        if !self.aboveorder.is_empty() {
            let s = self
                .aboveorder
                .iter()
                .filter_map(|v| to_attr_string(v))
                .collect::<Vec<_>>()
                .join(" ");
            if !s.is_empty() {
                attrs.push(("aboveorder", s));
            }
        }
        if !self.beloworder.is_empty() {
            let s = self
                .beloworder
                .iter()
                .filter_map(|v| to_attr_string(v))
                .collect::<Vec<_>>()
                .join(" ");
            if !s.is_empty() {
                attrs.push(("beloworder", s));
            }
        }
        if !self.betweenorder.is_empty() {
            let s = self
                .betweenorder
                .iter()
                .filter_map(|v| to_attr_string(v))
                .collect::<Vec<_>>()
                .join(" ");
            if !s.is_empty() {
                attrs.push(("betweenorder", s));
            }
        }

        // Text font attributes
        if let Some(v) = &self.text_fam {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.fam", s));
            }
        }
        if let Some(v) = &self.text_name {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.name", s));
            }
        }
        if let Some(v) = &self.text_size {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.size", s));
            }
        }
        if let Some(v) = &self.text_style {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.style", s));
            }
        }
        if let Some(v) = &self.text_weight {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("text.weight", s));
            }
        }

        // Beam attributes
        if let Some(v) = &self.beam_color {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("beam.color", s));
            }
        }
        if let Some(v) = &self.beam_rend {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("beam.rend", s));
            }
        }
        if let Some(v) = &self.beam_slope {
            attrs.push(("beam.slope", v.to_string()));
        }

        // Grid and pedal
        if let Some(v) = &self.grid_show {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("grid.show", s));
            }
        }
        if let Some(v) = &self.pedal_style {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("pedal.style", s));
            }
        }

        // Rehearsal mark enclosure
        if let Some(v) = &self.reh_enclose {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("reh.enclose", s));
            }
        }

        // Slur/tie attributes
        if let Some(v) = &self.slur_lform {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("slur.lform", s));
            }
        }
        if let Some(v) = &self.slur_lwidth {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("slur.lwidth", s));
            }
        }
        if let Some(v) = &self.tie_lform {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("tie.lform", s));
            }
        }
        if let Some(v) = &self.tie_lwidth {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("tie.lwidth", s));
            }
        }

        // Mensural notation attributes
        if let Some(v) = &self.mensur_color {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mensur.color", s));
            }
        }
        if let Some(v) = &self.mensur_dot {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mensur.dot", s));
            }
        }
        if let Some(v) = &self.mensur_form {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mensur.form", s));
            }
        }
        if let Some(v) = &self.mensur_loc {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mensur.loc", s));
            }
        }
        if let Some(v) = &self.mensur_orient {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mensur.orient", s));
            }
        }
        if let Some(v) = &self.mensur_sign {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mensur.sign", s));
            }
        }
        if let Some(v) = &self.mensur_size {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("mensur.size", s));
            }
        }
        if let Some(v) = &self.mensur_slash {
            attrs.push(("mensur.slash", v.to_string()));
        }

        // Virtual unit height
        if let Some(v) = &self.vu_height {
            attrs.push(("vu.height", v.clone()));
        }

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
            ScoreDefChild::PgHead(pg) => pg.collect_all_attributes(),
            ScoreDefChild::PgFoot(pg) => pg.collect_all_attributes(),
            // Other children - return common attributes only for now
            ScoreDefChild::GrpSym(_)
            | ScoreDefChild::InstrGrp(_)
            | ScoreDefChild::Ambitus(_)
            | ScoreDefChild::SymbolTable(_)
            | ScoreDefChild::MeterSigGrp(_)
            | ScoreDefChild::ChordTable(_) => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ScoreDefChild::StaffGrp(sg) => !sg.children.is_empty(),
            ScoreDefChild::PgHead(pg) => !pg.children.is_empty(),
            ScoreDefChild::PgFoot(pg) => !pg.children.is_empty(),
            // Most scoreDef children may have children
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ScoreDefChild::StaffGrp(sg) => sg.serialize_children(writer),
            ScoreDefChild::PgHead(pg) => pg.serialize_children(writer),
            ScoreDefChild::PgFoot(pg) => pg.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ScoreDefChild::{}::serialize_children",
                other.element_name()
            ))),
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
            tusk_model::elements::StaffGrpChild::Label(label) => label.collect_all_attributes(),
            tusk_model::elements::StaffGrpChild::LabelAbbr(label_abbr) => {
                label_abbr.collect_all_attributes()
            }
            tusk_model::elements::StaffGrpChild::InstrDef(instr_def) => {
                instr_def.collect_all_attributes()
            }
            // Other children not yet fully implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            tusk_model::elements::StaffGrpChild::StaffGrp(sg) => !sg.children.is_empty(),
            tusk_model::elements::StaffGrpChild::StaffDef(sd) => !sd.children.is_empty(),
            tusk_model::elements::StaffGrpChild::Label(label) => !label.children.is_empty(),
            tusk_model::elements::StaffGrpChild::LabelAbbr(label_abbr) => {
                !label_abbr.children.is_empty()
            }
            tusk_model::elements::StaffGrpChild::InstrDef(_) => false, // InstrDef has no children
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            tusk_model::elements::StaffGrpChild::StaffGrp(sg) => sg.serialize_children(writer),
            tusk_model::elements::StaffGrpChild::StaffDef(sd) => sd.serialize_children(writer),
            tusk_model::elements::StaffGrpChild::Label(label) => label.serialize_children(writer),
            tusk_model::elements::StaffGrpChild::LabelAbbr(label_abbr) => {
                label_abbr.serialize_children(writer)
            }
            tusk_model::elements::StaffGrpChild::InstrDef(_) => Ok(()), // InstrDef has no children
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "StaffGrpChild::{}::serialize_children",
                other.element_name()
            ))),
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

        // Transposition
        if let Some(v) = &self.staff_def_log.trans_diat {
            attrs.push(("trans.diat", v.to_string()));
        }
        if let Some(v) = &self.staff_def_log.trans_semi {
            attrs.push(("trans.semi", v.to_string()));
        }

        // PPQ (gestural - pulses per quarter note)
        if let Some(ppq) = self.staff_def_ges.ppq {
            attrs.push(("ppq", ppq.to_string()));
        }

        // Spacing (visual)
        if let Some(ref spacing) = self.staff_def_vis.spacing {
            if let Some(s) = to_attr_string(spacing) {
                attrs.push(("spacing", s));
            }
        }

        // Lines visible (visual)
        if let Some(ref lines_visible) = self.staff_def_vis.lines_visible {
            if let Some(s) = to_attr_string(lines_visible) {
                attrs.push(("lines.visible", s));
            }
        }

        // Clef visible (visual)
        if let Some(ref clef_visible) = self.staff_def_vis.clef_visible {
            if let Some(s) = to_attr_string(clef_visible) {
                attrs.push(("clef.visible", s));
            }
        }

        // Visibility (from AttStaffDefVis)
        if let Some(ref visible) = self.staff_def_vis.visible {
            if let Some(s) = to_attr_string(visible) {
                attrs.push(("visible", s));
            }
        }

        // Analytical attributes (key info)
        if let Some(v) = &self.staff_def_anl.key_accid {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("key.accid", s));
            }
        }
        if let Some(v) = &self.staff_def_anl.key_mode {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("key.mode", s));
            }
        }
        if let Some(v) = &self.staff_def_anl.key_pname {
            attrs.push(("key.pname", v.to_string()));
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
        use tusk_model::elements::StaffDefChild::*;
        match self {
            ClefGrp(_) => "clefGrp",
            LabelAbbr(_) => "labelAbbr",
            InstrDef(_) => "instrDef",
            Mensur(_) => "mensur",
            MeterSig(_) => "meterSig",
            Proport(_) => "proport",
            Label(_) => "label",
            MeterSigGrp(_) => "meterSigGrp",
            Ambitus(_) => "ambitus",
            Tuning(_) => "tuning",
            KeySig(_) => "keySig",
            LayerDef(_) => "layerDef",
            Clef(_) => "clef",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        use tusk_model::elements::StaffDefChild::*;
        match self {
            InstrDef(instr) => instr.collect_all_attributes(),
            Label(label) => label.collect_all_attributes(),
            KeySig(ks) => ks.collect_all_attributes(),
            MeterSig(ms) => ms.collect_all_attributes(),
            LayerDef(ld) => ld.collect_all_attributes(),
            // Types without MeiSerialize impl yet - return empty for now
            LabelAbbr(_) | Clef(_) | ClefGrp(_) | Mensur(_) | Proport(_) | MeterSigGrp(_)
            | Ambitus(_) | Tuning(_) => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        use tusk_model::elements::StaffDefChild::*;
        match self {
            Label(label) => !label.children.is_empty(),
            LabelAbbr(abbr) => !abbr.children.is_empty(),
            LayerDef(ld) => !ld.children.is_empty(),
            // InstrDef has no children
            InstrDef(_) => false,
            // Other elements - may have children
            ClefGrp(_) | Mensur(_) | MeterSig(_) | Proport(_) | MeterSigGrp(_) | Ambitus(_)
            | Tuning(_) | KeySig(_) | Clef(_) => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        use tusk_model::elements::StaffDefChild::*;
        match self {
            Label(label) => {
                for child in &label.children {
                    match child {
                        tusk_model::elements::LabelChild::Text(text) => {
                            writer.write_text(text)?;
                        }
                        _ => {}
                    }
                }
            }
            LabelAbbr(abbr) => {
                for child in &abbr.children {
                    match child {
                        tusk_model::elements::LabelAbbrChild::Text(text) => {
                            writer.write_text(text)?;
                        }
                        _ => {}
                    }
                }
            }
            LayerDef(ld) => ld.serialize_children(writer)?,
            // InstrDef has no children
            InstrDef(_) => {}
            // Other elements - no children to serialize
            ClefGrp(_) | Mensur(_) | MeterSig(_) | Proport(_) | MeterSigGrp(_) | Ambitus(_)
            | Tuning(_) | KeySig(_) | Clef(_) => {}
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

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            child.serialize_mei(writer)?;
        }
        Ok(())
    }
}

impl MeiSerialize for tusk_model::elements::LayerDefChild {
    fn element_name(&self) -> &'static str {
        use tusk_model::elements::LayerDefChild::*;
        match self {
            LabelAbbr(_) => "labelAbbr",
            MeterSigGrp(_) => "meterSigGrp",
            InstrDef(_) => "instrDef",
            Ambitus(_) => "ambitus",
            Label(_) => "label",
            MeterSig(_) => "meterSig",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        use tusk_model::elements::LayerDefChild::*;
        match self {
            LabelAbbr(la) => la.collect_all_attributes(),
            MeterSigGrp(msg) => msg.collect_all_attributes(),
            InstrDef(id) => id.collect_all_attributes(),
            Ambitus(a) => a.collect_all_attributes(),
            Label(l) => l.collect_all_attributes(),
            MeterSig(ms) => ms.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        use tusk_model::elements::LayerDefChild::*;
        match self {
            LabelAbbr(la) => !la.children.is_empty(),
            MeterSigGrp(msg) => !msg.children.is_empty(),
            InstrDef(_) => false, // InstrDef has no children
            Ambitus(a) => !a.children.is_empty(),
            Label(l) => !l.children.is_empty(),
            MeterSig(_) => false, // MeterSig has no children
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        use tusk_model::elements::LayerDefChild::*;
        match self {
            LabelAbbr(la) => la.serialize_children(writer),
            MeterSigGrp(msg) => msg.serialize_children(writer),
            InstrDef(_) => Ok(()), // InstrDef has no children
            Ambitus(a) => a.serialize_children(writer),
            Label(l) => l.serialize_children(writer),
            MeterSig(_) => Ok(()), // MeterSig has no children
        }
    }
}

// ============================================================================
// PgHead and PgFoot serialization
// ============================================================================

impl CollectAttributes for AttFormework {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl MeiSerialize for PgHead {
    fn element_name(&self) -> &'static str {
        "pgHead"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.formework.collect_attributes());
        attrs.extend(self.horizontal_align.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for PgHeadChild {
    fn element_name(&self) -> &'static str {
        match self {
            PgHeadChild::Text(_) => "#text",
            PgHeadChild::Rend(_) => "rend",
            PgHeadChild::Lb(_) => "lb",
            PgHeadChild::PersName(_) => "persName",
            PgHeadChild::CorpName(_) => "corpName",
            PgHeadChild::Name(_) => "name",
            PgHeadChild::Title(_) => "title",
            PgHeadChild::Date(_) => "date",
            PgHeadChild::Identifier(_) => "identifier",
            PgHeadChild::Ref(_) => "ref",
            PgHeadChild::Ptr(_) => "ptr",
            PgHeadChild::Lg(_) => "lg",
            PgHeadChild::P(_) => "p",
            PgHeadChild::List(_) => "list",
            PgHeadChild::Seg(_) => "seg",
            PgHeadChild::Table(_) => "table",
            PgHeadChild::AnchoredText(_) => "anchoredText",
            // Many other child types exist but are not commonly used
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            PgHeadChild::Rend(r) => r.collect_all_attributes(),
            PgHeadChild::Lb(lb) => lb.collect_all_attributes(),
            PgHeadChild::PersName(pn) => pn.collect_all_attributes(),
            PgHeadChild::CorpName(cn) => cn.collect_all_attributes(),
            PgHeadChild::Name(n) => n.collect_all_attributes(),
            PgHeadChild::Title(t) => t.collect_all_attributes(),
            PgHeadChild::Date(d) => d.collect_all_attributes(),
            PgHeadChild::Identifier(i) => i.collect_all_attributes(),
            PgHeadChild::Ref(r) => r.collect_all_attributes(),
            PgHeadChild::Ptr(p) => p.collect_all_attributes(),
            PgHeadChild::P(p) => p.collect_all_attributes(),
            PgHeadChild::List(l) => l.collect_all_attributes(),
            PgHeadChild::Seg(s) => s.collect_all_attributes(),
            PgHeadChild::Table(t) => t.collect_all_attributes(),
            PgHeadChild::AnchoredText(at) => at.collect_all_attributes(),
            // Lg and other elements - not yet implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            PgHeadChild::Text(_) => false,
            PgHeadChild::Rend(r) => !r.children.is_empty(),
            PgHeadChild::Lb(_) => false,
            PgHeadChild::PersName(pn) => !pn.children.is_empty(),
            PgHeadChild::CorpName(cn) => !cn.children.is_empty(),
            PgHeadChild::Name(n) => !n.children.is_empty(),
            PgHeadChild::Title(t) => !t.children.is_empty(),
            PgHeadChild::Date(d) => !d.children.is_empty(),
            PgHeadChild::Identifier(i) => !i.children.is_empty(),
            PgHeadChild::Ref(r) => !r.children.is_empty(),
            PgHeadChild::Ptr(_) => false,
            PgHeadChild::P(p) => !p.children.is_empty(),
            PgHeadChild::List(l) => !l.children.is_empty(),
            PgHeadChild::Seg(s) => !s.children.is_empty(),
            PgHeadChild::Table(t) => !t.children.is_empty(),
            PgHeadChild::AnchoredText(at) => !at.children.is_empty(),
            // Lg and other elements
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PgHeadChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PgHeadChild::Rend(r) => r.serialize_children(writer),
            PgHeadChild::Lb(_) => Ok(()),
            PgHeadChild::PersName(pn) => pn.serialize_children(writer),
            PgHeadChild::CorpName(cn) => cn.serialize_children(writer),
            PgHeadChild::Name(n) => n.serialize_children(writer),
            PgHeadChild::Title(t) => t.serialize_children(writer),
            PgHeadChild::Date(d) => d.serialize_children(writer),
            PgHeadChild::Identifier(i) => i.serialize_children(writer),
            PgHeadChild::Ref(r) => r.serialize_children(writer),
            PgHeadChild::Ptr(_) => Ok(()),
            PgHeadChild::P(p) => p.serialize_children(writer),
            PgHeadChild::List(l) => l.serialize_children(writer),
            PgHeadChild::Seg(s) => s.serialize_children(writer),
            PgHeadChild::Table(t) => t.serialize_children(writer),
            PgHeadChild::AnchoredText(at) => at.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PgHeadChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for PgFoot {
    fn element_name(&self) -> &'static str {
        "pgFoot"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.formework.collect_attributes());
        attrs.extend(self.horizontal_align.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
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

impl MeiSerialize for PgFootChild {
    fn element_name(&self) -> &'static str {
        match self {
            PgFootChild::Text(_) => "#text",
            PgFootChild::EventList(_) => "eventList",
            PgFootChild::App(_) => "app",
            PgFootChild::StyleName(_) => "styleName",
            PgFootChild::Choice(_) => "choice",
            PgFootChild::AnchoredText(_) => "anchoredText",
            PgFootChild::Seg(_) => "seg",
            PgFootChild::Width(_) => "width",
            PgFootChild::GeogName(_) => "geogName",
            PgFootChild::Unclear(_) => "unclear",
            PgFootChild::P(_) => "p",
            PgFootChild::Fig(_) => "fig",
            PgFootChild::Signatures(_) => "signatures",
            PgFootChild::Stack(_) => "stack",
            PgFootChild::Street(_) => "street",
            PgFootChild::Dimensions(_) => "dimensions",
            PgFootChild::Title(_) => "title",
            PgFootChild::Symbol(_) => "symbol",
            PgFootChild::Corr(_) => "corr",
            PgFootChild::Del(_) => "del",
            PgFootChild::Quote(_) => "quote",
            PgFootChild::Num(_) => "num",
            PgFootChild::PostCode(_) => "postCode",
            PgFootChild::Gap(_) => "gap",
            PgFootChild::Restore(_) => "restore",
            PgFootChild::PersName(_) => "persName",
            PgFootChild::Identifier(_) => "identifier",
            PgFootChild::Lg(_) => "lg",
            PgFootChild::Settlement(_) => "settlement",
            PgFootChild::Reg(_) => "reg",
            PgFootChild::RelationList(_) => "relationList",
            PgFootChild::Lb(_) => "lb",
            PgFootChild::Address(_) => "address",
            PgFootChild::Supplied(_) => "supplied",
            PgFootChild::List(_) => "list",
            PgFootChild::CastList(_) => "castList",
            PgFootChild::Heraldry(_) => "heraldry",
            PgFootChild::BiblStruct(_) => "biblStruct",
            PgFootChild::Ref(_) => "ref",
            PgFootChild::Ptr(_) => "ptr",
            PgFootChild::Annot(_) => "annot",
            PgFootChild::Expan(_) => "expan",
            PgFootChild::Extent(_) => "extent",
            PgFootChild::Region(_) => "region",
            PgFootChild::Term(_) => "term",
            PgFootChild::LocusGrp(_) => "locusGrp",
            PgFootChild::BiblList(_) => "biblList",
            PgFootChild::Q(_) => "q",
            PgFootChild::Country(_) => "country",
            PgFootChild::Depth(_) => "depth",
            PgFootChild::Sic(_) => "sic",
            PgFootChild::Height(_) => "height",
            PgFootChild::Catchwords(_) => "catchwords",
            PgFootChild::Dim(_) => "dim",
            PgFootChild::District(_) => "district",
            PgFootChild::Add(_) => "add",
            PgFootChild::HandShift(_) => "handShift",
            PgFootChild::Abbr(_) => "abbr",
            PgFootChild::PeriodName(_) => "periodName",
            PgFootChild::Relation(_) => "relation",
            PgFootChild::Date(_) => "date",
            PgFootChild::Name(_) => "name",
            PgFootChild::Subst(_) => "subst",
            PgFootChild::Orig(_) => "orig",
            PgFootChild::Repository(_) => "repository",
            PgFootChild::Rend(_) => "rend",
            PgFootChild::Bibl(_) => "bibl",
            PgFootChild::SecFolio(_) => "secFolio",
            PgFootChild::CorpName(_) => "corpName",
            PgFootChild::Locus(_) => "locus",
            PgFootChild::Bloc(_) => "bloc",
            PgFootChild::Damage(_) => "damage",
            PgFootChild::GeogFeat(_) => "geogFeat",
            PgFootChild::Table(_) => "table",
            PgFootChild::PostBox(_) => "postBox",
            PgFootChild::Stamp(_) => "stamp",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PgFootChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PgFootChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PgFootChild::EventList(elem) => elem.serialize_mei(writer),
            PgFootChild::App(elem) => elem.serialize_mei(writer),
            PgFootChild::StyleName(elem) => elem.serialize_mei(writer),
            PgFootChild::Choice(elem) => elem.serialize_mei(writer),
            PgFootChild::AnchoredText(elem) => elem.serialize_mei(writer),
            PgFootChild::Seg(elem) => elem.serialize_mei(writer),
            PgFootChild::Width(elem) => elem.serialize_mei(writer),
            PgFootChild::GeogName(elem) => elem.serialize_mei(writer),
            PgFootChild::Unclear(elem) => elem.serialize_mei(writer),
            PgFootChild::P(elem) => elem.serialize_mei(writer),
            PgFootChild::Fig(elem) => elem.serialize_mei(writer),
            PgFootChild::Signatures(elem) => elem.serialize_mei(writer),
            PgFootChild::Stack(elem) => elem.serialize_mei(writer),
            PgFootChild::Street(elem) => elem.serialize_mei(writer),
            PgFootChild::Dimensions(elem) => elem.serialize_mei(writer),
            PgFootChild::Title(elem) => elem.serialize_mei(writer),
            PgFootChild::Symbol(elem) => elem.serialize_mei(writer),
            PgFootChild::Corr(elem) => elem.serialize_mei(writer),
            PgFootChild::Del(elem) => elem.serialize_mei(writer),
            PgFootChild::Quote(elem) => elem.serialize_mei(writer),
            PgFootChild::Num(elem) => elem.serialize_mei(writer),
            PgFootChild::PostCode(elem) => elem.serialize_mei(writer),
            PgFootChild::Gap(elem) => elem.serialize_mei(writer),
            PgFootChild::Restore(elem) => elem.serialize_mei(writer),
            PgFootChild::PersName(elem) => elem.serialize_mei(writer),
            PgFootChild::Identifier(elem) => elem.serialize_mei(writer),
            PgFootChild::Lg(elem) => elem.serialize_mei(writer),
            PgFootChild::Settlement(elem) => elem.serialize_mei(writer),
            PgFootChild::Reg(elem) => elem.serialize_mei(writer),
            PgFootChild::RelationList(elem) => elem.serialize_mei(writer),
            PgFootChild::Lb(elem) => elem.serialize_mei(writer),
            PgFootChild::Address(elem) => elem.serialize_mei(writer),
            PgFootChild::Supplied(elem) => elem.serialize_mei(writer),
            PgFootChild::List(elem) => elem.serialize_mei(writer),
            PgFootChild::CastList(elem) => elem.serialize_mei(writer),
            PgFootChild::Heraldry(elem) => elem.serialize_mei(writer),
            PgFootChild::BiblStruct(elem) => elem.serialize_mei(writer),
            PgFootChild::Ref(elem) => elem.serialize_mei(writer),
            PgFootChild::Ptr(elem) => elem.serialize_mei(writer),
            PgFootChild::Annot(elem) => elem.serialize_mei(writer),
            PgFootChild::Expan(elem) => elem.serialize_mei(writer),
            PgFootChild::Extent(elem) => elem.serialize_mei(writer),
            PgFootChild::Region(elem) => elem.serialize_mei(writer),
            PgFootChild::Term(elem) => elem.serialize_mei(writer),
            PgFootChild::LocusGrp(elem) => elem.serialize_mei(writer),
            PgFootChild::BiblList(elem) => elem.serialize_mei(writer),
            PgFootChild::Q(elem) => elem.serialize_mei(writer),
            PgFootChild::Country(elem) => elem.serialize_mei(writer),
            PgFootChild::Depth(elem) => elem.serialize_mei(writer),
            PgFootChild::Sic(elem) => elem.serialize_mei(writer),
            PgFootChild::Height(elem) => elem.serialize_mei(writer),
            PgFootChild::Catchwords(elem) => elem.serialize_mei(writer),
            PgFootChild::Dim(elem) => elem.serialize_mei(writer),
            PgFootChild::District(elem) => elem.serialize_mei(writer),
            PgFootChild::Add(elem) => elem.serialize_mei(writer),
            PgFootChild::HandShift(elem) => elem.serialize_mei(writer),
            PgFootChild::Abbr(elem) => elem.serialize_mei(writer),
            PgFootChild::PeriodName(elem) => elem.serialize_mei(writer),
            PgFootChild::Relation(elem) => elem.serialize_mei(writer),
            PgFootChild::Date(elem) => elem.serialize_mei(writer),
            PgFootChild::Name(elem) => elem.serialize_mei(writer),
            PgFootChild::Subst(elem) => elem.serialize_mei(writer),
            PgFootChild::Orig(elem) => elem.serialize_mei(writer),
            PgFootChild::Repository(elem) => elem.serialize_mei(writer),
            PgFootChild::Rend(elem) => elem.serialize_mei(writer),
            PgFootChild::Bibl(elem) => elem.serialize_mei(writer),
            PgFootChild::SecFolio(elem) => elem.serialize_mei(writer),
            PgFootChild::CorpName(elem) => elem.serialize_mei(writer),
            PgFootChild::Locus(elem) => elem.serialize_mei(writer),
            PgFootChild::Bloc(elem) => elem.serialize_mei(writer),
            PgFootChild::Damage(elem) => elem.serialize_mei(writer),
            PgFootChild::GeogFeat(elem) => elem.serialize_mei(writer),
            PgFootChild::Table(elem) => elem.serialize_mei(writer),
            PgFootChild::PostBox(elem) => elem.serialize_mei(writer),
            PgFootChild::Stamp(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// InstrDef serialization
// ============================================================================

impl MeiSerialize for tusk_model::elements::InstrDef {
    fn element_name(&self) -> &'static str {
        "instrDef"
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

        // InstrDefGes MIDI attributes
        if let Some(v) = &self.instr_def_ges.midi_channel {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.channel", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.midi_duty {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.duty", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.midi_port {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.port", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.midi_track {
            attrs.push(("midi.track", v.to_string()));
        }
        if let Some(v) = &self.instr_def_ges.midi_instrnum {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.instrnum", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.midi_instrname {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.instrname", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.midi_pan {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.pan", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.midi_patchname {
            attrs.push(("midi.patchname", v.clone()));
        }
        if let Some(v) = &self.instr_def_ges.midi_patchnum {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.patchnum", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.midi_volume {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("midi.volume", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.azimuth {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("azimuth", s));
            }
        }
        if let Some(v) = &self.instr_def_ges.elevation {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("elevation", s));
            }
        }

        attrs
    }

    fn has_children(&self) -> bool {
        false // InstrDef has no children
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// LabelAbbr serialization
// ============================================================================

impl MeiSerialize for tusk_model::elements::LabelAbbr {
    fn element_name(&self) -> &'static str {
        "labelAbbr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.source.collect_attributes());
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

impl MeiSerialize for tusk_model::elements::LabelAbbrChild {
    fn element_name(&self) -> &'static str {
        use tusk_model::elements::LabelAbbrChild::*;
        match self {
            Text(_) => "#text",
            Q(_) => "q",
            BiblStruct(_) => "biblStruct",
            Sic(_) => "sic",
            Locus(_) => "locus",
            PeriodName(_) => "periodName",
            Width(_) => "width",
            Depth(_) => "depth",
            Ref(_) => "ref",
            Del(_) => "del",
            SecFolio(_) => "secFolio",
            Bloc(_) => "bloc",
            LocusGrp(_) => "locusGrp",
            Gap(_) => "gap",
            RelationList(_) => "relationList",
            PostBox(_) => "postBox",
            Stamp(_) => "stamp",
            Restore(_) => "restore",
            CorpName(_) => "corpName",
            Dimensions(_) => "dimensions",
            Region(_) => "region",
            PostCode(_) => "postCode",
            Supplied(_) => "supplied",
            Seg(_) => "seg",
            Subst(_) => "subst",
            Ptr(_) => "ptr",
            Settlement(_) => "settlement",
            Choice(_) => "choice",
            Abbr(_) => "abbr",
            Unclear(_) => "unclear",
            Address(_) => "address",
            Name(_) => "name",
            Catchwords(_) => "catchwords",
            Term(_) => "term",
            Annot(_) => "annot",
            Country(_) => "country",
            Identifier(_) => "identifier",
            Height(_) => "height",
            Heraldry(_) => "heraldry",
            Repository(_) => "repository",
            Lb(_) => "lb",
            Num(_) => "num",
            Stack(_) => "stack",
            Add(_) => "add",
            Fig(_) => "fig",
            StyleName(_) => "styleName",
            Bibl(_) => "bibl",
            Dim(_) => "dim",
            Title(_) => "title",
            Corr(_) => "corr",
            Extent(_) => "extent",
            Signatures(_) => "signatures",
            Expan(_) => "expan",
            District(_) => "district",
            GeogFeat(_) => "geogFeat",
            GeogName(_) => "geogName",
            PersName(_) => "persName",
            Symbol(_) => "symbol",
            Orig(_) => "orig",
            Relation(_) => "relation",
            Date(_) => "date",
            Rend(_) => "rend",
            Street(_) => "street",
            HandShift(_) => "handShift",
            Reg(_) => "reg",
            Damage(_) => "damage",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        use tusk_model::elements::LabelAbbrChild::*;
        match self {
            Rend(r) => r.collect_all_attributes(),
            Lb(lb) => lb.collect_all_attributes(),
            Ref(r) => r.collect_all_attributes(),
            Ptr(p) => p.collect_all_attributes(),
            PersName(pn) => pn.collect_all_attributes(),
            CorpName(cn) => cn.collect_all_attributes(),
            Name(n) => n.collect_all_attributes(),
            Title(t) => t.collect_all_attributes(),
            Date(d) => d.collect_all_attributes(),
            Identifier(i) => i.collect_all_attributes(),
            // Most children not commonly used - return empty for now
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        use tusk_model::elements::LabelAbbrChild::*;
        match self {
            Text(_) => false,
            Rend(r) => !r.children.is_empty(),
            Lb(_) => false,
            Ptr(_) => false,
            PersName(pn) => !pn.children.is_empty(),
            CorpName(cn) => !cn.children.is_empty(),
            Name(n) => !n.children.is_empty(),
            Title(t) => !t.children.is_empty(),
            Date(d) => !d.children.is_empty(),
            Identifier(i) => !i.children.is_empty(),
            Ref(r) => !r.children.is_empty(),
            // Most children not commonly used
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        use tusk_model::elements::LabelAbbrChild::*;
        match self {
            Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            Rend(r) => r.serialize_children(writer),
            Lb(_) => Ok(()),
            Ptr(_) => Ok(()),
            PersName(pn) => pn.serialize_children(writer),
            CorpName(cn) => cn.serialize_children(writer),
            Name(n) => n.serialize_children(writer),
            Title(t) => t.serialize_children(writer),
            Date(d) => d.serialize_children(writer),
            Identifier(i) => i.serialize_children(writer),
            Ref(r) => r.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "LabelAbbrChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

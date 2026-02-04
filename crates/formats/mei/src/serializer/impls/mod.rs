//! Manual implementations of serialization traits for MEI types.
//!
//! This module contains hand-written implementations for key attribute classes
//! and elements to demonstrate and test the serialization pattern.
//!
//! In the future, these implementations should be code-generated from the MEI ODD
//! specification to cover all types.

use super::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use serde::Serialize;
use std::io::Write;
use tusk_model::att::{
    AttAuthorized, AttBasic, AttBeamAnl, AttBeamGes, AttBeamLog, AttBeamVis, AttBibl,
    AttCalendared, AttClassed, AttCommon, AttDatable, AttDirAnl, AttDirGes, AttDirLog, AttDirVis,
    AttDynamAnl, AttDynamGes, AttDynamLog, AttDynamVis, AttEdit, AttFacsimile, AttFermataAnl,
    AttFermataGes, AttFermataLog, AttFermataVis, AttFiling, AttGraceGrpAnl, AttGraceGrpGes,
    AttGraceGrpLog, AttGraceGrpVis, AttHairpinAnl, AttHairpinGes, AttHairpinLog, AttHairpinVis,
    AttLabelled, AttLang, AttLinking, AttMeiVersion, AttMetadataPointing, AttNInteger,
    AttNNumberLike, AttPointing, AttResponsibility, AttScoreDefAnl, AttScoreDefGes, AttScoreDefLog,
    AttScoreDefVis, AttSlurAnl, AttSlurGes, AttSlurLog, AttSlurVis, AttStaffGrpAnl, AttStaffGrpGes,
    AttStaffGrpLog, AttStaffGrpVis, AttTargetEval, AttTempoAnl, AttTempoGes, AttTempoLog,
    AttTempoVis, AttTieAnl, AttTieGes, AttTieLog, AttTieVis, AttTupletAnl, AttTupletGes,
    AttTupletLog, AttTupletVis, AttTyped, AttXy,
};
use tusk_model::elements::{
    Beam, BeamChild, Change, ChangeChild, ChangeDesc, ChangeDescChild, Date, DateChild, Dir,
    DirChild, Dynam, DynamChild, EncodingDesc, EncodingDescChild, Fermata, FileDesc, FileDescChild,
    GraceGrp, GraceGrpChild, Hairpin, Head, HeadChild, MeiHead, MeiHeadChild, P, PChild, PubStmt,
    PubStmtChild, RevisionDesc, RevisionDescChild, ScoreDef, ScoreDefChild, Slur, SlurChild,
    SourceDesc, SourceDescChild, Tempo, TempoChild, Tie, TieChild, Title, TitleChild, TitleStmt,
    TitleStmtChild, Tuplet, TupletChild,
};

mod note;
mod structure;

/// Serialize any serde-serializable value to a JSON string and strip quotes.
/// This is used for all MEI data types that have serde derives.
pub(crate) fn to_attr_string<T: Serialize>(v: &T) -> Option<String> {
    serde_json::to_string(v)
        .ok()
        .map(|s| s.trim_matches('"').to_string())
}

/// Serialize a Vec of serde-serializable values to space-separated string.
pub(crate) fn serialize_vec_serde<T: Serialize>(vec: &[T]) -> Option<String> {
    if vec.is_empty() {
        None
    } else {
        let parts: Vec<String> = vec.iter().filter_map(to_attr_string).collect();
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(" "))
        }
    }
}

/// Helper macro to push attribute if value is Some and serializes successfully.
macro_rules! push_attr {
    ($attrs:expr, $name:expr, $opt_val:expr) => {
        if let Some(ref v) = $opt_val {
            if let Some(s) = to_attr_string(v) {
                $attrs.push(($name, s));
            }
        }
    };
    // For String/clone types
    ($attrs:expr, $name:expr, clone $opt_val:expr) => {
        if let Some(ref v) = $opt_val {
            $attrs.push(($name, v.clone()));
        }
    };
    // For Vec types
    ($attrs:expr, $name:expr, vec $vec_val:expr) => {
        if let Some(v) = serialize_vec_serde(&$vec_val) {
            $attrs.push(($name, v));
        }
    };
}
pub(crate) use push_attr;

// ============================================================================
// Common attribute class implementations
// ============================================================================

impl CollectAttributes for AttCommon {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();

        // xml:id should come first by convention
        push_attr!(attrs, "xml:id", clone self.xml_id);
        push_attr!(attrs, "xml:base", self.xml_base);
        push_attr!(attrs, "label", clone self.label);
        push_attr!(attrs, "copyof", self.copyof);
        push_attr!(attrs, "corresp", vec self.corresp);
        push_attr!(attrs, "follows", vec self.follows);
        push_attr!(attrs, "next", vec self.next);
        push_attr!(attrs, "precedes", vec self.precedes);
        push_attr!(attrs, "prev", vec self.prev);
        push_attr!(attrs, "sameas", vec self.sameas);
        push_attr!(attrs, "synch", vec self.synch);
        push_attr!(attrs, "n", self.n);
        push_attr!(attrs, "resp", vec self.resp);
        push_attr!(attrs, "class", vec self.class);
        push_attr!(attrs, "type", vec self.r#type);

        attrs
    }
}

impl CollectAttributes for AttFacsimile {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "facs", vec self.facs);
        attrs
    }
}

impl CollectAttributes for AttMetadataPointing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "decls", vec self.decls);
        attrs
    }
}

impl CollectAttributes for AttPointing {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xlink:actuate", self.xlink_actuate);
        push_attr!(attrs, "xlink:role", self.xlink_role);
        push_attr!(attrs, "xlink:show", self.xlink_show);
        push_attr!(attrs, "target", vec self.target);
        push_attr!(attrs, "targettype", clone self.targettype);
        attrs
    }
}

impl CollectAttributes for AttTargetEval {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "evaluate", self.evaluate);
        attrs
    }
}

// ============================================================================
// Staff attribute class implementations
// ============================================================================

impl CollectAttributes for AttBasic {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:id", clone self.xml_id);
        push_attr!(attrs, "xml:base", self.xml_base);
        attrs
    }
}

impl CollectAttributes for AttLabelled {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "label", clone self.label);
        attrs
    }
}

impl CollectAttributes for AttLinking {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "copyof", self.copyof);
        push_attr!(attrs, "corresp", vec self.corresp);
        push_attr!(attrs, "follows", vec self.follows);
        push_attr!(attrs, "next", vec self.next);
        push_attr!(attrs, "precedes", vec self.precedes);
        push_attr!(attrs, "prev", vec self.prev);
        push_attr!(attrs, "sameas", vec self.sameas);
        push_attr!(attrs, "synch", vec self.synch);
        attrs
    }
}

impl CollectAttributes for AttNInteger {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "n", self.n);
        attrs
    }
}

impl CollectAttributes for AttResponsibility {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "resp", vec self.resp);
        attrs
    }
}

impl CollectAttributes for AttTyped {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "class", vec self.class);
        push_attr!(attrs, "type", vec self.r#type);
        attrs
    }
}

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
            // StaffDef children not yet implemented
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

        // Notation type
        if let Some(v) = &self.staff_def_log.notationtype {
            if let Some(s) = to_attr_string(v) {
                attrs.push(("notationtype", s));
            }
        }

        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // StaffDef children not yet fully implemented
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

// ============================================================================
// Control event attribute class implementations
// ============================================================================

impl CollectAttributes for AttLang {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "xml:lang", clone self.xml_lang);
        push_attr!(attrs, "translit", clone self.translit);
        attrs
    }
}

impl CollectAttributes for AttBibl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "analog", clone self.analog);
        attrs
    }
}

// Slur attribute classes
impl CollectAttributes for AttSlurLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttSlurVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        if let Some(v) = &self.x2 {
            attrs.push(("x2", v.to_string()));
        }
        if let Some(v) = &self.y2 {
            attrs.push(("y2", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttSlurGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttSlurAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "join", vec self.join);
        attrs
    }
}

// Tie attribute classes
impl CollectAttributes for AttTieLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttTieVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "bezier", self.bezier);
        push_attr!(attrs, "bulge", self.bulge);
        push_attr!(attrs, "curvedir", self.curvedir);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        if let Some(v) = &self.x2 {
            attrs.push(("x2", v.to_string()));
        }
        if let Some(v) = &self.y2 {
            attrs.push(("y2", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttTieGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttTieAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// Dynam attribute classes
impl CollectAttributes for AttDynamLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttDynamVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        if let Some(v) = &self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttDynamGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "val", self.val);
        push_attr!(attrs, "val2", self.val2);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttDynamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// Hairpin attribute classes
impl CollectAttributes for AttHairpinLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "niente", self.niente);
        attrs
    }
}

impl CollectAttributes for AttHairpinVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "place", self.place);
        if let Some(v) = &self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        push_attr!(attrs, "startvo", self.startvo);
        push_attr!(attrs, "endvo", self.endvo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        if let Some(v) = &self.x2 {
            attrs.push(("x2", v.to_string()));
        }
        if let Some(v) = &self.y2 {
            attrs.push(("y2", v.to_string()));
        }
        push_attr!(attrs, "opening", self.opening);
        push_attr!(attrs, "closed", self.closed);
        push_attr!(attrs, "opening.vertical", self.opening_vertical);
        push_attr!(attrs, "angle.optimize", self.angle_optimize);
        attrs
    }
}

impl CollectAttributes for AttHairpinGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "val", self.val);
        push_attr!(attrs, "val2", self.val2);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttHairpinAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// Dir attribute classes
impl CollectAttributes for AttDirLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "dur", vec self.dur);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        attrs
    }
}

impl CollectAttributes for AttDirVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        if let Some(v) = &self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttDirGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        push_attr!(attrs, "tstamp2.ges", self.tstamp2_ges);
        push_attr!(attrs, "tstamp2.real", self.tstamp2_real);
        attrs
    }
}

impl CollectAttributes for AttDirAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// Tempo attribute classes
impl CollectAttributes for AttTempoLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "mm", self.mm);
        push_attr!(attrs, "mm.unit", self.mm_unit);
        push_attr!(attrs, "mm.dots", self.mm_dots);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        push_attr!(attrs, "tstamp2", self.tstamp2);
        push_attr!(attrs, "func", self.func);
        attrs
    }
}

impl CollectAttributes for AttTempoVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "lform", self.lform);
        push_attr!(attrs, "lwidth", self.lwidth);
        if let Some(v) = &self.lsegs {
            attrs.push(("lsegs", v.to_string()));
        }
        push_attr!(attrs, "lendsym", self.lendsym);
        push_attr!(attrs, "lendsym.size", self.lendsym_size);
        push_attr!(attrs, "lstartsym", self.lstartsym);
        push_attr!(attrs, "lstartsym.size", self.lstartsym_size);
        push_attr!(attrs, "extender", self.extender);
        push_attr!(attrs, "place", self.place);
        if let Some(v) = &self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        push_attr!(attrs, "startho", self.startho);
        push_attr!(attrs, "endho", self.endho);
        push_attr!(attrs, "startto", self.startto);
        push_attr!(attrs, "endto", self.endto);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttTempoGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "midi.bpm", self.midi_bpm);
        push_attr!(attrs, "midi.mspb", self.midi_mspb);
        attrs
    }
}

impl CollectAttributes for AttTempoAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// Fermata attribute classes
impl CollectAttributes for AttFermataLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "part", vec self.part);
        push_attr!(attrs, "partstaff", vec self.partstaff);
        push_attr!(attrs, "plist", vec self.plist);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "evaluate", self.evaluate);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        attrs
    }
}

impl CollectAttributes for AttFermataVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "altsym", self.altsym);
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "enclose", self.enclose);
        push_attr!(attrs, "glyph.auth", self.glyph_auth);
        push_attr!(attrs, "glyph.uri", self.glyph_uri);
        push_attr!(attrs, "glyph.name", clone self.glyph_name);
        push_attr!(attrs, "glyph.num", self.glyph_num);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "fontfam", self.fontfam);
        push_attr!(attrs, "fontname", self.fontname);
        push_attr!(attrs, "fontsize", self.fontsize);
        push_attr!(attrs, "fontstyle", self.fontstyle);
        push_attr!(attrs, "fontweight", self.fontweight);
        push_attr!(attrs, "letterspacing", self.letterspacing);
        push_attr!(attrs, "lineheight", self.lineheight);
        if let Some(v) = &self.vgrp {
            attrs.push(("vgrp", v.to_string()));
        }
        push_attr!(attrs, "ho", self.ho);
        push_attr!(attrs, "to", self.to);
        push_attr!(attrs, "vo", self.vo);
        if let Some(v) = &self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(v) = &self.y {
            attrs.push(("y", v.to_string()));
        }
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "shape", self.shape);
        attrs
    }
}

impl CollectAttributes for AttFermataGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}

impl CollectAttributes for AttFermataAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Grouping element attribute class implementations
// ============================================================================

// Beam attribute classes
impl CollectAttributes for AttBeamLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "beam.with", self.beam_with);
        attrs
    }
}

impl CollectAttributes for AttBeamVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "cue", self.cue);
        push_attr!(attrs, "form", self.form);
        push_attr!(attrs, "place", self.place);
        push_attr!(attrs, "slash", self.slash);
        if let Some(v) = &self.slope {
            attrs.push(("slope", v.to_string()));
        }
        push_attr!(attrs, "visible", self.visible);
        attrs
    }
}

impl CollectAttributes for AttBeamGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttBeamAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// Tuplet attribute classes
impl CollectAttributes for AttTupletLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "beam.with", self.beam_with);
        push_attr!(attrs, "dur", vec self.dur);
        if let Some(v) = &self.num {
            attrs.push(("num", v.to_string()));
        }
        if let Some(v) = &self.numbase {
            attrs.push(("numbase", v.to_string()));
        }
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "startid", self.startid);
        push_attr!(attrs, "endid", self.endid);
        attrs
    }
}

impl CollectAttributes for AttTupletVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        push_attr!(attrs, "num.place", self.num_place);
        push_attr!(attrs, "num.visible", self.num_visible);
        push_attr!(attrs, "bracket.place", self.bracket_place);
        push_attr!(attrs, "bracket.visible", self.bracket_visible);
        push_attr!(attrs, "num.format", self.num_format);
        attrs
    }
}

impl CollectAttributes for AttTupletGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "dur.ges", self.dur_ges);
        push_attr!(attrs, "dots.ges", self.dots_ges);
        if let Some(v) = &self.dur_metrical {
            attrs.push(("dur.metrical", v.to_string()));
        }
        if let Some(v) = &self.dur_ppq {
            attrs.push(("dur.ppq", v.to_string()));
        }
        if let Some(v) = &self.dur_real {
            attrs.push(("dur.real", v.to_string()));
        }
        push_attr!(attrs, "dur.recip", clone self.dur_recip);
        attrs
    }
}

impl CollectAttributes for AttTupletAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// GraceGrp attribute classes
impl CollectAttributes for AttGraceGrpLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "when", self.when);
        push_attr!(attrs, "layer", vec self.layer);
        push_attr!(attrs, "staff", vec self.staff);
        push_attr!(attrs, "tstamp.ges", self.tstamp_ges);
        push_attr!(attrs, "tstamp.real", self.tstamp_real);
        push_attr!(attrs, "tstamp", self.tstamp);
        push_attr!(attrs, "grace", self.grace);
        push_attr!(attrs, "grace.time", self.grace_time);
        push_attr!(attrs, "attach", self.attach);
        attrs
    }
}

impl CollectAttributes for AttGraceGrpVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "color", self.color);
        attrs
    }
}

impl CollectAttributes for AttGraceGrpGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttGraceGrpAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

// ============================================================================
// Header-related attribute class implementations
// ============================================================================

impl CollectAttributes for AttDatable {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "enddate", self.enddate);
        push_attr!(attrs, "isodate", self.isodate);
        push_attr!(attrs, "notafter", self.notafter);
        push_attr!(attrs, "notbefore", self.notbefore);
        push_attr!(attrs, "startdate", self.startdate);
        attrs
    }
}

impl CollectAttributes for AttMeiVersion {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "meiversion", self.meiversion);
        attrs
    }
}

impl CollectAttributes for AttCalendared {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "calendar", clone self.calendar);
        attrs
    }
}

impl CollectAttributes for AttAuthorized {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "codedval", vec self.codedval);
        push_attr!(attrs, "auth", clone self.auth);
        push_attr!(attrs, "auth.uri", self.auth_uri);
        attrs
    }
}

impl CollectAttributes for AttClassed {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "class", vec self.class);
        attrs
    }
}

impl CollectAttributes for AttFiling {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.nonfiling {
            attrs.push(("nonfiling", v.to_string()));
        }
        attrs
    }
}

impl CollectAttributes for AttNNumberLike {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "n", self.n);
        attrs
    }
}

impl CollectAttributes for AttEdit {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "source", vec self.source);
        push_attr!(attrs, "cert", self.cert);
        push_attr!(attrs, "evidence", self.evidence);
        attrs
    }
}

impl CollectAttributes for AttXy {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        if let Some(ref v) = self.x {
            attrs.push(("x", v.to_string()));
        }
        if let Some(ref v) = self.y {
            attrs.push(("y", v.to_string()));
        }
        attrs
    }
}

// ============================================================================
// Control event element implementations
// ============================================================================

impl MeiSerialize for Slur {
    fn element_name(&self) -> &'static str {
        "slur"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.slur_log.collect_attributes());
        attrs.extend(self.slur_vis.collect_attributes());
        attrs.extend(self.slur_ges.collect_attributes());
        attrs.extend(self.slur_anl.collect_attributes());
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

impl MeiSerialize for SlurChild {
    fn element_name(&self) -> &'static str {
        match self {
            SlurChild::Curve(_) => "curve",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            SlurChild::Curve(_) => Vec::new(), // Curve serialization not yet implemented
        }
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Tie {
    fn element_name(&self) -> &'static str {
        "tie"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.tie_log.collect_attributes());
        attrs.extend(self.tie_vis.collect_attributes());
        attrs.extend(self.tie_ges.collect_attributes());
        attrs.extend(self.tie_anl.collect_attributes());
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

impl MeiSerialize for TieChild {
    fn element_name(&self) -> &'static str {
        match self {
            TieChild::Curve(_) => "curve",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            TieChild::Curve(_) => Vec::new(), // Curve serialization not yet implemented
        }
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Dynam {
    fn element_name(&self) -> &'static str {
        "dynam"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.dynam_log.collect_attributes());
        attrs.extend(self.dynam_vis.collect_attributes());
        attrs.extend(self.dynam_ges.collect_attributes());
        attrs.extend(self.dynam_anl.collect_attributes());
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

impl MeiSerialize for DynamChild {
    fn element_name(&self) -> &'static str {
        match self {
            DynamChild::Text(_) => "$text",
            _ => "unknown", // Other children not yet fully implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DynamChild::Text(text) => {
                writer.write_text(text)?;
            }
            _ => {
                // Other children not yet fully implemented
            }
        }
        Ok(())
    }
}

impl MeiSerialize for Hairpin {
    fn element_name(&self) -> &'static str {
        "hairpin"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.hairpin_log.collect_attributes());
        attrs.extend(self.hairpin_vis.collect_attributes());
        attrs.extend(self.hairpin_ges.collect_attributes());
        attrs.extend(self.hairpin_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Hairpin is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

impl MeiSerialize for Dir {
    fn element_name(&self) -> &'static str {
        "dir"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.dir_log.collect_attributes());
        attrs.extend(self.dir_vis.collect_attributes());
        attrs.extend(self.dir_ges.collect_attributes());
        attrs.extend(self.dir_anl.collect_attributes());
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

impl MeiSerialize for DirChild {
    fn element_name(&self) -> &'static str {
        match self {
            DirChild::Text(_) => "$text",
            _ => "unknown", // Other children not yet fully implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DirChild::Text(text) => {
                writer.write_text(text)?;
            }
            _ => {
                // Other children not yet fully implemented
            }
        }
        Ok(())
    }
}

impl MeiSerialize for Tempo {
    fn element_name(&self) -> &'static str {
        "tempo"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.tempo_log.collect_attributes());
        attrs.extend(self.tempo_vis.collect_attributes());
        attrs.extend(self.tempo_ges.collect_attributes());
        attrs.extend(self.tempo_anl.collect_attributes());
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

impl MeiSerialize for TempoChild {
    fn element_name(&self) -> &'static str {
        match self {
            TempoChild::Text(_) => "$text",
            _ => "unknown", // Other children not yet fully implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TempoChild::Text(text) => {
                writer.write_text(text)?;
            }
            _ => {
                // Other children not yet fully implemented
            }
        }
        Ok(())
    }
}

impl MeiSerialize for Fermata {
    fn element_name(&self) -> &'static str {
        "fermata"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.fermata_log.collect_attributes());
        attrs.extend(self.fermata_vis.collect_attributes());
        attrs.extend(self.fermata_ges.collect_attributes());
        attrs.extend(self.fermata_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Fermata is an empty element
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Grouping element implementations
// ============================================================================

impl MeiSerialize for Beam {
    fn element_name(&self) -> &'static str {
        "beam"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.beam_log.collect_attributes());
        attrs.extend(self.beam_vis.collect_attributes());
        attrs.extend(self.beam_ges.collect_attributes());
        attrs.extend(self.beam_anl.collect_attributes());
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

impl MeiSerialize for BeamChild {
    fn element_name(&self) -> &'static str {
        match self {
            BeamChild::Note(_) => "note",
            BeamChild::Rest(_) => "rest",
            BeamChild::Chord(_) => "chord",
            BeamChild::Space(_) => "space",
            BeamChild::Beam(_) => "beam",
            BeamChild::Tuplet(_) => "tuplet",
            BeamChild::GraceGrp(_) => "graceGrp",
            _ => "unknown", // Other children not yet implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BeamChild::Note(note) => note.serialize_mei(writer),
            BeamChild::Rest(rest) => rest.serialize_mei(writer),
            BeamChild::Chord(chord) => chord.serialize_mei(writer),
            BeamChild::Space(space) => space.serialize_mei(writer),
            BeamChild::Beam(beam) => beam.serialize_mei(writer),
            BeamChild::Tuplet(tuplet) => tuplet.serialize_mei(writer),
            BeamChild::GraceGrp(grace_grp) => grace_grp.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Tuplet {
    fn element_name(&self) -> &'static str {
        "tuplet"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.tuplet_log.collect_attributes());
        attrs.extend(self.tuplet_vis.collect_attributes());
        attrs.extend(self.tuplet_ges.collect_attributes());
        attrs.extend(self.tuplet_anl.collect_attributes());
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

impl MeiSerialize for TupletChild {
    fn element_name(&self) -> &'static str {
        match self {
            TupletChild::Note(_) => "note",
            TupletChild::Rest(_) => "rest",
            TupletChild::Chord(_) => "chord",
            TupletChild::Space(_) => "space",
            TupletChild::Beam(_) => "beam",
            TupletChild::Tuplet(_) => "tuplet",
            TupletChild::GraceGrp(_) => "graceGrp",
            _ => "unknown", // Other children not yet implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TupletChild::Note(note) => note.serialize_mei(writer),
            TupletChild::Rest(rest) => rest.serialize_mei(writer),
            TupletChild::Chord(chord) => chord.serialize_mei(writer),
            TupletChild::Space(space) => space.serialize_mei(writer),
            TupletChild::Beam(beam) => beam.serialize_mei(writer),
            TupletChild::Tuplet(tuplet) => tuplet.serialize_mei(writer),
            TupletChild::GraceGrp(grace_grp) => grace_grp.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for GraceGrp {
    fn element_name(&self) -> &'static str {
        "graceGrp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.grace_grp_log.collect_attributes());
        attrs.extend(self.grace_grp_vis.collect_attributes());
        attrs.extend(self.grace_grp_ges.collect_attributes());
        attrs.extend(self.grace_grp_anl.collect_attributes());
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

impl MeiSerialize for GraceGrpChild {
    fn element_name(&self) -> &'static str {
        match self {
            GraceGrpChild::Note(_) => "note",
            GraceGrpChild::Rest(_) => "rest",
            GraceGrpChild::Chord(_) => "chord",
            GraceGrpChild::Space(_) => "space",
            GraceGrpChild::Beam(_) => "beam",
            GraceGrpChild::Tuplet(_) => "tuplet",
            GraceGrpChild::GraceGrp(_) => "graceGrp",
            _ => "unknown", // Other children not yet implemented
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            GraceGrpChild::Note(note) => note.serialize_mei(writer),
            GraceGrpChild::Rest(rest) => rest.serialize_mei(writer),
            GraceGrpChild::Chord(chord) => chord.serialize_mei(writer),
            GraceGrpChild::Space(space) => space.serialize_mei(writer),
            GraceGrpChild::Beam(beam) => beam.serialize_mei(writer),
            GraceGrpChild::Tuplet(tuplet) => tuplet.serialize_mei(writer),
            GraceGrpChild::GraceGrp(grace_grp) => grace_grp.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

// ============================================================================
// Header element implementations
// ============================================================================

impl MeiSerialize for MeiHead {
    fn element_name(&self) -> &'static str {
        "meiHead"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.mei_version.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
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

impl MeiSerialize for MeiHeadChild {
    fn element_name(&self) -> &'static str {
        match self {
            MeiHeadChild::FileDesc(_) => "fileDesc",
            MeiHeadChild::EncodingDesc(_) => "encodingDesc",
            MeiHeadChild::WorkList(_) => "workList",
            MeiHeadChild::RevisionDesc(_) => "revisionDesc",
            MeiHeadChild::ManifestationList(_) => "manifestationList",
            MeiHeadChild::AltId(_) => "altId",
            MeiHeadChild::ExtMeta(_) => "extMeta",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new() // Handled by recursive serialization
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MeiHeadChild::FileDesc(elem) => elem.serialize_mei(writer),
            MeiHeadChild::EncodingDesc(elem) => elem.serialize_mei(writer),
            MeiHeadChild::RevisionDesc(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for FileDesc {
    fn element_name(&self) -> &'static str {
        "fileDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for FileDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            FileDescChild::TitleStmt(_) => "titleStmt",
            FileDescChild::PubStmt(_) => "pubStmt",
            FileDescChild::SourceDesc(_) => "sourceDesc",
            FileDescChild::Extent(_) => "extent",
            FileDescChild::EditionStmt(_) => "editionStmt",
            FileDescChild::SeriesStmt(_) => "seriesStmt",
            FileDescChild::NotesStmt(_) => "notesStmt",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FileDescChild::TitleStmt(elem) => elem.serialize_mei(writer),
            FileDescChild::PubStmt(elem) => elem.serialize_mei(writer),
            FileDescChild::SourceDesc(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for TitleStmt {
    fn element_name(&self) -> &'static str {
        "titleStmt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for TitleStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            TitleStmtChild::Title(_) => "title",
            TitleStmtChild::Creator(_) => "creator",
            TitleStmtChild::Editor(_) => "editor",
            TitleStmtChild::Funder(_) => "funder",
            TitleStmtChild::Head(_) => "head",
            TitleStmtChild::RespStmt(_) => "respStmt",
            TitleStmtChild::Contributor(_) => "contributor",
            TitleStmtChild::Sponsor(_) => "sponsor",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TitleStmtChild::Title(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Head(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Title {
    fn element_name(&self) -> &'static str {
        "title"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.filing.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
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

impl MeiSerialize for TitleChild {
    fn element_name(&self) -> &'static str {
        match self {
            TitleChild::Text(_) => "#text",
            TitleChild::TitlePart(_) => "titlePart",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TitleChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for Head {
    fn element_name(&self) -> &'static str {
        "head"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
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

impl MeiSerialize for HeadChild {
    fn element_name(&self) -> &'static str {
        match self {
            HeadChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            HeadChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for PubStmt {
    fn element_name(&self) -> &'static str {
        "pubStmt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for PubStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            PubStmtChild::Date(_) => "date",
            PubStmtChild::Publisher(_) => "publisher",
            PubStmtChild::Address(_) => "address",
            PubStmtChild::PubPlace(_) => "pubPlace",
            PubStmtChild::RespStmt(_) => "respStmt",
            PubStmtChild::Availability(_) => "availability",
            PubStmtChild::Identifier(_) => "identifier",
            PubStmtChild::Distributor(_) => "distributor",
            PubStmtChild::Head(_) => "head",
            PubStmtChild::Unpub(_) => "unpub",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PubStmtChild::Date(elem) => elem.serialize_mei(writer),
            PubStmtChild::Head(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for SourceDesc {
    fn element_name(&self) -> &'static str {
        "sourceDesc"
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

impl MeiSerialize for SourceDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            SourceDescChild::Head(_) => "head",
            SourceDescChild::Source(_) => "source",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SourceDescChild::Head(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for EncodingDesc {
    fn element_name(&self) -> &'static str {
        "encodingDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for EncodingDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            EncodingDescChild::AppInfo(_) => "appInfo",
            EncodingDescChild::EditorialDecl(_) => "editorialDecl",
            EncodingDescChild::ProjectDesc(_) => "projectDesc",
            EncodingDescChild::SamplingDecl(_) => "samplingDecl",
            EncodingDescChild::TagsDecl(_) => "tagsDecl",
            EncodingDescChild::ClassDecls(_) => "classDecls",
            EncodingDescChild::DomainsDecl(_) => "domainsDecl",
            EncodingDescChild::Head(_) => "head",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EncodingDescChild::Head(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for RevisionDesc {
    fn element_name(&self) -> &'static str {
        "revisionDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for RevisionDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            RevisionDescChild::Head(_) => "head",
            RevisionDescChild::Change(_) => "change",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RevisionDescChild::Head(elem) => elem.serialize_mei(writer),
            RevisionDescChild::Change(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Change {
    fn element_name(&self) -> &'static str {
        "change"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for ChangeChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChangeChild::Date(_) => "date",
            ChangeChild::ChangeDesc(_) => "changeDesc",
            ChangeChild::RespStmt(_) => "respStmt",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ChangeChild::Date(elem) => elem.serialize_mei(writer),
            ChangeChild::ChangeDesc(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for ChangeDesc {
    fn element_name(&self) -> &'static str {
        "changeDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for ChangeDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            ChangeDescChild::P(_) => "p",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        true
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ChangeDescChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Date {
    fn element_name(&self) -> &'static str {
        "date"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.calendared.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
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

impl MeiSerialize for DateChild {
    fn element_name(&self) -> &'static str {
        match self {
            DateChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            DateChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for P {
    fn element_name(&self) -> &'static str {
        "p"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
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

impl MeiSerialize for PChild {
    fn element_name(&self) -> &'static str {
        match self {
            PChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            _ => Ok(()), // Other children skipped for now
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests moved to submodules (note.rs, structure.rs, etc.)
}

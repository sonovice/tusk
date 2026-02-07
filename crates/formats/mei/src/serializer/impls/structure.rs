//! Serializer implementations for structural MEI elements.
//!
//! This module contains implementations for Measure, Staff, Layer, Section, Mdiv,
//! and their child elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttClefAnl, AttClefGes, AttClefLog, AttClefVis, AttEndingAnl, AttEndingGes, AttEndingLog,
    AttEndingVis, AttEvent, AttLayerAnl, AttLayerGes, AttLayerLog, AttLayerVis, AttMRestAnl,
    AttMRestGes, AttMRestLog, AttMRestVis, AttMdivAnl, AttMdivGes, AttMdivLog, AttMdivVis,
    AttMeasureAnl, AttMeasureGes, AttMeasureLog, AttMeasureVis, AttPbAnl, AttPbGes, AttPbLog,
    AttPbVis, AttSbAnl, AttSbGes, AttSbLog, AttSbVis, AttSectionAnl, AttSectionGes, AttSectionLog,
    AttSectionVis, AttStaffAnl, AttStaffGes, AttStaffLog, AttStaffVis,
};
use tusk_model::elements::{
    Add, BTrem, Body, BodyChild, Clef, Ending, EndingChild, FTrem, Layer, LayerChild, MRest, Mdiv,
    MdivChild, Measure, MeasureChild, Pb, Sb, Score, ScoreChild, Section, SectionChild, Staff,
    StaffChild, StaffDef,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Measure attribute class implementations
// ============================================================================

// ============================================================================
// Staff attribute class implementations
// ============================================================================

// ============================================================================
// Layer attribute class implementations
// ============================================================================

// ============================================================================
// MRest (measure rest) attribute class implementations
// ============================================================================

// ============================================================================
// Section attribute class implementations
// ============================================================================

// ============================================================================
// Ending attribute class implementations
// ============================================================================

// ============================================================================
// Sb (system break) attribute class implementations
// ============================================================================

// ============================================================================
// Pb (page break) attribute class implementations
// ============================================================================

// ============================================================================
// Mdiv attribute class implementations
// ============================================================================

// ============================================================================
// Staff element implementation
// ============================================================================

impl MeiSerialize for Staff {
    fn element_name(&self) -> &'static str {
        "staff"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.n_integer.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.typed.collect_attributes());
        attrs.extend(self.staff_log.collect_attributes());
        attrs.extend(self.staff_vis.collect_attributes());
        attrs.extend(self.staff_ges.collect_attributes());
        attrs.extend(self.staff_anl.collect_attributes());
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

impl MeiSerialize for StaffChild {
    fn element_name(&self) -> &'static str {
        match self {
            StaffChild::Layer(_) => "layer",
            StaffChild::StaffDef(_) => "staffDef",
            StaffChild::Add(_) => "add",
            // Other child types - can be added as needed
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            StaffChild::Layer(layer) => layer.collect_all_attributes(),
            StaffChild::StaffDef(staff_def) => staff_def.collect_all_attributes(),
            StaffChild::Add(add) => add.collect_all_attributes(),
            // Other child types - not yet implemented
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            StaffChild::Layer(layer) => layer.has_children(),
            StaffChild::StaffDef(staff_def) => staff_def.has_children(),
            StaffChild::Add(add) => add.has_children(),
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StaffChild::Layer(layer) => layer.serialize_children(writer),
            StaffChild::StaffDef(staff_def) => staff_def.serialize_children(writer),
            StaffChild::Add(add) => add.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "StaffChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Layer element implementation
// ============================================================================

impl MeiSerialize for Layer {
    fn element_name(&self) -> &'static str {
        "layer"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.n_integer.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.typed.collect_attributes());
        attrs.extend(self.layer_log.collect_attributes());
        attrs.extend(self.layer_vis.collect_attributes());
        attrs.extend(self.layer_ges.collect_attributes());
        attrs.extend(self.layer_anl.collect_attributes());
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

impl MeiSerialize for LayerChild {
    fn element_name(&self) -> &'static str {
        match self {
            LayerChild::Note(_) => "note",
            LayerChild::Rest(_) => "rest",
            LayerChild::Chord(_) => "chord",
            LayerChild::Space(_) => "space",
            LayerChild::Beam(_) => "beam",
            LayerChild::Tuplet(_) => "tuplet",
            LayerChild::Clef(_) => "clef",
            LayerChild::Accid(_) => "accid",
            LayerChild::Artic(_) => "artic",
            LayerChild::Dot(_) => "dot",
            LayerChild::BarLine(_) => "barLine",
            LayerChild::KeySig(_) => "keySig",
            LayerChild::MeterSig(_) => "meterSig",
            LayerChild::MRest(_) => "mRest",
            LayerChild::MSpace(_) => "mSpace",
            LayerChild::MultiRest(_) => "multiRest",
            LayerChild::BTrem(_) => "bTrem",
            LayerChild::FTrem(_) => "fTrem",
            LayerChild::Add(_) => "add",
            LayerChild::App(_) => "app",
            LayerChild::Choice(_) => "choice",
            LayerChild::Corr(_) => "corr",
            LayerChild::Damage(_) => "damage",
            LayerChild::Del(_) => "del",
            LayerChild::Gap(_) => "gap",
            LayerChild::HandShift(_) => "handShift",
            LayerChild::Orig(_) => "orig",
            LayerChild::Reg(_) => "reg",
            LayerChild::Restore(_) => "restore",
            LayerChild::Sic(_) => "sic",
            LayerChild::Subst(_) => "subst",
            LayerChild::Supplied(_) => "supplied",
            LayerChild::Unclear(_) => "unclear",
            LayerChild::ClefGrp(_) => "clefGrp",
            LayerChild::Custos(_) => "custos",
            LayerChild::DivLine(_) => "divLine",
            LayerChild::GraceGrp(_) => "graceGrp",
            LayerChild::HalfmRpt(_) => "halfmRpt",
            LayerChild::Ligature(_) => "ligature",
            LayerChild::Mensur(_) => "mensur",
            LayerChild::MeterSigGrp(_) => "meterSigGrp",
            LayerChild::MRpt(_) => "mRpt",
            LayerChild::MRpt2(_) => "mRpt2",
            LayerChild::MultiRpt(_) => "multiRpt",
            LayerChild::Neume(_) => "neume",
            LayerChild::Pad(_) => "pad",
            LayerChild::Pb(_) => "pb",
            LayerChild::Proport(_) => "proport",
            LayerChild::Sb(_) => "sb",
            LayerChild::Syllable(_) => "syllable",
            LayerChild::TabDurSym(_) => "tabDurSym",
            LayerChild::TabGrp(_) => "tabGrp",
            LayerChild::AnchoredText(_) => "anchoredText",
            LayerChild::Annot(_) => "annot",
            LayerChild::BeatRpt(_) => "beatRpt",
            LayerChild::Cb(_) => "cb",
            LayerChild::ColLayout(_) => "colLayout",
            LayerChild::Curve(_) => "curve",
            LayerChild::Line(_) => "line",
            LayerChild::Midi(_) => "midi",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            LayerChild::Note(elem) => elem.collect_all_attributes(),
            LayerChild::Rest(elem) => elem.collect_all_attributes(),
            LayerChild::Chord(elem) => elem.collect_all_attributes(),
            LayerChild::Space(elem) => elem.collect_all_attributes(),
            LayerChild::Beam(elem) => elem.collect_all_attributes(),
            LayerChild::Tuplet(elem) => elem.collect_all_attributes(),
            LayerChild::Accid(elem) => elem.collect_all_attributes(),
            LayerChild::Artic(elem) => elem.collect_all_attributes(),
            LayerChild::Dot(elem) => elem.collect_all_attributes(),
            LayerChild::MRest(elem) => elem.collect_all_attributes(),
            LayerChild::Clef(elem) => elem.collect_all_attributes(),
            LayerChild::BTrem(elem) => elem.collect_all_attributes(),
            LayerChild::FTrem(elem) => elem.collect_all_attributes(),
            LayerChild::MSpace(elem) => elem.collect_all_attributes(),
            LayerChild::BarLine(elem) => elem.collect_all_attributes(),
            LayerChild::Add(elem) => elem.collect_all_attributes(),
            LayerChild::App(elem) => elem.collect_all_attributes(),
            LayerChild::Choice(elem) => elem.collect_all_attributes(),
            LayerChild::Corr(elem) => elem.collect_all_attributes(),
            LayerChild::Damage(elem) => elem.collect_all_attributes(),
            LayerChild::Del(elem) => elem.collect_all_attributes(),
            LayerChild::Gap(elem) => elem.collect_all_attributes(),
            LayerChild::HandShift(elem) => elem.collect_all_attributes(),
            LayerChild::Orig(elem) => elem.collect_all_attributes(),
            LayerChild::Reg(elem) => elem.collect_all_attributes(),
            LayerChild::Restore(elem) => elem.collect_all_attributes(),
            LayerChild::Sic(elem) => elem.collect_all_attributes(),
            LayerChild::Subst(elem) => elem.collect_all_attributes(),
            LayerChild::Supplied(elem) => elem.collect_all_attributes(),
            LayerChild::Unclear(elem) => elem.collect_all_attributes(),
            LayerChild::ClefGrp(elem) => elem.collect_all_attributes(),
            LayerChild::Custos(elem) => elem.collect_all_attributes(),
            LayerChild::DivLine(elem) => elem.collect_all_attributes(),
            LayerChild::GraceGrp(elem) => elem.collect_all_attributes(),
            LayerChild::HalfmRpt(elem) => elem.collect_all_attributes(),
            LayerChild::Ligature(elem) => elem.collect_all_attributes(),
            LayerChild::Mensur(elem) => elem.collect_all_attributes(),
            LayerChild::MeterSigGrp(elem) => elem.collect_all_attributes(),
            LayerChild::MRpt(elem) => elem.collect_all_attributes(),
            LayerChild::MRpt2(elem) => elem.collect_all_attributes(),
            LayerChild::MultiRpt(elem) => elem.collect_all_attributes(),
            LayerChild::MultiRest(elem) => elem.collect_all_attributes(),
            LayerChild::Neume(elem) => elem.collect_all_attributes(),
            LayerChild::Pad(elem) => elem.collect_all_attributes(),
            LayerChild::Pb(elem) => elem.collect_all_attributes(),
            LayerChild::Proport(elem) => elem.collect_all_attributes(),
            LayerChild::Sb(elem) => elem.collect_all_attributes(),
            LayerChild::Syllable(elem) => elem.collect_all_attributes(),
            LayerChild::TabDurSym(elem) => elem.collect_all_attributes(),
            LayerChild::TabGrp(elem) => elem.collect_all_attributes(),
            LayerChild::AnchoredText(elem) => elem.collect_all_attributes(),
            LayerChild::Annot(elem) => elem.collect_all_attributes(),
            LayerChild::BeatRpt(elem) => elem.collect_all_attributes(),
            LayerChild::Cb(elem) => elem.collect_all_attributes(),
            LayerChild::ColLayout(elem) => elem.collect_all_attributes(),
            LayerChild::Curve(elem) => elem.collect_all_attributes(),
            LayerChild::KeySig(elem) => elem.collect_all_attributes(),
            LayerChild::Line(elem) => elem.collect_all_attributes(),
            LayerChild::MeterSig(elem) => elem.collect_all_attributes(),
            LayerChild::Midi(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            LayerChild::Note(elem) => elem.has_children(),
            LayerChild::Rest(elem) => elem.has_children(),
            LayerChild::Chord(elem) => elem.has_children(),
            LayerChild::Beam(elem) => elem.has_children(),
            LayerChild::Tuplet(elem) => elem.has_children(),
            LayerChild::Accid(_) => false,
            LayerChild::Artic(_) => false,
            LayerChild::Dot(_) => false,
            LayerChild::Space(_) => false,
            LayerChild::MRest(_) => false,
            LayerChild::MSpace(_) => false,
            LayerChild::Clef(_) => false,
            LayerChild::BTrem(elem) => elem.has_children(),
            LayerChild::FTrem(elem) => elem.has_children(),
            LayerChild::BarLine(_) => false,
            LayerChild::Add(elem) => elem.has_children(),
            LayerChild::App(elem) => elem.has_children(),
            LayerChild::Choice(elem) => elem.has_children(),
            LayerChild::Corr(elem) => elem.has_children(),
            LayerChild::Damage(elem) => elem.has_children(),
            LayerChild::Del(elem) => elem.has_children(),
            LayerChild::Gap(_) => false,
            LayerChild::HandShift(_) => false,
            LayerChild::Orig(elem) => elem.has_children(),
            LayerChild::Reg(elem) => elem.has_children(),
            LayerChild::Restore(elem) => elem.has_children(),
            LayerChild::Sic(elem) => elem.has_children(),
            LayerChild::Subst(elem) => elem.has_children(),
            LayerChild::Supplied(elem) => elem.has_children(),
            LayerChild::Unclear(elem) => elem.has_children(),
            LayerChild::ClefGrp(elem) => elem.has_children(),
            LayerChild::Custos(_) => false,
            LayerChild::DivLine(_) => false,
            LayerChild::GraceGrp(elem) => elem.has_children(),
            LayerChild::HalfmRpt(_) => false,
            LayerChild::Ligature(elem) => elem.has_children(),
            LayerChild::Mensur(_) => false,
            LayerChild::MeterSigGrp(elem) => elem.has_children(),
            LayerChild::MRpt(_) => false,
            LayerChild::MRpt2(_) => false,
            LayerChild::MultiRpt(_) => false,
            LayerChild::MultiRest(_) => false,
            LayerChild::Neume(elem) => elem.has_children(),
            LayerChild::Pad(_) => false,
            LayerChild::Pb(elem) => elem.has_children(),
            LayerChild::Proport(_) => false,
            LayerChild::Sb(_) => false,
            LayerChild::Syllable(elem) => elem.has_children(),
            LayerChild::TabDurSym(_) => false,
            LayerChild::TabGrp(elem) => elem.has_children(),
            LayerChild::AnchoredText(elem) => elem.has_children(),
            LayerChild::Annot(elem) => elem.has_children(),
            LayerChild::BeatRpt(_) => false,
            LayerChild::Cb(_) => false,
            LayerChild::ColLayout(elem) => elem.has_children(),
            LayerChild::Curve(_) => false,
            LayerChild::KeySig(_) => false,
            LayerChild::Line(elem) => elem.has_children(),
            LayerChild::MeterSig(_) => false,
            LayerChild::Midi(elem) => elem.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LayerChild::Note(elem) => elem.serialize_children(writer),
            LayerChild::Rest(elem) => elem.serialize_children(writer),
            LayerChild::Chord(elem) => elem.serialize_children(writer),
            LayerChild::Beam(elem) => elem.serialize_children(writer),
            LayerChild::Tuplet(elem) => elem.serialize_children(writer),
            LayerChild::MRest(_) => Ok(()),
            LayerChild::MSpace(_) => Ok(()),
            LayerChild::BTrem(elem) => elem.serialize_children(writer),
            LayerChild::FTrem(elem) => elem.serialize_children(writer),
            LayerChild::BarLine(_) => Ok(()),
            LayerChild::Add(elem) => elem.serialize_children(writer),
            LayerChild::App(elem) => elem.serialize_children(writer),
            LayerChild::Choice(elem) => elem.serialize_children(writer),
            LayerChild::Corr(elem) => elem.serialize_children(writer),
            LayerChild::Damage(elem) => elem.serialize_children(writer),
            LayerChild::Del(elem) => elem.serialize_children(writer),
            LayerChild::Gap(_) => Ok(()),
            LayerChild::HandShift(_) => Ok(()),
            LayerChild::Orig(elem) => elem.serialize_children(writer),
            LayerChild::Reg(elem) => elem.serialize_children(writer),
            LayerChild::Restore(elem) => elem.serialize_children(writer),
            LayerChild::Sic(elem) => elem.serialize_children(writer),
            LayerChild::Subst(elem) => elem.serialize_children(writer),
            LayerChild::Supplied(elem) => elem.serialize_children(writer),
            LayerChild::Unclear(elem) => elem.serialize_children(writer),
            LayerChild::ClefGrp(elem) => elem.serialize_children(writer),
            LayerChild::Custos(_) => Ok(()),
            LayerChild::DivLine(_) => Ok(()),
            LayerChild::GraceGrp(elem) => elem.serialize_children(writer),
            LayerChild::HalfmRpt(_) => Ok(()),
            LayerChild::Ligature(elem) => elem.serialize_children(writer),
            LayerChild::Mensur(_) => Ok(()),
            LayerChild::MeterSigGrp(elem) => elem.serialize_children(writer),
            LayerChild::MRpt(_) => Ok(()),
            LayerChild::MRpt2(_) => Ok(()),
            LayerChild::MultiRpt(_) => Ok(()),
            LayerChild::MultiRest(_) => Ok(()),
            LayerChild::Neume(elem) => elem.serialize_children(writer),
            LayerChild::Pad(_) => Ok(()),
            LayerChild::Pb(elem) => elem.serialize_children(writer),
            LayerChild::Proport(_) => Ok(()),
            LayerChild::Sb(_) => Ok(()),
            LayerChild::Syllable(elem) => elem.serialize_children(writer),
            LayerChild::TabDurSym(_) => Ok(()),
            LayerChild::TabGrp(elem) => elem.serialize_children(writer),
            LayerChild::AnchoredText(elem) => elem.serialize_children(writer),
            LayerChild::Annot(elem) => elem.serialize_children(writer),
            LayerChild::BeatRpt(_) => Ok(()),
            LayerChild::Cb(_) => Ok(()),
            LayerChild::ColLayout(elem) => elem.serialize_children(writer),
            LayerChild::Curve(_) => Ok(()),
            LayerChild::KeySig(_) => Ok(()),
            LayerChild::Line(elem) => elem.serialize_children(writer),
            LayerChild::MeterSig(_) => Ok(()),
            LayerChild::Midi(elem) => elem.serialize_children(writer),
            LayerChild::Accid(_) => Ok(()),
            LayerChild::Artic(_) => Ok(()),
            LayerChild::Clef(_) => Ok(()),
            LayerChild::Dot(_) => Ok(()),
            LayerChild::Space(_) => Ok(()),
        }
    }
}

// ============================================================================
// MRest (measure rest) element implementation
// ============================================================================

impl MeiSerialize for MRest {
    fn element_name(&self) -> &'static str {
        "mRest"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.m_rest_log.collect_attributes());
        attrs.extend(self.m_rest_vis.collect_attributes());
        attrs.extend(self.m_rest_ges.collect_attributes());
        attrs.extend(self.m_rest_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // MRest has no children per MEI spec
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Clef attribute class implementations
// ============================================================================

// ============================================================================
// Clef element implementation
// ============================================================================

impl MeiSerialize for Clef {
    fn element_name(&self) -> &'static str {
        "clef"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.event.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.clef_log.collect_attributes());
        attrs.extend(self.clef_ges.collect_attributes());
        attrs.extend(self.clef_vis.collect_attributes());
        attrs.extend(self.clef_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false // Clef has no children per MEI spec
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }
}

// ============================================================================
// Measure element implementation
// ============================================================================

impl MeiSerialize for Measure {
    fn element_name(&self) -> &'static str {
        "measure"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.measure_log.collect_attributes());
        attrs.extend(self.measure_ges.collect_attributes());
        attrs.extend(self.measure_vis.collect_attributes());
        attrs.extend(self.measure_anl.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
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

impl MeiSerialize for MeasureChild {
    fn element_name(&self) -> &'static str {
        match self {
            MeasureChild::Staff(_) => "staff",
            MeasureChild::Hairpin(_) => "hairpin",
            MeasureChild::Slur(_) => "slur",
            MeasureChild::Tie(_) => "tie",
            MeasureChild::Dynam(_) => "dynam",
            MeasureChild::Dir(_) => "dir",
            MeasureChild::Tempo(_) => "tempo",
            MeasureChild::Fermata(_) => "fermata",
            MeasureChild::Breath(_) => "breath",
            MeasureChild::Caesura(_) => "caesura",
            MeasureChild::Trill(_) => "trill",
            MeasureChild::Mordent(_) => "mordent",
            MeasureChild::Turn(_) => "turn",
            MeasureChild::Harm(_) => "harm",
            MeasureChild::Pedal(_) => "pedal",
            MeasureChild::Arpeg(_) => "arpeg",
            MeasureChild::Gliss(_) => "gliss",
            MeasureChild::Bend(_) => "bend",
            MeasureChild::Octave(_) => "octave",
            MeasureChild::BeamSpan(_) => "beamSpan",
            MeasureChild::TupletSpan(_) => "tupletSpan",
            MeasureChild::BracketSpan(_) => "bracketSpan",
            MeasureChild::Phrase(_) => "phrase",
            MeasureChild::Lv(_) => "lv",
            MeasureChild::Ornam(_) => "ornam",
            MeasureChild::RepeatMark(_) => "repeatMark",
            MeasureChild::HarpPedal(_) => "harpPedal",
            MeasureChild::Fing(_) => "fing",
            MeasureChild::FingGrp(_) => "fingGrp",
            MeasureChild::AnchoredText(_) => "anchoredText",
            MeasureChild::Curve(_) => "curve",
            MeasureChild::Line(_) => "line",
            MeasureChild::Midi(_) => "midi",
            MeasureChild::Attacca(_) => "attacca",
            MeasureChild::CpMark(_) => "cpMark",
            MeasureChild::MetaMark(_) => "metaMark",
            MeasureChild::Reh(_) => "reh",
            MeasureChild::MNum(_) => "mNum",
            MeasureChild::StaffDef(_) => "staffDef",
            MeasureChild::Ossia(_) => "ossia",
            MeasureChild::Annot(_) => "annot",
            MeasureChild::Relation(_) => "relation",
            MeasureChild::RelationList(_) => "relationList",
            MeasureChild::Sp(_) => "sp",
            MeasureChild::StageDir(_) => "stageDir",
            MeasureChild::Pb(_) => "pb",
            MeasureChild::Sb(_) => "sb",
            MeasureChild::Cb(_) => "cb",
            MeasureChild::ColLayout(_) => "colLayout",
            MeasureChild::Gap(_) => "gap",
            MeasureChild::HandShift(_) => "handShift",
            // Editorial elements
            MeasureChild::Add(_) => "add",
            MeasureChild::App(_) => "app",
            MeasureChild::Choice(_) => "choice",
            MeasureChild::Corr(_) => "corr",
            MeasureChild::Damage(_) => "damage",
            MeasureChild::Del(_) => "del",
            MeasureChild::Orig(_) => "orig",
            MeasureChild::Reg(_) => "reg",
            MeasureChild::Restore(_) => "restore",
            MeasureChild::Sic(_) => "sic",
            MeasureChild::Subst(_) => "subst",
            MeasureChild::Supplied(_) => "supplied",
            MeasureChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            MeasureChild::Staff(elem) => elem.collect_all_attributes(),
            MeasureChild::Dynam(elem) => elem.collect_all_attributes(),
            MeasureChild::Dir(elem) => elem.collect_all_attributes(),
            MeasureChild::Hairpin(elem) => elem.collect_all_attributes(),
            MeasureChild::Tempo(elem) => elem.collect_all_attributes(),
            MeasureChild::Slur(elem) => elem.collect_all_attributes(),
            MeasureChild::Tie(elem) => elem.collect_all_attributes(),
            MeasureChild::Fermata(elem) => elem.collect_all_attributes(),
            MeasureChild::Trill(elem) => elem.collect_all_attributes(),
            MeasureChild::Mordent(elem) => elem.collect_all_attributes(),
            MeasureChild::Harm(elem) => elem.collect_all_attributes(),
            MeasureChild::Pedal(elem) => elem.collect_all_attributes(),
            MeasureChild::Arpeg(elem) => elem.collect_all_attributes(),
            MeasureChild::TupletSpan(elem) => elem.collect_all_attributes(),
            MeasureChild::Reh(elem) => elem.collect_all_attributes(),
            MeasureChild::BeamSpan(elem) => elem.collect_all_attributes(),
            MeasureChild::Octave(elem) => elem.collect_all_attributes(),
            MeasureChild::Gliss(elem) => elem.collect_all_attributes(),
            MeasureChild::Lv(elem) => elem.collect_all_attributes(),
            MeasureChild::BracketSpan(elem) => elem.collect_all_attributes(),
            MeasureChild::Fing(elem) => elem.collect_all_attributes(),
            MeasureChild::Phrase(elem) => elem.collect_all_attributes(),
            MeasureChild::Line(elem) => elem.collect_all_attributes(),
            MeasureChild::Sb(elem) => elem.collect_all_attributes(),
            MeasureChild::Pb(elem) => elem.collect_all_attributes(),
            MeasureChild::StaffDef(elem) => elem.collect_all_attributes(),
            // Editorial elements
            MeasureChild::Add(elem) => elem.collect_all_attributes(),
            MeasureChild::App(elem) => elem.collect_all_attributes(),
            MeasureChild::Choice(elem) => elem.collect_all_attributes(),
            MeasureChild::Corr(elem) => elem.collect_all_attributes(),
            MeasureChild::Damage(elem) => elem.collect_all_attributes(),
            MeasureChild::Del(elem) => elem.collect_all_attributes(),
            MeasureChild::Orig(elem) => elem.collect_all_attributes(),
            MeasureChild::Reg(elem) => elem.collect_all_attributes(),
            MeasureChild::Restore(elem) => elem.collect_all_attributes(),
            MeasureChild::Sic(elem) => elem.collect_all_attributes(),
            MeasureChild::Subst(elem) => elem.collect_all_attributes(),
            MeasureChild::Supplied(elem) => elem.collect_all_attributes(),
            MeasureChild::Unclear(elem) => elem.collect_all_attributes(),
            MeasureChild::Gap(elem) => elem.collect_all_attributes(),
            MeasureChild::HandShift(elem) => elem.collect_all_attributes(),
            // Other control events
            MeasureChild::Turn(elem) => elem.collect_all_attributes(),
            MeasureChild::Breath(elem) => elem.collect_all_attributes(),
            MeasureChild::Caesura(elem) => elem.collect_all_attributes(),
            MeasureChild::Bend(elem) => elem.collect_all_attributes(),
            MeasureChild::FingGrp(elem) => elem.collect_all_attributes(),
            MeasureChild::Ornam(elem) => elem.collect_all_attributes(),
            MeasureChild::RepeatMark(elem) => elem.collect_all_attributes(),
            MeasureChild::HarpPedal(elem) => elem.collect_all_attributes(),
            MeasureChild::AnchoredText(elem) => elem.collect_all_attributes(),
            MeasureChild::Curve(elem) => elem.collect_all_attributes(),
            MeasureChild::Midi(elem) => elem.collect_all_attributes(),
            MeasureChild::Attacca(elem) => elem.collect_all_attributes(),
            MeasureChild::CpMark(elem) => elem.collect_all_attributes(),
            MeasureChild::MetaMark(elem) => elem.collect_all_attributes(),
            MeasureChild::MNum(elem) => elem.collect_all_attributes(),
            MeasureChild::Ossia(elem) => elem.collect_all_attributes(),
            MeasureChild::Annot(elem) => elem.collect_all_attributes(),
            MeasureChild::Relation(elem) => elem.collect_all_attributes(),
            MeasureChild::RelationList(elem) => elem.collect_all_attributes(),
            MeasureChild::Sp(elem) => elem.collect_all_attributes(),
            MeasureChild::StageDir(elem) => elem.collect_all_attributes(),
            MeasureChild::Cb(elem) => elem.collect_all_attributes(),
            MeasureChild::ColLayout(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MeasureChild::Staff(elem) => elem.has_children(),
            MeasureChild::Dynam(elem) => elem.has_children(),
            MeasureChild::Dir(elem) => elem.has_children(),
            MeasureChild::Hairpin(_) => false,
            MeasureChild::Tempo(elem) => elem.has_children(),
            MeasureChild::Slur(_) => false,
            MeasureChild::Tie(_) => false,
            MeasureChild::Fermata(_) => false,
            MeasureChild::Trill(_) => false,
            MeasureChild::Mordent(_) => false,
            MeasureChild::Harm(elem) => elem.has_children(),
            MeasureChild::Pedal(_) => false,
            MeasureChild::Arpeg(_) => false,
            MeasureChild::TupletSpan(_) => false,
            MeasureChild::Reh(elem) => elem.has_children(),
            MeasureChild::BeamSpan(_) => false,
            MeasureChild::Octave(elem) => elem.has_children(),
            MeasureChild::Gliss(elem) => elem.has_children(),
            MeasureChild::Lv(elem) => elem.has_children(),
            MeasureChild::BracketSpan(elem) => elem.has_children(),
            MeasureChild::Fing(elem) => elem.has_children(),
            MeasureChild::Phrase(elem) => elem.has_children(),
            MeasureChild::Line(elem) => elem.has_children(),
            MeasureChild::Sb(_) => false,
            MeasureChild::Pb(elem) => elem.has_children(),
            MeasureChild::StaffDef(elem) => elem.has_children(),
            // Editorial elements
            MeasureChild::Add(elem) => elem.has_children(),
            MeasureChild::App(elem) => elem.has_children(),
            MeasureChild::Choice(elem) => elem.has_children(),
            MeasureChild::Corr(elem) => elem.has_children(),
            MeasureChild::Damage(elem) => elem.has_children(),
            MeasureChild::Del(elem) => elem.has_children(),
            MeasureChild::Orig(elem) => elem.has_children(),
            MeasureChild::Reg(elem) => elem.has_children(),
            MeasureChild::Restore(elem) => elem.has_children(),
            MeasureChild::Sic(elem) => elem.has_children(),
            MeasureChild::Subst(elem) => elem.has_children(),
            MeasureChild::Supplied(elem) => elem.has_children(),
            MeasureChild::Unclear(elem) => elem.has_children(),
            MeasureChild::Gap(_) => false,
            MeasureChild::HandShift(_) => false,
            // Other control events
            MeasureChild::Turn(_) => false,
            MeasureChild::Breath(_) => false,
            MeasureChild::Caesura(_) => false,
            MeasureChild::Bend(elem) => elem.has_children(),
            MeasureChild::FingGrp(elem) => elem.has_children(),
            MeasureChild::Ornam(elem) => elem.has_children(),
            MeasureChild::RepeatMark(elem) => elem.has_children(),
            MeasureChild::HarpPedal(_) => false,
            MeasureChild::AnchoredText(elem) => elem.has_children(),
            MeasureChild::Curve(_) => false,
            MeasureChild::Midi(elem) => elem.has_children(),
            MeasureChild::Attacca(elem) => elem.has_children(),
            MeasureChild::CpMark(elem) => elem.has_children(),
            MeasureChild::MetaMark(elem) => elem.has_children(),
            MeasureChild::MNum(elem) => elem.has_children(),
            MeasureChild::Ossia(elem) => elem.has_children(),
            MeasureChild::Annot(elem) => elem.has_children(),
            MeasureChild::Relation(elem) => elem.has_children(),
            MeasureChild::RelationList(elem) => elem.has_children(),
            MeasureChild::Sp(elem) => elem.has_children(),
            MeasureChild::StageDir(elem) => elem.has_children(),
            MeasureChild::Cb(_) => false,
            MeasureChild::ColLayout(elem) => elem.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MeasureChild::Staff(elem) => elem.serialize_children(writer),
            MeasureChild::Dynam(elem) => elem.serialize_children(writer),
            MeasureChild::Dir(elem) => elem.serialize_children(writer),
            MeasureChild::Tempo(elem) => elem.serialize_children(writer),
            MeasureChild::Fermata(_) => Ok(()),
            MeasureChild::Trill(_) => Ok(()),
            MeasureChild::Mordent(_) => Ok(()),
            MeasureChild::Harm(elem) => elem.serialize_children(writer),
            MeasureChild::Pedal(_) => Ok(()),
            MeasureChild::Arpeg(_) => Ok(()),
            MeasureChild::TupletSpan(_) => Ok(()),
            MeasureChild::Reh(elem) => elem.serialize_children(writer),
            MeasureChild::BeamSpan(_) => Ok(()),
            MeasureChild::Octave(elem) => elem.serialize_children(writer),
            MeasureChild::Gliss(elem) => elem.serialize_children(writer),
            MeasureChild::Lv(elem) => elem.serialize_children(writer),
            MeasureChild::BracketSpan(elem) => elem.serialize_children(writer),
            MeasureChild::Fing(elem) => elem.serialize_children(writer),
            MeasureChild::Phrase(elem) => elem.serialize_children(writer),
            MeasureChild::Line(elem) => elem.serialize_children(writer),
            MeasureChild::Sb(_) => Ok(()),
            MeasureChild::Pb(elem) => elem.serialize_children(writer),
            MeasureChild::StaffDef(elem) => elem.serialize_children(writer),
            MeasureChild::Hairpin(_) => Ok(()),
            MeasureChild::Slur(_) => Ok(()),
            MeasureChild::Tie(_) => Ok(()),
            // Editorial elements
            MeasureChild::Add(elem) => elem.serialize_children(writer),
            MeasureChild::App(elem) => elem.serialize_children(writer),
            MeasureChild::Choice(elem) => elem.serialize_children(writer),
            MeasureChild::Corr(elem) => elem.serialize_children(writer),
            MeasureChild::Damage(elem) => elem.serialize_children(writer),
            MeasureChild::Del(elem) => elem.serialize_children(writer),
            MeasureChild::Orig(elem) => elem.serialize_children(writer),
            MeasureChild::Reg(elem) => elem.serialize_children(writer),
            MeasureChild::Restore(elem) => elem.serialize_children(writer),
            MeasureChild::Sic(elem) => elem.serialize_children(writer),
            MeasureChild::Subst(elem) => elem.serialize_children(writer),
            MeasureChild::Supplied(elem) => elem.serialize_children(writer),
            MeasureChild::Unclear(elem) => elem.serialize_children(writer),
            MeasureChild::Gap(_) => Ok(()),
            MeasureChild::HandShift(_) => Ok(()),
            // Other control events
            MeasureChild::Turn(_) => Ok(()),
            MeasureChild::Breath(_) => Ok(()),
            MeasureChild::Caesura(_) => Ok(()),
            MeasureChild::Bend(elem) => elem.serialize_children(writer),
            MeasureChild::FingGrp(elem) => elem.serialize_children(writer),
            MeasureChild::Ornam(elem) => elem.serialize_children(writer),
            MeasureChild::RepeatMark(elem) => elem.serialize_children(writer),
            MeasureChild::HarpPedal(_) => Ok(()),
            MeasureChild::AnchoredText(elem) => elem.serialize_children(writer),
            MeasureChild::Curve(_) => Ok(()),
            MeasureChild::Midi(elem) => elem.serialize_children(writer),
            MeasureChild::Attacca(elem) => elem.serialize_children(writer),
            MeasureChild::CpMark(elem) => elem.serialize_children(writer),
            MeasureChild::MetaMark(elem) => elem.serialize_children(writer),
            MeasureChild::MNum(elem) => elem.serialize_children(writer),
            MeasureChild::Ossia(elem) => elem.serialize_children(writer),
            MeasureChild::Annot(elem) => elem.serialize_children(writer),
            MeasureChild::Relation(elem) => elem.serialize_children(writer),
            MeasureChild::RelationList(elem) => elem.serialize_children(writer),
            MeasureChild::Sp(elem) => elem.serialize_children(writer),
            MeasureChild::StageDir(elem) => elem.serialize_children(writer),
            MeasureChild::Cb(_) => Ok(()),
            MeasureChild::ColLayout(elem) => elem.serialize_children(writer),
        }
    }
}

// ============================================================================
// Section element implementation
// ============================================================================

impl MeiSerialize for Section {
    fn element_name(&self) -> &'static str {
        "section"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs.extend(self.section_log.collect_attributes());
        attrs.extend(self.section_ges.collect_attributes());
        attrs.extend(self.section_vis.collect_attributes());
        attrs.extend(self.section_anl.collect_attributes());
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

// ============================================================================
// Sb (system break) element implementation
// ============================================================================

impl MeiSerialize for Sb {
    fn element_name(&self) -> &'static str {
        "sb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        attrs.extend(self.sb_log.collect_attributes());
        attrs.extend(self.sb_ges.collect_attributes());
        attrs.extend(self.sb_vis.collect_attributes());
        attrs.extend(self.sb_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        // Sb is an empty element
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Sb has no children
        Ok(())
    }
}

// ============================================================================
// Pb (page break) element implementation
// ============================================================================

impl MeiSerialize for Pb {
    fn element_name(&self) -> &'static str {
        "pb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        attrs.extend(self.pb_log.collect_attributes());
        attrs.extend(self.pb_ges.collect_attributes());
        attrs.extend(self.pb_vis.collect_attributes());
        attrs.extend(self.pb_anl.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        // Pb can have children (pgFoot, pgDesc, pgHead) but we're not serializing them yet
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Pb children (pgFoot, pgDesc, pgHead) not yet implemented
        Ok(())
    }
}

// ============================================================================
// Ending element implementation
// ============================================================================

impl MeiSerialize for Ending {
    fn element_name(&self) -> &'static str {
        "ending"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs.extend(self.ending_log.collect_attributes());
        attrs.extend(self.ending_ges.collect_attributes());
        attrs.extend(self.ending_vis.collect_attributes());
        attrs.extend(self.ending_anl.collect_attributes());
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

impl MeiSerialize for EndingChild {
    fn element_name(&self) -> &'static str {
        match self {
            EndingChild::Measure(_) => "measure",
            EndingChild::Staff(_) => "staff",
            EndingChild::Section(_) => "section",
            EndingChild::ScoreDef(_) => "scoreDef",
            EndingChild::StaffDef(_) => "staffDef",
            EndingChild::Sb(_) => "sb",
            EndingChild::Pb(_) => "pb",
            EndingChild::Cb(_) => "cb",
            EndingChild::Annot(_) => "annot",
            EndingChild::App(_) => "app",
            EndingChild::Choice(_) => "choice",
            EndingChild::Orig(_) => "orig",
            EndingChild::Reg(_) => "reg",
            EndingChild::Sic(_) => "sic",
            EndingChild::Corr(_) => "corr",
            EndingChild::Add(_) => "add",
            EndingChild::Del(_) => "del",
            EndingChild::Subst(_) => "subst",
            EndingChild::Supplied(_) => "supplied",
            EndingChild::Unclear(_) => "unclear",
            EndingChild::Damage(_) => "damage",
            EndingChild::Gap(_) => "gap",
            EndingChild::Restore(_) => "restore",
            EndingChild::AnchoredText(_) => "anchoredText",
            EndingChild::ColLayout(_) => "colLayout",
            EndingChild::Curve(_) => "curve",
            EndingChild::Expansion(_) => "expansion",
            EndingChild::HandShift(_) => "handShift",
            EndingChild::Line(_) => "line",
            EndingChild::Relation(_) => "relation",
            EndingChild::RelationList(_) => "relationList",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            EndingChild::Measure(measure) => measure.collect_all_attributes(),
            EndingChild::Staff(staff) => staff.collect_all_attributes(),
            EndingChild::Section(section) => section.collect_all_attributes(),
            EndingChild::ScoreDef(score_def) => score_def.collect_all_attributes(),
            EndingChild::StaffDef(staff_def) => staff_def.collect_all_attributes(),
            EndingChild::Sb(sb) => sb.collect_all_attributes(),
            EndingChild::Pb(pb) => pb.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            EndingChild::Measure(measure) => measure.has_children(),
            EndingChild::Staff(staff) => staff.has_children(),
            EndingChild::Section(section) => section.has_children(),
            EndingChild::ScoreDef(score_def) => score_def.has_children(),
            EndingChild::StaffDef(staff_def) => staff_def.has_children(),
            EndingChild::Sb(sb) => sb.has_children(),
            EndingChild::Pb(pb) => pb.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EndingChild::Measure(measure) => measure.serialize_children(writer),
            EndingChild::Staff(staff) => staff.serialize_children(writer),
            EndingChild::Section(section) => section.serialize_children(writer),
            EndingChild::ScoreDef(score_def) => score_def.serialize_children(writer),
            EndingChild::StaffDef(staff_def) => staff_def.serialize_children(writer),
            EndingChild::Sb(sb) => sb.serialize_children(writer),
            EndingChild::Pb(pb) => pb.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "EndingChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

impl MeiSerialize for SectionChild {
    fn element_name(&self) -> &'static str {
        match self {
            SectionChild::Measure(_) => "measure",
            SectionChild::Staff(_) => "staff",
            SectionChild::Section(_) => "section",
            SectionChild::Expansion(_) => "expansion",
            SectionChild::Subst(_) => "subst",
            SectionChild::App(_) => "app",
            SectionChild::Ending(_) => "ending",
            SectionChild::Sb(_) => "sb",
            SectionChild::AnchoredText(_) => "anchoredText",
            SectionChild::Orig(_) => "orig",
            SectionChild::ScoreDef(_) => "scoreDef",
            SectionChild::Relation(_) => "relation",
            SectionChild::Annot(_) => "annot",
            SectionChild::Choice(_) => "choice",
            SectionChild::Add(_) => "add",
            SectionChild::Sic(_) => "sic",
            SectionChild::Reg(_) => "reg",
            SectionChild::Damage(_) => "damage",
            SectionChild::Curve(_) => "curve",
            SectionChild::Cb(_) => "cb",
            SectionChild::ColLayout(_) => "colLayout",
            SectionChild::Unclear(_) => "unclear",
            SectionChild::Pb(_) => "pb",
            SectionChild::Div(_) => "div",
            SectionChild::Gap(_) => "gap",
            SectionChild::Del(_) => "del",
            SectionChild::Line(_) => "line",
            SectionChild::HandShift(_) => "handShift",
            SectionChild::Restore(_) => "restore",
            SectionChild::StaffDef(_) => "staffDef",
            SectionChild::RelationList(_) => "relationList",
            SectionChild::Supplied(_) => "supplied",
            SectionChild::Corr(_) => "corr",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            SectionChild::Measure(measure) => measure.collect_all_attributes(),
            SectionChild::Staff(staff) => staff.collect_all_attributes(),
            SectionChild::Section(section) => section.collect_all_attributes(),
            SectionChild::ScoreDef(score_def) => score_def.collect_all_attributes(),
            SectionChild::Sb(sb) => sb.collect_all_attributes(),
            SectionChild::Pb(pb) => pb.collect_all_attributes(),
            SectionChild::Div(div) => div.collect_all_attributes(),
            SectionChild::StaffDef(staff_def) => staff_def.collect_all_attributes(),
            SectionChild::Ending(ending) => ending.collect_all_attributes(),
            SectionChild::Annot(annot) => annot.collect_all_attributes(),
            // Other child types not yet implemented - return empty
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            SectionChild::Measure(measure) => measure.has_children(),
            SectionChild::Staff(staff) => staff.has_children(),
            SectionChild::Section(section) => section.has_children(),
            SectionChild::ScoreDef(score_def) => score_def.has_children(),
            SectionChild::Sb(sb) => sb.has_children(),
            SectionChild::Pb(pb) => pb.has_children(),
            SectionChild::Div(div) => div.has_children(),
            SectionChild::StaffDef(staff_def) => staff_def.has_children(),
            SectionChild::Ending(ending) => ending.has_children(),
            SectionChild::Annot(annot) => annot.has_children(),
            // Other child types - assume no children for now
            _ => false,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SectionChild::Measure(measure) => measure.serialize_children(writer),
            SectionChild::Staff(staff) => staff.serialize_children(writer),
            SectionChild::Section(section) => section.serialize_children(writer),
            SectionChild::ScoreDef(score_def) => score_def.serialize_children(writer),
            SectionChild::Sb(sb) => sb.serialize_children(writer),
            SectionChild::Pb(pb) => pb.serialize_children(writer),
            SectionChild::Div(div) => div.serialize_children(writer),
            SectionChild::StaffDef(staff_def) => staff_def.serialize_children(writer),
            SectionChild::Ending(ending) => ending.serialize_children(writer),
            SectionChild::Annot(annot) => annot.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "SectionChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Body element implementation
// ============================================================================

impl MeiSerialize for Body {
    fn element_name(&self) -> &'static str {
        "body"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for BodyChild {
    fn element_name(&self) -> &'static str {
        match self {
            BodyChild::Div(_) => "div",
            BodyChild::Mdiv(_) => "mdiv",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            BodyChild::Div(_) => Vec::new(), // Div not fully implemented yet
            BodyChild::Mdiv(mdiv) => mdiv.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            BodyChild::Div(_) => true,
            BodyChild::Mdiv(mdiv) => mdiv.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            BodyChild::Div(_) => Ok(()), // Div not fully implemented yet
            BodyChild::Mdiv(mdiv) => mdiv.serialize_children(writer),
        }
    }
}

// ============================================================================
// Score element implementation
// ============================================================================

impl MeiSerialize for Score {
    fn element_name(&self) -> &'static str {
        "score"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        // score_anl, score_ges, score_log, score_vis have no serializers yet - add empty
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

impl MeiSerialize for ScoreChild {
    fn element_name(&self) -> &'static str {
        match self {
            ScoreChild::Section(_) => "section",
            ScoreChild::ScoreDef(_) => "scoreDef",
            ScoreChild::StaffDef(_) => "staffDef",
            ScoreChild::Ending(_) => "ending",
            ScoreChild::Pb(_) => "pb",
            ScoreChild::Sb(_) => "sb",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            ScoreChild::Section(s) => s.collect_all_attributes(),
            ScoreChild::ScoreDef(s) => s.collect_all_attributes(),
            ScoreChild::StaffDef(s) => s.collect_all_attributes(),
            ScoreChild::Ending(e) => e.collect_all_attributes(),
            ScoreChild::Pb(p) => p.collect_all_attributes(),
            ScoreChild::Sb(s) => s.collect_all_attributes(),
            _ => Vec::new(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            ScoreChild::Section(s) => s.has_children(),
            ScoreChild::ScoreDef(s) => s.has_children(),
            ScoreChild::StaffDef(s) => s.has_children(),
            ScoreChild::Ending(e) => e.has_children(),
            ScoreChild::Pb(p) => p.has_children(),
            ScoreChild::Sb(s) => s.has_children(),
            _ => true,
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ScoreChild::Section(s) => s.serialize_children(writer),
            ScoreChild::ScoreDef(s) => s.serialize_children(writer),
            ScoreChild::StaffDef(s) => s.serialize_children(writer),
            ScoreChild::Ending(e) => e.serialize_children(writer),
            ScoreChild::Pb(p) => p.serialize_children(writer),
            ScoreChild::Sb(s) => s.serialize_children(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ScoreChild::{}::serialize_children",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Mdiv element implementation
// ============================================================================

impl MeiSerialize for Mdiv {
    fn element_name(&self) -> &'static str {
        "mdiv"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.mdiv_log.collect_attributes());
        attrs.extend(self.mdiv_ges.collect_attributes());
        attrs.extend(self.mdiv_vis.collect_attributes());
        attrs.extend(self.mdiv_anl.collect_attributes());
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

impl MeiSerialize for MdivChild {
    fn element_name(&self) -> &'static str {
        match self {
            MdivChild::Mdiv(_) => "mdiv",
            MdivChild::Score(_) => "score",
            MdivChild::Parts(_) => "parts",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            MdivChild::Mdiv(mdiv) => mdiv.collect_all_attributes(),
            MdivChild::Score(score) => score.collect_all_attributes(),
            MdivChild::Parts(parts) => parts.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MdivChild::Mdiv(mdiv) => mdiv.has_children(),
            MdivChild::Score(score) => score.has_children(),
            MdivChild::Parts(parts) => parts.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MdivChild::Mdiv(mdiv) => mdiv.serialize_children(writer),
            MdivChild::Score(score) => score.serialize_children(writer),
            MdivChild::Parts(parts) => parts.serialize_children(writer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer::MeiSerialize;

    // ============================================================================
    // Mdiv serialization tests
    // ============================================================================

    #[test]
    fn mdiv_serializes_to_mei_xml() {
        let mut mdiv = Mdiv::default();
        mdiv.common.xml_id = Some("m1".to_string());

        let xml = mdiv.to_mei_string().expect("should serialize");

        assert!(xml.contains("<mdiv"), "should have mdiv element: {}", xml);
        assert!(xml.contains("xml:id=\"m1\""), "should have xml:id: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn empty_mdiv_serializes_minimal() {
        let mdiv = Mdiv::default();
        let xml = mdiv.to_mei_string().expect("should serialize");

        assert!(xml.contains("<mdiv"), "should have mdiv element: {}", xml);
        assert!(xml.contains("/>"), "should be self-closing: {}", xml);
    }

    #[test]
    fn mdiv_serializes_with_label() {
        let mut mdiv = Mdiv::default();
        mdiv.common.xml_id = Some("m1".to_string());
        mdiv.common.label = Some("Movement 1".to_string());

        let xml = mdiv.to_mei_string().expect("should serialize");

        assert!(
            xml.contains("label=\"Movement 1\""),
            "should have label: {}",
            xml
        );
    }

    #[test]
    fn mdiv_serializes_with_nested_mdiv() {
        let mut mdiv = Mdiv::default();
        mdiv.common.xml_id = Some("m1".to_string());

        // Add nested mdiv
        let mut nested = Mdiv::default();
        nested.common.xml_id = Some("m1a".to_string());
        mdiv.children.push(MdivChild::Mdiv(Box::new(nested)));

        let xml = mdiv.to_mei_string().expect("should serialize");

        assert!(xml.contains("<mdiv"), "should have mdiv element: {}", xml);
        assert!(xml.contains("</mdiv>"), "should have closing tag: {}", xml);
        assert!(
            xml.contains("xml:id=\"m1a\""),
            "should have nested mdiv: {}",
            xml
        );
    }

    #[test]
    fn mdiv_roundtrip_serialization_deserialization() {
        use crate::deserializer::MeiDeserialize;

        // Create an mdiv
        let mut original = Mdiv::default();
        original.common.xml_id = Some("m1".to_string());
        original.common.label = Some("Movement 1".to_string());

        // Add nested mdiv
        let mut nested = Mdiv::default();
        nested.common.xml_id = Some("m1a".to_string());
        original.children.push(MdivChild::Mdiv(Box::new(nested)));

        // Serialize
        let xml = original.to_mei_string().expect("should serialize");

        // Deserialize
        let parsed = Mdiv::from_mei_str(&xml).expect("should deserialize");

        // Compare
        assert_eq!(original.common.xml_id, parsed.common.xml_id);
        assert_eq!(original.common.label, parsed.common.label);
        assert_eq!(original.children.len(), parsed.children.len());

        // Check nested mdiv
        match (&original.children[0], &parsed.children[0]) {
            (MdivChild::Mdiv(orig_nested), MdivChild::Mdiv(parsed_nested)) => {
                assert_eq!(orig_nested.common.xml_id, parsed_nested.common.xml_id);
            }
            _ => panic!("Expected nested Mdiv"),
        }
    }
}

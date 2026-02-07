//! Serializer implementations for mensural notation MEI elements.
//!
//! This module contains implementations for Mensur, Mensuration, Proport, and Ligature
//! elements used in early music notation.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttLigatureAnl, AttLigatureGes, AttLigatureLog, AttLigatureVis, AttMensurAnl, AttMensurGes,
    AttMensurLog, AttMensurVis, AttProportAnl, AttProportGes, AttProportLog, AttProportVis,
};
use tusk_model::elements::{
    Ligature, LigatureChild, Mensur, Mensuration, MensurationChild, Proport,
};

use super::push_attr;

// ============================================================================
// Mensur attribute class implementations (Log/Vis already done elsewhere)
// ============================================================================

// ============================================================================
// Ligature attribute class implementations
// ============================================================================

// ============================================================================
// Proport attribute class implementations
// ============================================================================

// ============================================================================
// Mensur element implementation
// ============================================================================

impl MeiSerialize for Mensur {
    fn element_name(&self) -> &'static str {
        "mensur"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.mensur_log.collect_attributes());
        attrs.extend(self.mensur_vis.collect_attributes());
        attrs.extend(self.mensur_ges.collect_attributes());
        attrs.extend(self.mensur_anl.collect_attributes());
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
// Mensuration element implementation
// ============================================================================

impl MeiSerialize for Mensuration {
    fn element_name(&self) -> &'static str {
        "mensuration"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.mensur_log.collect_attributes());
        attrs.extend(self.mensur_vis.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                MensurationChild::Text(text) => {
                    writer.write_text(text)?;
                }
            }
        }
        Ok(())
    }
}

// ============================================================================
// Proport element implementation
// ============================================================================

impl MeiSerialize for Proport {
    fn element_name(&self) -> &'static str {
        "proport"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.proport_log.collect_attributes());
        attrs.extend(self.proport_vis.collect_attributes());
        attrs.extend(self.proport_ges.collect_attributes());
        attrs.extend(self.proport_anl.collect_attributes());
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
// Ligature element implementation
// ============================================================================

impl MeiSerialize for Ligature {
    fn element_name(&self) -> &'static str {
        "ligature"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.ligature_log.collect_attributes());
        attrs.extend(self.ligature_vis.collect_attributes());
        attrs.extend(self.ligature_ges.collect_attributes());
        attrs.extend(self.ligature_anl.collect_attributes());
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

impl MeiSerialize for LigatureChild {
    fn element_name(&self) -> &'static str {
        match self {
            LigatureChild::Note(_) => "note",
            LigatureChild::Rest(_) => "rest",
            LigatureChild::Dot(_) => "dot",
            LigatureChild::Space(_) => "space",
            LigatureChild::Chord(_) => "chord",
            LigatureChild::Clef(_) => "clef",
            LigatureChild::ClefGrp(_) => "clefGrp",
            LigatureChild::KeySig(_) => "keySig",
            LigatureChild::MeterSig(_) => "meterSig",
            LigatureChild::MeterSigGrp(_) => "meterSigGrp",
            LigatureChild::Mensur(_) => "mensur",
            LigatureChild::Proport(_) => "proport",
            LigatureChild::Ligature(_) => "ligature",
            LigatureChild::Neume(_) => "neume",
            LigatureChild::BarLine(_) => "barLine",
            LigatureChild::Custos(_) => "custos",
            LigatureChild::Pad(_) => "pad",
            LigatureChild::DivLine(_) => "divLine",
            LigatureChild::TabGrp(_) => "tabGrp",
            LigatureChild::TabDurSym(_) => "tabDurSym",
            // Editorial elements
            LigatureChild::Add(_) => "add",
            LigatureChild::App(_) => "app",
            LigatureChild::Choice(_) => "choice",
            LigatureChild::Corr(_) => "corr",
            LigatureChild::Damage(_) => "damage",
            LigatureChild::Del(_) => "del",
            LigatureChild::Gap(_) => "gap",
            LigatureChild::HandShift(_) => "handShift",
            LigatureChild::Orig(_) => "orig",
            LigatureChild::Reg(_) => "reg",
            LigatureChild::Restore(_) => "restore",
            LigatureChild::Sic(_) => "sic",
            LigatureChild::Subst(_) => "subst",
            LigatureChild::Supplied(_) => "supplied",
            LigatureChild::Unclear(_) => "unclear",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            LigatureChild::Note(e) => e.collect_all_attributes(),
            LigatureChild::Rest(e) => e.collect_all_attributes(),
            LigatureChild::Dot(e) => e.collect_all_attributes(),
            LigatureChild::Space(e) => e.collect_all_attributes(),
            LigatureChild::Chord(e) => e.collect_all_attributes(),
            LigatureChild::Clef(e) => e.collect_all_attributes(),
            LigatureChild::ClefGrp(e) => e.collect_all_attributes(),
            LigatureChild::KeySig(e) => e.collect_all_attributes(),
            LigatureChild::MeterSig(e) => e.collect_all_attributes(),
            LigatureChild::MeterSigGrp(e) => e.collect_all_attributes(),
            LigatureChild::Mensur(e) => e.collect_all_attributes(),
            LigatureChild::Proport(e) => e.collect_all_attributes(),
            LigatureChild::Ligature(e) => e.collect_all_attributes(),
            LigatureChild::Neume(e) => e.collect_all_attributes(),
            LigatureChild::BarLine(e) => e.collect_all_attributes(),
            LigatureChild::Custos(e) => e.collect_all_attributes(),
            LigatureChild::Pad(e) => e.collect_all_attributes(),
            LigatureChild::DivLine(e) => e.collect_all_attributes(),
            LigatureChild::TabGrp(e) => e.collect_all_attributes(),
            LigatureChild::TabDurSym(e) => e.collect_all_attributes(),
            // Editorial elements
            LigatureChild::Add(e) => e.collect_all_attributes(),
            LigatureChild::App(e) => e.collect_all_attributes(),
            LigatureChild::Choice(e) => e.collect_all_attributes(),
            LigatureChild::Corr(e) => e.collect_all_attributes(),
            LigatureChild::Damage(e) => e.collect_all_attributes(),
            LigatureChild::Del(e) => e.collect_all_attributes(),
            LigatureChild::Gap(e) => e.collect_all_attributes(),
            LigatureChild::HandShift(e) => e.collect_all_attributes(),
            LigatureChild::Orig(e) => e.collect_all_attributes(),
            LigatureChild::Reg(e) => e.collect_all_attributes(),
            LigatureChild::Restore(e) => e.collect_all_attributes(),
            LigatureChild::Sic(e) => e.collect_all_attributes(),
            LigatureChild::Subst(e) => e.collect_all_attributes(),
            LigatureChild::Supplied(e) => e.collect_all_attributes(),
            LigatureChild::Unclear(e) => e.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            LigatureChild::Note(e) => e.has_children(),
            LigatureChild::Rest(e) => e.has_children(),
            LigatureChild::Dot(_) => false,
            LigatureChild::Space(_) => false,
            LigatureChild::Chord(e) => e.has_children(),
            LigatureChild::Clef(_) => false,
            LigatureChild::ClefGrp(e) => e.has_children(),
            LigatureChild::KeySig(e) => e.has_children(),
            LigatureChild::MeterSig(_) => false,
            LigatureChild::MeterSigGrp(e) => e.has_children(),
            LigatureChild::Mensur(_) => false,
            LigatureChild::Proport(_) => false,
            LigatureChild::Ligature(e) => e.has_children(),
            LigatureChild::Neume(e) => e.has_children(),
            LigatureChild::BarLine(_) => false,
            LigatureChild::Custos(_) => false,
            LigatureChild::Pad(_) => false,
            LigatureChild::DivLine(_) => false,
            LigatureChild::TabGrp(e) => e.has_children(),
            LigatureChild::TabDurSym(_) => false,
            // Editorial elements
            LigatureChild::Add(e) => e.has_children(),
            LigatureChild::App(e) => e.has_children(),
            LigatureChild::Choice(e) => e.has_children(),
            LigatureChild::Corr(e) => e.has_children(),
            LigatureChild::Damage(e) => e.has_children(),
            LigatureChild::Del(e) => e.has_children(),
            LigatureChild::Gap(_) => false,
            LigatureChild::HandShift(_) => false,
            LigatureChild::Orig(e) => e.has_children(),
            LigatureChild::Reg(e) => e.has_children(),
            LigatureChild::Restore(e) => e.has_children(),
            LigatureChild::Sic(e) => e.has_children(),
            LigatureChild::Subst(e) => e.has_children(),
            LigatureChild::Supplied(e) => e.has_children(),
            LigatureChild::Unclear(e) => e.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            LigatureChild::Note(e) => e.serialize_children(writer),
            LigatureChild::Rest(e) => e.serialize_children(writer),
            LigatureChild::Dot(_) => Ok(()),
            LigatureChild::Space(_) => Ok(()),
            LigatureChild::Chord(e) => e.serialize_children(writer),
            LigatureChild::Clef(_) => Ok(()),
            LigatureChild::ClefGrp(e) => e.serialize_children(writer),
            LigatureChild::KeySig(e) => e.serialize_children(writer),
            LigatureChild::MeterSig(_) => Ok(()),
            LigatureChild::MeterSigGrp(e) => e.serialize_children(writer),
            LigatureChild::Mensur(_) => Ok(()),
            LigatureChild::Proport(_) => Ok(()),
            LigatureChild::Ligature(e) => e.serialize_children(writer),
            LigatureChild::Neume(e) => e.serialize_children(writer),
            LigatureChild::BarLine(_) => Ok(()),
            LigatureChild::Custos(_) => Ok(()),
            LigatureChild::Pad(_) => Ok(()),
            LigatureChild::DivLine(_) => Ok(()),
            LigatureChild::TabGrp(e) => e.serialize_children(writer),
            LigatureChild::TabDurSym(_) => Ok(()),
            // Editorial elements
            LigatureChild::Add(e) => e.serialize_children(writer),
            LigatureChild::App(e) => e.serialize_children(writer),
            LigatureChild::Choice(e) => e.serialize_children(writer),
            LigatureChild::Corr(e) => e.serialize_children(writer),
            LigatureChild::Damage(e) => e.serialize_children(writer),
            LigatureChild::Del(e) => e.serialize_children(writer),
            LigatureChild::Gap(_) => Ok(()),
            LigatureChild::HandShift(_) => Ok(()),
            LigatureChild::Orig(e) => e.serialize_children(writer),
            LigatureChild::Reg(e) => e.serialize_children(writer),
            LigatureChild::Restore(e) => e.serialize_children(writer),
            LigatureChild::Sic(e) => e.serialize_children(writer),
            LigatureChild::Subst(e) => e.serialize_children(writer),
            LigatureChild::Supplied(e) => e.serialize_children(writer),
            LigatureChild::Unclear(e) => e.serialize_children(writer),
        }
    }
}

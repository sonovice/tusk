//! Serializer implementations for editorial MEI elements.
//!
//! This module contains implementations for App, Lem, Rdg, Choice, Corr, Sic, Add, Del,
//! Abbr, Expan, Orig, Reg, Subst, Supplied, Unclear, Damage, Gap, Restore, HandShift
//! and related attribute classes.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttAgentIdent, AttCrit, AttExtent, AttHandIdent, AttMedium, AttRdgAnl, AttRdgGes, AttRdgLog,
    AttRdgVis, AttReasonIdent, AttTrans,
};
use tusk_model::elements::{
    Abbr, Add, AddChild, App, AppChild, Choice, ChoiceChild, Corr, CorrChild, Damage, Del,
    DelChild, Expan, Gap, HandShift, Lem, Orig, Rdg, Reg, Restore, Sic, SicChild, Space, Subst,
    Supplied, Unclear,
};

use super::{push_attr, serialize_vec_serde, to_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// App element implementation
// ============================================================================

impl MeiSerialize for App {
    fn element_name(&self) -> &'static str {
        "app"
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
            match child {
                AppChild::Lem(elem) => elem.serialize_mei(writer)?,
                AppChild::Rdg(elem) => elem.serialize_mei(writer)?,
            }
        }
        Ok(())
    }
}

// ============================================================================
// Lem element implementation
// ============================================================================

impl MeiSerialize for Lem {
    fn element_name(&self) -> &'static str {
        "lem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.crit.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.rdg_log.collect_attributes());
        attrs.extend(self.rdg_vis.collect_attributes());
        attrs.extend(self.rdg_ges.collect_attributes());
        attrs.extend(self.rdg_anl.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Lem can contain many children - serialize them using the dynamic dispatch
        for child in &self.children {
            serialize_lem_child(child, writer)?;
        }
        Ok(())
    }
}

// ============================================================================
// Rdg element implementation
// ============================================================================

impl MeiSerialize for Rdg {
    fn element_name(&self) -> &'static str {
        "rdg"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.crit.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.rdg_log.collect_attributes());
        attrs.extend(self.rdg_vis.collect_attributes());
        attrs.extend(self.rdg_ges.collect_attributes());
        attrs.extend(self.rdg_anl.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Rdg can contain many children - serialize them using dynamic dispatch
        for child in &self.children {
            serialize_rdg_child(child, writer)?;
        }
        Ok(())
    }
}

// ============================================================================
// Choice element implementation
// ============================================================================

impl MeiSerialize for Choice {
    fn element_name(&self) -> &'static str {
        "choice"
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
            match child {
                ChoiceChild::Sic(elem) => elem.serialize_mei(writer)?,
                ChoiceChild::Corr(elem) => elem.serialize_mei(writer)?,
                ChoiceChild::Choice(elem) => elem.serialize_mei(writer)?,
                // Other choice children not yet fully implemented - skip for now
                // These would need their own serializers when implemented
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Corr element implementation
// ============================================================================

impl MeiSerialize for Corr {
    fn element_name(&self) -> &'static str {
        "corr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                CorrChild::Text(text) => writer.write_text(text)?,
                // Other children would need their serializers
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Sic element implementation
// ============================================================================

impl MeiSerialize for Sic {
    fn element_name(&self) -> &'static str {
        "sic"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            match child {
                SicChild::Text(text) => writer.write_text(text)?,
                // Other children would need their serializers
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Add element implementation
// ============================================================================

impl MeiSerialize for Add {
    fn element_name(&self) -> &'static str {
        "add"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        // Handle place attribute
        push_attr!(attrs, "place", vec self.place);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            serialize_add_child(child, writer)?;
        }
        Ok(())
    }
}

// ============================================================================
// Del element implementation
// ============================================================================

impl MeiSerialize for Del {
    fn element_name(&self) -> &'static str {
        "del"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.text_rendition.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            serialize_del_child(child, writer)?;
        }
        Ok(())
    }
}

// ============================================================================
// Abbr element implementation
// ============================================================================

impl MeiSerialize for Abbr {
    fn element_name(&self) -> &'static str {
        "abbr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        push_attr!(attrs, "expan", clone self.expan);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::AbbrChild;
            match child {
                AbbrChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Expan element implementation
// ============================================================================

impl MeiSerialize for Expan {
    fn element_name(&self) -> &'static str {
        "expan"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        push_attr!(attrs, "abbr", clone self.abbr);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::ExpanChild;
            match child {
                ExpanChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Orig element implementation
// ============================================================================

impl MeiSerialize for Orig {
    fn element_name(&self) -> &'static str {
        "orig"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::OrigChild;
            match child {
                OrigChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Reg element implementation
// ============================================================================

impl MeiSerialize for Reg {
    fn element_name(&self) -> &'static str {
        "reg"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::RegChild;
            match child {
                RegChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Subst element implementation
// ============================================================================

impl MeiSerialize for Subst {
    fn element_name(&self) -> &'static str {
        "subst"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::SubstChild;
            match child {
                SubstChild::Add(elem) => elem.serialize_mei(writer)?,
                SubstChild::Del(elem) => elem.serialize_mei(writer)?,
                SubstChild::Gap(elem) => elem.serialize_mei(writer)?,
                SubstChild::Reg(elem) => elem.serialize_mei(writer)?,
                SubstChild::Sic(elem) => elem.serialize_mei(writer)?,
                SubstChild::Corr(elem) => elem.serialize_mei(writer)?,
                SubstChild::Damage(elem) => elem.serialize_mei(writer)?,
                SubstChild::HandShift(elem) => elem.serialize_mei(writer)?,
                SubstChild::Restore(elem) => elem.serialize_mei(writer)?,
                SubstChild::Unclear(elem) => elem.serialize_mei(writer)?,
                SubstChild::Orig(elem) => elem.serialize_mei(writer)?,
                SubstChild::Supplied(elem) => elem.serialize_mei(writer)?,
            }
        }
        Ok(())
    }
}

// ============================================================================
// Supplied element implementation
// ============================================================================

impl MeiSerialize for Supplied {
    fn element_name(&self) -> &'static str {
        "supplied"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.agent_ident.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.reason_ident.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::SuppliedChild;
            match child {
                SuppliedChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Unclear element implementation
// ============================================================================

impl MeiSerialize for Unclear {
    fn element_name(&self) -> &'static str {
        "unclear"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.agent_ident.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.hand_ident.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.reason_ident.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::UnclearChild;
            match child {
                UnclearChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Damage element implementation
// ============================================================================

impl MeiSerialize for Damage {
    fn element_name(&self) -> &'static str {
        "damage"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.agent_ident.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.hand_ident.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        push_attr!(attrs, "degree", clone self.degree);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::DamageChild;
            match child {
                DamageChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// Gap element implementation (empty element)
// ============================================================================

impl MeiSerialize for Gap {
    fn element_name(&self) -> &'static str {
        "gap"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.hand_ident.collect_attributes());
        attrs.extend(self.reason_ident.collect_attributes());
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // Gap is an empty element
        Ok(())
    }
}

// ============================================================================
// Restore element implementation
// ============================================================================

impl MeiSerialize for Restore {
    fn element_name(&self) -> &'static str {
        "restore"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.extent.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        push_attr!(attrs, "desc", clone self.desc);
        attrs
    }

    fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        for child in &self.children {
            use tusk_model::elements::RestoreChild;
            match child {
                RestoreChild::Text(text) => writer.write_text(text)?,
                _ => {}
            }
        }
        Ok(())
    }
}

// ============================================================================
// HandShift element implementation (empty element)
// ============================================================================

impl MeiSerialize for HandShift {
    fn element_name(&self) -> &'static str {
        "handShift"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.medium.collect_attributes());
        push_attr!(attrs, "character", clone self.character);
        push_attr!(attrs, "new", self.new);
        push_attr!(attrs, "old", self.old);
        attrs
    }

    fn has_children(&self) -> bool {
        false
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        // HandShift is an empty element
        Ok(())
    }
}

// ============================================================================
// Helper functions for child serialization
// ============================================================================

use tusk_model::elements::{LemChild, RdgChild};

/// Serialize a LemChild variant to the writer.
fn serialize_lem_child<W: Write>(
    child: &LemChild,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    match child {
        LemChild::Text(text) => writer.write_text(text),
        LemChild::Phrase(elem) => elem.serialize_mei(writer),
        LemChild::GeogFeat(elem) => elem.serialize_mei(writer),
        LemChild::Rend(elem) => elem.serialize_mei(writer),
        LemChild::P(elem) => elem.serialize_mei(writer),
        LemChild::Slur(elem) => elem.serialize_mei(writer),
        LemChild::Quilisma(elem) => elem.serialize_mei(writer),
        LemChild::Table(elem) => elem.serialize_mei(writer),
        LemChild::Corr(elem) => elem.serialize_mei(writer),
        LemChild::Space(elem) => elem.serialize_mei(writer),
        LemChild::Catchwords(elem) => elem.serialize_mei(writer),
        LemChild::Clef(elem) => elem.serialize_mei(writer),
        LemChild::MultiRpt(elem) => elem.serialize_mei(writer),
        LemChild::StageDir(elem) => elem.serialize_mei(writer),
        LemChild::Street(elem) => elem.serialize_mei(writer),
        LemChild::Curve(elem) => elem.serialize_mei(writer),
        LemChild::MeterSigGrp(elem) => elem.serialize_mei(writer),
        LemChild::Settlement(elem) => elem.serialize_mei(writer),
        LemChild::Chord(elem) => elem.serialize_mei(writer),
        LemChild::Artic(elem) => elem.serialize_mei(writer),
        LemChild::Extent(elem) => elem.serialize_mei(writer),
        LemChild::Dim(elem) => elem.serialize_mei(writer),
        LemChild::PersName(elem) => elem.serialize_mei(writer),
        LemChild::Annot(elem) => elem.serialize_mei(writer),
        LemChild::BeatRpt(elem) => elem.serialize_mei(writer),
        LemChild::HalfmRpt(elem) => elem.serialize_mei(writer),
        LemChild::Abbr(elem) => elem.serialize_mei(writer),
        LemChild::Ligature(elem) => elem.serialize_mei(writer),
        LemChild::BiblList(elem) => elem.serialize_mei(writer),
        LemChild::Hairpin(elem) => elem.serialize_mei(writer),
        LemChild::Locus(elem) => elem.serialize_mei(writer),
        LemChild::PostBox(elem) => elem.serialize_mei(writer),
        LemChild::Turn(elem) => elem.serialize_mei(writer),
        LemChild::Liquescent(elem) => elem.serialize_mei(writer),
        LemChild::Lb(elem) => elem.serialize_mei(writer),
        LemChild::HandShift(elem) => elem.serialize_mei(writer),
        LemChild::Oriscus(elem) => elem.serialize_mei(writer),
        LemChild::Term(elem) => elem.serialize_mei(writer),
        LemChild::MultiRest(elem) => elem.serialize_mei(writer),
        LemChild::GeogName(elem) => elem.serialize_mei(writer),
        LemChild::Section(elem) => elem.serialize_mei(writer),
        LemChild::CpMark(elem) => elem.serialize_mei(writer),
        LemChild::Num(elem) => elem.serialize_mei(writer),
        LemChild::ClefGrp(elem) => elem.serialize_mei(writer),
        LemChild::LocusGrp(elem) => elem.serialize_mei(writer),
        LemChild::ColLayout(elem) => elem.serialize_mei(writer),
        LemChild::Sb(elem) => elem.serialize_mei(writer),
        LemChild::Octave(elem) => elem.serialize_mei(writer),
        LemChild::Ptr(elem) => elem.serialize_mei(writer),
        LemChild::Gap(elem) => elem.serialize_mei(writer),
        LemChild::FTrem(elem) => elem.serialize_mei(writer),
        LemChild::Refrain(elem) => elem.serialize_mei(writer),
        LemChild::MRest(elem) => elem.serialize_mei(writer),
        LemChild::MSpace(elem) => elem.serialize_mei(writer),
        LemChild::App(elem) => elem.serialize_mei(writer),
        LemChild::Dot(elem) => elem.serialize_mei(writer),
        LemChild::Proport(elem) => elem.serialize_mei(writer),
        LemChild::SignifLet(elem) => elem.serialize_mei(writer),
        LemChild::CastList(elem) => elem.serialize_mei(writer),
        LemChild::Strophicus(elem) => elem.serialize_mei(writer),
        LemChild::Add(elem) => elem.serialize_mei(writer),
        LemChild::Layer(elem) => elem.serialize_mei(writer),
        LemChild::Fermata(elem) => elem.serialize_mei(writer),
        LemChild::MeterSig(elem) => elem.serialize_mei(writer),
        LemChild::Episema(elem) => elem.serialize_mei(writer),
        LemChild::CorpName(elem) => elem.serialize_mei(writer),
        LemChild::Region(elem) => elem.serialize_mei(writer),
        LemChild::Volta(elem) => elem.serialize_mei(writer),
        LemChild::Heraldry(elem) => elem.serialize_mei(writer),
        LemChild::Bibl(elem) => elem.serialize_mei(writer),
        LemChild::Caesura(elem) => elem.serialize_mei(writer),
        LemChild::Reh(elem) => elem.serialize_mei(writer),
        LemChild::Symbol(elem) => elem.serialize_mei(writer),
        LemChild::BracketSpan(elem) => elem.serialize_mei(writer),
        LemChild::District(elem) => elem.serialize_mei(writer),
        LemChild::Subst(elem) => elem.serialize_mei(writer),
        LemChild::DivLine(elem) => elem.serialize_mei(writer),
        LemChild::EventList(elem) => elem.serialize_mei(writer),
        LemChild::TabDurSym(elem) => elem.serialize_mei(writer),
        LemChild::Lg(elem) => elem.serialize_mei(writer),
        LemChild::SecFolio(elem) => elem.serialize_mei(writer),
        LemChild::Signatures(elem) => elem.serialize_mei(writer),
        LemChild::Pad(elem) => elem.serialize_mei(writer),
        LemChild::Title(elem) => elem.serialize_mei(writer),
        LemChild::Unclear(elem) => elem.serialize_mei(writer),
        LemChild::BeamSpan(elem) => elem.serialize_mei(writer),
        LemChild::Name(elem) => elem.serialize_mei(writer),
        LemChild::MRpt(elem) => elem.serialize_mei(writer),
        LemChild::Repository(elem) => elem.serialize_mei(writer),
        LemChild::Width(elem) => elem.serialize_mei(writer),
        LemChild::Fig(elem) => elem.serialize_mei(writer),
        LemChild::StaffDef(elem) => elem.serialize_mei(writer),
        LemChild::TabGrp(elem) => elem.serialize_mei(writer),
        LemChild::Expansion(elem) => elem.serialize_mei(writer),
        LemChild::Staff(elem) => elem.serialize_mei(writer),
        LemChild::Ref(elem) => elem.serialize_mei(writer),
        LemChild::BarLine(elem) => elem.serialize_mei(writer),
        LemChild::Del(elem) => elem.serialize_mei(writer),
        LemChild::Dimensions(elem) => elem.serialize_mei(writer),
        LemChild::Tuplet(elem) => elem.serialize_mei(writer),
        LemChild::Fing(elem) => elem.serialize_mei(writer),
        LemChild::Pedal(elem) => elem.serialize_mei(writer),
        LemChild::PeriodName(elem) => elem.serialize_mei(writer),
        LemChild::Date(elem) => elem.serialize_mei(writer),
        LemChild::Height(elem) => elem.serialize_mei(writer),
        LemChild::RelationList(elem) => elem.serialize_mei(writer),
        LemChild::Seg(elem) => elem.serialize_mei(writer),
        LemChild::Ending(elem) => elem.serialize_mei(writer),
        LemChild::Orig(elem) => elem.serialize_mei(writer),
        LemChild::Sic(elem) => elem.serialize_mei(writer),
        LemChild::Custos(elem) => elem.serialize_mei(writer),
        LemChild::Supplied(elem) => elem.serialize_mei(writer),
        LemChild::GraceGrp(elem) => elem.serialize_mei(writer),
        LemChild::Bloc(elem) => elem.serialize_mei(writer),
        LemChild::Midi(elem) => elem.serialize_mei(writer),
        LemChild::Neume(elem) => elem.serialize_mei(writer),
        LemChild::Dynam(elem) => elem.serialize_mei(writer),
        LemChild::F(elem) => elem.serialize_mei(writer),
        LemChild::Relation(elem) => elem.serialize_mei(writer),
        LemChild::StaffGrp(elem) => elem.serialize_mei(writer),
        LemChild::ScoreDef(elem) => elem.serialize_mei(writer),
        LemChild::Choice(elem) => elem.serialize_mei(writer),
        LemChild::Rest(elem) => elem.serialize_mei(writer),
        LemChild::Reg(elem) => elem.serialize_mei(writer),
        LemChild::Verse(elem) => elem.serialize_mei(writer),
        LemChild::Syl(elem) => elem.serialize_mei(writer),
        LemChild::Q(elem) => elem.serialize_mei(writer),
        LemChild::MRpt2(elem) => elem.serialize_mei(writer),
        LemChild::Mensur(elem) => elem.serialize_mei(writer),
        LemChild::Address(elem) => elem.serialize_mei(writer),
        LemChild::Bend(elem) => elem.serialize_mei(writer),
        LemChild::NcGrp(elem) => elem.serialize_mei(writer),
        LemChild::BiblStruct(elem) => elem.serialize_mei(writer),
        LemChild::Lv(elem) => elem.serialize_mei(writer),
        LemChild::Stack(elem) => elem.serialize_mei(writer),
        LemChild::Expan(elem) => elem.serialize_mei(writer),
        LemChild::Pb(elem) => elem.serialize_mei(writer),
        LemChild::Identifier(elem) => elem.serialize_mei(writer),
        LemChild::Accid(elem) => elem.serialize_mei(writer),
        LemChild::AnchoredText(elem) => elem.serialize_mei(writer),
        LemChild::Stamp(elem) => elem.serialize_mei(writer),
        LemChild::Trill(elem) => elem.serialize_mei(writer),
        LemChild::Tie(elem) => elem.serialize_mei(writer),
        LemChild::Measure(elem) => elem.serialize_mei(writer),
        LemChild::Arpeg(elem) => elem.serialize_mei(writer),
        LemChild::Note(elem) => elem.serialize_mei(writer),
        LemChild::Quote(elem) => elem.serialize_mei(writer),
        LemChild::List(elem) => elem.serialize_mei(writer),
        LemChild::RepeatMark(elem) => elem.serialize_mei(writer),
        LemChild::TupletSpan(elem) => elem.serialize_mei(writer),
        LemChild::HarpPedal(elem) => elem.serialize_mei(writer),
        LemChild::Beam(elem) => elem.serialize_mei(writer),
        LemChild::Tempo(elem) => elem.serialize_mei(writer),
        LemChild::Mordent(elem) => elem.serialize_mei(writer),
        LemChild::Restore(elem) => elem.serialize_mei(writer),
        LemChild::Dir(elem) => elem.serialize_mei(writer),
        LemChild::Attacca(elem) => elem.serialize_mei(writer),
        LemChild::Depth(elem) => elem.serialize_mei(writer),
        LemChild::Ornam(elem) => elem.serialize_mei(writer),
        LemChild::Line(elem) => elem.serialize_mei(writer),
        LemChild::Sp(elem) => elem.serialize_mei(writer),
        LemChild::Damage(elem) => elem.serialize_mei(writer),
        LemChild::KeySig(elem) => elem.serialize_mei(writer),
        LemChild::Harm(elem) => elem.serialize_mei(writer),
        LemChild::PostCode(elem) => elem.serialize_mei(writer),
        LemChild::Gliss(elem) => elem.serialize_mei(writer),
        LemChild::HispanTick(elem) => elem.serialize_mei(writer),
        LemChild::Syllable(elem) => elem.serialize_mei(writer),
        LemChild::MetaMark(elem) => elem.serialize_mei(writer),
        LemChild::BTrem(elem) => elem.serialize_mei(writer),
        LemChild::StyleName(elem) => elem.serialize_mei(writer),
        LemChild::Country(elem) => elem.serialize_mei(writer),
        LemChild::Nc(elem) => elem.serialize_mei(writer),
        LemChild::Breath(elem) => elem.serialize_mei(writer),
        LemChild::Cb(elem) => elem.serialize_mei(writer),
        LemChild::FingGrp(elem) => elem.serialize_mei(writer),
        LemChild::Div(elem) => elem.serialize_mei(writer),
    }
}

/// Serialize a RdgChild variant to the writer.
fn serialize_rdg_child<W: Write>(
    child: &RdgChild,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    match child {
        RdgChild::Text(text) => writer.write_text(text),
        RdgChild::Rest(elem) => elem.serialize_mei(writer),
        RdgChild::Ref(elem) => elem.serialize_mei(writer),
        RdgChild::Stack(elem) => elem.serialize_mei(writer),
        RdgChild::Rend(elem) => elem.serialize_mei(writer),
        RdgChild::Trill(elem) => elem.serialize_mei(writer),
        RdgChild::Line(elem) => elem.serialize_mei(writer),
        RdgChild::Reg(elem) => elem.serialize_mei(writer),
        RdgChild::Pedal(elem) => elem.serialize_mei(writer),
        RdgChild::Section(elem) => elem.serialize_mei(writer),
        RdgChild::Identifier(elem) => elem.serialize_mei(writer),
        RdgChild::Bloc(elem) => elem.serialize_mei(writer),
        RdgChild::ClefGrp(elem) => elem.serialize_mei(writer),
        RdgChild::RelationList(elem) => elem.serialize_mei(writer),
        RdgChild::MultiRest(elem) => elem.serialize_mei(writer),
        RdgChild::Stamp(elem) => elem.serialize_mei(writer),
        RdgChild::Attacca(elem) => elem.serialize_mei(writer),
        RdgChild::Heraldry(elem) => elem.serialize_mei(writer),
        RdgChild::Expan(elem) => elem.serialize_mei(writer),
        RdgChild::Term(elem) => elem.serialize_mei(writer),
        RdgChild::Relation(elem) => elem.serialize_mei(writer),
        RdgChild::Lv(elem) => elem.serialize_mei(writer),
        RdgChild::Dynam(elem) => elem.serialize_mei(writer),
        RdgChild::Locus(elem) => elem.serialize_mei(writer),
        RdgChild::Hairpin(elem) => elem.serialize_mei(writer),
        RdgChild::Corr(elem) => elem.serialize_mei(writer),
        RdgChild::AnchoredText(elem) => elem.serialize_mei(writer),
        RdgChild::Arpeg(elem) => elem.serialize_mei(writer),
        RdgChild::List(elem) => elem.serialize_mei(writer),
        RdgChild::F(elem) => elem.serialize_mei(writer),
        RdgChild::MetaMark(elem) => elem.serialize_mei(writer),
        RdgChild::BiblStruct(elem) => elem.serialize_mei(writer),
        RdgChild::Div(elem) => elem.serialize_mei(writer),
        RdgChild::Verse(elem) => elem.serialize_mei(writer),
        RdgChild::Repository(elem) => elem.serialize_mei(writer),
        RdgChild::Sp(elem) => elem.serialize_mei(writer),
        RdgChild::Catchwords(elem) => elem.serialize_mei(writer),
        RdgChild::App(elem) => elem.serialize_mei(writer),
        RdgChild::Mensur(elem) => elem.serialize_mei(writer),
        RdgChild::Accid(elem) => elem.serialize_mei(writer),
        RdgChild::Proport(elem) => elem.serialize_mei(writer),
        RdgChild::Staff(elem) => elem.serialize_mei(writer),
        RdgChild::Strophicus(elem) => elem.serialize_mei(writer),
        RdgChild::GeogFeat(elem) => elem.serialize_mei(writer),
        RdgChild::Num(elem) => elem.serialize_mei(writer),
        RdgChild::Street(elem) => elem.serialize_mei(writer),
        RdgChild::Q(elem) => elem.serialize_mei(writer),
        RdgChild::Gap(elem) => elem.serialize_mei(writer),
        RdgChild::Subst(elem) => elem.serialize_mei(writer),
        RdgChild::Pad(elem) => elem.serialize_mei(writer),
        RdgChild::Quote(elem) => elem.serialize_mei(writer),
        RdgChild::Curve(elem) => elem.serialize_mei(writer),
        RdgChild::Region(elem) => elem.serialize_mei(writer),
        RdgChild::ColLayout(elem) => elem.serialize_mei(writer),
        RdgChild::Cb(elem) => elem.serialize_mei(writer),
        RdgChild::Add(elem) => elem.serialize_mei(writer),
        RdgChild::Ligature(elem) => elem.serialize_mei(writer),
        RdgChild::Depth(elem) => elem.serialize_mei(writer),
        RdgChild::Nc(elem) => elem.serialize_mei(writer),
        RdgChild::EventList(elem) => elem.serialize_mei(writer),
        RdgChild::Del(elem) => elem.serialize_mei(writer),
        RdgChild::Space(elem) => elem.serialize_mei(writer),
        RdgChild::MeterSigGrp(elem) => elem.serialize_mei(writer),
        RdgChild::LocusGrp(elem) => elem.serialize_mei(writer),
        RdgChild::Fing(elem) => elem.serialize_mei(writer),
        RdgChild::Ornam(elem) => elem.serialize_mei(writer),
        RdgChild::Fig(elem) => elem.serialize_mei(writer),
        RdgChild::Custos(elem) => elem.serialize_mei(writer),
        RdgChild::Orig(elem) => elem.serialize_mei(writer),
        RdgChild::MSpace(elem) => elem.serialize_mei(writer),
        RdgChild::StageDir(elem) => elem.serialize_mei(writer),
        RdgChild::Height(elem) => elem.serialize_mei(writer),
        RdgChild::DivLine(elem) => elem.serialize_mei(writer),
        RdgChild::HarpPedal(elem) => elem.serialize_mei(writer),
        RdgChild::Measure(elem) => elem.serialize_mei(writer),
        RdgChild::Title(elem) => elem.serialize_mei(writer),
        RdgChild::Liquescent(elem) => elem.serialize_mei(writer),
        RdgChild::Lg(elem) => elem.serialize_mei(writer),
        RdgChild::Sic(elem) => elem.serialize_mei(writer),
        RdgChild::TabGrp(elem) => elem.serialize_mei(writer),
        RdgChild::Expansion(elem) => elem.serialize_mei(writer),
        RdgChild::MeterSig(elem) => elem.serialize_mei(writer),
        RdgChild::Refrain(elem) => elem.serialize_mei(writer),
        RdgChild::Extent(elem) => elem.serialize_mei(writer),
        RdgChild::Signatures(elem) => elem.serialize_mei(writer),
        RdgChild::PeriodName(elem) => elem.serialize_mei(writer),
        RdgChild::RepeatMark(elem) => elem.serialize_mei(writer),
        RdgChild::Symbol(elem) => elem.serialize_mei(writer),
        RdgChild::Tie(elem) => elem.serialize_mei(writer),
        RdgChild::Width(elem) => elem.serialize_mei(writer),
        RdgChild::Seg(elem) => elem.serialize_mei(writer),
        RdgChild::Octave(elem) => elem.serialize_mei(writer),
        RdgChild::SecFolio(elem) => elem.serialize_mei(writer),
        RdgChild::Quilisma(elem) => elem.serialize_mei(writer),
        RdgChild::Bibl(elem) => elem.serialize_mei(writer),
        RdgChild::Table(elem) => elem.serialize_mei(writer),
        RdgChild::HandShift(elem) => elem.serialize_mei(writer),
        RdgChild::Country(elem) => elem.serialize_mei(writer),
        RdgChild::Unclear(elem) => elem.serialize_mei(writer),
        RdgChild::PostCode(elem) => elem.serialize_mei(writer),
        RdgChild::Tuplet(elem) => elem.serialize_mei(writer),
        RdgChild::GeogName(elem) => elem.serialize_mei(writer),
        RdgChild::Annot(elem) => elem.serialize_mei(writer),
        RdgChild::Harm(elem) => elem.serialize_mei(writer),
        RdgChild::StyleName(elem) => elem.serialize_mei(writer),
        RdgChild::Caesura(elem) => elem.serialize_mei(writer),
        RdgChild::Bend(elem) => elem.serialize_mei(writer),
        RdgChild::BeamSpan(elem) => elem.serialize_mei(writer),
        RdgChild::MRpt(elem) => elem.serialize_mei(writer),
        RdgChild::PostBox(elem) => elem.serialize_mei(writer),
        RdgChild::Turn(elem) => elem.serialize_mei(writer),
        RdgChild::Tempo(elem) => elem.serialize_mei(writer),
        RdgChild::GraceGrp(elem) => elem.serialize_mei(writer),
        RdgChild::Name(elem) => elem.serialize_mei(writer),
        RdgChild::Dim(elem) => elem.serialize_mei(writer),
        RdgChild::Dir(elem) => elem.serialize_mei(writer),
        RdgChild::Damage(elem) => elem.serialize_mei(writer),
        RdgChild::Neume(elem) => elem.serialize_mei(writer),
        RdgChild::TabDurSym(elem) => elem.serialize_mei(writer),
        RdgChild::MRpt2(elem) => elem.serialize_mei(writer),
        RdgChild::Pb(elem) => elem.serialize_mei(writer),
        RdgChild::BarLine(elem) => elem.serialize_mei(writer),
        RdgChild::Lb(elem) => elem.serialize_mei(writer),
        RdgChild::Phrase(elem) => elem.serialize_mei(writer),
        RdgChild::Dot(elem) => elem.serialize_mei(writer),
        RdgChild::Reh(elem) => elem.serialize_mei(writer),
        RdgChild::Artic(elem) => elem.serialize_mei(writer),
        RdgChild::Oriscus(elem) => elem.serialize_mei(writer),
        RdgChild::Abbr(elem) => elem.serialize_mei(writer),
        RdgChild::KeySig(elem) => elem.serialize_mei(writer),
        RdgChild::Date(elem) => elem.serialize_mei(writer),
        RdgChild::CastList(elem) => elem.serialize_mei(writer),
        RdgChild::Beam(elem) => elem.serialize_mei(writer),
        RdgChild::Midi(elem) => elem.serialize_mei(writer),
        RdgChild::NcGrp(elem) => elem.serialize_mei(writer),
        RdgChild::Address(elem) => elem.serialize_mei(writer),
        RdgChild::BiblList(elem) => elem.serialize_mei(writer),
        RdgChild::BracketSpan(elem) => elem.serialize_mei(writer),
        RdgChild::Breath(elem) => elem.serialize_mei(writer),
        RdgChild::District(elem) => elem.serialize_mei(writer),
        RdgChild::P(elem) => elem.serialize_mei(writer),
        RdgChild::Ending(elem) => elem.serialize_mei(writer),
        RdgChild::StaffGrp(elem) => elem.serialize_mei(writer),
        RdgChild::CpMark(elem) => elem.serialize_mei(writer),
        RdgChild::CorpName(elem) => elem.serialize_mei(writer),
        RdgChild::FTrem(elem) => elem.serialize_mei(writer),
        RdgChild::Gliss(elem) => elem.serialize_mei(writer),
        RdgChild::Syl(elem) => elem.serialize_mei(writer),
        RdgChild::Settlement(elem) => elem.serialize_mei(writer),
        RdgChild::Restore(elem) => elem.serialize_mei(writer),
        RdgChild::Supplied(elem) => elem.serialize_mei(writer),
        RdgChild::TupletSpan(elem) => elem.serialize_mei(writer),
        RdgChild::Layer(elem) => elem.serialize_mei(writer),
        RdgChild::Chord(elem) => elem.serialize_mei(writer),
        RdgChild::BeatRpt(elem) => elem.serialize_mei(writer),
        RdgChild::StaffDef(elem) => elem.serialize_mei(writer),
        RdgChild::Fermata(elem) => elem.serialize_mei(writer),
        RdgChild::Episema(elem) => elem.serialize_mei(writer),
        RdgChild::Ptr(elem) => elem.serialize_mei(writer),
        RdgChild::Mordent(elem) => elem.serialize_mei(writer),
        RdgChild::Choice(elem) => elem.serialize_mei(writer),
        RdgChild::Note(elem) => elem.serialize_mei(writer),
        RdgChild::Syllable(elem) => elem.serialize_mei(writer),
        RdgChild::Slur(elem) => elem.serialize_mei(writer),
        RdgChild::Clef(elem) => elem.serialize_mei(writer),
        RdgChild::MultiRpt(elem) => elem.serialize_mei(writer),
        RdgChild::SignifLet(elem) => elem.serialize_mei(writer),
        RdgChild::Volta(elem) => elem.serialize_mei(writer),
        RdgChild::FingGrp(elem) => elem.serialize_mei(writer),
        RdgChild::ScoreDef(elem) => elem.serialize_mei(writer),
        RdgChild::PersName(elem) => elem.serialize_mei(writer),
        RdgChild::Sb(elem) => elem.serialize_mei(writer),
        RdgChild::HispanTick(elem) => elem.serialize_mei(writer),
        RdgChild::HalfmRpt(elem) => elem.serialize_mei(writer),
        RdgChild::MRest(elem) => elem.serialize_mei(writer),
        RdgChild::Dimensions(elem) => elem.serialize_mei(writer),
        RdgChild::BTrem(elem) => elem.serialize_mei(writer),
    }
}

/// Serialize an AddChild variant to the writer.
fn serialize_add_child<W: Write>(
    child: &AddChild,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    match child {
        AddChild::Text(text) => writer.write_text(text),
        AddChild::Abbr(elem) => elem.serialize_mei(writer),
        AddChild::Accid(elem) => elem.serialize_mei(writer),
        AddChild::Add(elem) => elem.serialize_mei(writer),
        AddChild::Address(elem) => elem.serialize_mei(writer),
        AddChild::AnchoredText(elem) => elem.serialize_mei(writer),
        AddChild::Annot(elem) => elem.serialize_mei(writer),
        AddChild::Arpeg(elem) => elem.serialize_mei(writer),
        AddChild::Artic(elem) => elem.serialize_mei(writer),
        AddChild::Attacca(elem) => elem.serialize_mei(writer),
        AddChild::BTrem(elem) => elem.serialize_mei(writer),
        AddChild::BarLine(elem) => elem.serialize_mei(writer),
        AddChild::Beam(elem) => elem.serialize_mei(writer),
        AddChild::BeamSpan(elem) => elem.serialize_mei(writer),
        AddChild::BeatRpt(elem) => elem.serialize_mei(writer),
        AddChild::Bend(elem) => elem.serialize_mei(writer),
        AddChild::Bibl(elem) => elem.serialize_mei(writer),
        AddChild::BiblList(elem) => elem.serialize_mei(writer),
        AddChild::BiblStruct(elem) => elem.serialize_mei(writer),
        AddChild::Bloc(elem) => elem.serialize_mei(writer),
        AddChild::BracketSpan(elem) => elem.serialize_mei(writer),
        AddChild::Breath(elem) => elem.serialize_mei(writer),
        AddChild::Caesura(elem) => elem.serialize_mei(writer),
        AddChild::CastList(elem) => elem.serialize_mei(writer),
        AddChild::Catchwords(elem) => elem.serialize_mei(writer),
        AddChild::Cb(elem) => elem.serialize_mei(writer),
        AddChild::Choice(elem) => elem.serialize_mei(writer),
        AddChild::Chord(elem) => elem.serialize_mei(writer),
        AddChild::Clef(elem) => elem.serialize_mei(writer),
        AddChild::ClefGrp(elem) => elem.serialize_mei(writer),
        AddChild::ColLayout(elem) => elem.serialize_mei(writer),
        AddChild::CorpName(elem) => elem.serialize_mei(writer),
        AddChild::Corr(elem) => elem.serialize_mei(writer),
        AddChild::Country(elem) => elem.serialize_mei(writer),
        AddChild::CpMark(elem) => elem.serialize_mei(writer),
        AddChild::Curve(elem) => elem.serialize_mei(writer),
        AddChild::Custos(elem) => elem.serialize_mei(writer),
        AddChild::Damage(elem) => elem.serialize_mei(writer),
        AddChild::Date(elem) => elem.serialize_mei(writer),
        AddChild::Del(elem) => elem.serialize_mei(writer),
        AddChild::Depth(elem) => elem.serialize_mei(writer),
        AddChild::Dim(elem) => elem.serialize_mei(writer),
        AddChild::Dimensions(elem) => elem.serialize_mei(writer),
        AddChild::Dir(elem) => elem.serialize_mei(writer),
        AddChild::District(elem) => elem.serialize_mei(writer),
        AddChild::Div(elem) => elem.serialize_mei(writer),
        AddChild::DivLine(elem) => elem.serialize_mei(writer),
        AddChild::Dot(elem) => elem.serialize_mei(writer),
        AddChild::Dynam(elem) => elem.serialize_mei(writer),
        AddChild::Ending(elem) => elem.serialize_mei(writer),
        AddChild::EventList(elem) => elem.serialize_mei(writer),
        AddChild::Expan(elem) => elem.serialize_mei(writer),
        AddChild::Extent(elem) => elem.serialize_mei(writer),
        AddChild::F(elem) => elem.serialize_mei(writer),
        AddChild::FTrem(elem) => elem.serialize_mei(writer),
        AddChild::Fermata(elem) => elem.serialize_mei(writer),
        AddChild::Fig(elem) => elem.serialize_mei(writer),
        AddChild::Fing(elem) => elem.serialize_mei(writer),
        AddChild::FingGrp(elem) => elem.serialize_mei(writer),
        AddChild::Gap(elem) => elem.serialize_mei(writer),
        AddChild::GeogFeat(elem) => elem.serialize_mei(writer),
        AddChild::GeogName(elem) => elem.serialize_mei(writer),
        AddChild::Gliss(elem) => elem.serialize_mei(writer),
        AddChild::GraceGrp(elem) => elem.serialize_mei(writer),
        AddChild::Hairpin(elem) => elem.serialize_mei(writer),
        AddChild::HalfmRpt(elem) => elem.serialize_mei(writer),
        AddChild::HandShift(elem) => elem.serialize_mei(writer),
        AddChild::Harm(elem) => elem.serialize_mei(writer),
        AddChild::HarpPedal(elem) => elem.serialize_mei(writer),
        AddChild::Height(elem) => elem.serialize_mei(writer),
        AddChild::Heraldry(elem) => elem.serialize_mei(writer),
        AddChild::Identifier(elem) => elem.serialize_mei(writer),
        AddChild::KeyAccid(elem) => elem.serialize_mei(writer),
        AddChild::KeySig(elem) => elem.serialize_mei(writer),
        AddChild::Layer(elem) => elem.serialize_mei(writer),
        AddChild::Lb(elem) => elem.serialize_mei(writer),
        AddChild::Lg(elem) => elem.serialize_mei(writer),
        AddChild::Ligature(elem) => elem.serialize_mei(writer),
        AddChild::Line(elem) => elem.serialize_mei(writer),
        AddChild::List(elem) => elem.serialize_mei(writer),
        AddChild::Locus(elem) => elem.serialize_mei(writer),
        AddChild::LocusGrp(elem) => elem.serialize_mei(writer),
        AddChild::Lv(elem) => elem.serialize_mei(writer),
        AddChild::MRest(elem) => elem.serialize_mei(writer),
        AddChild::MRpt(elem) => elem.serialize_mei(writer),
        AddChild::MRpt2(elem) => elem.serialize_mei(writer),
        AddChild::MSpace(elem) => elem.serialize_mei(writer),
        AddChild::Measure(elem) => elem.serialize_mei(writer),
        AddChild::Mensur(elem) => elem.serialize_mei(writer),
        AddChild::MetaMark(elem) => elem.serialize_mei(writer),
        AddChild::MeterSig(elem) => elem.serialize_mei(writer),
        AddChild::MeterSigGrp(elem) => elem.serialize_mei(writer),
        AddChild::Midi(elem) => elem.serialize_mei(writer),
        AddChild::Mordent(elem) => elem.serialize_mei(writer),
        AddChild::MultiRest(elem) => elem.serialize_mei(writer),
        AddChild::MultiRpt(elem) => elem.serialize_mei(writer),
        AddChild::Name(elem) => elem.serialize_mei(writer),
        AddChild::Neume(elem) => elem.serialize_mei(writer),
        AddChild::Note(elem) => elem.serialize_mei(writer),
        AddChild::Num(elem) => elem.serialize_mei(writer),
        AddChild::Octave(elem) => elem.serialize_mei(writer),
        AddChild::Orig(elem) => elem.serialize_mei(writer),
        AddChild::Ornam(elem) => elem.serialize_mei(writer),
        AddChild::P(elem) => elem.serialize_mei(writer),
        AddChild::Pad(elem) => elem.serialize_mei(writer),
        AddChild::Pb(elem) => elem.serialize_mei(writer),
        AddChild::Pedal(elem) => elem.serialize_mei(writer),
        AddChild::PeriodName(elem) => elem.serialize_mei(writer),
        AddChild::PersName(elem) => elem.serialize_mei(writer),
        AddChild::Phrase(elem) => elem.serialize_mei(writer),
        AddChild::PostBox(elem) => elem.serialize_mei(writer),
        AddChild::PostCode(elem) => elem.serialize_mei(writer),
        AddChild::Proport(elem) => elem.serialize_mei(writer),
        AddChild::Ptr(elem) => elem.serialize_mei(writer),
        AddChild::Q(elem) => elem.serialize_mei(writer),
        AddChild::Quote(elem) => elem.serialize_mei(writer),
        AddChild::Ref(elem) => elem.serialize_mei(writer),
        AddChild::Refrain(elem) => elem.serialize_mei(writer),
        AddChild::Reg(elem) => elem.serialize_mei(writer),
        AddChild::Region(elem) => elem.serialize_mei(writer),
        AddChild::Reh(elem) => elem.serialize_mei(writer),
        AddChild::Relation(elem) => elem.serialize_mei(writer),
        AddChild::RelationList(elem) => elem.serialize_mei(writer),
        AddChild::Rend(elem) => elem.serialize_mei(writer),
        AddChild::RepeatMark(elem) => elem.serialize_mei(writer),
        AddChild::Repository(elem) => elem.serialize_mei(writer),
        AddChild::Rest(elem) => elem.serialize_mei(writer),
        AddChild::Restore(elem) => elem.serialize_mei(writer),
        AddChild::Sb(elem) => elem.serialize_mei(writer),
        AddChild::ScoreDef(elem) => elem.serialize_mei(writer),
        AddChild::SecFolio(elem) => elem.serialize_mei(writer),
        AddChild::Section(elem) => elem.serialize_mei(writer),
        AddChild::Seg(elem) => elem.serialize_mei(writer),
        AddChild::Settlement(elem) => elem.serialize_mei(writer),
        AddChild::Sic(elem) => elem.serialize_mei(writer),
        AddChild::Signatures(elem) => elem.serialize_mei(writer),
        AddChild::Slur(elem) => elem.serialize_mei(writer),
        AddChild::Space(elem) => elem.serialize_mei(writer),
        AddChild::Sp(elem) => elem.serialize_mei(writer),
        AddChild::Stack(elem) => elem.serialize_mei(writer),
        AddChild::Stamp(elem) => elem.serialize_mei(writer),
        AddChild::Staff(elem) => elem.serialize_mei(writer),
        AddChild::StaffDef(elem) => elem.serialize_mei(writer),
        AddChild::StaffGrp(elem) => elem.serialize_mei(writer),
        AddChild::StageDir(elem) => elem.serialize_mei(writer),
        AddChild::Street(elem) => elem.serialize_mei(writer),
        AddChild::StyleName(elem) => elem.serialize_mei(writer),
        AddChild::Subst(elem) => elem.serialize_mei(writer),
        AddChild::Supplied(elem) => elem.serialize_mei(writer),
        AddChild::Syl(elem) => elem.serialize_mei(writer),
        AddChild::Syllable(elem) => elem.serialize_mei(writer),
        AddChild::Symbol(elem) => elem.serialize_mei(writer),
        AddChild::TabDurSym(elem) => elem.serialize_mei(writer),
        AddChild::TabGrp(elem) => elem.serialize_mei(writer),
        AddChild::Table(elem) => elem.serialize_mei(writer),
        AddChild::Tempo(elem) => elem.serialize_mei(writer),
        AddChild::Term(elem) => elem.serialize_mei(writer),
        AddChild::Tie(elem) => elem.serialize_mei(writer),
        AddChild::Title(elem) => elem.serialize_mei(writer),
        AddChild::Trill(elem) => elem.serialize_mei(writer),
        AddChild::Tuplet(elem) => elem.serialize_mei(writer),
        AddChild::TupletSpan(elem) => elem.serialize_mei(writer),
        AddChild::Turn(elem) => elem.serialize_mei(writer),
        AddChild::Unclear(elem) => elem.serialize_mei(writer),
        AddChild::Verse(elem) => elem.serialize_mei(writer),
        AddChild::Volta(elem) => elem.serialize_mei(writer),
        AddChild::Width(elem) => elem.serialize_mei(writer),
    }
}

/// Serialize a DelChild variant to the writer.
fn serialize_del_child<W: Write>(
    child: &DelChild,
    writer: &mut MeiWriter<W>,
) -> SerializeResult<()> {
    match child {
        DelChild::Text(text) => writer.write_text(text),
        DelChild::Abbr(elem) => elem.serialize_mei(writer),
        DelChild::Accid(elem) => elem.serialize_mei(writer),
        DelChild::Add(elem) => elem.serialize_mei(writer),
        DelChild::Address(elem) => elem.serialize_mei(writer),
        DelChild::AnchoredText(elem) => elem.serialize_mei(writer),
        DelChild::Annot(elem) => elem.serialize_mei(writer),
        DelChild::Arpeg(elem) => elem.serialize_mei(writer),
        DelChild::Artic(elem) => elem.serialize_mei(writer),
        DelChild::Attacca(elem) => elem.serialize_mei(writer),
        DelChild::BTrem(elem) => elem.serialize_mei(writer),
        DelChild::BarLine(elem) => elem.serialize_mei(writer),
        DelChild::Beam(elem) => elem.serialize_mei(writer),
        DelChild::BeamSpan(elem) => elem.serialize_mei(writer),
        DelChild::BeatRpt(elem) => elem.serialize_mei(writer),
        DelChild::Bend(elem) => elem.serialize_mei(writer),
        DelChild::Bibl(elem) => elem.serialize_mei(writer),
        DelChild::BiblList(elem) => elem.serialize_mei(writer),
        DelChild::BiblStruct(elem) => elem.serialize_mei(writer),
        DelChild::Bloc(elem) => elem.serialize_mei(writer),
        DelChild::BracketSpan(elem) => elem.serialize_mei(writer),
        DelChild::Breath(elem) => elem.serialize_mei(writer),
        DelChild::Caesura(elem) => elem.serialize_mei(writer),
        DelChild::CastList(elem) => elem.serialize_mei(writer),
        DelChild::Catchwords(elem) => elem.serialize_mei(writer),
        DelChild::Cb(elem) => elem.serialize_mei(writer),
        DelChild::Choice(elem) => elem.serialize_mei(writer),
        DelChild::Chord(elem) => elem.serialize_mei(writer),
        DelChild::Clef(elem) => elem.serialize_mei(writer),
        DelChild::ClefGrp(elem) => elem.serialize_mei(writer),
        DelChild::ColLayout(elem) => elem.serialize_mei(writer),
        DelChild::CorpName(elem) => elem.serialize_mei(writer),
        DelChild::Corr(elem) => elem.serialize_mei(writer),
        DelChild::Country(elem) => elem.serialize_mei(writer),
        DelChild::CpMark(elem) => elem.serialize_mei(writer),
        DelChild::Curve(elem) => elem.serialize_mei(writer),
        DelChild::Custos(elem) => elem.serialize_mei(writer),
        DelChild::Damage(elem) => elem.serialize_mei(writer),
        DelChild::Date(elem) => elem.serialize_mei(writer),
        DelChild::Del(elem) => elem.serialize_mei(writer),
        DelChild::Depth(elem) => elem.serialize_mei(writer),
        DelChild::Dim(elem) => elem.serialize_mei(writer),
        DelChild::Dimensions(elem) => elem.serialize_mei(writer),
        DelChild::Dir(elem) => elem.serialize_mei(writer),
        DelChild::District(elem) => elem.serialize_mei(writer),
        DelChild::Div(elem) => elem.serialize_mei(writer),
        DelChild::DivLine(elem) => elem.serialize_mei(writer),
        DelChild::Dot(elem) => elem.serialize_mei(writer),
        DelChild::Dynam(elem) => elem.serialize_mei(writer),
        DelChild::Ending(elem) => elem.serialize_mei(writer),
        DelChild::Episema(elem) => elem.serialize_mei(writer),
        DelChild::EventList(elem) => elem.serialize_mei(writer),
        DelChild::Expan(elem) => elem.serialize_mei(writer),
        DelChild::Extent(elem) => elem.serialize_mei(writer),
        DelChild::F(elem) => elem.serialize_mei(writer),
        DelChild::FTrem(elem) => elem.serialize_mei(writer),
        DelChild::Fermata(elem) => elem.serialize_mei(writer),
        DelChild::Fig(elem) => elem.serialize_mei(writer),
        DelChild::Fing(elem) => elem.serialize_mei(writer),
        DelChild::FingGrp(elem) => elem.serialize_mei(writer),
        DelChild::Gap(elem) => elem.serialize_mei(writer),
        DelChild::GeogFeat(elem) => elem.serialize_mei(writer),
        DelChild::GeogName(elem) => elem.serialize_mei(writer),
        DelChild::Gliss(elem) => elem.serialize_mei(writer),
        DelChild::GraceGrp(elem) => elem.serialize_mei(writer),
        DelChild::Hairpin(elem) => elem.serialize_mei(writer),
        DelChild::HalfmRpt(elem) => elem.serialize_mei(writer),
        DelChild::HandShift(elem) => elem.serialize_mei(writer),
        DelChild::Harm(elem) => elem.serialize_mei(writer),
        DelChild::HarpPedal(elem) => elem.serialize_mei(writer),
        DelChild::Height(elem) => elem.serialize_mei(writer),
        DelChild::Heraldry(elem) => elem.serialize_mei(writer),
        DelChild::HispanTick(elem) => elem.serialize_mei(writer),
        DelChild::Identifier(elem) => elem.serialize_mei(writer),
        DelChild::KeyAccid(elem) => elem.serialize_mei(writer),
        DelChild::KeySig(elem) => elem.serialize_mei(writer),
        DelChild::Layer(elem) => elem.serialize_mei(writer),
        DelChild::Lb(elem) => elem.serialize_mei(writer),
        DelChild::Lg(elem) => elem.serialize_mei(writer),
        DelChild::Ligature(elem) => elem.serialize_mei(writer),
        DelChild::Line(elem) => elem.serialize_mei(writer),
        DelChild::Liquescent(elem) => elem.serialize_mei(writer),
        DelChild::List(elem) => elem.serialize_mei(writer),
        DelChild::Locus(elem) => elem.serialize_mei(writer),
        DelChild::LocusGrp(elem) => elem.serialize_mei(writer),
        DelChild::Lv(elem) => elem.serialize_mei(writer),
        DelChild::MRest(elem) => elem.serialize_mei(writer),
        DelChild::MRpt(elem) => elem.serialize_mei(writer),
        DelChild::MRpt2(elem) => elem.serialize_mei(writer),
        DelChild::MSpace(elem) => elem.serialize_mei(writer),
        DelChild::Measure(elem) => elem.serialize_mei(writer),
        DelChild::Mensur(elem) => elem.serialize_mei(writer),
        DelChild::MetaMark(elem) => elem.serialize_mei(writer),
        DelChild::MeterSig(elem) => elem.serialize_mei(writer),
        DelChild::MeterSigGrp(elem) => elem.serialize_mei(writer),
        DelChild::Midi(elem) => elem.serialize_mei(writer),
        DelChild::Mordent(elem) => elem.serialize_mei(writer),
        DelChild::MultiRest(elem) => elem.serialize_mei(writer),
        DelChild::MultiRpt(elem) => elem.serialize_mei(writer),
        DelChild::Name(elem) => elem.serialize_mei(writer),
        DelChild::Nc(elem) => elem.serialize_mei(writer),
        DelChild::NcGrp(elem) => elem.serialize_mei(writer),
        DelChild::Neume(elem) => elem.serialize_mei(writer),
        DelChild::Note(elem) => elem.serialize_mei(writer),
        DelChild::Num(elem) => elem.serialize_mei(writer),
        DelChild::Octave(elem) => elem.serialize_mei(writer),
        DelChild::Orig(elem) => elem.serialize_mei(writer),
        DelChild::Oriscus(elem) => elem.serialize_mei(writer),
        DelChild::Ornam(elem) => elem.serialize_mei(writer),
        DelChild::P(elem) => elem.serialize_mei(writer),
        DelChild::Pad(elem) => elem.serialize_mei(writer),
        DelChild::Pb(elem) => elem.serialize_mei(writer),
        DelChild::Pedal(elem) => elem.serialize_mei(writer),
        DelChild::PeriodName(elem) => elem.serialize_mei(writer),
        DelChild::PersName(elem) => elem.serialize_mei(writer),
        DelChild::Phrase(elem) => elem.serialize_mei(writer),
        DelChild::PostBox(elem) => elem.serialize_mei(writer),
        DelChild::PostCode(elem) => elem.serialize_mei(writer),
        DelChild::Proport(elem) => elem.serialize_mei(writer),
        DelChild::Ptr(elem) => elem.serialize_mei(writer),
        DelChild::Q(elem) => elem.serialize_mei(writer),
        DelChild::Quilisma(elem) => elem.serialize_mei(writer),
        DelChild::Quote(elem) => elem.serialize_mei(writer),
        DelChild::Ref(elem) => elem.serialize_mei(writer),
        DelChild::Refrain(elem) => elem.serialize_mei(writer),
        DelChild::Reg(elem) => elem.serialize_mei(writer),
        DelChild::Region(elem) => elem.serialize_mei(writer),
        DelChild::Reh(elem) => elem.serialize_mei(writer),
        DelChild::Relation(elem) => elem.serialize_mei(writer),
        DelChild::RelationList(elem) => elem.serialize_mei(writer),
        DelChild::Rend(elem) => elem.serialize_mei(writer),
        DelChild::RepeatMark(elem) => elem.serialize_mei(writer),
        DelChild::Repository(elem) => elem.serialize_mei(writer),
        DelChild::Rest(elem) => elem.serialize_mei(writer),
        DelChild::Restore(elem) => elem.serialize_mei(writer),
        DelChild::Sb(elem) => elem.serialize_mei(writer),
        DelChild::ScoreDef(elem) => elem.serialize_mei(writer),
        DelChild::SecFolio(elem) => elem.serialize_mei(writer),
        DelChild::Section(elem) => elem.serialize_mei(writer),
        DelChild::Seg(elem) => elem.serialize_mei(writer),
        DelChild::Settlement(elem) => elem.serialize_mei(writer),
        DelChild::Sic(elem) => elem.serialize_mei(writer),
        DelChild::SignifLet(elem) => elem.serialize_mei(writer),
        DelChild::Signatures(elem) => elem.serialize_mei(writer),
        DelChild::Slur(elem) => elem.serialize_mei(writer),
        DelChild::Space(elem) => elem.serialize_mei(writer),
        DelChild::Sp(elem) => elem.serialize_mei(writer),
        DelChild::Stack(elem) => elem.serialize_mei(writer),
        DelChild::Staff(elem) => elem.serialize_mei(writer),
        DelChild::StaffDef(elem) => elem.serialize_mei(writer),
        DelChild::StaffGrp(elem) => elem.serialize_mei(writer),
        DelChild::StageDir(elem) => elem.serialize_mei(writer),
        DelChild::Stamp(elem) => elem.serialize_mei(writer),
        DelChild::Street(elem) => elem.serialize_mei(writer),
        DelChild::Strophicus(elem) => elem.serialize_mei(writer),
        DelChild::StyleName(elem) => elem.serialize_mei(writer),
        DelChild::Subst(elem) => elem.serialize_mei(writer),
        DelChild::Supplied(elem) => elem.serialize_mei(writer),
        DelChild::Syl(elem) => elem.serialize_mei(writer),
        DelChild::Syllable(elem) => elem.serialize_mei(writer),
        DelChild::Symbol(elem) => elem.serialize_mei(writer),
        DelChild::TabDurSym(elem) => elem.serialize_mei(writer),
        DelChild::TabGrp(elem) => elem.serialize_mei(writer),
        DelChild::Table(elem) => elem.serialize_mei(writer),
        DelChild::Tempo(elem) => elem.serialize_mei(writer),
        DelChild::Term(elem) => elem.serialize_mei(writer),
        DelChild::Tie(elem) => elem.serialize_mei(writer),
        DelChild::Title(elem) => elem.serialize_mei(writer),
        DelChild::Trill(elem) => elem.serialize_mei(writer),
        DelChild::Tuplet(elem) => elem.serialize_mei(writer),
        DelChild::TupletSpan(elem) => elem.serialize_mei(writer),
        DelChild::Turn(elem) => elem.serialize_mei(writer),
        DelChild::Unclear(elem) => elem.serialize_mei(writer),
        DelChild::Verse(elem) => elem.serialize_mei(writer),
        DelChild::Volta(elem) => elem.serialize_mei(writer),
        DelChild::Width(elem) => elem.serialize_mei(writer),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========================================================================
    // App tests
    // ========================================================================

    #[test]
    fn app_serializes_empty() {
        let app = App::default();
        let xml = app.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<app/>");
    }

    #[test]
    fn app_serializes_with_xml_id() {
        let mut app = App::default();
        app.common.xml_id = Some("app1".to_string());
        let xml = app.to_mei_string().expect("should serialize");
        assert_eq!(xml, r#"<app xml:id="app1"/>"#);
    }

    #[test]
    fn app_serializes_with_lem_and_rdg() {
        let mut app = App::default();
        app.children.push(AppChild::Lem(Box::new(Lem::default())));
        app.children.push(AppChild::Rdg(Box::new(Rdg::default())));
        let xml = app.to_mei_string().expect("should serialize");
        assert!(xml.contains("<lem/>"));
        assert!(xml.contains("<rdg/>"));
    }

    // ========================================================================
    // Lem tests
    // ========================================================================

    #[test]
    fn lem_serializes_empty() {
        let lem = Lem::default();
        let xml = lem.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<lem/>");
    }

    #[test]
    fn lem_serializes_with_source() {
        let mut lem = Lem::default();
        lem.crit.source = vec![tusk_model::data::DataUri("#src1".to_string())];
        let xml = lem.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"source="#src1""##));
    }

    // ========================================================================
    // Rdg tests
    // ========================================================================

    #[test]
    fn rdg_serializes_empty() {
        let rdg = Rdg::default();
        let xml = rdg.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<rdg/>");
    }

    #[test]
    fn rdg_serializes_with_source() {
        let mut rdg = Rdg::default();
        rdg.crit.source = vec![
            tusk_model::data::DataUri("#src1".to_string()),
            tusk_model::data::DataUri("#src2".to_string()),
        ];
        let xml = rdg.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"source="#src1 #src2""##));
    }

    // ========================================================================
    // Choice tests
    // ========================================================================

    #[test]
    fn choice_serializes_empty() {
        let choice = Choice::default();
        let xml = choice.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<choice/>");
    }

    #[test]
    fn choice_serializes_with_sic_corr() {
        let mut choice = Choice::default();
        choice
            .children
            .push(ChoiceChild::Sic(Box::new(Sic::default())));
        choice
            .children
            .push(ChoiceChild::Corr(Box::new(Corr::default())));
        let xml = choice.to_mei_string().expect("should serialize");
        assert!(xml.contains("<sic/>"));
        assert!(xml.contains("<corr/>"));
    }

    // ========================================================================
    // Corr tests
    // ========================================================================

    #[test]
    fn corr_serializes_empty() {
        let corr = Corr::default();
        let xml = corr.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<corr/>");
    }

    #[test]
    fn corr_serializes_with_cert() {
        let mut corr = Corr::default();
        corr.edit.cert = Some(tusk_model::data::DataCertainty::High);
        let xml = corr.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"cert="high""#));
    }

    // ========================================================================
    // Sic tests
    // ========================================================================

    #[test]
    fn sic_serializes_empty() {
        let sic = Sic::default();
        let xml = sic.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<sic/>");
    }

    // ========================================================================
    // Add tests
    // ========================================================================

    #[test]
    fn add_serializes_empty() {
        let add = Add::default();
        let xml = add.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<add/>");
    }

    #[test]
    fn add_serializes_with_hand() {
        let mut add = Add::default();
        add.trans.hand = Some(tusk_model::data::DataUri("#h1".to_string()));
        let xml = add.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"hand="#h1""##));
    }

    // ========================================================================
    // Del tests
    // ========================================================================

    #[test]
    fn del_serializes_empty() {
        let del = Del::default();
        let xml = del.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<del/>");
    }

    #[test]
    fn del_serializes_with_hand() {
        let mut del = Del::default();
        del.trans.hand = Some(tusk_model::data::DataUri("#h1".to_string()));
        let xml = del.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"hand="#h1""##));
    }

    // ========================================================================
    // Abbr tests
    // ========================================================================

    #[test]
    fn abbr_serializes_empty() {
        let abbr = Abbr::default();
        let xml = abbr.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<abbr/>");
    }

    #[test]
    fn abbr_serializes_with_expan_attr() {
        let mut abbr = Abbr::default();
        abbr.expan = Some("Doctor".to_string());
        let xml = abbr.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"expan="Doctor""#));
    }

    // ========================================================================
    // Expan tests
    // ========================================================================

    #[test]
    fn expan_serializes_empty() {
        let expan = Expan::default();
        let xml = expan.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<expan/>");
    }

    #[test]
    fn expan_serializes_with_abbr_attr() {
        let mut expan = Expan::default();
        expan.abbr = Some("Dr.".to_string());
        let xml = expan.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"abbr="Dr.""#));
    }

    // ========================================================================
    // Orig tests
    // ========================================================================

    #[test]
    fn orig_serializes_empty() {
        let orig = Orig::default();
        let xml = orig.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<orig/>");
    }

    // ========================================================================
    // Reg tests
    // ========================================================================

    #[test]
    fn reg_serializes_empty() {
        let reg = Reg::default();
        let xml = reg.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<reg/>");
    }

    // ========================================================================
    // Subst tests
    // ========================================================================

    #[test]
    fn subst_serializes_empty() {
        let subst = Subst::default();
        let xml = subst.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<subst/>");
    }

    #[test]
    fn subst_serializes_with_add_del() {
        use tusk_model::elements::SubstChild;
        let mut subst = Subst::default();
        subst
            .children
            .push(SubstChild::Del(Box::new(Del::default())));
        subst
            .children
            .push(SubstChild::Add(Box::new(Add::default())));
        let xml = subst.to_mei_string().expect("should serialize");
        assert!(xml.contains("<del/>"));
        assert!(xml.contains("<add/>"));
    }

    // ========================================================================
    // Supplied tests
    // ========================================================================

    #[test]
    fn supplied_serializes_empty() {
        let supplied = Supplied::default();
        let xml = supplied.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<supplied/>");
    }

    #[test]
    fn supplied_serializes_with_reason() {
        let mut supplied = Supplied::default();
        supplied.reason_ident.reason = Some("lost".to_string());
        let xml = supplied.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"reason="lost""#));
    }

    // ========================================================================
    // Unclear tests
    // ========================================================================

    #[test]
    fn unclear_serializes_empty() {
        let unclear = Unclear::default();
        let xml = unclear.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<unclear/>");
    }

    #[test]
    fn unclear_serializes_with_reason() {
        let mut unclear = Unclear::default();
        unclear.reason_ident.reason = Some("faded".to_string());
        let xml = unclear.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"reason="faded""#));
    }

    // ========================================================================
    // Damage tests
    // ========================================================================

    #[test]
    fn damage_serializes_empty() {
        let damage = Damage::default();
        let xml = damage.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<damage/>");
    }

    #[test]
    fn damage_serializes_with_degree() {
        let mut damage = Damage::default();
        damage.degree = Some("medium".to_string());
        let xml = damage.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"degree="medium""#));
    }

    // ========================================================================
    // Gap tests
    // ========================================================================

    #[test]
    fn gap_serializes_empty() {
        let gap = Gap::default();
        let xml = gap.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<gap/>");
    }

    #[test]
    fn gap_serializes_with_reason() {
        let mut gap = Gap::default();
        gap.reason_ident.reason = Some("illegible".to_string());
        let xml = gap.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"reason="illegible""#));
    }

    // ========================================================================
    // Restore tests
    // ========================================================================

    #[test]
    fn restore_serializes_empty() {
        let restore = Restore::default();
        let xml = restore.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<restore/>");
    }

    #[test]
    fn restore_serializes_with_desc() {
        let mut restore = Restore::default();
        restore.desc = Some("deleted and restored".to_string());
        let xml = restore.to_mei_string().expect("should serialize");
        assert!(xml.contains(r#"desc="deleted and restored""#));
    }

    // ========================================================================
    // HandShift tests
    // ========================================================================

    #[test]
    fn hand_shift_serializes_empty() {
        let hand_shift = HandShift::default();
        let xml = hand_shift.to_mei_string().expect("should serialize");
        assert_eq!(xml, "<handShift/>");
    }

    #[test]
    fn hand_shift_serializes_with_new() {
        let mut hand_shift = HandShift::default();
        hand_shift.new = Some(tusk_model::data::DataUri("#h2".to_string()));
        let xml = hand_shift.to_mei_string().expect("should serialize");
        assert!(xml.contains(r##"new="#h2""##));
    }
}

//! Serializer implementations for analysis, gestural, and linkage MEI elements.
//!
//! This module contains serializers for Phase 12 elements:
//! - Ambitus (range of a voice/instrument/piece)
//! - AmbNote (highest/lowest pitch)
//! - OStaff (ossia staff)
//! - OLayer (ossia layer)
//! - Attacca (instruction to begin next section without pause)
//! - When (time point)
//! - Clip (time segment within a recording)
//! - Expansion (programmatic section expansion)
//! - CpMark (copy/colla parte mark)
//! - GenDesc (genetic description)
//! - GenState (genetic state)
//! - MetaMark (graphical/textual statement about musical text)

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttAmbitusAnl, AttAmbitusGes, AttAmbitusLog, AttAmbitusVis, AttAttaccaAnl, AttAttaccaGes,
    AttAttaccaLog, AttAttaccaVis, AttCpMarkAnl, AttCpMarkGes, AttCpMarkLog, AttCpMarkVis,
    AttMediaBounds, AttMetaMarkAnl, AttMetaMarkGes, AttMetaMarkLog, AttMetaMarkVis,
};
use tusk_model::elements::{
    Ambitus, AmbitusChild, Attacca, AttaccaChild, Clip, ClipChild, CpMark, CpMarkChild, Expansion,
    GenDesc, GenDescChild, GenState, GenStateChild, MetaMark, MetaMarkChild, OLayer, OLayerChild,
    OStaff, OStaffChild, When, WhenChild,
};

use super::push_attr;

// ============================================================================
// Attribute class implementations
// ============================================================================

// Note: AttAmbNoteLog/Vis/Ges/Anl are implemented in neumes.rs

// Note: AttPlist is implemented in misc.rs
// Note: AttSource is implemented in misc.rs

// Note: AttStaffLog/Vis/Ges/Anl are implemented in structure.rs
// Note: AttLayerLog/Vis/Ges/Anl are implemented in structure.rs
// Note: AmbNote is implemented in neumes.rs

// ============================================================================
// Ambitus element
// ============================================================================

impl MeiSerialize for Ambitus {
    fn element_name(&self) -> &'static str {
        "ambitus"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.ambitus_anl.collect_attributes());
        attrs.extend(self.ambitus_ges.collect_attributes());
        attrs.extend(self.ambitus_log.collect_attributes());
        attrs.extend(self.ambitus_vis.collect_attributes());
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

impl MeiSerialize for AmbitusChild {
    fn element_name(&self) -> &'static str {
        match self {
            AmbitusChild::AmbNote(_) => "ambNote",
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
            AmbitusChild::AmbNote(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// OStaff element
// ============================================================================

impl MeiSerialize for OStaff {
    fn element_name(&self) -> &'static str {
        "oStaff"
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

impl MeiSerialize for OStaffChild {
    fn element_name(&self) -> &'static str {
        match self {
            OStaffChild::Layer(_) => "layer",
            OStaffChild::RelationList(_) => "relationList",
            OStaffChild::Annot(_) => "annot",
            OStaffChild::Supplied(_) => "supplied",
            OStaffChild::MetaMark(_) => "metaMark",
            OStaffChild::Del(_) => "del",
            OStaffChild::AnchoredText(_) => "anchoredText",
            OStaffChild::Dir(_) => "dir",
            OStaffChild::Dynam(_) => "dynam",
            OStaffChild::FingGrp(_) => "fingGrp",
            OStaffChild::Corr(_) => "corr",
            OStaffChild::Damage(_) => "damage",
            OStaffChild::StageDir(_) => "stageDir",
            OStaffChild::App(_) => "app",
            OStaffChild::Harm(_) => "harm",
            OStaffChild::Gap(_) => "gap",
            OStaffChild::Reg(_) => "reg",
            OStaffChild::Subst(_) => "subst",
            OStaffChild::CpMark(_) => "cpMark",
            OStaffChild::Bend(_) => "bend",
            OStaffChild::Sp(_) => "sp",
            OStaffChild::Add(_) => "add",
            OStaffChild::StaffDef(_) => "staffDef",
            OStaffChild::Tempo(_) => "tempo",
            OStaffChild::ColLayout(_) => "colLayout",
            OStaffChild::Pb(_) => "pb",
            OStaffChild::Relation(_) => "relation",
            OStaffChild::Sic(_) => "sic",
            OStaffChild::Fing(_) => "fing",
            OStaffChild::Unclear(_) => "unclear",
            OStaffChild::Orig(_) => "orig",
            OStaffChild::Gliss(_) => "gliss",
            OStaffChild::RepeatMark(_) => "repeatMark",
            OStaffChild::Ornam(_) => "ornam",
            OStaffChild::Curve(_) => "curve",
            OStaffChild::Caesura(_) => "caesura",
            OStaffChild::Cb(_) => "cb",
            OStaffChild::Choice(_) => "choice",
            OStaffChild::Restore(_) => "restore",
            OStaffChild::Sb(_) => "sb",
            OStaffChild::Line(_) => "line",
            OStaffChild::Ossia(_) => "ossia",
            OStaffChild::HandShift(_) => "handShift",
            OStaffChild::Phrase(_) => "phrase",
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
        // Only serialize children that have MeiSerialize implementations
        // Other children are skipped for now
        match self {
            OStaffChild::Layer(elem) => elem.serialize_mei(writer),
            OStaffChild::Annot(elem) => elem.serialize_mei(writer),
            OStaffChild::Supplied(elem) => elem.serialize_mei(writer),
            OStaffChild::MetaMark(elem) => elem.serialize_mei(writer),
            OStaffChild::Del(elem) => elem.serialize_mei(writer),
            OStaffChild::AnchoredText(elem) => elem.serialize_mei(writer),
            OStaffChild::Dir(elem) => elem.serialize_mei(writer),
            OStaffChild::Dynam(elem) => elem.serialize_mei(writer),
            OStaffChild::FingGrp(elem) => elem.serialize_mei(writer),
            OStaffChild::Corr(elem) => elem.serialize_mei(writer),
            OStaffChild::Damage(elem) => elem.serialize_mei(writer),
            OStaffChild::StageDir(elem) => elem.serialize_mei(writer),
            OStaffChild::App(elem) => elem.serialize_mei(writer),
            OStaffChild::Harm(elem) => elem.serialize_mei(writer),
            OStaffChild::Gap(elem) => elem.serialize_mei(writer),
            OStaffChild::Reg(elem) => elem.serialize_mei(writer),
            OStaffChild::Subst(elem) => elem.serialize_mei(writer),
            OStaffChild::CpMark(elem) => elem.serialize_mei(writer),
            OStaffChild::Bend(elem) => elem.serialize_mei(writer),
            OStaffChild::Sp(elem) => elem.serialize_mei(writer),
            OStaffChild::Add(elem) => elem.serialize_mei(writer),
            OStaffChild::StaffDef(elem) => elem.serialize_mei(writer),
            OStaffChild::Tempo(elem) => elem.serialize_mei(writer),
            OStaffChild::Pb(elem) => elem.serialize_mei(writer),
            OStaffChild::Sic(elem) => elem.serialize_mei(writer),
            OStaffChild::Fing(elem) => elem.serialize_mei(writer),
            OStaffChild::Unclear(elem) => elem.serialize_mei(writer),
            OStaffChild::Orig(elem) => elem.serialize_mei(writer),
            OStaffChild::Gliss(elem) => elem.serialize_mei(writer),
            OStaffChild::RepeatMark(elem) => elem.serialize_mei(writer),
            OStaffChild::Curve(elem) => elem.serialize_mei(writer),
            OStaffChild::Caesura(elem) => elem.serialize_mei(writer),
            OStaffChild::Cb(elem) => elem.serialize_mei(writer),
            OStaffChild::Choice(elem) => elem.serialize_mei(writer),
            OStaffChild::Restore(elem) => elem.serialize_mei(writer),
            OStaffChild::Sb(elem) => elem.serialize_mei(writer),
            OStaffChild::Line(elem) => elem.serialize_mei(writer),
            OStaffChild::Ossia(elem) => elem.serialize_mei(writer),
            OStaffChild::HandShift(elem) => elem.serialize_mei(writer),
            OStaffChild::Phrase(elem) => elem.serialize_mei(writer),
            // Elements without MeiSerialize implementations are skipped
            _ => Ok(()),
        }
    }
}

// ============================================================================
// OLayer element
// ============================================================================

impl MeiSerialize for OLayer {
    fn element_name(&self) -> &'static str {
        "oLayer"
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

impl MeiSerialize for OLayerChild {
    fn element_name(&self) -> &'static str {
        match self {
            OLayerChild::Sic(_) => "sic",
            OLayerChild::MRest(_) => "mRest",
            OLayerChild::MeterSigGrp(_) => "meterSigGrp",
            OLayerChild::MultiRpt(_) => "multiRpt",
            OLayerChild::Corr(_) => "corr",
            OLayerChild::Clef(_) => "clef",
            OLayerChild::Space(_) => "space",
            OLayerChild::Supplied(_) => "supplied",
            OLayerChild::ColLayout(_) => "colLayout",
            OLayerChild::Ligature(_) => "ligature",
            OLayerChild::Beam(_) => "beam",
            OLayerChild::BTrem(_) => "bTrem",
            OLayerChild::AnchoredText(_) => "anchoredText",
            OLayerChild::Note(_) => "note",
            OLayerChild::Custos(_) => "custos",
            OLayerChild::Sb(_) => "sb",
            OLayerChild::Del(_) => "del",
            OLayerChild::Restore(_) => "restore",
            OLayerChild::TabDurSym(_) => "tabDurSym",
            OLayerChild::Proport(_) => "proport",
            OLayerChild::Artic(_) => "artic",
            OLayerChild::TabGrp(_) => "tabGrp",
            OLayerChild::Mensur(_) => "mensur",
            OLayerChild::Reg(_) => "reg",
            OLayerChild::Neume(_) => "neume",
            OLayerChild::Subst(_) => "subst",
            OLayerChild::Accid(_) => "accid",
            OLayerChild::HandShift(_) => "handShift",
            OLayerChild::App(_) => "app",
            OLayerChild::Choice(_) => "choice",
            OLayerChild::BarLine(_) => "barLine",
            OLayerChild::Rest(_) => "rest",
            OLayerChild::BeatRpt(_) => "beatRpt",
            OLayerChild::Cb(_) => "cb",
            OLayerChild::FTrem(_) => "fTrem",
            OLayerChild::Pb(_) => "pb",
            OLayerChild::Unclear(_) => "unclear",
            OLayerChild::Orig(_) => "orig",
            OLayerChild::MRpt2(_) => "mRpt2",
            OLayerChild::Gap(_) => "gap",
            OLayerChild::Damage(_) => "damage",
            OLayerChild::Line(_) => "line",
            OLayerChild::Curve(_) => "curve",
            OLayerChild::ClefGrp(_) => "clefGrp",
            OLayerChild::DivLine(_) => "divLine",
            OLayerChild::Add(_) => "add",
            OLayerChild::MRpt(_) => "mRpt",
            OLayerChild::KeySig(_) => "keySig",
            OLayerChild::MeterSig(_) => "meterSig",
            OLayerChild::HalfmRpt(_) => "halfmRpt",
            OLayerChild::Chord(_) => "chord",
            OLayerChild::Dot(_) => "dot",
            OLayerChild::Syllable(_) => "syllable",
            OLayerChild::Annot(_) => "annot",
            OLayerChild::MultiRest(_) => "multiRest",
            OLayerChild::GraceGrp(_) => "graceGrp",
            OLayerChild::Pad(_) => "pad",
            OLayerChild::Tuplet(_) => "tuplet",
            OLayerChild::MSpace(_) => "mSpace",
            OLayerChild::Midi(_) => "midi",
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
        // Only serialize children that have MeiSerialize implementations
        match self {
            OLayerChild::Sic(elem) => elem.serialize_mei(writer),
            OLayerChild::MRest(elem) => elem.serialize_mei(writer),
            OLayerChild::MeterSigGrp(elem) => elem.serialize_mei(writer),
            OLayerChild::MultiRpt(elem) => elem.serialize_mei(writer),
            OLayerChild::Corr(elem) => elem.serialize_mei(writer),
            OLayerChild::Clef(elem) => elem.serialize_mei(writer),
            OLayerChild::Space(elem) => elem.serialize_mei(writer),
            OLayerChild::Supplied(elem) => elem.serialize_mei(writer),
            OLayerChild::Ligature(elem) => elem.serialize_mei(writer),
            OLayerChild::Beam(elem) => elem.serialize_mei(writer),
            OLayerChild::BTrem(elem) => elem.serialize_mei(writer),
            OLayerChild::AnchoredText(elem) => elem.serialize_mei(writer),
            OLayerChild::Note(elem) => elem.serialize_mei(writer),
            OLayerChild::Custos(elem) => elem.serialize_mei(writer),
            OLayerChild::Sb(elem) => elem.serialize_mei(writer),
            OLayerChild::Del(elem) => elem.serialize_mei(writer),
            OLayerChild::Restore(elem) => elem.serialize_mei(writer),
            OLayerChild::TabDurSym(elem) => elem.serialize_mei(writer),
            OLayerChild::Proport(elem) => elem.serialize_mei(writer),
            OLayerChild::Artic(elem) => elem.serialize_mei(writer),
            OLayerChild::TabGrp(elem) => elem.serialize_mei(writer),
            OLayerChild::Mensur(elem) => elem.serialize_mei(writer),
            OLayerChild::Reg(elem) => elem.serialize_mei(writer),
            OLayerChild::Neume(elem) => elem.serialize_mei(writer),
            OLayerChild::Subst(elem) => elem.serialize_mei(writer),
            OLayerChild::Accid(elem) => elem.serialize_mei(writer),
            OLayerChild::HandShift(elem) => elem.serialize_mei(writer),
            OLayerChild::App(elem) => elem.serialize_mei(writer),
            OLayerChild::Choice(elem) => elem.serialize_mei(writer),
            OLayerChild::BarLine(elem) => elem.serialize_mei(writer),
            OLayerChild::Rest(elem) => elem.serialize_mei(writer),
            OLayerChild::BeatRpt(elem) => elem.serialize_mei(writer),
            OLayerChild::Cb(elem) => elem.serialize_mei(writer),
            OLayerChild::FTrem(elem) => elem.serialize_mei(writer),
            OLayerChild::Pb(elem) => elem.serialize_mei(writer),
            OLayerChild::Unclear(elem) => elem.serialize_mei(writer),
            OLayerChild::Orig(elem) => elem.serialize_mei(writer),
            OLayerChild::MRpt2(elem) => elem.serialize_mei(writer),
            OLayerChild::Gap(elem) => elem.serialize_mei(writer),
            OLayerChild::Damage(elem) => elem.serialize_mei(writer),
            OLayerChild::Line(elem) => elem.serialize_mei(writer),
            OLayerChild::Curve(elem) => elem.serialize_mei(writer),
            OLayerChild::ClefGrp(elem) => elem.serialize_mei(writer),
            OLayerChild::DivLine(elem) => elem.serialize_mei(writer),
            OLayerChild::Add(elem) => elem.serialize_mei(writer),
            OLayerChild::MRpt(elem) => elem.serialize_mei(writer),
            OLayerChild::KeySig(elem) => elem.serialize_mei(writer),
            OLayerChild::MeterSig(elem) => elem.serialize_mei(writer),
            OLayerChild::HalfmRpt(elem) => elem.serialize_mei(writer),
            OLayerChild::Chord(elem) => elem.serialize_mei(writer),
            OLayerChild::Dot(elem) => elem.serialize_mei(writer),
            OLayerChild::Syllable(elem) => elem.serialize_mei(writer),
            OLayerChild::Annot(elem) => elem.serialize_mei(writer),
            OLayerChild::MultiRest(elem) => elem.serialize_mei(writer),
            OLayerChild::GraceGrp(elem) => elem.serialize_mei(writer),
            OLayerChild::Pad(elem) => elem.serialize_mei(writer),
            OLayerChild::Tuplet(elem) => elem.serialize_mei(writer),
            OLayerChild::MSpace(elem) => elem.serialize_mei(writer),
            OLayerChild::Midi(elem) => elem.serialize_mei(writer),
            // Elements without MeiSerialize implementations are skipped
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Attacca element
// ============================================================================

impl MeiSerialize for Attacca {
    fn element_name(&self) -> &'static str {
        "attacca"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.attacca_anl.collect_attributes());
        attrs.extend(self.attacca_ges.collect_attributes());
        attrs.extend(self.attacca_log.collect_attributes());
        attrs.extend(self.attacca_vis.collect_attributes());
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

impl MeiSerialize for AttaccaChild {
    fn element_name(&self) -> &'static str {
        match self {
            AttaccaChild::Text(_) => "$text",
            AttaccaChild::BiblStruct(_) => "biblStruct",
            AttaccaChild::Expan(_) => "expan",
            AttaccaChild::Orig(_) => "orig",
            AttaccaChild::GeogName(_) => "geogName",
            AttaccaChild::Heraldry(_) => "heraldry",
            AttaccaChild::Curve(_) => "curve",
            AttaccaChild::Bibl(_) => "bibl",
            AttaccaChild::Dim(_) => "dim",
            AttaccaChild::Term(_) => "term",
            AttaccaChild::Stack(_) => "stack",
            AttaccaChild::Add(_) => "add",
            AttaccaChild::Height(_) => "height",
            AttaccaChild::PersName(_) => "persName",
            AttaccaChild::Reg(_) => "reg",
            AttaccaChild::Q(_) => "q",
            AttaccaChild::Repository(_) => "repository",
            AttaccaChild::Depth(_) => "depth",
            AttaccaChild::AnchoredText(_) => "anchoredText",
            AttaccaChild::LocusGrp(_) => "locusGrp",
            AttaccaChild::Settlement(_) => "settlement",
            AttaccaChild::Signatures(_) => "signatures",
            AttaccaChild::Identifier(_) => "identifier",
            AttaccaChild::Gap(_) => "gap",
            AttaccaChild::Abbr(_) => "abbr",
            AttaccaChild::Country(_) => "country",
            AttaccaChild::GeogFeat(_) => "geogFeat",
            AttaccaChild::Ptr(_) => "ptr",
            AttaccaChild::Num(_) => "num",
            AttaccaChild::Name(_) => "name",
            AttaccaChild::Catchwords(_) => "catchwords",
            AttaccaChild::Address(_) => "address",
            AttaccaChild::Annot(_) => "annot",
            AttaccaChild::Region(_) => "region",
            AttaccaChild::PostCode(_) => "postCode",
            AttaccaChild::PeriodName(_) => "periodName",
            AttaccaChild::Extent(_) => "extent",
            AttaccaChild::Corr(_) => "corr",
            AttaccaChild::Street(_) => "street",
            AttaccaChild::Line(_) => "line",
            AttaccaChild::Supplied(_) => "supplied",
            AttaccaChild::Unclear(_) => "unclear",
            AttaccaChild::Rend(_) => "rend",
            AttaccaChild::Fig(_) => "fig",
            AttaccaChild::StyleName(_) => "styleName",
            AttaccaChild::Dimensions(_) => "dimensions",
            AttaccaChild::Ref(_) => "ref",
            AttaccaChild::Choice(_) => "choice",
            AttaccaChild::Locus(_) => "locus",
            AttaccaChild::Subst(_) => "subst",
            AttaccaChild::Sic(_) => "sic",
            AttaccaChild::RelationList(_) => "relationList",
            AttaccaChild::Title(_) => "title",
            AttaccaChild::Lb(_) => "lb",
            AttaccaChild::HandShift(_) => "handShift",
            AttaccaChild::Bloc(_) => "bloc",
            AttaccaChild::Stamp(_) => "stamp",
            AttaccaChild::Restore(_) => "restore",
            AttaccaChild::Width(_) => "width",
            AttaccaChild::Damage(_) => "damage",
            AttaccaChild::District(_) => "district",
            AttaccaChild::Symbol(_) => "symbol",
            AttaccaChild::Date(_) => "date",
            AttaccaChild::Relation(_) => "relation",
            AttaccaChild::Del(_) => "del",
            AttaccaChild::Seg(_) => "seg",
            AttaccaChild::PostBox(_) => "postBox",
            AttaccaChild::SecFolio(_) => "secFolio",
            AttaccaChild::CorpName(_) => "corpName",
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
            AttaccaChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            AttaccaChild::BiblStruct(elem) => elem.serialize_mei(writer),
            AttaccaChild::Expan(elem) => elem.serialize_mei(writer),
            AttaccaChild::Orig(elem) => elem.serialize_mei(writer),
            AttaccaChild::GeogName(elem) => elem.serialize_mei(writer),
            AttaccaChild::Curve(elem) => elem.serialize_mei(writer),
            AttaccaChild::Bibl(elem) => elem.serialize_mei(writer),
            AttaccaChild::Term(elem) => elem.serialize_mei(writer),
            AttaccaChild::Stack(elem) => elem.serialize_mei(writer),
            AttaccaChild::Add(elem) => elem.serialize_mei(writer),
            AttaccaChild::PersName(elem) => elem.serialize_mei(writer),
            AttaccaChild::Reg(elem) => elem.serialize_mei(writer),
            AttaccaChild::Q(elem) => elem.serialize_mei(writer),
            AttaccaChild::AnchoredText(elem) => elem.serialize_mei(writer),
            AttaccaChild::LocusGrp(elem) => elem.serialize_mei(writer),
            AttaccaChild::Settlement(elem) => elem.serialize_mei(writer),
            AttaccaChild::Identifier(elem) => elem.serialize_mei(writer),
            AttaccaChild::Gap(elem) => elem.serialize_mei(writer),
            AttaccaChild::Abbr(elem) => elem.serialize_mei(writer),
            AttaccaChild::Country(elem) => elem.serialize_mei(writer),
            AttaccaChild::GeogFeat(elem) => elem.serialize_mei(writer),
            AttaccaChild::Ptr(elem) => elem.serialize_mei(writer),
            AttaccaChild::Num(elem) => elem.serialize_mei(writer),
            AttaccaChild::Name(elem) => elem.serialize_mei(writer),
            AttaccaChild::Address(elem) => elem.serialize_mei(writer),
            AttaccaChild::Annot(elem) => elem.serialize_mei(writer),
            AttaccaChild::Region(elem) => elem.serialize_mei(writer),
            AttaccaChild::PostCode(elem) => elem.serialize_mei(writer),
            AttaccaChild::PeriodName(elem) => elem.serialize_mei(writer),
            AttaccaChild::Extent(elem) => elem.serialize_mei(writer),
            AttaccaChild::Corr(elem) => elem.serialize_mei(writer),
            AttaccaChild::Street(elem) => elem.serialize_mei(writer),
            AttaccaChild::Line(elem) => elem.serialize_mei(writer),
            AttaccaChild::Supplied(elem) => elem.serialize_mei(writer),
            AttaccaChild::Unclear(elem) => elem.serialize_mei(writer),
            AttaccaChild::Rend(elem) => elem.serialize_mei(writer),
            AttaccaChild::Fig(elem) => elem.serialize_mei(writer),
            AttaccaChild::StyleName(elem) => elem.serialize_mei(writer),
            AttaccaChild::Ref(elem) => elem.serialize_mei(writer),
            AttaccaChild::Choice(elem) => elem.serialize_mei(writer),
            AttaccaChild::Locus(elem) => elem.serialize_mei(writer),
            AttaccaChild::Subst(elem) => elem.serialize_mei(writer),
            AttaccaChild::Sic(elem) => elem.serialize_mei(writer),
            AttaccaChild::Title(elem) => elem.serialize_mei(writer),
            AttaccaChild::Lb(elem) => elem.serialize_mei(writer),
            AttaccaChild::HandShift(elem) => elem.serialize_mei(writer),
            AttaccaChild::Bloc(elem) => elem.serialize_mei(writer),
            AttaccaChild::Stamp(elem) => elem.serialize_mei(writer),
            AttaccaChild::Restore(elem) => elem.serialize_mei(writer),
            AttaccaChild::Damage(elem) => elem.serialize_mei(writer),
            AttaccaChild::District(elem) => elem.serialize_mei(writer),
            AttaccaChild::Symbol(elem) => elem.serialize_mei(writer),
            AttaccaChild::Date(elem) => elem.serialize_mei(writer),
            AttaccaChild::Del(elem) => elem.serialize_mei(writer),
            AttaccaChild::Seg(elem) => elem.serialize_mei(writer),
            AttaccaChild::PostBox(elem) => elem.serialize_mei(writer),
            AttaccaChild::CorpName(elem) => elem.serialize_mei(writer),
            // Elements without MeiSerialize implementations are skipped
            _ => Ok(()),
        }
    }
}

// ============================================================================
// When element
// ============================================================================

impl MeiSerialize for When {
    fn element_name(&self) -> &'static str {
        "when"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        push_attr!(attrs, "absolute", string self.absolute);
        push_attr!(attrs, "interval", string self.interval);
        push_attr!(attrs, "abstype", self.abstype);
        push_attr!(attrs, "inttype", self.inttype);
        push_attr!(attrs, "since", self.since);
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

impl MeiSerialize for WhenChild {
    fn element_name(&self) -> &'static str {
        match self {
            WhenChild::ExtData(_) => "extData",
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
            // ExtData does not have MeiSerialize implementation yet
            WhenChild::ExtData(_) => Ok(()),
        }
    }
}

// ============================================================================
// Clip element
// ============================================================================

impl MeiSerialize for Clip {
    fn element_name(&self) -> &'static str {
        "clip"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.media_bounds.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.start_id.collect_attributes());
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

impl MeiSerialize for ClipChild {
    fn element_name(&self) -> &'static str {
        match self {
            ClipChild::When(_) => "when",
            ClipChild::AvFile(_) => "avFile",
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
            ClipChild::When(elem) => elem.serialize_mei(writer),
            // AvFile does not have MeiSerialize implementation yet
            ClipChild::AvFile(_) => Ok(()),
        }
    }
}

// ============================================================================
// Expansion element
// ============================================================================

impl MeiSerialize for Expansion {
    fn element_name(&self) -> &'static str {
        "expansion"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.plist.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
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
// CpMark element
// ============================================================================

impl MeiSerialize for CpMark {
    fn element_name(&self) -> &'static str {
        "cpMark"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.cp_mark_log.collect_attributes());
        attrs.extend(self.cp_mark_vis.collect_attributes());
        attrs.extend(self.cp_mark_ges.collect_attributes());
        attrs.extend(self.cp_mark_anl.collect_attributes());
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

impl MeiSerialize for CpMarkChild {
    fn element_name(&self) -> &'static str {
        match self {
            CpMarkChild::Text(_) => "$text",
            CpMarkChild::Term(_) => "term",
            CpMarkChild::Dim(_) => "dim",
            CpMarkChild::Reg(_) => "reg",
            CpMarkChild::Street(_) => "street",
            CpMarkChild::Dimensions(_) => "dimensions",
            CpMarkChild::Q(_) => "q",
            CpMarkChild::Identifier(_) => "identifier",
            CpMarkChild::Address(_) => "address",
            CpMarkChild::LocusGrp(_) => "locusGrp",
            CpMarkChild::PostBox(_) => "postBox",
            CpMarkChild::Gap(_) => "gap",
            CpMarkChild::CorpName(_) => "corpName",
            CpMarkChild::Width(_) => "width",
            CpMarkChild::Supplied(_) => "supplied",
            CpMarkChild::HandShift(_) => "handShift",
            CpMarkChild::Bloc(_) => "bloc",
            CpMarkChild::Corr(_) => "corr",
            CpMarkChild::Num(_) => "num",
            CpMarkChild::Choice(_) => "choice",
            CpMarkChild::Annot(_) => "annot",
            CpMarkChild::Region(_) => "region",
            CpMarkChild::Relation(_) => "relation",
            CpMarkChild::Sic(_) => "sic",
            CpMarkChild::Stamp(_) => "stamp",
            CpMarkChild::Date(_) => "date",
            CpMarkChild::PeriodName(_) => "periodName",
            CpMarkChild::Stack(_) => "stack",
            CpMarkChild::Extent(_) => "extent",
            CpMarkChild::Ref(_) => "ref",
            CpMarkChild::Bibl(_) => "bibl",
            CpMarkChild::GeogName(_) => "geogName",
            CpMarkChild::PostCode(_) => "postCode",
            CpMarkChild::Locus(_) => "locus",
            CpMarkChild::Symbol(_) => "symbol",
            CpMarkChild::Unclear(_) => "unclear",
            CpMarkChild::Fig(_) => "fig",
            CpMarkChild::Orig(_) => "orig",
            CpMarkChild::Abbr(_) => "abbr",
            CpMarkChild::Rend(_) => "rend",
            CpMarkChild::Title(_) => "title",
            CpMarkChild::PersName(_) => "persName",
            CpMarkChild::Name(_) => "name",
            CpMarkChild::RelationList(_) => "relationList",
            CpMarkChild::Settlement(_) => "settlement",
            CpMarkChild::Subst(_) => "subst",
            CpMarkChild::BiblStruct(_) => "biblStruct",
            CpMarkChild::Heraldry(_) => "heraldry",
            CpMarkChild::StyleName(_) => "styleName",
            CpMarkChild::Repository(_) => "repository",
            CpMarkChild::Country(_) => "country",
            CpMarkChild::Add(_) => "add",
            CpMarkChild::Damage(_) => "damage",
            CpMarkChild::Catchwords(_) => "catchwords",
            CpMarkChild::Restore(_) => "restore",
            CpMarkChild::District(_) => "district",
            CpMarkChild::Ptr(_) => "ptr",
            CpMarkChild::Height(_) => "height",
            CpMarkChild::GeogFeat(_) => "geogFeat",
            CpMarkChild::Expan(_) => "expan",
            CpMarkChild::Del(_) => "del",
            CpMarkChild::Depth(_) => "depth",
            CpMarkChild::Seg(_) => "seg",
            CpMarkChild::Lb(_) => "lb",
            CpMarkChild::Signatures(_) => "signatures",
            CpMarkChild::SecFolio(_) => "secFolio",
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
            CpMarkChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CpMarkChild::Term(elem) => elem.serialize_mei(writer),
            CpMarkChild::Reg(elem) => elem.serialize_mei(writer),
            CpMarkChild::Street(elem) => elem.serialize_mei(writer),
            CpMarkChild::Q(elem) => elem.serialize_mei(writer),
            CpMarkChild::Identifier(elem) => elem.serialize_mei(writer),
            CpMarkChild::Address(elem) => elem.serialize_mei(writer),
            CpMarkChild::LocusGrp(elem) => elem.serialize_mei(writer),
            CpMarkChild::PostBox(elem) => elem.serialize_mei(writer),
            CpMarkChild::Gap(elem) => elem.serialize_mei(writer),
            CpMarkChild::CorpName(elem) => elem.serialize_mei(writer),
            CpMarkChild::Supplied(elem) => elem.serialize_mei(writer),
            CpMarkChild::HandShift(elem) => elem.serialize_mei(writer),
            CpMarkChild::Bloc(elem) => elem.serialize_mei(writer),
            CpMarkChild::Corr(elem) => elem.serialize_mei(writer),
            CpMarkChild::Num(elem) => elem.serialize_mei(writer),
            CpMarkChild::Choice(elem) => elem.serialize_mei(writer),
            CpMarkChild::Annot(elem) => elem.serialize_mei(writer),
            CpMarkChild::Region(elem) => elem.serialize_mei(writer),
            CpMarkChild::Sic(elem) => elem.serialize_mei(writer),
            CpMarkChild::Stamp(elem) => elem.serialize_mei(writer),
            CpMarkChild::Date(elem) => elem.serialize_mei(writer),
            CpMarkChild::PeriodName(elem) => elem.serialize_mei(writer),
            CpMarkChild::Stack(elem) => elem.serialize_mei(writer),
            CpMarkChild::Extent(elem) => elem.serialize_mei(writer),
            CpMarkChild::Ref(elem) => elem.serialize_mei(writer),
            CpMarkChild::Bibl(elem) => elem.serialize_mei(writer),
            CpMarkChild::GeogName(elem) => elem.serialize_mei(writer),
            CpMarkChild::PostCode(elem) => elem.serialize_mei(writer),
            CpMarkChild::Locus(elem) => elem.serialize_mei(writer),
            CpMarkChild::Symbol(elem) => elem.serialize_mei(writer),
            CpMarkChild::Unclear(elem) => elem.serialize_mei(writer),
            CpMarkChild::Fig(elem) => elem.serialize_mei(writer),
            CpMarkChild::Orig(elem) => elem.serialize_mei(writer),
            CpMarkChild::Abbr(elem) => elem.serialize_mei(writer),
            CpMarkChild::Rend(elem) => elem.serialize_mei(writer),
            CpMarkChild::Title(elem) => elem.serialize_mei(writer),
            CpMarkChild::PersName(elem) => elem.serialize_mei(writer),
            CpMarkChild::Name(elem) => elem.serialize_mei(writer),
            CpMarkChild::Settlement(elem) => elem.serialize_mei(writer),
            CpMarkChild::Subst(elem) => elem.serialize_mei(writer),
            CpMarkChild::BiblStruct(elem) => elem.serialize_mei(writer),
            CpMarkChild::StyleName(elem) => elem.serialize_mei(writer),
            CpMarkChild::Country(elem) => elem.serialize_mei(writer),
            CpMarkChild::Add(elem) => elem.serialize_mei(writer),
            CpMarkChild::Damage(elem) => elem.serialize_mei(writer),
            CpMarkChild::Restore(elem) => elem.serialize_mei(writer),
            CpMarkChild::District(elem) => elem.serialize_mei(writer),
            CpMarkChild::Ptr(elem) => elem.serialize_mei(writer),
            CpMarkChild::GeogFeat(elem) => elem.serialize_mei(writer),
            CpMarkChild::Expan(elem) => elem.serialize_mei(writer),
            CpMarkChild::Del(elem) => elem.serialize_mei(writer),
            CpMarkChild::Seg(elem) => elem.serialize_mei(writer),
            CpMarkChild::Lb(elem) => elem.serialize_mei(writer),
            // Elements without MeiSerialize implementations are skipped
            _ => Ok(()),
        }
    }
}

// ============================================================================
// GenDesc element
// ============================================================================

impl MeiSerialize for GenDesc {
    fn element_name(&self) -> &'static str {
        "genDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        push_attr!(attrs, "ordered", self.ordered);
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

impl MeiSerialize for GenDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            GenDescChild::GenDesc(_) => "genDesc",
            GenDescChild::GenState(_) => "genState",
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
            GenDescChild::GenDesc(elem) => elem.serialize_mei(writer),
            GenDescChild::GenState(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// GenState element
// ============================================================================

impl MeiSerialize for GenState {
    fn element_name(&self) -> &'static str {
        "genState"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
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

impl MeiSerialize for GenStateChild {
    fn element_name(&self) -> &'static str {
        match self {
            GenStateChild::Date(_) => "date",
            GenStateChild::Desc(_) => "desc",
            GenStateChild::RespStmt(_) => "respStmt",
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
            GenStateChild::Date(elem) => elem.serialize_mei(writer),
            GenStateChild::RespStmt(elem) => elem.serialize_mei(writer),
            // Desc does not have MeiSerialize implementation yet
            GenStateChild::Desc(_) => Ok(()),
        }
    }
}

// ============================================================================
// MetaMark element
// ============================================================================

impl MeiSerialize for MetaMark {
    fn element_name(&self) -> &'static str {
        "metaMark"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.meta_mark_log.collect_attributes());
        attrs.extend(self.meta_mark_vis.collect_attributes());
        attrs.extend(self.meta_mark_ges.collect_attributes());
        attrs.extend(self.meta_mark_anl.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        push_attr!(attrs, "function", string self.function);
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

impl MeiSerialize for MetaMarkChild {
    fn element_name(&self) -> &'static str {
        match self {
            MetaMarkChild::Text(_) => "$text",
            MetaMarkChild::BiblStruct(_) => "biblStruct",
            MetaMarkChild::Section(_) => "section",
            MetaMarkChild::Choice(_) => "choice",
            MetaMarkChild::Del(_) => "del",
            MetaMarkChild::Dim(_) => "dim",
            MetaMarkChild::Sic(_) => "sic",
            MetaMarkChild::GeogName(_) => "geogName",
            MetaMarkChild::Add(_) => "add",
            MetaMarkChild::Num(_) => "num",
            MetaMarkChild::Width(_) => "width",
            MetaMarkChild::Seg(_) => "seg",
            MetaMarkChild::Bibl(_) => "bibl",
            MetaMarkChild::Title(_) => "title",
            MetaMarkChild::Region(_) => "region",
            MetaMarkChild::Locus(_) => "locus",
            MetaMarkChild::Annot(_) => "annot",
            MetaMarkChild::Supplied(_) => "supplied",
            MetaMarkChild::Relation(_) => "relation",
            MetaMarkChild::Reg(_) => "reg",
            MetaMarkChild::District(_) => "district",
            MetaMarkChild::GeogFeat(_) => "geogFeat",
            MetaMarkChild::LocusGrp(_) => "locusGrp",
            MetaMarkChild::Restore(_) => "restore",
            MetaMarkChild::Heraldry(_) => "heraldry",
            MetaMarkChild::Address(_) => "address",
            MetaMarkChild::Extent(_) => "extent",
            MetaMarkChild::Abbr(_) => "abbr",
            MetaMarkChild::Country(_) => "country",
            MetaMarkChild::Street(_) => "street",
            MetaMarkChild::RelationList(_) => "relationList",
            MetaMarkChild::Settlement(_) => "settlement",
            MetaMarkChild::Term(_) => "term",
            MetaMarkChild::Depth(_) => "depth",
            MetaMarkChild::Q(_) => "q",
            MetaMarkChild::Signatures(_) => "signatures",
            MetaMarkChild::Stack(_) => "stack",
            MetaMarkChild::Corr(_) => "corr",
            MetaMarkChild::Bloc(_) => "bloc",
            MetaMarkChild::SecFolio(_) => "secFolio",
            MetaMarkChild::Expan(_) => "expan",
            MetaMarkChild::Ptr(_) => "ptr",
            MetaMarkChild::PeriodName(_) => "periodName",
            MetaMarkChild::Symbol(_) => "symbol",
            MetaMarkChild::Dimensions(_) => "dimensions",
            MetaMarkChild::Damage(_) => "damage",
            MetaMarkChild::Date(_) => "date",
            MetaMarkChild::Lb(_) => "lb",
            MetaMarkChild::Rend(_) => "rend",
            MetaMarkChild::Unclear(_) => "unclear",
            MetaMarkChild::Height(_) => "height",
            MetaMarkChild::Ref(_) => "ref",
            MetaMarkChild::Stamp(_) => "stamp",
            MetaMarkChild::PostCode(_) => "postCode",
            MetaMarkChild::Subst(_) => "subst",
            MetaMarkChild::Name(_) => "name",
            MetaMarkChild::HandShift(_) => "handShift",
            MetaMarkChild::CorpName(_) => "corpName",
            MetaMarkChild::Fig(_) => "fig",
            MetaMarkChild::PostBox(_) => "postBox",
            MetaMarkChild::Gap(_) => "gap",
            MetaMarkChild::Orig(_) => "orig",
            MetaMarkChild::Catchwords(_) => "catchwords",
            MetaMarkChild::PersName(_) => "persName",
            MetaMarkChild::Repository(_) => "repository",
            MetaMarkChild::StyleName(_) => "styleName",
            MetaMarkChild::Identifier(_) => "identifier",
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
            MetaMarkChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            MetaMarkChild::BiblStruct(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Section(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Choice(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Del(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Sic(elem) => elem.serialize_mei(writer),
            MetaMarkChild::GeogName(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Add(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Num(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Seg(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Bibl(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Title(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Region(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Locus(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Annot(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Supplied(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Reg(elem) => elem.serialize_mei(writer),
            MetaMarkChild::District(elem) => elem.serialize_mei(writer),
            MetaMarkChild::GeogFeat(elem) => elem.serialize_mei(writer),
            MetaMarkChild::LocusGrp(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Restore(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Address(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Extent(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Abbr(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Country(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Street(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Settlement(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Term(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Q(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Stack(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Corr(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Bloc(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Expan(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Ptr(elem) => elem.serialize_mei(writer),
            MetaMarkChild::PeriodName(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Symbol(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Damage(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Date(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Lb(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Rend(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Unclear(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Ref(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Stamp(elem) => elem.serialize_mei(writer),
            MetaMarkChild::PostCode(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Subst(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Name(elem) => elem.serialize_mei(writer),
            MetaMarkChild::HandShift(elem) => elem.serialize_mei(writer),
            MetaMarkChild::CorpName(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Fig(elem) => elem.serialize_mei(writer),
            MetaMarkChild::PostBox(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Gap(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Orig(elem) => elem.serialize_mei(writer),
            MetaMarkChild::PersName(elem) => elem.serialize_mei(writer),
            MetaMarkChild::StyleName(elem) => elem.serialize_mei(writer),
            MetaMarkChild::Identifier(elem) => elem.serialize_mei(writer),
            // Elements without MeiSerialize implementations are skipped
            _ => Ok(()),
        }
    }
}

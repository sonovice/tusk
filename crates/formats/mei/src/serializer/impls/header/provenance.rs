//! Serializer implementations for provenance, history, watermark, and type description elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    AccMat, AccMatChild, Acquisition, AcquisitionChild, AddDesc, AddDescChild, ExhibHist,
    ExhibHistChild, Provenance, ProvenanceChild, TypeDesc, TypeDescChild, TypeNote, TypeNoteChild,
    Watermark, WatermarkChild, WatermarkDesc, WatermarkDescChild, WatermarkList,
    WatermarkListChild,
};

// ============================================================================
// Provenance
// ============================================================================

impl MeiSerialize for Provenance {
    fn element_name(&self) -> &'static str {
        "provenance"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for ProvenanceChild {
    fn element_name(&self) -> &'static str {
        ""
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
            ProvenanceChild::Text(text) => writer.write_text(text),
            ProvenanceChild::Head(elem) => elem.serialize_mei(writer),
            ProvenanceChild::P(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Date(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Bibl(elem) => elem.serialize_mei(writer),
            ProvenanceChild::BiblStruct(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Annot(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Ptr(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Ref(elem) => elem.serialize_mei(writer),
            ProvenanceChild::PersName(elem) => elem.serialize_mei(writer),
            ProvenanceChild::CorpName(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Name(elem) => elem.serialize_mei(writer),
            ProvenanceChild::GeogName(elem) => elem.serialize_mei(writer),
            ProvenanceChild::GeogFeat(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Address(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Country(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Region(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Settlement(elem) => elem.serialize_mei(writer),
            ProvenanceChild::District(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Bloc(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Dimensions(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Height(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Width(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Depth(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Dim(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Term(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Lb(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Rend(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Num(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Fig(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Seg(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Identifier(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Locus(elem) => elem.serialize_mei(writer),
            ProvenanceChild::LocusGrp(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Title(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Symbol(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Q(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Extent(elem) => elem.serialize_mei(writer),
            ProvenanceChild::EventList(elem) => elem.serialize_mei(writer),
            ProvenanceChild::RelationList(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Relation(elem) => elem.serialize_mei(writer),
            ProvenanceChild::PeriodName(elem) => elem.serialize_mei(writer),
            ProvenanceChild::StyleName(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Abbr(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Expan(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Stack(elem) => elem.serialize_mei(writer),
            ProvenanceChild::PostBox(elem) => elem.serialize_mei(writer),
            ProvenanceChild::PostCode(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Street(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Repository(elem) => elem.serialize_mei(writer),
            ProvenanceChild::Heraldry(_) => Ok(()), // Not yet implemented
            ProvenanceChild::SecFolio(_) => Ok(()), // Not yet implemented
            ProvenanceChild::Catchwords(_) => Ok(()), // Not yet implemented
            ProvenanceChild::Signatures(_) => Ok(()), // Not yet implemented
            ProvenanceChild::Stamp(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Acquisition
// ============================================================================

impl MeiSerialize for Acquisition {
    fn element_name(&self) -> &'static str {
        "acquisition"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for AcquisitionChild {
    fn element_name(&self) -> &'static str {
        ""
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
            AcquisitionChild::Text(text) => writer.write_text(text),
            AcquisitionChild::Head(elem) => elem.serialize_mei(writer),
            AcquisitionChild::P(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Date(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Bibl(elem) => elem.serialize_mei(writer),
            AcquisitionChild::BiblStruct(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Annot(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Ptr(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Ref(elem) => elem.serialize_mei(writer),
            AcquisitionChild::PersName(elem) => elem.serialize_mei(writer),
            AcquisitionChild::CorpName(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Name(elem) => elem.serialize_mei(writer),
            AcquisitionChild::GeogName(elem) => elem.serialize_mei(writer),
            AcquisitionChild::GeogFeat(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Address(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Country(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Region(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Settlement(elem) => elem.serialize_mei(writer),
            AcquisitionChild::District(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Bloc(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Dimensions(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Height(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Width(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Depth(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Dim(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Term(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Lb(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Rend(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Num(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Fig(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Seg(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Identifier(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Locus(elem) => elem.serialize_mei(writer),
            AcquisitionChild::LocusGrp(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Title(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Symbol(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Q(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Extent(elem) => elem.serialize_mei(writer),
            AcquisitionChild::EventList(elem) => elem.serialize_mei(writer),
            AcquisitionChild::RelationList(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Relation(elem) => elem.serialize_mei(writer),
            AcquisitionChild::PeriodName(elem) => elem.serialize_mei(writer),
            AcquisitionChild::StyleName(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Abbr(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Expan(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Stack(elem) => elem.serialize_mei(writer),
            AcquisitionChild::PostBox(elem) => elem.serialize_mei(writer),
            AcquisitionChild::PostCode(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Street(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Repository(elem) => elem.serialize_mei(writer),
            AcquisitionChild::Heraldry(_) => Ok(()), // Not yet implemented
            AcquisitionChild::SecFolio(_) => Ok(()), // Not yet implemented
            AcquisitionChild::Catchwords(_) => Ok(()), // Not yet implemented
            AcquisitionChild::Signatures(_) => Ok(()), // Not yet implemented
            AcquisitionChild::Stamp(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// ExhibHist
// ============================================================================

impl MeiSerialize for ExhibHist {
    fn element_name(&self) -> &'static str {
        "exhibHist"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for ExhibHistChild {
    fn element_name(&self) -> &'static str {
        ""
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
            ExhibHistChild::Text(text) => writer.write_text(text),
            ExhibHistChild::Head(elem) => elem.serialize_mei(writer),
            ExhibHistChild::P(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Date(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Bibl(elem) => elem.serialize_mei(writer),
            ExhibHistChild::BiblStruct(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Annot(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Ptr(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Ref(elem) => elem.serialize_mei(writer),
            ExhibHistChild::PersName(elem) => elem.serialize_mei(writer),
            ExhibHistChild::CorpName(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Name(elem) => elem.serialize_mei(writer),
            ExhibHistChild::GeogName(elem) => elem.serialize_mei(writer),
            ExhibHistChild::GeogFeat(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Address(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Country(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Region(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Settlement(elem) => elem.serialize_mei(writer),
            ExhibHistChild::District(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Bloc(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Dimensions(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Height(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Width(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Depth(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Dim(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Term(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Lb(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Rend(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Num(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Fig(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Seg(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Identifier(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Locus(elem) => elem.serialize_mei(writer),
            ExhibHistChild::LocusGrp(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Title(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Symbol(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Q(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Extent(elem) => elem.serialize_mei(writer),
            ExhibHistChild::EventList(elem) => elem.serialize_mei(writer),
            ExhibHistChild::RelationList(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Relation(elem) => elem.serialize_mei(writer),
            ExhibHistChild::PeriodName(elem) => elem.serialize_mei(writer),
            ExhibHistChild::StyleName(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Abbr(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Expan(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Stack(elem) => elem.serialize_mei(writer),
            ExhibHistChild::PostBox(elem) => elem.serialize_mei(writer),
            ExhibHistChild::PostCode(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Street(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Repository(elem) => elem.serialize_mei(writer),
            ExhibHistChild::Heraldry(_) => Ok(()), // Not yet implemented
            ExhibHistChild::SecFolio(_) => Ok(()), // Not yet implemented
            ExhibHistChild::Catchwords(_) => Ok(()), // Not yet implemented
            ExhibHistChild::Signatures(_) => Ok(()), // Not yet implemented
            ExhibHistChild::Stamp(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// AccMat
// ============================================================================

impl MeiSerialize for AccMat {
    fn element_name(&self) -> &'static str {
        "accMat"
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

impl MeiSerialize for AccMatChild {
    fn element_name(&self) -> &'static str {
        ""
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
            AccMatChild::Text(text) => writer.write_text(text),
            AccMatChild::Head(elem) => elem.serialize_mei(writer),
            AccMatChild::P(elem) => elem.serialize_mei(writer),
            AccMatChild::Date(elem) => elem.serialize_mei(writer),
            AccMatChild::Bibl(elem) => elem.serialize_mei(writer),
            AccMatChild::BiblStruct(elem) => elem.serialize_mei(writer),
            AccMatChild::Annot(elem) => elem.serialize_mei(writer),
            AccMatChild::Ptr(elem) => elem.serialize_mei(writer),
            AccMatChild::Ref(elem) => elem.serialize_mei(writer),
            AccMatChild::PersName(elem) => elem.serialize_mei(writer),
            AccMatChild::CorpName(elem) => elem.serialize_mei(writer),
            AccMatChild::Name(elem) => elem.serialize_mei(writer),
            AccMatChild::GeogName(elem) => elem.serialize_mei(writer),
            AccMatChild::GeogFeat(elem) => elem.serialize_mei(writer),
            AccMatChild::Address(elem) => elem.serialize_mei(writer),
            AccMatChild::Country(elem) => elem.serialize_mei(writer),
            AccMatChild::Region(elem) => elem.serialize_mei(writer),
            AccMatChild::Settlement(elem) => elem.serialize_mei(writer),
            AccMatChild::District(elem) => elem.serialize_mei(writer),
            AccMatChild::Bloc(elem) => elem.serialize_mei(writer),
            AccMatChild::Dimensions(elem) => elem.serialize_mei(writer),
            AccMatChild::Height(elem) => elem.serialize_mei(writer),
            AccMatChild::Width(elem) => elem.serialize_mei(writer),
            AccMatChild::Depth(elem) => elem.serialize_mei(writer),
            AccMatChild::Dim(elem) => elem.serialize_mei(writer),
            AccMatChild::Term(elem) => elem.serialize_mei(writer),
            AccMatChild::Lb(elem) => elem.serialize_mei(writer),
            AccMatChild::Rend(elem) => elem.serialize_mei(writer),
            AccMatChild::Num(elem) => elem.serialize_mei(writer),
            AccMatChild::Fig(elem) => elem.serialize_mei(writer),
            AccMatChild::Seg(elem) => elem.serialize_mei(writer),
            AccMatChild::Identifier(elem) => elem.serialize_mei(writer),
            AccMatChild::Locus(elem) => elem.serialize_mei(writer),
            AccMatChild::LocusGrp(elem) => elem.serialize_mei(writer),
            AccMatChild::Title(elem) => elem.serialize_mei(writer),
            AccMatChild::Symbol(elem) => elem.serialize_mei(writer),
            AccMatChild::Q(elem) => elem.serialize_mei(writer),
            AccMatChild::Extent(elem) => elem.serialize_mei(writer),
            AccMatChild::RelationList(elem) => elem.serialize_mei(writer),
            AccMatChild::Relation(elem) => elem.serialize_mei(writer),
            AccMatChild::PeriodName(elem) => elem.serialize_mei(writer),
            AccMatChild::StyleName(elem) => elem.serialize_mei(writer),
            AccMatChild::Abbr(elem) => elem.serialize_mei(writer),
            AccMatChild::Expan(elem) => elem.serialize_mei(writer),
            AccMatChild::Stack(elem) => elem.serialize_mei(writer),
            AccMatChild::PostBox(elem) => elem.serialize_mei(writer),
            AccMatChild::PostCode(elem) => elem.serialize_mei(writer),
            AccMatChild::Street(elem) => elem.serialize_mei(writer),
            AccMatChild::Repository(elem) => elem.serialize_mei(writer),
            AccMatChild::Heraldry(_) => Ok(()), // Not yet implemented
            AccMatChild::SecFolio(_) => Ok(()), // Not yet implemented
            AccMatChild::Catchwords(_) => Ok(()), // Not yet implemented
            AccMatChild::Signatures(_) => Ok(()), // Not yet implemented
            AccMatChild::Stamp(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// AddDesc
// ============================================================================

impl MeiSerialize for AddDesc {
    fn element_name(&self) -> &'static str {
        "addDesc"
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

impl MeiSerialize for AddDescChild {
    fn element_name(&self) -> &'static str {
        ""
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
            AddDescChild::Text(text) => writer.write_text(text),
            AddDescChild::Head(elem) => elem.serialize_mei(writer),
            AddDescChild::P(elem) => elem.serialize_mei(writer),
            AddDescChild::Date(elem) => elem.serialize_mei(writer),
            AddDescChild::Bibl(elem) => elem.serialize_mei(writer),
            AddDescChild::BiblStruct(elem) => elem.serialize_mei(writer),
            AddDescChild::Annot(elem) => elem.serialize_mei(writer),
            AddDescChild::Ptr(elem) => elem.serialize_mei(writer),
            AddDescChild::Ref(elem) => elem.serialize_mei(writer),
            AddDescChild::PersName(elem) => elem.serialize_mei(writer),
            AddDescChild::CorpName(elem) => elem.serialize_mei(writer),
            AddDescChild::Name(elem) => elem.serialize_mei(writer),
            AddDescChild::GeogName(elem) => elem.serialize_mei(writer),
            AddDescChild::GeogFeat(elem) => elem.serialize_mei(writer),
            AddDescChild::Address(elem) => elem.serialize_mei(writer),
            AddDescChild::Country(elem) => elem.serialize_mei(writer),
            AddDescChild::Region(elem) => elem.serialize_mei(writer),
            AddDescChild::Settlement(elem) => elem.serialize_mei(writer),
            AddDescChild::District(elem) => elem.serialize_mei(writer),
            AddDescChild::Bloc(elem) => elem.serialize_mei(writer),
            AddDescChild::Dimensions(elem) => elem.serialize_mei(writer),
            AddDescChild::Height(elem) => elem.serialize_mei(writer),
            AddDescChild::Width(elem) => elem.serialize_mei(writer),
            AddDescChild::Depth(elem) => elem.serialize_mei(writer),
            AddDescChild::Dim(elem) => elem.serialize_mei(writer),
            AddDescChild::Term(elem) => elem.serialize_mei(writer),
            AddDescChild::Lb(elem) => elem.serialize_mei(writer),
            AddDescChild::Rend(elem) => elem.serialize_mei(writer),
            AddDescChild::Num(elem) => elem.serialize_mei(writer),
            AddDescChild::Fig(elem) => elem.serialize_mei(writer),
            AddDescChild::Seg(elem) => elem.serialize_mei(writer),
            AddDescChild::Identifier(elem) => elem.serialize_mei(writer),
            AddDescChild::Locus(elem) => elem.serialize_mei(writer),
            AddDescChild::LocusGrp(elem) => elem.serialize_mei(writer),
            AddDescChild::Title(elem) => elem.serialize_mei(writer),
            AddDescChild::Symbol(elem) => elem.serialize_mei(writer),
            AddDescChild::Q(elem) => elem.serialize_mei(writer),
            AddDescChild::Extent(elem) => elem.serialize_mei(writer),
            AddDescChild::RelationList(elem) => elem.serialize_mei(writer),
            AddDescChild::Relation(elem) => elem.serialize_mei(writer),
            AddDescChild::PeriodName(elem) => elem.serialize_mei(writer),
            AddDescChild::StyleName(elem) => elem.serialize_mei(writer),
            AddDescChild::Abbr(elem) => elem.serialize_mei(writer),
            AddDescChild::Expan(elem) => elem.serialize_mei(writer),
            AddDescChild::Stack(elem) => elem.serialize_mei(writer),
            AddDescChild::PostBox(elem) => elem.serialize_mei(writer),
            AddDescChild::PostCode(elem) => elem.serialize_mei(writer),
            AddDescChild::Street(elem) => elem.serialize_mei(writer),
            AddDescChild::Repository(elem) => elem.serialize_mei(writer),
            AddDescChild::Heraldry(_) => Ok(()), // Not yet implemented
            AddDescChild::SecFolio(_) => Ok(()), // Not yet implemented
            AddDescChild::Catchwords(_) => Ok(()), // Not yet implemented
            AddDescChild::Signatures(_) => Ok(()), // Not yet implemented
            AddDescChild::Stamp(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Watermark
// ============================================================================

impl MeiSerialize for Watermark {
    fn element_name(&self) -> &'static str {
        "watermark"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for WatermarkChild {
    fn element_name(&self) -> &'static str {
        ""
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
            WatermarkChild::Text(text) => writer.write_text(text),
            WatermarkChild::Date(elem) => elem.serialize_mei(writer),
            WatermarkChild::Bibl(elem) => elem.serialize_mei(writer),
            WatermarkChild::BiblStruct(elem) => elem.serialize_mei(writer),
            WatermarkChild::BiblList(_) => Ok(()), // Not yet implemented
            WatermarkChild::Annot(elem) => elem.serialize_mei(writer),
            WatermarkChild::Ptr(elem) => elem.serialize_mei(writer),
            WatermarkChild::Ref(elem) => elem.serialize_mei(writer),
            WatermarkChild::GeogName(elem) => elem.serialize_mei(writer),
            WatermarkChild::GeogFeat(elem) => elem.serialize_mei(writer),
            WatermarkChild::Address(elem) => elem.serialize_mei(writer),
            WatermarkChild::Country(elem) => elem.serialize_mei(writer),
            WatermarkChild::Region(elem) => elem.serialize_mei(writer),
            WatermarkChild::Settlement(elem) => elem.serialize_mei(writer),
            WatermarkChild::District(elem) => elem.serialize_mei(writer),
            WatermarkChild::Bloc(elem) => elem.serialize_mei(writer),
            WatermarkChild::Dimensions(elem) => elem.serialize_mei(writer),
            WatermarkChild::Height(elem) => elem.serialize_mei(writer),
            WatermarkChild::Width(elem) => elem.serialize_mei(writer),
            WatermarkChild::Depth(elem) => elem.serialize_mei(writer),
            WatermarkChild::Dim(elem) => elem.serialize_mei(writer),
            WatermarkChild::Term(elem) => elem.serialize_mei(writer),
            WatermarkChild::Rend(elem) => elem.serialize_mei(writer),
            WatermarkChild::Num(elem) => elem.serialize_mei(writer),
            WatermarkChild::Fig(elem) => elem.serialize_mei(writer),
            WatermarkChild::Identifier(elem) => elem.serialize_mei(writer),
            WatermarkChild::Locus(elem) => elem.serialize_mei(writer),
            WatermarkChild::LocusGrp(elem) => elem.serialize_mei(writer),
            WatermarkChild::Title(elem) => elem.serialize_mei(writer),
            WatermarkChild::Q(elem) => elem.serialize_mei(writer),
            WatermarkChild::Extent(elem) => elem.serialize_mei(writer),
            WatermarkChild::RelationList(elem) => elem.serialize_mei(writer),
            WatermarkChild::Relation(elem) => elem.serialize_mei(writer),
            WatermarkChild::Heraldry(_) => Ok(()), // Not yet implemented
            WatermarkChild::Stack(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// WatermarkDesc
// ============================================================================

impl MeiSerialize for WatermarkDesc {
    fn element_name(&self) -> &'static str {
        "watermarkDesc"
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

impl MeiSerialize for WatermarkDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            WatermarkDescChild::Head(_) => "head",
            WatermarkDescChild::P(_) => "p",
            WatermarkDescChild::Watermark(_) => "watermark",
            WatermarkDescChild::WatermarkList(_) => "watermarkList",
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
            WatermarkDescChild::Head(elem) => elem.serialize_mei(writer),
            WatermarkDescChild::P(elem) => elem.serialize_mei(writer),
            WatermarkDescChild::Watermark(elem) => elem.serialize_mei(writer),
            WatermarkDescChild::WatermarkList(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// WatermarkList
// ============================================================================

impl MeiSerialize for WatermarkList {
    fn element_name(&self) -> &'static str {
        "watermarkList"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
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

impl MeiSerialize for WatermarkListChild {
    fn element_name(&self) -> &'static str {
        match self {
            WatermarkListChild::Watermark(_) => "watermark",
            WatermarkListChild::Annot(_) => "annot",
            WatermarkListChild::Head(_) => "head",
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
            WatermarkListChild::Watermark(elem) => elem.serialize_mei(writer),
            WatermarkListChild::Annot(elem) => elem.serialize_mei(writer),
            WatermarkListChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// TypeDesc
// ============================================================================

impl MeiSerialize for TypeDesc {
    fn element_name(&self) -> &'static str {
        "typeDesc"
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

impl MeiSerialize for TypeDescChild {
    fn element_name(&self) -> &'static str {
        ""
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
            TypeDescChild::Text(text) => writer.write_text(text),
            TypeDescChild::Head(elem) => elem.serialize_mei(writer),
            TypeDescChild::P(elem) => elem.serialize_mei(writer),
            TypeDescChild::TypeNote(elem) => elem.serialize_mei(writer),
            TypeDescChild::Date(elem) => elem.serialize_mei(writer),
            TypeDescChild::Bibl(elem) => elem.serialize_mei(writer),
            TypeDescChild::BiblStruct(elem) => elem.serialize_mei(writer),
            TypeDescChild::Annot(elem) => elem.serialize_mei(writer),
            TypeDescChild::Ptr(elem) => elem.serialize_mei(writer),
            TypeDescChild::Ref(elem) => elem.serialize_mei(writer),
            TypeDescChild::PersName(elem) => elem.serialize_mei(writer),
            TypeDescChild::CorpName(elem) => elem.serialize_mei(writer),
            TypeDescChild::Name(elem) => elem.serialize_mei(writer),
            TypeDescChild::GeogName(elem) => elem.serialize_mei(writer),
            TypeDescChild::GeogFeat(elem) => elem.serialize_mei(writer),
            TypeDescChild::Address(elem) => elem.serialize_mei(writer),
            TypeDescChild::Country(elem) => elem.serialize_mei(writer),
            TypeDescChild::Region(elem) => elem.serialize_mei(writer),
            TypeDescChild::Settlement(elem) => elem.serialize_mei(writer),
            TypeDescChild::District(elem) => elem.serialize_mei(writer),
            TypeDescChild::Bloc(elem) => elem.serialize_mei(writer),
            TypeDescChild::Dimensions(elem) => elem.serialize_mei(writer),
            TypeDescChild::Height(elem) => elem.serialize_mei(writer),
            TypeDescChild::Width(elem) => elem.serialize_mei(writer),
            TypeDescChild::Depth(elem) => elem.serialize_mei(writer),
            TypeDescChild::Dim(elem) => elem.serialize_mei(writer),
            TypeDescChild::Term(elem) => elem.serialize_mei(writer),
            TypeDescChild::Lb(elem) => elem.serialize_mei(writer),
            TypeDescChild::Rend(elem) => elem.serialize_mei(writer),
            TypeDescChild::Num(elem) => elem.serialize_mei(writer),
            TypeDescChild::Fig(elem) => elem.serialize_mei(writer),
            TypeDescChild::Seg(elem) => elem.serialize_mei(writer),
            TypeDescChild::Identifier(elem) => elem.serialize_mei(writer),
            TypeDescChild::Locus(elem) => elem.serialize_mei(writer),
            TypeDescChild::LocusGrp(elem) => elem.serialize_mei(writer),
            TypeDescChild::Title(elem) => elem.serialize_mei(writer),
            TypeDescChild::Symbol(elem) => elem.serialize_mei(writer),
            TypeDescChild::Q(elem) => elem.serialize_mei(writer),
            TypeDescChild::Extent(elem) => elem.serialize_mei(writer),
            TypeDescChild::RelationList(elem) => elem.serialize_mei(writer),
            TypeDescChild::Relation(elem) => elem.serialize_mei(writer),
            TypeDescChild::PeriodName(elem) => elem.serialize_mei(writer),
            TypeDescChild::StyleName(elem) => elem.serialize_mei(writer),
            TypeDescChild::Abbr(elem) => elem.serialize_mei(writer),
            TypeDescChild::Expan(elem) => elem.serialize_mei(writer),
            TypeDescChild::Stack(elem) => elem.serialize_mei(writer),
            TypeDescChild::PostBox(elem) => elem.serialize_mei(writer),
            TypeDescChild::PostCode(elem) => elem.serialize_mei(writer),
            TypeDescChild::Street(elem) => elem.serialize_mei(writer),
            TypeDescChild::Repository(elem) => elem.serialize_mei(writer),
            TypeDescChild::Heraldry(_) => Ok(()), // Not yet implemented
            TypeDescChild::SecFolio(_) => Ok(()), // Not yet implemented
            TypeDescChild::Catchwords(_) => Ok(()), // Not yet implemented
            TypeDescChild::Signatures(_) => Ok(()), // Not yet implemented
            TypeDescChild::Stamp(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// TypeNote
// ============================================================================

impl MeiSerialize for TypeNote {
    fn element_name(&self) -> &'static str {
        "typeNote"
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

impl MeiSerialize for TypeNoteChild {
    fn element_name(&self) -> &'static str {
        ""
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
            TypeNoteChild::Text(text) => writer.write_text(text),
            TypeNoteChild::Head(elem) => elem.serialize_mei(writer),
            TypeNoteChild::P(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Date(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Bibl(elem) => elem.serialize_mei(writer),
            TypeNoteChild::BiblStruct(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Annot(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Ptr(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Ref(elem) => elem.serialize_mei(writer),
            TypeNoteChild::PersName(elem) => elem.serialize_mei(writer),
            TypeNoteChild::CorpName(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Name(elem) => elem.serialize_mei(writer),
            TypeNoteChild::GeogName(elem) => elem.serialize_mei(writer),
            TypeNoteChild::GeogFeat(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Address(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Country(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Region(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Settlement(elem) => elem.serialize_mei(writer),
            TypeNoteChild::District(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Bloc(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Dimensions(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Height(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Width(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Depth(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Dim(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Term(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Lb(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Rend(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Num(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Fig(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Seg(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Identifier(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Locus(elem) => elem.serialize_mei(writer),
            TypeNoteChild::LocusGrp(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Title(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Symbol(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Q(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Extent(elem) => elem.serialize_mei(writer),
            TypeNoteChild::RelationList(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Relation(elem) => elem.serialize_mei(writer),
            TypeNoteChild::PeriodName(elem) => elem.serialize_mei(writer),
            TypeNoteChild::StyleName(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Abbr(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Expan(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Stack(elem) => elem.serialize_mei(writer),
            TypeNoteChild::PostBox(elem) => elem.serialize_mei(writer),
            TypeNoteChild::PostCode(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Street(elem) => elem.serialize_mei(writer),
            TypeNoteChild::Repository(_) => Ok(()), // Not yet implemented
            TypeNoteChild::Heraldry(_) => Ok(()),   // Not yet implemented
            TypeNoteChild::SecFolio(_) => Ok(()),   // Not yet implemented
            TypeNoteChild::Catchwords(_) => Ok(()), // Not yet implemented
            TypeNoteChild::Signatures(_) => Ok(()), // Not yet implemented
            TypeNoteChild::Stamp(elem) => elem.serialize_mei(writer),
        }
    }
}

//! Serializer implementations for agent/contributor and name elements.
//!
//! Contains: Creator, Editor, Funder, RespStmt, Resp, Contributor, Sponsor,
//! PersName, CorpName, Name, Rend, Lb, Seg.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Contributor, ContributorChild, CorpName, CorpNameChild, Creator, CreatorChild, Editor,
    EditorChild, Funder, FunderChild, Lb, Name, NameChild, PersName, PersNameChild, Rend,
    RendChild, Resp, RespChild, RespStmt, RespStmtChild, Seg, SegChild, Sponsor, SponsorChild,
};

// ============================================================================
// Creator
// ============================================================================

impl MeiSerialize for Creator {
    fn element_name(&self) -> &'static str {
        "creator"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for CreatorChild {
    fn element_name(&self) -> &'static str {
        match self {
            CreatorChild::Text(_) => "#text",
            CreatorChild::CorpName(_) => "corpName",
            CreatorChild::Name(_) => "name",
            CreatorChild::PersName(_) => "persName",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, CreatorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CreatorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CreatorChild::CorpName(elem) => elem.serialize_mei(writer),
            CreatorChild::Name(elem) => elem.serialize_mei(writer),
            CreatorChild::PersName(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Editor
// ============================================================================

impl MeiSerialize for Editor {
    fn element_name(&self) -> &'static str {
        "editor"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
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

impl MeiSerialize for EditorChild {
    fn element_name(&self) -> &'static str {
        match self {
            EditorChild::Text(_) => "#text",
            EditorChild::Name(_) => "name",
            EditorChild::CorpName(_) => "corpName",
            EditorChild::PersName(_) => "persName",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, EditorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            EditorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            EditorChild::Name(elem) => elem.serialize_mei(writer),
            EditorChild::CorpName(elem) => elem.serialize_mei(writer),
            EditorChild::PersName(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Funder
// ============================================================================

impl MeiSerialize for Funder {
    fn element_name(&self) -> &'static str {
        "funder"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
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

impl MeiSerialize for FunderChild {
    fn element_name(&self) -> &'static str {
        match self {
            FunderChild::Text(_) => "#text",
            FunderChild::CorpName(_) => "corpName",
            FunderChild::Name(_) => "name",
            FunderChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, FunderChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FunderChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            FunderChild::CorpName(elem) => elem.serialize_mei(writer),
            FunderChild::Name(elem) => elem.serialize_mei(writer),
            FunderChild::PersName(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FunderChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// RespStmt
// ============================================================================

impl MeiSerialize for RespStmt {
    fn element_name(&self) -> &'static str {
        "respStmt"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
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

impl MeiSerialize for RespStmtChild {
    fn element_name(&self) -> &'static str {
        match self {
            RespStmtChild::Resp(_) => "resp",
            RespStmtChild::CorpName(_) => "corpName",
            RespStmtChild::Annot(_) => "annot",
            RespStmtChild::Name(_) => "name",
            RespStmtChild::PersName(_) => "persName",
            RespStmtChild::Head(_) => "head",
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
            RespStmtChild::Resp(elem) => elem.serialize_mei(writer),
            RespStmtChild::CorpName(elem) => elem.serialize_mei(writer),
            RespStmtChild::Name(elem) => elem.serialize_mei(writer),
            RespStmtChild::PersName(elem) => elem.serialize_mei(writer),
            RespStmtChild::Head(elem) => elem.serialize_mei(writer),
            RespStmtChild::Annot(_) => Ok(()), // Annot needs separate implementation
        }
    }
}

// ============================================================================
// Resp
// ============================================================================

impl MeiSerialize for Resp {
    fn element_name(&self) -> &'static str {
        "resp"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
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

impl MeiSerialize for RespChild {
    fn element_name(&self) -> &'static str {
        match self {
            RespChild::Text(_) => "#text",
            RespChild::PersName(_) => "persName",
            RespChild::CorpName(_) => "corpName",
            RespChild::Name(_) => "name",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, RespChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RespChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            RespChild::PersName(elem) => elem.serialize_mei(writer),
            RespChild::CorpName(elem) => elem.serialize_mei(writer),
            RespChild::Name(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "RespChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Contributor
// ============================================================================

impl MeiSerialize for Contributor {
    fn element_name(&self) -> &'static str {
        "contributor"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for ContributorChild {
    fn element_name(&self) -> &'static str {
        match self {
            ContributorChild::Text(_) => "#text",
            ContributorChild::CorpName(_) => "corpName",
            ContributorChild::Name(_) => "name",
            ContributorChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, ContributorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ContributorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            ContributorChild::CorpName(elem) => elem.serialize_mei(writer),
            ContributorChild::Name(elem) => elem.serialize_mei(writer),
            ContributorChild::PersName(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ContributorChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Sponsor
// ============================================================================

impl MeiSerialize for Sponsor {
    fn element_name(&self) -> &'static str {
        "sponsor"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
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

impl MeiSerialize for SponsorChild {
    fn element_name(&self) -> &'static str {
        match self {
            SponsorChild::Text(_) => "#text",
            SponsorChild::CorpName(_) => "corpName",
            SponsorChild::Name(_) => "name",
            SponsorChild::PersName(_) => "persName",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, SponsorChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SponsorChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            SponsorChild::CorpName(elem) => elem.serialize_mei(writer),
            SponsorChild::Name(elem) => elem.serialize_mei(writer),
            SponsorChild::PersName(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "SponsorChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// PersName
// ============================================================================

impl MeiSerialize for PersName {
    fn element_name(&self) -> &'static str {
        "persName"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for PersNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            PersNameChild::Text(_) => "#text",
            PersNameChild::CorpName(_) => "corpName",
            PersNameChild::Name(_) => "name",
            PersNameChild::PersName(_) => "persName",
            PersNameChild::Rend(_) => "rend",
            PersNameChild::Lb(_) => "lb",
            PersNameChild::Date(_) => "date",
            PersNameChild::Identifier(_) => "identifier",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PersNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PersNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PersNameChild::CorpName(elem) => elem.serialize_mei(writer),
            PersNameChild::Name(elem) => elem.serialize_mei(writer),
            PersNameChild::PersName(elem) => elem.serialize_mei(writer),
            PersNameChild::Rend(elem) => elem.serialize_mei(writer),
            PersNameChild::Lb(elem) => elem.serialize_mei(writer),
            PersNameChild::Date(elem) => elem.serialize_mei(writer),
            PersNameChild::Identifier(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PersNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// CorpName
// ============================================================================

impl MeiSerialize for CorpName {
    fn element_name(&self) -> &'static str {
        "corpName"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for CorpNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            CorpNameChild::Text(_) => "#text",
            CorpNameChild::Bibl(_) => "bibl",
            CorpNameChild::Fig(_) => "fig",
            CorpNameChild::Num(_) => "num",
            CorpNameChild::Catchwords(_) => "catchwords",
            CorpNameChild::Address(_) => "address",
            CorpNameChild::Height(_) => "height",
            CorpNameChild::Relation(_) => "relation",
            CorpNameChild::Reg(_) => "reg",
            CorpNameChild::Supplied(_) => "supplied",
            CorpNameChild::Restore(_) => "restore",
            CorpNameChild::Country(_) => "country",
            CorpNameChild::Locus(_) => "locus",
            CorpNameChild::Bloc(_) => "bloc",
            CorpNameChild::Signatures(_) => "signatures",
            CorpNameChild::Repository(_) => "repository",
            CorpNameChild::RelationList(_) => "relationList",
            CorpNameChild::Symbol(_) => "symbol",
            CorpNameChild::Seg(_) => "seg",
            CorpNameChild::Add(_) => "add",
            CorpNameChild::Choice(_) => "choice",
            CorpNameChild::Stack(_) => "stack",
            CorpNameChild::StyleName(_) => "styleName",
            CorpNameChild::CorpName(_) => "corpName",
            CorpNameChild::Title(_) => "title",
            CorpNameChild::Heraldry(_) => "heraldry",
            CorpNameChild::Ptr(_) => "ptr",
            CorpNameChild::PersName(_) => "persName",
            CorpNameChild::BiblStruct(_) => "biblStruct",
            CorpNameChild::District(_) => "district",
            CorpNameChild::Identifier(_) => "identifier",
            CorpNameChild::Region(_) => "region",
            CorpNameChild::PostCode(_) => "postCode",
            CorpNameChild::Dim(_) => "dim",
            CorpNameChild::Del(_) => "del",
            CorpNameChild::Annot(_) => "annot",
            CorpNameChild::Name(_) => "name",
            CorpNameChild::Gap(_) => "gap",
            CorpNameChild::Settlement(_) => "settlement",
            CorpNameChild::Expan(_) => "expan",
            CorpNameChild::GeogName(_) => "geogName",
            CorpNameChild::Lb(_) => "lb",
            CorpNameChild::Stamp(_) => "stamp",
            CorpNameChild::Depth(_) => "depth",
            CorpNameChild::Extent(_) => "extent",
            CorpNameChild::Corr(_) => "corr",
            CorpNameChild::Dimensions(_) => "dimensions",
            CorpNameChild::Rend(_) => "rend",
            CorpNameChild::Ref(_) => "ref",
            CorpNameChild::Width(_) => "width",
            CorpNameChild::PostBox(_) => "postBox",
            CorpNameChild::Pb(_) => "pb",
            CorpNameChild::Abbr(_) => "abbr",
            CorpNameChild::LocusGrp(_) => "locusGrp",
            CorpNameChild::Term(_) => "term",
            CorpNameChild::Damage(_) => "damage",
            CorpNameChild::Unclear(_) => "unclear",
            CorpNameChild::PeriodName(_) => "periodName",
            CorpNameChild::Orig(_) => "orig",
            CorpNameChild::Street(_) => "street",
            CorpNameChild::Subst(_) => "subst",
            CorpNameChild::SecFolio(_) => "secFolio",
            CorpNameChild::GeogFeat(_) => "geogFeat",
            CorpNameChild::Sic(_) => "sic",
            CorpNameChild::Date(_) => "date",
            CorpNameChild::Q(_) => "q",
            CorpNameChild::HandShift(_) => "handShift",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, CorpNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            CorpNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            CorpNameChild::Address(elem) => elem.serialize_mei(writer),
            CorpNameChild::Annot(elem) => elem.serialize_mei(writer),
            CorpNameChild::Bibl(elem) => elem.serialize_mei(writer),
            CorpNameChild::BiblStruct(elem) => elem.serialize_mei(writer),
            CorpNameChild::Bloc(elem) => elem.serialize_mei(writer),
            CorpNameChild::CorpName(elem) => elem.serialize_mei(writer),
            CorpNameChild::Country(elem) => elem.serialize_mei(writer),
            CorpNameChild::Date(elem) => elem.serialize_mei(writer),
            CorpNameChild::District(elem) => elem.serialize_mei(writer),
            CorpNameChild::Extent(elem) => elem.serialize_mei(writer),
            CorpNameChild::GeogFeat(elem) => elem.serialize_mei(writer),
            CorpNameChild::GeogName(elem) => elem.serialize_mei(writer),
            CorpNameChild::Identifier(elem) => elem.serialize_mei(writer),
            CorpNameChild::Lb(elem) => elem.serialize_mei(writer),
            CorpNameChild::Locus(elem) => elem.serialize_mei(writer),
            CorpNameChild::LocusGrp(elem) => elem.serialize_mei(writer),
            CorpNameChild::Name(elem) => elem.serialize_mei(writer),
            CorpNameChild::Num(elem) => elem.serialize_mei(writer),
            CorpNameChild::PersName(elem) => elem.serialize_mei(writer),
            CorpNameChild::PostBox(elem) => elem.serialize_mei(writer),
            CorpNameChild::PostCode(elem) => elem.serialize_mei(writer),
            CorpNameChild::Ptr(elem) => elem.serialize_mei(writer),
            CorpNameChild::Ref(elem) => elem.serialize_mei(writer),
            CorpNameChild::Region(elem) => elem.serialize_mei(writer),
            CorpNameChild::Rend(elem) => elem.serialize_mei(writer),
            CorpNameChild::Settlement(elem) => elem.serialize_mei(writer),
            CorpNameChild::Street(elem) => elem.serialize_mei(writer),
            CorpNameChild::Term(elem) => elem.serialize_mei(writer),
            CorpNameChild::Title(elem) => elem.serialize_mei(writer),
            // Editorial elements that need their own serializers
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "CorpNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Name
// ============================================================================

impl MeiSerialize for Name {
    fn element_name(&self) -> &'static str {
        "name"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.classed.collect_attributes());
        attrs.extend(self.edit.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.name.collect_attributes());
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

impl MeiSerialize for NameChild {
    fn element_name(&self) -> &'static str {
        match self {
            NameChild::Text(_) => "#text",
            NameChild::CorpName(_) => "corpName",
            NameChild::Name(_) => "name",
            NameChild::PersName(_) => "persName",
            NameChild::Rend(_) => "rend",
            NameChild::Lb(_) => "lb",
            NameChild::Title(_) => "title",
            NameChild::Date(_) => "date",
            NameChild::Ref(_) => "ref",
            NameChild::Ptr(_) => "ptr",
            NameChild::GeogName(_) => "geogName",
            NameChild::Identifier(_) => "identifier",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, NameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            NameChild::CorpName(elem) => elem.serialize_mei(writer),
            NameChild::Name(elem) => elem.serialize_mei(writer),
            NameChild::PersName(elem) => elem.serialize_mei(writer),
            NameChild::Rend(elem) => elem.serialize_mei(writer),
            NameChild::Lb(elem) => elem.serialize_mei(writer),
            NameChild::Title(elem) => elem.serialize_mei(writer),
            NameChild::Date(elem) => elem.serialize_mei(writer),
            NameChild::Ref(elem) => elem.serialize_mei(writer),
            NameChild::Ptr(elem) => elem.serialize_mei(writer),
            NameChild::GeogName(elem) => elem.serialize_mei(writer),
            NameChild::Identifier(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "NameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Rend
// ============================================================================

impl MeiSerialize for Rend {
    fn element_name(&self) -> &'static str {
        "rend"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.color.collect_attributes());
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.ext_sym_auth.collect_attributes());
        attrs.extend(self.horizontal_align.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.text_rendition.collect_attributes());
        attrs.extend(self.typography.collect_attributes());
        attrs.extend(self.vertical_align.collect_attributes());
        attrs.extend(self.whitespace.collect_attributes());
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

impl MeiSerialize for RendChild {
    fn element_name(&self) -> &'static str {
        match self {
            RendChild::Text(_) => "#text",
            RendChild::Rend(_) => "rend",
            RendChild::Date(_) => "date",
            RendChild::Name(_) => "name",
            RendChild::PersName(_) => "persName",
            RendChild::CorpName(_) => "corpName",
            RendChild::Title(_) => "title",
            RendChild::Lb(_) => "lb",
            RendChild::Ref(_) => "ref",
            RendChild::Ptr(_) => "ptr",
            RendChild::Identifier(_) => "identifier",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, RendChild::Text(_) | RendChild::Lb(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            RendChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            RendChild::Rend(elem) => elem.serialize_mei(writer),
            RendChild::Date(elem) => elem.serialize_mei(writer),
            RendChild::Name(elem) => elem.serialize_mei(writer),
            RendChild::PersName(elem) => elem.serialize_mei(writer),
            RendChild::CorpName(elem) => elem.serialize_mei(writer),
            RendChild::Title(elem) => elem.serialize_mei(writer),
            RendChild::Lb(elem) => elem.serialize_mei(writer),
            RendChild::Ref(elem) => elem.serialize_mei(writer),
            RendChild::Ptr(elem) => elem.serialize_mei(writer),
            RendChild::Identifier(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "RendChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// Lb (line break)
// ============================================================================

impl MeiSerialize for Lb {
    fn element_name(&self) -> &'static str {
        "lb"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.source.collect_attributes());
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
// Seg (arbitrary segment)
// ============================================================================

impl MeiSerialize for Seg {
    fn element_name(&self) -> &'static str {
        "seg"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
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

impl MeiSerialize for SegChild {
    fn element_name(&self) -> &'static str {
        match self {
            SegChild::Text(_) => "#text",
            SegChild::Rend(_) => "rend",
            SegChild::Lb(_) => "lb",
            SegChild::PersName(_) => "persName",
            SegChild::CorpName(_) => "corpName",
            SegChild::Name(_) => "name",
            SegChild::Title(_) => "title",
            SegChild::Date(_) => "date",
            SegChild::Identifier(_) => "identifier",
            SegChild::Ref(_) => "ref",
            SegChild::Ptr(_) => "ptr",
            SegChild::Num(_) => "num",
            SegChild::Seg(_) => "seg",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, SegChild::Text(_) | SegChild::Lb(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SegChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            SegChild::Rend(elem) => elem.serialize_mei(writer),
            SegChild::Lb(elem) => elem.serialize_mei(writer),
            SegChild::PersName(elem) => elem.serialize_mei(writer),
            SegChild::CorpName(elem) => elem.serialize_mei(writer),
            SegChild::Name(elem) => elem.serialize_mei(writer),
            SegChild::Title(elem) => elem.serialize_mei(writer),
            SegChild::Date(elem) => elem.serialize_mei(writer),
            SegChild::Identifier(elem) => elem.serialize_mei(writer),
            SegChild::Ref(elem) => elem.serialize_mei(writer),
            SegChild::Ptr(elem) => elem.serialize_mei(writer),
            SegChild::Num(elem) => elem.serialize_mei(writer),
            SegChild::Seg(elem) => elem.serialize_mei(writer),
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "SegChild::{}",
                other.element_name()
            ))),
        }
    }
}

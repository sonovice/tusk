//! Serializer implementations for agent/contributor and name elements.
//!
//! Contains: Creator, Editor, Funder, RespStmt, Resp, Contributor, Sponsor,
//! PersName, CorpName, Name, Rend, Lb.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Contributor, ContributorChild, CorpName, CorpNameChild, Creator, CreatorChild, Editor,
    EditorChild, Funder, FunderChild, Lb, Name, NameChild, PersName, PersNameChild, Rend,
    RendChild, Resp, RespChild, RespStmt, RespStmtChild, Sponsor, SponsorChild,
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
            CorpNameChild::CorpName(_) => "corpName",
            CorpNameChild::Name(_) => "name",
            CorpNameChild::PersName(_) => "persName",
            _ => "unknown",
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
            CorpNameChild::CorpName(elem) => elem.serialize_mei(writer),
            CorpNameChild::Name(elem) => elem.serialize_mei(writer),
            CorpNameChild::PersName(elem) => elem.serialize_mei(writer),
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
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, RendChild::Text(_))
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

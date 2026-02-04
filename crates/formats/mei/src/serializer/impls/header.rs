//! Serializer implementations for MEI header elements.
//!
//! This module contains implementations for MeiHead, FileDesc, TitleStmt, PubStmt,
//! and their child elements.

use super::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Change, ChangeChild, ChangeDesc, ChangeDescChild, Date, DateChild, EncodingDesc,
    EncodingDescChild, FileDesc, FileDescChild, Head, HeadChild, MeiHead, MeiHeadChild, P, PChild,
    PubStmt, PubStmtChild, RevisionDesc, RevisionDescChild, SourceDesc, SourceDescChild, Title,
    TitleChild, TitleStmt, TitleStmtChild,
};

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

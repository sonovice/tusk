//! Serializer implementations for MEI header elements.
//!
//! This module contains implementations for MeiHead, FileDesc, TitleStmt, PubStmt,
//! and their child elements.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Change, ChangeChild, ChangeDesc, ChangeDescChild, Contributor, ContributorChild, CorpName,
    CorpNameChild, Creator, CreatorChild, Date, DateChild, Editor, EditorChild, EncodingDesc,
    EncodingDescChild, FileDesc, FileDescChild, Funder, FunderChild, Head, HeadChild, Lb, MeiHead,
    MeiHeadChild, Name, NameChild, P, PChild, PersName, PersNameChild, PubStmt, PubStmtChild, Rend,
    RendChild, Resp, RespChild, RespStmt, RespStmtChild, RevisionDesc, RevisionDescChild,
    SourceDesc, SourceDescChild, Sponsor, SponsorChild, Title, TitleChild, TitlePart,
    TitlePartChild, TitleStmt, TitleStmtChild,
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
            TitleStmtChild::Creator(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Editor(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Funder(elem) => elem.serialize_mei(writer),
            TitleStmtChild::RespStmt(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Contributor(elem) => elem.serialize_mei(writer),
            TitleStmtChild::Sponsor(elem) => elem.serialize_mei(writer),
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
            TitleChild::TitlePart(elem) => elem.serialize_mei(writer),
            TitleChild::Rend(elem) => elem.serialize_mei(writer),
            TitleChild::CorpName(elem) => elem.serialize_mei(writer),
            TitleChild::Name(elem) => elem.serialize_mei(writer),
            TitleChild::PersName(elem) => elem.serialize_mei(writer),
            TitleChild::Date(elem) => elem.serialize_mei(writer),
            _ => Ok(()), // Other children skipped for now
        }
    }
}

impl MeiSerialize for TitlePart {
    fn element_name(&self) -> &'static str {
        "titlePart"
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
        attrs.extend(self.n_integer.collect_attributes());
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

impl MeiSerialize for TitlePartChild {
    fn element_name(&self) -> &'static str {
        match self {
            TitlePartChild::Text(_) => "#text",
            TitlePartChild::Date(_) => "date",
            TitlePartChild::Name(_) => "name",
            TitlePartChild::PersName(_) => "persName",
            TitlePartChild::CorpName(_) => "corpName",
            TitlePartChild::Rend(_) => "rend",
            TitlePartChild::Title(_) => "title",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, TitlePartChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            TitlePartChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            TitlePartChild::Date(elem) => elem.serialize_mei(writer),
            TitlePartChild::Name(elem) => elem.serialize_mei(writer),
            TitlePartChild::PersName(elem) => elem.serialize_mei(writer),
            TitlePartChild::CorpName(elem) => elem.serialize_mei(writer),
            TitlePartChild::Rend(elem) => elem.serialize_mei(writer),
            TitlePartChild::Title(elem) => elem.serialize_mei(writer),
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

// ============================================================================
// TitleStmt child element implementations (Creator, Editor, Funder, RespStmt, Contributor, Sponsor)
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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

// ============================================================================
// Name-related element implementations (PersName, CorpName, Name)
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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

// ============================================================================
// Rend element implementation (used by Title, TitlePart, and others)
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
            _ => Ok(()), // Other children skipped for now
        }
    }
}

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

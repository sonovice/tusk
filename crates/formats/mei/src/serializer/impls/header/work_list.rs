//! Serializer implementations for work list elements.
//!
//! Contains: WorkList, Work, WorkChild.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{Work, WorkChild, WorkList, WorkListChild};

// ============================================================================
// WorkList
// ============================================================================

impl MeiSerialize for WorkList {
    fn element_name(&self) -> &'static str {
        "workList"
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

impl MeiSerialize for WorkListChild {
    fn element_name(&self) -> &'static str {
        match self {
            WorkListChild::Head(_) => "head",
            WorkListChild::Work(_) => "work",
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
            WorkListChild::Head(elem) => elem.serialize_mei(writer),
            WorkListChild::Work(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Work
// ============================================================================

impl MeiSerialize for Work {
    fn element_name(&self) -> &'static str {
        "work"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
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

impl MeiSerialize for WorkChild {
    fn element_name(&self) -> &'static str {
        match self {
            WorkChild::Incip(_) => "incip",
            WorkChild::Meter(_) => "meter",
            WorkChild::Creation(_) => "creation",
            WorkChild::History(_) => "history",
            WorkChild::Mensuration(_) => "mensuration",
            WorkChild::PerfDuration(_) => "perfDuration",
            WorkChild::Context(_) => "context",
            WorkChild::NotesStmt(_) => "notesStmt",
            WorkChild::ExtMeta(_) => "extMeta",
            WorkChild::Dedication(_) => "dedication",
            WorkChild::BiblList(_) => "biblList",
            WorkChild::Title(_) => "title",
            WorkChild::Classification(_) => "classification",
            WorkChild::Head(_) => "head",
            WorkChild::Tempo(_) => "tempo",
            WorkChild::OtherChar(_) => "otherChar",
            WorkChild::RespStmt(_) => "respStmt",
            WorkChild::PerfMedium(_) => "perfMedium",
            WorkChild::Audience(_) => "audience",
            WorkChild::Key(_) => "key",
            WorkChild::Contents(_) => "contents",
            WorkChild::ExpressionList(_) => "expressionList",
            WorkChild::RelationList(_) => "relationList",
            WorkChild::ComponentList(_) => "componentList",
            WorkChild::LangUsage(_) => "langUsage",
            WorkChild::Identifier(_) => "identifier",
            WorkChild::Creator(_) => "creator",
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
            WorkChild::Title(elem) => elem.serialize_mei(writer),
            WorkChild::Head(elem) => elem.serialize_mei(writer),
            WorkChild::RespStmt(elem) => elem.serialize_mei(writer),
            WorkChild::NotesStmt(elem) => elem.serialize_mei(writer),
            WorkChild::ExtMeta(elem) => elem.serialize_mei(writer),
            WorkChild::Identifier(elem) => elem.serialize_mei(writer),
            WorkChild::Contents(elem) => elem.serialize_mei(writer),
            WorkChild::Key(elem) => elem.serialize_mei(writer),
            WorkChild::Meter(elem) => elem.serialize_mei(writer),
            WorkChild::Incip(elem) => elem.serialize_mei(writer),
            WorkChild::Creation(elem) => elem.serialize_mei(writer),
            WorkChild::PerfMedium(elem) => elem.serialize_mei(writer),
            WorkChild::Classification(elem) => elem.serialize_mei(writer),
            WorkChild::Creator(elem) => elem.serialize_mei(writer),
            // The following children need dedicated serializers - for now write empty element
            _ => {
                let name = self.element_name();
                let start = writer.start_element(name)?;
                writer.write_empty(start)?;
                Ok(())
            }
        }
    }
}

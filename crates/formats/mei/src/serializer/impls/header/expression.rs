//! Serializer implementations for expression list elements.
//!
//! Contains: ExpressionList, Expression, ExpressionChild, ExpressionListChild.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{Expression, ExpressionChild, ExpressionList, ExpressionListChild};

// ============================================================================
// ExpressionList
// ============================================================================

impl MeiSerialize for ExpressionList {
    fn element_name(&self) -> &'static str {
        "expressionList"
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

impl MeiSerialize for ExpressionListChild {
    fn element_name(&self) -> &'static str {
        match self {
            ExpressionListChild::Head(_) => "head",
            ExpressionListChild::Expression(_) => "expression",
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
            ExpressionListChild::Head(elem) => elem.serialize_mei(writer),
            ExpressionListChild::Expression(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Expression
// ============================================================================

impl MeiSerialize for Expression {
    fn element_name(&self) -> &'static str {
        "expression"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for ExpressionChild {
    fn element_name(&self) -> &'static str {
        match self {
            ExpressionChild::RelationList(_) => "relationList",
            ExpressionChild::Extent(_) => "extent",
            ExpressionChild::Head(_) => "head",
            ExpressionChild::Context(_) => "context",
            ExpressionChild::Key(_) => "key",
            ExpressionChild::OtherChar(_) => "otherChar",
            ExpressionChild::Incip(_) => "incip",
            ExpressionChild::ScoreFormat(_) => "scoreFormat",
            ExpressionChild::Title(_) => "title",
            ExpressionChild::RespStmt(_) => "respStmt",
            ExpressionChild::Contents(_) => "contents",
            ExpressionChild::Classification(_) => "classification",
            ExpressionChild::History(_) => "history",
            ExpressionChild::ComponentList(_) => "componentList",
            ExpressionChild::Identifier(_) => "identifier",
            ExpressionChild::Creation(_) => "creation",
            ExpressionChild::PerfDuration(_) => "perfDuration",
            ExpressionChild::Mensuration(_) => "mensuration",
            ExpressionChild::Meter(_) => "meter",
            ExpressionChild::PerfMedium(_) => "perfMedium",
            ExpressionChild::LangUsage(_) => "langUsage",
            ExpressionChild::BiblList(_) => "biblList",
            ExpressionChild::Tempo(_) => "tempo",
            ExpressionChild::Dedication(_) => "dedication",
            ExpressionChild::NotesStmt(_) => "notesStmt",
            ExpressionChild::ExtMeta(_) => "extMeta",
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
            ExpressionChild::Extent(elem) => elem.serialize_mei(writer),
            ExpressionChild::Head(elem) => elem.serialize_mei(writer),
            ExpressionChild::Key(elem) => elem.serialize_mei(writer),
            ExpressionChild::Incip(elem) => elem.serialize_mei(writer),
            ExpressionChild::Title(elem) => elem.serialize_mei(writer),
            ExpressionChild::RespStmt(elem) => elem.serialize_mei(writer),
            ExpressionChild::Contents(elem) => elem.serialize_mei(writer),
            ExpressionChild::Classification(elem) => elem.serialize_mei(writer),
            ExpressionChild::History(elem) => elem.serialize_mei(writer),
            ExpressionChild::Identifier(elem) => elem.serialize_mei(writer),
            ExpressionChild::Creation(elem) => elem.serialize_mei(writer),
            ExpressionChild::Meter(elem) => elem.serialize_mei(writer),
            ExpressionChild::PerfMedium(elem) => elem.serialize_mei(writer),
            ExpressionChild::LangUsage(elem) => elem.serialize_mei(writer),
            ExpressionChild::Tempo(elem) => elem.serialize_mei(writer),
            ExpressionChild::NotesStmt(elem) => elem.serialize_mei(writer),
            ExpressionChild::ExtMeta(elem) => elem.serialize_mei(writer),
            // Elements not yet fully implemented - return error
            _other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ExpressionChild::{}",
                self.element_name()
            ))),
        }
    }
}

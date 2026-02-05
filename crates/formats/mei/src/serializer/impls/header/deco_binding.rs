//! Serializer implementations for decoration and binding elements.
//!
//! Contains: DecoDesc, DecoNote, BindingDesc, Binding, SealDesc, Seal.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{AttContemporary, AttDatable};
use tusk_model::elements::{
    Binding, BindingChild, BindingDesc, BindingDescChild, DecoDesc, DecoDescChild, DecoNote,
    DecoNoteChild, Seal, SealChild, SealDesc, SealDescChild,
};

use super::super::push_attr;

// ============================================================================
// Attribute class implementations
// ============================================================================

impl CollectAttributes for AttContemporary {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "contemporary", self.contemporary);
        attrs
    }
}

// Note: AttDatable is already implemented in misc.rs

// ============================================================================
// DecoDesc
// ============================================================================

impl MeiSerialize for DecoDesc {
    fn element_name(&self) -> &'static str {
        "decoDesc"
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

impl MeiSerialize for DecoDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            DecoDescChild::Text(_) => "",
            DecoDescChild::DecoNote(_) => "decoNote",
            DecoDescChild::Head(_) => "head",
            DecoDescChild::P(_) => "p",
            DecoDescChild::Condition(_) => "condition",
            DecoDescChild::Dimensions(_) => "dimensions",
            DecoDescChild::Height(_) => "height",
            DecoDescChild::Width(_) => "width",
            DecoDescChild::Depth(_) => "depth",
            DecoDescChild::Dim(_) => "dim",
            DecoDescChild::Lb(_) => "lb",
            DecoDescChild::Rend(_) => "rend",
            DecoDescChild::Num(_) => "num",
            DecoDescChild::Fig(_) => "fig",
            DecoDescChild::Annot(_) => "annot",
            DecoDescChild::Bibl(_) => "bibl",
            DecoDescChild::BiblStruct(_) => "biblStruct",
            DecoDescChild::Term(_) => "term",
            _ => "",
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
            DecoDescChild::Text(text) => writer.write_text(text),
            DecoDescChild::DecoNote(elem) => elem.serialize_mei(writer),
            DecoDescChild::Head(elem) => elem.serialize_mei(writer),
            DecoDescChild::P(elem) => elem.serialize_mei(writer),
            DecoDescChild::Condition(elem) => elem.serialize_mei(writer),
            DecoDescChild::Dimensions(elem) => elem.serialize_mei(writer),
            DecoDescChild::Height(elem) => elem.serialize_mei(writer),
            DecoDescChild::Width(elem) => elem.serialize_mei(writer),
            DecoDescChild::Depth(elem) => elem.serialize_mei(writer),
            DecoDescChild::Dim(elem) => elem.serialize_mei(writer),
            DecoDescChild::Lb(elem) => elem.serialize_mei(writer),
            DecoDescChild::Rend(elem) => elem.serialize_mei(writer),
            DecoDescChild::Num(elem) => elem.serialize_mei(writer),
            DecoDescChild::Fig(elem) => elem.serialize_mei(writer),
            DecoDescChild::Annot(elem) => elem.serialize_mei(writer),
            DecoDescChild::Bibl(elem) => elem.serialize_mei(writer),
            DecoDescChild::BiblStruct(elem) => elem.serialize_mei(writer),
            DecoDescChild::Term(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// DecoNote
// ============================================================================

impl MeiSerialize for DecoNote {
    fn element_name(&self) -> &'static str {
        "decoNote"
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

impl MeiSerialize for DecoNoteChild {
    fn element_name(&self) -> &'static str {
        match self {
            DecoNoteChild::Text(_) => "",
            DecoNoteChild::DecoNote(_) => "decoNote",
            DecoNoteChild::Head(_) => "head",
            DecoNoteChild::P(_) => "p",
            DecoNoteChild::Condition(_) => "condition",
            DecoNoteChild::Dimensions(_) => "dimensions",
            DecoNoteChild::Height(_) => "height",
            DecoNoteChild::Width(_) => "width",
            DecoNoteChild::Depth(_) => "depth",
            DecoNoteChild::Dim(_) => "dim",
            DecoNoteChild::Lb(_) => "lb",
            DecoNoteChild::Rend(_) => "rend",
            DecoNoteChild::Num(_) => "num",
            DecoNoteChild::Fig(_) => "fig",
            DecoNoteChild::Annot(_) => "annot",
            DecoNoteChild::Bibl(_) => "bibl",
            DecoNoteChild::BiblStruct(_) => "biblStruct",
            DecoNoteChild::Term(_) => "term",
            _ => "",
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
            DecoNoteChild::Text(text) => writer.write_text(text),
            DecoNoteChild::DecoNote(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Head(elem) => elem.serialize_mei(writer),
            DecoNoteChild::P(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Condition(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Dimensions(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Height(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Width(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Depth(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Dim(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Lb(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Rend(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Num(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Fig(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Annot(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Bibl(elem) => elem.serialize_mei(writer),
            DecoNoteChild::BiblStruct(elem) => elem.serialize_mei(writer),
            DecoNoteChild::Term(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// BindingDesc
// ============================================================================

impl MeiSerialize for BindingDesc {
    fn element_name(&self) -> &'static str {
        "bindingDesc"
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

impl MeiSerialize for BindingDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            BindingDescChild::Text(_) => "",
            BindingDescChild::Binding(_) => "binding",
            BindingDescChild::DecoNote(_) => "decoNote",
            BindingDescChild::Head(_) => "head",
            BindingDescChild::P(_) => "p",
            BindingDescChild::Condition(_) => "condition",
            BindingDescChild::Dimensions(_) => "dimensions",
            BindingDescChild::Height(_) => "height",
            BindingDescChild::Width(_) => "width",
            BindingDescChild::Depth(_) => "depth",
            BindingDescChild::Dim(_) => "dim",
            BindingDescChild::Lb(_) => "lb",
            BindingDescChild::Rend(_) => "rend",
            BindingDescChild::Num(_) => "num",
            BindingDescChild::Fig(_) => "fig",
            BindingDescChild::Annot(_) => "annot",
            BindingDescChild::Bibl(_) => "bibl",
            BindingDescChild::BiblStruct(_) => "biblStruct",
            BindingDescChild::Term(_) => "term",
            _ => "",
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
            BindingDescChild::Text(text) => writer.write_text(text),
            BindingDescChild::Binding(elem) => elem.serialize_mei(writer),
            BindingDescChild::DecoNote(elem) => elem.serialize_mei(writer),
            BindingDescChild::Head(elem) => elem.serialize_mei(writer),
            BindingDescChild::P(elem) => elem.serialize_mei(writer),
            BindingDescChild::Condition(elem) => elem.serialize_mei(writer),
            BindingDescChild::Dimensions(elem) => elem.serialize_mei(writer),
            BindingDescChild::Height(elem) => elem.serialize_mei(writer),
            BindingDescChild::Width(elem) => elem.serialize_mei(writer),
            BindingDescChild::Depth(elem) => elem.serialize_mei(writer),
            BindingDescChild::Dim(elem) => elem.serialize_mei(writer),
            BindingDescChild::Lb(elem) => elem.serialize_mei(writer),
            BindingDescChild::Rend(elem) => elem.serialize_mei(writer),
            BindingDescChild::Num(elem) => elem.serialize_mei(writer),
            BindingDescChild::Fig(elem) => elem.serialize_mei(writer),
            BindingDescChild::Annot(elem) => elem.serialize_mei(writer),
            BindingDescChild::Bibl(elem) => elem.serialize_mei(writer),
            BindingDescChild::BiblStruct(elem) => elem.serialize_mei(writer),
            BindingDescChild::Term(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Binding
// ============================================================================

impl MeiSerialize for Binding {
    fn element_name(&self) -> &'static str {
        "binding"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.contemporary.collect_attributes());
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

impl MeiSerialize for BindingChild {
    fn element_name(&self) -> &'static str {
        match self {
            BindingChild::Dimensions(_) => "dimensions",
            BindingChild::P(_) => "p",
            BindingChild::Head(_) => "head",
            BindingChild::Condition(_) => "condition",
            BindingChild::DecoNote(_) => "decoNote",
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
            BindingChild::Dimensions(elem) => elem.serialize_mei(writer),
            BindingChild::P(elem) => elem.serialize_mei(writer),
            BindingChild::Head(elem) => elem.serialize_mei(writer),
            BindingChild::Condition(elem) => elem.serialize_mei(writer),
            BindingChild::DecoNote(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// SealDesc
// ============================================================================

impl MeiSerialize for SealDesc {
    fn element_name(&self) -> &'static str {
        "sealDesc"
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

impl MeiSerialize for SealDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            SealDescChild::Text(_) => "",
            SealDescChild::Seal(_) => "seal",
            SealDescChild::DecoNote(_) => "decoNote",
            SealDescChild::Head(_) => "head",
            SealDescChild::P(_) => "p",
            SealDescChild::Condition(_) => "condition",
            SealDescChild::Dimensions(_) => "dimensions",
            SealDescChild::Height(_) => "height",
            SealDescChild::Width(_) => "width",
            SealDescChild::Depth(_) => "depth",
            SealDescChild::Dim(_) => "dim",
            SealDescChild::Lb(_) => "lb",
            SealDescChild::Rend(_) => "rend",
            SealDescChild::Num(_) => "num",
            SealDescChild::Fig(_) => "fig",
            SealDescChild::Annot(_) => "annot",
            SealDescChild::Bibl(_) => "bibl",
            SealDescChild::BiblStruct(_) => "biblStruct",
            SealDescChild::Term(_) => "term",
            _ => "",
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
            SealDescChild::Text(text) => writer.write_text(text),
            SealDescChild::Seal(elem) => elem.serialize_mei(writer),
            SealDescChild::DecoNote(elem) => elem.serialize_mei(writer),
            SealDescChild::Head(elem) => elem.serialize_mei(writer),
            SealDescChild::P(elem) => elem.serialize_mei(writer),
            SealDescChild::Condition(elem) => elem.serialize_mei(writer),
            SealDescChild::Dimensions(elem) => elem.serialize_mei(writer),
            SealDescChild::Height(elem) => elem.serialize_mei(writer),
            SealDescChild::Width(elem) => elem.serialize_mei(writer),
            SealDescChild::Depth(elem) => elem.serialize_mei(writer),
            SealDescChild::Dim(elem) => elem.serialize_mei(writer),
            SealDescChild::Lb(elem) => elem.serialize_mei(writer),
            SealDescChild::Rend(elem) => elem.serialize_mei(writer),
            SealDescChild::Num(elem) => elem.serialize_mei(writer),
            SealDescChild::Fig(elem) => elem.serialize_mei(writer),
            SealDescChild::Annot(elem) => elem.serialize_mei(writer),
            SealDescChild::Bibl(elem) => elem.serialize_mei(writer),
            SealDescChild::BiblStruct(elem) => elem.serialize_mei(writer),
            SealDescChild::Term(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Seal
// ============================================================================

impl MeiSerialize for Seal {
    fn element_name(&self) -> &'static str {
        "seal"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.contemporary.collect_attributes());
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

impl MeiSerialize for SealChild {
    fn element_name(&self) -> &'static str {
        match self {
            SealChild::Text(_) => "",
            SealChild::DecoNote(_) => "decoNote",
            SealChild::Head(_) => "head",
            SealChild::P(_) => "p",
            SealChild::Condition(_) => "condition",
            SealChild::Dimensions(_) => "dimensions",
            SealChild::Height(_) => "height",
            SealChild::Width(_) => "width",
            SealChild::Depth(_) => "depth",
            SealChild::Dim(_) => "dim",
            SealChild::Lb(_) => "lb",
            SealChild::Rend(_) => "rend",
            SealChild::Num(_) => "num",
            SealChild::Fig(_) => "fig",
            SealChild::Annot(_) => "annot",
            SealChild::Bibl(_) => "bibl",
            SealChild::BiblStruct(_) => "biblStruct",
            SealChild::Term(_) => "term",
            SealChild::Date(_) => "date",
            _ => "",
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
            SealChild::Text(text) => writer.write_text(text),
            SealChild::DecoNote(elem) => elem.serialize_mei(writer),
            SealChild::Head(elem) => elem.serialize_mei(writer),
            SealChild::P(elem) => elem.serialize_mei(writer),
            SealChild::Condition(elem) => elem.serialize_mei(writer),
            SealChild::Dimensions(elem) => elem.serialize_mei(writer),
            SealChild::Height(elem) => elem.serialize_mei(writer),
            SealChild::Width(elem) => elem.serialize_mei(writer),
            SealChild::Depth(elem) => elem.serialize_mei(writer),
            SealChild::Dim(elem) => elem.serialize_mei(writer),
            SealChild::Lb(elem) => elem.serialize_mei(writer),
            SealChild::Rend(elem) => elem.serialize_mei(writer),
            SealChild::Num(elem) => elem.serialize_mei(writer),
            SealChild::Fig(elem) => elem.serialize_mei(writer),
            SealChild::Annot(elem) => elem.serialize_mei(writer),
            SealChild::Bibl(elem) => elem.serialize_mei(writer),
            SealChild::BiblStruct(elem) => elem.serialize_mei(writer),
            SealChild::Term(elem) => elem.serialize_mei(writer),
            SealChild::Date(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

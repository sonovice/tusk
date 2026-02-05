//! Serializer implementations for layout, hand, and script elements.
//!
//! Contains: LayoutDesc, Layout, ColLayout, HandList, Hand, ScriptDesc, ScriptNote.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    ColLayout, Hand, HandChild, HandList, HandListChild, Layout, LayoutChild, LayoutDesc,
    LayoutDescChild, ScriptDesc, ScriptDescChild, ScriptNote, ScriptNoteChild,
};

use super::super::push_attr;

// ============================================================================
// LayoutDesc
// ============================================================================

impl MeiSerialize for LayoutDesc {
    fn element_name(&self) -> &'static str {
        "layoutDesc"
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

impl MeiSerialize for LayoutDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            LayoutDescChild::Text(_) => "",
            LayoutDescChild::Layout(_) => "layout",
            LayoutDescChild::Head(_) => "head",
            LayoutDescChild::P(_) => "p",
            LayoutDescChild::Dimensions(_) => "dimensions",
            LayoutDescChild::Height(_) => "height",
            LayoutDescChild::Width(_) => "width",
            LayoutDescChild::Depth(_) => "depth",
            LayoutDescChild::Dim(_) => "dim",
            LayoutDescChild::Lb(_) => "lb",
            LayoutDescChild::Rend(_) => "rend",
            LayoutDescChild::Num(_) => "num",
            LayoutDescChild::Bibl(_) => "bibl",
            LayoutDescChild::Annot(_) => "annot",
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
            LayoutDescChild::Text(text) => writer.write_text(text),
            LayoutDescChild::Layout(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Head(elem) => elem.serialize_mei(writer),
            LayoutDescChild::P(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Dimensions(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Height(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Width(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Depth(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Dim(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Lb(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Rend(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Num(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Bibl(elem) => elem.serialize_mei(writer),
            LayoutDescChild::Annot(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Layout
// ============================================================================

impl MeiSerialize for Layout {
    fn element_name(&self) -> &'static str {
        "layout"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Element-specific attributes
        if let Some(ref v) = self.cols {
            attrs.push(("cols", v.to_string()));
        }
        if let Some(ref v) = self.ruledlines {
            attrs.push(("ruledlines", v.to_string()));
        }
        if let Some(ref v) = self.writtenlines {
            attrs.push(("writtenlines", v.to_string()));
        }
        if let Some(ref v) = self.ruledstaves {
            attrs.push(("ruledstaves", v.to_string()));
        }
        if let Some(ref v) = self.writtenstaves {
            attrs.push(("writtenstaves", v.to_string()));
        }
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

impl MeiSerialize for LayoutChild {
    fn element_name(&self) -> &'static str {
        match self {
            LayoutChild::Text(_) => "",
            LayoutChild::Head(_) => "head",
            LayoutChild::P(_) => "p",
            LayoutChild::Dimensions(_) => "dimensions",
            LayoutChild::Height(_) => "height",
            LayoutChild::Width(_) => "width",
            LayoutChild::Depth(_) => "depth",
            LayoutChild::Dim(_) => "dim",
            LayoutChild::Lb(_) => "lb",
            LayoutChild::Rend(_) => "rend",
            LayoutChild::Num(_) => "num",
            LayoutChild::Fig(_) => "fig",
            LayoutChild::Bibl(_) => "bibl",
            LayoutChild::Annot(_) => "annot",
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
            LayoutChild::Text(text) => writer.write_text(text),
            LayoutChild::Head(elem) => elem.serialize_mei(writer),
            LayoutChild::P(elem) => elem.serialize_mei(writer),
            LayoutChild::Dimensions(elem) => elem.serialize_mei(writer),
            LayoutChild::Height(elem) => elem.serialize_mei(writer),
            LayoutChild::Width(elem) => elem.serialize_mei(writer),
            LayoutChild::Depth(elem) => elem.serialize_mei(writer),
            LayoutChild::Dim(elem) => elem.serialize_mei(writer),
            LayoutChild::Lb(elem) => elem.serialize_mei(writer),
            LayoutChild::Rend(elem) => elem.serialize_mei(writer),
            LayoutChild::Num(elem) => elem.serialize_mei(writer),
            LayoutChild::Fig(elem) => elem.serialize_mei(writer),
            LayoutChild::Bibl(elem) => elem.serialize_mei(writer),
            LayoutChild::Annot(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// ColLayout
// ============================================================================

impl MeiSerialize for ColLayout {
    fn element_name(&self) -> &'static str {
        "colLayout"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.source.collect_attributes());
        // Element-specific attribute
        if let Some(ref v) = self.cols {
            attrs.push(("cols", v.to_string()));
        }
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
// HandList
// ============================================================================

impl MeiSerialize for HandList {
    fn element_name(&self) -> &'static str {
        "handList"
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

impl MeiSerialize for HandListChild {
    fn element_name(&self) -> &'static str {
        match self {
            HandListChild::Head(_) => "head",
            HandListChild::Label(_) => "label",
            HandListChild::Hand(_) => "hand",
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
            HandListChild::Head(elem) => elem.serialize_mei(writer),
            HandListChild::Label(elem) => elem.serialize_mei(writer),
            HandListChild::Hand(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Hand
// ============================================================================

impl MeiSerialize for Hand {
    fn element_name(&self) -> &'static str {
        "hand"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.medium.collect_attributes());
        // Element-specific attribute
        push_attr!(attrs, "initial", self.initial);
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

impl MeiSerialize for HandChild {
    fn element_name(&self) -> &'static str {
        match self {
            HandChild::Text(_) => "",
            HandChild::Dimensions(_) => "dimensions",
            HandChild::Height(_) => "height",
            HandChild::Width(_) => "width",
            HandChild::Depth(_) => "depth",
            HandChild::Dim(_) => "dim",
            HandChild::Lb(_) => "lb",
            HandChild::Rend(_) => "rend",
            HandChild::Num(_) => "num",
            HandChild::PersName(_) => "persName",
            HandChild::Bibl(_) => "bibl",
            HandChild::Annot(_) => "annot",
            HandChild::Date(_) => "date",
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
            HandChild::Text(text) => writer.write_text(text),
            HandChild::Dimensions(elem) => elem.serialize_mei(writer),
            HandChild::Height(elem) => elem.serialize_mei(writer),
            HandChild::Width(elem) => elem.serialize_mei(writer),
            HandChild::Depth(elem) => elem.serialize_mei(writer),
            HandChild::Dim(elem) => elem.serialize_mei(writer),
            HandChild::Lb(elem) => elem.serialize_mei(writer),
            HandChild::Rend(elem) => elem.serialize_mei(writer),
            HandChild::Num(elem) => elem.serialize_mei(writer),
            HandChild::PersName(elem) => elem.serialize_mei(writer),
            HandChild::Bibl(elem) => elem.serialize_mei(writer),
            HandChild::Annot(elem) => elem.serialize_mei(writer),
            HandChild::Date(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// ScriptDesc
// ============================================================================

impl MeiSerialize for ScriptDesc {
    fn element_name(&self) -> &'static str {
        "scriptDesc"
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

impl MeiSerialize for ScriptDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            ScriptDescChild::Text(_) => "",
            ScriptDescChild::ScriptNote(_) => "scriptNote",
            ScriptDescChild::Head(_) => "head",
            ScriptDescChild::P(_) => "p",
            ScriptDescChild::Dimensions(_) => "dimensions",
            ScriptDescChild::Height(_) => "height",
            ScriptDescChild::Width(_) => "width",
            ScriptDescChild::Depth(_) => "depth",
            ScriptDescChild::Dim(_) => "dim",
            ScriptDescChild::Lb(_) => "lb",
            ScriptDescChild::Rend(_) => "rend",
            ScriptDescChild::Num(_) => "num",
            ScriptDescChild::Bibl(_) => "bibl",
            ScriptDescChild::Annot(_) => "annot",
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
            ScriptDescChild::Text(text) => writer.write_text(text),
            ScriptDescChild::ScriptNote(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Head(elem) => elem.serialize_mei(writer),
            ScriptDescChild::P(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Dimensions(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Height(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Width(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Depth(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Dim(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Lb(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Rend(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Num(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Bibl(elem) => elem.serialize_mei(writer),
            ScriptDescChild::Annot(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// ScriptNote
// ============================================================================

impl MeiSerialize for ScriptNote {
    fn element_name(&self) -> &'static str {
        "scriptNote"
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

impl MeiSerialize for ScriptNoteChild {
    fn element_name(&self) -> &'static str {
        match self {
            ScriptNoteChild::Text(_) => "",
            ScriptNoteChild::Head(_) => "head",
            ScriptNoteChild::P(_) => "p",
            ScriptNoteChild::Dimensions(_) => "dimensions",
            ScriptNoteChild::Height(_) => "height",
            ScriptNoteChild::Width(_) => "width",
            ScriptNoteChild::Depth(_) => "depth",
            ScriptNoteChild::Dim(_) => "dim",
            ScriptNoteChild::Lb(_) => "lb",
            ScriptNoteChild::Rend(_) => "rend",
            ScriptNoteChild::Num(_) => "num",
            ScriptNoteChild::Bibl(_) => "bibl",
            ScriptNoteChild::Annot(_) => "annot",
            ScriptNoteChild::Date(_) => "date",
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
            ScriptNoteChild::Text(text) => writer.write_text(text),
            ScriptNoteChild::Head(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::P(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Dimensions(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Height(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Width(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Depth(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Dim(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Lb(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Rend(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Num(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Bibl(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Annot(elem) => elem.serialize_mei(writer),
            ScriptNoteChild::Date(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

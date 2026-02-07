//! Serializer implementations for symbol-related MEI elements.
//!
//! This module contains serializers for:
//! - SymbolTable (container for symbol definitions)
//! - SymbolDef (symbol definition)
//! - SymName (symbol name)
//! - SymProp (symbol property)
//! - PropName (property name)
//! - PropValue (property value)
//! - Mapping (character mapping)

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::AttDataSelecting;
use tusk_model::elements::{
    Mapping, MappingChild, PropName, PropNameChild, PropValue, PropValueChild, SymName,
    SymNameChild, SymProp, SymPropChild, SymbolDef, SymbolDefChild, SymbolTable, SymbolTableChild,
};

use super::{push_attr, to_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// SymbolTable element
// ============================================================================

impl MeiSerialize for SymbolTable {
    fn element_name(&self) -> &'static str {
        "symbolTable"
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

impl MeiSerialize for SymbolTableChild {
    fn element_name(&self) -> &'static str {
        match self {
            SymbolTableChild::SymbolDef(_) => "symbolDef",
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
            SymbolTableChild::SymbolDef(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// SymbolDef element
// ============================================================================

impl MeiSerialize for SymbolDef {
    fn element_name(&self) -> &'static str {
        "symbolDef"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.coordinated.collect_attributes());
        attrs.extend(self.data_selecting.collect_attributes());
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

impl MeiSerialize for SymbolDefChild {
    fn element_name(&self) -> &'static str {
        match self {
            SymbolDefChild::Annot(_) => "annot",
            SymbolDefChild::SymProp(_) => "symProp",
            SymbolDefChild::Symbol(_) => "symbol",
            SymbolDefChild::SymName(_) => "symName",
            SymbolDefChild::Graphic(_) => "graphic",
            SymbolDefChild::Curve(_) => "curve",
            SymbolDefChild::Line(_) => "line",
            SymbolDefChild::AnchoredText(_) => "anchoredText",
            SymbolDefChild::Mapping(_) => "mapping",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            SymbolDefChild::Annot(elem) => elem.collect_all_attributes(),
            SymbolDefChild::SymProp(elem) => elem.collect_all_attributes(),
            SymbolDefChild::Symbol(elem) => elem.collect_all_attributes(),
            SymbolDefChild::SymName(elem) => elem.collect_all_attributes(),
            SymbolDefChild::Graphic(elem) => elem.collect_all_attributes(),
            SymbolDefChild::Curve(elem) => elem.collect_all_attributes(),
            SymbolDefChild::Line(elem) => elem.collect_all_attributes(),
            SymbolDefChild::AnchoredText(elem) => elem.collect_all_attributes(),
            SymbolDefChild::Mapping(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            SymbolDefChild::Annot(elem) => elem.has_children(),
            SymbolDefChild::SymProp(elem) => elem.has_children(),
            SymbolDefChild::Symbol(elem) => elem.has_children(),
            SymbolDefChild::SymName(elem) => elem.has_children(),
            SymbolDefChild::Graphic(elem) => elem.has_children(),
            SymbolDefChild::Curve(elem) => elem.has_children(),
            SymbolDefChild::Line(elem) => elem.has_children(),
            SymbolDefChild::AnchoredText(elem) => elem.has_children(),
            SymbolDefChild::Mapping(elem) => elem.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SymbolDefChild::Annot(elem) => elem.serialize_children(writer),
            SymbolDefChild::SymProp(elem) => elem.serialize_children(writer),
            SymbolDefChild::Symbol(elem) => elem.serialize_children(writer),
            SymbolDefChild::SymName(elem) => elem.serialize_children(writer),
            SymbolDefChild::Graphic(elem) => elem.serialize_children(writer),
            SymbolDefChild::Curve(elem) => elem.serialize_children(writer),
            SymbolDefChild::Line(elem) => elem.serialize_children(writer),
            SymbolDefChild::AnchoredText(elem) => elem.serialize_children(writer),
            SymbolDefChild::Mapping(elem) => elem.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SymbolDefChild::Annot(elem) => elem.serialize_mei(writer),
            SymbolDefChild::SymProp(elem) => elem.serialize_mei(writer),
            SymbolDefChild::Symbol(elem) => elem.serialize_mei(writer),
            SymbolDefChild::SymName(elem) => elem.serialize_mei(writer),
            SymbolDefChild::Graphic(elem) => elem.serialize_mei(writer),
            SymbolDefChild::Curve(elem) => elem.serialize_mei(writer),
            SymbolDefChild::Line(elem) => elem.serialize_mei(writer),
            SymbolDefChild::AnchoredText(elem) => elem.serialize_mei(writer),
            SymbolDefChild::Mapping(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// SymName element
// ============================================================================

impl MeiSerialize for SymName {
    fn element_name(&self) -> &'static str {
        "symName"
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

impl MeiSerialize for SymNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            SymNameChild::Text(_) => "$text",
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
            SymNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// SymProp element
// ============================================================================

impl MeiSerialize for SymProp {
    fn element_name(&self) -> &'static str {
        "symProp"
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

impl MeiSerialize for SymPropChild {
    fn element_name(&self) -> &'static str {
        match self {
            SymPropChild::PropName(_) => "propName",
            SymPropChild::PropValue(_) => "propValue",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            SymPropChild::PropName(elem) => elem.collect_all_attributes(),
            SymPropChild::PropValue(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            SymPropChild::PropName(elem) => elem.has_children(),
            SymPropChild::PropValue(elem) => elem.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SymPropChild::PropName(elem) => elem.serialize_children(writer),
            SymPropChild::PropValue(elem) => elem.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            SymPropChild::PropName(elem) => elem.serialize_mei(writer),
            SymPropChild::PropValue(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// PropName element
// ============================================================================

impl MeiSerialize for PropName {
    fn element_name(&self) -> &'static str {
        "propName"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        push_attr!(attrs, "type", string self.r#type);
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

impl MeiSerialize for PropNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            PropNameChild::Text(_) => "$text",
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
            PropNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// PropValue element
// ============================================================================

impl MeiSerialize for PropValue {
    fn element_name(&self) -> &'static str {
        "propValue"
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

impl MeiSerialize for PropValueChild {
    fn element_name(&self) -> &'static str {
        match self {
            PropValueChild::Text(_) => "$text",
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
            PropValueChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
        }
    }
}

// ============================================================================
// Mapping element
// ============================================================================

impl MeiSerialize for Mapping {
    fn element_name(&self) -> &'static str {
        "mapping"
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

impl MeiSerialize for MappingChild {
    fn element_name(&self) -> &'static str {
        match self {
            MappingChild::Text(_) => "$text",
            MappingChild::Symbol(_) => "symbol",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        match self {
            MappingChild::Text(_) => Vec::new(),
            MappingChild::Symbol(elem) => elem.collect_all_attributes(),
        }
    }

    fn has_children(&self) -> bool {
        match self {
            MappingChild::Text(_) => false,
            MappingChild::Symbol(elem) => elem.has_children(),
        }
    }

    fn serialize_children<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MappingChild::Text(_) => Ok(()),
            MappingChild::Symbol(elem) => elem.serialize_children(writer),
        }
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            MappingChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            MappingChild::Symbol(elem) => elem.serialize_mei(writer),
        }
    }
}

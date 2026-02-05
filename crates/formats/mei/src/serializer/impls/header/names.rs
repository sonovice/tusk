//! Serializer implementations for name elements.
//!
//! Contains: ForeName, FamName, AddName, GenName, NameLink, PeriodName, StyleName

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    AddName, AddNameChild, FamName, FamNameChild, ForeName, ForeNameChild, GenName, GenNameChild,
    NameLink, NameLinkChild, PeriodName, PeriodNameChild, StyleName, StyleNameChild,
};

// ============================================================================
// ForeName
// ============================================================================

impl MeiSerialize for ForeName {
    fn element_name(&self) -> &'static str {
        "foreName"
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

impl MeiSerialize for ForeNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            ForeNameChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, ForeNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            ForeNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ForeNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// FamName
// ============================================================================

impl MeiSerialize for FamName {
    fn element_name(&self) -> &'static str {
        "famName"
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

impl MeiSerialize for FamNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            FamNameChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, FamNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            FamNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "FamNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// AddName
// ============================================================================

impl MeiSerialize for AddName {
    fn element_name(&self) -> &'static str {
        "addName"
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

impl MeiSerialize for AddNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            AddNameChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, AddNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            AddNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "AddNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// GenName
// ============================================================================

impl MeiSerialize for GenName {
    fn element_name(&self) -> &'static str {
        "genName"
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

impl MeiSerialize for GenNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            GenNameChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, GenNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            GenNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "GenNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// NameLink
// ============================================================================

impl MeiSerialize for NameLink {
    fn element_name(&self) -> &'static str {
        "nameLink"
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

impl MeiSerialize for NameLinkChild {
    fn element_name(&self) -> &'static str {
        match self {
            NameLinkChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, NameLinkChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            NameLinkChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "NameLinkChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// PeriodName
// ============================================================================

impl MeiSerialize for PeriodName {
    fn element_name(&self) -> &'static str {
        "periodName"
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

impl MeiSerialize for PeriodNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            PeriodNameChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, PeriodNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            PeriodNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PeriodNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// StyleName
// ============================================================================

impl MeiSerialize for StyleName {
    fn element_name(&self) -> &'static str {
        "styleName"
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

impl MeiSerialize for StyleNameChild {
    fn element_name(&self) -> &'static str {
        match self {
            StyleNameChild::Text(_) => "#text",
            _ => "unknown",
        }
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }

    fn has_children(&self) -> bool {
        !matches!(self, StyleNameChild::Text(_))
    }

    fn serialize_children<W: Write>(&self, _writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        Ok(())
    }

    fn serialize_mei<W: Write>(&self, writer: &mut MeiWriter<W>) -> SerializeResult<()> {
        match self {
            StyleNameChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "StyleNameChild::{}",
                other.element_name()
            ))),
        }
    }
}

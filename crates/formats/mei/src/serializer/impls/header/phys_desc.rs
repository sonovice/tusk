//! Serializer implementations for physical description elements.
//!
//! Contains: Dimensions, Height, Width, Depth, Dim, Support, SupportDesc, Collation, Foliation, Condition.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::AttQuantity;
use tusk_model::elements::{
    Collation, CollationChild, Condition, ConditionChild, Depth, DepthChild, Dim, DimChild,
    Dimensions, DimensionsChild, Foliation, FoliationChild, Height, HeightChild, Support,
    SupportChild, SupportDesc, SupportDescChild, Width, WidthChild,
};

use super::super::push_attr;

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// Dimensions
// ============================================================================

impl MeiSerialize for Dimensions {
    fn element_name(&self) -> &'static str {
        "dimensions"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.measurement.collect_attributes());
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

impl MeiSerialize for DimensionsChild {
    fn element_name(&self) -> &'static str {
        match self {
            DimensionsChild::Text(_) => "",
            DimensionsChild::Head(_) => "head",
            DimensionsChild::P(_) => "p",
            DimensionsChild::Height(_) => "height",
            DimensionsChild::Width(_) => "width",
            DimensionsChild::Depth(_) => "depth",
            DimensionsChild::Dim(_) => "dim",
            DimensionsChild::Dimensions(_) => "dimensions",
            DimensionsChild::Lb(_) => "lb",
            DimensionsChild::Rend(_) => "rend",
            DimensionsChild::Num(_) => "num",
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
            DimensionsChild::Text(text) => writer.write_text(text),
            DimensionsChild::Head(elem) => elem.serialize_mei(writer),
            DimensionsChild::P(elem) => elem.serialize_mei(writer),
            DimensionsChild::Height(elem) => elem.serialize_mei(writer),
            DimensionsChild::Width(elem) => elem.serialize_mei(writer),
            DimensionsChild::Depth(elem) => elem.serialize_mei(writer),
            DimensionsChild::Dim(elem) => elem.serialize_mei(writer),
            DimensionsChild::Dimensions(elem) => elem.serialize_mei(writer),
            DimensionsChild::Lb(elem) => elem.serialize_mei(writer),
            DimensionsChild::Rend(elem) => elem.serialize_mei(writer),
            DimensionsChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Height
// ============================================================================

impl MeiSerialize for Height {
    fn element_name(&self) -> &'static str {
        "height"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.quantity.collect_attributes());
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

impl MeiSerialize for HeightChild {
    fn element_name(&self) -> &'static str {
        match self {
            HeightChild::Text(_) => "",
            HeightChild::Height(_) => "height",
            HeightChild::Width(_) => "width",
            HeightChild::Depth(_) => "depth",
            HeightChild::Dim(_) => "dim",
            HeightChild::Dimensions(_) => "dimensions",
            HeightChild::Lb(_) => "lb",
            HeightChild::Rend(_) => "rend",
            HeightChild::Num(_) => "num",
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
            HeightChild::Text(text) => writer.write_text(text),
            HeightChild::Height(elem) => elem.serialize_mei(writer),
            HeightChild::Width(elem) => elem.serialize_mei(writer),
            HeightChild::Depth(elem) => elem.serialize_mei(writer),
            HeightChild::Dim(elem) => elem.serialize_mei(writer),
            HeightChild::Dimensions(elem) => elem.serialize_mei(writer),
            HeightChild::Lb(elem) => elem.serialize_mei(writer),
            HeightChild::Rend(elem) => elem.serialize_mei(writer),
            HeightChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Width
// ============================================================================

impl MeiSerialize for Width {
    fn element_name(&self) -> &'static str {
        "width"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.quantity.collect_attributes());
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

impl MeiSerialize for WidthChild {
    fn element_name(&self) -> &'static str {
        match self {
            WidthChild::Text(_) => "",
            WidthChild::Height(_) => "height",
            WidthChild::Width(_) => "width",
            WidthChild::Depth(_) => "depth",
            WidthChild::Dim(_) => "dim",
            WidthChild::Dimensions(_) => "dimensions",
            WidthChild::Lb(_) => "lb",
            WidthChild::Rend(_) => "rend",
            WidthChild::Num(_) => "num",
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
            WidthChild::Text(text) => writer.write_text(text),
            WidthChild::Height(elem) => elem.serialize_mei(writer),
            WidthChild::Width(elem) => elem.serialize_mei(writer),
            WidthChild::Depth(elem) => elem.serialize_mei(writer),
            WidthChild::Dim(elem) => elem.serialize_mei(writer),
            WidthChild::Dimensions(elem) => elem.serialize_mei(writer),
            WidthChild::Lb(elem) => elem.serialize_mei(writer),
            WidthChild::Rend(elem) => elem.serialize_mei(writer),
            WidthChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Depth
// ============================================================================

impl MeiSerialize for Depth {
    fn element_name(&self) -> &'static str {
        "depth"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.quantity.collect_attributes());
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

impl MeiSerialize for DepthChild {
    fn element_name(&self) -> &'static str {
        match self {
            DepthChild::Text(_) => "",
            DepthChild::Height(_) => "height",
            DepthChild::Width(_) => "width",
            DepthChild::Depth(_) => "depth",
            DepthChild::Dim(_) => "dim",
            DepthChild::Dimensions(_) => "dimensions",
            DepthChild::Lb(_) => "lb",
            DepthChild::Rend(_) => "rend",
            DepthChild::Num(_) => "num",
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
            DepthChild::Text(text) => writer.write_text(text),
            DepthChild::Height(elem) => elem.serialize_mei(writer),
            DepthChild::Width(elem) => elem.serialize_mei(writer),
            DepthChild::Depth(elem) => elem.serialize_mei(writer),
            DepthChild::Dim(elem) => elem.serialize_mei(writer),
            DepthChild::Dimensions(elem) => elem.serialize_mei(writer),
            DepthChild::Lb(elem) => elem.serialize_mei(writer),
            DepthChild::Rend(elem) => elem.serialize_mei(writer),
            DepthChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Dim
// ============================================================================

impl MeiSerialize for Dim {
    fn element_name(&self) -> &'static str {
        "dim"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.quantity.collect_attributes());
        // Element-specific attribute
        push_attr!(attrs, "form", clone self.form);
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

impl MeiSerialize for DimChild {
    fn element_name(&self) -> &'static str {
        match self {
            DimChild::Text(_) => "",
            DimChild::Height(_) => "height",
            DimChild::Width(_) => "width",
            DimChild::Depth(_) => "depth",
            DimChild::Dim(_) => "dim",
            DimChild::Dimensions(_) => "dimensions",
            DimChild::Lb(_) => "lb",
            DimChild::Rend(_) => "rend",
            DimChild::Num(_) => "num",
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
            DimChild::Text(text) => writer.write_text(text),
            DimChild::Height(elem) => elem.serialize_mei(writer),
            DimChild::Width(elem) => elem.serialize_mei(writer),
            DimChild::Depth(elem) => elem.serialize_mei(writer),
            DimChild::Dim(elem) => elem.serialize_mei(writer),
            DimChild::Dimensions(elem) => elem.serialize_mei(writer),
            DimChild::Lb(elem) => elem.serialize_mei(writer),
            DimChild::Rend(elem) => elem.serialize_mei(writer),
            DimChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Support
// ============================================================================

impl MeiSerialize for Support {
    fn element_name(&self) -> &'static str {
        "support"
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

impl MeiSerialize for SupportChild {
    fn element_name(&self) -> &'static str {
        match self {
            SupportChild::Head(_) => "head",
            SupportChild::P(_) => "p",
            SupportChild::Dimensions(_) => "dimensions",
            SupportChild::Condition(_) => "condition",
            SupportChild::DecoNote(_) => "decoNote",
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
            SupportChild::Head(elem) => elem.serialize_mei(writer),
            SupportChild::P(elem) => elem.serialize_mei(writer),
            SupportChild::Dimensions(elem) => elem.serialize_mei(writer),
            SupportChild::Condition(elem) => elem.serialize_mei(writer),
            // decoNote not yet implemented
            SupportChild::DecoNote(_) => Ok(()),
        }
    }
}

// ============================================================================
// SupportDesc
// ============================================================================

impl MeiSerialize for SupportDesc {
    fn element_name(&self) -> &'static str {
        "supportDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        // Element-specific attribute
        push_attr!(attrs, "material", clone self.material);
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

impl MeiSerialize for SupportDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            SupportDescChild::Text(_) => "",
            SupportDescChild::Head(_) => "head",
            SupportDescChild::P(_) => "p",
            SupportDescChild::Support(_) => "support",
            SupportDescChild::Dimensions(_) => "dimensions",
            SupportDescChild::Height(_) => "height",
            SupportDescChild::Width(_) => "width",
            SupportDescChild::Depth(_) => "depth",
            SupportDescChild::Dim(_) => "dim",
            SupportDescChild::Collation(_) => "collation",
            SupportDescChild::Foliation(_) => "foliation",
            SupportDescChild::Condition(_) => "condition",
            SupportDescChild::Lb(_) => "lb",
            SupportDescChild::Rend(_) => "rend",
            SupportDescChild::Num(_) => "num",
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
            SupportDescChild::Text(text) => writer.write_text(text),
            SupportDescChild::Head(elem) => elem.serialize_mei(writer),
            SupportDescChild::P(elem) => elem.serialize_mei(writer),
            SupportDescChild::Support(elem) => elem.serialize_mei(writer),
            SupportDescChild::Dimensions(elem) => elem.serialize_mei(writer),
            SupportDescChild::Height(elem) => elem.serialize_mei(writer),
            SupportDescChild::Width(elem) => elem.serialize_mei(writer),
            SupportDescChild::Depth(elem) => elem.serialize_mei(writer),
            SupportDescChild::Dim(elem) => elem.serialize_mei(writer),
            SupportDescChild::Collation(elem) => elem.serialize_mei(writer),
            SupportDescChild::Foliation(elem) => elem.serialize_mei(writer),
            SupportDescChild::Condition(elem) => elem.serialize_mei(writer),
            SupportDescChild::Lb(elem) => elem.serialize_mei(writer),
            SupportDescChild::Rend(elem) => elem.serialize_mei(writer),
            SupportDescChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Collation
// ============================================================================

impl MeiSerialize for Collation {
    fn element_name(&self) -> &'static str {
        "collation"
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

impl MeiSerialize for CollationChild {
    fn element_name(&self) -> &'static str {
        match self {
            CollationChild::Text(_) => "",
            CollationChild::Head(_) => "head",
            CollationChild::P(_) => "p",
            CollationChild::Dimensions(_) => "dimensions",
            CollationChild::Height(_) => "height",
            CollationChild::Width(_) => "width",
            CollationChild::Depth(_) => "depth",
            CollationChild::Dim(_) => "dim",
            CollationChild::Lb(_) => "lb",
            CollationChild::Rend(_) => "rend",
            CollationChild::Num(_) => "num",
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
            CollationChild::Text(text) => writer.write_text(text),
            CollationChild::Head(elem) => elem.serialize_mei(writer),
            CollationChild::P(elem) => elem.serialize_mei(writer),
            CollationChild::Dimensions(elem) => elem.serialize_mei(writer),
            CollationChild::Height(elem) => elem.serialize_mei(writer),
            CollationChild::Width(elem) => elem.serialize_mei(writer),
            CollationChild::Depth(elem) => elem.serialize_mei(writer),
            CollationChild::Dim(elem) => elem.serialize_mei(writer),
            CollationChild::Lb(elem) => elem.serialize_mei(writer),
            CollationChild::Rend(elem) => elem.serialize_mei(writer),
            CollationChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Foliation
// ============================================================================

impl MeiSerialize for Foliation {
    fn element_name(&self) -> &'static str {
        "foliation"
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

impl MeiSerialize for FoliationChild {
    fn element_name(&self) -> &'static str {
        match self {
            FoliationChild::Text(_) => "",
            FoliationChild::Head(_) => "head",
            FoliationChild::P(_) => "p",
            FoliationChild::Dimensions(_) => "dimensions",
            FoliationChild::Height(_) => "height",
            FoliationChild::Width(_) => "width",
            FoliationChild::Depth(_) => "depth",
            FoliationChild::Dim(_) => "dim",
            FoliationChild::Lb(_) => "lb",
            FoliationChild::Rend(_) => "rend",
            FoliationChild::Num(_) => "num",
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
            FoliationChild::Text(text) => writer.write_text(text),
            FoliationChild::Head(elem) => elem.serialize_mei(writer),
            FoliationChild::P(elem) => elem.serialize_mei(writer),
            FoliationChild::Dimensions(elem) => elem.serialize_mei(writer),
            FoliationChild::Height(elem) => elem.serialize_mei(writer),
            FoliationChild::Width(elem) => elem.serialize_mei(writer),
            FoliationChild::Depth(elem) => elem.serialize_mei(writer),
            FoliationChild::Dim(elem) => elem.serialize_mei(writer),
            FoliationChild::Lb(elem) => elem.serialize_mei(writer),
            FoliationChild::Rend(elem) => elem.serialize_mei(writer),
            FoliationChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// Condition
// ============================================================================

impl MeiSerialize for Condition {
    fn element_name(&self) -> &'static str {
        "condition"
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

impl MeiSerialize for ConditionChild {
    fn element_name(&self) -> &'static str {
        match self {
            ConditionChild::Text(_) => "",
            ConditionChild::Head(_) => "head",
            ConditionChild::P(_) => "p",
            ConditionChild::Dimensions(_) => "dimensions",
            ConditionChild::Height(_) => "height",
            ConditionChild::Width(_) => "width",
            ConditionChild::Depth(_) => "depth",
            ConditionChild::Dim(_) => "dim",
            ConditionChild::Lb(_) => "lb",
            ConditionChild::Rend(_) => "rend",
            ConditionChild::Num(_) => "num",
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
            ConditionChild::Text(text) => writer.write_text(text),
            ConditionChild::Head(elem) => elem.serialize_mei(writer),
            ConditionChild::P(elem) => elem.serialize_mei(writer),
            ConditionChild::Dimensions(elem) => elem.serialize_mei(writer),
            ConditionChild::Height(elem) => elem.serialize_mei(writer),
            ConditionChild::Width(elem) => elem.serialize_mei(writer),
            ConditionChild::Depth(elem) => elem.serialize_mei(writer),
            ConditionChild::Dim(elem) => elem.serialize_mei(writer),
            ConditionChild::Lb(elem) => elem.serialize_mei(writer),
            ConditionChild::Rend(elem) => elem.serialize_mei(writer),
            ConditionChild::Num(elem) => elem.serialize_mei(writer),
            // Skip other children for now
            _ => Ok(()),
        }
    }
}

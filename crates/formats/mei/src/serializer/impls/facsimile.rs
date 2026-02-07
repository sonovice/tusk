//! Serializer implementations for facsimile-related MEI elements.
//!
//! This module contains serializers for:
//! - Facsimile (container for surface elements)
//! - Surface (a writing surface in terms of coordinates)
//! - Zone (area of interest within a surface)
//! - Graphic (inline graphic reference)

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttCoordinated, AttCoordinatedUl, AttDimensions, AttStartId, AttVisualOffset,
};
use tusk_model::elements::{
    Facsimile, FacsimileChild, Graphic, GraphicChild, Surface, SurfaceChild, Zone, ZoneChild,
};

use super::{push_attr, to_attr_string};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// Facsimile element
// ============================================================================

impl MeiSerialize for Facsimile {
    fn element_name(&self) -> &'static str {
        "facsimile"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
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

impl MeiSerialize for FacsimileChild {
    fn element_name(&self) -> &'static str {
        match self {
            FacsimileChild::Surface(_) => "surface",
            FacsimileChild::Graphic(_) => "graphic",
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
            FacsimileChild::Surface(elem) => elem.serialize_mei(writer),
            FacsimileChild::Graphic(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Surface element
// ============================================================================

impl MeiSerialize for Surface {
    fn element_name(&self) -> &'static str {
        "surface"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.coordinated.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.start_id.collect_attributes());
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

impl MeiSerialize for SurfaceChild {
    fn element_name(&self) -> &'static str {
        match self {
            SurfaceChild::Graphic(_) => "graphic",
            SurfaceChild::Zone(_) => "zone",
            SurfaceChild::FigDesc(_) => "figDesc",
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
            SurfaceChild::Graphic(elem) => elem.serialize_mei(writer),
            SurfaceChild::Zone(elem) => elem.serialize_mei(writer),
            SurfaceChild::FigDesc(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Zone element
// ============================================================================

impl MeiSerialize for Zone {
    fn element_name(&self) -> &'static str {
        "zone"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.coordinated.collect_attributes());
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

impl MeiSerialize for ZoneChild {
    fn element_name(&self) -> &'static str {
        match self {
            ZoneChild::FigDesc(_) => "figDesc",
            ZoneChild::Graphic(_) => "graphic",
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
            ZoneChild::FigDesc(elem) => elem.serialize_mei(writer),
            ZoneChild::Graphic(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Graphic element
// ============================================================================

impl MeiSerialize for Graphic {
    fn element_name(&self) -> &'static str {
        "graphic"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.coordinated_ul.collect_attributes());
        attrs.extend(self.dimensions.collect_attributes());
        attrs.extend(self.internet_media.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.start_id.collect_attributes());
        attrs.extend(self.visual_offset.collect_attributes());
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

impl MeiSerialize for GraphicChild {
    fn element_name(&self) -> &'static str {
        match self {
            GraphicChild::Zone(_) => "zone",
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
            GraphicChild::Zone(elem) => elem.serialize_mei(writer),
        }
    }
}

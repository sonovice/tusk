//! Serializer implementations for manifestation list elements.
//!
//! Contains: ManifestationList, Manifestation, ManifestationChild.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Manifestation, ManifestationChild, ManifestationList, ManifestationListChild,
};

// ============================================================================
// ManifestationList
// ============================================================================

impl MeiSerialize for ManifestationList {
    fn element_name(&self) -> &'static str {
        "manifestationList"
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

impl MeiSerialize for ManifestationListChild {
    fn element_name(&self) -> &'static str {
        match self {
            ManifestationListChild::Head(_) => "head",
            ManifestationListChild::Manifestation(_) => "manifestation",
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
            ManifestationListChild::Head(elem) => elem.serialize_mei(writer),
            ManifestationListChild::Manifestation(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Manifestation
// ============================================================================

impl MeiSerialize for Manifestation {
    fn element_name(&self) -> &'static str {
        "manifestation"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.component_type.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.record_type.collect_attributes());
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

impl MeiSerialize for ManifestationChild {
    fn element_name(&self) -> &'static str {
        match self {
            ManifestationChild::Contents(_) => "contents",
            ManifestationChild::Availability(_) => "availability",
            ManifestationChild::BiblList(_) => "biblList",
            ManifestationChild::Classification(_) => "classification",
            ManifestationChild::RelationList(_) => "relationList",
            ManifestationChild::SeriesStmt(_) => "seriesStmt",
            ManifestationChild::PubStmt(_) => "pubStmt",
            ManifestationChild::NotesStmt(_) => "notesStmt",
            ManifestationChild::LocusGrp(_) => "locusGrp",
            ManifestationChild::LangUsage(_) => "langUsage",
            ManifestationChild::ExtMeta(_) => "extMeta",
            ManifestationChild::Identifier(_) => "identifier",
            ManifestationChild::TitleStmt(_) => "titleStmt",
            ManifestationChild::Creation(_) => "creation",
            ManifestationChild::PhysLoc(_) => "physLoc",
            ManifestationChild::ComponentList(_) => "componentList",
            ManifestationChild::ItemList(_) => "itemList",
            ManifestationChild::EditionStmt(_) => "editionStmt",
            ManifestationChild::Dedication(_) => "dedication",
            ManifestationChild::History(_) => "history",
            ManifestationChild::Head(_) => "head",
            ManifestationChild::Locus(_) => "locus",
            ManifestationChild::PhysDesc(_) => "physDesc",
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
            ManifestationChild::Contents(elem) => elem.serialize_mei(writer),
            ManifestationChild::Availability(elem) => elem.serialize_mei(writer),
            ManifestationChild::SeriesStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::PubStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::NotesStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::ExtMeta(elem) => elem.serialize_mei(writer),
            ManifestationChild::Identifier(elem) => elem.serialize_mei(writer),
            ManifestationChild::TitleStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::EditionStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::Head(elem) => elem.serialize_mei(writer),
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

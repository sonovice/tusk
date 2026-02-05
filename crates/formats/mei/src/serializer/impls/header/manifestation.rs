//! Serializer implementations for manifestation list elements.
//!
//! Contains: ManifestationList, Manifestation, ManifestationChild, PhysDesc, PlateNum.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    Manifestation, ManifestationChild, ManifestationList, ManifestationListChild, PhysDesc,
    PhysDescChild, PlateNum, PlateNumChild,
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
            ManifestationChild::Classification(elem) => elem.serialize_mei(writer),
            ManifestationChild::SeriesStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::PubStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::NotesStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::LangUsage(elem) => elem.serialize_mei(writer),
            ManifestationChild::ExtMeta(elem) => elem.serialize_mei(writer),
            ManifestationChild::Identifier(elem) => elem.serialize_mei(writer),
            ManifestationChild::TitleStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::Creation(elem) => elem.serialize_mei(writer),
            ManifestationChild::EditionStmt(elem) => elem.serialize_mei(writer),
            ManifestationChild::History(elem) => elem.serialize_mei(writer),
            ManifestationChild::Head(elem) => elem.serialize_mei(writer),
            ManifestationChild::Locus(elem) => elem.serialize_mei(writer),
            ManifestationChild::PhysDesc(elem) => elem.serialize_mei(writer),
            // Elements not yet fully implemented - return error
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "ManifestationChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// PhysDesc
// ============================================================================

impl MeiSerialize for PhysDesc {
    fn element_name(&self) -> &'static str {
        "physDesc"
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

impl MeiSerialize for PhysDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            PhysDescChild::P(_) => "p",
            PhysDescChild::HandList(_) => "handList",
            PhysDescChild::SecFolio(_) => "secFolio",
            PhysDescChild::Explicit(_) => "explicit",
            PhysDescChild::AddDesc(_) => "addDesc",
            PhysDescChild::Dimensions(_) => "dimensions",
            PhysDescChild::FileChar(_) => "fileChar",
            PhysDescChild::History(_) => "history",
            PhysDescChild::Condition(_) => "condition",
            PhysDescChild::Catchwords(_) => "catchwords",
            PhysDescChild::Inscription(_) => "inscription",
            PhysDescChild::BindingDesc(_) => "bindingDesc",
            PhysDescChild::LayoutDesc(_) => "layoutDesc",
            PhysDescChild::WatermarkDesc(_) => "watermarkDesc",
            PhysDescChild::PlateNum(_) => "plateNum",
            PhysDescChild::SealDesc(_) => "sealDesc",
            PhysDescChild::PlayingSpeed(_) => "playingSpeed",
            PhysDescChild::Heraldry(_) => "heraldry",
            PhysDescChild::CarrierForm(_) => "carrierForm",
            PhysDescChild::Incip(_) => "incip",
            PhysDescChild::SpecRepro(_) => "specRepro",
            PhysDescChild::PerfDuration(_) => "perfDuration",
            PhysDescChild::CaptureMode(_) => "captureMode",
            PhysDescChild::ScriptDesc(_) => "scriptDesc",
            PhysDescChild::Stamp(_) => "stamp",
            PhysDescChild::AccMat(_) => "accMat",
            PhysDescChild::PhysMedium(_) => "physMedium",
            PhysDescChild::SupportDesc(_) => "supportDesc",
            PhysDescChild::Colophon(_) => "colophon",
            PhysDescChild::TypeDesc(_) => "typeDesc",
            PhysDescChild::Extent(_) => "extent",
            PhysDescChild::DecoDesc(_) => "decoDesc",
            PhysDescChild::TrackConfig(_) => "trackConfig",
            PhysDescChild::FoliaDesc(_) => "foliaDesc",
            PhysDescChild::TitlePage(_) => "titlePage",
            PhysDescChild::ScoreFormat(_) => "scoreFormat",
            PhysDescChild::SoundChan(_) => "soundChan",
            PhysDescChild::Rubric(_) => "rubric",
            PhysDescChild::Signatures(_) => "signatures",
            PhysDescChild::Head(_) => "head",
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
            PhysDescChild::P(elem) => elem.serialize_mei(writer),
            PhysDescChild::History(elem) => elem.serialize_mei(writer),
            PhysDescChild::Incip(elem) => elem.serialize_mei(writer),
            PhysDescChild::Extent(elem) => elem.serialize_mei(writer),
            PhysDescChild::Head(elem) => elem.serialize_mei(writer),
            PhysDescChild::PlateNum(elem) => elem.serialize_mei(writer),
            // Elements not yet fully implemented - return error
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PhysDescChild::{}",
                other.element_name()
            ))),
        }
    }
}

// ============================================================================
// PlateNum
// ============================================================================

impl MeiSerialize for PlateNum {
    fn element_name(&self) -> &'static str {
        "plateNum"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for PlateNumChild {
    fn element_name(&self) -> &'static str {
        match self {
            PlateNumChild::Text(_) => "#text",
            PlateNumChild::Ptr(_) => "ptr",
            PlateNumChild::Stack(_) => "stack",
            PlateNumChild::Height(_) => "height",
            PlateNumChild::Lb(_) => "lb",
            PlateNumChild::Name(_) => "name",
            PlateNumChild::Num(_) => "num",
            PlateNumChild::PersName(_) => "persName",
            PlateNumChild::Head(_) => "head",
            PlateNumChild::Street(_) => "street",
            PlateNumChild::Region(_) => "region",
            PlateNumChild::PostBox(_) => "postBox",
            PlateNumChild::Q(_) => "q",
            PlateNumChild::GeogFeat(_) => "geogFeat",
            PlateNumChild::Expan(_) => "expan",
            PlateNumChild::Locus(_) => "locus",
            PlateNumChild::PostCode(_) => "postCode",
            PlateNumChild::Symbol(_) => "symbol",
            PlateNumChild::Abbr(_) => "abbr",
            PlateNumChild::PeriodName(_) => "periodName",
            PlateNumChild::P(_) => "p",
            PlateNumChild::Stamp(_) => "stamp",
            PlateNumChild::Width(_) => "width",
            PlateNumChild::Seg(_) => "seg",
            PlateNumChild::Address(_) => "address",
            PlateNumChild::Identifier(_) => "identifier",
            PlateNumChild::LocusGrp(_) => "locusGrp",
            PlateNumChild::Ref(_) => "ref",
            PlateNumChild::Extent(_) => "extent",
            PlateNumChild::Repository(_) => "repository",
            PlateNumChild::Signatures(_) => "signatures",
            PlateNumChild::Bibl(_) => "bibl",
            PlateNumChild::CorpName(_) => "corpName",
            PlateNumChild::Annot(_) => "annot",
            PlateNumChild::Country(_) => "country",
            PlateNumChild::District(_) => "district",
            PlateNumChild::Dimensions(_) => "dimensions",
            PlateNumChild::Bloc(_) => "bloc",
            PlateNumChild::Heraldry(_) => "heraldry",
            PlateNumChild::Rend(_) => "rend",
            PlateNumChild::BiblStruct(_) => "biblStruct",
            PlateNumChild::Date(_) => "date",
            PlateNumChild::Settlement(_) => "settlement",
            PlateNumChild::Title(_) => "title",
            PlateNumChild::Dim(_) => "dim",
            PlateNumChild::Catchwords(_) => "catchwords",
            PlateNumChild::Fig(_) => "fig",
            PlateNumChild::Relation(_) => "relation",
            PlateNumChild::Depth(_) => "depth",
            PlateNumChild::GeogName(_) => "geogName",
            PlateNumChild::StyleName(_) => "styleName",
            PlateNumChild::RelationList(_) => "relationList",
            PlateNumChild::Term(_) => "term",
            PlateNumChild::SecFolio(_) => "secFolio",
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
            PlateNumChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            PlateNumChild::Ptr(elem) => elem.serialize_mei(writer),
            PlateNumChild::Lb(elem) => elem.serialize_mei(writer),
            PlateNumChild::Name(elem) => elem.serialize_mei(writer),
            PlateNumChild::Num(elem) => elem.serialize_mei(writer),
            PlateNumChild::PersName(elem) => elem.serialize_mei(writer),
            PlateNumChild::Head(elem) => elem.serialize_mei(writer),
            PlateNumChild::Street(elem) => elem.serialize_mei(writer),
            PlateNumChild::Region(elem) => elem.serialize_mei(writer),
            PlateNumChild::PostBox(elem) => elem.serialize_mei(writer),
            PlateNumChild::GeogFeat(elem) => elem.serialize_mei(writer),
            PlateNumChild::Locus(elem) => elem.serialize_mei(writer),
            PlateNumChild::PostCode(elem) => elem.serialize_mei(writer),
            PlateNumChild::P(elem) => elem.serialize_mei(writer),
            PlateNumChild::Seg(elem) => elem.serialize_mei(writer),
            PlateNumChild::Address(elem) => elem.serialize_mei(writer),
            PlateNumChild::Identifier(elem) => elem.serialize_mei(writer),
            PlateNumChild::Ref(elem) => elem.serialize_mei(writer),
            PlateNumChild::Extent(elem) => elem.serialize_mei(writer),
            PlateNumChild::Bibl(elem) => elem.serialize_mei(writer),
            PlateNumChild::CorpName(elem) => elem.serialize_mei(writer),
            PlateNumChild::Annot(elem) => elem.serialize_mei(writer),
            PlateNumChild::Country(elem) => elem.serialize_mei(writer),
            PlateNumChild::District(elem) => elem.serialize_mei(writer),
            PlateNumChild::Bloc(elem) => elem.serialize_mei(writer),
            PlateNumChild::Rend(elem) => elem.serialize_mei(writer),
            PlateNumChild::Date(elem) => elem.serialize_mei(writer),
            PlateNumChild::Settlement(elem) => elem.serialize_mei(writer),
            PlateNumChild::Title(elem) => elem.serialize_mei(writer),
            PlateNumChild::GeogName(elem) => elem.serialize_mei(writer),
            PlateNumChild::Term(elem) => elem.serialize_mei(writer),
            // Elements not yet fully implemented - return error
            other => Err(crate::serializer::SerializeError::NotImplemented(format!(
                "PlateNumChild::{}",
                other.element_name()
            ))),
        }
    }
}

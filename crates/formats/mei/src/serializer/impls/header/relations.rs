//! Serializer implementations for relation and bibliographic item elements.
//!
//! Contains: Relation, RelationList, RelatedItem, Item, ItemList, ComponentList,
//! PhysLoc, Repository.

use super::super::to_attr_string;
use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    ComponentList, ComponentListChild, Item, ItemChild, ItemList, ItemListChild, PhysLoc,
    PhysLocChild, RelatedItem, RelatedItemChild, Relation, RelationList, RelationListChild,
    Repository, RepositoryChild,
};

// ============================================================================
// Relation
// ============================================================================

impl MeiSerialize for Relation {
    fn element_name(&self) -> &'static str {
        "relation"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
        attrs.extend(self.plist.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        if let Some(ref rel) = self.rel {
            if let Some(s) = to_attr_string(rel) {
                attrs.push(("rel", s));
            }
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
// RelationList
// ============================================================================

impl MeiSerialize for RelationList {
    fn element_name(&self) -> &'static str {
        "relationList"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        self.common.collect_attributes()
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

impl MeiSerialize for RelationListChild {
    fn element_name(&self) -> &'static str {
        match self {
            RelationListChild::Relation(_) => "relation",
            RelationListChild::RelationList(_) => "relationList",
            RelationListChild::Head(_) => "head",
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
            RelationListChild::Relation(elem) => elem.serialize_mei(writer),
            RelationListChild::RelationList(elem) => elem.serialize_mei(writer),
            RelationListChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// RelatedItem
// ============================================================================

impl MeiSerialize for RelatedItem {
    fn element_name(&self) -> &'static str {
        "relatedItem"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.target_eval.collect_attributes());
        if let Some(ref rel) = self.rel {
            if let Some(s) = to_attr_string(rel) {
                attrs.push(("rel", s));
            }
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

impl MeiSerialize for RelatedItemChild {
    fn element_name(&self) -> &'static str {
        match self {
            RelatedItemChild::BiblStruct(_) => "biblStruct",
            RelatedItemChild::Bibl(_) => "bibl",
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
            RelatedItemChild::BiblStruct(elem) => elem.serialize_mei(writer),
            RelatedItemChild::Bibl(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Item
// ============================================================================

impl MeiSerialize for Item {
    fn element_name(&self) -> &'static str {
        "item"
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

impl MeiSerialize for ItemChild {
    fn element_name(&self) -> &'static str {
        match self {
            ItemChild::PhysDesc(_) => "physDesc",
            ItemChild::History(_) => "history",
            ItemChild::Dedication(_) => "dedication",
            ItemChild::PhysLoc(_) => "physLoc",
            ItemChild::Head(_) => "head",
            ItemChild::NotesStmt(_) => "notesStmt",
            ItemChild::ComponentList(_) => "componentList",
            ItemChild::Identifier(_) => "identifier",
            ItemChild::Availability(_) => "availability",
            ItemChild::RelationList(_) => "relationList",
            ItemChild::ExtMeta(_) => "extMeta",
            ItemChild::Classification(_) => "classification",
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
            ItemChild::PhysDesc(elem) => elem.serialize_mei(writer),
            ItemChild::History(elem) => elem.serialize_mei(writer),
            ItemChild::Dedication(elem) => elem.serialize_mei(writer),
            ItemChild::PhysLoc(elem) => elem.serialize_mei(writer),
            ItemChild::Head(elem) => elem.serialize_mei(writer),
            ItemChild::NotesStmt(elem) => elem.serialize_mei(writer),
            ItemChild::ComponentList(elem) => elem.serialize_mei(writer),
            ItemChild::Identifier(elem) => elem.serialize_mei(writer),
            ItemChild::Availability(elem) => elem.serialize_mei(writer),
            ItemChild::RelationList(elem) => elem.serialize_mei(writer),
            ItemChild::ExtMeta(elem) => elem.serialize_mei(writer),
            ItemChild::Classification(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// ItemList
// ============================================================================

impl MeiSerialize for ItemList {
    fn element_name(&self) -> &'static str {
        "itemList"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        self.common.collect_attributes()
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

impl MeiSerialize for ItemListChild {
    fn element_name(&self) -> &'static str {
        match self {
            ItemListChild::Head(_) => "head",
            ItemListChild::Item(_) => "item",
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
            ItemListChild::Head(elem) => elem.serialize_mei(writer),
            ItemListChild::Item(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// ComponentList
// ============================================================================

impl MeiSerialize for ComponentList {
    fn element_name(&self) -> &'static str {
        "componentList"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        self.common.collect_attributes()
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

impl MeiSerialize for ComponentListChild {
    fn element_name(&self) -> &'static str {
        match self {
            ComponentListChild::Item(_) => "item",
            ComponentListChild::Work(_) => "work",
            ComponentListChild::Head(_) => "head",
            ComponentListChild::Expression(_) => "expression",
            ComponentListChild::Manifestation(_) => "manifestation",
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
            ComponentListChild::Item(elem) => elem.serialize_mei(writer),
            ComponentListChild::Work(elem) => elem.serialize_mei(writer),
            ComponentListChild::Head(elem) => elem.serialize_mei(writer),
            ComponentListChild::Expression(elem) => elem.serialize_mei(writer),
            ComponentListChild::Manifestation(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// PhysLoc
// ============================================================================

impl MeiSerialize for PhysLoc {
    fn element_name(&self) -> &'static str {
        "physLoc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
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

impl MeiSerialize for PhysLocChild {
    fn element_name(&self) -> &'static str {
        match self {
            PhysLocChild::Identifier(_) => "identifier",
            PhysLocChild::Head(_) => "head",
            PhysLocChild::History(_) => "history",
            PhysLocChild::Repository(_) => "repository",
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
            PhysLocChild::Identifier(elem) => elem.serialize_mei(writer),
            PhysLocChild::Head(elem) => elem.serialize_mei(writer),
            PhysLocChild::History(elem) => elem.serialize_mei(writer),
            PhysLocChild::Repository(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Repository
// ============================================================================

impl MeiSerialize for Repository {
    fn element_name(&self) -> &'static str {
        "repository"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for RepositoryChild {
    fn element_name(&self) -> &'static str {
        match self {
            RepositoryChild::Text(_) => "#text",
            RepositoryChild::Seg(_) => "seg",
            RepositoryChild::Catchwords(_) => "catchwords",
            RepositoryChild::Depth(_) => "depth",
            RepositoryChild::Address(_) => "address",
            RepositoryChild::Date(_) => "date",
            RepositoryChild::PeriodName(_) => "periodName",
            RepositoryChild::Expan(_) => "expan",
            RepositoryChild::Num(_) => "num",
            RepositoryChild::PostCode(_) => "postCode",
            RepositoryChild::Fig(_) => "fig",
            RepositoryChild::Width(_) => "width",
            RepositoryChild::BiblStruct(_) => "biblStruct",
            RepositoryChild::Extent(_) => "extent",
            RepositoryChild::RelationList(_) => "relationList",
            RepositoryChild::SecFolio(_) => "secFolio",
            RepositoryChild::Title(_) => "title",
            RepositoryChild::Height(_) => "height",
            RepositoryChild::Annot(_) => "annot",
            RepositoryChild::PersName(_) => "persName",
            RepositoryChild::Stamp(_) => "stamp",
            RepositoryChild::Settlement(_) => "settlement",
            RepositoryChild::Repository(_) => "repository",
            RepositoryChild::Region(_) => "region",
            RepositoryChild::Symbol(_) => "symbol",
            RepositoryChild::Rend(_) => "rend",
            RepositoryChild::Ptr(_) => "ptr",
            RepositoryChild::LocusGrp(_) => "locusGrp",
            RepositoryChild::Signatures(_) => "signatures",
            RepositoryChild::Q(_) => "q",
            RepositoryChild::Street(_) => "street",
            RepositoryChild::Identifier(_) => "identifier",
            RepositoryChild::StyleName(_) => "styleName",
            RepositoryChild::Abbr(_) => "abbr",
            RepositoryChild::Locus(_) => "locus",
            RepositoryChild::Name(_) => "name",
            RepositoryChild::Dim(_) => "dim",
            RepositoryChild::GeogName(_) => "geogName",
            RepositoryChild::Heraldry(_) => "heraldry",
            RepositoryChild::Lb(_) => "lb",
            RepositoryChild::District(_) => "district",
            RepositoryChild::CorpName(_) => "corpName",
            RepositoryChild::Ref(_) => "ref",
            RepositoryChild::Stack(_) => "stack",
            RepositoryChild::Term(_) => "term",
            RepositoryChild::Relation(_) => "relation",
            RepositoryChild::Bibl(_) => "bibl",
            RepositoryChild::GeogFeat(_) => "geogFeat",
            RepositoryChild::PostBox(_) => "postBox",
            RepositoryChild::Dimensions(_) => "dimensions",
            RepositoryChild::Bloc(_) => "bloc",
            RepositoryChild::Country(_) => "country",
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
            RepositoryChild::Text(text) => writer.write_text(text),
            RepositoryChild::Seg(elem) => elem.serialize_mei(writer),
            // Phase 14 element - stub
            RepositoryChild::Catchwords(_) => Ok(()),
            RepositoryChild::Depth(elem) => elem.serialize_mei(writer),
            RepositoryChild::Address(elem) => elem.serialize_mei(writer),
            RepositoryChild::Date(elem) => elem.serialize_mei(writer),
            RepositoryChild::PeriodName(elem) => elem.serialize_mei(writer),
            RepositoryChild::Expan(elem) => elem.serialize_mei(writer),
            RepositoryChild::Num(elem) => elem.serialize_mei(writer),
            RepositoryChild::PostCode(elem) => elem.serialize_mei(writer),
            RepositoryChild::Fig(elem) => elem.serialize_mei(writer),
            RepositoryChild::Width(elem) => elem.serialize_mei(writer),
            RepositoryChild::BiblStruct(elem) => elem.serialize_mei(writer),
            RepositoryChild::Extent(elem) => elem.serialize_mei(writer),
            RepositoryChild::RelationList(elem) => elem.serialize_mei(writer),
            // Phase 14 element - stub
            RepositoryChild::SecFolio(_) => Ok(()),
            RepositoryChild::Title(elem) => elem.serialize_mei(writer),
            RepositoryChild::Height(elem) => elem.serialize_mei(writer),
            RepositoryChild::Annot(elem) => elem.serialize_mei(writer),
            RepositoryChild::PersName(elem) => elem.serialize_mei(writer),
            RepositoryChild::Stamp(elem) => elem.serialize_mei(writer),
            RepositoryChild::Settlement(elem) => elem.serialize_mei(writer),
            RepositoryChild::Repository(elem) => elem.serialize_mei(writer),
            RepositoryChild::Region(elem) => elem.serialize_mei(writer),
            RepositoryChild::Symbol(elem) => elem.serialize_mei(writer),
            RepositoryChild::Rend(elem) => elem.serialize_mei(writer),
            RepositoryChild::Ptr(elem) => elem.serialize_mei(writer),
            RepositoryChild::LocusGrp(elem) => elem.serialize_mei(writer),
            // Phase 14 element - stub
            RepositoryChild::Signatures(_) => Ok(()),
            RepositoryChild::Q(elem) => elem.serialize_mei(writer),
            RepositoryChild::Street(elem) => elem.serialize_mei(writer),
            RepositoryChild::Identifier(elem) => elem.serialize_mei(writer),
            RepositoryChild::StyleName(elem) => elem.serialize_mei(writer),
            RepositoryChild::Abbr(elem) => elem.serialize_mei(writer),
            RepositoryChild::Locus(elem) => elem.serialize_mei(writer),
            RepositoryChild::Name(elem) => elem.serialize_mei(writer),
            RepositoryChild::Dim(elem) => elem.serialize_mei(writer),
            RepositoryChild::GeogName(elem) => elem.serialize_mei(writer),
            // Phase 14 element - stub
            RepositoryChild::Heraldry(_) => Ok(()),
            RepositoryChild::Lb(elem) => elem.serialize_mei(writer),
            RepositoryChild::District(elem) => elem.serialize_mei(writer),
            RepositoryChild::CorpName(elem) => elem.serialize_mei(writer),
            RepositoryChild::Ref(elem) => elem.serialize_mei(writer),
            RepositoryChild::Stack(elem) => elem.serialize_mei(writer),
            RepositoryChild::Term(elem) => elem.serialize_mei(writer),
            RepositoryChild::Relation(elem) => elem.serialize_mei(writer),
            RepositoryChild::Bibl(elem) => elem.serialize_mei(writer),
            RepositoryChild::GeogFeat(elem) => elem.serialize_mei(writer),
            RepositoryChild::PostBox(elem) => elem.serialize_mei(writer),
            RepositoryChild::Dimensions(elem) => elem.serialize_mei(writer),
            RepositoryChild::Bloc(elem) => elem.serialize_mei(writer),
            RepositoryChild::Country(elem) => elem.serialize_mei(writer),
        }
    }
}

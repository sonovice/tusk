//! Serializer implementations for bibliographic and codicological MEI elements.
//!
//! This module contains implementations for:
//! - ExtData, AvFile, Cutout, Bifolium, Folium, Patch (codicology)
//! - Analytic, Monogr, Series, Desc (bibliography)
//! - Catchwords, Signatures, SignifLet (manuscript description)
//! - Actor, CatRel, Context (misc elements)

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{
    AttBifoliumSurfaces, AttFoliumSurfaces, AttSignifLetAnl, AttSignifLetGes, AttSignifLetLog,
    AttSignifLetVis,
};
use tusk_model::elements::{
    Actor, ActorChild, Analytic, AnalyticChild, AvFile, AvFileChild, Bifolium, BifoliumChild,
    CatRel, CatRelChild, Catchwords, CatchwordsChild, Context, ContextChild, Cutout, CutoutChild,
    ExtData, ExtDataChild, Folium, FoliumChild, Monogr, MonogrChild, Patch, PatchChild, Series,
    SeriesChild, Signatures, SignaturesChild, SignifLet, SignifLetChild,
};

use super::push_attr;

// ============================================================================
// Attribute class implementations
// ============================================================================

impl CollectAttributes for AttBifoliumSurfaces {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "outer.recto", self.outer_recto);
        push_attr!(attrs, "inner.verso", self.inner_verso);
        push_attr!(attrs, "inner.recto", self.inner_recto);
        push_attr!(attrs, "outer.verso", self.outer_verso);
        attrs
    }
}

impl CollectAttributes for AttFoliumSurfaces {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        push_attr!(attrs, "recto", self.recto);
        push_attr!(attrs, "verso", self.verso);
        attrs
    }
}

// Note: AttSource is implemented in misc.rs

// ============================================================================
// ExtData element
// ============================================================================

impl MeiSerialize for ExtData {
    fn element_name(&self) -> &'static str {
        "extData"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        attrs.extend(self.typed.collect_attributes());
        attrs.extend(self.whitespace.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
        attrs.extend(self.internet_media.collect_attributes());
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

impl MeiSerialize for ExtDataChild {
    fn element_name(&self) -> &'static str {
        match self {
            ExtDataChild::Text(_) => "#text",
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
            ExtDataChild::Text(text) => writer.write_text(text),
        }
    }
}

// ============================================================================
// AvFile element
// ============================================================================

impl MeiSerialize for AvFile {
    fn element_name(&self) -> &'static str {
        "avFile"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.internet_media.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.metadata_pointing.collect_attributes());
        attrs.extend(self.pointing.collect_attributes());
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

impl MeiSerialize for AvFileChild {
    fn element_name(&self) -> &'static str {
        match self {
            AvFileChild::Clip(_) => "clip",
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
            AvFileChild::Clip(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Patch element
// ============================================================================

impl MeiSerialize for Patch {
    fn element_name(&self) -> &'static str {
        "patch"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
        attrs.extend(self.measurement.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
        if let Some(ref v) = self.attached_to {
            attrs.push(("attached.to", v.clone()));
        }
        if let Some(ref v) = self.attached_by {
            attrs.push(("attached.by", v.clone()));
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

impl MeiSerialize for PatchChild {
    fn element_name(&self) -> &'static str {
        match self {
            PatchChild::Bifolium(_) => "bifolium",
            PatchChild::Folium(_) => "folium",
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
            PatchChild::Bifolium(elem) => elem.serialize_mei(writer),
            PatchChild::Folium(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Cutout element
// ============================================================================

impl MeiSerialize for Cutout {
    fn element_name(&self) -> &'static str {
        "cutout"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.dimensions.collect_attributes());
        attrs.extend(self.evidence.collect_attributes());
        attrs.extend(self.measurement.collect_attributes());
        attrs.extend(self.trans.collect_attributes());
        attrs.extend(self.xy.collect_attributes());
        if let Some(ref v) = self.removed_from {
            attrs.push(("removed.from", v.clone()));
        }
        if let Some(ref v) = self.removed_by {
            attrs.push(("removed.by", v.clone()));
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

impl MeiSerialize for CutoutChild {
    fn element_name(&self) -> &'static str {
        match self {
            CutoutChild::Bifolium(_) => "bifolium",
            CutoutChild::Folium(_) => "folium",
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
            CutoutChild::Bifolium(elem) => elem.serialize_mei(writer),
            CutoutChild::Folium(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Folium element
// ============================================================================

impl MeiSerialize for Folium {
    fn element_name(&self) -> &'static str {
        "folium"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.dimensions.collect_attributes());
        attrs.extend(self.measurement.collect_attributes());
        attrs.extend(self.folium_surfaces.collect_attributes());
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

impl MeiSerialize for FoliumChild {
    fn element_name(&self) -> &'static str {
        match self {
            FoliumChild::Cutout(_) => "cutout",
            FoliumChild::Patch(_) => "patch",
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
            FoliumChild::Cutout(elem) => elem.serialize_mei(writer),
            FoliumChild::Patch(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Bifolium element
// ============================================================================

impl MeiSerialize for Bifolium {
    fn element_name(&self) -> &'static str {
        "bifolium"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.dimensions.collect_attributes());
        attrs.extend(self.measurement.collect_attributes());
        attrs.extend(self.bifolium_surfaces.collect_attributes());
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

impl MeiSerialize for BifoliumChild {
    fn element_name(&self) -> &'static str {
        match self {
            BifoliumChild::Restore(_) => "restore",
            BifoliumChild::Damage(_) => "damage",
            BifoliumChild::Bifolium(_) => "bifolium",
            BifoliumChild::Patch(_) => "patch",
            BifoliumChild::Del(_) => "del",
            BifoliumChild::Folium(_) => "folium",
            BifoliumChild::Add(_) => "add",
            BifoliumChild::Gap(_) => "gap",
            BifoliumChild::Cutout(_) => "cutout",
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
            BifoliumChild::Restore(elem) => elem.serialize_mei(writer),
            BifoliumChild::Damage(elem) => elem.serialize_mei(writer),
            BifoliumChild::Bifolium(elem) => elem.serialize_mei(writer),
            BifoliumChild::Patch(elem) => elem.serialize_mei(writer),
            BifoliumChild::Del(elem) => elem.serialize_mei(writer),
            BifoliumChild::Folium(elem) => elem.serialize_mei(writer),
            BifoliumChild::Add(elem) => elem.serialize_mei(writer),
            BifoliumChild::Gap(elem) => elem.serialize_mei(writer),
            BifoliumChild::Cutout(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Analytic element
// ============================================================================

impl MeiSerialize for Analytic {
    fn element_name(&self) -> &'static str {
        "analytic"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
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

impl MeiSerialize for AnalyticChild {
    fn element_name(&self) -> &'static str {
        match self {
            AnalyticChild::Contributor(_) => "contributor",
            AnalyticChild::Identifier(_) => "identifier",
            AnalyticChild::Title(_) => "title",
            AnalyticChild::Funder(_) => "funder",
            AnalyticChild::Editor(_) => "editor",
            AnalyticChild::RespStmt(_) => "respStmt",
            AnalyticChild::BiblScope(_) => "biblScope",
            AnalyticChild::Sponsor(_) => "sponsor",
            AnalyticChild::Creator(_) => "creator",
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
            AnalyticChild::Contributor(elem) => elem.serialize_mei(writer),
            AnalyticChild::Identifier(elem) => elem.serialize_mei(writer),
            AnalyticChild::Title(elem) => elem.serialize_mei(writer),
            AnalyticChild::Funder(elem) => elem.serialize_mei(writer),
            AnalyticChild::Editor(elem) => elem.serialize_mei(writer),
            AnalyticChild::RespStmt(elem) => elem.serialize_mei(writer),
            AnalyticChild::BiblScope(elem) => elem.serialize_mei(writer),
            AnalyticChild::Sponsor(elem) => elem.serialize_mei(writer),
            AnalyticChild::Creator(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Monogr element
// ============================================================================

impl MeiSerialize for Monogr {
    fn element_name(&self) -> &'static str {
        "monogr"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for MonogrChild {
    fn element_name(&self) -> &'static str {
        match self {
            MonogrChild::Editor(_) => "editor",
            MonogrChild::CorpName(_) => "corpName",
            MonogrChild::Identifier(_) => "identifier",
            MonogrChild::Funder(_) => "funder",
            MonogrChild::Sponsor(_) => "sponsor",
            MonogrChild::Title(_) => "title",
            MonogrChild::Extent(_) => "extent",
            MonogrChild::Annot(_) => "annot",
            MonogrChild::Creator(_) => "creator",
            MonogrChild::Edition(_) => "edition",
            MonogrChild::Imprint(_) => "imprint",
            MonogrChild::Contributor(_) => "contributor",
            MonogrChild::RespStmt(_) => "respStmt",
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
            MonogrChild::Editor(elem) => elem.serialize_mei(writer),
            MonogrChild::CorpName(elem) => elem.serialize_mei(writer),
            MonogrChild::Identifier(elem) => elem.serialize_mei(writer),
            MonogrChild::Funder(elem) => elem.serialize_mei(writer),
            MonogrChild::Sponsor(elem) => elem.serialize_mei(writer),
            MonogrChild::Title(elem) => elem.serialize_mei(writer),
            MonogrChild::Extent(elem) => elem.serialize_mei(writer),
            MonogrChild::Annot(elem) => elem.serialize_mei(writer),
            MonogrChild::Creator(elem) => elem.serialize_mei(writer),
            MonogrChild::Edition(elem) => elem.serialize_mei(writer),
            MonogrChild::Imprint(elem) => elem.serialize_mei(writer),
            MonogrChild::Contributor(elem) => elem.serialize_mei(writer),
            MonogrChild::RespStmt(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Series element
// ============================================================================

impl MeiSerialize for Series {
    fn element_name(&self) -> &'static str {
        "series"
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

impl MeiSerialize for SeriesChild {
    fn element_name(&self) -> &'static str {
        match self {
            SeriesChild::Text(_) => "#text",
            SeriesChild::ColLayout(_) => "colLayout",
            SeriesChild::RespStmt(_) => "respStmt",
            SeriesChild::Ref(_) => "ref",
            SeriesChild::Identifier(_) => "identifier",
            SeriesChild::Editor(_) => "editor",
            SeriesChild::TextLang(_) => "textLang",
            SeriesChild::Lb(_) => "lb",
            SeriesChild::Pb(_) => "pb",
            SeriesChild::Title(_) => "title",
            SeriesChild::Cb(_) => "cb",
            SeriesChild::Extent(_) => "extent",
            SeriesChild::Ptr(_) => "ptr",
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
            SeriesChild::Text(text) => writer.write_text(text),
            SeriesChild::ColLayout(elem) => elem.serialize_mei(writer),
            SeriesChild::RespStmt(elem) => elem.serialize_mei(writer),
            SeriesChild::Ref(elem) => elem.serialize_mei(writer),
            SeriesChild::Identifier(elem) => elem.serialize_mei(writer),
            SeriesChild::Editor(elem) => elem.serialize_mei(writer),
            SeriesChild::TextLang(elem) => elem.serialize_mei(writer),
            SeriesChild::Lb(elem) => elem.serialize_mei(writer),
            SeriesChild::Pb(elem) => elem.serialize_mei(writer),
            SeriesChild::Title(elem) => elem.serialize_mei(writer),
            SeriesChild::Cb(elem) => elem.serialize_mei(writer),
            SeriesChild::Extent(elem) => elem.serialize_mei(writer),
            SeriesChild::Ptr(elem) => elem.serialize_mei(writer),
        }
    }
}

// Note: Desc and DescChild implementations are in header/encoding_desc.rs

// ============================================================================
// Catchwords element
// ============================================================================

impl MeiSerialize for Catchwords {
    fn element_name(&self) -> &'static str {
        "catchwords"
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

impl MeiSerialize for CatchwordsChild {
    fn element_name(&self) -> &'static str {
        match self {
            CatchwordsChild::Text(_) => "#text",
            CatchwordsChild::Head(_) => "head",
            CatchwordsChild::Country(_) => "country",
            CatchwordsChild::Identifier(_) => "identifier",
            CatchwordsChild::Catchwords(_) => "catchwords",
            CatchwordsChild::District(_) => "district",
            CatchwordsChild::Abbr(_) => "abbr",
            CatchwordsChild::Date(_) => "date",
            CatchwordsChild::Extent(_) => "extent",
            CatchwordsChild::Locus(_) => "locus",
            CatchwordsChild::LocusGrp(_) => "locusGrp",
            CatchwordsChild::Num(_) => "num",
            CatchwordsChild::PeriodName(_) => "periodName",
            CatchwordsChild::PersName(_) => "persName",
            CatchwordsChild::Ref(_) => "ref",
            CatchwordsChild::SecFolio(_) => "secFolio",
            CatchwordsChild::Settlement(_) => "settlement",
            CatchwordsChild::Stamp(_) => "stamp",
            CatchwordsChild::StyleName(_) => "styleName",
            CatchwordsChild::Q(_) => "q",
            CatchwordsChild::Seg(_) => "seg",
            CatchwordsChild::Annot(_) => "annot",
            CatchwordsChild::Fig(_) => "fig",
            CatchwordsChild::CorpName(_) => "corpName",
            CatchwordsChild::Bloc(_) => "bloc",
            CatchwordsChild::Signatures(_) => "signatures",
            CatchwordsChild::Lb(_) => "lb",
            CatchwordsChild::Address(_) => "address",
            CatchwordsChild::Name(_) => "name",
            CatchwordsChild::PostCode(_) => "postCode",
            CatchwordsChild::RelationList(_) => "relationList",
            CatchwordsChild::P(_) => "p",
            CatchwordsChild::PostBox(_) => "postBox",
            CatchwordsChild::GeogFeat(_) => "geogFeat",
            CatchwordsChild::Ptr(_) => "ptr",
            CatchwordsChild::Dimensions(_) => "dimensions",
            CatchwordsChild::Expan(_) => "expan",
            CatchwordsChild::Rend(_) => "rend",
            CatchwordsChild::Region(_) => "region",
            CatchwordsChild::Repository(_) => "repository",
            CatchwordsChild::Street(_) => "street",
            CatchwordsChild::Stack(_) => "stack",
            CatchwordsChild::Title(_) => "title",
            CatchwordsChild::Term(_) => "term",
            CatchwordsChild::GeogName(_) => "geogName",
            CatchwordsChild::Dim(_) => "dim",
            CatchwordsChild::Heraldry(_) => "heraldry",
            CatchwordsChild::Height(_) => "height",
            CatchwordsChild::Relation(_) => "relation",
            CatchwordsChild::Symbol(_) => "symbol",
            CatchwordsChild::Depth(_) => "depth",
            CatchwordsChild::Width(_) => "width",
            CatchwordsChild::BiblStruct(_) => "biblStruct",
            CatchwordsChild::Bibl(_) => "bibl",
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
            CatchwordsChild::Text(text) => writer.write_text(text),
            CatchwordsChild::Head(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Country(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Identifier(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Catchwords(elem) => elem.serialize_mei(writer),
            CatchwordsChild::District(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Abbr(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Date(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Extent(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Locus(elem) => elem.serialize_mei(writer),
            CatchwordsChild::LocusGrp(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Num(elem) => elem.serialize_mei(writer),
            CatchwordsChild::PeriodName(elem) => elem.serialize_mei(writer),
            CatchwordsChild::PersName(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Ref(elem) => elem.serialize_mei(writer),
            CatchwordsChild::SecFolio(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Settlement(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Stamp(elem) => elem.serialize_mei(writer),
            CatchwordsChild::StyleName(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Q(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Seg(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Annot(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Fig(elem) => elem.serialize_mei(writer),
            CatchwordsChild::CorpName(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Bloc(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Signatures(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Lb(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Address(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Name(elem) => elem.serialize_mei(writer),
            CatchwordsChild::PostCode(elem) => elem.serialize_mei(writer),
            CatchwordsChild::RelationList(elem) => elem.serialize_mei(writer),
            CatchwordsChild::P(elem) => elem.serialize_mei(writer),
            CatchwordsChild::PostBox(elem) => elem.serialize_mei(writer),
            CatchwordsChild::GeogFeat(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Ptr(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Dimensions(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Expan(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Rend(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Region(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Repository(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Street(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Stack(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Title(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Term(elem) => elem.serialize_mei(writer),
            CatchwordsChild::GeogName(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Dim(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Heraldry(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Height(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Relation(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Symbol(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Depth(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Width(elem) => elem.serialize_mei(writer),
            CatchwordsChild::BiblStruct(elem) => elem.serialize_mei(writer),
            CatchwordsChild::Bibl(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Signatures element
// ============================================================================

impl MeiSerialize for Signatures {
    fn element_name(&self) -> &'static str {
        "signatures"
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

impl MeiSerialize for SignaturesChild {
    fn element_name(&self) -> &'static str {
        match self {
            SignaturesChild::Text(_) => "#text",
            SignaturesChild::PostCode(_) => "postCode",
            SignaturesChild::Seg(_) => "seg",
            SignaturesChild::SecFolio(_) => "secFolio",
            SignaturesChild::Signatures(_) => "signatures",
            SignaturesChild::Annot(_) => "annot",
            SignaturesChild::Stamp(_) => "stamp",
            SignaturesChild::LocusGrp(_) => "locusGrp",
            SignaturesChild::Settlement(_) => "settlement",
            SignaturesChild::PersName(_) => "persName",
            SignaturesChild::Country(_) => "country",
            SignaturesChild::P(_) => "p",
            SignaturesChild::Address(_) => "address",
            SignaturesChild::PostBox(_) => "postBox",
            SignaturesChild::Ptr(_) => "ptr",
            SignaturesChild::Symbol(_) => "symbol",
            SignaturesChild::Title(_) => "title",
            SignaturesChild::Num(_) => "num",
            SignaturesChild::Width(_) => "width",
            SignaturesChild::Bloc(_) => "bloc",
            SignaturesChild::Bibl(_) => "bibl",
            SignaturesChild::Term(_) => "term",
            SignaturesChild::CorpName(_) => "corpName",
            SignaturesChild::Abbr(_) => "abbr",
            SignaturesChild::Dimensions(_) => "dimensions",
            SignaturesChild::Height(_) => "height",
            SignaturesChild::Heraldry(_) => "heraldry",
            SignaturesChild::Depth(_) => "depth",
            SignaturesChild::Ref(_) => "ref",
            SignaturesChild::Head(_) => "head",
            SignaturesChild::Region(_) => "region",
            SignaturesChild::GeogFeat(_) => "geogFeat",
            SignaturesChild::Repository(_) => "repository",
            SignaturesChild::Dim(_) => "dim",
            SignaturesChild::RelationList(_) => "relationList",
            SignaturesChild::Extent(_) => "extent",
            SignaturesChild::Lb(_) => "lb",
            SignaturesChild::Q(_) => "q",
            SignaturesChild::Identifier(_) => "identifier",
            SignaturesChild::Stack(_) => "stack",
            SignaturesChild::Date(_) => "date",
            SignaturesChild::GeogName(_) => "geogName",
            SignaturesChild::Catchwords(_) => "catchwords",
            SignaturesChild::Relation(_) => "relation",
            SignaturesChild::Street(_) => "street",
            SignaturesChild::BiblStruct(_) => "biblStruct",
            SignaturesChild::District(_) => "district",
            SignaturesChild::Locus(_) => "locus",
            SignaturesChild::Expan(_) => "expan",
            SignaturesChild::Name(_) => "name",
            SignaturesChild::Fig(_) => "fig",
            SignaturesChild::Rend(_) => "rend",
            SignaturesChild::PeriodName(_) => "periodName",
            SignaturesChild::StyleName(_) => "styleName",
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
            SignaturesChild::Text(text) => writer.write_text(text),
            SignaturesChild::PostCode(elem) => elem.serialize_mei(writer),
            SignaturesChild::Seg(elem) => elem.serialize_mei(writer),
            SignaturesChild::SecFolio(elem) => elem.serialize_mei(writer),
            SignaturesChild::Signatures(elem) => elem.serialize_mei(writer),
            SignaturesChild::Annot(elem) => elem.serialize_mei(writer),
            SignaturesChild::Stamp(elem) => elem.serialize_mei(writer),
            SignaturesChild::LocusGrp(elem) => elem.serialize_mei(writer),
            SignaturesChild::Settlement(elem) => elem.serialize_mei(writer),
            SignaturesChild::PersName(elem) => elem.serialize_mei(writer),
            SignaturesChild::Country(elem) => elem.serialize_mei(writer),
            SignaturesChild::P(elem) => elem.serialize_mei(writer),
            SignaturesChild::Address(elem) => elem.serialize_mei(writer),
            SignaturesChild::PostBox(elem) => elem.serialize_mei(writer),
            SignaturesChild::Ptr(elem) => elem.serialize_mei(writer),
            SignaturesChild::Symbol(elem) => elem.serialize_mei(writer),
            SignaturesChild::Title(elem) => elem.serialize_mei(writer),
            SignaturesChild::Num(elem) => elem.serialize_mei(writer),
            SignaturesChild::Width(elem) => elem.serialize_mei(writer),
            SignaturesChild::Bloc(elem) => elem.serialize_mei(writer),
            SignaturesChild::Bibl(elem) => elem.serialize_mei(writer),
            SignaturesChild::Term(elem) => elem.serialize_mei(writer),
            SignaturesChild::CorpName(elem) => elem.serialize_mei(writer),
            SignaturesChild::Abbr(elem) => elem.serialize_mei(writer),
            SignaturesChild::Dimensions(elem) => elem.serialize_mei(writer),
            SignaturesChild::Height(elem) => elem.serialize_mei(writer),
            SignaturesChild::Heraldry(elem) => elem.serialize_mei(writer),
            SignaturesChild::Depth(elem) => elem.serialize_mei(writer),
            SignaturesChild::Ref(elem) => elem.serialize_mei(writer),
            SignaturesChild::Head(elem) => elem.serialize_mei(writer),
            SignaturesChild::Region(elem) => elem.serialize_mei(writer),
            SignaturesChild::GeogFeat(elem) => elem.serialize_mei(writer),
            SignaturesChild::Repository(elem) => elem.serialize_mei(writer),
            SignaturesChild::Dim(elem) => elem.serialize_mei(writer),
            SignaturesChild::RelationList(elem) => elem.serialize_mei(writer),
            SignaturesChild::Extent(elem) => elem.serialize_mei(writer),
            SignaturesChild::Lb(elem) => elem.serialize_mei(writer),
            SignaturesChild::Q(elem) => elem.serialize_mei(writer),
            SignaturesChild::Identifier(elem) => elem.serialize_mei(writer),
            SignaturesChild::Stack(elem) => elem.serialize_mei(writer),
            SignaturesChild::Date(elem) => elem.serialize_mei(writer),
            SignaturesChild::GeogName(elem) => elem.serialize_mei(writer),
            SignaturesChild::Catchwords(elem) => elem.serialize_mei(writer),
            SignaturesChild::Relation(elem) => elem.serialize_mei(writer),
            SignaturesChild::Street(elem) => elem.serialize_mei(writer),
            SignaturesChild::BiblStruct(elem) => elem.serialize_mei(writer),
            SignaturesChild::District(elem) => elem.serialize_mei(writer),
            SignaturesChild::Locus(elem) => elem.serialize_mei(writer),
            SignaturesChild::Expan(elem) => elem.serialize_mei(writer),
            SignaturesChild::Name(elem) => elem.serialize_mei(writer),
            SignaturesChild::Fig(elem) => elem.serialize_mei(writer),
            SignaturesChild::Rend(elem) => elem.serialize_mei(writer),
            SignaturesChild::PeriodName(elem) => elem.serialize_mei(writer),
            SignaturesChild::StyleName(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// SignifLet element
// ============================================================================

impl CollectAttributes for AttSignifLetAnl {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttSignifLetGes {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttSignifLetLog {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl CollectAttributes for AttSignifLetVis {
    fn collect_attributes(&self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

impl MeiSerialize for SignifLet {
    fn element_name(&self) -> &'static str {
        "signifLet"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.signif_let_anl.collect_attributes());
        attrs.extend(self.signif_let_ges.collect_attributes());
        attrs.extend(self.signif_let_log.collect_attributes());
        attrs.extend(self.signif_let_vis.collect_attributes());
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

impl MeiSerialize for SignifLetChild {
    fn element_name(&self) -> &'static str {
        match self {
            SignifLetChild::Text(_) => "#text",
            SignifLetChild::Stack(_) => "stack",
            SignifLetChild::District(_) => "district",
            SignifLetChild::Relation(_) => "relation",
            SignifLetChild::Bibl(_) => "bibl",
            SignifLetChild::Add(_) => "add",
            SignifLetChild::Sic(_) => "sic",
            SignifLetChild::Stamp(_) => "stamp",
            SignifLetChild::PostBox(_) => "postBox",
            SignifLetChild::Subst(_) => "subst",
            SignifLetChild::Fig(_) => "fig",
            SignifLetChild::Catchwords(_) => "catchwords",
            SignifLetChild::Q(_) => "q",
            SignifLetChild::Del(_) => "del",
            SignifLetChild::Lb(_) => "lb",
            SignifLetChild::Curve(_) => "curve",
            SignifLetChild::Width(_) => "width",
            SignifLetChild::LocusGrp(_) => "locusGrp",
            SignifLetChild::Extent(_) => "extent",
            SignifLetChild::Seg(_) => "seg",
            SignifLetChild::StyleName(_) => "styleName",
            SignifLetChild::Gap(_) => "gap",
            SignifLetChild::Dimensions(_) => "dimensions",
            SignifLetChild::Date(_) => "date",
            SignifLetChild::GeogFeat(_) => "geogFeat",
            SignifLetChild::Signatures(_) => "signatures",
            SignifLetChild::Corr(_) => "corr",
            SignifLetChild::Term(_) => "term",
            SignifLetChild::Settlement(_) => "settlement",
            SignifLetChild::Street(_) => "street",
            SignifLetChild::CorpName(_) => "corpName",
            SignifLetChild::Damage(_) => "damage",
            SignifLetChild::Unclear(_) => "unclear",
            SignifLetChild::Identifier(_) => "identifier",
            SignifLetChild::Repository(_) => "repository",
            SignifLetChild::Dim(_) => "dim",
            SignifLetChild::Restore(_) => "restore",
            SignifLetChild::Ptr(_) => "ptr",
            SignifLetChild::Reg(_) => "reg",
            SignifLetChild::AnchoredText(_) => "anchoredText",
            SignifLetChild::Heraldry(_) => "heraldry",
            SignifLetChild::BiblStruct(_) => "biblStruct",
            SignifLetChild::Supplied(_) => "supplied",
            SignifLetChild::Address(_) => "address",
            SignifLetChild::SecFolio(_) => "secFolio",
            SignifLetChild::PostCode(_) => "postCode",
            SignifLetChild::Num(_) => "num",
            SignifLetChild::PersName(_) => "persName",
            SignifLetChild::Choice(_) => "choice",
            SignifLetChild::Bloc(_) => "bloc",
            SignifLetChild::Rend(_) => "rend",
            SignifLetChild::Abbr(_) => "abbr",
            SignifLetChild::Depth(_) => "depth",
            SignifLetChild::Name(_) => "name",
            SignifLetChild::Country(_) => "country",
            SignifLetChild::Expan(_) => "expan",
            SignifLetChild::Orig(_) => "orig",
            SignifLetChild::Height(_) => "height",
            SignifLetChild::RelationList(_) => "relationList",
            SignifLetChild::Locus(_) => "locus",
            SignifLetChild::Line(_) => "line",
            SignifLetChild::PeriodName(_) => "periodName",
            SignifLetChild::Annot(_) => "annot",
            SignifLetChild::Ref(_) => "ref",
            SignifLetChild::Title(_) => "title",
            SignifLetChild::GeogName(_) => "geogName",
            SignifLetChild::HandShift(_) => "handShift",
            SignifLetChild::Region(_) => "region",
            SignifLetChild::Symbol(_) => "symbol",
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
            SignifLetChild::Text(text) => writer.write_text(text),
            SignifLetChild::Stack(elem) => elem.serialize_mei(writer),
            SignifLetChild::District(elem) => elem.serialize_mei(writer),
            SignifLetChild::Relation(elem) => elem.serialize_mei(writer),
            SignifLetChild::Bibl(elem) => elem.serialize_mei(writer),
            SignifLetChild::Add(elem) => elem.serialize_mei(writer),
            SignifLetChild::Sic(elem) => elem.serialize_mei(writer),
            SignifLetChild::Stamp(elem) => elem.serialize_mei(writer),
            SignifLetChild::PostBox(elem) => elem.serialize_mei(writer),
            SignifLetChild::Subst(elem) => elem.serialize_mei(writer),
            SignifLetChild::Fig(elem) => elem.serialize_mei(writer),
            SignifLetChild::Catchwords(elem) => elem.serialize_mei(writer),
            SignifLetChild::Q(elem) => elem.serialize_mei(writer),
            SignifLetChild::Del(elem) => elem.serialize_mei(writer),
            SignifLetChild::Lb(elem) => elem.serialize_mei(writer),
            SignifLetChild::Curve(elem) => elem.serialize_mei(writer),
            SignifLetChild::Width(elem) => elem.serialize_mei(writer),
            SignifLetChild::LocusGrp(elem) => elem.serialize_mei(writer),
            SignifLetChild::Extent(elem) => elem.serialize_mei(writer),
            SignifLetChild::Seg(elem) => elem.serialize_mei(writer),
            SignifLetChild::StyleName(elem) => elem.serialize_mei(writer),
            SignifLetChild::Gap(elem) => elem.serialize_mei(writer),
            SignifLetChild::Dimensions(elem) => elem.serialize_mei(writer),
            SignifLetChild::Date(elem) => elem.serialize_mei(writer),
            SignifLetChild::GeogFeat(elem) => elem.serialize_mei(writer),
            SignifLetChild::Signatures(elem) => elem.serialize_mei(writer),
            SignifLetChild::Corr(elem) => elem.serialize_mei(writer),
            SignifLetChild::Term(elem) => elem.serialize_mei(writer),
            SignifLetChild::Settlement(elem) => elem.serialize_mei(writer),
            SignifLetChild::Street(elem) => elem.serialize_mei(writer),
            SignifLetChild::CorpName(elem) => elem.serialize_mei(writer),
            SignifLetChild::Damage(elem) => elem.serialize_mei(writer),
            SignifLetChild::Unclear(elem) => elem.serialize_mei(writer),
            SignifLetChild::Identifier(elem) => elem.serialize_mei(writer),
            SignifLetChild::Repository(elem) => elem.serialize_mei(writer),
            SignifLetChild::Dim(elem) => elem.serialize_mei(writer),
            SignifLetChild::Restore(elem) => elem.serialize_mei(writer),
            SignifLetChild::Ptr(elem) => elem.serialize_mei(writer),
            SignifLetChild::Reg(elem) => elem.serialize_mei(writer),
            SignifLetChild::AnchoredText(elem) => elem.serialize_mei(writer),
            SignifLetChild::Heraldry(elem) => elem.serialize_mei(writer),
            SignifLetChild::BiblStruct(elem) => elem.serialize_mei(writer),
            SignifLetChild::Supplied(elem) => elem.serialize_mei(writer),
            SignifLetChild::Address(elem) => elem.serialize_mei(writer),
            SignifLetChild::SecFolio(elem) => elem.serialize_mei(writer),
            SignifLetChild::PostCode(elem) => elem.serialize_mei(writer),
            SignifLetChild::Num(elem) => elem.serialize_mei(writer),
            SignifLetChild::PersName(elem) => elem.serialize_mei(writer),
            SignifLetChild::Choice(elem) => elem.serialize_mei(writer),
            SignifLetChild::Bloc(elem) => elem.serialize_mei(writer),
            SignifLetChild::Rend(elem) => elem.serialize_mei(writer),
            SignifLetChild::Abbr(elem) => elem.serialize_mei(writer),
            SignifLetChild::Depth(elem) => elem.serialize_mei(writer),
            SignifLetChild::Name(elem) => elem.serialize_mei(writer),
            SignifLetChild::Country(elem) => elem.serialize_mei(writer),
            SignifLetChild::Expan(elem) => elem.serialize_mei(writer),
            SignifLetChild::Orig(elem) => elem.serialize_mei(writer),
            SignifLetChild::Height(elem) => elem.serialize_mei(writer),
            SignifLetChild::RelationList(elem) => elem.serialize_mei(writer),
            SignifLetChild::Locus(elem) => elem.serialize_mei(writer),
            SignifLetChild::Line(elem) => elem.serialize_mei(writer),
            SignifLetChild::PeriodName(elem) => elem.serialize_mei(writer),
            SignifLetChild::Annot(elem) => elem.serialize_mei(writer),
            SignifLetChild::Ref(elem) => elem.serialize_mei(writer),
            SignifLetChild::Title(elem) => elem.serialize_mei(writer),
            SignifLetChild::GeogName(elem) => elem.serialize_mei(writer),
            SignifLetChild::HandShift(elem) => elem.serialize_mei(writer),
            SignifLetChild::Region(elem) => elem.serialize_mei(writer),
            SignifLetChild::Symbol(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Actor element
// ============================================================================

impl MeiSerialize for Actor {
    fn element_name(&self) -> &'static str {
        "actor"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
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

impl MeiSerialize for ActorChild {
    fn element_name(&self) -> &'static str {
        match self {
            ActorChild::Text(_) => "#text",
            ActorChild::District(_) => "district",
            ActorChild::Ref(_) => "ref",
            ActorChild::Bibl(_) => "bibl",
            ActorChild::Expan(_) => "expan",
            ActorChild::Stack(_) => "stack",
            ActorChild::Fig(_) => "fig",
            ActorChild::Street(_) => "street",
            ActorChild::Title(_) => "title",
            ActorChild::Depth(_) => "depth",
            ActorChild::Width(_) => "width",
            ActorChild::Bloc(_) => "bloc",
            ActorChild::Identifier(_) => "identifier",
            ActorChild::Relation(_) => "relation",
            ActorChild::Annot(_) => "annot",
            ActorChild::Address(_) => "address",
            ActorChild::RelationList(_) => "relationList",
            ActorChild::Dim(_) => "dim",
            ActorChild::Locus(_) => "locus",
            ActorChild::GeogName(_) => "geogName",
            ActorChild::PostCode(_) => "postCode",
            ActorChild::Heraldry(_) => "heraldry",
            ActorChild::BiblStruct(_) => "biblStruct",
            ActorChild::Date(_) => "date",
            ActorChild::PostBox(_) => "postBox",
            ActorChild::Abbr(_) => "abbr",
            ActorChild::Dimensions(_) => "dimensions",
            ActorChild::Ptr(_) => "ptr",
            ActorChild::Q(_) => "q",
            ActorChild::Catchwords(_) => "catchwords",
            ActorChild::Rend(_) => "rend",
            ActorChild::LocusGrp(_) => "locusGrp",
            ActorChild::PersName(_) => "persName",
            ActorChild::Stamp(_) => "stamp",
            ActorChild::Region(_) => "region",
            ActorChild::GeogFeat(_) => "geogFeat",
            ActorChild::Height(_) => "height",
            ActorChild::Extent(_) => "extent",
            ActorChild::Name(_) => "name",
            ActorChild::Num(_) => "num",
            ActorChild::Signatures(_) => "signatures",
            ActorChild::StyleName(_) => "styleName",
            ActorChild::SecFolio(_) => "secFolio",
            ActorChild::Settlement(_) => "settlement",
            ActorChild::CorpName(_) => "corpName",
            ActorChild::Symbol(_) => "symbol",
            ActorChild::Term(_) => "term",
            ActorChild::Lb(_) => "lb",
            ActorChild::PeriodName(_) => "periodName",
            ActorChild::Country(_) => "country",
            ActorChild::Repository(_) => "repository",
            ActorChild::Seg(_) => "seg",
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
            ActorChild::Text(text) => writer.write_text(text),
            ActorChild::District(elem) => elem.serialize_mei(writer),
            ActorChild::Ref(elem) => elem.serialize_mei(writer),
            ActorChild::Bibl(elem) => elem.serialize_mei(writer),
            ActorChild::Expan(elem) => elem.serialize_mei(writer),
            ActorChild::Stack(elem) => elem.serialize_mei(writer),
            ActorChild::Fig(elem) => elem.serialize_mei(writer),
            ActorChild::Street(elem) => elem.serialize_mei(writer),
            ActorChild::Title(elem) => elem.serialize_mei(writer),
            ActorChild::Depth(elem) => elem.serialize_mei(writer),
            ActorChild::Width(elem) => elem.serialize_mei(writer),
            ActorChild::Bloc(elem) => elem.serialize_mei(writer),
            ActorChild::Identifier(elem) => elem.serialize_mei(writer),
            ActorChild::Relation(elem) => elem.serialize_mei(writer),
            ActorChild::Annot(elem) => elem.serialize_mei(writer),
            ActorChild::Address(elem) => elem.serialize_mei(writer),
            ActorChild::RelationList(elem) => elem.serialize_mei(writer),
            ActorChild::Dim(elem) => elem.serialize_mei(writer),
            ActorChild::Locus(elem) => elem.serialize_mei(writer),
            ActorChild::GeogName(elem) => elem.serialize_mei(writer),
            ActorChild::PostCode(elem) => elem.serialize_mei(writer),
            ActorChild::Heraldry(elem) => elem.serialize_mei(writer),
            ActorChild::BiblStruct(elem) => elem.serialize_mei(writer),
            ActorChild::Date(elem) => elem.serialize_mei(writer),
            ActorChild::PostBox(elem) => elem.serialize_mei(writer),
            ActorChild::Abbr(elem) => elem.serialize_mei(writer),
            ActorChild::Dimensions(elem) => elem.serialize_mei(writer),
            ActorChild::Ptr(elem) => elem.serialize_mei(writer),
            ActorChild::Q(elem) => elem.serialize_mei(writer),
            ActorChild::Catchwords(elem) => elem.serialize_mei(writer),
            ActorChild::Rend(elem) => elem.serialize_mei(writer),
            ActorChild::LocusGrp(elem) => elem.serialize_mei(writer),
            ActorChild::PersName(elem) => elem.serialize_mei(writer),
            ActorChild::Stamp(elem) => elem.serialize_mei(writer),
            ActorChild::Region(elem) => elem.serialize_mei(writer),
            ActorChild::GeogFeat(elem) => elem.serialize_mei(writer),
            ActorChild::Height(elem) => elem.serialize_mei(writer),
            ActorChild::Extent(elem) => elem.serialize_mei(writer),
            ActorChild::Name(elem) => elem.serialize_mei(writer),
            ActorChild::Num(elem) => elem.serialize_mei(writer),
            ActorChild::Signatures(elem) => elem.serialize_mei(writer),
            ActorChild::StyleName(elem) => elem.serialize_mei(writer),
            ActorChild::SecFolio(elem) => elem.serialize_mei(writer),
            ActorChild::Settlement(elem) => elem.serialize_mei(writer),
            ActorChild::CorpName(elem) => elem.serialize_mei(writer),
            ActorChild::Symbol(elem) => elem.serialize_mei(writer),
            ActorChild::Term(elem) => elem.serialize_mei(writer),
            ActorChild::Lb(elem) => elem.serialize_mei(writer),
            ActorChild::PeriodName(elem) => elem.serialize_mei(writer),
            ActorChild::Country(elem) => elem.serialize_mei(writer),
            ActorChild::Repository(elem) => elem.serialize_mei(writer),
            ActorChild::Seg(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// CatRel element
// ============================================================================

impl MeiSerialize for CatRel {
    fn element_name(&self) -> &'static str {
        "catRel"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.basic.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.labelled.collect_attributes());
        attrs.extend(self.linking.collect_attributes());
        attrs.extend(self.n_number_like.collect_attributes());
        attrs.extend(self.responsibility.collect_attributes());
        if let Some(ref v) = self.r#type {
            attrs.push(("type", v.clone()));
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

impl MeiSerialize for CatRelChild {
    fn element_name(&self) -> &'static str {
        match self {
            CatRelChild::Label(_) => "label",
            CatRelChild::Desc(_) => "desc",
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
            CatRelChild::Label(elem) => elem.serialize_mei(writer),
            CatRelChild::Desc(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Context element
// ============================================================================

impl MeiSerialize for Context {
    fn element_name(&self) -> &'static str {
        "context"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
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

impl MeiSerialize for ContextChild {
    fn element_name(&self) -> &'static str {
        match self {
            ContextChild::Text(_) => "#text",
            ContextChild::Dim(_) => "dim",
            ContextChild::BiblStruct(_) => "biblStruct",
            ContextChild::PostCode(_) => "postCode",
            ContextChild::Depth(_) => "depth",
            ContextChild::Extent(_) => "extent",
            ContextChild::Width(_) => "width",
            ContextChild::Term(_) => "term",
            ContextChild::Title(_) => "title",
            ContextChild::Head(_) => "head",
            ContextChild::Name(_) => "name",
            ContextChild::Date(_) => "date",
            ContextChild::Expan(_) => "expan",
            ContextChild::Ptr(_) => "ptr",
            ContextChild::Dimensions(_) => "dimensions",
            ContextChild::Street(_) => "street",
            ContextChild::Bloc(_) => "bloc",
            ContextChild::Identifier(_) => "identifier",
            ContextChild::Region(_) => "region",
            ContextChild::LocusGrp(_) => "locusGrp",
            ContextChild::Stack(_) => "stack",
            ContextChild::Symbol(_) => "symbol",
            ContextChild::Address(_) => "address",
            ContextChild::PeriodName(_) => "periodName",
            ContextChild::Rend(_) => "rend",
            ContextChild::Lb(_) => "lb",
            ContextChild::GeogFeat(_) => "geogFeat",
            ContextChild::Repository(_) => "repository",
            ContextChild::Locus(_) => "locus",
            ContextChild::SecFolio(_) => "secFolio",
            ContextChild::Seg(_) => "seg",
            ContextChild::Settlement(_) => "settlement",
            ContextChild::Country(_) => "country",
            ContextChild::GeogName(_) => "geogName",
            ContextChild::Q(_) => "q",
            ContextChild::Height(_) => "height",
            ContextChild::Bibl(_) => "bibl",
            ContextChild::Stamp(_) => "stamp",
            ContextChild::StyleName(_) => "styleName",
            ContextChild::Catchwords(_) => "catchwords",
            ContextChild::Abbr(_) => "abbr",
            ContextChild::PersName(_) => "persName",
            ContextChild::PostBox(_) => "postBox",
            ContextChild::P(_) => "p",
            ContextChild::Annot(_) => "annot",
            ContextChild::Ref(_) => "ref",
            ContextChild::District(_) => "district",
            ContextChild::CorpName(_) => "corpName",
            ContextChild::Heraldry(_) => "heraldry",
            ContextChild::Num(_) => "num",
            ContextChild::RelationList(_) => "relationList",
            ContextChild::Relation(_) => "relation",
            ContextChild::Signatures(_) => "signatures",
            ContextChild::Fig(_) => "fig",
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
            ContextChild::Text(text) => writer.write_text(text),
            ContextChild::Dim(elem) => elem.serialize_mei(writer),
            ContextChild::BiblStruct(elem) => elem.serialize_mei(writer),
            ContextChild::PostCode(elem) => elem.serialize_mei(writer),
            ContextChild::Depth(elem) => elem.serialize_mei(writer),
            ContextChild::Extent(elem) => elem.serialize_mei(writer),
            ContextChild::Width(elem) => elem.serialize_mei(writer),
            ContextChild::Term(elem) => elem.serialize_mei(writer),
            ContextChild::Title(elem) => elem.serialize_mei(writer),
            ContextChild::Head(elem) => elem.serialize_mei(writer),
            ContextChild::Name(elem) => elem.serialize_mei(writer),
            ContextChild::Date(elem) => elem.serialize_mei(writer),
            ContextChild::Expan(elem) => elem.serialize_mei(writer),
            ContextChild::Ptr(elem) => elem.serialize_mei(writer),
            ContextChild::Dimensions(elem) => elem.serialize_mei(writer),
            ContextChild::Street(elem) => elem.serialize_mei(writer),
            ContextChild::Bloc(elem) => elem.serialize_mei(writer),
            ContextChild::Identifier(elem) => elem.serialize_mei(writer),
            ContextChild::Region(elem) => elem.serialize_mei(writer),
            ContextChild::LocusGrp(elem) => elem.serialize_mei(writer),
            ContextChild::Stack(elem) => elem.serialize_mei(writer),
            ContextChild::Symbol(elem) => elem.serialize_mei(writer),
            ContextChild::Address(elem) => elem.serialize_mei(writer),
            ContextChild::PeriodName(elem) => elem.serialize_mei(writer),
            ContextChild::Rend(elem) => elem.serialize_mei(writer),
            ContextChild::Lb(elem) => elem.serialize_mei(writer),
            ContextChild::GeogFeat(elem) => elem.serialize_mei(writer),
            ContextChild::Repository(elem) => elem.serialize_mei(writer),
            ContextChild::Locus(elem) => elem.serialize_mei(writer),
            ContextChild::SecFolio(elem) => elem.serialize_mei(writer),
            ContextChild::Seg(elem) => elem.serialize_mei(writer),
            ContextChild::Settlement(elem) => elem.serialize_mei(writer),
            ContextChild::Country(elem) => elem.serialize_mei(writer),
            ContextChild::GeogName(elem) => elem.serialize_mei(writer),
            ContextChild::Q(elem) => elem.serialize_mei(writer),
            ContextChild::Height(elem) => elem.serialize_mei(writer),
            ContextChild::Bibl(elem) => elem.serialize_mei(writer),
            ContextChild::Stamp(elem) => elem.serialize_mei(writer),
            ContextChild::StyleName(elem) => elem.serialize_mei(writer),
            ContextChild::Catchwords(elem) => elem.serialize_mei(writer),
            ContextChild::Abbr(elem) => elem.serialize_mei(writer),
            ContextChild::PersName(elem) => elem.serialize_mei(writer),
            ContextChild::PostBox(elem) => elem.serialize_mei(writer),
            ContextChild::P(elem) => elem.serialize_mei(writer),
            ContextChild::Annot(elem) => elem.serialize_mei(writer),
            ContextChild::Ref(elem) => elem.serialize_mei(writer),
            ContextChild::District(elem) => elem.serialize_mei(writer),
            ContextChild::CorpName(elem) => elem.serialize_mei(writer),
            ContextChild::Heraldry(elem) => elem.serialize_mei(writer),
            ContextChild::Num(elem) => elem.serialize_mei(writer),
            ContextChild::RelationList(elem) => elem.serialize_mei(writer),
            ContextChild::Relation(elem) => elem.serialize_mei(writer),
            ContextChild::Signatures(elem) => elem.serialize_mei(writer),
            ContextChild::Fig(elem) => elem.serialize_mei(writer),
        }
    }
}

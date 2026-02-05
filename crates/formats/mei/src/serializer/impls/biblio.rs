//! Serializer implementations for bibliographic and codicological MEI elements.
//!
//! This module contains implementations for:
//! - ExtData, AvFile, Cutout, Bifolium, Folium, Patch (codicology)
//! - Analytic, Monogr, Series, Desc (bibliography)

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::att::{AttBifoliumSurfaces, AttFoliumSurfaces};
use tusk_model::elements::{
    Analytic, AnalyticChild, AvFile, AvFileChild, Bifolium, BifoliumChild, Cutout, CutoutChild,
    ExtData, ExtDataChild, Folium, FoliumChild, Monogr, MonogrChild, Patch, PatchChild, Series,
    SeriesChild,
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

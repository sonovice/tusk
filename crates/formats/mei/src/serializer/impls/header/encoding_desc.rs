//! Serializer implementations for encoding description elements.
//!
//! Contains: EncodingDesc, AppInfo, Application, ClassDecls, Taxonomy, Category,
//! EditorialDecl, Segmentation, StdVals, Interpretation, Normalization, Correction,
//! ProjectDesc, SamplingDecl, DomainsDecl, TagsDecl, Namespace, TagUsage, AttUsage.

use super::super::to_attr_string;
use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    AppInfo, AppInfoChild, Application, ApplicationChild, AttUsage, AttUsageChild, Category,
    CategoryChild, ClassDecls, ClassDeclsChild, Correction, CorrectionChild, Desc, DescChild,
    DomainsDecl, EditorialDecl, EditorialDeclChild, EncodingDesc, EncodingDescChild,
    Interpretation, InterpretationChild, Label, LabelChild, Namespace, NamespaceChild,
    Normalization, NormalizationChild, ProjectDesc, ProjectDescChild, SamplingDecl,
    SamplingDeclChild, Segmentation, SegmentationChild, StdVals, StdValsChild, TagUsage,
    TagUsageChild, TagsDecl, TagsDeclChild, Taxonomy, TaxonomyChild,
};

// ============================================================================
// EncodingDesc
// ============================================================================

impl MeiSerialize for EncodingDesc {
    fn element_name(&self) -> &'static str {
        "encodingDesc"
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

impl MeiSerialize for EncodingDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            EncodingDescChild::AppInfo(_) => "appInfo",
            EncodingDescChild::EditorialDecl(_) => "editorialDecl",
            EncodingDescChild::ProjectDesc(_) => "projectDesc",
            EncodingDescChild::SamplingDecl(_) => "samplingDecl",
            EncodingDescChild::TagsDecl(_) => "tagsDecl",
            EncodingDescChild::ClassDecls(_) => "classDecls",
            EncodingDescChild::DomainsDecl(_) => "domainsDecl",
            EncodingDescChild::Head(_) => "head",
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
            EncodingDescChild::Head(elem) => elem.serialize_mei(writer),
            EncodingDescChild::AppInfo(elem) => elem.serialize_mei(writer),
            EncodingDescChild::ClassDecls(elem) => elem.serialize_mei(writer),
            EncodingDescChild::EditorialDecl(elem) => elem.serialize_mei(writer),
            EncodingDescChild::ProjectDesc(elem) => elem.serialize_mei(writer),
            EncodingDescChild::SamplingDecl(elem) => elem.serialize_mei(writer),
            EncodingDescChild::TagsDecl(elem) => elem.serialize_mei(writer),
            EncodingDescChild::DomainsDecl(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// AppInfo and Application
// ============================================================================

impl MeiSerialize for AppInfo {
    fn element_name(&self) -> &'static str {
        "appInfo"
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

impl MeiSerialize for AppInfoChild {
    fn element_name(&self) -> &'static str {
        match self {
            AppInfoChild::Application(_) => "application",
            AppInfoChild::Head(_) => "head",
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
            AppInfoChild::Application(elem) => elem.serialize_mei(writer),
            AppInfoChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Application {
    fn element_name(&self) -> &'static str {
        "application"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.datable.collect_attributes());
        // Element-local attribute
        if let Some(ref version) = self.version {
            attrs.push(("version", version.clone()));
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

impl MeiSerialize for ApplicationChild {
    fn element_name(&self) -> &'static str {
        match self {
            ApplicationChild::Name(_) => "name",
            ApplicationChild::Ptr(_) => "ptr",
            ApplicationChild::Ref(_) => "ref",
            ApplicationChild::P(_) => "p",
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
            ApplicationChild::Name(elem) => elem.serialize_mei(writer),
            ApplicationChild::P(elem) => elem.serialize_mei(writer),
            ApplicationChild::Ptr(elem) => elem.serialize_mei(writer),
            ApplicationChild::Ref(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// ClassDecls, Taxonomy, Category
// ============================================================================

impl MeiSerialize for ClassDecls {
    fn element_name(&self) -> &'static str {
        "classDecls"
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

impl MeiSerialize for ClassDeclsChild {
    fn element_name(&self) -> &'static str {
        match self {
            ClassDeclsChild::Head(_) => "head",
            ClassDeclsChild::Taxonomy(_) => "taxonomy",
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
            ClassDeclsChild::Head(elem) => elem.serialize_mei(writer),
            ClassDeclsChild::Taxonomy(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Taxonomy {
    fn element_name(&self) -> &'static str {
        "taxonomy"
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

impl MeiSerialize for TaxonomyChild {
    fn element_name(&self) -> &'static str {
        match self {
            TaxonomyChild::Head(_) => "head",
            TaxonomyChild::Bibl(_) => "bibl",
            TaxonomyChild::Taxonomy(_) => "taxonomy",
            TaxonomyChild::Desc(_) => "desc",
            TaxonomyChild::Category(_) => "category",
            TaxonomyChild::BiblStruct(_) => "biblStruct",
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
            TaxonomyChild::Head(elem) => elem.serialize_mei(writer),
            TaxonomyChild::Taxonomy(elem) => elem.serialize_mei(writer),
            TaxonomyChild::Category(elem) => elem.serialize_mei(writer),
            TaxonomyChild::Bibl(elem) => elem.serialize_mei(writer),
            TaxonomyChild::Desc(_) => Ok(()), // TODO: implement Desc serializer
            TaxonomyChild::BiblStruct(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Category {
    fn element_name(&self) -> &'static str {
        "category"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.authorized.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
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

impl MeiSerialize for CategoryChild {
    fn element_name(&self) -> &'static str {
        match self {
            CategoryChild::Label(_) => "label",
            CategoryChild::Desc(_) => "desc",
            CategoryChild::CatRel(_) => "catRel",
            CategoryChild::AltId(_) => "altId",
            CategoryChild::Category(_) => "category",
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
            CategoryChild::AltId(elem) => elem.serialize_mei(writer),
            CategoryChild::Category(elem) => elem.serialize_mei(writer),
            CategoryChild::Label(elem) => elem.serialize_mei(writer),
            CategoryChild::Desc(_) => Ok(()), // TODO: implement Desc serializer
            CategoryChild::CatRel(_) => Ok(()), // TODO: implement CatRel serializer
        }
    }
}

// ============================================================================
// EditorialDecl and related
// ============================================================================

impl MeiSerialize for EditorialDecl {
    fn element_name(&self) -> &'static str {
        "editorialDecl"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for EditorialDeclChild {
    fn element_name(&self) -> &'static str {
        match self {
            EditorialDeclChild::Segmentation(_) => "segmentation",
            EditorialDeclChild::StdVals(_) => "stdVals",
            EditorialDeclChild::Interpretation(_) => "interpretation",
            EditorialDeclChild::Normalization(_) => "normalization",
            EditorialDeclChild::Correction(_) => "correction",
            EditorialDeclChild::Head(_) => "head",
            EditorialDeclChild::P(_) => "p",
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
            EditorialDeclChild::Segmentation(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::StdVals(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::Interpretation(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::Normalization(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::Correction(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::Head(elem) => elem.serialize_mei(writer),
            EditorialDeclChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Segmentation {
    fn element_name(&self) -> &'static str {
        "segmentation"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for SegmentationChild {
    fn element_name(&self) -> &'static str {
        match self {
            SegmentationChild::P(_) => "p",
            SegmentationChild::Head(_) => "head",
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
            SegmentationChild::P(elem) => elem.serialize_mei(writer),
            SegmentationChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for StdVals {
    fn element_name(&self) -> &'static str {
        "stdVals"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for StdValsChild {
    fn element_name(&self) -> &'static str {
        match self {
            StdValsChild::P(_) => "p",
            StdValsChild::Head(_) => "head",
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
            StdValsChild::P(elem) => elem.serialize_mei(writer),
            StdValsChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Interpretation {
    fn element_name(&self) -> &'static str {
        "interpretation"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for InterpretationChild {
    fn element_name(&self) -> &'static str {
        match self {
            InterpretationChild::Head(_) => "head",
            InterpretationChild::P(_) => "p",
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
            InterpretationChild::Head(elem) => elem.serialize_mei(writer),
            InterpretationChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Normalization {
    fn element_name(&self) -> &'static str {
        "normalization"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.regular_method.collect_attributes());
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

impl MeiSerialize for NormalizationChild {
    fn element_name(&self) -> &'static str {
        match self {
            NormalizationChild::Head(_) => "head",
            NormalizationChild::P(_) => "p",
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
            NormalizationChild::Head(elem) => elem.serialize_mei(writer),
            NormalizationChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

impl MeiSerialize for Correction {
    fn element_name(&self) -> &'static str {
        "correction"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.regular_method.collect_attributes());
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

impl MeiSerialize for CorrectionChild {
    fn element_name(&self) -> &'static str {
        match self {
            CorrectionChild::Head(_) => "head",
            CorrectionChild::P(_) => "p",
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
            CorrectionChild::Head(elem) => elem.serialize_mei(writer),
            CorrectionChild::P(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// ProjectDesc
// ============================================================================

impl MeiSerialize for ProjectDesc {
    fn element_name(&self) -> &'static str {
        "projectDesc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for ProjectDescChild {
    fn element_name(&self) -> &'static str {
        match self {
            ProjectDescChild::P(_) => "p",
            ProjectDescChild::Head(_) => "head",
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
            ProjectDescChild::P(elem) => elem.serialize_mei(writer),
            ProjectDescChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// SamplingDecl
// ============================================================================

impl MeiSerialize for SamplingDecl {
    fn element_name(&self) -> &'static str {
        "samplingDecl"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        attrs.extend(self.data_pointing.collect_attributes());
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

impl MeiSerialize for SamplingDeclChild {
    fn element_name(&self) -> &'static str {
        match self {
            SamplingDeclChild::P(_) => "p",
            SamplingDeclChild::Head(_) => "head",
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
            SamplingDeclChild::P(elem) => elem.serialize_mei(writer),
            SamplingDeclChild::Head(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Label
// ============================================================================

impl MeiSerialize for Label {
    fn element_name(&self) -> &'static str {
        "label"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.source.collect_attributes());
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

impl MeiSerialize for LabelChild {
    fn element_name(&self) -> &'static str {
        match self {
            LabelChild::Text(_) => "#text",
            LabelChild::CorpName(_) => "corpName",
            LabelChild::Height(_) => "height",
            LabelChild::Relation(_) => "relation",
            LabelChild::Bloc(_) => "bloc",
            LabelChild::Stack(_) => "stack",
            LabelChild::Ref(_) => "ref",
            LabelChild::Subst(_) => "subst",
            LabelChild::PostCode(_) => "postCode",
            LabelChild::Corr(_) => "corr",
            LabelChild::Width(_) => "width",
            LabelChild::GeogFeat(_) => "geogFeat",
            LabelChild::Dimensions(_) => "dimensions",
            LabelChild::Q(_) => "q",
            LabelChild::HandShift(_) => "handShift",
            LabelChild::Del(_) => "del",
            LabelChild::Orig(_) => "orig",
            LabelChild::PostBox(_) => "postBox",
            LabelChild::Restore(_) => "restore",
            LabelChild::Supplied(_) => "supplied",
            LabelChild::Catchwords(_) => "catchwords",
            LabelChild::Bibl(_) => "bibl",
            LabelChild::Expan(_) => "expan",
            LabelChild::Add(_) => "add",
            LabelChild::Reg(_) => "reg",
            LabelChild::Repository(_) => "repository",
            LabelChild::Region(_) => "region",
            LabelChild::StyleName(_) => "styleName",
            LabelChild::Sic(_) => "sic",
            LabelChild::Abbr(_) => "abbr",
            LabelChild::Title(_) => "title",
            LabelChild::Heraldry(_) => "heraldry",
            LabelChild::Fig(_) => "fig",
            LabelChild::Locus(_) => "locus",
            LabelChild::Num(_) => "num",
            LabelChild::Rend(_) => "rend",
            LabelChild::LocusGrp(_) => "locusGrp",
            LabelChild::SecFolio(_) => "secFolio",
            LabelChild::Seg(_) => "seg",
            LabelChild::BiblStruct(_) => "biblStruct",
            LabelChild::Stamp(_) => "stamp",
            LabelChild::Damage(_) => "damage",
            LabelChild::Name(_) => "name",
            LabelChild::RelationList(_) => "relationList",
            LabelChild::Street(_) => "street",
            LabelChild::Identifier(_) => "identifier",
            LabelChild::Lb(_) => "lb",
            LabelChild::Settlement(_) => "settlement",
            LabelChild::Annot(_) => "annot",
            LabelChild::Ptr(_) => "ptr",
            LabelChild::Address(_) => "address",
            LabelChild::Choice(_) => "choice",
            LabelChild::PersName(_) => "persName",
            LabelChild::Symbol(_) => "symbol",
            LabelChild::PeriodName(_) => "periodName",
            LabelChild::Date(_) => "date",
            LabelChild::Gap(_) => "gap",
            LabelChild::District(_) => "district",
            LabelChild::GeogName(_) => "geogName",
            LabelChild::Country(_) => "country",
            LabelChild::Term(_) => "term",
            LabelChild::Depth(_) => "depth",
            LabelChild::Unclear(_) => "unclear",
            LabelChild::Signatures(_) => "signatures",
            LabelChild::Extent(_) => "extent",
            LabelChild::Dim(_) => "dim",
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
            LabelChild::Text(text) => {
                writer.write_text(text)?;
                Ok(())
            }
            LabelChild::Rend(elem) => elem.serialize_mei(writer),
            LabelChild::Lb(elem) => elem.serialize_mei(writer),
            LabelChild::Title(elem) => elem.serialize_mei(writer),
            LabelChild::Name(elem) => elem.serialize_mei(writer),
            LabelChild::Identifier(elem) => elem.serialize_mei(writer),
            LabelChild::Ref(elem) => elem.serialize_mei(writer),
            LabelChild::Ptr(elem) => elem.serialize_mei(writer),
            LabelChild::Date(elem) => elem.serialize_mei(writer),
            LabelChild::PersName(elem) => elem.serialize_mei(writer),
            LabelChild::CorpName(elem) => elem.serialize_mei(writer),
            LabelChild::GeogName(elem) => elem.serialize_mei(writer),
            LabelChild::Address(elem) => elem.serialize_mei(writer),
            LabelChild::Bibl(elem) => elem.serialize_mei(writer),
            LabelChild::BiblStruct(elem) => elem.serialize_mei(writer),
            LabelChild::Annot(elem) => elem.serialize_mei(writer),
            LabelChild::Extent(elem) => elem.serialize_mei(writer),
            // Many other children - skip for now
            _ => Ok(()),
        }
    }
}

// ============================================================================
// DomainsDecl
// ============================================================================

impl MeiSerialize for DomainsDecl {
    fn element_name(&self) -> &'static str {
        "domainsDecl"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        if let Some(ref anl) = self.anl {
            if let Some(s) = to_attr_string(anl) {
                attrs.push(("anl", s));
            }
        }
        if let Some(ref ges) = self.ges {
            if let Some(s) = to_attr_string(ges) {
                attrs.push(("ges", s));
            }
        }
        if let Some(ref vis) = self.vis {
            if let Some(s) = to_attr_string(vis) {
                attrs.push(("vis", s));
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
// TagsDecl
// ============================================================================

impl MeiSerialize for TagsDecl {
    fn element_name(&self) -> &'static str {
        "tagsDecl"
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

impl MeiSerialize for TagsDeclChild {
    fn element_name(&self) -> &'static str {
        match self {
            TagsDeclChild::Head(_) => "head",
            TagsDeclChild::Namespace(_) => "namespace",
            TagsDeclChild::Desc(_) => "desc",
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
            TagsDeclChild::Head(elem) => elem.serialize_mei(writer),
            TagsDeclChild::Namespace(elem) => elem.serialize_mei(writer),
            TagsDeclChild::Desc(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Namespace
// ============================================================================

impl MeiSerialize for Namespace {
    fn element_name(&self) -> &'static str {
        "namespace"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        if let Some(ref name) = self.name {
            attrs.push(("name", name.0.clone()));
        }
        if let Some(ref prefix) = self.prefix {
            attrs.push(("prefix", prefix.to_string()));
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

impl MeiSerialize for NamespaceChild {
    fn element_name(&self) -> &'static str {
        match self {
            NamespaceChild::Desc(_) => "desc",
            NamespaceChild::TagUsage(_) => "tagUsage",
            NamespaceChild::AttUsage(_) => "attUsage",
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
            NamespaceChild::Desc(elem) => elem.serialize_mei(writer),
            NamespaceChild::TagUsage(elem) => elem.serialize_mei(writer),
            NamespaceChild::AttUsage(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// TagUsage
// ============================================================================

impl MeiSerialize for TagUsage {
    fn element_name(&self) -> &'static str {
        "tagUsage"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        if let Some(ref name) = self.name {
            attrs.push(("name", name.to_string()));
        }
        if let Some(ref context) = self.context {
            attrs.push(("context", context.clone()));
        }
        if let Some(occurs) = self.occurs {
            attrs.push(("occurs", occurs.to_string()));
        }
        if let Some(withid) = self.withid {
            attrs.push(("withid", withid.to_string()));
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

impl MeiSerialize for TagUsageChild {
    fn element_name(&self) -> &'static str {
        match self {
            TagUsageChild::AttUsage(_) => "attUsage",
            TagUsageChild::Desc(_) => "desc",
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
            TagUsageChild::AttUsage(elem) => elem.serialize_mei(writer),
            TagUsageChild::Desc(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// AttUsage
// ============================================================================

impl MeiSerialize for AttUsage {
    fn element_name(&self) -> &'static str {
        "attUsage"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.bibl.collect_attributes());
        if let Some(ref name) = self.name {
            attrs.push(("name", name.to_string()));
        }
        if let Some(ref context) = self.context {
            attrs.push(("context", context.clone()));
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

impl MeiSerialize for AttUsageChild {
    fn element_name(&self) -> &'static str {
        match self {
            AttUsageChild::Desc(_) => "desc",
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
            AttUsageChild::Desc(elem) => elem.serialize_mei(writer),
        }
    }
}

// ============================================================================
// Desc
// ============================================================================

impl MeiSerialize for Desc {
    fn element_name(&self) -> &'static str {
        "desc"
    }

    fn collect_all_attributes(&self) -> Vec<(&'static str, String)> {
        let mut attrs = Vec::new();
        attrs.extend(self.common.collect_attributes());
        attrs.extend(self.facsimile.collect_attributes());
        attrs.extend(self.lang.collect_attributes());
        attrs.extend(self.source.collect_attributes());
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

impl MeiSerialize for DescChild {
    fn element_name(&self) -> &'static str {
        match self {
            DescChild::Text(_) => "#text",
            DescChild::Region(_) => "region",
            DescChild::Depth(_) => "depth",
            DescChild::Locus(_) => "locus",
            DescChild::Num(_) => "num",
            DescChild::Signatures(_) => "signatures",
            DescChild::Expan(_) => "expan",
            DescChild::Symbol(_) => "symbol",
            DescChild::GeogFeat(_) => "geogFeat",
            DescChild::District(_) => "district",
            DescChild::Stamp(_) => "stamp",
            DescChild::Annot(_) => "annot",
            DescChild::Orig(_) => "orig",
            DescChild::SecFolio(_) => "secFolio",
            DescChild::Seg(_) => "seg",
            DescChild::LocusGrp(_) => "locusGrp",
            DescChild::GeogName(_) => "geogName",
            DescChild::Unclear(_) => "unclear",
            DescChild::Width(_) => "width",
            DescChild::Gap(_) => "gap",
            DescChild::Corr(_) => "corr",
            DescChild::Lb(_) => "lb",
            DescChild::Dimensions(_) => "dimensions",
            DescChild::Address(_) => "address",
            DescChild::PostCode(_) => "postCode",
            DescChild::Sic(_) => "sic",
            DescChild::Term(_) => "term",
            DescChild::Fig(_) => "fig",
            DescChild::PeriodName(_) => "periodName",
            DescChild::Stack(_) => "stack",
            DescChild::Catchwords(_) => "catchwords",
            DescChild::Extent(_) => "extent",
            DescChild::Dim(_) => "dim",
            DescChild::Ref(_) => "ref",
            DescChild::CorpName(_) => "corpName",
            DescChild::Bloc(_) => "bloc",
            DescChild::Date(_) => "date",
            DescChild::Q(_) => "q",
            DescChild::Title(_) => "title",
            DescChild::Subst(_) => "subst",
            DescChild::BiblStruct(_) => "biblStruct",
            DescChild::Reg(_) => "reg",
            DescChild::Height(_) => "height",
            DescChild::PostBox(_) => "postBox",
            DescChild::Ptr(_) => "ptr",
            DescChild::Identifier(_) => "identifier",
            DescChild::Rend(_) => "rend",
            DescChild::Damage(_) => "damage",
            DescChild::RelationList(_) => "relationList",
            DescChild::Name(_) => "name",
            DescChild::Del(_) => "del",
            DescChild::Street(_) => "street",
            DescChild::HandShift(_) => "handShift",
            DescChild::Choice(_) => "choice",
            DescChild::Abbr(_) => "abbr",
            DescChild::Restore(_) => "restore",
            DescChild::Repository(_) => "repository",
            DescChild::Relation(_) => "relation",
            DescChild::Add(_) => "add",
            DescChild::Heraldry(_) => "heraldry",
            DescChild::Country(_) => "country",
            DescChild::StyleName(_) => "styleName",
            DescChild::PersName(_) => "persName",
            DescChild::Bibl(_) => "bibl",
            DescChild::Settlement(_) => "settlement",
            DescChild::Supplied(_) => "supplied",
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
            DescChild::Text(text) => writer.write_text(text),
            DescChild::Region(elem) => elem.serialize_mei(writer),
            DescChild::Depth(elem) => elem.serialize_mei(writer),
            DescChild::Locus(elem) => elem.serialize_mei(writer),
            DescChild::Num(elem) => elem.serialize_mei(writer),
            // Phase 14 elements - stub
            DescChild::Signatures(_) => Ok(()),
            DescChild::Expan(elem) => elem.serialize_mei(writer),
            DescChild::Symbol(elem) => elem.serialize_mei(writer),
            DescChild::GeogFeat(elem) => elem.serialize_mei(writer),
            DescChild::District(elem) => elem.serialize_mei(writer),
            DescChild::Stamp(elem) => elem.serialize_mei(writer),
            DescChild::Annot(elem) => elem.serialize_mei(writer),
            DescChild::Orig(elem) => elem.serialize_mei(writer),
            // Phase 14 element - stub
            DescChild::SecFolio(_) => Ok(()),
            DescChild::Seg(elem) => elem.serialize_mei(writer),
            DescChild::LocusGrp(elem) => elem.serialize_mei(writer),
            DescChild::GeogName(elem) => elem.serialize_mei(writer),
            DescChild::Unclear(elem) => elem.serialize_mei(writer),
            DescChild::Width(elem) => elem.serialize_mei(writer),
            DescChild::Gap(elem) => elem.serialize_mei(writer),
            DescChild::Corr(elem) => elem.serialize_mei(writer),
            DescChild::Lb(elem) => elem.serialize_mei(writer),
            DescChild::Dimensions(elem) => elem.serialize_mei(writer),
            DescChild::Address(elem) => elem.serialize_mei(writer),
            DescChild::PostCode(elem) => elem.serialize_mei(writer),
            DescChild::Sic(elem) => elem.serialize_mei(writer),
            DescChild::Term(elem) => elem.serialize_mei(writer),
            DescChild::Fig(elem) => elem.serialize_mei(writer),
            DescChild::PeriodName(elem) => elem.serialize_mei(writer),
            DescChild::Stack(elem) => elem.serialize_mei(writer),
            // Phase 14 element - stub
            DescChild::Catchwords(_) => Ok(()),
            DescChild::Extent(elem) => elem.serialize_mei(writer),
            DescChild::Dim(elem) => elem.serialize_mei(writer),
            DescChild::Ref(elem) => elem.serialize_mei(writer),
            DescChild::CorpName(elem) => elem.serialize_mei(writer),
            DescChild::Bloc(elem) => elem.serialize_mei(writer),
            DescChild::Date(elem) => elem.serialize_mei(writer),
            DescChild::Q(elem) => elem.serialize_mei(writer),
            DescChild::Title(elem) => elem.serialize_mei(writer),
            DescChild::Subst(elem) => elem.serialize_mei(writer),
            DescChild::BiblStruct(elem) => elem.serialize_mei(writer),
            DescChild::Reg(elem) => elem.serialize_mei(writer),
            DescChild::Height(elem) => elem.serialize_mei(writer),
            DescChild::PostBox(elem) => elem.serialize_mei(writer),
            DescChild::Ptr(elem) => elem.serialize_mei(writer),
            DescChild::Identifier(elem) => elem.serialize_mei(writer),
            DescChild::Rend(elem) => elem.serialize_mei(writer),
            DescChild::Damage(elem) => elem.serialize_mei(writer),
            DescChild::RelationList(elem) => elem.serialize_mei(writer),
            DescChild::Name(elem) => elem.serialize_mei(writer),
            DescChild::Del(elem) => elem.serialize_mei(writer),
            DescChild::Street(elem) => elem.serialize_mei(writer),
            DescChild::HandShift(elem) => elem.serialize_mei(writer),
            DescChild::Choice(elem) => elem.serialize_mei(writer),
            DescChild::Abbr(elem) => elem.serialize_mei(writer),
            DescChild::Restore(elem) => elem.serialize_mei(writer),
            DescChild::Repository(elem) => elem.serialize_mei(writer),
            DescChild::Relation(elem) => elem.serialize_mei(writer),
            DescChild::Add(elem) => elem.serialize_mei(writer),
            // Phase 14 element - stub
            DescChild::Heraldry(_) => Ok(()),
            DescChild::Country(elem) => elem.serialize_mei(writer),
            DescChild::StyleName(elem) => elem.serialize_mei(writer),
            DescChild::PersName(elem) => elem.serialize_mei(writer),
            DescChild::Bibl(elem) => elem.serialize_mei(writer),
            DescChild::Settlement(elem) => elem.serialize_mei(writer),
            DescChild::Supplied(elem) => elem.serialize_mei(writer),
        }
    }
}

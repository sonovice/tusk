//! Serializer implementations for encoding description elements.
//!
//! Contains: EncodingDesc, AppInfo, Application, ClassDecls, Taxonomy, Category,
//! EditorialDecl, Segmentation, StdVals, Interpretation, Normalization, Correction,
//! ProjectDesc, SamplingDecl.

use crate::serializer::{CollectAttributes, MeiSerialize, MeiWriter, SerializeResult};
use std::io::Write;
use tusk_model::elements::{
    AppInfo, AppInfoChild, Application, ApplicationChild, Category, CategoryChild, ClassDecls,
    ClassDeclsChild, Correction, CorrectionChild, EditorialDecl, EditorialDeclChild, EncodingDesc,
    EncodingDescChild, Interpretation, InterpretationChild, Normalization, NormalizationChild,
    ProjectDesc, ProjectDescChild, SamplingDecl, SamplingDeclChild, Segmentation,
    SegmentationChild, StdVals, StdValsChild, Taxonomy, TaxonomyChild,
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
            EncodingDescChild::TagsDecl(_) => Ok(()), // TODO: implement TagsDecl serializer
            EncodingDescChild::DomainsDecl(_) => Ok(()), // TODO: implement DomainsDecl serializer
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
            TaxonomyChild::Bibl(_) => Ok(()), // TODO: implement Bibl serializer
            TaxonomyChild::Desc(_) => Ok(()), // TODO: implement Desc serializer
            TaxonomyChild::BiblStruct(_) => Ok(()), // TODO: implement BiblStruct serializer
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
            CategoryChild::Label(_) => Ok(()), // TODO: implement Label serializer
            CategoryChild::Desc(_) => Ok(()),  // TODO: implement Desc serializer
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

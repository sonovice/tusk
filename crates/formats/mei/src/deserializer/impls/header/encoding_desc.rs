//! Encoding description elements (EncodingDesc, AppInfo, EditorialDecl, etc.).

use super::super::{extract_attr, from_attr_string};
use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    AltId, AltIdChild, AppInfo, AppInfoChild, Application, ApplicationChild, CatRel, CatRelChild,
    Category, CategoryChild, ClassDecls, ClassDeclsChild, Correction, CorrectionChild, Desc,
    DescChild, EditorialDecl, EditorialDeclChild, EncodingDesc, EncodingDescChild, Interpretation,
    InterpretationChild, Normalization, NormalizationChild, ProjectDesc, ProjectDescChild,
    SamplingDecl, SamplingDeclChild, Segmentation, SegmentationChild, StdVals, StdValsChild,
    Taxonomy, TaxonomyChild,
};

// MeiDeserialize trait implementations
impl MeiDeserialize for EncodingDesc {
    fn element_name() -> &'static str {
        "encodingDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_encoding_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for AppInfo {
    fn element_name() -> &'static str {
        "appInfo"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_app_info_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for EditorialDecl {
    fn element_name() -> &'static str {
        "editorialDecl"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_editorial_decl_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Correction {
    fn element_name() -> &'static str {
        "correction"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_correction_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Interpretation {
    fn element_name() -> &'static str {
        "interpretation"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_interpretation_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Normalization {
    fn element_name() -> &'static str {
        "normalization"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_normalization_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Segmentation {
    fn element_name() -> &'static str {
        "segmentation"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_segmentation_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for StdVals {
    fn element_name() -> &'static str {
        "stdVals"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_std_vals_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ProjectDesc {
    fn element_name() -> &'static str {
        "projectDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_project_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SamplingDecl {
    fn element_name() -> &'static str {
        "samplingDecl"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sampling_decl_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ClassDecls {
    fn element_name() -> &'static str {
        "classDecls"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_class_decls_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Taxonomy {
    fn element_name() -> &'static str {
        "taxonomy"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_taxonomy_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Category {
    fn element_name() -> &'static str {
        "category"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_category_from_event(reader, attrs, is_empty)
    }
}

/// Parse an `<encodingDesc>` element from within another element.
pub(crate) fn parse_encoding_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<EncodingDesc> {
    let mut encoding_desc = EncodingDesc::default();

    // Extract attributes
    encoding_desc.common.extract_attributes(&mut attrs)?;
    encoding_desc.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("encodingDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::Head(Box::new(head)));
                }
                "appInfo" => {
                    let app_info = parse_app_info_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::AppInfo(Box::new(app_info)));
                }
                "editorialDecl" => {
                    let editorial_decl =
                        parse_editorial_decl_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::EditorialDecl(Box::new(editorial_decl)));
                }
                "projectDesc" => {
                    let project_desc =
                        parse_project_desc_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::ProjectDesc(Box::new(project_desc)));
                }
                "samplingDecl" => {
                    let sampling_decl =
                        parse_sampling_decl_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::SamplingDecl(Box::new(sampling_decl)));
                }
                "classDecls" => {
                    let class_decls =
                        parse_class_decls_from_event(reader, child_attrs, child_empty)?;
                    encoding_desc
                        .children
                        .push(EncodingDescChild::ClassDecls(Box::new(class_decls)));
                }
                // domainsDecl, tagsDecl are more complex - skip for now
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(encoding_desc)
}

/// Parse an `<appInfo>` element from within another element.
pub(crate) fn parse_app_info_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AppInfo> {
    let mut app_info = AppInfo::default();

    // Extract attributes
    app_info.common.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("appInfo")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    app_info.children.push(AppInfoChild::Head(Box::new(head)));
                }
                "application" => {
                    let application =
                        parse_application_from_event(reader, child_attrs, child_empty)?;
                    app_info
                        .children
                        .push(AppInfoChild::Application(Box::new(application)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(app_info)
}

/// Parse an `<application>` element from within another element.
pub(crate) fn parse_application_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Application> {
    let mut application = Application::default();

    // Extract attributes
    application.common.extract_attributes(&mut attrs)?;
    application.datable.extract_attributes(&mut attrs)?;

    // Element-local attribute: @version
    extract_attr!(attrs, "version", string application.version);

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("application")?
        {
            match name.as_str() {
                "name" => {
                    let name_elem = super::parse_name_from_event(reader, child_attrs, child_empty)?;
                    application
                        .children
                        .push(ApplicationChild::Name(Box::new(name_elem)));
                }
                "ptr" => {
                    let ptr = super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                    application
                        .children
                        .push(ApplicationChild::Ptr(Box::new(ptr)));
                }
                "ref" => {
                    let ref_elem = super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                    application
                        .children
                        .push(ApplicationChild::Ref(Box::new(ref_elem)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    application.children.push(ApplicationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(application)
}

/// Parse a `<correction>` element from within another element.
pub(crate) fn parse_correction_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Correction> {
    let mut correction = Correction::default();

    // Extract attributes
    correction.common.extract_attributes(&mut attrs)?;
    correction.bibl.extract_attributes(&mut attrs)?;
    correction.data_pointing.extract_attributes(&mut attrs)?;
    correction.lang.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "method", correction.regular_method.method);

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("correction")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    correction
                        .children
                        .push(CorrectionChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    correction.children.push(CorrectionChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(correction)
}

/// Parse an `<interpretation>` element from within another element.
pub(crate) fn parse_interpretation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Interpretation> {
    let mut interpretation = Interpretation::default();

    // Extract attributes
    interpretation.common.extract_attributes(&mut attrs)?;
    interpretation.bibl.extract_attributes(&mut attrs)?;
    interpretation
        .data_pointing
        .extract_attributes(&mut attrs)?;
    interpretation.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("interpretation")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    interpretation
                        .children
                        .push(InterpretationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    interpretation
                        .children
                        .push(InterpretationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(interpretation)
}

/// Parse a `<normalization>` element from within another element.
pub(crate) fn parse_normalization_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Normalization> {
    let mut normalization = Normalization::default();

    // Extract attributes
    normalization.common.extract_attributes(&mut attrs)?;
    normalization.bibl.extract_attributes(&mut attrs)?;
    normalization.data_pointing.extract_attributes(&mut attrs)?;
    normalization.lang.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "method", normalization.regular_method.method);

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("normalization")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    normalization
                        .children
                        .push(NormalizationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    normalization
                        .children
                        .push(NormalizationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(normalization)
}

/// Parse a `<segmentation>` element from within another element.
pub(crate) fn parse_segmentation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Segmentation> {
    let mut segmentation = Segmentation::default();

    // Extract attributes
    segmentation.common.extract_attributes(&mut attrs)?;
    segmentation.bibl.extract_attributes(&mut attrs)?;
    segmentation.data_pointing.extract_attributes(&mut attrs)?;
    segmentation.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("segmentation")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    segmentation
                        .children
                        .push(SegmentationChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    segmentation
                        .children
                        .push(SegmentationChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(segmentation)
}

/// Parse a `<stdVals>` element from within another element.
pub(crate) fn parse_std_vals_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StdVals> {
    let mut std_vals = StdVals::default();

    // Extract attributes
    std_vals.common.extract_attributes(&mut attrs)?;
    std_vals.bibl.extract_attributes(&mut attrs)?;
    std_vals.data_pointing.extract_attributes(&mut attrs)?;
    std_vals.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("stdVals")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    std_vals.children.push(StdValsChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    std_vals.children.push(StdValsChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(std_vals)
}

/// Parse an `<editorialDecl>` element from within another element.
pub(crate) fn parse_editorial_decl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<EditorialDecl> {
    let mut editorial_decl = EditorialDecl::default();

    // Extract attributes
    editorial_decl.common.extract_attributes(&mut attrs)?;
    editorial_decl.bibl.extract_attributes(&mut attrs)?;
    editorial_decl
        .data_pointing
        .extract_attributes(&mut attrs)?;
    editorial_decl.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("editorialDecl")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::P(Box::new(p)));
                }
                "correction" => {
                    let correction = parse_correction_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Correction(Box::new(correction)));
                }
                "interpretation" => {
                    let interpretation =
                        parse_interpretation_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Interpretation(Box::new(interpretation)));
                }
                "normalization" => {
                    let normalization =
                        parse_normalization_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Normalization(Box::new(normalization)));
                }
                "segmentation" => {
                    let segmentation =
                        parse_segmentation_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::Segmentation(Box::new(segmentation)));
                }
                "stdVals" => {
                    let std_vals = parse_std_vals_from_event(reader, child_attrs, child_empty)?;
                    editorial_decl
                        .children
                        .push(EditorialDeclChild::StdVals(Box::new(std_vals)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(editorial_decl)
}

/// Parse a `<projectDesc>` element from within another element.
pub(crate) fn parse_project_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ProjectDesc> {
    let mut project_desc = ProjectDesc::default();

    // Extract attributes
    project_desc.common.extract_attributes(&mut attrs)?;
    project_desc.bibl.extract_attributes(&mut attrs)?;
    project_desc.data_pointing.extract_attributes(&mut attrs)?;
    project_desc.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("projectDesc")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    project_desc
                        .children
                        .push(ProjectDescChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    project_desc.children.push(ProjectDescChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(project_desc)
}

/// Parse a `<samplingDecl>` element from within another element.
pub(crate) fn parse_sampling_decl_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SamplingDecl> {
    let mut sampling_decl = SamplingDecl::default();

    // Extract attributes
    sampling_decl.common.extract_attributes(&mut attrs)?;
    sampling_decl.bibl.extract_attributes(&mut attrs)?;
    sampling_decl.data_pointing.extract_attributes(&mut attrs)?;
    sampling_decl.lang.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("samplingDecl")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    sampling_decl
                        .children
                        .push(SamplingDeclChild::Head(Box::new(head)));
                }
                "p" => {
                    let p = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    sampling_decl
                        .children
                        .push(SamplingDeclChild::P(Box::new(p)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(sampling_decl)
}

/// Parse a `<classDecls>` element from within another element.
pub(crate) fn parse_class_decls_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ClassDecls> {
    let mut class_decls = ClassDecls::default();

    // Extract attributes
    class_decls.common.extract_attributes(&mut attrs)?;
    class_decls.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("classDecls")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    class_decls
                        .children
                        .push(ClassDeclsChild::Head(Box::new(head)));
                }
                "taxonomy" => {
                    let taxonomy = parse_taxonomy_from_event(reader, child_attrs, child_empty)?;
                    class_decls
                        .children
                        .push(ClassDeclsChild::Taxonomy(Box::new(taxonomy)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(class_decls)
}

/// Parse a `<taxonomy>` element from within another element.
pub(crate) fn parse_taxonomy_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Taxonomy> {
    let mut taxonomy = Taxonomy::default();

    // Extract attributes
    taxonomy.common.extract_attributes(&mut attrs)?;
    taxonomy.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("taxonomy")?
        {
            match name.as_str() {
                "head" => {
                    let head = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    taxonomy.children.push(TaxonomyChild::Head(Box::new(head)));
                }
                "category" => {
                    let category = parse_category_from_event(reader, child_attrs, child_empty)?;
                    taxonomy
                        .children
                        .push(TaxonomyChild::Category(Box::new(category)));
                }
                "bibl" => {
                    let bibl = super::parse_bibl_from_event(reader, child_attrs, child_empty)?;
                    taxonomy.children.push(TaxonomyChild::Bibl(Box::new(bibl)));
                }
                "biblStruct" => {
                    let bibl_struct =
                        super::parse_bibl_struct_from_event(reader, child_attrs, child_empty)?;
                    taxonomy
                        .children
                        .push(TaxonomyChild::BiblStruct(Box::new(bibl_struct)));
                }
                "desc" => {
                    let desc = parse_desc_from_event(reader, child_attrs, child_empty)?;
                    taxonomy.children.push(TaxonomyChild::Desc(Box::new(desc)));
                }
                "taxonomy" => {
                    let nested_taxonomy =
                        parse_taxonomy_from_event(reader, child_attrs, child_empty)?;
                    taxonomy
                        .children
                        .push(TaxonomyChild::Taxonomy(Box::new(nested_taxonomy)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(taxonomy)
}

/// Parse a `<category>` element from within another element.
pub(crate) fn parse_category_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Category> {
    let mut category = Category::default();

    // Extract attributes
    category.common.extract_attributes(&mut attrs)?;
    category.bibl.extract_attributes(&mut attrs)?;

    // Read children if not an empty element
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("category")?
        {
            match name.as_str() {
                "desc" => {
                    let desc = parse_desc_from_event(reader, child_attrs, child_empty)?;
                    category.children.push(CategoryChild::Desc(Box::new(desc)));
                }
                "category" => {
                    // Recursive category
                    let nested = parse_category_from_event(reader, child_attrs, child_empty)?;
                    category
                        .children
                        .push(CategoryChild::Category(Box::new(nested)));
                }
                "label" => {
                    let label =
                        super::super::parse_label_from_event(reader, child_attrs, child_empty)?;
                    category
                        .children
                        .push(CategoryChild::Label(Box::new(label)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(category)
}

/// Parse an `<altId>` element from within another element.
pub(crate) fn parse_alt_id_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AltId> {
    let mut alt_id = AltId::default();

    // Extract attributes
    alt_id.common.extract_attributes(&mut attrs)?;
    alt_id.bibl.extract_attributes(&mut attrs)?;

    // altId can contain text
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("altId")? {
            if !text.is_empty() {
                alt_id.children.push(AltIdChild::Text(text));
            }
        }
    }

    Ok(alt_id)
}

/// Parse a `<catRel>` element from within another element.
pub(crate) fn parse_cat_rel_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CatRel> {
    use super::super::extract_attr;

    let mut cat_rel = CatRel::default();

    // Extract attributes
    cat_rel.authorized.extract_attributes(&mut attrs)?;
    cat_rel.basic.extract_attributes(&mut attrs)?;
    cat_rel.bibl.extract_attributes(&mut attrs)?;
    cat_rel.labelled.extract_attributes(&mut attrs)?;
    cat_rel.linking.extract_attributes(&mut attrs)?;
    cat_rel.n_number_like.extract_attributes(&mut attrs)?;
    cat_rel.responsibility.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "type", string cat_rel.r#type);

    // catRel can contain label and desc children
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("catRel")? {
            match name.as_str() {
                "label" => {
                    let label =
                        super::super::parse_label_from_event(reader, child_attrs, child_empty)?;
                    cat_rel.children.push(CatRelChild::Label(Box::new(label)));
                }
                "desc" => {
                    let desc = parse_desc_from_event(reader, child_attrs, child_empty)?;
                    cat_rel.children.push(CatRelChild::Desc(Box::new(desc)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(cat_rel)
}

/// Parse a `<desc>` element from within another element.
pub(crate) fn parse_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Desc> {
    let mut desc = Desc::default();

    // Extract attributes
    desc.common.extract_attributes(&mut attrs)?;
    desc.facsimile.extract_attributes(&mut attrs)?;
    desc.lang.extract_attributes(&mut attrs)?;
    desc.source.extract_attributes(&mut attrs)?;

    // desc can contain mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("desc")? {
            match content {
                MixedContent::Text(text) => {
                    desc.children.push(DescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "rend" => {
                            let rend = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            desc.children.push(DescChild::Rend(Box::new(rend)));
                        }
                        "lb" => {
                            let lb = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            desc.children.push(DescChild::Lb(Box::new(lb)));
                        }
                        "ref" => {
                            let ref_elem =
                                super::parse_ref_from_event(reader, child_attrs, child_empty)?;
                            desc.children.push(DescChild::Ref(Box::new(ref_elem)));
                        }
                        "ptr" => {
                            let ptr =
                                super::parse_ptr_from_event(reader, child_attrs, child_empty)?;
                            desc.children.push(DescChild::Ptr(Box::new(ptr)));
                        }
                        _ => {
                            // Skip unknown children
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(desc)
}

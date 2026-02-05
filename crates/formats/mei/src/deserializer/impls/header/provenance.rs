//! Provenance, history, watermark, and type description elements.
//!
//! These elements describe the history, ownership, watermarks, and typographic features
//! of manuscript and printed materials.

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    AccMat, AccMatChild, Acquisition, AcquisitionChild, AddDesc, AddDescChild, ExhibHist,
    ExhibHistChild, Provenance, ProvenanceChild, TypeDesc, TypeDescChild, TypeNote, TypeNoteChild,
    Watermark, WatermarkChild, WatermarkDesc, WatermarkDescChild, WatermarkList,
    WatermarkListChild,
};

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for Provenance {
    fn element_name() -> &'static str {
        "provenance"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_provenance_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Acquisition {
    fn element_name() -> &'static str {
        "acquisition"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_acquisition_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for ExhibHist {
    fn element_name() -> &'static str {
        "exhibHist"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_exhib_hist_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for AccMat {
    fn element_name() -> &'static str {
        "accMat"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_acc_mat_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for AddDesc {
    fn element_name() -> &'static str {
        "addDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_add_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Watermark {
    fn element_name() -> &'static str {
        "watermark"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_watermark_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for WatermarkDesc {
    fn element_name() -> &'static str {
        "watermarkDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_watermark_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for WatermarkList {
    fn element_name() -> &'static str {
        "watermarkList"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_watermark_list_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TypeDesc {
    fn element_name() -> &'static str {
        "typeDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_type_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TypeNote {
    fn element_name() -> &'static str {
        "typeNote"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_type_note_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Parse functions
// ============================================================================

/// Parse a `<provenance>` element.
pub(crate) fn parse_provenance_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Provenance> {
    let mut elem = Provenance::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.datable.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("provenance")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(ProvenanceChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_provenance_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_provenance_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<ProvenanceChild>> {
    Ok(Some(match name {
        "head" => ProvenanceChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => ProvenanceChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => ProvenanceChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => ProvenanceChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => ProvenanceChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => ProvenanceChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => ProvenanceChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => ProvenanceChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => ProvenanceChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => ProvenanceChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => ProvenanceChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => ProvenanceChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => ProvenanceChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => ProvenanceChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => ProvenanceChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => ProvenanceChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => ProvenanceChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => ProvenanceChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => ProvenanceChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => ProvenanceChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => ProvenanceChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => ProvenanceChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => ProvenanceChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => ProvenanceChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => ProvenanceChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => ProvenanceChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => ProvenanceChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => ProvenanceChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => ProvenanceChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => ProvenanceChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => ProvenanceChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => ProvenanceChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => ProvenanceChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        _ => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            return Ok(None);
        }
    }))
}

/// Parse a `<acquisition>` element.
pub(crate) fn parse_acquisition_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Acquisition> {
    let mut elem = Acquisition::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.datable.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("acquisition")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(AcquisitionChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_acquisition_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_acquisition_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<AcquisitionChild>> {
    Ok(Some(match name {
        "head" => AcquisitionChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => AcquisitionChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => AcquisitionChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => AcquisitionChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => AcquisitionChild::BiblStruct(Box::new(
            super::parse_bibl_struct_from_event(reader, child_attrs, child_empty)?,
        )),
        "annot" => AcquisitionChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => AcquisitionChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => AcquisitionChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => AcquisitionChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => AcquisitionChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => AcquisitionChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => AcquisitionChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => AcquisitionChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => AcquisitionChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => AcquisitionChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => AcquisitionChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => AcquisitionChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => AcquisitionChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => AcquisitionChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => AcquisitionChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => AcquisitionChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => AcquisitionChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => AcquisitionChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => AcquisitionChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => AcquisitionChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => AcquisitionChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => AcquisitionChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => AcquisitionChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => AcquisitionChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => AcquisitionChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => AcquisitionChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => AcquisitionChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => AcquisitionChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        _ => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            return Ok(None);
        }
    }))
}

/// Parse a `<exhibHist>` element.
pub(crate) fn parse_exhib_hist_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ExhibHist> {
    let mut elem = ExhibHist::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.datable.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("exhibHist")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(ExhibHistChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_exhib_hist_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_exhib_hist_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<ExhibHistChild>> {
    Ok(Some(match name {
        "head" => ExhibHistChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => ExhibHistChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => ExhibHistChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => ExhibHistChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => ExhibHistChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => ExhibHistChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => ExhibHistChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => ExhibHistChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => ExhibHistChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => ExhibHistChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => ExhibHistChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => ExhibHistChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => ExhibHistChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => ExhibHistChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => ExhibHistChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => ExhibHistChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => ExhibHistChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => ExhibHistChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => ExhibHistChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => ExhibHistChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => ExhibHistChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => ExhibHistChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => ExhibHistChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => ExhibHistChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => ExhibHistChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => ExhibHistChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => ExhibHistChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => ExhibHistChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => ExhibHistChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => ExhibHistChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => ExhibHistChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => ExhibHistChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => ExhibHistChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        _ => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            return Ok(None);
        }
    }))
}

/// Parse an `<accMat>` element.
pub(crate) fn parse_acc_mat_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AccMat> {
    let mut elem = AccMat::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("accMat")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(AccMatChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_acc_mat_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_acc_mat_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<AccMatChild>> {
    Ok(Some(match name {
        "head" => AccMatChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => AccMatChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => AccMatChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => AccMatChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => AccMatChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => AccMatChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => AccMatChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => AccMatChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => AccMatChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => AccMatChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => AccMatChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => AccMatChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => AccMatChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => AccMatChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => AccMatChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => AccMatChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => AccMatChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => AccMatChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => AccMatChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => AccMatChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => AccMatChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => AccMatChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => AccMatChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => AccMatChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => AccMatChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => AccMatChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => AccMatChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => AccMatChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => AccMatChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => AccMatChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => AccMatChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => AccMatChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => AccMatChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        _ => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            return Ok(None);
        }
    }))
}

/// Parse an `<addDesc>` element.
pub(crate) fn parse_add_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AddDesc> {
    let mut elem = AddDesc::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("addDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(AddDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_add_desc_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_add_desc_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<AddDescChild>> {
    Ok(Some(match name {
        "head" => AddDescChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => AddDescChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => AddDescChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => AddDescChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => AddDescChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => AddDescChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => AddDescChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => AddDescChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => AddDescChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => AddDescChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => AddDescChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => AddDescChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => AddDescChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => AddDescChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => AddDescChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => AddDescChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => AddDescChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => AddDescChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => AddDescChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => AddDescChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => AddDescChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => AddDescChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => AddDescChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => AddDescChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => AddDescChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => AddDescChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => AddDescChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => AddDescChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => AddDescChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => AddDescChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => AddDescChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => AddDescChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => AddDescChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        _ => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            return Ok(None);
        }
    }))
}

/// Parse a `<watermark>` element.
pub(crate) fn parse_watermark_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Watermark> {
    let mut elem = Watermark::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("watermark")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(WatermarkChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_watermark_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_watermark_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<WatermarkChild>> {
    Ok(Some(match name {
        "date" => WatermarkChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => WatermarkChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => WatermarkChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => WatermarkChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => WatermarkChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => WatermarkChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => WatermarkChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => WatermarkChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => WatermarkChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => WatermarkChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => WatermarkChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => WatermarkChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => WatermarkChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => WatermarkChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => WatermarkChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => WatermarkChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => WatermarkChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => WatermarkChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => WatermarkChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => WatermarkChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => WatermarkChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => WatermarkChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => WatermarkChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => WatermarkChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => WatermarkChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => WatermarkChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        _ => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            return Ok(None);
        }
    }))
}

/// Parse a `<watermarkDesc>` element.
pub(crate) fn parse_watermark_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<WatermarkDesc> {
    let mut elem = WatermarkDesc::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("watermarkDesc")?
        {
            match name.as_str() {
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(WatermarkDescChild::Head(Box::new(child)));
                }
                "p" => {
                    let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(WatermarkDescChild::P(Box::new(child)));
                }
                "watermark" => {
                    let child = parse_watermark_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(WatermarkDescChild::Watermark(Box::new(child)));
                }
                "watermarkList" => {
                    let child = parse_watermark_list_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(WatermarkDescChild::WatermarkList(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse a `<watermarkList>` element.
pub(crate) fn parse_watermark_list_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<WatermarkList> {
    let mut elem = WatermarkList::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.edit.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("watermarkList")?
        {
            match name.as_str() {
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(WatermarkListChild::Head(Box::new(child)));
                }
                "watermark" => {
                    let child = parse_watermark_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(WatermarkListChild::Watermark(Box::new(child)));
                }
                "annot" => {
                    let child = super::parse_annot_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(WatermarkListChild::Annot(Box::new(child)));
                }
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(elem)
}

/// Parse a `<typeDesc>` element.
pub(crate) fn parse_type_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TypeDesc> {
    let mut elem = TypeDesc::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("typeDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(TypeDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_type_desc_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_type_desc_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<TypeDescChild>> {
    Ok(Some(match name {
        "head" => TypeDescChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => TypeDescChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "typeNote" => TypeDescChild::TypeNote(Box::new(parse_type_note_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => TypeDescChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => TypeDescChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => TypeDescChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => TypeDescChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => TypeDescChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => TypeDescChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => TypeDescChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => TypeDescChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => TypeDescChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => TypeDescChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => TypeDescChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => TypeDescChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => TypeDescChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => TypeDescChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => TypeDescChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => TypeDescChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => TypeDescChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => TypeDescChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => TypeDescChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => TypeDescChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => TypeDescChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => TypeDescChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => TypeDescChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => TypeDescChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => TypeDescChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => TypeDescChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => TypeDescChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => TypeDescChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => TypeDescChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => TypeDescChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => TypeDescChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        _ => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            return Ok(None);
        }
    }))
}

/// Parse a `<typeNote>` element.
pub(crate) fn parse_type_note_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TypeNote> {
    let mut elem = TypeNote::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("typeNote")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(TypeNoteChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_type_note_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_type_note_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<TypeNoteChild>> {
    Ok(Some(match name {
        "head" => TypeNoteChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => TypeNoteChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => TypeNoteChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => TypeNoteChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => TypeNoteChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => TypeNoteChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => TypeNoteChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => TypeNoteChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => TypeNoteChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => TypeNoteChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => TypeNoteChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => TypeNoteChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => TypeNoteChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => TypeNoteChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => TypeNoteChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => TypeNoteChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => TypeNoteChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => TypeNoteChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => TypeNoteChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => TypeNoteChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => TypeNoteChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => TypeNoteChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => TypeNoteChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => TypeNoteChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => TypeNoteChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => TypeNoteChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => TypeNoteChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => TypeNoteChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => TypeNoteChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => TypeNoteChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => TypeNoteChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => TypeNoteChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => TypeNoteChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        _ => {
            if !child_empty {
                reader.skip_to_end(name)?;
            }
            return Ok(None);
        }
    }))
}

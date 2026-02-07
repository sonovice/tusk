//! Deserializer implementations for bibliographic and codicological MEI elements.
//!
//! This module contains implementations for:
//! - ExtData, AvFile, Cutout, Bifolium, Folium (codicology)
//! - Analytic, Monogr, Series (bibliography)
//! - Patch (codicology helper)
//! - Catchwords, Signatures, SignifLet (manuscript description)
//! - Actor, CatRel, Context (misc elements)
//!
//! Note: Desc is implemented in header/encoding_desc.rs

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::att::{
    AttBibl, AttBifoliumSurfaces, AttComponentType, AttDataPointing, AttDimensions, AttEvidence,
    AttFoliumSurfaces, AttMeasurement, AttRecordType, AttSignifLetAnl, AttSignifLetGes,
    AttSignifLetLog, AttSignifLetVis, AttTargetEval, AttTrans,
};
use tusk_model::elements::{
    Actor, ActorChild, Analytic, AnalyticChild, AvFile, AvFileChild, Bifolium, BifoliumChild,
    CatRel, CatRelChild, Catchwords, CatchwordsChild, Context, ContextChild, Cutout, CutoutChild,
    ExtData, ExtDataChild, Folium, FoliumChild, Monogr, MonogrChild, Patch, PatchChild, Series,
    SeriesChild, Signatures, SignaturesChild, SignifLet, SignifLetChild,
};

use super::header::parse_desc_from_event;
use super::{
    extract_attr, parse_bibl_scope_from_event, parse_clip_from_event, parse_contributor_from_event,
    parse_creator_from_event, parse_edition_from_event, parse_editor_from_event,
    parse_extent_from_event, parse_funder_from_event, parse_identifier_from_event,
    parse_label_from_event, parse_lb_from_event, parse_resp_stmt_from_event,
    parse_sponsor_from_event, parse_title_from_event,
};

// ============================================================================
// Attribute class implementations
// ============================================================================

// Note: AttSource is implemented in text.rs

// ============================================================================
// ExtData element implementation
// ============================================================================

impl MeiDeserialize for ExtData {
    fn element_name() -> &'static str {
        "extData"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = ExtData::default();

        // Extract attributes
        elem.basic.extract_attributes(&mut attrs)?;
        elem.labelled.extract_attributes(&mut attrs)?;
        elem.responsibility.extract_attributes(&mut attrs)?;
        elem.typed.extract_attributes(&mut attrs)?;
        elem.whitespace.extract_attributes(&mut attrs)?;
        elem.pointing.extract_attributes(&mut attrs)?;
        elem.internet_media.extract_attributes(&mut attrs)?;

        // Parse text content
        if !is_empty {
            if let Some(text) = reader.read_text_until_end("extData")? {
                if !text.trim().is_empty() {
                    elem.children.push(ExtDataChild::Text(text));
                }
            }
        }

        Ok(elem)
    }
}

// ============================================================================
// AvFile element implementation
// ============================================================================

impl MeiDeserialize for AvFile {
    fn element_name() -> &'static str {
        "avFile"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut elem = AvFile::default();

        // Extract attributes
        elem.common.extract_attributes(&mut attrs)?;
        elem.bibl.extract_attributes(&mut attrs)?;
        elem.internet_media.extract_attributes(&mut attrs)?;
        elem.facsimile.extract_attributes(&mut attrs)?;
        elem.metadata_pointing.extract_attributes(&mut attrs)?;
        elem.pointing.extract_attributes(&mut attrs)?;

        // Parse children (can contain clip elements)
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("avFile")?
            {
                match name.as_str() {
                    "clip" => {
                        let clip = parse_clip_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(AvFileChild::Clip(Box::new(clip)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "avFile", child_empty)?;
                    }
                }
            }
        }

        Ok(elem)
    }
}

// ============================================================================
// Patch element implementation (needed for Folium and Bifolium)
// ============================================================================

pub(crate) fn parse_patch_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Patch> {
    let mut elem = Patch::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.evidence.extract_attributes(&mut attrs)?;
    elem.measurement.extract_attributes(&mut attrs)?;
    elem.trans.extract_attributes(&mut attrs)?;
    elem.xy.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "attached.to", string elem.attached_to);
    extract_attr!(attrs, "attached.by", string elem.attached_by);

    // Parse children (can contain bifolium, folium)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("patch")? {
            match name.as_str() {
                "bifolium" => {
                    let child = parse_bifolium_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(PatchChild::Bifolium(Box::new(child)));
                }
                "folium" => {
                    let child = parse_folium_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(PatchChild::Folium(Box::new(child)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "patch", child_empty)?;
                }
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Patch {
    fn element_name() -> &'static str {
        "patch"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_patch_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Cutout element implementation
// ============================================================================

pub(crate) fn parse_cutout_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Cutout> {
    let mut elem = Cutout::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.dimensions.extract_attributes(&mut attrs)?;
    elem.evidence.extract_attributes(&mut attrs)?;
    elem.measurement.extract_attributes(&mut attrs)?;
    elem.trans.extract_attributes(&mut attrs)?;
    elem.xy.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "removed.from", string elem.removed_from);
    extract_attr!(attrs, "removed.by", string elem.removed_by);

    // Parse children (can contain bifolium, folium)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("cutout")? {
            match name.as_str() {
                "bifolium" => {
                    let child = parse_bifolium_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(CutoutChild::Bifolium(Box::new(child)));
                }
                "folium" => {
                    let child = parse_folium_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(CutoutChild::Folium(Box::new(child)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "cutout", child_empty)?;
                }
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Cutout {
    fn element_name() -> &'static str {
        "cutout"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_cutout_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Folium element implementation
// ============================================================================

pub(crate) fn parse_folium_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Folium> {
    let mut elem = Folium::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.dimensions.extract_attributes(&mut attrs)?;
    elem.measurement.extract_attributes(&mut attrs)?;
    elem.folium_surfaces.extract_attributes(&mut attrs)?;

    // Parse children (can contain cutout, patch)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("folium")? {
            match name.as_str() {
                "cutout" => {
                    let child = parse_cutout_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(FoliumChild::Cutout(Box::new(child)));
                }
                "patch" => {
                    let child = parse_patch_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(FoliumChild::Patch(Box::new(child)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "folium", child_empty)?;
                }
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Folium {
    fn element_name() -> &'static str {
        "folium"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_folium_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Bifolium element implementation
// ============================================================================

pub(crate) fn parse_bifolium_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Bifolium> {
    let mut elem = Bifolium::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.dimensions.extract_attributes(&mut attrs)?;
    elem.measurement.extract_attributes(&mut attrs)?;
    elem.bifolium_surfaces.extract_attributes(&mut attrs)?;

    // Parse children (can contain restore, damage, bifolium, patch, del, folium, add, gap, cutout)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("bifolium")?
        {
            match name.as_str() {
                "restore" => {
                    let child = tusk_model::elements::Restore::from_mei_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(BifoliumChild::Restore(Box::new(child)));
                }
                "damage" => {
                    let child = tusk_model::elements::Damage::from_mei_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(BifoliumChild::Damage(Box::new(child)));
                }
                "bifolium" => {
                    let child = parse_bifolium_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(BifoliumChild::Bifolium(Box::new(child)));
                }
                "patch" => {
                    let child = parse_patch_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(BifoliumChild::Patch(Box::new(child)));
                }
                "del" => {
                    let child = tusk_model::elements::Del::from_mei_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(BifoliumChild::Del(Box::new(child)));
                }
                "folium" => {
                    let child = parse_folium_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(BifoliumChild::Folium(Box::new(child)));
                }
                "add" => {
                    let child = tusk_model::elements::Add::from_mei_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(BifoliumChild::Add(Box::new(child)));
                }
                "gap" => {
                    let child = tusk_model::elements::Gap::from_mei_event(
                        reader,
                        child_attrs,
                        child_empty,
                    )?;
                    elem.children.push(BifoliumChild::Gap(Box::new(child)));
                }
                "cutout" => {
                    let child = parse_cutout_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(BifoliumChild::Cutout(Box::new(child)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "bifolium", child_empty)?;
                }
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Bifolium {
    fn element_name() -> &'static str {
        "bifolium"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_bifolium_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Analytic element implementation
// ============================================================================

pub(crate) fn parse_analytic_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Analytic> {
    let mut elem = Analytic::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.component_type.extract_attributes(&mut attrs)?;
    elem.data_pointing.extract_attributes(&mut attrs)?;
    elem.pointing.extract_attributes(&mut attrs)?;
    elem.record_type.extract_attributes(&mut attrs)?;
    elem.target_eval.extract_attributes(&mut attrs)?;

    // Parse children
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("analytic")?
        {
            match name.as_str() {
                "contributor" => {
                    let child = parse_contributor_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(AnalyticChild::Contributor(Box::new(child)));
                }
                "identifier" => {
                    let child = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(AnalyticChild::Identifier(Box::new(child)));
                }
                "title" => {
                    let child = parse_title_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(AnalyticChild::Title(Box::new(child)));
                }
                "funder" => {
                    let child = parse_funder_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(AnalyticChild::Funder(Box::new(child)));
                }
                "editor" => {
                    let child = parse_editor_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(AnalyticChild::Editor(Box::new(child)));
                }
                "respStmt" => {
                    let child = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(AnalyticChild::RespStmt(Box::new(child)));
                }
                "biblScope" => {
                    let child = parse_bibl_scope_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(AnalyticChild::BiblScope(Box::new(child)));
                }
                "sponsor" => {
                    let child = parse_sponsor_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(AnalyticChild::Sponsor(Box::new(child)));
                }
                "creator" => {
                    let child = parse_creator_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(AnalyticChild::Creator(Box::new(child)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "analytic", child_empty)?;
                }
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Analytic {
    fn element_name() -> &'static str {
        "analytic"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_analytic_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Monogr element implementation
// ============================================================================

pub(crate) fn parse_monogr_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Monogr> {
    let mut elem = Monogr::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.data_pointing.extract_attributes(&mut attrs)?;
    elem.pointing.extract_attributes(&mut attrs)?;
    elem.record_type.extract_attributes(&mut attrs)?;
    elem.target_eval.extract_attributes(&mut attrs)?;

    // Parse children
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("monogr")? {
            match name.as_str() {
                "editor" => {
                    let child = parse_editor_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::Editor(Box::new(child)));
                }
                "corpName" => {
                    // Skip corpName for now - would need dedicated parser
                    if !child_empty {
                        reader.skip_to_end("corpName")?;
                    }
                }
                "identifier" => {
                    let child = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::Identifier(Box::new(child)));
                }
                "funder" => {
                    let child = parse_funder_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::Funder(Box::new(child)));
                }
                "sponsor" => {
                    let child = parse_sponsor_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::Sponsor(Box::new(child)));
                }
                "title" => {
                    let child = parse_title_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::Title(Box::new(child)));
                }
                "extent" => {
                    let child = parse_extent_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::Extent(Box::new(child)));
                }
                "annot" => {
                    // Skip annot for now - would need dedicated parser
                    if !child_empty {
                        reader.skip_to_end("annot")?;
                    }
                }
                "creator" => {
                    let child = parse_creator_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::Creator(Box::new(child)));
                }
                "edition" => {
                    let child = parse_edition_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::Edition(Box::new(child)));
                }
                "imprint" => {
                    // Skip imprint for now - would need dedicated parser
                    if !child_empty {
                        reader.skip_to_end("imprint")?;
                    }
                }
                "contributor" => {
                    let child = parse_contributor_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(MonogrChild::Contributor(Box::new(child)));
                }
                "respStmt" => {
                    let child = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(MonogrChild::RespStmt(Box::new(child)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "monogr", child_empty)?;
                }
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Monogr {
    fn element_name() -> &'static str {
        "monogr"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_monogr_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Series element implementation
// ============================================================================

pub(crate) fn parse_series_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Series> {
    let mut elem = Series::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("series")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        elem.children.push(SeriesChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "respStmt" => {
                        let child = parse_resp_stmt_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SeriesChild::RespStmt(Box::new(child)));
                    }
                    "identifier" => {
                        let child = parse_identifier_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SeriesChild::Identifier(Box::new(child)));
                    }
                    "editor" => {
                        let child = parse_editor_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SeriesChild::Editor(Box::new(child)));
                    }
                    "lb" => {
                        let child = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SeriesChild::Lb(Box::new(child)));
                    }
                    "title" => {
                        let child = parse_title_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SeriesChild::Title(Box::new(child)));
                    }
                    "extent" => {
                        let child = parse_extent_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SeriesChild::Extent(Box::new(child)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "series", child_empty)?;
                    }
                },
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Series {
    fn element_name() -> &'static str {
        "series"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_series_from_event(reader, attrs, is_empty)
    }
}

// Note: Desc implementation is in header/encoding_desc.rs

// ============================================================================
// Catchwords element implementation
// ============================================================================

pub(crate) fn parse_catchwords_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Catchwords> {
    let mut elem = Catchwords::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("catchwords")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        elem.children.push(CatchwordsChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "lb" => {
                        let child = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(CatchwordsChild::Lb(Box::new(child)));
                    }
                    "signatures" => {
                        let child = parse_signatures_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(CatchwordsChild::Signatures(Box::new(child)));
                    }
                    "catchwords" => {
                        let child = parse_catchwords_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(CatchwordsChild::Catchwords(Box::new(child)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "catchwords", child_empty)?;
                    }
                },
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Catchwords {
    fn element_name() -> &'static str {
        "catchwords"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_catchwords_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Signatures element implementation
// ============================================================================

pub(crate) fn parse_signatures_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Signatures> {
    let mut elem = Signatures::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("signatures")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        elem.children.push(SignaturesChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "lb" => {
                        let child = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SignaturesChild::Lb(Box::new(child)));
                    }
                    "catchwords" => {
                        let child = parse_catchwords_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(SignaturesChild::Catchwords(Box::new(child)));
                    }
                    "signatures" => {
                        let child = parse_signatures_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(SignaturesChild::Signatures(Box::new(child)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "signatures", child_empty)?;
                    }
                },
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Signatures {
    fn element_name() -> &'static str {
        "signatures"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_signatures_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// SignifLet element implementation
// ============================================================================

pub(crate) fn parse_signif_let_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SignifLet> {
    let mut elem = SignifLet::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.signif_let_anl.extract_attributes(&mut attrs)?;
    elem.signif_let_ges.extract_attributes(&mut attrs)?;
    elem.signif_let_log.extract_attributes(&mut attrs)?;
    elem.signif_let_vis.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("signifLet")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        elem.children.push(SignifLetChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "lb" => {
                        let child = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(SignifLetChild::Lb(Box::new(child)));
                    }
                    "catchwords" => {
                        let child = parse_catchwords_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(SignifLetChild::Catchwords(Box::new(child)));
                    }
                    "signatures" => {
                        let child = parse_signatures_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(SignifLetChild::Signatures(Box::new(child)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "signifLet", child_empty)?;
                    }
                },
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for SignifLet {
    fn element_name() -> &'static str {
        "signifLet"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_signif_let_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Actor element implementation
// ============================================================================

pub(crate) fn parse_actor_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Actor> {
    let mut elem = Actor::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("actor")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        elem.children.push(ActorChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "lb" => {
                        let child = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ActorChild::Lb(Box::new(child)));
                    }
                    "catchwords" => {
                        let child = parse_catchwords_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ActorChild::Catchwords(Box::new(child)));
                    }
                    "signatures" => {
                        let child = parse_signatures_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ActorChild::Signatures(Box::new(child)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "actor", child_empty)?;
                    }
                },
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Actor {
    fn element_name() -> &'static str {
        "actor"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_actor_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// CatRel element implementation
// ============================================================================

pub(crate) fn parse_cat_rel_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<CatRel> {
    let mut elem = CatRel::default();

    // Extract attributes
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.basic.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.labelled.extract_attributes(&mut attrs)?;
    elem.linking.extract_attributes(&mut attrs)?;
    elem.n_number_like.extract_attributes(&mut attrs)?;
    elem.responsibility.extract_attributes(&mut attrs)?;
    extract_attr!(attrs, "type", string elem.r#type);

    // Parse children (label and desc)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("catRel")? {
            match name.as_str() {
                "label" => {
                    let child = parse_label_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(CatRelChild::Label(Box::new(child)));
                }
                "desc" => {
                    let child = parse_desc_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(CatRelChild::Desc(Box::new(child)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "catRel", child_empty)?;
                }
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for CatRel {
    fn element_name() -> &'static str {
        "catRel"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_cat_rel_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Context element implementation
// ============================================================================

pub(crate) fn parse_context_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Context> {
    let mut elem = Context::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content (text and child elements)
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("context")? {
            match content {
                MixedContent::Text(text) => {
                    if !text.trim().is_empty() {
                        elem.children.push(ContextChild::Text(text));
                    }
                }
                MixedContent::Element(name, child_attrs, child_empty) => match name.as_str() {
                    "lb" => {
                        let child = parse_lb_from_event(reader, child_attrs, child_empty)?;
                        elem.children.push(ContextChild::Lb(Box::new(child)));
                    }
                    "catchwords" => {
                        let child = parse_catchwords_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(ContextChild::Catchwords(Box::new(child)));
                    }
                    "signatures" => {
                        let child = parse_signatures_from_event(reader, child_attrs, child_empty)?;
                        elem.children
                            .push(ContextChild::Signatures(Box::new(child)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "context", child_empty)?;
                    }
                },
            }
        }
    }

    Ok(elem)
}

impl MeiDeserialize for Context {
    fn element_name() -> &'static str {
        "context"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_context_from_event(reader, attrs, is_empty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_data_deserializes_empty() {
        let xml = r#"<extData/>"#;
        let elem = ExtData::from_mei_str(xml).expect("should deserialize");
        assert!(elem.basic.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn ext_data_deserializes_with_text() {
        let xml = r#"<extData xml:id="ed1" mimetype="text/plain">Some external data</extData>"#;
        let elem = ExtData::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.basic.xml_id, Some("ed1".to_string()));
        assert_eq!(elem.internet_media.mimetype, Some("text/plain".to_string()));
        assert_eq!(elem.children.len(), 1);
    }

    #[test]
    fn av_file_deserializes_empty() {
        let xml = r#"<avFile/>"#;
        let elem = AvFile::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
        assert!(elem.children.is_empty());
    }

    #[test]
    fn av_file_deserializes_with_attrs() {
        let xml = r#"<avFile xml:id="av1" target="audio.mp3" mimetype="audio/mpeg"/>"#;
        let elem = AvFile::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("av1".to_string()));
        assert_eq!(elem.internet_media.mimetype, Some("audio/mpeg".to_string()));
    }

    #[test]
    fn cutout_deserializes_empty() {
        let xml = r#"<cutout/>"#;
        let elem = Cutout::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn cutout_deserializes_with_attrs() {
        let xml = r#"<cutout xml:id="c1" removed.from="top" removed.by="scissors"/>"#;
        let elem = Cutout::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("c1".to_string()));
        assert_eq!(elem.removed_from, Some("top".to_string()));
        assert_eq!(elem.removed_by, Some("scissors".to_string()));
    }

    #[test]
    fn folium_deserializes_empty() {
        let xml = r#"<folium/>"#;
        let elem = Folium::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn bifolium_deserializes_empty() {
        let xml = r#"<bifolium/>"#;
        let elem = Bifolium::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn analytic_deserializes_empty() {
        let xml = r#"<analytic/>"#;
        let elem = Analytic::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn monogr_deserializes_empty() {
        let xml = r#"<monogr/>"#;
        let elem = Monogr::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn series_deserializes_empty() {
        let xml = r#"<series/>"#;
        let elem = Series::from_mei_str(xml).expect("should deserialize");
        assert!(elem.common.xml_id.is_none());
    }

    #[test]
    fn series_deserializes_with_text() {
        let xml = r#"<series xml:id="s1">Series Name</series>"#;
        let elem = Series::from_mei_str(xml).expect("should deserialize");
        assert_eq!(elem.common.xml_id, Some("s1".to_string()));
        assert!(!elem.children.is_empty());
    }

    // Note: Desc tests are in header/encoding_desc.rs
}

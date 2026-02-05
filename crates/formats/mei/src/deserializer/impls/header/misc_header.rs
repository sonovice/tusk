//! Deserializer implementations for miscellaneous header elements.
//!
//! Contains: Genre, Audience, TextLang, Heraldry, Inscription, SecFolio, SpecRepro,
//! Recipient, TreatHist, TreatSched, PgDesc

use super::super::extract_attr;
use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    Abbr, AnchoredText, Audience, AudienceChild, Curve, EventList, Expan, Genre, GenreChild,
    Heraldry, HeraldryChild, Inscription, InscriptionChild, Line, PgDesc, PgDescChild, Q, Quote,
    Recipient, RecipientChild, SecFolio, SecFolioChild, SpecRepro, SpecReproChild, Stack, Stamp,
    Table, TextLang, TextLangChild, TreatHist, TreatHistChild, TreatSched, TreatSchedChild,
};

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for Genre {
    fn element_name() -> &'static str {
        "genre"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_genre_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Audience {
    fn element_name() -> &'static str {
        "audience"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_audience_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TextLang {
    fn element_name() -> &'static str {
        "textLang"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_text_lang_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Heraldry {
    fn element_name() -> &'static str {
        "heraldry"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_heraldry_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Inscription {
    fn element_name() -> &'static str {
        "inscription"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_inscription_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SecFolio {
    fn element_name() -> &'static str {
        "secFolio"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_sec_folio_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SpecRepro {
    fn element_name() -> &'static str {
        "specRepro"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_spec_repro_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Recipient {
    fn element_name() -> &'static str {
        "recipient"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_recipient_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TreatHist {
    fn element_name() -> &'static str {
        "treatHist"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_treat_hist_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for TreatSched {
    fn element_name() -> &'static str {
        "treatSched"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_treat_sched_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for PgDesc {
    fn element_name() -> &'static str {
        "pgDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_pg_desc_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Parse functions
// ============================================================================

/// Parse a `<genre>` element.
pub(crate) fn parse_genre_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Genre> {
    let mut elem = Genre::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("genre")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(GenreChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) = parse_genre_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_genre_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<GenreChild>> {
    Ok(Some(match name {
        "date" => GenreChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => GenreChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => GenreChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => GenreChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => GenreChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => GenreChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => GenreChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => GenreChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => GenreChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => GenreChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogFeat" => GenreChild::GeogFeat(Box::new(super::parse_geog_feat_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => GenreChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => GenreChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => GenreChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => GenreChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => GenreChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => GenreChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => GenreChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => GenreChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => GenreChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => GenreChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => GenreChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => GenreChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => GenreChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => GenreChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => GenreChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => GenreChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => GenreChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => GenreChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => GenreChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => GenreChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => GenreChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "symbol" => GenreChild::Symbol(Box::new(super::super::control::parse_symbol_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "q" => GenreChild::Q(Box::new(Q::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "extent" => GenreChild::Extent(Box::new(super::super::parse_extent_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "relationList" => GenreChild::RelationList(Box::new(
            super::parse_relation_list_from_event(reader, child_attrs, child_empty)?,
        )),
        "relation" => GenreChild::Relation(Box::new(super::parse_relation_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "periodName" => GenreChild::PeriodName(Box::new(super::parse_period_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "styleName" => GenreChild::StyleName(Box::new(super::parse_style_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "abbr" => GenreChild::Abbr(Box::new(Abbr::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "expan" => GenreChild::Expan(Box::new(Expan::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "stack" => GenreChild::Stack(Box::new(Stack::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "postBox" => GenreChild::PostBox(Box::new(super::parse_post_box_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "postCode" => GenreChild::PostCode(Box::new(super::parse_post_code_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "street" => GenreChild::Street(Box::new(super::parse_street_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "repository" => GenreChild::Repository(Box::new(super::parse_repository_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "heraldry" => GenreChild::Heraldry(Box::new(parse_heraldry_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "secFolio" => GenreChild::SecFolio(Box::new(parse_sec_folio_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "stamp" => GenreChild::Stamp(Box::new(Stamp::from_mei_event(
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

/// Parse an `<audience>` element.
pub(crate) fn parse_audience_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Audience> {
    let mut elem = Audience::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("audience")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(AudienceChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_audience_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_audience_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<AudienceChild>> {
    // Audience has similar child content to Genre
    Ok(Some(match name {
        "date" => AudienceChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => AudienceChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => AudienceChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => AudienceChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => AudienceChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => AudienceChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => AudienceChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => AudienceChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => AudienceChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => AudienceChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogFeat" => AudienceChild::GeogFeat(Box::new(super::parse_geog_feat_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => AudienceChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => AudienceChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => AudienceChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => AudienceChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => AudienceChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => AudienceChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => AudienceChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => AudienceChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => AudienceChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => AudienceChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => AudienceChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => AudienceChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => AudienceChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => AudienceChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => AudienceChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => AudienceChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => AudienceChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => AudienceChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => AudienceChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => AudienceChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => AudienceChild::Title(Box::new(super::parse_title_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "head" => AudienceChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => AudienceChild::P(Box::new(super::parse_p_from_event(
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

/// Parse a `<textLang>` element.
pub(crate) fn parse_text_lang_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TextLang> {
    let mut elem = TextLang::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    // TextLang-specific attributes
    extract_attr!(attrs, "lang.main", string elem.lang_main);
    extract_attr!(attrs, "lang.other", vec_string elem.lang_other);

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("textLang")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(TextLangChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_text_lang_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_text_lang_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<TextLangChild>> {
    // TextLang has similar child content
    Ok(Some(match name {
        "date" => TextLangChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => TextLangChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => TextLangChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => TextLangChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => TextLangChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => TextLangChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => TextLangChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => TextLangChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => TextLangChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => TextLangChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogFeat" => TextLangChild::GeogFeat(Box::new(super::parse_geog_feat_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "address" => TextLangChild::Address(Box::new(super::parse_address_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "country" => TextLangChild::Country(Box::new(super::parse_country_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "region" => TextLangChild::Region(Box::new(super::parse_region_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "settlement" => TextLangChild::Settlement(Box::new(super::parse_settlement_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "district" => TextLangChild::District(Box::new(super::parse_district_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bloc" => TextLangChild::Bloc(Box::new(super::parse_bloc_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dimensions" => TextLangChild::Dimensions(Box::new(
            super::phys_desc::parse_dimensions_from_event(reader, child_attrs, child_empty)?,
        )),
        "height" => TextLangChild::Height(Box::new(super::phys_desc::parse_height_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "width" => TextLangChild::Width(Box::new(super::phys_desc::parse_width_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "depth" => TextLangChild::Depth(Box::new(super::phys_desc::parse_depth_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "dim" => TextLangChild::Dim(Box::new(super::phys_desc::parse_dim_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "term" => TextLangChild::Term(Box::new(super::super::parse_term_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => TextLangChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => TextLangChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "num" => TextLangChild::Num(Box::new(super::super::parse_num_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => TextLangChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => TextLangChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "identifier" => TextLangChild::Identifier(Box::new(super::parse_identifier_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locus" => TextLangChild::Locus(Box::new(super::parse_locus_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "locusGrp" => TextLangChild::LocusGrp(Box::new(super::parse_locus_grp_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "title" => TextLangChild::Title(Box::new(super::parse_title_from_event(
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

/// Parse a `<heraldry>` element.
pub(crate) fn parse_heraldry_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Heraldry> {
    let mut elem = Heraldry::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("heraldry")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(HeraldryChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_heraldry_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_heraldry_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<HeraldryChild>> {
    Ok(Some(match name {
        "head" => HeraldryChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => HeraldryChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => HeraldryChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => HeraldryChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => HeraldryChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => HeraldryChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => HeraldryChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => HeraldryChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => HeraldryChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => HeraldryChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => HeraldryChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => HeraldryChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogFeat" => HeraldryChild::GeogFeat(Box::new(super::parse_geog_feat_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => HeraldryChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => HeraldryChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => HeraldryChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => HeraldryChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "heraldry" => HeraldryChild::Heraldry(Box::new(parse_heraldry_from_event(
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

/// Parse an `<inscription>` element.
pub(crate) fn parse_inscription_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Inscription> {
    let mut elem = Inscription::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("inscription")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(InscriptionChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_inscription_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_inscription_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<InscriptionChild>> {
    Ok(Some(match name {
        "head" => InscriptionChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => InscriptionChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => InscriptionChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => InscriptionChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => InscriptionChild::BiblStruct(Box::new(
            super::parse_bibl_struct_from_event(reader, child_attrs, child_empty)?,
        )),
        "annot" => InscriptionChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => InscriptionChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => InscriptionChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => InscriptionChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => InscriptionChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => InscriptionChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => InscriptionChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => InscriptionChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => InscriptionChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => InscriptionChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => InscriptionChild::Fig(Box::new(super::super::parse_fig_from_event(
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

/// Parse a `<secFolio>` element.
pub(crate) fn parse_sec_folio_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SecFolio> {
    let mut elem = SecFolio::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("secFolio")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(SecFolioChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_sec_folio_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_sec_folio_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<SecFolioChild>> {
    Ok(Some(match name {
        "head" => SecFolioChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => SecFolioChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => SecFolioChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => SecFolioChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => SecFolioChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => SecFolioChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => SecFolioChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => SecFolioChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => SecFolioChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => SecFolioChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => SecFolioChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => SecFolioChild::Fig(Box::new(super::super::parse_fig_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "secFolio" => SecFolioChild::SecFolio(Box::new(parse_sec_folio_from_event(
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

/// Parse a `<specRepro>` element.
pub(crate) fn parse_spec_repro_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SpecRepro> {
    let mut elem = SpecRepro::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.authorized.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("specRepro")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(SpecReproChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_spec_repro_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_spec_repro_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<SpecReproChild>> {
    Ok(Some(match name {
        "head" => SpecReproChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => SpecReproChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => SpecReproChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => SpecReproChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => SpecReproChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => SpecReproChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => SpecReproChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => SpecReproChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => SpecReproChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => SpecReproChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => SpecReproChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => SpecReproChild::Fig(Box::new(super::super::parse_fig_from_event(
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

/// Parse a `<recipient>` element.
pub(crate) fn parse_recipient_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Recipient> {
    let mut elem = Recipient::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("recipient")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(RecipientChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_recipient_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_recipient_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<RecipientChild>> {
    Ok(Some(match name {
        "date" => RecipientChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => RecipientChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => RecipientChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => RecipientChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => RecipientChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => RecipientChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => RecipientChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => RecipientChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => RecipientChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => RecipientChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => RecipientChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => RecipientChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => RecipientChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => RecipientChild::Fig(Box::new(super::super::parse_fig_from_event(
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

/// Parse a `<treatHist>` element.
pub(crate) fn parse_treat_hist_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TreatHist> {
    let mut elem = TreatHist::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.datable.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("treatHist")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(TreatHistChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_treat_hist_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_treat_hist_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<TreatHistChild>> {
    Ok(Some(match name {
        "head" => TreatHistChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => TreatHistChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => TreatHistChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => TreatHistChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => TreatHistChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => TreatHistChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => TreatHistChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => TreatHistChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => TreatHistChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => TreatHistChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => TreatHistChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => TreatHistChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "eventList" => TreatHistChild::EventList(Box::new(EventList::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => TreatHistChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => TreatHistChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => TreatHistChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => TreatHistChild::Fig(Box::new(super::super::parse_fig_from_event(
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

/// Parse a `<treatSched>` element.
pub(crate) fn parse_treat_sched_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<TreatSched> {
    let mut elem = TreatSched::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.datable.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("treatSched")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(TreatSchedChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_treat_sched_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_treat_sched_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<TreatSchedChild>> {
    Ok(Some(match name {
        "head" => TreatSchedChild::Head(Box::new(super::parse_head_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "p" => TreatSchedChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "date" => TreatSchedChild::Date(Box::new(super::parse_date_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "bibl" => TreatSchedChild::Bibl(Box::new(super::parse_bibl_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "biblStruct" => TreatSchedChild::BiblStruct(Box::new(super::parse_bibl_struct_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => TreatSchedChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => TreatSchedChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => TreatSchedChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "persName" => TreatSchedChild::PersName(Box::new(super::parse_pers_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "corpName" => TreatSchedChild::CorpName(Box::new(super::parse_corp_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "name" => TreatSchedChild::Name(Box::new(super::parse_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "geogName" => TreatSchedChild::GeogName(Box::new(super::parse_geog_name_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "eventList" => TreatSchedChild::EventList(Box::new(EventList::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lb" => TreatSchedChild::Lb(Box::new(super::super::parse_lb_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "rend" => TreatSchedChild::Rend(Box::new(super::super::parse_rend_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "seg" => TreatSchedChild::Seg(Box::new(super::super::parse_seg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "fig" => TreatSchedChild::Fig(Box::new(super::super::parse_fig_from_event(
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

/// Parse a `<pgDesc>` element.
pub(crate) fn parse_pg_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PgDesc> {
    let mut elem = PgDesc::default();

    elem.common.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("pgDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(PgDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    if let Some(child) =
                        parse_pg_desc_child(reader, &name, child_attrs, child_empty)?
                    {
                        elem.children.push(child);
                    }
                }
            }
        }
    }

    Ok(elem)
}

fn parse_pg_desc_child<R: BufRead>(
    reader: &mut MeiReader<R>,
    name: &str,
    child_attrs: AttributeMap,
    child_empty: bool,
) -> DeserializeResult<Option<PgDescChild>> {
    Ok(Some(match name {
        "p" => PgDescChild::P(Box::new(super::parse_p_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ptr" => PgDescChild::Ptr(Box::new(super::parse_ptr_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "ref" => PgDescChild::Ref(Box::new(super::parse_ref_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "annot" => PgDescChild::Annot(Box::new(super::parse_annot_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "eventList" => PgDescChild::EventList(Box::new(EventList::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "lg" => PgDescChild::Lg(Box::new(super::super::parse_lg_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "list" => PgDescChild::List(Box::new(super::super::parse_list_from_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "table" => PgDescChild::Table(Box::new(Table::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "anchoredText" => PgDescChild::AnchoredText(Box::new(AnchoredText::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "line" => PgDescChild::Line(Box::new(Line::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "curve" => PgDescChild::Curve(Box::new(Curve::from_mei_event(
            reader,
            child_attrs,
            child_empty,
        )?)),
        "quote" => PgDescChild::Quote(Box::new(Quote::from_mei_event(
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

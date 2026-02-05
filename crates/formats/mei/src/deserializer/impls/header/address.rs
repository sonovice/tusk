//! Address-related elements (Address, AddrLine, GeogName, Street, etc.).

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{AddrLine, AddrLineChild, GeogName, GeogNameChild};

/// Parse an `<address>` element from within another element.
pub(crate) fn parse_address_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Address> {
    use tusk_model::elements::{Address, AddressChild};

    let mut address = Address::default();

    // Extract attributes
    address.common.extract_attributes(&mut attrs)?;
    address.facsimile.extract_attributes(&mut attrs)?;
    address.lang.extract_attributes(&mut attrs)?;

    // Address can contain: addrLine, street, postCode, settlement, country, region, bloc, geogFeat, district, postBox
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("address")?
        {
            match name.as_str() {
                "addrLine" => {
                    let addr_line = parse_addr_line_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::AddrLine(Box::new(addr_line)));
                }
                "street" => {
                    let street = parse_street_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::Street(Box::new(street)));
                }
                "postCode" => {
                    let post_code = parse_post_code_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::PostCode(Box::new(post_code)));
                }
                "settlement" => {
                    let settlement = parse_settlement_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::Settlement(Box::new(settlement)));
                }
                "country" => {
                    let country = parse_country_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::Country(Box::new(country)));
                }
                "region" => {
                    let region = parse_region_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::Region(Box::new(region)));
                }
                "bloc" => {
                    let bloc = parse_bloc_from_event(reader, child_attrs, child_empty)?;
                    address.children.push(AddressChild::Bloc(Box::new(bloc)));
                }
                "geogFeat" => {
                    let geog_feat = parse_geog_feat_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::GeogFeat(Box::new(geog_feat)));
                }
                "district" => {
                    let district = parse_district_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::District(Box::new(district)));
                }
                "postBox" => {
                    let post_box = parse_post_box_from_event(reader, child_attrs, child_empty)?;
                    address
                        .children
                        .push(AddressChild::PostBox(Box::new(post_box)));
                }
                // Skip unknown children in lenient mode
                _ => {
                    if !child_empty {
                        reader.skip_to_end(&name)?;
                    }
                }
            }
        }
    }

    Ok(address)
}

/// Parse an `<addrLine>` element from within another element.
pub(crate) fn parse_addr_line_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AddrLine> {
    let mut addr_line = AddrLine::default();

    // Extract attributes
    addr_line.common.extract_attributes(&mut attrs)?;
    addr_line.facsimile.extract_attributes(&mut attrs)?;
    addr_line.lang.extract_attributes(&mut attrs)?;

    // addrLine is a mixed content element - can have text and child elements like geogName
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("addrLine")? {
            match content {
                MixedContent::Text(text) => {
                    addr_line.children.push(AddrLineChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "geogName" => {
                            let geog_name =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::GeogName(Box::new(geog_name)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            addr_line
                                .children
                                .push(AddrLineChild::CorpName(Box::new(corp_name)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            addr_line
                                .children
                                .push(AddrLineChild::PersName(Box::new(pers_name)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::Name(Box::new(name_elem)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::Address(Box::new(address)));
                        }
                        "street" => {
                            let street = parse_street_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::Street(Box::new(street)));
                        }
                        "postCode" => {
                            let post_code =
                                parse_post_code_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::PostCode(Box::new(post_code)));
                        }
                        "postBox" => {
                            let post_box =
                                parse_post_box_from_event(reader, child_attrs, child_empty)?;
                            addr_line
                                .children
                                .push(AddrLineChild::PostBox(Box::new(post_box)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(addr_line)
}

/// Parse a `<geogName>` element from within another element.
pub(crate) fn parse_geog_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<GeogName> {
    let mut geog_name = GeogName::default();

    // Extract attributes
    geog_name.common.extract_attributes(&mut attrs)?;
    geog_name.bibl.extract_attributes(&mut attrs)?;
    geog_name.edit.extract_attributes(&mut attrs)?;
    geog_name.facsimile.extract_attributes(&mut attrs)?;
    geog_name.lang.extract_attributes(&mut attrs)?;
    geog_name.name.extract_attributes(&mut attrs)?;

    // geogName is a mixed content element - can have text and child elements
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("geogName")? {
            match content {
                MixedContent::Text(text) => {
                    geog_name.children.push(GeogNameChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "geogName" => {
                            // Nested geogName
                            let nested =
                                parse_geog_name_from_event(reader, child_attrs, child_empty)?;
                            geog_name
                                .children
                                .push(GeogNameChild::GeogName(Box::new(nested)));
                        }
                        "corpName" => {
                            let corp_name = super::parse_corp_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            geog_name
                                .children
                                .push(GeogNameChild::CorpName(Box::new(corp_name)));
                        }
                        "persName" => {
                            let pers_name = super::parse_pers_name_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            geog_name
                                .children
                                .push(GeogNameChild::PersName(Box::new(pers_name)));
                        }
                        "name" => {
                            let name_elem =
                                super::parse_name_from_event(reader, child_attrs, child_empty)?;
                            geog_name
                                .children
                                .push(GeogNameChild::Name(Box::new(name_elem)));
                        }
                        "address" => {
                            let address =
                                parse_address_from_event(reader, child_attrs, child_empty)?;
                            geog_name
                                .children
                                .push(GeogNameChild::Address(Box::new(address)));
                        }
                        _ => {
                            // Skip unknown children in lenient mode
                            if !child_empty {
                                reader.skip_to_end(&name)?;
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(geog_name)
}

/// Parse a `<street>` element from within another element.
pub(crate) fn parse_street_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Street> {
    use tusk_model::elements::{Street, StreetChild};

    let mut street = Street::default();

    // Extract attributes
    street.common.extract_attributes(&mut attrs)?;
    street.facsimile.extract_attributes(&mut attrs)?;
    street.lang.extract_attributes(&mut attrs)?;

    // street is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("street")? {
            if !text.is_empty() {
                street.children.push(StreetChild::Text(text));
            }
        }
    }

    Ok(street)
}

/// Parse a `<postCode>` element from within another element.
pub(crate) fn parse_post_code_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::PostCode> {
    use tusk_model::elements::{PostCode, PostCodeChild};

    let mut post_code = PostCode::default();

    // Extract attributes
    post_code.common.extract_attributes(&mut attrs)?;
    post_code.facsimile.extract_attributes(&mut attrs)?;
    post_code.lang.extract_attributes(&mut attrs)?;

    // postCode is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("postCode")? {
            if !text.is_empty() {
                post_code.children.push(PostCodeChild::Text(text));
            }
        }
    }

    Ok(post_code)
}

/// Parse a `<settlement>` element from within another element.
pub(crate) fn parse_settlement_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Settlement> {
    use tusk_model::elements::{Settlement, SettlementChild};

    let mut settlement = Settlement::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    settlement.common.extract_attributes(&mut attrs)?;
    settlement.bibl.extract_attributes(&mut attrs)?;
    settlement.edit.extract_attributes(&mut attrs)?;
    settlement.facsimile.extract_attributes(&mut attrs)?;
    settlement.lang.extract_attributes(&mut attrs)?;
    settlement.name.extract_attributes(&mut attrs)?;

    // settlement is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("settlement")? {
            if !text.is_empty() {
                settlement.children.push(SettlementChild::Text(text));
            }
        }
    }

    Ok(settlement)
}

/// Parse a `<country>` element from within another element.
pub(crate) fn parse_country_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Country> {
    use tusk_model::elements::{Country, CountryChild};

    let mut country = Country::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    country.common.extract_attributes(&mut attrs)?;
    country.bibl.extract_attributes(&mut attrs)?;
    country.edit.extract_attributes(&mut attrs)?;
    country.facsimile.extract_attributes(&mut attrs)?;
    country.lang.extract_attributes(&mut attrs)?;
    country.name.extract_attributes(&mut attrs)?;

    // country is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("country")? {
            if !text.is_empty() {
                country.children.push(CountryChild::Text(text));
            }
        }
    }

    Ok(country)
}

/// Parse a `<region>` element from within another element.
pub(crate) fn parse_region_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Region> {
    use tusk_model::elements::{Region, RegionChild};

    let mut region = Region::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    region.common.extract_attributes(&mut attrs)?;
    region.bibl.extract_attributes(&mut attrs)?;
    region.edit.extract_attributes(&mut attrs)?;
    region.facsimile.extract_attributes(&mut attrs)?;
    region.lang.extract_attributes(&mut attrs)?;
    region.name.extract_attributes(&mut attrs)?;

    // region is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("region")? {
            if !text.is_empty() {
                region.children.push(RegionChild::Text(text));
            }
        }
    }

    Ok(region)
}

/// Parse a `<bloc>` element from within another element.
pub(crate) fn parse_bloc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::Bloc> {
    use tusk_model::elements::{Bloc, BlocChild};

    let mut bloc = Bloc::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    bloc.common.extract_attributes(&mut attrs)?;
    bloc.bibl.extract_attributes(&mut attrs)?;
    bloc.edit.extract_attributes(&mut attrs)?;
    bloc.facsimile.extract_attributes(&mut attrs)?;
    bloc.lang.extract_attributes(&mut attrs)?;
    bloc.name.extract_attributes(&mut attrs)?;

    // bloc is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("bloc")? {
            if !text.is_empty() {
                bloc.children.push(BlocChild::Text(text));
            }
        }
    }

    Ok(bloc)
}

/// Parse a `<geogFeat>` element from within another element.
pub(crate) fn parse_geog_feat_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::GeogFeat> {
    use tusk_model::elements::{GeogFeat, GeogFeatChild};

    let mut geog_feat = GeogFeat::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    geog_feat.common.extract_attributes(&mut attrs)?;
    geog_feat.bibl.extract_attributes(&mut attrs)?;
    geog_feat.edit.extract_attributes(&mut attrs)?;
    geog_feat.facsimile.extract_attributes(&mut attrs)?;
    geog_feat.lang.extract_attributes(&mut attrs)?;
    geog_feat.name.extract_attributes(&mut attrs)?;

    // geogFeat is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("geogFeat")? {
            if !text.is_empty() {
                geog_feat.children.push(GeogFeatChild::Text(text));
            }
        }
    }

    Ok(geog_feat)
}

/// Parse a `<district>` element from within another element.
pub(crate) fn parse_district_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::District> {
    use tusk_model::elements::{District, DistrictChild};

    let mut district = District::default();

    // Extract attributes (common, bibl, edit, facsimile, lang, name)
    district.common.extract_attributes(&mut attrs)?;
    district.bibl.extract_attributes(&mut attrs)?;
    district.edit.extract_attributes(&mut attrs)?;
    district.facsimile.extract_attributes(&mut attrs)?;
    district.lang.extract_attributes(&mut attrs)?;
    district.name.extract_attributes(&mut attrs)?;

    // district is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("district")? {
            if !text.is_empty() {
                district.children.push(DistrictChild::Text(text));
            }
        }
    }

    Ok(district)
}

/// Parse a `<postBox>` element from within another element.
pub(crate) fn parse_post_box_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<tusk_model::elements::PostBox> {
    use tusk_model::elements::{PostBox, PostBoxChild};

    let mut post_box = PostBox::default();

    // Extract attributes
    post_box.common.extract_attributes(&mut attrs)?;
    post_box.facsimile.extract_attributes(&mut attrs)?;
    post_box.lang.extract_attributes(&mut attrs)?;

    // postBox is a mixed content element
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("postBox")? {
            if !text.is_empty() {
                post_box.children.push(PostBoxChild::Text(text));
            }
        }
    }

    Ok(post_box)
}

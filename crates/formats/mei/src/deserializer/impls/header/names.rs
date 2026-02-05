//! Deserializer implementations for name elements.
//!
//! Contains: ForeName, FamName, AddName, GenName, NameLink, PeriodName, StyleName

use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader,
};
use std::io::BufRead;
use tusk_model::elements::{
    AddName, AddNameChild, FamName, FamNameChild, ForeName, ForeNameChild, GenName, GenNameChild,
    NameLink, NameLinkChild, PeriodName, PeriodNameChild, StyleName, StyleNameChild,
};

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for ForeName {
    fn element_name() -> &'static str {
        "foreName"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_fore_name_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for FamName {
    fn element_name() -> &'static str {
        "famName"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_fam_name_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for AddName {
    fn element_name() -> &'static str {
        "addName"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_add_name_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for GenName {
    fn element_name() -> &'static str {
        "genName"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_gen_name_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for NameLink {
    fn element_name() -> &'static str {
        "nameLink"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_name_link_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for PeriodName {
    fn element_name() -> &'static str {
        "periodName"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_period_name_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for StyleName {
    fn element_name() -> &'static str {
        "styleName"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_style_name_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<foreName>` element from within another element.
pub(crate) fn parse_fore_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<ForeName> {
    let mut elem = ForeName::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.edit.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.name.extract_attributes(&mut attrs)?;

    // ForeName has mixed content
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("foreName")? {
            if !text.is_empty() {
                elem.children.push(ForeNameChild::Text(text));
            }
        }
    }

    Ok(elem)
}

/// Parse a `<famName>` element from within another element.
pub(crate) fn parse_fam_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<FamName> {
    let mut elem = FamName::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.edit.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.name.extract_attributes(&mut attrs)?;

    // FamName has mixed content
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("famName")? {
            if !text.is_empty() {
                elem.children.push(FamNameChild::Text(text));
            }
        }
    }

    Ok(elem)
}

/// Parse an `<addName>` element from within another element.
pub(crate) fn parse_add_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<AddName> {
    let mut elem = AddName::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.edit.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.name.extract_attributes(&mut attrs)?;

    // AddName has mixed content
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("addName")? {
            if !text.is_empty() {
                elem.children.push(AddNameChild::Text(text));
            }
        }
    }

    Ok(elem)
}

/// Parse a `<genName>` element from within another element.
pub(crate) fn parse_gen_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<GenName> {
    let mut elem = GenName::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.edit.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.name.extract_attributes(&mut attrs)?;

    // GenName has mixed content
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("genName")? {
            if !text.is_empty() {
                elem.children.push(GenNameChild::Text(text));
            }
        }
    }

    Ok(elem)
}

/// Parse a `<nameLink>` element from within another element.
pub(crate) fn parse_name_link_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<NameLink> {
    let mut elem = NameLink::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.edit.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.name.extract_attributes(&mut attrs)?;

    // NameLink has mixed content
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("nameLink")? {
            if !text.is_empty() {
                elem.children.push(NameLinkChild::Text(text));
            }
        }
    }

    Ok(elem)
}

/// Parse a `<periodName>` element from within another element.
pub(crate) fn parse_period_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<PeriodName> {
    let mut elem = PeriodName::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.edit.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.name.extract_attributes(&mut attrs)?;

    // PeriodName has mixed content
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("periodName")? {
            if !text.is_empty() {
                elem.children.push(PeriodNameChild::Text(text));
            }
        }
    }

    Ok(elem)
}

/// Parse a `<styleName>` element from within another element.
pub(crate) fn parse_style_name_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<StyleName> {
    let mut elem = StyleName::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.edit.extract_attributes(&mut attrs)?;
    elem.facsimile.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.name.extract_attributes(&mut attrs)?;

    // StyleName has mixed content
    if !is_empty {
        if let Some(text) = reader.read_text_until_end("styleName")? {
            if !text.is_empty() {
                elem.children.push(StyleNameChild::Text(text));
            }
        }
    }

    Ok(elem)
}

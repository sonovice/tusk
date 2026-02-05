//! Physical description elements (Dimensions, Height, Width, Depth, Dim, Support, SupportDesc, Collation, Foliation, Condition).
//!
//! These elements describe the physical properties of manuscript materials.

use super::super::{extract_attr, from_attr_string};
use crate::deserializer::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, MixedContent,
};
use std::io::BufRead;
use tusk_model::elements::{
    Collation, CollationChild, Condition, ConditionChild, Depth, DepthChild, Dim, DimChild,
    Dimensions, DimensionsChild, Foliation, FoliationChild, Height, HeightChild, Support,
    SupportChild, SupportDesc, SupportDescChild, Width, WidthChild,
};

// ============================================================================
// MeiDeserialize trait implementations
// ============================================================================

impl MeiDeserialize for Dimensions {
    fn element_name() -> &'static str {
        "dimensions"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_dimensions_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Height {
    fn element_name() -> &'static str {
        "height"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_height_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Width {
    fn element_name() -> &'static str {
        "width"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_width_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Depth {
    fn element_name() -> &'static str {
        "depth"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_depth_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Dim {
    fn element_name() -> &'static str {
        "dim"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_dim_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Support {
    fn element_name() -> &'static str {
        "support"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_support_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for SupportDesc {
    fn element_name() -> &'static str {
        "supportDesc"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_support_desc_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Collation {
    fn element_name() -> &'static str {
        "collation"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_collation_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Foliation {
    fn element_name() -> &'static str {
        "foliation"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_foliation_from_event(reader, attrs, is_empty)
    }
}

impl MeiDeserialize for Condition {
    fn element_name() -> &'static str {
        "condition"
    }
    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_condition_from_event(reader, attrs, is_empty)
    }
}

// ============================================================================
// Parse functions
// ============================================================================

/// Parse a `<dimensions>` element.
///
/// Dimensions provides physical size information and can contain mixed content
/// including text, height, width, depth, dim, and many other children.
pub(crate) fn parse_dimensions_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Dimensions> {
    let mut elem = Dimensions::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.evidence.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.measurement.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("dimensions")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(DimensionsChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimensionsChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimensionsChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimensionsChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimensionsChild::Dim(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(DimensionsChild::Dimensions(Box::new(child)));
                        }
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimensionsChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimensionsChild::P(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DimensionsChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DimensionsChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DimensionsChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

/// Parse a `<height>` element.
///
/// Height describes the vertical size of an object.
pub(crate) fn parse_height_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Height> {
    let mut elem = Height::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.quantity.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("height")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(HeightChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(HeightChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(HeightChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(HeightChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(HeightChild::Dim(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(HeightChild::Dimensions(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HeightChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HeightChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(HeightChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

/// Parse a `<width>` element.
///
/// Width describes the horizontal size of an object.
pub(crate) fn parse_width_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Width> {
    let mut elem = Width::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.quantity.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("width")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(WidthChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(WidthChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(WidthChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(WidthChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(WidthChild::Dim(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(WidthChild::Dimensions(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(WidthChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(WidthChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(WidthChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

/// Parse a `<depth>` element.
///
/// Depth describes a measurement through a three-dimensional object.
pub(crate) fn parse_depth_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Depth> {
    let mut elem = Depth::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.quantity.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("depth")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(DepthChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DepthChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DepthChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DepthChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DepthChild::Dim(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DepthChild::Dimensions(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DepthChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DepthChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DepthChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

/// Parse a `<dim>` element.
///
/// Dim is a generic dimension element for any single dimensional specification.
pub(crate) fn parse_dim_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Dim> {
    let mut elem = Dim::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;
    elem.quantity.extract_attributes(&mut attrs)?;

    // Extract element-specific attribute
    extract_attr!(attrs, "form", string elem.form);

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("dim")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(DimChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimChild::Dim(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(DimChild::Dimensions(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DimChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DimChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(DimChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

/// Parse a `<support>` element.
///
/// Support provides a description of the physical support material of a written item.
/// It has a restricted set of children: head, p, dimensions, condition, decoNote.
pub(crate) fn parse_support_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Support> {
    let mut elem = Support::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse children (no text content, just specific child elements)
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("support")?
        {
            match name.as_str() {
                "head" => {
                    let child = super::parse_head_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(SupportChild::Head(Box::new(child)));
                }
                "p" => {
                    let child = super::parse_p_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(SupportChild::P(Box::new(child)));
                }
                "dimensions" => {
                    let child = parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                    elem.children
                        .push(SupportChild::Dimensions(Box::new(child)));
                }
                "condition" => {
                    let child = parse_condition_from_event(reader, child_attrs, child_empty)?;
                    elem.children.push(SupportChild::Condition(Box::new(child)));
                }
                // decoNote is not yet implemented, skip for now
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

/// Parse a `<supportDesc>` element.
///
/// SupportDesc groups elements describing the physical support material of an item.
pub(crate) fn parse_support_desc_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<SupportDesc> {
    let mut elem = SupportDesc::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Extract element-specific attribute
    extract_attr!(attrs, "material", string elem.material);

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("supportDesc")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(SupportDescChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SupportDescChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SupportDescChild::P(Box::new(child)));
                        }
                        "support" => {
                            let child = parse_support_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(SupportDescChild::Support(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(SupportDescChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(SupportDescChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SupportDescChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SupportDescChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(SupportDescChild::Dim(Box::new(child)));
                        }
                        "collation" => {
                            let child =
                                parse_collation_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(SupportDescChild::Collation(Box::new(child)));
                        }
                        "foliation" => {
                            let child =
                                parse_foliation_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(SupportDescChild::Foliation(Box::new(child)));
                        }
                        "condition" => {
                            let child =
                                parse_condition_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(SupportDescChild::Condition(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SupportDescChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SupportDescChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(SupportDescChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

/// Parse a `<collation>` element.
///
/// Collation describes how leaves or bifolia of an item are physically arranged.
pub(crate) fn parse_collation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Collation> {
    let mut elem = Collation::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("collation")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(CollationChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(CollationChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(CollationChild::P(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(CollationChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(CollationChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(CollationChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(CollationChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(CollationChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(CollationChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(CollationChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(CollationChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

/// Parse a `<foliation>` element.
///
/// Foliation describes the numbering system(s) used to count leaves or pages in a codex.
pub(crate) fn parse_foliation_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Foliation> {
    let mut elem = Foliation::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("foliation")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(FoliationChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(FoliationChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(FoliationChild::P(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(FoliationChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(FoliationChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(FoliationChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(FoliationChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(FoliationChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(FoliationChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(FoliationChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(FoliationChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

/// Parse a `<condition>` element.
///
/// Condition describes the physical condition of an item.
pub(crate) fn parse_condition_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Condition> {
    let mut elem = Condition::default();

    // Extract attributes
    elem.common.extract_attributes(&mut attrs)?;
    elem.bibl.extract_attributes(&mut attrs)?;
    elem.lang.extract_attributes(&mut attrs)?;

    // Parse mixed content
    if !is_empty {
        while let Some(content) = reader.read_next_mixed_content("condition")? {
            match content {
                MixedContent::Text(text) => {
                    elem.children.push(ConditionChild::Text(text));
                }
                MixedContent::Element(name, child_attrs, child_empty) => {
                    match name.as_str() {
                        "head" => {
                            let child =
                                super::parse_head_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ConditionChild::Head(Box::new(child)));
                        }
                        "p" => {
                            let child =
                                super::parse_p_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ConditionChild::P(Box::new(child)));
                        }
                        "dimensions" => {
                            let child =
                                parse_dimensions_from_event(reader, child_attrs, child_empty)?;
                            elem.children
                                .push(ConditionChild::Dimensions(Box::new(child)));
                        }
                        "height" => {
                            let child = parse_height_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ConditionChild::Height(Box::new(child)));
                        }
                        "width" => {
                            let child = parse_width_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ConditionChild::Width(Box::new(child)));
                        }
                        "depth" => {
                            let child = parse_depth_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ConditionChild::Depth(Box::new(child)));
                        }
                        "dim" => {
                            let child = parse_dim_from_event(reader, child_attrs, child_empty)?;
                            elem.children.push(ConditionChild::Dim(Box::new(child)));
                        }
                        "lb" => {
                            let child = super::super::parse_lb_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ConditionChild::Lb(Box::new(child)));
                        }
                        "rend" => {
                            let child = super::super::parse_rend_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ConditionChild::Rend(Box::new(child)));
                        }
                        "num" => {
                            let child = super::super::parse_num_from_event(
                                reader,
                                child_attrs,
                                child_empty,
                            )?;
                            elem.children.push(ConditionChild::Num(Box::new(child)));
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
        }
    }

    Ok(elem)
}

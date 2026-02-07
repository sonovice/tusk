//! Deserializer implementations for facsimile-related MEI elements.
//!
//! This module contains deserializers for:
//! - Facsimile (container for surface elements)
//! - Surface (a writing surface in terms of coordinates)
//! - Zone (area of interest within a surface)
//! - Graphic (inline graphic reference)

use super::{
    AttributeMap, DeserializeResult, ExtractAttributes, MeiDeserialize, MeiReader, extract_attr,
    from_attr_string, parse_fig_desc_from_event,
};
use std::io::BufRead;
use tusk_model::att::{
    AttCoordinated, AttCoordinatedUl, AttDataPointing, AttDimensions, AttStartId, AttVisualOffset,
};
use tusk_model::elements::{
    Facsimile, FacsimileChild, Graphic, GraphicChild, Surface, SurfaceChild, Zone, ZoneChild,
};

// ============================================================================
// Attribute class implementations
// ============================================================================

// ============================================================================
// Facsimile element
// ============================================================================

impl MeiDeserialize for Facsimile {
    fn element_name() -> &'static str {
        "facsimile"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        mut attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        let mut facsimile = Facsimile::default();

        // Extract attributes
        facsimile.common.extract_attributes(&mut attrs)?;
        facsimile.metadata_pointing.extract_attributes(&mut attrs)?;

        // Read children if not empty
        // Facsimile can contain: surface*, graphic*
        if !is_empty {
            while let Some((name, child_attrs, child_empty)) =
                reader.read_next_child_start("facsimile")?
            {
                match name.as_str() {
                    "surface" => {
                        let surface = parse_surface_from_event(reader, child_attrs, child_empty)?;
                        facsimile
                            .children
                            .push(FacsimileChild::Surface(Box::new(surface)));
                    }
                    "graphic" => {
                        let graphic = parse_graphic_from_event(reader, child_attrs, child_empty)?;
                        facsimile
                            .children
                            .push(FacsimileChild::Graphic(Box::new(graphic)));
                    }
                    _ => {
                        reader.skip_unknown_child(&name, "facsimile", child_empty)?;
                    }
                }
            }
        }

        Ok(facsimile)
    }
}

// ============================================================================
// Surface element
// ============================================================================

impl MeiDeserialize for Surface {
    fn element_name() -> &'static str {
        "surface"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_surface_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<surface>` element from within another element.
pub(crate) fn parse_surface_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Surface> {
    let mut surface = Surface::default();

    // Extract attributes
    surface.common.extract_attributes(&mut attrs)?;
    surface.coordinated.extract_attributes(&mut attrs)?;
    surface.data_pointing.extract_attributes(&mut attrs)?;
    surface.metadata_pointing.extract_attributes(&mut attrs)?;
    surface.start_id.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // Surface can contain: graphic*, zone*, figDesc*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("surface")?
        {
            match name.as_str() {
                "graphic" => {
                    let graphic = parse_graphic_from_event(reader, child_attrs, child_empty)?;
                    surface
                        .children
                        .push(SurfaceChild::Graphic(Box::new(graphic)));
                }
                "zone" => {
                    let zone = parse_zone_from_event(reader, child_attrs, child_empty)?;
                    surface.children.push(SurfaceChild::Zone(Box::new(zone)));
                }
                "figDesc" => {
                    let fig_desc = parse_fig_desc_from_event(reader, child_attrs, child_empty)?;
                    surface
                        .children
                        .push(SurfaceChild::FigDesc(Box::new(fig_desc)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "surface", child_empty)?;
                }
            }
        }
    }

    Ok(surface)
}

// ============================================================================
// Zone element
// ============================================================================

impl MeiDeserialize for Zone {
    fn element_name() -> &'static str {
        "zone"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_zone_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<zone>` element from within another element.
pub(crate) fn parse_zone_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Zone> {
    let mut zone = Zone::default();

    // Extract attributes
    zone.common.extract_attributes(&mut attrs)?;
    zone.coordinated.extract_attributes(&mut attrs)?;
    zone.data_pointing.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // Zone can contain: figDesc*, graphic*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) = reader.read_next_child_start("zone")? {
            match name.as_str() {
                "figDesc" => {
                    let fig_desc = parse_fig_desc_from_event(reader, child_attrs, child_empty)?;
                    zone.children.push(ZoneChild::FigDesc(Box::new(fig_desc)));
                }
                "graphic" => {
                    let graphic = parse_graphic_from_event(reader, child_attrs, child_empty)?;
                    zone.children.push(ZoneChild::Graphic(Box::new(graphic)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "zone", child_empty)?;
                }
            }
        }
    }

    Ok(zone)
}

// ============================================================================
// Graphic element
// ============================================================================

impl MeiDeserialize for Graphic {
    fn element_name() -> &'static str {
        "graphic"
    }

    fn from_mei_event<R: BufRead>(
        reader: &mut MeiReader<R>,
        attrs: AttributeMap,
        is_empty: bool,
    ) -> DeserializeResult<Self> {
        parse_graphic_from_event(reader, attrs, is_empty)
    }
}

/// Parse a `<graphic>` element from within another element.
pub(crate) fn parse_graphic_from_event<R: BufRead>(
    reader: &mut MeiReader<R>,
    mut attrs: AttributeMap,
    is_empty: bool,
) -> DeserializeResult<Graphic> {
    let mut graphic = Graphic::default();

    // Extract attributes
    graphic.common.extract_attributes(&mut attrs)?;
    graphic.coordinated_ul.extract_attributes(&mut attrs)?;
    graphic.dimensions.extract_attributes(&mut attrs)?;
    graphic.internet_media.extract_attributes(&mut attrs)?;
    graphic.facsimile.extract_attributes(&mut attrs)?;
    graphic.metadata_pointing.extract_attributes(&mut attrs)?;
    graphic.pointing.extract_attributes(&mut attrs)?;
    graphic.start_id.extract_attributes(&mut attrs)?;
    graphic.visual_offset.extract_attributes(&mut attrs)?;

    // Read children if not empty
    // Graphic can contain: zone*
    if !is_empty {
        while let Some((name, child_attrs, child_empty)) =
            reader.read_next_child_start("graphic")?
        {
            match name.as_str() {
                "zone" => {
                    let zone = parse_zone_from_event(reader, child_attrs, child_empty)?;
                    graphic.children.push(GraphicChild::Zone(Box::new(zone)));
                }
                _ => {
                    reader.skip_unknown_child(&name, "graphic", child_empty)?;
                }
            }
        }
    }

    Ok(graphic)
}

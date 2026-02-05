//! Round-trip serialization tests for facsimile-related MEI elements.
//!
//! Tests for Facsimile, Surface, Zone, Graphic elements.

use crate::deserializer::MeiDeserialize;
use crate::serializer::MeiSerialize;

// ============================================================================
// Facsimile Tests
// ============================================================================

#[test]
fn facsimile_roundtrip_empty() {
    use tusk_model::elements::Facsimile;

    let original = Facsimile::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Facsimile::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn facsimile_roundtrip_with_xml_id() {
    use tusk_model::elements::Facsimile;

    let mut original = Facsimile::default();
    original.common.xml_id = Some("facs-1".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Facsimile::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("facs-1".to_string()));
}

#[test]
fn facsimile_roundtrip_with_surface() {
    use tusk_model::elements::{Facsimile, FacsimileChild, Surface};

    let mut original = Facsimile::default();
    original.common.xml_id = Some("facs-1".to_string());

    let mut surface = Surface::default();
    surface.common.xml_id = Some("surface-1".to_string());
    original
        .children
        .push(FacsimileChild::Surface(Box::new(surface)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Facsimile::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        FacsimileChild::Surface(s) => {
            assert_eq!(s.common.xml_id, Some("surface-1".to_string()));
        }
        _ => panic!("Expected Surface child"),
    }
}

#[test]
fn facsimile_roundtrip_with_graphic() {
    use tusk_model::elements::{Facsimile, FacsimileChild, Graphic};

    let mut original = Facsimile::default();
    original.common.xml_id = Some("facs-1".to_string());

    let mut graphic = Graphic::default();
    graphic.common.xml_id = Some("graphic-1".to_string());
    original
        .children
        .push(FacsimileChild::Graphic(Box::new(graphic)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Facsimile::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        FacsimileChild::Graphic(g) => {
            assert_eq!(g.common.xml_id, Some("graphic-1".to_string()));
        }
        _ => panic!("Expected Graphic child"),
    }
}

// ============================================================================
// Surface Tests
// ============================================================================

#[test]
fn surface_roundtrip_empty() {
    use tusk_model::elements::Surface;

    let original = Surface::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Surface::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn surface_roundtrip_with_coordinates() {
    use tusk_model::elements::Surface;

    let mut original = Surface::default();
    original.common.xml_id = Some("surface-1".to_string());
    original.coordinated.ulx = Some(0);
    original.coordinated.uly = Some(0);
    original.coordinated.lrx = Some(1000);
    original.coordinated.lry = Some(800);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Surface::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("surface-1".to_string()));
    assert_eq!(parsed.coordinated.ulx, Some(0));
    assert_eq!(parsed.coordinated.uly, Some(0));
    assert_eq!(parsed.coordinated.lrx, Some(1000));
    assert_eq!(parsed.coordinated.lry, Some(800));
}

#[test]
fn surface_roundtrip_with_zone() {
    use tusk_model::elements::{Surface, SurfaceChild, Zone};

    let mut original = Surface::default();
    original.common.xml_id = Some("surface-1".to_string());

    let mut zone = Zone::default();
    zone.common.xml_id = Some("zone-1".to_string());
    zone.coordinated.ulx = Some(100);
    zone.coordinated.uly = Some(100);
    zone.coordinated.lrx = Some(200);
    zone.coordinated.lry = Some(150);
    original.children.push(SurfaceChild::Zone(Box::new(zone)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Surface::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SurfaceChild::Zone(z) => {
            assert_eq!(z.common.xml_id, Some("zone-1".to_string()));
            assert_eq!(z.coordinated.ulx, Some(100));
            assert_eq!(z.coordinated.uly, Some(100));
            assert_eq!(z.coordinated.lrx, Some(200));
            assert_eq!(z.coordinated.lry, Some(150));
        }
        _ => panic!("Expected Zone child"),
    }
}

#[test]
fn surface_roundtrip_with_graphic() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{Graphic, Surface, SurfaceChild};

    let mut original = Surface::default();
    original.common.xml_id = Some("surface-1".to_string());

    let mut graphic = Graphic::default();
    graphic.common.xml_id = Some("graphic-1".to_string());
    graphic.pointing.target = vec![DataUri("image.jpg".to_string())];
    original
        .children
        .push(SurfaceChild::Graphic(Box::new(graphic)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Surface::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.children.len(), 1);
    match &parsed.children[0] {
        SurfaceChild::Graphic(g) => {
            assert_eq!(g.common.xml_id, Some("graphic-1".to_string()));
            assert_eq!(g.pointing.target.len(), 1);
            assert_eq!(g.pointing.target[0].0, "image.jpg");
        }
        _ => panic!("Expected Graphic child"),
    }
}

// ============================================================================
// Zone Tests
// ============================================================================

#[test]
fn zone_roundtrip_empty() {
    use tusk_model::elements::Zone;

    let original = Zone::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Zone::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn zone_roundtrip_with_coordinates() {
    use tusk_model::elements::Zone;

    let mut original = Zone::default();
    original.common.xml_id = Some("zone-1".to_string());
    original.coordinated.ulx = Some(50);
    original.coordinated.uly = Some(100);
    original.coordinated.lrx = Some(250);
    original.coordinated.lry = Some(200);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Zone::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("zone-1".to_string()));
    assert_eq!(parsed.coordinated.ulx, Some(50));
    assert_eq!(parsed.coordinated.uly, Some(100));
    assert_eq!(parsed.coordinated.lrx, Some(250));
    assert_eq!(parsed.coordinated.lry, Some(200));
}

#[test]
fn zone_roundtrip_with_data_pointing() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Zone;

    let mut original = Zone::default();
    original.common.xml_id = Some("zone-1".to_string());
    original.data_pointing.data = vec![DataUri("#note-1".to_string())];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Zone::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("zone-1".to_string()));
    assert_eq!(parsed.data_pointing.data.len(), 1);
    assert_eq!(parsed.data_pointing.data[0].0, "#note-1");
}

// ============================================================================
// Graphic Tests
// ============================================================================

#[test]
fn graphic_roundtrip_empty() {
    use tusk_model::elements::Graphic;

    let original = Graphic::default();
    let xml = original.to_mei_string().expect("serialize");
    let parsed = Graphic::from_mei_str(&xml).expect("deserialize");

    assert!(parsed.common.xml_id.is_none());
    assert!(parsed.children.is_empty());
}

#[test]
fn graphic_roundtrip_with_target() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::Graphic;

    let mut original = Graphic::default();
    original.common.xml_id = Some("graphic-1".to_string());
    original.pointing.target = vec![DataUri("page1.png".to_string())];

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Graphic::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("graphic-1".to_string()));
    assert_eq!(parsed.pointing.target.len(), 1);
    assert_eq!(parsed.pointing.target[0].0, "page1.png");
}

#[test]
fn graphic_roundtrip_with_dimensions() {
    use tusk_model::data::DataMeasurementunsigned;
    use tusk_model::elements::Graphic;

    let mut original = Graphic::default();
    original.common.xml_id = Some("graphic-1".to_string());
    original.dimensions.width = Some(DataMeasurementunsigned("1920px".to_string()));
    original.dimensions.height = Some(DataMeasurementunsigned("1080px".to_string()));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Graphic::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("graphic-1".to_string()));
    assert_eq!(
        parsed.dimensions.width,
        Some(DataMeasurementunsigned("1920px".to_string()))
    );
    assert_eq!(
        parsed.dimensions.height,
        Some(DataMeasurementunsigned("1080px".to_string()))
    );
}

#[test]
fn graphic_roundtrip_with_mimetype() {
    use tusk_model::elements::Graphic;

    let mut original = Graphic::default();
    original.common.xml_id = Some("graphic-1".to_string());
    original.internet_media.mimetype = Some("image/png".to_string());

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Graphic::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("graphic-1".to_string()));
    assert_eq!(
        parsed.internet_media.mimetype,
        Some("image/png".to_string())
    );
}

#[test]
fn graphic_roundtrip_with_ul_coordinates() {
    use tusk_model::elements::Graphic;

    let mut original = Graphic::default();
    original.common.xml_id = Some("graphic-1".to_string());
    original.coordinated_ul.ulx = Some(10);
    original.coordinated_ul.uly = Some(20);

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Graphic::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("graphic-1".to_string()));
    assert_eq!(parsed.coordinated_ul.ulx, Some(10));
    assert_eq!(parsed.coordinated_ul.uly, Some(20));
}

// ============================================================================
// Complex nested structure tests
// ============================================================================

#[test]
fn facsimile_roundtrip_complex_nested() {
    use tusk_model::data::DataUri;
    use tusk_model::elements::{Facsimile, FacsimileChild, Graphic, Surface, SurfaceChild, Zone};

    let mut original = Facsimile::default();
    original.common.xml_id = Some("facs-1".to_string());

    // Create surface with zone and graphic
    let mut surface = Surface::default();
    surface.common.xml_id = Some("surface-1".to_string());
    surface.coordinated.ulx = Some(0);
    surface.coordinated.uly = Some(0);
    surface.coordinated.lrx = Some(2000);
    surface.coordinated.lry = Some(1500);

    // Add graphic to surface
    let mut graphic = Graphic::default();
    graphic.common.xml_id = Some("graphic-1".to_string());
    graphic.pointing.target = vec![DataUri("page1.tiff".to_string())];
    surface
        .children
        .push(SurfaceChild::Graphic(Box::new(graphic)));

    // Add zone to surface
    let mut zone = Zone::default();
    zone.common.xml_id = Some("zone-1".to_string());
    zone.coordinated.ulx = Some(100);
    zone.coordinated.uly = Some(200);
    zone.coordinated.lrx = Some(500);
    zone.coordinated.lry = Some(400);
    zone.data_pointing.data = vec![DataUri("#measure-1".to_string())];
    surface.children.push(SurfaceChild::Zone(Box::new(zone)));

    original
        .children
        .push(FacsimileChild::Surface(Box::new(surface)));

    let xml = original.to_mei_string().expect("serialize");
    let parsed = Facsimile::from_mei_str(&xml).expect("deserialize");

    assert_eq!(parsed.common.xml_id, Some("facs-1".to_string()));
    assert_eq!(parsed.children.len(), 1);

    match &parsed.children[0] {
        FacsimileChild::Surface(s) => {
            assert_eq!(s.common.xml_id, Some("surface-1".to_string()));
            assert_eq!(s.coordinated.lrx, Some(2000));
            assert_eq!(s.children.len(), 2);

            // Verify graphic
            match &s.children[0] {
                SurfaceChild::Graphic(g) => {
                    assert_eq!(g.common.xml_id, Some("graphic-1".to_string()));
                    assert_eq!(g.pointing.target[0].0, "page1.tiff");
                }
                _ => panic!("Expected Graphic as first child"),
            }

            // Verify zone
            match &s.children[1] {
                SurfaceChild::Zone(z) => {
                    assert_eq!(z.common.xml_id, Some("zone-1".to_string()));
                    assert_eq!(z.coordinated.ulx, Some(100));
                    assert_eq!(z.data_pointing.data[0].0, "#measure-1");
                }
                _ => panic!("Expected Zone as second child"),
            }
        }
        _ => panic!("Expected Surface child"),
    }
}

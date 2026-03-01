//! Cross-format stabilization: MEI -> MusicXML -> MEI.
//!
//! Two-pass pipeline: MEI -> MusicXML1 -> MEI1 -> MusicXML2 -> MEI2.
//! Asserts MEI1 == MEI2 via tree-based xml_compare.

use tusk_roundtrip_tests::{
    cross_roundtrip, load_fixture, mei_to_musicxml, musicxml_to_mei,
    assert_stable_mei,
};

// ============================================================================
// Golden fixture tests (must pass)
// ============================================================================

fn assert_mei_via_musicxml(subdir: &str, name: &str) {
    let src = load_fixture(subdir, name);
    let (a1, a2) = cross_roundtrip(&src, mei_to_musicxml, musicxml_to_mei)
        .unwrap_or_else(|e| panic!("{name}: {e}"));
    assert_stable_mei(&a1, &a2, name);
}

#[test]
fn golden_hello_world() {
    assert_mei_via_musicxml("mei", "hello_world.mei");
}

#[test]
fn golden_scale() {
    assert_mei_via_musicxml("mei", "scale.mei");
}

// ============================================================================
// Per-file generated tests (build.rs)
// ============================================================================

include!(concat!(env!("OUT_DIR"), "/cross_mei_via_musicxml.rs"));


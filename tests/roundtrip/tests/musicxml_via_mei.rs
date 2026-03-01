//! Cross-format stabilization: MusicXML -> MEI -> MusicXML.
//!
//! Two-pass pipeline: MusicXML -> MEI serialize -> MEI parse -> MusicXML1 -> MEI -> MusicXML2.
//! Asserts MusicXML1 == MusicXML2 (string equality after normalization).

use tusk_roundtrip_tests::{
    cross_roundtrip, load_fixture, musicxml_to_mei, mei_to_musicxml,
    assert_stable,
};

// ============================================================================
// Golden fixture tests (must pass)
// ============================================================================

fn assert_musicxml_via_mei(subdir: &str, name: &str) {
    let src = load_fixture(subdir, name);
    let (a1, a2) = cross_roundtrip(&src, musicxml_to_mei, mei_to_musicxml)
        .unwrap_or_else(|e| panic!("{name}: {e}"));
    assert_stable(&a1, &a2, name);
}

#[test]
fn golden_hello_world() {
    assert_musicxml_via_mei("musicxml", "hello_world.musicxml");
}

#[test]
fn golden_scale() {
    assert_musicxml_via_mei("musicxml", "scale.musicxml");
}

#[test]
fn golden_chords_and_rests() {
    assert_musicxml_via_mei("musicxml", "chords_and_rests.musicxml");
}

// ============================================================================
// Per-file generated tests (build.rs)
// ============================================================================

include!(concat!(env!("OUT_DIR"), "/cross_musicxml_via_mei.rs"));

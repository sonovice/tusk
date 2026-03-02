//! Cross-format stabilization: LilyPond -> MEI -> LilyPond.
//!
//! Two-pass pipeline: LilyPond -> MEI1 -> LilyPond1 -> MEI2 -> LilyPond2.
//! Asserts LilyPond1 == LilyPond2 (string equality).

use tusk_roundtrip_tests::{
    load_fixture, lilypond_via_mei, assert_stable,
};

// ============================================================================
// Golden fixture tests (must pass)
// ============================================================================

fn assert_lilypond_via_mei(subdir: &str, name: &str) {
    let src = load_fixture(subdir, name);
    let ly1 = lilypond_via_mei(&src).unwrap_or_else(|e| panic!("{name}: {e}"));
    let ly2 = lilypond_via_mei(&ly1).unwrap_or_else(|e| panic!("{name} pass2: {e}"));
    assert_stable(&ly1, &ly2, name);
}

#[test]
fn golden_simple() {
    assert_lilypond_via_mei("lilypond", "fragment_score_minimal.ly");
}

#[test]
fn golden_fragment_score_minimal() {
    assert_lilypond_via_mei("lilypond", "fragment_score_minimal.ly");
}

// ============================================================================
// Per-file generated tests (build.rs)
// ============================================================================

include!(concat!(env!("OUT_DIR"), "/cross_lilypond_via_mei.rs"));

include!(concat!(env!("OUT_DIR"), "/mutopia_lilypond_via_mei.rs"));

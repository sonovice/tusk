use super::*;
use crate::import;
use crate::parser::Parser;
use crate::serializer;

/// Parse LilyPond -> import to MEI -> export to LilyPond AST -> serialize.
fn roundtrip(src: &str) -> String {
    let file = Parser::new(src).unwrap().parse().unwrap();
    let mei = import::import(&file).unwrap();
    let exported = export(&mei).unwrap();
    serializer::serialize(&exported)
}

#[test]
fn roundtrip_single_note() {
    let output = roundtrip("{ c'4 }");
    assert!(output.contains("c'4"), "output: {output}");
}

#[test]
fn roundtrip_note_with_accidental() {
    let output = roundtrip("{ cis''2 }");
    assert!(output.contains("cis''2"), "output: {output}");
}

#[test]
fn roundtrip_rest() {
    let output = roundtrip("{ r4 }");
    assert!(output.contains("r4"), "output: {output}");
}

#[test]
fn roundtrip_dotted() {
    let output = roundtrip("{ c'2. r8. }");
    assert!(output.contains("c'2."), "output: {output}");
    assert!(output.contains("r8."), "output: {output}");
}

#[test]
fn roundtrip_flat() {
    let output = roundtrip("{ bes,16 }");
    assert!(output.contains("bes,16"), "output: {output}");
}

#[test]
fn roundtrip_multiple_notes() {
    let output = roundtrip("{ c4 d8 e16 f2 }");
    assert!(output.contains("c4"), "output: {output}");
    assert!(output.contains("d8"), "output: {output}");
    assert!(output.contains("e16"), "output: {output}");
    assert!(output.contains("f2"), "output: {output}");
}

#[test]
fn roundtrip_multi_measure_rest() {
    let output = roundtrip("{ R1*4 }");
    assert!(output.contains("R1*4"), "output: {output}");
}

#[test]
fn roundtrip_pitched_rest() {
    let output = roundtrip("{ c4\\rest }");
    assert!(output.contains("c4\\rest"), "output: {output}");
}

#[test]
fn roundtrip_force_accidental() {
    let output = roundtrip("{ cis'!4 }");
    assert!(output.contains("cis'!4"), "output: {output}");
}

#[test]
fn roundtrip_cautionary_accidental() {
    let output = roundtrip("{ bes'?4 }");
    assert!(output.contains("bes'?4"), "output: {output}");
}

#[test]
fn roundtrip_two_voices() {
    let output = roundtrip("<< { c'4 d'4 } { e'4 f'4 } >>");
    // Should produce simultaneous with two sequential voices
    assert!(output.contains("<<"), "output: {output}");
    assert!(output.contains(">>"), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
    assert!(output.contains("d'4"), "output: {output}");
    assert!(output.contains("e'4"), "output: {output}");
    assert!(output.contains("f'4"), "output: {output}");
}

#[test]
fn roundtrip_three_voices() {
    let output = roundtrip("<< { c'4 } { e'4 } { g'4 } >>");
    assert!(output.contains("<<"), "output: {output}");
    assert!(output.contains(">>"), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
    assert!(output.contains("e'4"), "output: {output}");
    assert!(output.contains("g'4"), "output: {output}");
}

#[test]
fn roundtrip_sequential_preserved() {
    // Single voice should stay sequential, no << >>
    let output = roundtrip("{ c'4 d'4 e'4 }");
    assert!(!output.contains("<<"), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
}

// --- Phase 5.2: Context export/roundtrip tests ---

#[test]
fn roundtrip_staff_group() {
    let output =
        roundtrip("\\new StaffGroup << \\new Staff { c'4 d'4 } \\new Staff { e'4 f'4 } >>");
    assert!(output.contains("\\new StaffGroup"), "output: {output}");
    assert!(output.contains("\\new Staff"), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
    assert!(output.contains("e'4"), "output: {output}");
}

#[test]
fn roundtrip_piano_staff() {
    let output = roundtrip("\\new PianoStaff << \\new Staff { c'4 } \\new Staff { e4 } >>");
    assert!(output.contains("\\new PianoStaff"), "output: {output}");
    assert!(output.contains("\\new Staff"), "output: {output}");
}

#[test]
fn roundtrip_named_staves() {
    let output = roundtrip(
        "\\new StaffGroup << \\new Staff = \"violin\" { c'4 } \\new Staff = \"viola\" { e4 } >>",
    );
    assert!(output.contains("\"violin\""), "output: {output}");
    assert!(output.contains("\"viola\""), "output: {output}");
}

#[test]
fn roundtrip_single_named_staff() {
    let output = roundtrip("\\new Staff = \"piano\" { c'4 d'4 }");
    assert!(output.contains("\\new Staff"), "output: {output}");
    assert!(output.contains("\"piano\""), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
}

#[test]
fn roundtrip_staff_with_block() {
    let output = roundtrip("\\new Staff \\with { \\consists \"Span_arpeggio_engraver\" } { c'4 }");
    assert!(output.contains("\\new Staff"), "output: {output}");
    assert!(output.contains("\\with"), "output: {output}");
    assert!(
        output.contains("Span_arpeggio_engraver"),
        "output: {output}"
    );
}

// --- Phase 6.2: Clef/key/time roundtrip tests ---

#[test]
fn roundtrip_clef_treble() {
    let output = roundtrip("{ \\clef \"treble\" c'4 d'4 }");
    assert!(output.contains("\\clef \"treble\""), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
}

#[test]
fn roundtrip_clef_bass() {
    let output = roundtrip("{ \\clef \"bass\" c4 d4 }");
    assert!(output.contains("\\clef \"bass\""), "output: {output}");
}

#[test]
fn roundtrip_clef_alto() {
    let output = roundtrip("{ \\clef \"alto\" c'4 }");
    assert!(output.contains("\\clef \"alto\""), "output: {output}");
}

#[test]
fn roundtrip_key_d_major() {
    let output = roundtrip("{ \\key d \\major c'4 }");
    assert!(output.contains("\\key d \\major"), "output: {output}");
}

#[test]
fn roundtrip_key_bes_minor() {
    let output = roundtrip("{ \\key bes \\minor c'4 }");
    assert!(output.contains("\\key bes \\minor"), "output: {output}");
}

#[test]
fn roundtrip_time_3_4() {
    let output = roundtrip("{ \\time 3/4 c'4 }");
    assert!(output.contains("\\time 3/4"), "output: {output}");
}

#[test]
fn roundtrip_time_compound() {
    let output = roundtrip("{ \\time 2+3/8 c'4 }");
    assert!(output.contains("\\time 2+3/8"), "output: {output}");
}

#[test]
fn roundtrip_clef_key_time_combined() {
    let output = roundtrip("{ \\clef \"treble\" \\key d \\major \\time 4/4 c'4 d'4 e'4 f'4 }");
    assert!(output.contains("\\clef \"treble\""), "output: {output}");
    assert!(output.contains("\\key d \\major"), "output: {output}");
    assert!(output.contains("\\time 4/4"), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
}

#[test]
fn roundtrip_clef_change_mid_stream() {
    let output = roundtrip("{ \\clef \"treble\" c'4 d'4 \\clef \"bass\" e4 f4 }");
    assert!(output.contains("\\clef \"treble\""), "output: {output}");
    assert!(output.contains("\\clef \"bass\""), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
    assert!(output.contains("e4"), "output: {output}");
}

#[test]
fn roundtrip_key_change() {
    let output = roundtrip("{ \\key c \\major c'4 d'4 \\key g \\major e'4 f'4 }");
    assert!(output.contains("\\key c \\major"), "output: {output}");
    assert!(output.contains("\\key g \\major"), "output: {output}");
}

#[test]
fn roundtrip_time_change() {
    let output = roundtrip("{ \\time 4/4 c'4 d'4 \\time 3/4 e'4 f'4 }");
    assert!(output.contains("\\time 4/4"), "output: {output}");
    assert!(output.contains("\\time 3/4"), "output: {output}");
}

#[test]
fn roundtrip_transposed_clef() {
    let output = roundtrip("{ \\clef \"treble_8\" c4 }");
    assert!(output.contains("\\clef \"treble_8\""), "output: {output}");
}

#[test]
fn roundtrip_clef_key_time_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_clef_key_time.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(output.contains("\\clef \"treble\""), "output: {output}");
    assert!(output.contains("\\key d \\major"), "output: {output}");
    assert!(output.contains("\\time 4/4"), "output: {output}");
    assert!(output.contains("\\clef \"bass\""), "output: {output}");
    assert!(output.contains("\\key bes \\minor"), "output: {output}");
    assert!(output.contains("\\time 3/4"), "output: {output}");
    assert!(output.contains("\\time 2+3/8"), "output: {output}");
}

#[test]
fn roundtrip_contexts_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_contexts.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(output.contains("\\new StaffGroup"), "output: {output}");
    assert!(output.contains("\\new Staff"), "output: {output}");
    assert!(output.contains("\"violin\""), "output: {output}");
    assert!(output.contains("\"viola\""), "output: {output}");
    assert!(output.contains("c'4"), "output: {output}");
}

// --- Phase 7.2: Relative / transpose roundtrip tests ---

#[test]
fn roundtrip_relative_basic() {
    // \relative c' { c d e f } -> notes resolve to c' d' e' f'
    // On export, should wrap in \relative c' and use relative marks
    let output = roundtrip("\\relative c' { c4 d e f }");
    assert!(output.contains("\\relative"), "output: {output}");
    assert!(output.contains("c4"), "output: {output}");
    assert!(output.contains("d"), "output: {output}");
    assert!(output.contains("e"), "output: {output}");
    assert!(output.contains("f"), "output: {output}");
}

#[test]
fn roundtrip_relative_no_pitch() {
    // \relative { c d e f } -- default reference is f
    let output = roundtrip("\\relative { c4 d e f }");
    assert!(output.contains("\\relative"), "output: {output}");
}

#[test]
fn roundtrip_relative_octave_jump() {
    // \relative c' { c c' c, c } -- the ' and , adjust from closest position
    let output = roundtrip("\\relative c' { c4 c' c, c }");
    assert!(output.contains("\\relative"), "output: {output}");
    // Should contain notes with octave marks
    assert!(output.contains("c4"), "output: {output}");
}

#[test]
fn roundtrip_relative_with_accidentals() {
    let output = roundtrip("\\relative c' { c4 cis d bes }");
    assert!(output.contains("\\relative"), "output: {output}");
    assert!(output.contains("cis"), "output: {output}");
    assert!(output.contains("bes"), "output: {output}");
}

#[test]
fn roundtrip_relative_descending() {
    // In relative c': b is closest going down from c
    let output = roundtrip("\\relative c' { c4 b a g }");
    assert!(output.contains("\\relative"), "output: {output}");
    assert!(output.contains("b"), "output: {output}");
    assert!(output.contains("a"), "output: {output}");
    assert!(output.contains("g"), "output: {output}");
}

#[test]
fn roundtrip_transpose_basic() {
    // \transpose c d { c4 d e f } -> all pitches shifted up a whole step
    let output = roundtrip("\\transpose c d { c4 d e f }");
    assert!(output.contains("\\transpose"), "output: {output}");
    // The notes inside should be the original (un-transposed) pitches
    assert!(output.contains("c"), "output: {output}");
}

#[test]
fn roundtrip_transpose_with_accidentals() {
    let output = roundtrip("\\transpose c d { cis4 bes e fis }");
    assert!(output.contains("\\transpose"), "output: {output}");
    assert!(output.contains("cis"), "output: {output}");
}

#[test]
fn roundtrip_relative_in_staff() {
    // \relative inside \new Staff
    let output = roundtrip("\\new Staff \\relative c' { c4 d e f }");
    assert!(output.contains("\\relative"), "output: {output}");
    assert!(output.contains("\\new Staff"), "output: {output}");
}

#[test]
fn roundtrip_relative_transpose_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_relative_transpose.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    // The fixture has multiple top-level expressions; the importer picks the first.
    // The first is \relative c' { c4 d e f }
    assert!(output.contains("\\relative"), "output: {output}");
}

#[test]
fn roundtrip_chord_basic() {
    let output = roundtrip("{ <c' e' g'>4 }");
    assert!(output.contains("<c' e' g'>4"), "output: {output}");
}

#[test]
fn roundtrip_chord_dotted() {
    let output = roundtrip("{ <c' e'>2. }");
    assert!(output.contains("<c' e'>2."), "output: {output}");
}

#[test]
fn roundtrip_chord_with_accidentals() {
    let output = roundtrip("{ <cis' es' g'>4 }");
    // es -> ees canonical form (both valid LilyPond)
    assert!(output.contains("<cis' ees' g'>4"), "output: {output}");
}

#[test]
fn roundtrip_chord_force_cautionary() {
    let output = roundtrip("{ <cis'! e'?>4 }");
    assert!(output.contains("cis'!"), "output: {output}");
    assert!(output.contains("e'?"), "output: {output}");
}

#[test]
fn roundtrip_chord_mixed_with_notes() {
    let output = roundtrip("{ c'4 <d' f'>8 e'2 }");
    assert!(output.contains("c'4"), "output: {output}");
    assert!(output.contains("<d' f'>8"), "output: {output}");
    assert!(output.contains("e'2"), "output: {output}");
}

#[test]
fn roundtrip_chord_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_chords.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(output.contains("<c e g>4"), "output: {output}");
    // es -> ees canonical form (both valid LilyPond)
    assert!(output.contains("<c ees g>2."), "output: {output}");
    assert!(output.contains("<d' fis' a'>8"), "output: {output}");
    assert!(output.contains("<bes, d f>1"), "output: {output}");
    assert!(output.contains("cis''!"), "output: {output}");
    assert!(output.contains("e''?"), "output: {output}");
}

// --- Phase 9.2: Tie / Slur / Phrasing slur roundtrip tests ---

#[test]
fn roundtrip_tie() {
    let output = roundtrip("{ c4~ c4 }");
    assert!(output.contains("c4~"), "output: {output}");
}

#[test]
fn roundtrip_slur() {
    let output = roundtrip("{ d4( e4 f4) }");
    assert!(output.contains("d4("), "output: {output}");
    assert!(output.contains("f4)"), "output: {output}");
}

#[test]
fn roundtrip_phrasing_slur() {
    let output = roundtrip("{ g4\\( a4 b4\\) }");
    assert!(output.contains("g4\\("), "output: {output}");
    assert!(output.contains("b4\\)"), "output: {output}");
}

#[test]
fn roundtrip_chord_tie() {
    let output = roundtrip("{ <c e g>4~ <c e g>4 }");
    assert!(output.contains("<c e g>4~"), "output: {output}");
}

#[test]
fn roundtrip_combined_tie_and_slur() {
    let output = roundtrip("{ c4~( d4 e4) }");
    assert!(output.contains("~"), "output: {output}");
    assert!(output.contains("("), "output: {output}");
    assert!(output.contains(")"), "output: {output}");
}

#[test]
fn roundtrip_ties_slurs_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_ties_slurs.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(output.contains("~"), "output should contain tie: {output}");
    assert!(
        output.contains("("),
        "output should contain slur start: {output}"
    );
    assert!(
        output.contains(")"),
        "output should contain slur end: {output}"
    );
    assert!(
        output.contains("\\("),
        "output should contain phrasing slur start: {output}"
    );
    assert!(
        output.contains("\\)"),
        "output should contain phrasing slur end: {output}"
    );
}

// --- Phase 10.2: Beam roundtrip tests ---

#[test]
fn roundtrip_beam_basic() {
    let output = roundtrip("{ c8[ d e f] }");
    assert!(
        output.contains("["),
        "output should contain beam start: {output}"
    );
    assert!(
        output.contains("]"),
        "output should contain beam end: {output}"
    );
    // First note should have [ and last should have ]
    assert!(
        output.contains("c8["),
        "first note should have beam start: {output}"
    );
    assert!(
        output.contains("f]"),
        "last note should have beam end: {output}"
    );
}

#[test]
fn roundtrip_multiple_beams() {
    let output = roundtrip("{ c8[ d] e8[ f] }");
    // Should have two beam groups
    let bracket_opens: Vec<_> = output.match_indices('[').collect();
    let bracket_closes: Vec<_> = output.match_indices(']').collect();
    assert_eq!(
        bracket_opens.len(),
        2,
        "should have 2 beam starts: {output}"
    );
    assert_eq!(bracket_closes.len(), 2, "should have 2 beam ends: {output}");
}

#[test]
fn roundtrip_beam_with_unbeamed() {
    let output = roundtrip("{ c4 d8[ e f] g4 }");
    assert!(output.contains("c4"), "unbeamed note preserved: {output}");
    assert!(output.contains("d8["), "beam start: {output}");
    assert!(output.contains("f]"), "beam end: {output}");
    assert!(output.contains("g4"), "unbeamed note preserved: {output}");
}

#[test]
fn roundtrip_autobeam_commands() {
    let output = roundtrip("{ \\autoBeamOff c8 d \\autoBeamOn e8 }");
    assert!(
        output.contains("\\autoBeamOff"),
        "autoBeamOff preserved: {output}"
    );
    assert!(
        output.contains("\\autoBeamOn"),
        "autoBeamOn preserved: {output}"
    );
}

#[test]
fn roundtrip_beam_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_beams.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(
        output.contains("["),
        "output should contain beam start: {output}"
    );
    assert!(
        output.contains("]"),
        "output should contain beam end: {output}"
    );
    assert!(
        output.contains("\\autoBeamOff"),
        "autoBeamOff preserved: {output}"
    );
    assert!(
        output.contains("\\autoBeamOn"),
        "autoBeamOn preserved: {output}"
    );
}

// --- Phase 11.2: Dynamics and hairpin roundtrip tests ---

#[test]
fn roundtrip_dynamic_f() {
    let output = roundtrip("{ c4\\f d4 }");
    assert!(output.contains("\\f"), "output: {output}");
    assert!(output.contains("c4"), "output: {output}");
}

#[test]
fn roundtrip_dynamic_p() {
    let output = roundtrip("{ c4\\p d4 }");
    assert!(output.contains("\\p"), "output: {output}");
}

#[test]
fn roundtrip_multiple_dynamics() {
    let output = roundtrip("{ c4\\f d4\\p e4\\ff f4\\pp }");
    assert!(output.contains("\\f"), "output: {output}");
    assert!(output.contains("\\p"), "output: {output}");
    assert!(output.contains("\\ff"), "output: {output}");
    assert!(output.contains("\\pp"), "output: {output}");
}

#[test]
fn roundtrip_crescendo_hairpin() {
    let output = roundtrip("{ c4\\< d4 e4\\! }");
    assert!(
        output.contains("\\<"),
        "output should contain crescendo: {output}"
    );
    assert!(
        output.contains("\\!"),
        "output should contain hairpin end: {output}"
    );
}

#[test]
fn roundtrip_decrescendo_hairpin() {
    let output = roundtrip("{ c4\\> d4 e4\\! }");
    assert!(
        output.contains("\\>"),
        "output should contain decrescendo: {output}"
    );
    assert!(
        output.contains("\\!"),
        "output should contain hairpin end: {output}"
    );
}

#[test]
fn roundtrip_dynamic_with_hairpin() {
    let output = roundtrip("{ c4\\f\\< d4 e4\\!\\ff }");
    assert!(output.contains("\\f"), "output: {output}");
    assert!(output.contains("\\<"), "output: {output}");
    assert!(output.contains("\\!"), "output: {output}");
    assert!(output.contains("\\ff"), "output: {output}");
}

#[test]
fn roundtrip_dynamics_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_dynamics.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    assert!(output.contains("\\f"), "output: {output}");
    assert!(output.contains("\\p"), "output: {output}");
    assert!(output.contains("\\<"), "output: {output}");
    assert!(output.contains("\\>"), "output: {output}");
    assert!(output.contains("\\!"), "output: {output}");
    assert!(output.contains("\\ff"), "output: {output}");
    assert!(output.contains("\\mp"), "output: {output}");
    assert!(output.contains("\\mf"), "output: {output}");
    assert!(output.contains("\\sfz"), "output: {output}");
    assert!(output.contains("\\fp"), "output: {output}");
}

// ---------------------------------------------------------------------------
// Articulation & fingering roundtrip tests
// ---------------------------------------------------------------------------

#[test]
fn roundtrip_staccato() {
    let output = roundtrip("{ c4-. }");
    assert!(
        output.contains("-."),
        "output should contain staccato: {output}"
    );
}

#[test]
fn roundtrip_accent() {
    let output = roundtrip("{ c4-> }");
    assert!(
        output.contains("->"),
        "output should contain accent: {output}"
    );
}

#[test]
fn roundtrip_tenuto() {
    let output = roundtrip("{ c4-- }");
    assert!(
        output.contains("--"),
        "output should contain tenuto: {output}"
    );
}

#[test]
fn roundtrip_marcato() {
    let output = roundtrip("{ c4-^ }");
    assert!(
        output.contains("-^"),
        "output should contain marcato: {output}"
    );
}

#[test]
fn roundtrip_staccatissimo() {
    let output = roundtrip("{ c4-! }");
    assert!(
        output.contains("-!"),
        "output should contain staccatissimo: {output}"
    );
}

#[test]
fn roundtrip_portato() {
    let output = roundtrip("{ c4-_ }");
    assert!(
        output.contains("-_"),
        "output should contain portato: {output}"
    );
}

#[test]
fn roundtrip_stopped() {
    let output = roundtrip("{ c4-+ }");
    assert!(
        output.contains("-+"),
        "output should contain stopped: {output}"
    );
}

#[test]
fn roundtrip_artic_with_direction_up() {
    let output = roundtrip("{ c4^. }");
    assert!(
        output.contains("^."),
        "output should contain up-staccato: {output}"
    );
}

#[test]
fn roundtrip_artic_with_direction_down() {
    let output = roundtrip("{ c4_- }");
    assert!(
        output.contains("_-"),
        "output should contain down-tenuto: {output}"
    );
}

#[test]
fn roundtrip_fingering() {
    let output = roundtrip("{ c4-1 d4-2 }");
    assert!(
        output.contains("-1"),
        "output should contain fingering 1: {output}"
    );
    assert!(
        output.contains("-2"),
        "output should contain fingering 2: {output}"
    );
}

#[test]
fn roundtrip_fingering_with_direction() {
    let output = roundtrip("{ c4^3 d4_4 }");
    assert!(
        output.contains("^3"),
        "output should contain up-fingering: {output}"
    );
    assert!(
        output.contains("_4"),
        "output should contain down-fingering: {output}"
    );
}

#[test]
fn roundtrip_named_articulation() {
    let output = roundtrip("{ c4-\\staccato }");
    // Named artics with a known script abbreviation get roundtripped as the abbreviation
    assert!(
        output.contains("-."),
        "named staccato should roundtrip as abbreviation: {output}"
    );
}

#[test]
fn roundtrip_multiple_artics_on_note() {
    let output = roundtrip("{ c4-. -3 }");
    assert!(
        output.contains("-."),
        "output should contain staccato: {output}"
    );
    assert!(
        output.contains("-3"),
        "output should contain fingering: {output}"
    );
}

#[test]
fn roundtrip_artics_fixture() {
    let src = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../tests/fixtures/lilypond/fragment_articulations.ly"
    ))
    .unwrap();
    let output = roundtrip(&src);
    // Check all 7 script abbreviations
    assert!(output.contains("-."), "staccato: {output}");
    assert!(output.contains("->"), "accent: {output}");
    assert!(output.contains("-+"), "stopped: {output}");
    assert!(output.contains("--"), "tenuto: {output}");
    assert!(output.contains("-!"), "staccatissimo: {output}");
    assert!(output.contains("-_"), "portato: {output}");
    assert!(output.contains("-^"), "marcato: {output}");
    // Check directional articulations
    assert!(output.contains("^."), "up-staccato: {output}");
    assert!(output.contains("^>"), "up-accent: {output}");
    assert!(output.contains("_."), "down-staccato: {output}");
    assert!(output.contains("_-"), "down-tenuto: {output}");
    // Check fingerings
    assert!(output.contains("-1"), "fingering 1: {output}");
    assert!(output.contains("-2"), "fingering 2: {output}");
    assert!(output.contains("^3"), "up-fingering 3: {output}");
    assert!(output.contains("_4"), "down-fingering 4: {output}");
}

# Roundtrip Testing Tasks

Tasks generated from MusicXML → MEI → MusicXML roundtrip tests. Each task documents a missing conversion, unsupported element, or discrepancy found during testing.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: The `tusk_roundtrip.sh` script both implements existing tasks AND generates new ones when issues are discovered during roundtrip testing.

---

## Initial Roundtrip Infrastructure

### Setup
- [x] Create roundtrip test harness in `crates/formats/musicxml/tests/roundtrip.rs`
- [x] Add test helper: parse MusicXML → convert to MEI → convert back to MusicXML
- [x] Add comparison logic to detect differences between input and output
- [x] Run roundtrip tests on all fixtures in `tests/fixtures/musicxml/`

### Basic Fixtures
- [x] Roundtrip test: `hello_world.musicxml`
- [x] Roundtrip test: `scale.musicxml`
- [x] Roundtrip test: `durations.musicxml`
- [x] Roundtrip test: `chords_and_rests.musicxml`
- [x] Roundtrip test: `high_divisions.musicxml`
- [x] Roundtrip test: `directions.musicxml`

### Spec Example Fixtures
- [x] Roundtrip test: `specs/musicxml/examples/Telemann.musicxml`
- [x] Roundtrip test: `specs/musicxml/examples/Binchois.musicxml`
- [x] Roundtrip test: `specs/musicxml/examples/MozartPianoSonata.musicxml`
- [x] Roundtrip test: `specs/musicxml/examples/ActorPreludeSample.musicxml`
- [x] Roundtrip test: `specs/musicxml/examples/BeetAnGeSample.musicxml`
- [x] Roundtrip test: `specs/musicxml/examples/BrahWiMeSample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/BrookeWestSample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/Chant.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/DebuMandSample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/Dichterliebe01.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/Echigo-Jishi.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/FaurReveSample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/MahlFaGe4Sample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/MozaChloSample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/MozartTrio.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/MozaVeilSample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/Saltarello.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/SchbAvMaSample.musicxml`

### Extracted Spec Doc Examples (Complete Files)
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/assess_and_player_elements.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/concert_score_and_for_part_elements.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/instrument_change_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/movement_number_and_movement_title_elements.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/score_timewise_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/tutorial_apres_un_reve.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/tutorial_chopin_prelude.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/tutorial_chord_symbols.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/tutorial_hello_world.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/tutorial_percussion.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/spec_examples/tutorial_tablature.musicxml`

### Fragment Examples (275 files)

Fragment examples extracted from spec docs, wrapped in complete MusicXML structure.

- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/accent_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/accidental_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/accidental_mark_element_notation.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/accidental_mark_element_ornament.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/accordion_high_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/accordion_low_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/accordion_middle_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/accordion_registration_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/alter_element_microtones.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/alter_element_semitones.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/alto_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/arpeggiate_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/arrow_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/arrowhead_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/articulations_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/artificial_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/attributes_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/backup_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/baritone_c_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/baritone_f_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/barline_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/barre_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/bass_alter_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/bass_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/bass_clef_down_octave.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/bass_separator_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/bass_step_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/beam_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/beat_repeat_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/beat_type_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/beat_unit_dot_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/beat_unit_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/beat_unit_tied_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/beater_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/beats_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/bend_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/bookmark_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/bracket_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/brass_bend_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/breath_mark_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/caesura_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/cancel_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/capo_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/chord_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/chord_element_multiple_stop.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/circular_arrow_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/coda_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/cue_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/damp_all_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/damp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/dashes_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/degree_alter_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/degree_type_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/degree_value_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/delayed_inverted_turn_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/delayed_turn_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/detached_legato_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/divisions_and_duration_elements.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/doit_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/dot_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/double_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/double_tongue_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/down_bow_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/effect_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/elision_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/end_line_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/end_paragraph_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/ending_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/ensemble_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/except_voice_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/extend_element_figure.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/extend_element_lyric.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/eyeglasses_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/f_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/falloff_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fermata_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/ff_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fff_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/ffff_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fffff_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/ffffff_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/figure_number_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fingering_element_frame.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fingering_element_notation.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fingernails_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/flip_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/footnote_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/forward_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fret_element_frame.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/fz_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/glass_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/glissando_element_multiple.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/glissando_element_single.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/glyph_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/golpe_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/grace_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/grace_element_appoggiatura.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/group_abbreviation_display_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/group_abbreviation_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/group_barline_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/group_name_display_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/group_time_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/grouping_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/half_muted_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/handbell_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/harmon_mute_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/harp_pedals_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/haydn_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/heel_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/heel_toe_substitution.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/hole_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/hole_type_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/humming_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/image_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/instrument_link_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/interchangeable_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/inversion_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/inverted_mordent_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/inverted_turn_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/inverted_vertical_turn_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/ipa_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/key_element_non_traditional.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/key_element_traditional.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/key_octave_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/kind_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/laughing_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/level_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/line_detail_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/line_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/link_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/lyric_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/measure_distance_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/measure_numbering_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/measure_repeat_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/membrane_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/metal_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/metronome_arrows_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/metronome_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/metronome_note_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/metronome_tied_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/mezzo_soprano_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/mf_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/midi_device_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/midi_instrument_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/midi_name_and_midi_bank_elements.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/midi_unpitched_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/mordent_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/mp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/multiple_rest_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/n_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/natural_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/non_arpeggiate_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/normal_dot_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/notehead_text_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/numeral_alter_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/numeral_key_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/numeral_root_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/octave_change_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/octave_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/octave_shift_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/open_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/open_string_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/p_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pan_and_elevation_elements.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/part_abbreviation_display_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/part_link_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/part_name_display_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/part_symbol_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pedal_element_lines.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pedal_element_symbols.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/per_minute_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/percussion_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pf_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pitch_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pitched_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/plop_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pluck_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/ppp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pppp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/ppppp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pppppp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/pre_bend_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/prefix_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/principal_voice_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/rehearsal_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/release_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/repeat_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/rest_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/rf_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/rfz_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/root_alter_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/root_step_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/schleifer_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/scoop_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/scordatura_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/segno_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/senza_misura_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/sf_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/sffz_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/sfp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/sfpp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/sfz_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/sfzp_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/shake_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/slash_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/slash_type_and_slash_dot_elements.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/slide_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/slur_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/smear_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/snap_pizzicato_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/soft_accent_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/soprano_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/spiccato_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staccatissimo_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staccato_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staff_distance_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staff_divide_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staff_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staff_lines_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staff_size_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staff_tuning_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staff_type_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/staves_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/step_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/stick_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/stick_location_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/stopped_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/straight_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/stress_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/string_mute_element_off.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/string_mute_element_on.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/strong_accent_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/suffix_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/swing_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/syllabic_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/symbol_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/sync_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/system_attribute_also_top.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/system_attribute_only_top.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/system_distance_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/system_dividers_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tab_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tap_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/technical_element_tablature.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tenor_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tenuto_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/thumb_position_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tied_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/time_modification_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/timpani_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/toe_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/transpose_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/treble_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tremolo_element_double.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tremolo_element_single.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/trill_mark_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/triple_tongue_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tuplet_dot_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tuplet_element_nested.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/tuplet_element_regular.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/turn_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/unpitched_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/unstress_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/up_bow_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/vertical_turn_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/virtual_instrument_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/vocal_tenor_clef.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/voice_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/wait_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/wavy_line_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/wedge_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/with_bar_element.musicxml`
- [ ] Roundtrip test: `tests/fixtures/musicxml/fragment_examples/wood_element.musicxml`

---

## Completed Bug Fixes

- [x] [BUGFIX] Clef selection used global staff number instead of part-internal staff number (source: Telemann.musicxml)
  - Multi-staff parts (like piano) have clefs with `number=1` and `number=2` within the part
  - Fixed to use `number=1` or `None` for first staffDef of each part

- [x] [BUGFIX] Part-group nesting was incorrect when outer groups closed before inner groups (source: ActorPreludeSample.musicxml)
  - When a part-group stop was encountered, any groups pushed after it (still on stack) were not moved inside the closing group
  - Example: `<part-group 2 start> P14 <part-group 1 start> P15 P16 <part-group 2 stop>` - group 1 should be nested inside group 2
  - Fixed import/parts.rs to move inner groups into the closing outer group before closing

---

## Generated Tasks

<!-- Tasks below this line are auto-generated by tusk_roundtrip.sh -->
<!-- Format: - [ ] [CATEGORY] Description (source: filename.musicxml) -->

- [x] [MISSING_ELEMENT] Add support for unpitched notes in MusicXML import/export (source: ActorPreludeSample.musicxml)
  - Percussion parts (P15, P16) use `<unpitched>` instead of `<pitch>`
  - Fixed: MusicXML unpitched notes now convert to MEI using @loc attribute
  - MEI notes without @pname are converted back to MusicXML unpitched elements

- [x] [MISSING_ATTR] Percussion clef should preserve line=None when original has no line specified (source: ActorPreludeSample.musicxml)
  - P15, P16 Measure 1: clef line mismatch: original=None, roundtripped=Some(2)
  - Fixed: clef_line is now unconditionally set when processing clef (even if None)

---

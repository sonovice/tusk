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
- [ ] Roundtrip test: `specs/musicxml/examples/ActorPreludeSample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/BeetAnGeSample.musicxml`
- [ ] Roundtrip test: `specs/musicxml/examples/BrahWiMeSample.musicxml`
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
Located in `tests/fixtures/musicxml/fragment_examples/`.

- [ ] Add roundtrip tests for all fragment examples in `tests/fixtures/musicxml/fragment_examples/`
- [ ] Run batch roundtrip validation on fragment_examples directory

Categories covered:
- Articulations: accent, staccato, tenuto, etc.
- Beams and tuplets
- Clefs: treble, bass, alto, tenor, percussion, tab, etc.
- Directions: dynamics (p, f, ff, etc.), tempo, rehearsal marks
- Harmony: chords, figured bass
- Notations: slurs, ties, ornaments, technical markings
- Note elements: pitch, duration, accidentals, grace notes
- Time signatures and key signatures
- Barlines and repeats
- And many more...

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

- [ ] [MISSING_ELEMENT] Add support for unpitched notes in MusicXML import/export (source: ActorPreludeSample.musicxml)
  - Percussion parts (P15, P16) use `<unpitched>` instead of `<pitch>`
  - Currently converted to pitched notes during roundtrip
  - Need to add handling for `FullNoteContent::Unpitched` in import/note.rs and export/note.rs

- [ ] [MISSING_ATTR] Percussion clef should preserve line=None when original has no line specified (source: ActorPreludeSample.musicxml)
  - P15, P16 Measure 1: clef line mismatch: original=None, roundtripped=Some(2)
  - Percussion staves may omit clef line; roundtrip adds default line=2

---

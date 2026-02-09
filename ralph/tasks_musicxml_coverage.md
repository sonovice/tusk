# MusicXML Coverage Tasks

Tasks for achieving full MusicXML 4.1 ↔ MEI bidirectional conversion coverage. Derived from the comprehensive plan in `docs/musicxml.md`.

Each task covers: `[P]` Parser, `[S]` Serializer, `[I]` Import (MusicXML→MEI), `[E]` Export (MEI→MusicXML), `[T]` Tests.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: The `tusk_musicxml_coverage.sh` script runs tests/clippy, feeds results + this task list to Claude, which works on one section at a time.

**Constraint**: Every change must pass `cargo test` and `cargo clippy --all-targets` with no regressions.

**Key references**:
- Coverage plan: `docs/musicxml.md`
- MusicXML XSD: `specs/musicxml/schema/musicxml.xsd`
- MusicXML model: `crates/formats/musicxml/src/model/`
- Parser: `crates/formats/musicxml/src/parser.rs` + `parser/` submodules
- Serializer: `crates/formats/musicxml/src/serializer/`
- Import (MusicXML→MEI): `crates/formats/musicxml/src/import/`
- Export (MEI→MusicXML): `crates/formats/musicxml/src/export/`
- Conversion context: `crates/formats/musicxml/src/context/`
- Fragment test fixtures: `tests/fixtures/musicxml/fragment_examples/`
- Roundtrip tests: `crates/formats/musicxml/tests/roundtrip.rs`
- MEI model (generated): `crates/core/model/src/generated/`

---

## Phase 1: Notations — Tuplets

### 1.1 Model & Parser

- [x] Add `Tuplet` struct to `model/notations.rs` (tuplet_type start/stop, number, bracket, show_number, show_type, placement, actual_notes, normal_notes, actual_type, normal_type)
  - Added `Tuplet`, `TupletPortion`, `TupletNumber`, `TupletType`, `TupletDot`, `ShowTuplet` types
  - Tuplet has attributes: type (StartStop), number, bracket (YesNo), show-number/show-type (ShowTuplet), line-shape (LineShape), placement (AboveBelow)
  - TupletPortion has optional tuplet-number, tuplet-type, and Vec of tuplet-dots
- [x] Add `tuplets: Vec<Tuplet>` field to `Notations` struct
- [x] Parse `<tuplet>` inside `parse_notations()` in `parser/parse_note.rs` (currently falls through to `skip_element`)
  - Handles both empty (self-closing) and start-tag tuplet elements
  - Parses all attributes and optional tuplet-actual/tuplet-normal children
  - Added `parse_tuplet`, `parse_tuplet_empty`, `parse_tuplet_attrs`, `parse_tuplet_portion`, `parse_note_type_value` functions
- [x] Serialize `<tuplet>` in `serializer/score.rs` within `Notations` serialization
  - Tuplet implements MusicXmlSerialize; emits as empty element when no children
  - `serialize_tuplet_portion` helper writes tuplet-actual/tuplet-normal children
  - Added `show_tuplet_str` and `line_shape_str` helper functions
- [x] Verify `time-modification` parsing completeness (actual-notes, normal-notes, normal-type, normal-dot)
  - Fixed: now parses `<normal-dot>` empty elements (was previously always Vec::new())
  - Fixed: `normal-type` now uses full `parse_note_type_value` covering all 14 NoteTypeValue variants (was only 6)

### 1.2 Import: MusicXML Tuplets → MEI

- [x] Detect tuplet boundaries from `<tuplet type="start/stop">` notations in `import/note.rs`
  - Added `process_tuplets()` in `import/note.rs`, following slur pattern
  - On start: creates PendingTuplet with time-modification ratio and visual attributes
  - On stop: resolves pending tuplet into CompletedTuplet
- [x] Emit MEI `<tupletSpan>` control events on measures in `import/structure.rs`
  - Uses TupletSpan (measure-level control event) instead of Tuplet container, matching slur pattern
  - Added `emit_tuplet_spans()` in `import/structure.rs`
  - Added TupletSpan to MeasureChild via codegen EXTRA_CHILDREN
- [x] Map `time-modification` → MEI `@num` and `@numbase` on the tupletSpan element
- [x] Map bracket/show-number/show-type → MEI `@bracket.visible`, `@num.visible`, `@num.format`
  - bracket → @bracket.visible (true/false)
  - show-number=actual → @num.visible=true; both → @num.format=ratio; none → @num.visible=false
  - placement → @num.place and @bracket.place (above/below)
- [x] Handle nested tuplets (tuplet number attribute distinguishes nesting levels)
  - PendingTuplet tracks number (1-6) for matching start/stop pairs
- [x] Handle tuplets that span across beams
  - TupletSpan is a measure-level control event referencing notes by startid/endid, independent of beam structure

### 1.3 Export: MEI Tuplets → MusicXML

- [x] Detect MEI `<tupletSpan>` control events in `export/content.rs`
  - Added `convert_tuplet_events()` called after slur events
- [x] Emit `<time-modification>` on each note inside the tuplet
  - All notes between startid and endid get time-modification with num/numbase
- [x] Emit `<tuplet type="start">` on first note, `<tuplet type="stop">` on last note (as notations)
- [x] Map MEI `@num`/`@numbase` → `actual-notes`/`normal-notes`
- [x] Handle nested tuplets with proper numbering
  - Each tupletSpan creates its own start/stop pair; nested tuplets produce multiple tuplet notations on same note

### 1.4 Tests

- [x] Verify fragment examples roundtrip: `tuplet_element_regular`, `tuplet_element_nested`, `tuplet_dot_element`
  - All 3 tuplet fragment tests pass all 4 roundtrip levels (conversion, full, triangle MEI, triangle MusicXML)
- [x] All 310 roundtrip tests pass, 478 unit tests pass, 31 MEI→MusicXML tests pass

---

## Phase 2: Multi-Staff Parts

### 2.1 Import: Multi-Staff Detection & Mapping

- [x] Detect `<staves>` element in `<attributes>` to determine multi-staff parts in `import/parts.rs`
  - Added `extract_attributes_with_staves()` to detect `<staves>N</staves>` in first measure attributes
- [x] Create multiple `<staffDef>` elements within a single `<staffGrp>` for multi-staff parts (resolve the TODO at `import/parts.rs:299`)
  - Added `convert_multi_staff_part()` creating nested `<staffGrp symbol="brace">` with multiple `<staffDef>` elements
  - Extended `convert_staff_def_from_score_part()` with `clef_number` and `include_label` params
- [x] Route notes to correct MEI `<staff>` based on `<staff>` child in MusicXML notes
  - All notes from multi-staff parts go into first MEI staff for roundtrip fidelity (notes carry `@staff` attribute)
- [x] Propagate clefs, key signatures, and time signatures to all staves in the part
  - Each `<staffDef>` gets its own clef from `<clef number="N">`; key/time shared across staves
- [x] Handle cross-staff notation (`<staff>` element on notes placing them on a different staff)
  - Notes with `<staff>` differing from voice default get `@staff` attribute in MEI referencing global staff number
- [x] Handle `<backup>`/`<forward>` across staves within a part
  - Backup/forward durations tracked per-part; `register_part_staff()` maps (part_id, local_staff) → global_staff

### 2.2 Export: Multi-Staff → MusicXML

- [x] Detect multi-`<staffDef>` within a single part `<staffGrp>` in `export/parts.rs`
  - Added `is_multi_staff_part()` detecting brace symbol + ≥2 staffDefs + no nested staffGrp + no individual labels
  - Added `convert_multi_staff_grp_to_score_part()` for multi-staff part export
- [x] Emit `<staves>` in `<attributes>` in `export/attributes.rs`
  - `build_first_measure_attributes_multi()` emits `<staves>`, multiple `<clef number="N">`, key/time
  - Extracted from `content.rs` to `attributes.rs` to keep file under 1500 lines
- [x] Route notes from multiple `<staff>` elements into the same `<part>` with proper `<staff>` tags
  - Multi-staff export branch merges MEI staves into single MusicXML part with `<staff>` tags
- [x] Manage `<backup>` for cross-staff voice movement in `export/content.rs`
  - `calculate_staff_duration()` computes backup duration; `<backup>` inserted between staves
  - Fixed single-staff parts not calling `register_part_staff()` (caused ActorPreludeSample regression)

### 2.3 Tests

- [x] Add roundtrip fixture: `piano_two_staves.musicxml`
  - Piano with 2 staves, treble/bass clef, half notes + whole note with backup
- [x] Add roundtrip fixture: `organ_three_staves.musicxml`
  - Organ with 3 staves (2 manuals + pedal), treble/treble/bass clefs
- [x] Add roundtrip fixture: `cross_staff_notes.musicxml`
  - Piano with voice 1 notes crossing between staves (staff 1 → staff 2 → staff 1)
- [x] Verify existing spec examples with multi-staff parts roundtrip correctly (MozartPianoSonata, Telemann, etc.)
  - 313/313 roundtrip tests pass including ActorPreludeSample (21 parts with multi-staff harp)

---

## Phase 3: Notations — Ornaments

### 3.1 Model & Parser

- [ ] Create `Ornaments` struct in `model/notations.rs` with all ornament types: `trill_mark`, `turn`, `delayed_turn`, `inverted_turn`, `delayed_inverted_turn`, `vertical_turn`, `inverted_vertical_turn`, `shake`, `mordent`, `inverted_mordent`, `wavy_line`, `schleifer`, `tremolo`, `haydn`, `other_ornament`, `accidental_marks`
- [ ] Create supporting structs: `TrillMark` (placement, start-note, trill-step, two-note-turn, accelerate, beats, second-beat, last-beat), `Tremolo` (type single/start/stop, value 1-8), `WavyLine` (type, number, placement), `Mordent` (placement, long, approach, departure)
- [ ] Add `ornaments: Option<Ornaments>` field to `Notations` struct
- [ ] Parse `<ornaments>` inside `parse_notations()` → implement `parse_ornaments()` function
- [ ] Serialize all ornament types in the serializer

### 3.2 Import: MusicXML Ornaments → MEI

- [ ] `trill-mark` → MEI `<trill>` control event with `@place`, `@startid`
- [ ] `mordent`/`inverted-mordent` → MEI `<mordent>` with `@form` (lower/upper), `@long`
- [ ] `turn`/`delayed-turn`/`inverted-turn`/`delayed-inverted-turn` → MEI `<turn>` with `@form` (upper/lower), `@delayed`
- [ ] `vertical-turn`/`inverted-vertical-turn` → MEI `<turn>` with label for roundtrip
- [ ] `shake` → MEI `<ornam>` with label
- [ ] `schleifer` → MEI `<ornam>` with `@label="schleifer"`
- [ ] `tremolo type="single"` → MEI `<bTrem>` container around note
- [ ] `tremolo type="start/stop"` → MEI `<fTrem>` container
- [ ] `tremolo` value (beam count) → MEI `@unitdur` or `@measperf`
- [ ] `haydn` → MEI `<turn>` variant with label
- [ ] `wavy-line` → MEI trill extension line
- [ ] `accidental-mark` within ornaments → MEI `@accidlower`/`@accidupper`

### 3.3 Export: MEI Ornaments → MusicXML

- [ ] MEI `<trill>` → `trill-mark` (+ `wavy-line` if extended)
- [ ] MEI `<mordent>` → `mordent` / `inverted-mordent` based on `@form`
- [ ] MEI `<turn>` → `turn` / `inverted-turn` / `delayed-turn` / `delayed-inverted-turn` based on `@form` and `@delayed`
- [ ] MEI `<bTrem>` → `tremolo type="single"` on contained note
- [ ] MEI `<fTrem>` → `tremolo type="start/stop"` on contained notes
- [ ] MEI labeled ornaments → roundtrip back to correct MusicXML type

### 3.4 Tests

- [ ] Add roundtrip fixtures for trills, mordents, turns, tremolos
- [ ] Verify fragment examples: `trill_mark_element`, `mordent_element`, `inverted_mordent_element`, `turn_element`, `delayed_turn_element`, `inverted_turn_element`, `delayed_inverted_turn_element`, `vertical_turn_element`, `inverted_vertical_turn_element`, `shake_element`, `schleifer_element`, `tremolo_element_single`, `tremolo_element_double`, `haydn_element`, `wavy_line_element`

---

## Phase 4: Notations — Fermata, Arpeggiate, Glissando, Slide

### 4.1 Model & Parser

- [ ] Add to `Notations` struct: `fermatas: Vec<Fermata>`, `arpeggiate: Option<Arpeggiate>`, `non_arpeggiate: Option<NonArpeggiate>`, `glissandos: Vec<Glissando>`, `slides: Vec<Slide>`, `accidental_marks: Vec<AccidentalMark>`, `other_notations: Vec<OtherNotation>`
- [ ] Create structs: `Fermata` (shape, type upright/inverted, placement), `Arpeggiate` (number, direction up/down, placement), `NonArpeggiate` (type top/bottom, number, placement), `Glissando` (type start/stop, number, line-type, text, placement), `Slide` (type start/stop, number, line-type, text, placement), `AccidentalMark` (value, placement, parentheses), `OtherNotation` (type, text, placement)
- [ ] Parse all in `parse_notations()`
- [ ] Serialize all in serializer

### 4.2 Import & Export

- [ ] `fermata` → MEI `<fermata>` control event with `@shape`, `@form`, `@place`, `@startid`
- [ ] `arpeggiate` → MEI `<arpeg>` control event with `@order` (up/down)
- [ ] `non-arpeggiate` → MEI `<arpeg>` with `@order="nonarp"`
- [ ] `glissando` → MEI `<gliss>` control event with `@startid`/`@endid`, `@lform`
- [ ] `slide` → MEI `<gliss>` with slide semantics or label for roundtrip
- [ ] `accidental-mark` (standalone) → MEI `<accid>` element or attribute
- [ ] Export: reverse all mappings

### 4.3 Tests

- [ ] Add roundtrip fixtures for fermata, arpeggiate, glissando, slide
- [ ] Verify fragment examples: `fermata_element`, `arpeggiate_element`, `non_arpeggiate_element`, `glissando_element_single`, `glissando_element_multiple`, `slide_element`, `accidental_mark_element_notation`

---

## Phase 5: Notations — Technical

### 5.1 Model & Parser

- [ ] Add `technical: Option<Technical>` field to `Notations` struct
- [ ] Create `Technical` struct with all 30+ types as `Option<_>`: up_bow, down_bow, harmonic, open_string, thumb_position, fingering, pluck, snap_pizzicato, stopped, fret, string, hammer_on, pull_off, bend, tap, heel, toe, double_tongue, triple_tongue, fingernails, hole, arrow, brass_bend, flip, smear, open, half_muted, harmon_mute, golpe, handbell, other_technical
- [ ] Create supporting structs for complex types: `Harmonic`, `Fingering`, `Bend`, `HammerOn`, `PullOff`, `Hole`, `Arrow`, `HarmonMute`
- [ ] Parse all in `parse_notations()` → `parse_technical()` function
- [ ] Serialize all in serializer

### 5.2 Import & Export

- [ ] Bowing marks (up-bow, down-bow) → MEI `<artic>` or dedicated elements
- [ ] `fingering` → MEI `<fing>` element
- [ ] `bend`, `hammer-on`, `pull-off` → MEI guitar/tablature elements or labels for roundtrip
- [ ] For elements without direct MEI equivalents: store with `musicxml:` label for lossless roundtrip
- [ ] Export: reverse all mappings

### 5.3 Tests

- [ ] Add roundtrip fixtures for key technical notations
- [ ] Verify fragment examples: `up_bow_element`, `down_bow_element`, `open_string_element`, `thumb_position_element`, `snap_pizzicato_element`, `stopped_element`, `double_tongue_element`, `triple_tongue_element`, `fingernails_element`, `pluck_element`, `tap_element`, `heel_element`, `toe_element`, `heel_toe_substitution`, `fingering_element_notation`, `bend_element`, `brass_bend_element`, `flip_element`, `smear_element`, `open_element`, `half_muted_element`, `harmon_mute_element`, `golpe_element`, `handbell_element`, `hole_element`, `hole_type_element`, `arrow_element`, `arrowhead_element`, `circular_arrow_element`, `pre_bend_element`, `with_bar_element`, `technical_element_tablature`

---

## Phase 6: Notations — Dynamics within Notations

### 6.1 Model, Parser, Import, Export

- [ ] Add `dynamics: Vec<Dynamics>` field to `Notations` struct (reuse existing `Dynamics`/`DynamicsValue` from `model/direction/dynamics.rs`)
- [ ] Parse `<dynamics>` within `parse_notations()` (currently only parsed within `<direction-type>`)
- [ ] Serialize dynamics within notations
- [ ] Import: notation-level dynamics → MEI `<dynam>` with `@startid` referencing the note
- [ ] Export: MEI `<dynam>` attached to specific note → notation-level dynamics

### 6.2 Tests

- [ ] Add roundtrip fixture with notation-level dynamics
- [ ] Verify dynamics in notations context produce correct output

---

## Phase 7: Lyrics

### 7.1 Model & Parser

- [ ] Create `model/lyric.rs` with: `Lyric` (number, name, placement, justify, default-x/y, children), `Syllabic` enum (single/begin/middle/end), `LyricText` (text, font/color), `Elision` (text, font), `Extend` (type start/stop/continue)
- [ ] Add `lyrics: Vec<Lyric>` field to `Note` struct
- [ ] Parse `<lyric>` within `parse_note()` (currently falls through to `skip_element`)
- [ ] Serialize all lyric elements

### 7.2 Import: MusicXML Lyrics → MEI

- [ ] `<lyric>` on notes → MEI `<syl>` children on `<note>` elements
- [ ] `<syllabic>` (single/begin/middle/end) → MEI `@wordpos` and `@con` attributes
- [ ] `<text>` content → MEI `<syl>` text
- [ ] `<elision>` → MEI elision handling
- [ ] Lyric `number` → MEI `<verse>` `@n` for multi-verse support
- [ ] `<extend>` → MEI extender line (underscore continuation)
- [ ] Handle `<humming>` and `<laughing>` special syllable types

### 7.3 Export: MEI Lyrics → MusicXML

- [ ] MEI `<syl>` children on notes → MusicXML `<lyric>` elements
- [ ] MEI `@wordpos`/`@con` → `<syllabic>`
- [ ] MEI `<verse>` `@n` → lyric `number`
- [ ] MEI extender lines → `<extend>`

### 7.4 Tests

- [ ] Add roundtrip fixture: `lyrics_simple.musicxml` (single verse)
- [ ] Add roundtrip fixture: `lyrics_multiverse.musicxml` (multiple verses)
- [ ] Add roundtrip fixture: `lyrics_elision.musicxml`
- [ ] Verify fragment examples: `lyric_element`, `syllabic_element`, `elision_element`, `extend_element_lyric`, `end_line_element`, `end_paragraph_element`, `humming_element`, `laughing_element`

---

## Phase 8: Harmony & Chord Symbols

### 8.1 Model & Parser

- [ ] Add `Harmony` variant to `MeasureContent` enum
- [ ] Create `model/harmony.rs` with: `Harmony`, `HarmonyChord`, `Root`, `Bass`, `Kind`, `Degree`, `Numeral`, `Frame`, `FrameNote`
- [ ] Parse `<harmony>` in `parse_measure()` (currently falls through to `skip_element`)
- [ ] Serialize all harmony elements

### 8.2 Import & Export

- [ ] `harmony` → MEI `<harm>` control event
- [ ] `root` + `kind` → MEI harm text or structured `<chordDef>` reference
- [ ] `bass` → MEI slash notation in harm
- [ ] `degree` → MEI harm extensions
- [ ] `frame` → MEI `<chordDef>` with `<chordMember>`
- [ ] `function` → MEI `<harm>` with function text
- [ ] Export: reverse all mappings

### 8.3 Tests

- [ ] Add roundtrip fixtures for chord symbols, Roman numerals, Nashville numbers
- [ ] Verify fragment examples: `kind_element`, `root_step_element`, `root_alter_element`, `bass_step_element`, `bass_alter_element`, `bass_separator_element`, `degree_value_element`, `degree_alter_element`, `degree_type_element`, `inversion_element`, `numeral_root_element`, `numeral_alter_element`, `numeral_key_element`

---

## Phase 9: Figured Bass

### 9.1 Model, Parser, Import, Export

- [ ] Add `FiguredBass` variant to `MeasureContent` enum
- [ ] Create `model/figured_bass.rs` with: `FiguredBass`, `Figure` (prefix, figure-number, suffix, extend)
- [ ] Parse `<figured-bass>` in `parse_measure()`
- [ ] Serialize all
- [ ] Import: `figured-bass` → MEI `<fb>` with `<f>` children
- [ ] Export: reverse mapping

### 9.2 Tests

- [ ] Add roundtrip fixture for figured bass
- [ ] Verify fragment examples: `figure_number_element`, `prefix_element`, `suffix_element`, `extend_element_figure`

---

## Phase 10: Header & Metadata Completion

### 10.1 Identification

- [ ] Import `creator type="composer"` → MEI `<titleStmt>/<composer>/<persName>`
- [ ] Import `creator type="lyricist"` → MEI `<titleStmt>/<lyricist>/<persName>`
- [ ] Import `creator type="arranger"` → MEI `<titleStmt>/<arranger>/<persName>`
- [ ] Import `rights` → MEI `<pubStmt>/<availability>/<useRestrict>`
- [ ] Import `source` → MEI `<sourceDesc>/<source>`
- [ ] Import `relation` → MEI `<relationList>/<relation>`
- [ ] Import `encoding` → MEI `<encodingDesc>/<appInfo>/<application>` (software, date, supports, description)
- [ ] Import `miscellaneous` → MEI `<notesStmt>/<annot>`
- [ ] Export: reverse all identification mappings

### 10.2 Work Element

- [ ] Import `work-number` → MEI `<workList>/<work>/<identifier>`
- [ ] Import `opus` → MEI `<workList>/<work>/<identifier type="opus">`
- [ ] Export: reverse work mappings

### 10.3 Tests

- [ ] Add roundtrip fixture with rich metadata (all identification fields)
- [ ] Verify metadata roundtrips correctly

---

## Phase 11: Defaults, Layout & Appearance

### 11.1 Serializer Completion

- [ ] Complete `Defaults` serialization in `serializer/score.rs` (resolve TODO at line 289: "appearance, fonts, etc.")
- [ ] Serialize `appearance` children: `line-width`, `note-size`, `distance`, `glyph`, `other-appearance`
- [ ] Serialize font elements: `music-font`, `word-font`, `lyric-font`, `lyric-language`
- [ ] Serialize `scaling`: `millimeters`, `tenths`

### 11.2 Import & Export

- [ ] Import `scaling` → MEI `<scoreDef>` page/spacing attributes
- [ ] Import `page-layout` → MEI `@page.height`, `@page.width`, `@page.*mar`
- [ ] Import `system-layout` → MEI `@system.leftmar`, `@system.rightmar`, `@spacing.system`
- [ ] Import `staff-layout` → MEI `@spacing.staff`
- [ ] Import font info → MEI `@fontfam`, `@fontsize`, `@fontstyle`, `@fontweight`
- [ ] Export: reverse layout mappings (lossy — many MEI visual attributes have no MusicXML equivalent)

### 11.3 Tests

- [ ] Add roundtrip fixture with layout information
- [ ] Verify fragment examples: `measure_distance_element`, `staff_distance_element`, `system_distance_element`, `staff_size_element`, `line_detail_element`, `line_element`, `measure_numbering_element`, `system_dividers_element`, `glyph_element`

---

## Phase 12: Credits

### 12.1 Import & Export

- [ ] Import `credit` → MEI `<pgHead>` / `<pgFoot>` with `<rend>` elements
- [ ] Import `credit-words` positioning (justify, valign, default-x/y) → appropriate `<rend>` placement
- [ ] Import `credit-image` → MEI `<graphic>`
- [ ] Export: reverse credit mappings

### 12.2 Tests

- [ ] Add roundtrip fixture with title page credits
- [ ] Verify fragment example: `image_element`

---

## Phase 13: Print Element

### 13.1 Model & Parser

- [ ] Add `Print` variant to `MeasureContent` enum
- [ ] Create `model/print.rs` with `Print` struct (staff-spacing, new-system, new-page, blank-page, page-number, layout children)
- [ ] Parse `<print>` in `parse_measure()` (currently falls through to `skip_element`)
- [ ] Serialize all

### 13.2 Import & Export

- [ ] Import `new-system="yes"` → MEI `<sb>` (system break)
- [ ] Import `new-page="yes"` → MEI `<pb>` (page break)
- [ ] Import `staff-spacing` and inline layouts → MEI `<scoreDef>` overrides
- [ ] Import `measure-numbering` → MEI `@mnum.visible`
- [ ] Export: MEI `<sb>` → `<print new-system="yes">`, MEI `<pb>` → `<print new-page="yes">`

### 13.3 Tests

- [ ] Add roundtrip fixture with system/page breaks
- [ ] Verify fragment examples: `system_attribute_only_top`, `system_attribute_also_top`, `staff_lines_element`, `staff_type_element`, `staves_element`

---

## Phase 14: Standalone Sound Element

### 14.1 Model & Parser

- [ ] Add `Sound` variant to `MeasureContent` enum
- [ ] Expand `Sound` struct for all attributes: tempo, dynamics, dacapo, segno, dalsegno, coda, tocoda, divisions, forward-repeat, fine, time-only, pizzicato, pan, elevation, damper-pedal, soft-pedal, sostenuto-pedal
- [ ] Parse children: `instrument-change`, `midi-device`, `midi-instrument`, `play`, `swing`, `offset`
- [ ] Parse standalone `<sound>` in `parse_measure()` (currently falls through to `skip_element`)
- [ ] Serialize standalone sound elements

### 14.2 Import & Export

- [ ] Playback sound → MEI `<tempo>` or annotation
- [ ] Repeat-related sound (dacapo, segno, coda, fine, etc.) → MEI repeat structure or labels
- [ ] MIDI attributes → MEI `<midi>` elements or annotation
- [ ] Export: reverse where MEI equivalents exist

### 14.3 Tests

- [ ] Add roundtrip fixture for standalone sound
- [ ] Verify fragment examples: `swing_element`, `pan_and_elevation_elements`

---

## Phase 15: Advanced Attributes

### 15.1 Staff Details

- [ ] Integrate `staff-details` conversion: `staff-type` → MEI notation type, `staff-lines` → MEI `@lines`, `staff-tuning` → MEI `<tuning>`, `capo` → MEI capo attr, `staff-size` → MEI `@scale`
- [ ] Export: reverse mappings
- [ ] Verify fragment examples: `staff_tuning_element`, `capo_element`, `staff_lines_element`, `staff_size_element`

### 15.2 Part Symbol

- [ ] `part-symbol` → MEI `<staffGrp>` `@symbol`; export reverse
- [ ] Verify fragment example: `part_symbol_element`

### 15.3 Measure Style

- [ ] `multiple-rest` → MEI `<multiRest>`; `measure-repeat` → MEI `<mRpt>`/`<mRpt2>`; `beat-repeat` → MEI `<beatRpt>`; `slash` → MEI slash notation
- [ ] Export: reverse mappings
- [ ] Verify fragment examples: `multiple_rest_element`, `measure_repeat_element`, `beat_repeat_element`, `slash_element`, `slash_type_and_slash_dot_elements`

### 15.4 Non-Traditional Keys & Interchangeable Time

- [ ] Non-traditional key → MEI `@keysig` with `<keyAccid>` children; export reverse
- [ ] Interchangeable time → MEI `<meterSigGrp>` with multiple `<meterSig>`; export reverse
- [ ] Verify fragment examples: `key_element_non_traditional`, `key_octave_element`, `interchangeable_element`

---

## Phase 16: Barline Completion

### 16.1 Barline Children

- [ ] Parse and convert barline `fermata` (up to 2) → MEI `<fermata>` control event
- [ ] Parse and convert barline `segno`/`coda` → MEI repeat marks
- [ ] Parse and convert barline `wavy-line` → MEI trill continuation
- [ ] Serialize these barline children
- [ ] Export: reverse mappings

### 16.2 Tests

- [ ] Add roundtrip fixture for decorated barlines
- [ ] Verify fragment examples: `barline_element`, `repeat_element`, `ending_element`

---

## Phase 17: Score Instruments & MIDI

### 17.1 Import & Export

- [ ] `score-instrument` → MEI `<instrDef>` with `@midi.instrname`
- [ ] `midi-instrument` → MEI `<instrDef>` `@midi.channel`, `@midi.instrnum`, `@midi.volume`, `@midi.pan`
- [ ] `instrument-sound` → MEI `<instrDef>` label or sound reference
- [ ] `virtual-instrument` → MEI annotation
- [ ] Note-level `<instrument>` → MEI note-level instrument reference
- [ ] Export: reverse all mappings

### 17.2 Tests

- [ ] Verify fragment examples: `midi_device_element`, `midi_instrument_element`, `midi_name_and_midi_bank_elements`, `midi_unpitched_element`, `virtual_instrument_element`, `ensemble_element`, `instrument_link_element`, `instrument_change_element`

---

## Phase 18: Part/Score Details

### 18.1 Part Name Display & Group Details

- [ ] Parse and convert `part-name-display`/`part-abbreviation-display` → MEI `<label>` with `<rend>` formatting
- [ ] Parse and convert `group-name-display`/`group-abbreviation-display` → MEI `<staffGrp>` `<label>` formatting
- [ ] Parse and convert `group-time` → MEI time signature propagation
- [ ] Parse `<player>` elements → MEI performer metadata
- [ ] Export: reverse all mappings

### 18.2 Tests

- [ ] Verify fragment examples: `part_name_display_element`, `part_abbreviation_display_element`, `group_name_display_element`, `group_abbreviation_display_element`, `group_time_element`, `part_link_element`

---

## Phase 19: Remaining Measure-Level Elements

### 19.1 Listening, Grouping, Link, Bookmark

- [ ] Add `Listening` variant to `MeasureContent`, parse `<listening>`, import → MEI annotation or ignore
- [ ] Add `Grouping` variant to `MeasureContent`, parse `<grouping>`, import → MEI `<expansion>` or annotation
- [ ] Add `Link`/`Bookmark` variants to `MeasureContent`, parse, import → MEI `<ptr>`/`<ref>` or annotation
- [ ] Export: reverse where possible

### 19.2 Tests

- [ ] Verify fragment examples: `sync_element`, `wait_element`, `assess_and_player_elements`, `grouping_element`, `link_element`, `bookmark_element`

---

## Phase 20: Note-Level Completion

### 20.1 Notehead, Play, Listen, Editorial

- [ ] Convert `notehead` value → MEI `@head.shape`, `@head.fill`, parentheses; export reverse
- [ ] Parse and convert `<play>` on notes (IPA, mute, semi-pitched); export reverse
- [ ] Parse and convert `<listen>` on notes; export reverse
- [ ] Parse `<footnote>` and `<level>` on notes → MEI `<annot>` / editorial attrs; export reverse

### 20.2 Tests

- [ ] Verify fragment examples: `notehead_text_element`, `ipa_element`, `level_element`

---

## Phase 21: Direction Serialization Completion

### 21.1 Structured Direction Serialization

Resolve TODO at `serializer/score.rs:1833`: "implement other direction types".

- [ ] Serialize `Rehearsal` → `<rehearsal>` with enclosure
- [ ] Serialize `Segno` → `<segno>`, `Coda` → `<coda>`, `Symbol` → `<symbol>`
- [ ] Serialize `Bracket` → `<bracket>`, `Dashes` → `<dashes>`, `Pedal` → `<pedal>`
- [ ] Serialize `OctaveShift` → `<octave-shift>`, `HarpPedals` → `<harp-pedals>`
- [ ] Serialize `Damp`, `DampAll`, `Eyeglasses`, `StringMute`, `Scordatura`
- [ ] Serialize `PrincipalVoice`, `Percussion`, `AccordionRegistration`, `StaffDivide`, `Image`, `OtherDirection`

### 21.2 Tests

- [ ] Verify all direction type fragment examples serialize correctly without fallback to `<words>`

---

## Phase 22: Compressed MusicXML (.mxl)

### 22.1 Implementation

- [ ] Add `zip` crate dependency
- [ ] Read `.mxl` archive → locate `META-INF/container.xml` → find rootfile → extract and parse MusicXML
- [ ] Write `.mxl` archive → create `META-INF/container.xml` → compress MusicXML
- [ ] Handle multiple rootfiles and accompanying files

### 22.2 Tests

- [ ] Add .mxl roundtrip tests
- [ ] Test with real-world .mxl files

---

## Phase 23: Visual & Position Attributes

### 23.1 Position, Font, Color, Print Attributes

- [ ] Import position attributes (default-x/y, relative-x/y) → MEI `@ho`, `@vo`
- [ ] Import font attributes → MEI `@fontfam`, `@fontsize`, `@fontstyle`, `@fontweight`
- [ ] Import `color` → MEI `@color`; `enclosure` → MEI `@enclose`
- [ ] Import `print-object="no"` → MEI `@visible="false"`; `print-leger`, `print-spacing`
- [ ] Export: reverse where MEI carries these attributes

### 23.2 Tests

- [ ] Add roundtrip fixture testing visual attribute preservation

---

## Phase 24: Remaining Edge Cases & Polish

### 24.1 Note Attributes, Stem Extensions, Clef, Transposition

- [ ] Import `end-dynamics`, `attack`/`release`, `pizzicato` → appropriate MEI attributes
- [ ] Import `StemValue::Double` → MEI dual stem; `StemValue::None` → MEI `@stem.visible="false"`
- [ ] Import `ClefSign::Jianpu` → proper MEI numbered notation clef (currently mapped to G)
- [ ] Import `concert-score` and `for-part` with `part-clef`/`part-transpose` → MEI per-part transposition
- [ ] Export: reverse all mappings

### 24.2 Tests

- [ ] Verify fragment example: `concert_score_and_for_part_elements`
- [ ] Verify edge case handling produces correct output

---

## Phase 25: Version Compatibility

### 25.1 MusicXML Version Detection & Upgrade

- [ ] Detect version from DOCTYPE or `version` attribute
- [ ] Implement MusicXML 2.0 → 4.1, 3.0 → 4.1, 3.1 → 4.1, 4.0 → 4.1 migration
- [ ] Add version-specific export option (e.g. export as MusicXML 3.1)

### 25.2 Tests

- [ ] Cross-version roundtrip tests
- [ ] Test with real-world files from different MusicXML versions

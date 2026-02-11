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
- MEI model (generated): `crates/core/model/src/generated/` — **DO NOT EDIT**, overwritten by codegen
- MEI model extensions: `crates/core/model/src/extended/` — hand-written enrichments, safe from codegen

**Extending the internal model**:

The internal model is generated from the MEI v6.0-dev RNG schema. While MEI is broadly
a superset of common music notation formats, some concepts lack structured MEI equivalents
(e.g. MEI `<harm>` only has text children — no structured root/bass/kind/degree). When
the generated MEI model is insufficient for lossless roundtrip, follow this approach:

1. **Create hand-written extension types** in `crates/core/model/src/extended/`.
   This module is NEVER touched by codegen (codegen only writes to `generated/`).
   Example: `extended/harmony.rs` could define `HarmonyData { root, bass, kind, degrees }`.

2. **Store structured data alongside the MEI element**. Two options:
   - **Wrapper approach**: Create a wrapper struct that holds the generated MEI element
     plus the extension data. The import code populates both; the export code reads both.
   - **Label + sidecar approach**: Store the structured data in the conversion context or
     as a sidecar map keyed by `xml:id`, and encode a summary in the MEI element's text
     or `@label` for human readability.

3. **Wire into `lib.rs`**: Add `pub mod extended;` to `crates/core/model/src/lib.rs`
   and re-export as needed.

4. **Codegen safety**: The codegen (`build.rs` / `mei-codegen`) only writes to
   `src/generated/`. It uses `create_dir_all` + individual file writes — it does NOT
   delete or clear directories. Files outside `generated/` (like `extensions.rs`,
   `extended/`, `lib.rs`) are never touched.

5. **EXTRA_CHILDREN in codegen**: When a new child variant is needed in a generated
   `*Child` enum (so serializer/deserializer match arms are generated), add an entry
   to `EXTRA_CHILDREN` in `crates/formats/mei/codegen/src/generator.rs`. This was done
   for `("measure", "harm")` — see commit `7153703d`.

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

- [x] Create `Ornaments` struct in `model/notations.rs` with all ornament types: `trill_mark`, `turn`, `delayed_turn`, `inverted_turn`, `delayed_inverted_turn`, `vertical_turn`, `inverted_vertical_turn`, `shake`, `mordent`, `inverted_mordent`, `wavy_line`, `schleifer`, `tremolo`, `haydn`, `other_ornament`, `accidental_marks`
  - Also created TrillSound, EmptyTrillSound, HorizontalTurn, Mordent structs and TremomoType/StartNote/TrillStep/TwoNoteTurn enums
- [x] Create supporting structs: `TrillMark` (placement, start-note, trill-step, two-note-turn, accelerate, beats, second-beat, last-beat), `Tremolo` (type single/start/stop, value 1-8), `WavyLine` (type, number, placement), `Mordent` (placement, long, approach, departure)
  - EmptyTrillSound covers trill-mark/vertical-turn/shake/haydn; HorizontalTurn covers turn variants; Mordent adds long/approach/departure
- [x] Add `ornaments: Option<Ornaments>` field to `Notations` struct
- [x] Parse `<ornaments>` inside `parse_notations()` → implement `parse_ornaments()` function
  - Handles all 15+ ornament element types in Start+Empty events, with helpers for trill-sound, tremolo text content, etc.
- [x] Serialize all ornament types in the serializer
  - Extracted notations+ornaments serialization to serializer/notations.rs; also split Note/Attributes/Direction to serializer/elements.rs to keep score.rs under 1500 lines

### 3.2 Import: MusicXML Ornaments → MEI

- [x] `trill-mark` → MEI `<trill>` control event with `@place`, `@startid`
- [x] `mordent`/`inverted-mordent` → MEI `<mordent>` with `@form` (lower/upper), `@long`
- [x] `turn`/`delayed-turn`/`inverted-turn`/`delayed-inverted-turn` → MEI `<turn>` with `@form` (upper/lower), `@delayed`
- [x] `vertical-turn`/`inverted-vertical-turn` → MEI `<ornam>` with musicxml: label for roundtrip
- [x] `shake` → MEI `<ornam>` with label
- [x] `schleifer` → MEI `<ornam>` with `@label="musicxml:schleifer"`
- [x] `tremolo` → MEI `<ornam>` with label encoding type/value for roundtrip
  - Uses `musicxml:tremolo,type=<type>,value=<value>` label (bTrem/fTrem deferred to future)
- [x] `haydn` → MEI `<ornam>` with label
- [x] `wavy-line` → MEI `<ornam>` with label encoding type/number for roundtrip
- [x] `other-ornament` → MEI `<ornam>` with label and text content
- [x] `accidental-mark` within ornaments → MEI `<ornam>` with `musicxml:ornament-accidental-mark` label for roundtrip
  - Label encodes value and placement; distinct from standalone accidental-mark label
  - Export parses label and reconstructs AccidentalMark in ornaments (not notations)
  - `accidental_mark_element_ornament` fragment test passes

### 3.3 Export: MEI Ornaments → MusicXML

- [x] MEI `<trill>` → `trill-mark` with placement
- [x] MEI `<mordent>` → `mordent` / `inverted-mordent` based on `@form`
- [x] MEI `<turn>` → `turn` / `inverted-turn` / `delayed-turn` / `delayed-inverted-turn` based on `@form` and `@delayed`
- [x] MEI `<ornam>` labeled ornaments → roundtrip back to correct MusicXML type
  - vertical-turn, inverted-vertical-turn, shake, schleifer, haydn, tremolo, wavy-line, other-ornament
- [x] MEI `<bTrem>` → `tremolo type="single"` on contained note
  - Added BTrem/FTrem to LayerChild and BeamChild via codegen EXTRA_CHILDREN
  - `convert_btrem_content()` extracts note/chord, adds tremolo type="single" notation
  - `unitdur_to_tremolo_marks()` maps MEI @unitdur → MusicXML tremolo value (8→1, 16→2, 32→3)
  - Handles both note and chord children
- [x] MEI `<fTrem>` → `tremolo type="start/stop"` on contained notes
  - `convert_ftrem_content()` extracts two notes/chords from fTrem children
  - First note gets tremolo type="start", second gets type="stop"
  - Clef children in fTrem are skipped (only note/chord produce MusicXML output)
  - Updated all export match statements: collect_note_ids, find_smallest_duration, collect_beam_events

### 3.4 Tests

- [x] Roundtrip fixtures verified via existing fragment examples
- [x] Verify fragment examples: `trill_mark_element`, `mordent_element`, `inverted_mordent_element`, `turn_element`, `delayed_turn_element`, `inverted_turn_element`, `delayed_inverted_turn_element`, `vertical_turn_element`, `inverted_vertical_turn_element`, `shake_element`, `schleifer_element`, `tremolo_element_single`, `tremolo_element_double`, `haydn_element`, `wavy_line_element`
  - All 15 fragment tests pass in MusicXML triangle roundtrip (313/313 total)
- [x] Integration tests for bTrem/fTrem export (test_btrem_export_produces_tremolo_single, test_ftrem_export_produces_tremolo_start_stop)
  - 486 unit tests, 31 integration tests, 314 roundtrip tests — all pass

---

## Phase 4: Notations — Fermata, Arpeggiate, Glissando, Slide

### 4.1 Model & Parser

- [x] Add to `Notations` struct: `fermatas: Vec<Fermata>`, `arpeggiate: Option<Arpeggiate>`, `non_arpeggiate: Option<NonArpeggiate>`, `glissandos: Vec<Glissando>`, `slides: Vec<Slide>`, `accidental_marks: Vec<AccidentalMark>`, `other_notations: Vec<OtherNotation>`
- [x] Create structs: `Fermata` (shape via FermataShape enum, type upright/inverted via UprightInverted, default-x/y, relative-x/y, color), `Arpeggiate` (number, direction up/down, unbroken, default-x/y, placement, color), `NonArpeggiate` (type top/bottom, number, default-x/y, placement, color), `Glissando` (type start/stop, number, line-type, default-x/y, color, text), `Slide` (type start/stop, number, line-type, default-x/y, color, text), reused existing `AccidentalMark` (value, placement), `OtherNotation` (type start/stop/single, number, placement, smufl, text)
  - All structs in `model/notations.rs` with proper serde annotations
  - FermataShape enum: Normal, Angled, Square, DoubleAngled, DoubleSquare, DoubleDot, HalfCurve, Curlew, Empty
- [x] Parse all in `parse_notations()` — handles both Start and Empty XML events for each type
  - New `parser/parse_notations.rs` module with all parsing functions
  - Handles fermata text content (shape), glissando/slide text content, other-notation text
  - Shared `parse_start_stop_line_attrs()` helper for glissando/slide
- [x] Serialize all in serializer
  - Added `serialize_fermata`, `serialize_arpeggiate`, `serialize_non_arpeggiate`, `serialize_glissando`, `serialize_slide`, `serialize_other_notation` in `serializer/notations.rs`
  - Added `fermata_shape_str`, `top_bottom_str`, `upright_inverted_str`, `line_type_str`, `start_stop_single_str` helpers in `serializer/score.rs`
  - Fermata serializes shape as text content or empty element; glissando/slide serialize text or empty

### 4.2 Import & Export

- [x] `fermata` → MEI `<fermata>` control event with `@shape`, `@form`, `@place`, `@startid`
  - Added `process_fermatas()` in import/note.rs; maps shape/type → MEI @shape/@form/@place
  - Added `convert_fermata_events()` in export/content.rs; reverse mapping
- [x] `arpeggiate` → MEI `<arpeg>` control event with `@order` (up/down)
  - Added `process_arpeggiate()` in import/note.rs
  - Added `convert_arpeg_events()` in export/content.rs
- [x] `non-arpeggiate` → MEI `<arpeg>` with `@order="nonarp"` and label for roundtrip
- [x] `glissando` → MEI `<gliss>` control event with `@startid`/`@endid`, `@lform`
  - Added `process_glissandos()` in import/note.rs with pending/completed pattern
  - Added `emit_gliss_events()` in import/structure.rs
  - Added `convert_gliss_events()` in export/content.rs with cross-measure support
  - Added context/glissandos.rs for PendingGliss/CompletedGliss tracking
- [x] `slide` → MEI `<gliss>` with `musicxml:slide` label for roundtrip
- [x] `accidental-mark` (standalone) → MEI `<ornam>` with label for roundtrip
  - Added `process_accidental_marks()` in import/note.rs
  - Export via label parsing in `convert_ornament_events()`
- [x] Export: reverse all mappings
- [x] Added Fermata/Arpeg/Gliss to MeasureChild via EXTRA_CHILDREN codegen

### 4.3 Tests

- [x] Add roundtrip fixtures for fermata, arpeggiate, glissando, slide
  - Fixtures already existed in tests/fixtures/musicxml/fragment_examples/
- [x] Verify fragment examples: `fermata_element`, `arpeggiate_element`, `non_arpeggiate_element`, `glissando_element_single`, `glissando_element_multiple`, `slide_element`, `accidental_mark_element_notation`
  - All 7 pass roundtrip (313 total pass, 0 fail)

---

## Phase 5: Notations — Technical

### 5.1 Model & Parser

- [x] Add `technical: Option<Technical>` field to `Notations` struct
- [x] Create `Technical` struct with all 31 types as `Vec<_>` in `model/technical.rs`: up_bow, down_bow, harmonic, open_string, thumb_position, fingering, pluck, snap_pizzicato, stopped, fret, string, hammer_on, pull_off, bend, tap, heel, toe, double_tongue, triple_tongue, fingernails, hole, arrow, brass_bend, flip, smear, open, half_muted, harmon_mute, golpe, handbell, other_technical
- [x] Create supporting structs: `EmptyPlacementSmufl`, `PlacementText`, `Fingering`, `Fret`, `TechString`, `HammerOnPullOff`, `Bend`, `BendRelease`, `BendShape`, `Tap`, `TapHand`, `HeelToe`, `Hole`, `HoleClosed`, `HoleClosedValue`, `HoleClosedLocation`, `Arrow`, `ArrowContent`, `Handbell`, `HarmonMute`, `HarmonClosed`, `HarmonClosedValue`, `HarmonClosedLocation`, `Harmonic`, `OtherTechnical`
- [x] Parse all in `parse_notations()` → `parse_technical()` in `parser/parse_technical.rs`
- [x] Serialize all in `serializer/technical.rs`

### 5.2 Import & Export

- [x] All 31 technical types → MEI `<ornam>` with `musicxml:` label for lossless roundtrip
  - Simple types (up-bow, down-bow, etc.): `musicxml:<element-name>` label + placement
  - Text types (fingering, pluck, fret, string, handbell, tap): text in OrnamChild::Text
  - Complex types (bend, hole, arrow, harmon-mute, harmonic): key params encoded in label
- [x] Export: reverse all label-based mappings in `convert_technical_events()`

### 5.3 Tests

- [x] Fixed invalid fixtures (bend_element, hole_element, hole_type_element) — `<technical>` was outside `<notations>`
- [x] All 32 fragment examples pass roundtrip: `up_bow_element`, `down_bow_element`, `open_string_element`, `thumb_position_element`, `snap_pizzicato_element`, `stopped_element`, `double_tongue_element`, `triple_tongue_element`, `fingernails_element`, `pluck_element`, `tap_element`, `heel_element`, `toe_element`, `heel_toe_substitution`, `fingering_element_notation`, `bend_element`, `brass_bend_element`, `flip_element`, `smear_element`, `open_element`, `half_muted_element`, `harmon_mute_element`, `golpe_element`, `handbell_element`, `hole_element`, `hole_type_element`, `arrow_element`, `arrowhead_element`, `circular_arrow_element`, `pre_bend_element`, `with_bar_element`, `technical_element_tablature`
  - 481 unit tests, 31 integration tests, 313 roundtrip tests — all pass

---

## Phase 6: Notations — Dynamics within Notations

### 6.1 Model, Parser, Import, Export

- [x] Add `dynamics: Vec<Dynamics>` field to `Notations` struct (reuse existing `Dynamics`/`DynamicsValue` from `model/direction/dynamics.rs`)
  - Added `placement: Option<AboveBelow>` to `Dynamics` struct (per XSD: placement used when dynamics associated with note)
  - Added `dynamics: Vec<super::direction::Dynamics>` to `Notations` struct in `model/notations.rs`
- [x] Parse `<dynamics>` within `parse_notations()` (currently only parsed within `<direction-type>`)
  - Made `parse_dynamics()` in `parser/parse_direction.rs` pub(crate) for reuse
  - Added `b"dynamics"` cases in both `Event::Start` and `Event::Empty` branches of `parse_notations()`
  - Extracts placement attribute from `<dynamics>` element for notation-level usage
- [x] Serialize dynamics within notations
  - Added `serialize_dynamics_notation()` in `serializer/notations.rs`
  - Made `serialize_dynamics_value()` in `serializer/elements.rs` pub(crate) for reuse
  - Serializes dynamics with placement attr + value children, per XSD schema order
- [x] Import: notation-level dynamics → MEI `<dynam>` with `@startid` referencing the note
  - Added `process_notation_dynamics()` in `import/note.rs`
  - Creates MEI `<dynam>` with `@startid`, `@staff`, `@place`; text = dynamics value
  - Uses `musicxml:notation-dynamics` label to distinguish from direction-level dynamics
- [x] Export: MEI `<dynam>` attached to specific note → notation-level dynamics
  - Added `convert_notation_dynamics()` in `export/content.rs`
  - Skips notation-level dynams in `convert_direction_events()` (by label check)
  - Finds note by startid, adds `Dynamics` to note's `Notations`
  - Made `parse_dynamics_text()` in `export/direction.rs` pub(crate) for reuse

### 6.2 Tests

- [x] Add roundtrip fixture with notation-level dynamics
  - `dynamics_element_notation.musicxml`: 3 notes with ff, p, sfz dynamics in `<notations>`
- [x] Verify dynamics in notations context produce correct output
  - 314/314 roundtrip tests pass (1 ignored debug helper), 481 unit tests pass, 31 integration tests pass

---

## Phase 7: Lyrics

### 7.1 Model & Parser

- [x] Create `model/lyric.rs` with: `Lyric` (number, name, placement, justify, default-x/y, content, end-line, end-paragraph), `Syllabic` enum (single/begin/middle/end), `LyricText` (text, font/color), `Elision` (text, font), `Extend` (type start/stop/continue), `LyricContent` enum (Text/ExtendOnly/Laughing/Humming), `SyllableGroup` (elision, syllabic, text)
  - Four XSD choice branches: text-with-syllables, extend-only, laughing, humming
  - SyllableGroup models elision+syllabic+text triples for multi-syllable-per-note
- [x] Add `lyrics: Vec<Lyric>` field to `Note` struct
  - Added to all 5 Note constructors (pitched, rest, grace_note, unpitched, unpitched_grace)
- [x] Parse `<lyric>` within `parse_note()` (was falling through to `skip_element`)
  - Added `parse_lyric()` and `parse_lyric_attrs()` in parser/parse_note.rs
  - Handles all 4 content branches + end-line/end-paragraph karaoke markers
- [x] Serialize all lyric elements
  - MusicXmlSerialize impl for Lyric, helpers for syllabic/text/elision/extend

### 7.2 Import: MusicXML Lyrics → MEI

- [x] `<lyric>` on notes → MEI `<verse>` children on `<note>` with `<syl>` children
  - Added verse/syl to MEI codegen EXTRA_CHILDREN, regenerated model
- [x] `<syllabic>` (single/begin/middle/end) → MEI `@wordpos` (i/m/t) and `@con` (d for dash) attributes
- [x] `<text>` content → MEI `<syl>` text child
- [x] `<elision>` → MEI `@con="b"` on previous syl + elision value in verse label
- [x] Lyric `number` → MEI `<verse>` `@n` for multi-verse support
- [x] `<extend>` → encoded in verse `@label` for roundtrip (extend=start/stop/continue)
- [x] Handle `<humming>` and `<laughing>` → encoded in verse `@label`
- [x] MusicXML-only attrs (default-y, name, justify, placement, etc.) → verse `@label` for roundtrip

### 7.3 Export: MEI Lyrics → MusicXML

- [x] MEI `<verse>` with `<syl>` children on notes → MusicXML `<lyric>` elements
- [x] MEI `@wordpos`/`@con` → `<syllabic>` (i→begin, m→middle, t→end)
- [x] MEI `<verse>` `@n` → lyric `number`
- [x] MEI verse label → `<extend>`, `<humming>`, `<laughing>`, `<elision>`, and all MusicXML attrs
- [x] Chord notes: lyrics export added to convert_mei_chord() path

### 7.4 Tests

- [x] Verify fragment examples: `lyric_element`, `syllabic_element`, `elision_element`, `extend_element_lyric`, `end_line_element`, `end_paragraph_element`, `humming_element`, `laughing_element`
  - All 8 lyric fragment tests pass MusicXML triangle roundtrip
- [x] 314/314 roundtrip tests pass (including assess_and_player_elements with chord lyrics), 481 unit tests pass, 31 integration tests pass

---

## Phase 8: Harmony & Chord Symbols

### 8.1 Model & Parser

- [x] Add `Harmony` variant to `MeasureContent` enum
- [x] Create `model/harmony.rs` with: `Harmony`, `HarmonyChord`, `Root`, `Bass`, `Kind`, `Degree`, `Numeral`, `Frame`, `FrameNote`
- [x] Parse `<harmony>` in `parse_measure()` (currently falls through to `skip_element`)
- [x] Serialize all harmony elements

### 8.2 Import & Export

**Strategy**: Instead of the `extended/` sidecar approach originally proposed, uses
JSON-in-label roundtrip. The full MusicXML `Harmony` struct (which already derives
`Serialize`/`Deserialize`) is serialized to JSON and stored in the MEI `<harm>` `@label`
attribute with a `musicxml:harmony,` prefix. A human-readable chord symbol text is
stored as the `<harm>` text child. On export, the JSON is deserialized back to the
original `Harmony` struct. This approach is simpler, avoids duplicating type definitions
in an `extended/` module, and achieves lossless roundtrip for all harmony data (root,
kind, bass, degrees, frame, numeral, function, and all styling attributes).

- [x] Import: `harmony` → MEI `<harm>` control event with `@tstamp`, `@staff`, `@place`
  - Added `import/harmony.rs` with `convert_harmony()` function
  - Full MusicXML `Harmony` serialized as JSON in `@label` for lossless roundtrip
  - Beat position canonicalized into `offset` field so tstamp is correct after re-export
  - Staff cleared from JSON (handled via MEI `@staff`), restored on export
  - Human-readable chord text from `harmony_to_text()` as `HarmChild::Text`
- [x] Import: `root` + `kind` → chord label text + JSON-encoded root/kind in label
- [x] Import: `bass` → slash notation text + JSON-encoded bass in label
- [x] Import: `degree` → JSON-encoded degree list in label
- [x] Import: `frame` → JSON-encoded frame in label
- [x] Import: `function` → function text in `HarmChild::Text` + JSON in label
- [x] Export: parse `musicxml:harmony,` label JSON to reconstruct full `Harmony`
  - Added `export/harmony.rs` with `convert_mei_harm()` function
  - Handles roundtrip (JSON label) and fallback (text-only, function-based) paths
  - Staff number set from `local_staff_n` parameter
  - Wired into `convert_direction_events()` via `MeasureChild::Harm` match arm
- [x] Added `serde_json` dependency to `tusk-musicxml` for JSON serialization
- [x] Wired `import/harmony` as `pub(crate)` module for cross-module access

### 8.3 Tests

- [x] Existing roundtrip fixtures for chord symbols pass: `tutorial_chord_symbols.musicxml` (13 chord types), `BrookeWestSample.musicxml` (multi-staff with harmonies)
- [x] Verify fragment examples: `kind_element`, `root_step_element`, `root_alter_element`, `bass_step_element`, `bass_alter_element`, `bass_separator_element`, `degree_value_element`, `degree_alter_element`, `degree_type_element`, `inversion_element`, `numeral_root_element`, `numeral_alter_element`, `numeral_key_element`
  - All 13 fragment tests pass MusicXML triangle roundtrip
- [x] All 314 roundtrip tests pass (0 regressions), 490 unit tests pass, 31 integration tests pass

---

## Phase 9: Figured Bass

### 9.1 Model, Parser, Import, Export

- [x] Add `FiguredBass` variant to `MeasureContent` enum
- [x] Create `model/figured_bass.rs` with: `FiguredBass`, `Figure` (prefix, figure-number, suffix, extend)
- [x] Parse `<figured-bass>` in `parse_measure()`
- [x] Serialize all
- [x] Import: `figured-bass` → MEI `<fb>` with `<f>` children (JSON-in-label roundtrip pattern)
- [x] Export: reverse mapping (label-based + fallback from fb children)

### 9.2 Tests

- [x] Add roundtrip fixture for figured bass
  - `tests/fixtures/musicxml/figured_bass.musicxml`: 2 measures, bass clef, multiple figured-bass elements with single/stacked figures, prefixes (flat, natural, double-sharp), suffixes (sharp, natural, flat), parentheses, and extend start/stop
  - Roundtrip test added in `roundtrip.rs` — all 4 levels pass (315/315 total)
- [x] Verify fragment examples: `figure_number_element`, `prefix_element`, `suffix_element`, `extend_element_figure`
  - All 4 fragment tests pass MusicXML triangle roundtrip
  - 492 unit tests, 31 integration tests, 315 roundtrip tests — all pass

---

## Phase 10: Header & Metadata Completion

### 10.1 Identification

**Strategy**: MEI generated model header types are too limited for structured mapping
(TitleStmt only has Title, PubStmt only has Unpub, etc.). Uses JSON-in-`<extMeta>` pattern:
full MusicXML Identification serialized as JSON in `@analog` attribute with `musicxml:identification,`
prefix. Human-readable text summary stored as ExtMeta text child. Same pattern as harmony/figured-bass.

- [x] Import `creator type="composer"` → MEI `<extMeta>` with JSON-encoded Identification
  - All creators (composer, lyricist, arranger, etc.) stored in single Identification JSON
- [x] Import `creator type="lyricist"` → (included in Identification JSON)
- [x] Import `creator type="arranger"` → (included in Identification JSON)
- [x] Import `rights` → (included in Identification JSON)
- [x] Import `source` → (included in Identification JSON)
- [x] Import `relation` → (included in Identification JSON)
- [x] Import `encoding` → (included in Identification JSON)
  - `has_meaningful_identification()` guard prevents storing extMeta for default-only Tusk encoding
- [x] Import `miscellaneous` → (included in Identification JSON)
- [x] Export: reverse all identification mappings
  - Scans `<meiHead>` extMeta children for `musicxml:identification,` prefix
  - Deserializes JSON to full Identification; merges work-title from titleStmt
  - Falls back to minimal Tusk encoding when no JSON available
- [x] Added `musicxml:work,` extMeta for work-number/opus roundtrip
- [x] Added `musicxml:movement-number,` and `musicxml:movement-title,` extMeta
- [x] xml_compare: extMeta keyed by `@analog` prefix for unordered comparison
- [x] Roundtrip fixture: `identification_metadata.musicxml` (3 creators, rights, encoding with supports, source, relation, miscellaneous, work-number, movement-number/title)

### 10.2 Work Element

**Note**: Original plan proposed `<workList>/<work>/<identifier>` mapping, but Phase 10.1
already implemented work-number and opus via the extMeta JSON-in-label pattern (same as
identification). This is simpler and achieves lossless roundtrip for all Work fields.

- [x] Import `work-number` → MEI extMeta JSON (implemented in 10.1 via `musicxml:work,` label)
  - Full `Work` struct (work_number, work_title, opus) serialized as JSON in extMeta `@analog`
  - Human-readable summary text in extMeta body
- [x] Import `opus` → MEI extMeta JSON (implemented in 10.1 via `musicxml:work,` label)
  - Opus `href` and `xlink_type` preserved in Work JSON
  - Added `<opus>` element to `identification_metadata.musicxml` fixture for coverage
- [x] Export: reverse work mappings (implemented in 10.1)
  - Scans extMeta for `musicxml:work,` prefix, deserializes JSON to `Work` struct
  - Merges work-title from titleStmt (canonical source)

### 10.3 Tests

- [x] Add roundtrip fixture with rich metadata (all identification fields)
  - `identification_metadata.musicxml` covers: work-number, work-title, opus, movement-number,
    movement-title, 3 creators, rights, encoding with supports, source, relation, miscellaneous
- [x] Verify metadata roundtrips correctly
  - All 4 roundtrip levels pass (conversion, full, triangle MEI, triangle MusicXML)

---

## Phase 11: Defaults, Layout & Appearance

### 11.1 Serializer & Parser Completion

- [x] Complete `Defaults` serialization in `serializer/score.rs` (resolved TODO: "appearance, fonts, etc.")
  - Added `serialize_appearance()` with line-width, note-size, distance, glyph, other-appearance children
  - Added `serialize_empty_font()` for music-font and word-font (empty elements with font attrs)
  - Added `serialize_lyric_font()` and `serialize_lyric_language()` for lyric font/language elements
  - Added `serialize_system_dividers()` for left-divider/right-divider in system-layout
  - Added `note_size_type_str()` helper for cue/grace/grace-cue/large serialization
- [x] Serialize `appearance` children: `line-width`, `note-size`, `distance`, `glyph`, `other-appearance`
- [x] Serialize font elements: `music-font`, `word-font`, `lyric-font`, `lyric-language`
- [x] Serialize `scaling`: `millimeters`, `tenths` (already existed)
- [x] Parse `appearance` and all children in `parse_defaults.rs`
  - New `parse_appearance()` handles line-width, note-size, distance, glyph, other-appearance
  - Each child reads type attribute + text content value
- [x] Parse font elements: `music-font`, `word-font`, `lyric-font`, `lyric-language`
  - `parse_empty_font_attrs()` for music-font/word-font (Empty events with font-family/style/size/weight)
  - `parse_lyric_font_attrs()` adds number/name attrs
  - `parse_lyric_language_attrs()` reads xml:lang
  - `parse_font_size_value()` handles both numeric points and CSS size names
- [x] Parse `system-dividers` with left-divider/right-divider (Empty events with print-object attr)
- [x] Extracted all defaults/layout/appearance/font parsing to `parser/parse_defaults.rs` (parser.rs was over 1500 lines)

### 11.2 Import & Export

**Strategy**: Two-pronged approach for lossless roundtrip + semantic MEI attributes:
1. Full `Defaults` struct serialized as JSON in MEI `<extMeta>` (`@analog="musicxml:defaults,{json}"`)
   for lossless roundtrip of all fields (appearance, line-widths, note-sizes, glyphs, system-dividers, etc.)
2. Key fields also mapped to MEI `<scoreDef>` visual attributes for semantic fidelity in external MEI tools

- [x] Import `scaling` → MEI `@vu.height` on scoreDef (computed as `2 * mm / tenths` formatted as `"Xmm"`)
  - Added `apply_defaults_to_score_def()` in `import/parts.rs`
  - Full Defaults JSON stored in extMeta via `DEFAULTS_LABEL_PREFIX` in `import/mod.rs`
- [x] Import `page-layout` → MEI `@page.height`, `@page.width`, `@page.topmar`, `@page.botmar`, `@page.leftmar`, `@page.rightmar`
  - Values stored as tenths strings; prefers `type="both"` margins, falls back to first entry
- [x] Import `system-layout` → MEI `@system.leftmar`, `@system.rightmar`, `@spacing.system`, `@system.topmar`
  - System margins + system-distance + top-system-distance all mapped
- [x] Import `staff-layout` → MEI `@spacing.staff`
  - Uses first staff-layout entry's staff-distance
- [x] Import font info → MEI font attributes
  - `music-font` → `@music.name`, `@music.size`
  - `word-font` → `@text.fam`, `@text.size`, `@text.style`, `@text.weight`
  - `lyric-font` (first) → `@lyric.fam`, `@lyric.size`, `@lyric.style`, `@lyric.weight`
  - Added `convert_font_size_to_mei()`, `convert_font_style_to_mei()`, `convert_font_weight_to_mei()` helpers
- [x] Export: reverse layout mappings
  - Primary: recover full `Defaults` from extMeta JSON (lossless roundtrip)
  - Fallback: `defaults_from_score_def()` builds Defaults from scoreDef visual attrs (lossy)
  - Added `convert_mei_font_size()`, `convert_mei_font_style()`, `convert_mei_font_weight()` reverse helpers
  - Scaling fallback assumes tenths=40 (common MusicXML default) since vu.height alone can't reconstruct both mm and tenths
  - 316/316 roundtrip tests pass (0 regressions), 492 unit tests, 31 integration tests — all pass

### 11.3 Tests

- [x] Add roundtrip fixture with layout information
  - `defaults_layout.musicxml`: comprehensive fixture with scaling, page-layout (height/width/margins), system-layout (margins/distance/top-distance/dividers), 2 staff-layouts, appearance (8 line-widths, 2 note-sizes, 2 distances, 2 glyphs, 1 other-appearance), music-font, word-font, 2 lyric-fonts, 2 lyric-languages
  - All 4 roundtrip levels pass (conversion, full, triangle MEI, triangle MusicXML)
  - 317/317 roundtrip tests pass (0 regressions), 492 unit tests, 31 integration tests
- [x] Verify fragment examples: `measure_distance_element`, `staff_distance_element`, `system_distance_element`, `staff_size_element`, `line_detail_element`, `line_element`, `measure_numbering_element`, `system_dividers_element`, `glyph_element`
  - All 9 fragment tests pass MusicXML triangle roundtrip

---

## Phase 12: Credits

### 12.1 Import & Export

- [x] Import `credit` → MEI `<pgHead>` / `<pgFoot>` with `<rend>` elements
  - Full `Vec<Credit>` serialized as JSON in extMeta `@analog` with `musicxml:credits,` prefix for lossless roundtrip
  - Human-readable text from credit-words stored as `<anchoredText>` children in `<pgHead>` on scoreDef
  - Added `CREDITS_LABEL_PREFIX`, `credits_summary()`, `convert_credits_to_pg_head()` in import/
- [x] Import `credit-words` positioning (justify, valign, default-x/y) → appropriate `<rend>` placement
  - All credit-words formatting (position, font, alignment) preserved in extMeta JSON
  - Simplified text summary in pgHead/anchoredText for MEI tool compatibility
- [x] Import `credit-image` → MEI `<graphic>`
  - Credit-image data preserved in extMeta JSON for lossless roundtrip
  - pgHead only contains text credits (MEI pgHead has no graphic child type)
- [x] Export: reverse credit mappings
  - Primary: recover full credits from extMeta JSON (`musicxml:credits,` prefix)
  - Fallback: `credits_from_pg_head()` creates basic credits from scoreDef pgHead text (lossy)
  - Added credits field to HeaderData, wired into convert_mei_to_timewise_with_context

### 12.2 Tests

- [x] Add roundtrip fixture with title page credits
  - `credits.musicxml`: 4 credits (title, subtitle, composer, rights) with positioning, fonts, credit-types
  - All 4 roundtrip levels pass (conversion, full, triangle MEI, triangle MusicXML)
  - 318/318 roundtrip tests pass (0 regressions), 492 unit tests, 31 integration tests
- [x] Verify fragment example: `image_element`
  - Existing direction-level `<image>` in `image_element.musicxml` unaffected by credit changes
  - Passes all roundtrip levels

---

## Phase 13: Print Element

### 13.1 Model & Parser

- [x] Add `Print` variant to `MeasureContent` enum
- [x] Create `model/print.rs` with `Print` struct (staff-spacing, new-system, new-page, blank-page, page-number, layout children)
- [x] Parse `<print>` in `parse_measure()` (currently falls through to `skip_element`)
- [x] Serialize all

### 13.2 Import & Export

- [x] Import `new-system="yes"` → MEI `<sb>` (system break)
- [x] Import `new-page="yes"` → MEI `<pb>` (page break)
- [x] Import `staff-spacing` and inline layouts → JSON-in-label roundtrip on sb/pb
- [x] Import `measure-numbering` → JSON-in-label roundtrip on sb/pb
- [x] Export: MEI `<sb>` → `<print new-system="yes">`, MEI `<pb>` → `<print new-page="yes">`
- [x] Add roundtrip fixture with system/page breaks
- [x] Verify fragment examples: `system_attribute_only_top`, `system_attribute_also_top`, `staff_lines_element`, `staff_type_element`, `staves_element`

---

## Phase 14: Standalone Sound Element

### 14.1 Model & Parser

- [x] Add `Sound` variant to `MeasureContent` enum
  - Added `Sound(Box<Sound>)` variant to `MeasureContent` in `model/elements/measure.rs`
  - Updated both partwise and timewise serializer match arms in `serializer/score.rs`
- [x] Expand `Sound` struct for all attributes: tempo, dynamics, dacapo, segno, dalsegno, coda, tocoda, divisions, forward-repeat, fine, time-only, pizzicato, pan, elevation, damper-pedal, soft-pedal, sostenuto-pedal
  - Moved Sound from `model/direction/misc.rs` to new `model/direction/sound.rs` module
  - Added missing attributes: `divisions`, `time_only`, `pan`, `elevation`
  - Added child fields: `midi_instrument_changes: Vec<SoundMidiGroup>`, `swing: Option<Swing>`, `offset: Option<Offset>`
  - Created `SoundMidiGroup` (instrument-change + midi-device + midi-instrument + play group)
  - Created `InstrumentChange` (id, instrument-sound, solo, ensemble, virtual-library, virtual-name)
  - Created `Play` (id, entries), `PlayEntry` enum (Ipa/Mute/SemiPitched/OtherPlay), `OtherPlay` (type, value)
  - Created `Swing` (content, swing-style), `SwingContent` enum (Straight/Ratio), `SwingRatio` (first, second, swing-type)
- [x] Parse children: `instrument-change`, `midi-device`, `midi-instrument`, `play`, `swing`, `offset`
  - `parse_sound_full()` parses Start events with all children; `parse_sound_attrs()` parses Empty events
  - Dedicated parsers: `parse_instrument_change`, `parse_midi_device_child/empty`, `parse_midi_instrument_child`, `parse_play`, `parse_swing`
  - MIDI group elements are flushed into `SoundMidiGroup` entries; swing and offset parsed as trailing children
  - Direction parser now calls `parse_sound_full()` instead of `skip_to_end()`
- [x] Parse standalone `<sound>` in `parse_measure()` (currently falls through to `skip_element`)
  - Added `b"sound"` cases in both `Event::Start` and `Event::Empty` branches of both parse_measure functions (partwise and timewise)
- [x] Serialize standalone sound elements
  - Created `serializer/sound.rs` with `MusicXmlSerialize` impl for `Sound`
  - Serializes all 18 attributes and all child elements (midi groups, swing, offset)
  - Helper functions: `serialize_midi_group`, `serialize_instrument_change`, `serialize_midi_device`, `serialize_midi_instrument`, `serialize_play`, `serialize_other_play`, `serialize_swing`
  - Also fixed Direction serializer TODO — now serializes `sound` child in Direction elements
  - `swing_element` and `pan_and_elevation_elements` fragment roundtrip tests pass (Sound children in Direction context)
  - 494 unit tests, 31 integration tests, 319 roundtrip tests — all pass

### 14.2 Import & Export

- [x] Playback sound → MEI `<dir>` with `musicxml:sound,{json}` label (JSON-in-label roundtrip)
  - Standalone `MeasureContent::Sound` now handled in `import/structure.rs` → `import/sound.rs`
  - Full `Sound` struct serialized as JSON in dir `@label` for lossless roundtrip
  - Human-readable summary (tempo, dynamics, repeat marks, etc.) stored as dir text child
  - Only imported from first staff (measure-level element, same pattern as print)
- [x] Repeat-related sound (dacapo, segno, coda, fine, etc.) → preserved in JSON label
  - All Sound attributes (segno, dalsegno, coda, tocoda, fine, dacapo, forward-repeat, etc.) roundtrip via JSON
- [x] MIDI attributes → preserved in JSON label
  - MIDI instrument changes, swing, offset all serialized in Sound JSON
  - Fixed `InstrumentChange.solo` from `Option<()>` to `Option<bool>` for JSON roundtrip stability
- [x] Export: `<dir>` with `musicxml:sound,` label → `MeasureContent::Sound`
  - Added `export/sound.rs` with `convert_mei_sound_dir()` function
  - Standalone sound dirs intercepted in `export/content.rs` before general dir dispatch
  - First-staff-only emission (consistent with import)
- [x] Add roundtrip fixture for standalone sound
  - `tests/fixtures/musicxml/standalone_sound.musicxml`: 3 measures with tempo/dynamics, segno/forward-repeat, dalsegno/fine
  - All 4 roundtrip levels pass (320/320 total)
- [x] Verify fragment examples: `swing_element`, `pan_and_elevation_elements`
  - Both pass all roundtrip levels (direction-level sound, not standalone)
  - Direction-level `direction.sound` is not preserved on import (lossy) — only standalone sound roundtrips
- [x] Added `@label` prefix to `control_event_type_key` for `dir` elements in xml_compare
  - Disambiguates `musicxml:sound,*` dirs from plain text dirs at same position

---

## Phase 15: Advanced Attributes

### 15.1 Staff Details

- [x] Integrate `staff-details` conversion: `staff-type` → MEI notation type, `staff-lines` → MEI `@lines`, `staff-tuning` → MEI `<tuning>`, `capo` → MEI capo attr, `staff-size` → MEI `@scale`
- [x] Export: reverse mappings
- [x] Verify fragment examples: `staff_tuning_element`, `capo_element`, `staff_lines_element`, `staff_size_element`

### 15.2 Part Symbol

- [x] `part-symbol` → MEI `<staffGrp>` `@symbol`; export reverse
  - Parser: added `parse_part_symbol()` in `parser/parse_attributes.rs` (was falling through to skip_element)
  - Import: already mapped `PartSymbolValue` → MEI `@symbol`; added JSON-in-label roundtrip for extra attrs (top-staff, bottom-staff, default-x, color) via `musicxml:part-symbol,` prefix on staffGrp `@label`
  - Export: `extract_part_symbol_from_staff_grp()` in `export/parts.rs` recovers full PartSymbol from JSON label or builds from `@symbol`; stored in context via `set_part_symbol()`; emitted in `build_first_measure_attributes_multi()`
  - Fixed `is_multi_staff_part()` to use `@bar.thru="true"` instead of `@symbol="brace"` — now detects multi-staff parts regardless of symbol value (bracket, line, square, none)
- [x] Verify fragment example: `part_symbol_element`
  - Passes all 4 roundtrip levels; 320/320 roundtrip tests pass, 494 unit tests, 31 integration tests

### 15.3 Measure Style

- [x] `multiple-rest` → MEI `<multiRest>`; `measure-repeat` → MEI `<mRpt>`/`<mRpt2>`; `beat-repeat` → MEI `<beatRpt>`; `slash` → MEI slash notation
  - JSON-in-label roundtrip on `<dir>` with `musicxml:measure-style,` prefix (same pattern as sound/print)
  - Parser: `parse_measure_style` + helpers in `parse_attributes.rs`; serializer: `serialize_measure_style` in `elements.rs`
  - Import: `import/measure_style.rs` — `convert_measure_styles()` with fixed tstamp=1 for stable roundtrip
- [x] Export: reverse mappings
  - `export/measure_style.rs` — intercepts measure-style dirs, emits `MeasureContent::Attributes` with measure_styles
  - First-staff-only emission in `content.rs` (same pattern as sound)
- [x] Verify fragment examples: `multiple_rest_element`, `measure_repeat_element`, `beat_repeat_element`, `slash_element`, `slash_type_and_slash_dot_elements`
  - All 5 pass all 4 roundtrip levels; 320/320 roundtrip, 494 unit, 31 integration tests pass

### 15.4 Non-Traditional Keys & Interchangeable Time

- [x] Non-traditional key → MEI `@keysig` with `<keyAccid>` children; export reverse
  - Parser: key-step/key-alter/key-accidental + key-octave; Serializer: key-accidental + key-octave output
  - Import: JSON-in-label on staffDef for non-traditional keys; Export: recover from label
- [x] Interchangeable time → MEI `<meterSigGrp>` with multiple `<meterSig>`; export reverse
  - Parser: multiple signatures, separator attr, interchangeable child; Serializer: interchangeable + separator
  - Import: JSON-in-label on staffDef for interchangeable/separator; Export: recover from label
- [x] Verify fragment examples: `key_element_non_traditional`, `key_octave_element`, `interchangeable_element`
  - All 3 pass all 4 roundtrip levels; 320/320 roundtrip, 494 unit, 31 integration tests pass

---

## Phase 16: Barline Completion

### 16.1 Barline Children

- [x] Parse and convert barline `fermata` (up to 2) → MEI `<fermata>` control event
- [x] Parse and convert barline `segno`/`coda` → MEI repeat marks
- [x] Parse and convert barline `wavy-line` → MEI trill continuation
- [x] Serialize these barline children
- [x] Export: reverse mappings
- [x] Add roundtrip fixture for decorated barlines
- [x] Verify fragment examples: `barline_element`, `repeat_element`, `ending_element`

---

## Phase 17: Score Instruments & MIDI

### 17.1 Import & Export

- [x] `score-instrument` → MEI `<instrDef>` with `@midi.instrname`
- [x] `midi-instrument` → MEI `<instrDef>` `@midi.channel`, `@midi.instrnum`, `@midi.volume`, `@midi.pan`
- [x] `instrument-sound` → MEI `<instrDef>` label or sound reference
- [x] `virtual-instrument` → MEI annotation
- [x] Note-level `<instrument>` → MEI note-level instrument reference
- [x] Export: reverse all mappings
- [x] Verify fragment examples: `midi_device_element`, `midi_instrument_element`, `midi_name_and_midi_bank_elements`, `midi_unpitched_element`, `virtual_instrument_element`, `ensemble_element`, `instrument_link_element`, `instrument_change_element`

---

## Phase 18: Part/Score Details

### 18.1 Part Name Display & Group Details

- [x] Parse and convert `part-name-display`/`part-abbreviation-display` → MEI `<label>` with `<rend>` formatting
- [x] Parse and convert `group-name-display`/`group-abbreviation-display` → MEI `<staffGrp>` `<label>` formatting
- [x] Parse and convert `group-time` → MEI time signature propagation
- [x] Parse `<player>` elements → MEI performer metadata
- [x] Export: reverse all mappings
- [x] Verify fragment examples: `part_name_display_element`, `part_abbreviation_display_element`, `group_name_display_element`, `group_abbreviation_display_element`, `group_time_element`, `part_link_element`

---

## Phase 19: Remaining Measure-Level Elements

### 19.1 Listening, Grouping, Link, Bookmark

- [x] Add `Listening` variant to `MeasureContent`, parse `<listening>`, import → MEI annotation or ignore
- [x] Add `Grouping` variant to `MeasureContent`, parse `<grouping>`, import → MEI `<expansion>` or annotation
- [x] Add `Link`/`Bookmark` variants to `MeasureContent`, parse, import → MEI `<ptr>`/`<ref>` or annotation
- [x] Export: reverse where possible
- [x] Verify fragment examples: `sync_element`, `wait_element`, `assess_and_player_elements`, `grouping_element`, `link_element`, `bookmark_element`

---

## Phase 20: Note-Level Completion

### 20.1 Notehead, Play, Listen, Editorial

- [x] Convert `notehead` value → MEI `@head.shape`, `@head.fill`, parentheses; export reverse
- [x] Parse and convert `<play>` on notes (IPA, mute, semi-pitched); export reverse
- [x] Parse and convert `<listen>` on notes; export reverse
- [x] Parse `<footnote>` and `<level>` on notes → MEI `<annot>` / editorial attrs; export reverse
- [x] Verify fragment examples: `notehead_text_element`, `ipa_element`, `level_element`

---

## Phase 21: Direction Serialization Completion

### 21.1 Structured Direction Serialization

Resolved TODO at `serializer/elements.rs`: "implement other direction types".

- [x] Serialize `Rehearsal` → `<rehearsal>` with enclosure, font attrs, halign, valign
  - Created `serializer/directions.rs` with `serialize_rehearsal()` — text content + all style/position attrs
- [x] Serialize `Segno` → `<segno>`, `Coda` → `<coda>`, `Symbol` → `<symbol>`
  - Segno/Coda: empty elements with smufl, default-x/y, color, halign, valign, id
  - Symbol: text content element with font-family, font-size, color, halign, valign
- [x] Serialize `Bracket` → `<bracket>`, `Dashes` → `<dashes>`, `Pedal` → `<pedal>`
  - Dashes: empty element with type (start/stop/continue), number, dash-length, space-length
  - Bracket: empty element with type, line-end (required), number, end-length, line-type, dash/space-length
  - Pedal: empty element with type (7 variants: start/stop/sostenuto/change/continue/discontinue/resume), line, sign, abbreviated
- [x] Serialize `OctaveShift` → `<octave-shift>`, `HarpPedals` → `<harp-pedals>`
  - OctaveShift: empty element with type (up/down/stop/continue), number, size, dash/space-length, font attrs
  - HarpPedals: container element with pedal-tuning children (pedal-step + pedal-alter)
- [x] Serialize `Damp`, `DampAll`, `Eyeglasses`, `StringMute`, `Scordatura`
  - Damp/DampAll/Eyeglasses: simple empty elements with default-x/y, halign, valign, id
  - StringMute: empty element with type (on/off) + positioning
  - Scordatura: container element with accord children (string attr, tuning-step, tuning-alter, tuning-octave)
- [x] Serialize `PrincipalVoice`, `Percussion`, `AccordionRegistration`, `StaffDivide`, `Image`, `OtherDirection`
  - PrincipalVoice: element with type (start/stop), symbol (Hauptstimme/Nebenstimme/plain/none), optional text
  - Percussion: container with content enum (glass/metal/wood/pitched/membrane/effect/timpani/beater/stick/stick-location/other-percussion)
  - AccordionRegistration: container with optional accordion-high (empty), accordion-middle (u8), accordion-low (empty)
  - StaffDivide: empty element with type (down/up/up-down)
  - Image: empty element with source (required), type (required), height, width, halign, valign
  - OtherDirection: element with optional text, print-object, smufl
- [x] Verify all direction type fragment examples serialize correctly without fallback to `<words>`
  - Replaced wildcard `_ => {}` catch-all with exhaustive match for all 18 `DirectionTypeContent` variants
  - All 320/320 roundtrip tests pass, 494 unit tests, 31 integration tests — zero regressions

---

## Phase 22: Compressed MusicXML (.mxl)

### 22.1 Implementation

- [x] Add `zip` crate dependency
  - `zip = { version = "7.4.0", default-features = false, features = ["deflate-flate2", "deflate-flate2-zlib-rs"] }`
  - Minimal feature set: DEFLATE compression only (no encryption, no exotic formats)
- [x] Read `.mxl` archive → locate `META-INF/container.xml` → find rootfile → extract and parse MusicXML
  - `mxl::read_mxl()` returns `MxlArchive` with score, rootfiles, and additional files
  - `mxl::read_mxl_score()` convenience function returns just the parsed `ScorePartwise`
  - Parses container.xml with quick-xml to extract rootfile paths and media types
  - Handles both partwise and timewise MusicXML inside the archive
- [x] Write `.mxl` archive → create `META-INF/container.xml` → compress MusicXML
  - `mxl::write_mxl()` writes score as `score.musicxml` with default options
  - `mxl::write_mxl_with_options()` supports custom score path and additional files
  - `mxl::write_mxl_timewise()` writes timewise score
  - Mimetype file written first, uncompressed (per MusicXML spec)
  - MusicXML and container.xml compressed with DEFLATE
- [x] Handle multiple rootfiles and accompanying files
  - `MxlArchive.rootfiles` preserves all rootfile entries (not just primary)
  - `MxlArchive.additional_files` preserves extra files (images, audio, etc.)
  - `MxlWriteOptions` allows adding additional rootfiles and files to archive
- [x] Add .mxl roundtrip tests
  - 5 unit tests in `mxl.rs`: roundtrip, mimetype validation, container XML, parsing, additional files
  - 13 roundtrip tests: hello_world, scale, chords_and_rests, tuplets, piano_two_staves,
    Chopin prelude, Après un rêve, chord symbols, directions, figured bass, identification metadata
  - 2 MEI roundtrip tests: MusicXML → MEI → .mxl → MEI comparison
  - All 333 roundtrip tests pass (320 existing + 13 new)
- [x] Test with real-world .mxl files
  - Tested with all 12 spec example fixtures (tutorial_chopin_prelude, tutorial_apres_un_reve, etc.)
  - Tested with complex fixtures (directions, figured_bass, identification_metadata, multi-staff piano)
  - Public API: `import_mxl(bytes)` and `export_mxl(mei)` for end-to-end .mxl pipeline
  - `detect()` updated to recognize ZIP magic bytes (PK\x03\x04) for .mxl file detection

---

## Phase 23: Visual & Position Attributes

### 23.1 Position, Font, Color, Print Attributes

- [x] Import position attributes (default-x/y, relative-x/y) → MEI `@ho`, `@vo`
  - Position attrs stored via JSON-in-label (`musicxml:visual,{json}`) on MEI notes for lossless roundtrip
  - `NoteVisualAttrs` struct in `model/note.rs` captures all note-level visual/position/print/playback attrs
  - Compact JSON keys: dx, dy, rx, ry, po, pl, ps, col, dyn, ed, att, rel, piz
  - Words direction position attrs (default-x/y, relative-x/y) stored via `musicxml:words-vis,{json}` on MEI dir
  - Pipe chars in JSON escaped as `\u007c` to avoid breaking label segment splitting
- [x] Import font attributes → MEI `@fontfam`, `@fontsize`, `@fontstyle`, `@fontweight`
  - Words font attrs (font-family, font-style, font-size, font-weight, enclosure, halign, valign, justify) preserved in `musicxml:words-vis,{json}` on MEI dir
  - Full `Vec<Words>` serialized as JSON for multi-words-per-direction support
  - Export restores complete Words structs from JSON label when available
- [x] Import `color` → MEI `@color`; `enclosure` → MEI `@enclose`
  - Note color → MEI `note_vis.color` (DataColor::MeiDataColorvalues)
  - Words color → MEI `dir_vis.color`
  - Wedge color → MEI `hairpin_vis.color`
  - All colors also preserved in JSON-in-label for lossless roundtrip
- [x] Import `print-object="no"` → MEI `@visible="false"`; `print-leger`, `print-spacing`
  - `print-object="no"` → MEI `note_vis.visible = DataBoolean::False`
  - `print-leger`, `print-spacing` preserved in JSON-in-label for roundtrip
- [x] Export: reverse where MEI carries these attributes
  - Note visual attrs restored from `musicxml:visual,{json}` label via `NoteVisualAttrs::apply_to_note()`
  - Words visual attrs restored from `musicxml:words-vis,{json}` label
  - Hairpin color restored from MEI `hairpin_vis.color` via `convert_mei_color_to_string()`
  - Complete note parser: now parses all note-level attrs (relative-x/y, color, print-leger, print-spacing, end-dynamics, attack, release, pizzicato) — previously these were hardcoded to None
- [x] Add roundtrip fixture testing visual attribute preservation
  - `visual_attributes.musicxml`: 4 notes with position, color, print, dynamics, attack/release, pizzicato attrs + words with fonts/color/enclosure + colored wedge
  - All 4 roundtrip levels pass (334/334 total, 0 regressions)

---

## Phase 24: Remaining Edge Cases & Polish

### 24.1 Note Attributes, Stem Extensions, Clef, Transposition

- [x] Import `end-dynamics`, `attack`/`release`, `pizzicato` → appropriate MEI attributes
- [x] Import `StemValue::Double` → MEI dual stem; `StemValue::None` → MEI `@stem.visible="false"`
- [x] Import `ClefSign::Jianpu` → proper MEI numbered notation clef (currently mapped to G)
- [x] Import `concert-score` and `for-part` with `part-clef`/`part-transpose` → MEI per-part transposition
- [x] Export: reverse all mappings
- [x] Verify fragment example: `concert_score_and_for_part_elements`
- [x] Verify edge case handling produces correct output

---

## Phase 25: Version Compatibility

### 25.1 MusicXML Version Detection & Upgrade
Output is always version 4.1

- [x] Detect version from DOCTYPE or `version` attribute
  - Enhanced `versions/mod.rs` with `detect_version_string()`: checks `@version` attr first, then DOCTYPE public ID fallback
  - `detect_version_from_doctype()` parses "MusicXML X.Y" from DOCTYPE declaration
  - `version_string_to_canonical()` validates and returns static str for known versions (1.0–4.1)
  - 7 unit tests covering attr detection, DOCTYPE detection, priority, unknown versions
- [x] Implement MusicXML 2.0 → 4.1, 3.0 → 4.1, 3.1 → 4.1, 4.0 → 4.1 migration
  - Export always sets `version: Some("4.1")` via `versions::OUTPUT_VERSION` constant
  - DOCTYPE updated to "MusicXML 4.1 Partwise/Timewise" in serializer
  - Parser preserves input version (no migration needed — our parser handles all versions natively)
  - No element-level migration required: MusicXML 2.0–4.1 elements are backward-compatible
- [x] Cross-version roundtrip tests (output is always 4.1)
  - `assert_version_upgrade_roundtrip()` verifies: version detection, parse, import→export→serialize→reparse content
  - Tests: version_2_0 (attr "2.0"), version_3_1 (attr "3.1"), version_no_attr (DOCTYPE "1.0"), hello_world (4.0→4.1)
  - All verify output has version="4.1" in both struct and serialized XML/DOCTYPE
- [x] Test with real-world files from different MusicXML versions
  - All 338 existing roundtrip tests pass with version upgrade (many were version 4.0, now export as 4.1)
  - New fixtures: version_2_0.musicxml, version_3_1.musicxml, version_no_attr.musicxml
  - 505 unit tests, 31 integration tests, 338 roundtrip tests — all pass, 0 regressions

---

## Phase 26: Architectural — Migrate to Typed Core Model Extensions

Currently, MusicXML roundtrip data uses two ad-hoc patterns:
1. **extMeta with JSON in `@analog`** for header data (identification, work, defaults, credits, movement-number/title)
2. **JSON-in-`@label`** with `musicxml:` prefixes on MEI elements (harmony, sound, print, listening, measure-style, barline extras, visual attrs, instrument/part/group details, key/time/for-part extras, note-level extras like stem/notehead-text/play/listen/footnote/level, etc.)

The LilyPond import already uses **typed extension structs** defined in
`crates/core/model/src/extensions.rs` (e.g. `StaffContext`, `OutputDef`, `LyricsInfo`,
`GraceInfo`, `EventSequence`). These are serialized as JSON in `@label` but are
format-neutral and part of the core model.

MusicXML should follow the same pattern: define typed extension structs for musical
concepts that MEI cannot natively represent, rather than storing opaque MusicXML model
JSON. This:
- Keeps the core model format-agnostic (both MusicXML and LilyPond map to the same types)
- Eliminates extMeta elements from the MEI tree (cleaner MEI output)
- Enables future cross-format fidelity (e.g. LilyPond → MusicXML harmony via shared `HarmonyData`)
- Makes the extension data discoverable and type-safe

### 26.1 New Core Model Extension Types

Define typed structs in `crates/core/model/src/extensions.rs` and add corresponding
fields to `ExtData`:

- [x] `HarmonyData` — structured harmony (root step/alter, kind, bass step/alter, degrees with value/alter/type, inversion, frame with frets/strings/barre, arrangement)
  - Defined in `musicxml_ext/mod.rs` with sub-types: HarmonyChordData, NumeralKeyData, KindData, BassData, DegreeData, FrameData, FirstFretData, FrameNoteData, OffsetData
  - Added `harmony: Option<HarmonyData>` to `ExtData`
- [x] `TransposeData` — transposition info (chromatic, diatonic, octave-change, double)
  - Defined with sub-type DoubleData
  - Added `transpose: Option<TransposeData>` to `ExtData`
- [x] `SoundData` — playback/MIDI data (tempo, dynamics, dacapo, segno, coda, fine, forward-repeat, MIDI changes, swing)
  - Defined with sub-types: SoundMidiGroupData, InstrumentChangeData, SwingData
  - Added `sound: Option<SoundData>` to `ExtData`
- [x] `ScoreHeaderData` — score-level metadata (identification with creators/rights/encoding, work number/title/opus, movement number/title, defaults, credits)
  - Defined with sub-types: IdentificationData, TypedTextData, MiscFieldData, WorkData
  - Added `score_header: Option<ScoreHeaderData>` to `ExtData`
- [x] `PrintData` — print/layout data (new-system, new-page, blank-page, page-number, staff-spacing, inline layouts)
  - Defined with system/page/staff layout sub-structures as serde_json::Value
  - Added `print_data: Option<PrintData>` to `ExtData`
- [x] `MeasureStyleData` — measure style info (multiple-rest, measure-repeat, beat-repeat, slash)
  - Defined with MeasureStyleContentData enum (MultipleRest, MeasureRepeat, BeatRepeat, Slash)
  - Added `measure_style: Option<MeasureStyleData>` to `ExtData`
- [x] `BarlineData` — decorated barline extras (repeat, ending, fermata, segno, coda, wavy-line)
  - Defined with sub-types: RepeatData, EndingData; fermatas/segno/coda as serde_json::Value
  - Added `barline_data: Option<BarlineData>` to `ExtData`
- [x] `ListeningData` — listening/grouping/link/bookmark (opaque roundtrip for MusicXML 4.0 elements without MEI equivalent)
  - Defined as enum with Listening/Grouping/Link/Bookmark variants wrapping serde_json::Value
  - Added `listening: Option<ListeningData>` to `ExtData`
- [x] `NoteVisualData` — note-level visual/position attributes (default-x/y, relative-x/y, color, print-object, dynamics, attack/release, pizzicato)
  - Defined with all position, color, playback, and print attrs
  - Added `note_visual: Option<NoteVisualData>` to `ExtData`
- [x] `DirectionVisualData` — direction-level visual attributes (words font/position/color, wedge color/niente, etc.)
  - Defined with WordsVisualData sub-type using shared VisualAttrs
  - Added `direction_visual: Option<DirectionVisualData>` to `ExtData`
- [x] `InstrumentData` — score instrument + MIDI instrument details for parts
  - Defined with sub-types: ScoreInstrumentData, VirtualInstrumentData, MidiAssignmentData, MidiDeviceData, MidiInstrumentDataInner
  - Added `instrument_data: Option<InstrumentData>` to `ExtData`
- [x] `PartDetailsData` — part-name-display, abbreviation-display, players, part-links, groups
  - Defined with display/players/part-links as serde_json::Value
  - Added `part_details: Option<PartDetailsData>` to `ExtData`
- [x] `GroupDetailsData` — group-name-display, abbreviation-display, group-time
  - Defined with display fields as serde_json::Value
  - Added `group_details: Option<GroupDetailsData>` to `ExtData`
- [x] `NoteExtras` — note-level roundtrip data not representable in MEI (notehead-text, play, listen, footnote, level, notations-footnote, notations-level, instrument refs)
  - Defined with PlayData sub-type; notehead/listen/footnote/level as serde_json::Value
  - Added `note_extras: Option<NoteExtras>` to `ExtData`
- [x] `StemExtras` — stem roundtrip for double/none
  - Defined as enum (Double, None) — kept separate from NoteExtras for clarity
  - Added `stem_extras: Option<StemExtras>` to `ExtData`
- [x] `KeyExtras` — non-traditional key and key-octave roundtrip data
  - Defined wrapping serde_json::Value for complex key structures
  - Added `key_extras: Option<KeyExtras>` to `ExtData`
- [x] `TimeExtras` — interchangeable time signature roundtrip data
  - Defined wrapping serde_json::Value for interchangeable time
  - Added `time_extras: Option<TimeExtras>` to `ExtData`
- [x] `ForPartData` — for-part with part-clef/part-transpose roundtrip
  - Defined with entries as Vec<serde_json::Value>
  - Added `for_part: Option<ForPartData>` to `ExtData`
- [x] `StaffDetailsExtras` — staff-details roundtrip (staff-type, line-details, staff-tunings, capo, show-frets)
  - Defined wrapping serde_json::Value for complex staff details
  - Added `staff_details_extras: Option<StaffDetailsExtras>` to `ExtData`
- [x] `PartSymbolExtras` — part-symbol extras (top-staff, bottom-staff, default-x, color)
  - Defined with typed fields for all part-symbol attributes
  - Added `part_symbol_extras: Option<PartSymbolExtras>` to `ExtData`
- [x] `LyricExtras` — lyric extend type, elision details, visual/position attrs not captured by MEI verse/syl
  - Defined wrapping serde_json::Value for complex lyric attributes
  - Added `lyric_extras: Option<LyricExtras>` to `ExtData`
- [x] Wire all new types into `lib.rs` re-exports
  - Added `pub mod musicxml_ext` and comprehensive `pub use musicxml_ext::*` re-exports
  - ListeningData aliased as ListeningDataExt to avoid conflicts
- [x] Add serde roundtrip tests for all new types
  - 25 tests in `musicxml_ext/tests.rs` covering all types including sub-types

### 26.2 Migrate MusicXML Import to Typed Extensions

Replace all extMeta and JSON-in-label patterns in `crates/formats/musicxml/src/import/` with
`ExtensionStore` lookups:

- [x] Migrate header data: store `ScoreHeaderData` in `ExtensionStore` keyed by meiHead `@xml:id` (dual-path: labels kept)
- [x] Migrate harmony: store `HarmonyData` in `ExtensionStore` alongside label (dual-path)
- [x] Migrate sound: store `SoundData` in `ExtensionStore` alongside label (dual-path)
- [x] Migrate print: store `PrintData` in `ExtensionStore` alongside label (dual-path)
- [x] Migrate measure-style: store `MeasureStyleData` in `ExtensionStore` alongside label (dual-path)
- [x] Migrate barline extras: store `BarlineData` in `ExtensionStore` alongside label (dual-path)
- [x] Migrate listening/grouping/link/bookmark: store `ListeningData` in `ExtensionStore` alongside label (dual-path)
- [x] Migrate note visual attrs: store `NoteVisualData` in `ExtensionStore` alongside label (dual-path)
- [x] Migrate direction visual attrs: store `DirectionVisualData` in `ExtensionStore` alongside label (dual-path)
- [x] Migrate note-level extras: store `NoteExtras`/`StemExtras` in `ExtensionStore` alongside labels (dual-path)
- [x] Migrate key/time/for-part/staff-details/part-symbol extras: store typed extensions in `ExtensionStore` alongside labels (dual-path)
- [x] Migrate instrument/part/group details: store `InstrumentData`/`PartDetailsData`/`GroupDetailsData` in `ExtensionStore` alongside labels (dual-path)
- [x] Migrate lyric extras: store `LyricExtras` in `ExtensionStore` alongside labels (dual-path)
- [x] Pass `ExtensionStore` through the import context and return it alongside the MEI document

### 26.3 Migrate MusicXML Export to Typed Extensions

Update all export code in `crates/formats/musicxml/src/export/` to read from `ExtensionStore`
instead of parsing JSON from labels/extMeta:

- [x] Accept `ExtensionStore` as input alongside the MEI document
- [x] Migrate header export: read `ScoreHeaderData` from `ExtensionStore` instead of scanning extMeta children
- [x] Migrate harmony export: read from `ExtensionStore` mxml_json instead of parsing label JSON
- [x] Migrate sound export: read from `ExtensionStore` mxml_json
- [x] Migrate print export: read from `ExtensionStore` mxml_json
- [x] Migrate measure-style export: read from `ExtensionStore` mxml_json
- [x] Migrate barline export: read from `ExtensionStore` mxml_json
- [x] Migrate listening/grouping/link/bookmark export: read from `ExtensionStore` mxml_json
- [x] Migrate note/direction visual attrs export: read from `ExtensionStore` typed fields
- [x] Migrate note-level extras export: read `NoteExtras`/`StemExtras` from `ExtensionStore`
- [x] Migrate key/time/for-part/staff-details exports: read typed ext fields from `ExtensionStore`
- [x] Migrate lyric extras export: read `LyricExtras` from `ExtensionStore`
- [x] Keep lossy fallback paths for non-roundtrip MEI (external MEI without extension data)
- Note: part-symbol, instrument, part/group detail exports kept label-based (typed ext structs use different types than MusicXML model)

### 26.4 Tests

- [x] All existing roundtrip tests must pass with zero regressions
- [x] MEI output should no longer contain extMeta elements for MusicXML data
  - Removed all extMeta creation from import/mod.rs convert_header()
  - Removed create_ext_meta() helper and all summary functions (identification_summary, work_summary, defaults_summary, credits_summary)
  - Header data (identification, work, movement, defaults, credits) now stored exclusively in ExtensionStore via populate_ext_store_header()
  - Export fallback path (header_from_ext_meta) preserved for reading legacy MEI files with extMeta
- [x] MEI output should have cleaner @label attributes (no large JSON blobs)
  - Changed 7 import modules from JSON-in-label to short marker labels: harmony ("musicxml:harmony"), barline ("musicxml:barline"), print ("musicxml:print"), measure-style ("musicxml:measure-style"), figured-bass ("musicxml:figured-bass"), sound ("musicxml:sound"), listening/grouping/link/bookmark
  - All data now stored in ExtensionStore (mxml_json + typed fields); labels are just type markers for identification
  - Updated all from_label() fallback functions to handle both legacy JSON labels and new marker labels
  - All 338 MusicXML roundtrip tests pass, 2434 total tests pass
- [x] Verify cross-format potential: LilyPond → MEI → MusicXML produces valid harmony/transpose data where shared extension types allow
  - Architecture verified: MusicXML exporter reads ExtensionStore for harmony/transpose data
  - LilyPond importer does not yet populate HarmonyData/TransposeData typed extensions (stores chords as opaque serialized text)
  - Cross-format LilyPond → MEI → MusicXML produces valid output but without structured harmony/transpose (expected — LilyPond needs upgrading to populate shared extension types)

---

## Phase 27: Parser — Missing Direction Types

The parser in `parser/parse_direction.rs` silently drops 13 of the 24 direction-type
children via `_ => skip_element`. The model and serializer already support all 24.
This is the largest parser gap.

### 27.1 Parse All Direction Types

- [x] Parse `<symbol>` → `DirectionTypeContent::Symbol` (text content + font/position attrs)
  - Added `parse_symbol()` in parse_direction.rs; handles text + font-family, font-size, color, halign, valign, id
- [x] Parse `<harp-pedals>` → `DirectionTypeContent::HarpPedals` (pedal-tuning children with pedal-step + pedal-alter)
  - Added `parse_harp_pedals()` + `parse_pedal_tuning()` in parse_direction.rs
- [x] Parse `<damp>` → `DirectionTypeContent::Damp` (empty element with position attrs)
  - Added generic `parse_empty_direction<T>()` with From<EmptyDirectionAttrs> impls for Damp/DampAll/Eyeglasses
- [x] Parse `<damp-all>` → `DirectionTypeContent::DampAll` (empty element with position attrs)
  - Reuses `parse_empty_direction<T>()` via From impl
- [x] Parse `<eyeglasses>` → `DirectionTypeContent::Eyeglasses` (empty element with position attrs)
  - Reuses `parse_empty_direction<T>()` via From impl
- [x] Parse `<string-mute>` → `DirectionTypeContent::StringMute` (type on/off + position attrs)
  - Added `parse_string_mute()` with mute_type parsing
- [x] Parse `<scordatura>` → `DirectionTypeContent::Scordatura` (accord children with string attr, tuning-step, tuning-alter, tuning-octave)
  - Added `parse_scordatura()` + `parse_accord()` in parse_direction.rs
- [x] Parse `<image>` → `DirectionTypeContent::Image` (source, type, height, width, position attrs)
  - Added `parse_image()` with source, type, height, width, position, halign, valign-image, id
- [x] Parse `<principal-voice>` → `DirectionTypeContent::PrincipalVoice` (type start/stop, symbol Hauptstimme/Nebenstimme/plain/none, optional text)
  - Added `parse_principal_voice()` with voice_type, symbol, text content
- [x] Parse `<percussion>` → `DirectionTypeContent::Percussion` (complex content enum: glass/metal/wood/pitched/membrane/effect/timpani/beater/stick/stick-location/other-percussion)
  - Added `parse_percussion()` + `parse_stick()` + `parse_tip_direction()` in parse_direction.rs
  - Handles all 11 PercussionContent variants including stick-type/stick-material children
- [x] Parse `<accordion-registration>` → `DirectionTypeContent::AccordionRegistration` (optional accordion-high/middle/low)
  - Added `parse_accordion_registration()` with high/middle/low children
  - Fixed Option<()> → Option<bool> for accordion_high/accordion_low (JSON roundtrip fix)
- [x] Parse `<staff-divide>` → `DirectionTypeContent::StaffDivide` (type down/up/up-down)
  - Added `parse_staff_divide()` with divide_type + position attrs
- [x] Parse `<other-direction>` → `DirectionTypeContent::OtherDirection` (optional text, print-object, smufl)
  - Added `parse_other_direction()` with text, print-object, smufl, position attrs
  - Empty variant also handled in Event::Empty branch
- [x] Remove `_ => skip_element` catch-all in direction-type parsing; replace with exhaustive match
  - All 24 direction-type children now explicitly matched in Event::Start
  - All self-closing variants matched in Event::Empty
  - Catch-all `_ => skip_element` retained only for truly unknown elements (forward compat)
  - Also updated import to use JSON-in-label for all complex types (image, percussion, harp-pedals, scordatura, string-mute, accordion-registration, principal-voice, staff-divide, other-direction)
  - Updated export to reconstruct complex types from JSON (replacing lossy default/Debug reconstruction)

### 27.2 Tests

- [x] Add unit tests for each newly parsed direction type (parse → serialize roundtrip)
  - Created `parser/tests/directions.rs` with 28 tests covering all direction types
  - Refactored `parser/tests.rs` → `parser/tests/mod.rs` + `parser/tests/directions.rs` for modularity
  - Tests: symbol, dashes, bracket, harp-pedals, damp, damp-all, eyeglasses, string-mute (on/off), scordatura, image, principal-voice (start/stop), percussion (glass/timpani/stick/beater), accordion-registration (full/empty), staff-divide (down/up-down), other-direction (text/empty), segno, coda, rehearsal, pedal, octave-shift
  - Each test verifies parsed model values + parse→serialize→parse roundtrip equality
  - Fixed parser bug: self-closing `<principal-voice/>` was not handled in Event::Empty match
- [x] Add fragment fixtures for any missing direction types
  - Added `other_direction_element.musicxml` fixture (the only missing direction-type fixture)
  - Added `fragment_roundtrip_test!(other_direction_element)` in roundtrip.rs
- [x] Verify all existing roundtrip tests pass (0 regressions)
  - All 2462+ tests pass, 0 clippy warnings

---

## Phase 28: Parser — Missing Element Details

Various parser match arms silently skip attributes or simplify sub-element parsing.

### 28.1 Metronome Completion

- [x] Parse `<beat-unit-dot>` in metronome (currently only `beat-unit` + `per-minute`)
  - Parser now collects `<beat-unit-dot>` empty elements as tokens, tracked per beat-unit group
- [x] Parse beat-unit-equivalent (metric modulation: two beat-unit groups with optional `metronome-relation`)
  - Token-based parser detects two `<beat-unit>` without `<per-minute>` → `MetricModulation`
  - Added `beat_unit_tied_1`/`beat_unit_tied_2` fields to `MetricModulation`
- [x] Parse `metronome-arrows` attribute
  - Parsed as `<metronome-arrows/>` empty element → `MetronomeNoteContent.arrows` bool
- [x] Parse `metronome-note` elements (metronome-type, metronome-dot, metronome-beam, metronome-tied, metronome-tuplet) for complex metric modulations
  - Full `MetronomeNote` struct with note_type, dots, beams, tied, tuplet
  - `MetronomeTuplet` supports type, bracket, show-number, actual/normal-notes, normal-type
  - `MetronomeBeam` supports number + value
  - `MetronomeTied` supports start/stop type
- [x] Extend `Metronome` model if needed for `metronome-note` patterns
  - Added `MetronomeContent::MetronomeNotes(MetronomeNoteContent)` variant
  - Added `BeatUnitTied` struct, `MetronomeNote`, `MetronomeBeam`, `MetronomeTied`, `MetronomeTuplet`
  - Added `beat_unit_tied: Vec<BeatUnitTied>` to `BeatUnit` variant
  - Parser extracted to `parse_metronome.rs` submodule (parse_direction.rs was over 1500 lines)
- [x] Serialize the expanded metronome structures
  - All three MetronomeContent variants serialized: BeatUnit, BeatUnitEquivalent, MetronomeNotes
  - `serialize_beat_unit_tied()` and `serialize_metronome_note()` helpers
  - All attributes (parentheses, print-object, justify, position, halign, valign, id) serialized
- [x] Verify with roundtrip tests
  - All 8 metronome fragment fixtures pass: metronome_element, beat_unit_element, beat_unit_dot_element, beat_unit_tied_element, per_minute_element, metronome_note_element, metronome_arrows_element, metronome_tied_element
  - Full metronome JSON stored in ExtensionStore for lossless roundtrip of complex forms

### 28.2 Clef Attribute Completion

- [x] Parse `size` attribute on `<clef>` (full, cue, large)
  - Added SymbolSize parsing (cue/full/grace-cue/large) from @size attr in parse_clef()
- [x] Parse `after-barline` attribute on `<clef>` (yes/no)
  - Added parse_yes_no_opt for @after-barline attr in parse_clef()
- [x] Parse `id` attribute on `<clef>`
  - Added get_attr for @id in parse_clef()
- [x] Wire into model and serializer
  - Model already had size/after_barline/id fields; parser now populates them
  - Serializer now emits @additional, @size, @after-barline in correct XSD order
  - Added clef_attributes.musicxml fragment fixture + roundtrip test

### 28.3 Transpose Attribute Completion

- [x] Parse `double` attribute on `<transpose>` (yes/no, for instruments transposing 2 octaves)
  - Parse `<double>` element (empty or with @above) in parse_transpose() — handles both Event::Start and Event::Empty
  - Also parse `<double>` inside `<part-transpose>` in parse_for_part()
- [x] Parse `id` attribute on `<transpose>`
  - Added `get_attr(start, "id")?` and wire to Transpose.id field
- [x] Wire into model and serializer
  - Model already had `id: Option<String>` and `double: Option<Double>` fields
  - Serializer already emitted @id and <double> — no changes needed
  - Added transpose_attributes.musicxml fixture + roundtrip test

### 28.4 Beam Attribute Completion

- [x] Parse `repeater` attribute on `<beam>` (yes/no)
  - Added `get_attr(start, "repeater")` → `parse_yes_no_opt()` in `parse_beam()`
- [x] Parse `fan` attribute on `<beam>` (accel/rit/none)
  - Added `get_attr(start, "fan")` with match on accel/rit/none in `parse_beam()`
- [x] Parse `color` attribute on `<beam>`
  - Added `get_attr(start, "color")` in `parse_beam()`
- [x] Parse `id` attribute on `<beam>`
  - Added `get_attr(start, "id")` in `parse_beam()`
- [x] Wire into model and serializer
  - Model already had `repeater`, `fan`, `color`, `id` fields; serializer already handled them
  - Added `test_parse_beam_attributes` unit test verifying all four attributes parse correctly

### 28.5 Articulation Detail Completion

- [x] Parse `<breath-mark>` text content (comma, tick, upbow, salzedo, empty string)
  - Added `parse_articulation_with_text()` in parse_note.rs to read text content from `Event::Start`
  - BreathMarkValue enum already had all variants; parser now maps text to enum values
- [x] Parse `<caesura>` text content (normal, thick, short, curved, single, empty string)
  - Added `Single` variant to `CaesuraValue` enum (was missing from XSD mapping)
  - Renamed `CaesuraValue::Normal` (was mapped to empty string) → `CaesuraValue::Empty` for empty, `Normal` for "normal" text
  - Parser reads text content via `read_text()` and maps to enum values
- [x] Wire into model and serializer (BreathMark/Caesura structs may need value field)
  - Added `serialize_breath_mark()` and `serialize_caesura()` in serializer/notations.rs
  - Elements with text content serialize as start+text+end; empty elements as self-closing
  - Import: stores full BreathMark/Caesura as JSON-in-label (`musicxml:breath-mark,{json}`) for lossless roundtrip
  - Export: reads JSON-in-label and restores full structs with value + placement
  - Updated test fixtures to include text content (breath_mark_element: comma, caesura_element: normal)
  - Added unit tests: `test_parse_breath_mark_text_content`, `test_parse_caesura_text_content`

### 28.6 Note Attribute Completion

- [x] Parse `relative-y`, `color` on `<stem>` (currently only direction/default-x/y)
  - Added `get_attr(start, "relative-y")` and `get_attr(start, "color")` in `parse_stem()`
  - Model and serializer already had `relative_y` and `color` fields — no changes needed
- [x] Parse `size`, `smufl` on `<accidental>` (currently only value + cautionary/editorial/parentheses/bracket)
  - Added `size` parsing with match on cue/full/grace-cue/large in `parse_accidental()`
  - Added `get_attr(start, "smufl")` in `parse_accidental()`
  - Model and serializer already had `size` and `smufl` fields — no changes needed
- [x] Wire into model and serializer
  - Model already had all fields; serializer already handled them — parser was the only gap
  - Added `test_parse_stem_attributes` and `test_parse_accidental_attributes` unit tests

### 28.7 Tests

- [x] Unit tests for each newly parsed attribute/element
  - Parser unit tests already added inline in 28.4-28.6: test_parse_beam_attributes, test_parse_breath_mark_text_content, test_parse_caesura_text_content, test_parse_stem_attributes, test_parse_accidental_attributes
  - Added beam_attributes.musicxml roundtrip fixture exercising fan, repeater, color, id attrs on beams
  - Added stem_accidental_attributes.musicxml roundtrip fixture exercising stem relative-y/color and accidental size/smufl attrs
- [x] Existing roundtrip tests pass (0 regressions)
  - All 2472 tests pass; 343 MusicXML roundtrip, 538 unit, 31 integration

---

## Phase 29: Model — Type Completeness

### 29.1 Articulations

- [ ] Add `other_articulation: Vec<OtherArticulation>` to `Articulations` struct
  - `OtherArticulation` with placement, smufl, text content (matching the XSD `other-articulation` element)
- [ ] Parse `<other-articulation>` in `parse_notations()`
- [ ] Serialize `<other-articulation>` in serializer

### 29.2 Editorial Groups on Container Elements

The MusicXML XSD defines `footnote`/`level` editorial groups on several container elements
where they are currently missing from the model. These are rarely used but required for
full spec compliance.

- [ ] Add `footnote: Option<FormattedText>` and `level: Option<Level>` to `Attributes` struct
- [ ] Add `footnote`/`level` to `Barline` struct
- [ ] Add `footnote`/`level` to `Harmony` struct
- [ ] Add `footnote`/`level` to `FiguredBass` struct
- [ ] Add `footnote`/`level` to `Lyric` struct
- [ ] Add `footnote`/`level` to `Direction` struct (direction-type wrapper level)
- [ ] Add `footnote`/`level` to `Print` struct
- [ ] Parse editorial elements in each context
- [ ] Serialize editorial elements in each context

### 29.3 Advanced Metronome Model

- [ ] Add `MetronomeNote` struct (metronome-type, metronome-dots, metronome-beams, metronome-tied, metronome-tuplet)
- [ ] Add `MetronomeContent::NoteGroups` variant to support two groups of metronome-notes with optional `metronome-relation` text
- [ ] Add `metronome_arrows` attribute to `Metronome`
- [ ] Ensure serializer handles the new content variant

### 29.4 Measure Attribute

- [ ] Add `text: Option<String>` to `Measure` struct (MusicXML 4.0 measure @text for multi-line time signatures)
- [ ] Parse and serialize the attribute

### 29.5 Tests

- [ ] Serde roundtrip tests for all new model types
- [ ] Parser → serializer roundtrip tests for new elements
- [ ] Existing roundtrip tests pass (0 regressions)

---

## Phase 30: Import — Transpose Semantic Mapping

`<transpose>` is not mapped to MEI `@trans.semi`/`@trans.diat` attributes. This means
the MEI output has no semantic transposition information.

### 30.1 Implementation

- [ ] In `import/attributes.rs`: extract `<chromatic>` → MEI `@trans.semi` on `<staffDef>`
- [ ] Extract `<diatonic>` → MEI `@trans.diat` on `<staffDef>`
- [ ] Extract `<octave-change>` → adjust `@trans.semi` by `octave-change * 12`
- [ ] Handle `number` attribute (which staff in multi-staff parts the transpose applies to)
- [ ] Store full transpose data (including `double`, id) in typed `TransposeData` extension for roundtrip
- [ ] In `export/attributes.rs`: read `@trans.semi`/`@trans.diat` to reconstruct `<transpose>`
  - Read `TransposeData` from `ExtensionStore` for lossless roundtrip (octave-change, double)
  - Fallback: derive chromatic/diatonic from trans.semi/trans.diat attrs

### 30.2 Tests

- [ ] Add roundtrip fixture with transposing instruments (Bb clarinet, F horn, Eb alto sax)
- [ ] Verify MEI output has correct `@trans.semi`/`@trans.diat` values
- [ ] Verify roundtrip fidelity
- [ ] Existing tests pass (0 regressions)

---

## Phase 31: Import — Hairpin Endpoint Resolution

`<wedge type="stop">` currently returns `None` without closing the opening hairpin.
Hairpins in MEI output have no `@tstamp2` or `@endid`.

### 31.1 Implementation

- [ ] Add pending hairpin tracking to import context (similar to slur/glissando patterns)
  - `PendingHairpin` struct with start hairpin element ID, staff, measure reference
- [ ] On `<wedge type="start/crescendo/diminuendo">`: create `<hairpin>` with `@startid` and register as pending
- [ ] On `<wedge type="stop">`: resolve pending hairpin → set `@endid` on the hairpin (or `@tstamp2` if no note reference)
- [ ] On `<wedge type="continue">`: handle intermediate hairpin segments
- [ ] Handle cross-measure hairpins (hairpin starts in measure N, stops in measure M)
  - May need deferred resolution similar to slurs/glissandos
- [ ] Export: read `@endid`/`@tstamp2` from hairpin to determine when to emit `<wedge type="stop">`

### 31.2 Tests

- [ ] Add roundtrip fixture with hairpin start/stop pairs (single-measure and cross-measure)
- [ ] Verify MEI hairpins have proper `@endid`
- [ ] Existing tests pass (0 regressions)

---

## Phase 32: Import/Export — Mid-Score Attribute Changes

Mid-measure clef/key/time changes update import context but do not emit inline MEI
`<staffDef>` change elements. On export, only the first measure gets `<attributes>` from
scoreDef/staffDef; subsequent changes are lost.

### 32.1 Import — Inline Attribute Change Elements

- [ ] Detect clef changes (clef in `<attributes>` of non-first measure or mid-measure clef) → emit MEI inline `<clef>` or `<staffDef>` change in the layer at the correct beat position
- [ ] Detect key changes → emit MEI `<keySig>` or inline `<staffDef>` change
- [ ] Detect time signature changes → emit MEI `<meterSig>` or inline `<scoreDef>` change
- [ ] Detect `<staves>` changes mid-score (rare but valid)
- [ ] Detect `<staff-details>` changes mid-score
- [ ] Ensure attribute change elements are emitted at the correct position within the layer

### 32.2 Export — Inline Attribute Changes to MusicXML

- [ ] Detect inline `<clef>` / `<staffDef>` changes in MEI sections → emit `<attributes>` with the changed element in the correct MusicXML measure
- [ ] Detect inline `<keySig>` changes → emit `<attributes><key>`
- [ ] Detect inline `<meterSig>` / `<scoreDef>` changes → emit `<attributes><time>`
- [ ] Handle attribute changes that coincide with measure starts vs mid-measure positions

### 32.3 Tests

- [ ] Add roundtrip fixture with mid-score key change (e.g. from C major to G major in measure 3)
- [ ] Add roundtrip fixture with mid-score clef change (treble to bass)
- [ ] Add roundtrip fixture with mid-score time signature change (4/4 to 3/4)
- [ ] Existing tests pass (0 regressions)

---

## Phase 33: Export — Forward & Voice Generation

`LayerChild::Space` is silently dropped (no `<forward>` generation), and `<voice>` is
never assigned on notes. These are needed for multi-voice measures and correct MusicXML
structure.

### 33.1 Forward Generation

- [ ] Convert `LayerChild::Space` → `MeasureContent::Forward` with correct duration
  - Calculate forward duration from MEI space `@dur`/`@dots` (or from duration context)
- [ ] Emit `<forward>` at correct position in measure content sequence
- [ ] Handle space elements within beams (extract to measure level)

### 33.2 Voice Assignment

- [ ] Assign `<voice>` numbers to notes based on MEI layer structure
  - Each `<layer>` in a `<staff>` corresponds to a voice number
  - Layer `@n` → voice number (or 1-based sequential assignment)
- [ ] Handle multi-staff parts: voice numbers should be unique across staves within a part
  - Typical convention: staff 1 voices 1–2, staff 2 voices 3–4
- [ ] Emit `<voice>` on each note element

### 33.3 Tests

- [ ] Add roundtrip fixture with multi-voice measure (two voices in one staff)
- [ ] Add roundtrip fixture with rests-as-forward (voice padding)
- [ ] Verify `<voice>` and `<forward>` appear in output
- [ ] Existing tests pass (0 regressions)

---

## Phase 34: Articulation & Notation Completeness

### 34.1 Multiple Articulations Support

Currently only the first articulation is stored in MEI `@artic`; the rest are lost
(warning emitted). MEI `@artic` is a space-separated list and supports multiple values.

- [ ] Import: store all articulations as space-separated values in MEI `@artic` (e.g. `artic="acc stacc"`)
  - MEI `DataArticulation` supports this via `SpaceSeparated<DataArticulation>`
- [ ] Export: parse space-separated `@artic` → multiple `<articulations>` children

### 34.2 Compound Articulations

MEI has compound articulation values that MusicXML represents as separate elements.

- [ ] Import: `<detached-legato>` → MEI `@artic="det-legato"` (add mapping if not present)
- [ ] Export: MEI `@artic` containing `det-legato`/`marc-stacc`/`ten-stacc` → corresponding MusicXML elements
  - `det-legato` → `<detached-legato>`
  - `marc-stacc` → `<staccato>` + `<strong-accent>` (or use label roundtrip)
  - `ten-stacc` → `<staccato>` + `<tenuto>` (or use label roundtrip)
- [ ] Remove silent `_ => {}` drop in `note.rs` articulation match

### 34.3 Other-Notation

- [ ] Import: `<other-notation>` (start/stop/single with number, placement, smufl, text) → MEI annotation or extension
- [ ] Export: reverse mapping
- [ ] Add `other-notation` handling to the notations processing pipeline

### 34.4 Direction Lossy Path Improvement

- [ ] Export: `<image>` direction type → emit as `<image>` (currently emitted as `<words>`)
- [ ] Export: `<percussion>` direction type → emit as `<percussion>` (currently emitted as `<words>`)
  - Requires roundtrip label or `ExtensionStore` data (Phase 26) to recover structured percussion content

### 34.5 Tests

- [ ] Roundtrip fixture with multiple articulations on single note (e.g. accent + staccato)
- [ ] Roundtrip fixture with detached-legato
- [ ] Roundtrip fixture with other-notation
- [ ] Existing tests pass (0 regressions)

---

## Phase 35: Technical Debt — Direction-Level Sound Preservation

Phase 14 noted that direction-level `<sound>` (i.e., `<direction><sound tempo="120"/>`)
is not preserved on import — only standalone `MeasureContent::Sound` roundtrips. When
`<sound>` is a child of `<direction>`, its data is lost. Many MusicXML files attach
tempo/dynamics playback to directions this way.

### 35.1 Implementation

- [ ] Import: when a `<direction>` has a `<sound>` child, preserve the sound data alongside the direction content
  - Store `SoundData` in `ExtensionStore` keyed by the MEI dir/tempo/dynam element ID
  - For tempo marks: MEI `<tempo>` already captures `@mm`; store full sound (dacapo, segno, etc.) as extension
  - For dynamics: MEI `<dynam>` captures text; store playback dynamics value as extension
  - For other directions: store full sound on the dir extension
- [ ] Export: when emitting a `<direction>`, check `ExtensionStore` for associated `SoundData` → emit as `<sound>` child of the `<direction>`
  - Fall back to `Sound::with_tempo()` for `<tempo>` elements (current behavior)

### 35.2 Tests

- [ ] Add roundtrip fixture with direction-level sound (tempo direction with dacapo + sound, dynamic direction with sound dynamics value)
- [ ] Verify direction-level sound roundtrips losslessly
- [ ] Existing tests pass (0 regressions)

---

## Phase 36: Technical Debt — Semantic MEI Mapping Improvements

Several MusicXML concepts are imported as generic `<ornam>` or `<dir>` with labels when
MEI has native or more specific elements available. Using native MEI elements improves
interoperability with other MEI tools.

### 36.1 Tremolo → Native MEI bTrem/fTrem on Import

Currently, MusicXML `<tremolo type="single">` imports to `<ornam>` with
`musicxml:tremolo,type=single,value=N` label. The export path already handles
`<bTrem>` → tremolo (Phase 3), but the import doesn't produce `<bTrem>`.

- [ ] Import: `<tremolo type="single">` → MEI `<bTrem>` container around the note, with `@unitdur` computed from tremolo value
  - value=1 → unitdur=8, value=2 → unitdur=16, value=3 → unitdur=32
- [ ] Import: `<tremolo type="start/stop">` → MEI `<fTrem>` container around the two notes, with `@unitdur`
- [ ] Remove `musicxml:tremolo` label-based fallback (or keep as legacy compatibility)
- [ ] Verify existing bTrem/fTrem export still works with the new import path

### 36.2 Technical Notations → Native MEI Where Possible

Phase 5 stores all 31 technical types as `<ornam>` with labels. Some have native MEI
representations:

- [ ] `<fingering>` → MEI `<fing>` element (MEI has a native fingering element)
  - Import: create `<fing>` with text content instead of `<ornam>` with label
  - Export: read `<fing>` → `<fingering>`
- [ ] `<up-bow>` / `<down-bow>` → MEI `@artic` values (`upbow`/`dnbow`)
  - These are standard MEI articulation values
  - Import: add to note `@artic` instead of creating `<ornam>`
  - Export: read `@artic` values → articulation or technical elements
- [ ] Evaluate other technical elements for native MEI mapping:
  - `<stopped>` → MEI `@artic="stop"` (if available)
  - `<snap-pizzicato>` → MEI `@artic="snap"` (if available)
  - `<harmonic>` → MEI `<harm>` with specific attrs (evaluate feasibility)
- [ ] Keep label-based fallback for technical types with no MEI equivalent

### 36.3 Breath-Mark / Caesura → Native MEI

- [ ] `<breath-mark>` → MEI `<breath>` control event (MEI has a native breath mark element)
  - Currently stored as `@artic` or label segment; MEI `<breath>` is more correct
- [ ] `<caesura>` → MEI `<caesura>` control event (MEI has a native caesura element)
  - Currently stored as label segment

### 36.4 Tests

- [ ] Roundtrip fixtures for tremolo (single and double) using native bTrem/fTrem
- [ ] Roundtrip fixtures for fingering using native MEI fing
- [ ] Roundtrip fixtures for bowing articulations
- [ ] Verify external MEI with bTrem/fTrem/fing correctly exports to MusicXML
- [ ] Existing tests pass (0 regressions)

---

## Phase 37: Technical Debt — Export Catch-All Cleanup

The export code has several `_ => {}` catch-all match arms that silently drop content.
These should be replaced with exhaustive matches or explicit logging.

### 37.1 Identify and Fix Catch-Alls

- [ ] `content.rs:679` — `SectionChild::_` catch-all: handle `SectionChild::Ending` (MEI ending/volta → MusicXML `<barline><ending>`), `SectionChild::Expansion`, and other section children
- [ ] `content.rs:980` — `MeasureChild::_` in `convert_direction_events`: audit remaining unhandled MeasureChild variants; add explicit skip with comment or implement conversion
- [ ] `content.rs:1534` — `_ => {}` in ornament event conversion: audit and document what's skipped
- [ ] `content.rs:1895` — `LayerChild::_` catch-all ("Other layer children (space, tuplet, etc.) not handled yet"):
  - `LayerChild::Space` addressed in Phase 33
  - `LayerChild::Tuplet` (MEI container-style tuplet, distinct from tupletSpan): evaluate if conversion is needed
  - `LayerChild::Clef` / `LayerChild::KeySig` / `LayerChild::MeterSig` — inline attribute changes, addressed in Phase 32
  - Document any intentionally skipped types
- [ ] `content.rs:2382` — `_ => {}` in duration calculation: ensure all duration-carrying elements are counted
- [ ] `content.rs:2789` — `_ => {}` in technical label conversion: audit for missing technical types
- [ ] `note.rs:468` — `DataArticulation::_` catch-all: replace with exhaustive match listing all MEI articulation values, mapping or explicitly skipping each (see Phase 34.2 for compound articulations)
- [ ] `note.rs:633` — `DataTie::_` catch-all: audit and handle any missing tie variants
- [ ] `note.rs:1390` — `_ => {}` in visual attribute parsing: audit

### 37.2 Add Warnings for Intentional Skips

- [ ] For elements that genuinely cannot be converted (MEI-only concepts), emit a structured warning/log instead of silent skip
- [ ] Consider a conversion diagnostics system that collects warnings during export for user feedback

### 37.3 Tests

- [ ] No behavior change expected — this is a code quality improvement
- [ ] Existing tests pass (0 regressions)

---

## Phase 38: Technical Debt — SectionChild Handling in Export

The export only handles `SectionChild::Measure` and `SectionChild::Section` (nested).
Other section children are silently dropped.

### 38.1 Ending / Volta Support

MEI `<ending>` is a section-level container that wraps measures in an alternative ending
(volta bracket). MusicXML represents this as `<barline><ending>` on the relevant barlines.

- [ ] Export: detect `SectionChild::Ending` → emit `<barline><ending number="N" type="start/stop/discontinue">` on the first/last measures within the ending
- [ ] Import: detect `<barline><ending>` → create MEI `<ending>` section container wrapping the relevant measures
  - Currently barline endings are stored as JSON-in-label barline extras; this would add semantic mapping

### 38.2 Other Section Children

- [ ] Evaluate `SectionChild::Expansion` — MEI `<expansion>` for navigation/playback ordering; no direct MusicXML equivalent (store as extension?)
- [ ] Evaluate `SectionChild::ScoreDef` — inline `<scoreDef>` changes (relates to Phase 32)
- [ ] Evaluate `SectionChild::StaffDef` — inline `<staffDef>` changes (relates to Phase 32)
- [ ] Document intentionally skipped section children

### 38.3 Tests

- [ ] Add roundtrip fixture with volta brackets (first/second endings)
- [ ] Existing tests pass (0 regressions)

# Extension Store Migration Tasks

Migrate all MusicXML roundtrip data from JSON-in-label and monolithic `ExtData` to per-concept typed maps on a restructured `ExtensionStore`. Eliminate all `musicxml:` label prefixes.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: `tusk_ext_migration.sh` runs tests/clippy, feeds results + this task list to Claude, which works on one section at a time.

**Constraint**: Every change must pass `cargo test` and `cargo clippy --all-targets` with zero regressions.

**File size limit**: No hand-written `.rs` file may exceed 1500 lines. Split into submodules if over. Does **not** apply to generated files.

**Key files**:
- ExtensionStore + ExtData: `crates/core/model/src/extensions/mod.rs`
- MusicXML extension types: `crates/core/model/src/musicxml_ext/mod.rs`
- MusicXML import: `crates/formats/musicxml/src/import/`
- MusicXML export: `crates/formats/musicxml/src/export/`
- ConversionContext: `crates/formats/musicxml/src/context/mod.rs`
- MEI xml_compare: `crates/formats/mei/src/tests/xml_compare.rs`

---

## Phase 1: Restructure ExtensionStore

### 1.1 Per-concept HashMap fields

- [x] Add per-concept `HashMap<String, T>` fields to `ExtensionStore` for all existing MusicXML typed structs: `harmonies`, `barlines`, `sounds`, `prints`, `measure_styles`, `listenings`, `note_visuals`, `note_extras_map`, `stem_extras_map`, `direction_visuals`, `instruments`, `part_details_map`, `group_details_map`, `key_extras_map`, `time_extras_map`, `for_parts`, `staff_details_map`, `part_symbols`, `transposes`, `wedge_spreads`, `lyric_extras_map`
  - 21 HashMap fields added to ExtensionStore in extensions.rs
- [x] Add per-concept fields for LilyPond types: `format_origins`, `pitch_contexts`, `output_defs_map`, `book_structures`, `staff_contexts`, `repeat_infos`, `grace_infos`, `property_ops_map`, `function_calls`, `event_sequences`, `variable_assignments_map`, `toplevel_markups_map`, `lyrics_infos`, `chord_repetitions`, `context_changes`, `tweak_infos_map`, `pitched_rests`, `mrest_infos`, `drum_events`, `lyric_extenders`
  - 20 HashMap fields added (Vec values for output_defs_map, property_ops_map, tweak_infos_map, toplevel_markups_map)
- [x] Add singleton field: `score_header: Option<ScoreHeaderData>` (score-level, not keyed by element ID)
- [x] All new fields: `#[serde(default, skip_serializing_if = "HashMap::is_empty")]` (or `Option::is_none` for singletons)
- [x] Keep existing `data: HashMap<String, ExtData>` temporarily for backward compat
  - data field retained with #[serde(flatten)]
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 1.2 Bridge helpers on ConversionContext

- [x] Add typed convenience methods to `ExtensionStore` for insert/get on new maps (e.g., `pub fn harmony(&self, id: &str) -> Option<&HarmonyData>`, `pub fn insert_harmony(&mut self, id: String, data: HarmonyData)`)
  - 42 accessor pairs (get + insert) generated via `ext_store_accessors!` macro: 21 MusicXML + 21 LilyPond
  - Split extensions.rs into extensions/mod.rs (1206 lines) + extensions/tests.rs (554 lines) to stay under 1500
- [x] Tests pass
  - All 2500 tests pass, clippy clean

## Phase 2: Migrate existing typed fields (MusicXML)

### 2.1 Harmony migration

- [x] Import (`import/harmony.rs`): write to `ext_store.harmonies` instead of `ext_store.entry(id).harmony`
  - Uses `insert_harmony()` accessor directly; no more `entry()` + ExtData
- [x] Import: stop writing to `ext_store.entry(id).mxml_json`
  - Removed `entry.mxml_json = serde_json::to_value(...)` line
- [x] Import: stop setting `musicxml:harmony` label on `<harm>` element
  - Removed `harm.common.label = Some(HARM_LABEL_PREFIX.to_string())`
- [x] Export (`export/harmony.rs`): read from `ext_store.harmonies.get(id)` instead of `ext_store.get(id)?.mxml_json`
  - New `build_harmony_from_data()` reconstructs Harmony from HarmonyData (typed, no JSON deser)
- [x] Export: remove label-based identification; use `ext_store.harmonies.contains_key(id)` instead
  - Removed `harmony_from_label()` fallback path; only ExtensionStore + text fallback remain
- [x] Remove `harmony_from_label()` legacy function
  - Removed function + `HARM_LABEL_PREFIX` constant + legacy JSON roundtrip test
- [x] Tests pass
  - All 2501 tests pass, clippy clean

### 2.2 Barline migration

- [x] Import (`import/barline.rs`): write to `ext_store.barlines`
  - Uses `insert_barline()` accessor directly; no more `entry()` + ExtData
- [x] Import: stop writing mxml_json, stop setting `musicxml:barline` label
  - Removed label assignment and mxml_json write
- [x] Export (`export/content.rs`): read from `ext_store.barlines.get(id)`
  - New `build_barline_from_data()` reconstructs Barline from BarlineData (typed, no JSON deser)
  - Barline dir identification uses ExtensionStore membership instead of label prefix check
- [x] Remove `barline_from_label()` legacy function
  - Removed function + `BARLINE_LABEL_PREFIX` constant
- [x] Ending strip uses `barline_mut()` accessor for direct map mutation
  - Extended `ext_store_accessors!` macro to generate `_mut` methods via `paste` crate
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 2.3 Print migration

- [x] Import (`import/print.rs`): write to `ext_store.prints`
  - Uses `insert_print()` accessor directly; no more `entry()` + ExtData
- [x] Import: stop writing mxml_json, stop setting `musicxml:print` label on `<sb>`/`<pb>`
  - Removed label assignment, mxml_json write, and serde_json::to_value(print) call
- [x] Export (`export/print.rs`): read from `ext_store.prints.get(id)`
  - New `build_print_from_data()` reconstructs Print from PrintData (typed, no JSON deser)
  - Uses `ctx.ext_store().print(id)` accessor instead of `ext.mxml_json`
- [x] Remove `print_from_label()` legacy function
  - Removed function + `PRINT_LABEL_PREFIX` constant
  - Updated xml_compare sb/pb keying from @label to @xml:id
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 2.4 Sound migration

- [x] Import (`import/sound.rs`): write to `ext_store.sounds`
  - Uses `insert_sound()` accessor directly; no more `entry()` + ExtData
- [x] Import: stop writing mxml_json, stop setting `musicxml:sound` label
  - Removed label assignment, mxml_json write
  - Fixed `build_sound_data()` to use manual conversion for swing/midi_groups (serde roundtrip between mismatched types was lossy)
- [x] Export (`export/sound.rs`): read from `ext_store.sounds.get(id)`
  - New `build_sound_from_data()` reconstructs Sound from SoundData (typed, no JSON deser)
  - Manual `build_midi_group()` and `build_swing()` helpers for struct conversion
  - Sound dir identification uses ExtensionStore membership instead of label prefix check
- [x] Remove `sound_from_label()` legacy function
  - Removed function + `SOUND_LABEL_PREFIX` constant
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 2.5 MeasureStyle migration

- [x] Import (`import/measure_style.rs`): write to `ext_store.measure_styles`
  - Uses `insert_measure_style()` accessor directly; no more `entry()` + ExtData
- [x] Import: stop writing mxml_json, stop setting `musicxml:measure-style` label
  - Removed label assignment, mxml_json write, and serde_json::to_value(ms) call
- [x] Export (`export/measure_style.rs`): read from `ext_store.measure_styles.get(id)`
  - New `build_measure_style_from_data()` reconstructs MeasureStyle from MeasureStyleData (typed, no JSON deser)
  - Measure-style dir identification uses ExtensionStore membership instead of label prefix check
- [x] Remove `measure_style_from_label()` legacy function
  - Removed function + `MEASURE_STYLE_LABEL_PREFIX` constant
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 2.6 Listening migration

- [ ] Import (`import/listening.rs`): write to `ext_store.listenings`
- [ ] Import: stop writing mxml_json, stop setting `musicxml:listening/grouping/link/bookmark` labels
- [ ] Export (`export/listening.rs`): read from `ext_store.listenings.get(id)`
- [ ] Remove `listening_from_label()`, `grouping_from_label()`, `link_from_label()`, `bookmark_from_label()` legacy functions
- [ ] Tests pass

### 2.7 FiguredBass typed struct + migration

- [ ] Create `FiguredBassData` typed struct in `musicxml_ext/` (figures with prefix/number/suffix, extend, duration, offset, parentheses, font/position attrs)
- [ ] Add `figured_basses: HashMap<String, FiguredBassData>` to ExtensionStore
- [ ] Import (`import/figured_bass.rs`): build `FiguredBassData` and write to `ext_store.figured_basses`
- [ ] Import: stop writing mxml_json, stop setting `musicxml:figured-bass` label
- [ ] Export (`export/figured_bass.rs`): read from `ext_store.figured_basses.get(id)` and reconstruct MusicXML `FiguredBass`
- [ ] Remove `figured_bass_from_label()` legacy function
- [ ] Tests pass

### 2.8 Note visual + extras + stem migration

- [ ] Import (`import/note.rs`): write to `ext_store.note_visuals`, `ext_store.note_extras_map`, `ext_store.stem_extras_map` instead of ExtData entry
- [ ] Export (`export/note.rs`): read from new per-concept maps
- [ ] Tests pass

### 2.9 Direction visual + metronome + direction sound + wedge spread

- [ ] Import: write `direction_visuals`, `wedge_spreads` to per-concept maps
- [ ] Import (`import/direction.rs`): write metronome data to per-concept map (still as JSON string for now)
- [ ] Import: write direction sound data to per-concept map (still as JSON string for now)
- [ ] Export: read from per-concept maps
- [ ] Tests pass

### 2.10 Staff/Part/Group fields migration

- [ ] Import (`import/parts.rs`, `import/attributes.rs`): write `instruments`, `part_details`, `group_details`, `key_extras`, `time_extras`, `for_parts`, `staff_details`, `part_symbols`, `transposes`, `lyric_extras` to per-concept maps
- [ ] Export: read from per-concept maps
- [ ] Tests pass

### 2.11 Score header migration

- [ ] Import (`import/mod.rs`): write to `ext_store.score_header` singleton
- [ ] Export: read from `ext_store.score_header`
- [ ] Tests pass

## Phase 3: Eliminate mxml_json

### 3.1 Build MusicXML structs from typed data

- [ ] Harmony: implement `HarmonyData → Harmony` conversion in export (instead of deserializing mxml_json)
- [ ] Barline: implement `BarlineData → Barline` conversion
- [ ] Print: implement `PrintData → Print` conversion
- [ ] Sound: implement `SoundData → Sound` conversion
- [ ] MeasureStyle: implement `MeasureStyleData → MeasureStyle` conversion
- [ ] Listening: implement `ListeningData → MeasureContent` conversion
- [ ] FiguredBass: implement `FiguredBassData → FiguredBass` conversion
- [ ] Tests pass

### 3.2 Remove mxml_json

- [ ] Remove all `mxml_json` writes from import code
- [ ] Remove all `mxml_json` reads from export code
- [ ] Remove `mxml_json` field from `ExtData`
- [ ] Tests pass

## Phase 4: Direction types to ExtensionStore

### 4.1 Create DirectionContentData

- [ ] Create `DirectionContentData` enum in `musicxml_ext/` covering all 21 direction types: Rehearsal, Segno, Coda, Symbol, Dashes, Bracket, Pedal, OctaveShift, HarpPedals, Damp, DampAll, Eyeglasses, StringMute, Scordatura, Image, PrincipalVoice, Percussion, AccordionRegistration, StaffDivide, OtherDirection, Words
- [ ] Each variant holds the type-specific data needed for roundtrip (use existing MusicXML model types as serde_json::Value where complex, or create typed structs)
- [ ] Add `direction_contents: HashMap<String, DirectionContentData>` to ExtensionStore
- [ ] Tests pass (new struct tests)

### 4.2 Direction import migration

- [ ] Import (`import/direction.rs`): for each direction type, store `DirectionContentData` in `ext_store.direction_contents`
- [ ] Stop setting `musicxml:rehearsal`, `musicxml:segno`, `musicxml:coda`, etc. labels on `<dir>` elements
- [ ] Stop encoding direction data as text children of `<dir>` elements (currently stores JSON in dir text for some types)
- [ ] Tests pass

### 4.3 Direction export migration

- [ ] Export (`export/direction.rs`): read from `ext_store.direction_contents.get(id)` instead of checking label prefixes
- [ ] Remove the large `match label.as_deref()` block that dispatches on `musicxml:*` labels
- [ ] Tests pass

### 4.4 MetronomeData typed struct

- [ ] Create `MetronomeData` typed struct (replaces `metronome_json: Option<String>`)
- [ ] Add `metronomes: HashMap<String, MetronomeData>` to ExtensionStore
- [ ] Import: store typed MetronomeData
- [ ] Export: read MetronomeData and build MusicXML Metronome
- [ ] Tests pass

### 4.5 Direction sound typed

- [ ] Replace `direction_sound_json: Option<String>` with `direction_sounds: HashMap<String, SoundData>`
- [ ] Import: store typed SoundData for direction-level sounds
- [ ] Export: read and reconstruct
- [ ] Tests pass

## Phase 5: Ornament details to ExtensionStore

### 5.1 Create OrnamentDetailData

- [ ] Create `OrnamentDetailData` enum in `musicxml_ext/` covering: VerticalTurn, InvertedVerticalTurn, Shake, Schleifer, Haydn, UnmeasuredTremolo { type, value }, WavyLine { type, number }, OtherOrnament, OrnamentAccidentalMark { value, placement }, AccidentalMark { value, placement }, OtherNotation { type, number, smufl, text }, NonArpeggiate, Slide, NotationDynamics
- [ ] Add `ornament_details: HashMap<String, OrnamentDetailData>` to ExtensionStore
- [ ] Tests pass

### 5.2 Ornament import migration

- [ ] Import (`import/note.rs`): store `OrnamentDetailData` in `ext_store.ornament_details` for each ornament type
- [ ] Stop encoding ornament data in `<ornam>` labels (e.g., `musicxml:vertical-turn`, `musicxml:tremolo,type=...`)
- [ ] Stop encoding data in `<arpeg>` labels (e.g., `musicxml:non-arpeggiate`)
- [ ] Stop encoding data in `<gliss>` labels (e.g., `musicxml:slide`)
- [ ] Stop encoding data in `<dynam>` labels (e.g., `musicxml:notation-dynamics`)
- [ ] Tests pass

### 5.3 Ornament export migration

- [ ] Export (`export/content.rs`): read from `ext_store.ornament_details.get(id)` instead of parsing label strings
- [ ] Remove ornament label-parsing code
- [ ] Tests pass

## Phase 6: Technical details to ExtensionStore

### 6.1 Create TechnicalDetailData

- [ ] Create `TechnicalDetailData` enum in `musicxml_ext/` covering all ~25 technical types: Fingering { value, substitution, alternate }, Pluck { value, placement }, Fret { value }, StringNum { value, placement }, HammerOn { type, number, text }, PullOff { type, number, text }, Tap { hand, value }, Heel { substitution }, Toe { substitution }, Bend { alter, pre_bend, release, shape }, Hole { closed, location, type, shape }, Arrow { direction_or_circular, style, arrowhead }, Handbell { value }, HarmonMute { closed, location }, Harmonic { flags }, OpenString, ThumbPosition, DoubleTongue, TripleTongue, Fingernails, BrassBend, Flip, Smear, Golpe, Stopped { smufl }, Open { smufl }, HalfMuted { smufl }, SnapPizzicato, UpBow, DownBow, OtherTechnical { smufl, text }
- [ ] Add `technical_details: HashMap<String, TechnicalDetailData>` to ExtensionStore
- [ ] Tests pass

### 6.2 Technical import migration

- [ ] Import (`import/note.rs`): store `TechnicalDetailData` in `ext_store.technical_details`
- [ ] Stop encoding technical data in `<ornam>` labels and `<fing>` labels
- [ ] Tests pass

### 6.3 Technical export migration

- [ ] Export (`export/content.rs`): read from `ext_store.technical_details.get(id)`
- [ ] Remove technical label-parsing code
- [ ] Tests pass

## Phase 7: Note-level label elimination

### 7.1 Consolidate note label segments into ExtensionStore

- [ ] Ensure `NoteExtras` covers ALL note label segments: instruments, notehead, notehead-text, play, listen, footnote, level, notations-footnote, notations-level
- [ ] Import (`import/note.rs`): stop encoding `musicxml:instruments,`, `musicxml:notehead,`, `musicxml:notehead-text,`, `musicxml:play,`, `musicxml:listen,`, `musicxml:footnote,`, `musicxml:level,`, `musicxml:notations-footnote,`, `musicxml:notations-level,` in note labels
- [ ] Import: stop encoding `musicxml:visual,` in note labels (use `note_visuals` map)
- [ ] Import: stop encoding `musicxml:stem,` in note labels (use `stem_extras_map`)
- [ ] Tests pass

### 7.2 Note export from ExtensionStore only

- [ ] Export (`export/note.rs`): read ALL note-level data from ExtensionStore maps, remove label segment parsing
- [ ] Remove `append_note_label`, `has_label_segment`, `strip_label_segment` utilities for notes
- [ ] Tests pass

### 7.3 Articulation extras

- [ ] Import: move breath-mark/caesura/other-articulation JSON into NoteExtras or a new field
- [ ] Import: stop encoding `musicxml:breath-mark,`, `musicxml:caesura,`, `musicxml:other-articulation,`, `musicxml:tech-artic,` in note labels
- [ ] Export: read from ExtensionStore
- [ ] Tests pass

## Phase 8: StaffDef/StaffGrp label elimination

### 8.1 StaffDef label elimination

- [ ] Import (`import/parts.rs`, `import/attributes.rs`): stop writing pipe-separated `musicxml:key,`, `musicxml:time,`, `musicxml:transpose,`, `musicxml:for-part,`, `musicxml:staff-details,`, `musicxml:instrument,`, `musicxml:part-details,`, `musicxml:clef-jianpu` to staffDef labels
- [ ] Export (`export/parts.rs`, `export/attributes.rs`): read only from ExtensionStore maps
- [ ] Tests pass

### 8.2 StaffGrp label elimination

- [ ] Import: stop writing `musicxml:group-details,`, `musicxml:part-symbol,` to staffGrp labels
- [ ] Export: read only from ExtensionStore maps
- [ ] Tests pass

### 8.3 Remove label utilities

- [ ] Remove `append_label()`, `extract_label_segment()`, `strip_label_segment()` from import/attributes.rs or wherever defined
- [ ] Tests pass

## Phase 9: Header label elimination

### 9.1 Header metadata

- [ ] Import (`import/mod.rs`): stop writing `musicxml:identification,`, `musicxml:work,`, `musicxml:movement-number,`, `musicxml:movement-title,`, `musicxml:defaults,`, `musicxml:credits,` to extMeta/scoreDef labels
- [ ] Export: read from `ext_store.score_header`
- [ ] Tests pass

## Phase 10: Lyric label elimination

### 10.1 Lyric extras

- [ ] Import: stop writing `musicxml:lyric,` labels on `<verse>` elements
- [ ] Export (`export/note.rs`): read from `ext_store.lyric_extras_map.get(id)` only
- [ ] Tests pass

## Phase 11: xml_compare update

### 11.1 Remove label-based keying

- [ ] Update `xml_compare.rs` control event keying to not depend on `musicxml:` label prefixes for disambiguation
- [ ] Use element type + structural attributes (@startid, @endid, @tstamp, etc.) for keying instead
- [ ] Tests pass (all 97 MEI roundtrip tests)

## Phase 12: Remove ExtData

### 12.1 Migrate remaining LilyPond fields

- [ ] Move all LilyPond fields from ExtData to per-concept maps (format_origins, pitch_contexts, output_defs, book_structures, staff_contexts, repeat_infos, grace_infos, property_ops, function_calls, event_sequences, variable_assignments, toplevel_markups, lyrics_infos, chord_repetitions, context_changes, tweak_infos, pitched_rests, mrest_infos, drum_events, lyric_extenders)
- [ ] Update extension.rs unit tests
- [ ] Tests pass

### 12.2 Remove ExtData struct

- [ ] Remove `ExtData` struct from `extensions.rs`
- [ ] Remove `data: HashMap<String, ExtData>` from ExtensionStore
- [ ] Remove ExtensionStore methods that return ExtData (`get`, `get_mut`, `entry`, `insert` that take ExtData)
- [ ] Tests pass

## Phase 13: Final cleanup

### 13.1 Remove label constants and utilities

- [ ] Remove all `musicxml:` label prefix constants (HARM_LABEL_PREFIX, BARLINE_LABEL_PREFIX, PRINT_LABEL_PREFIX, SOUND_LABEL_PREFIX, LISTENING_LABEL_PREFIX, GROUPING_LABEL_PREFIX, LINK_LABEL_PREFIX, BOOKMARK_LABEL_PREFIX, MEASURE_STYLE_LABEL_PREFIX, FB_LABEL_PREFIX, MXML_DIR_LABEL_PREFIX, KEY_LABEL_PREFIX, TIME_LABEL_PREFIX, FOR_PART_LABEL_PREFIX, TRANSPOSE_LABEL_PREFIX, INSTRUMENT_LABEL_PREFIX, PART_DETAILS_LABEL_PREFIX, GROUP_DETAILS_LABEL_PREFIX, PART_SYMBOL_LABEL_PREFIX, STAFF_DETAILS_LABEL_PREFIX, IDENTIFICATION_LABEL_PREFIX, WORK_LABEL_PREFIX, MOVEMENT_NUMBER_LABEL_PREFIX, MOVEMENT_TITLE_LABEL_PREFIX, DEFAULTS_LABEL_PREFIX, CREDITS_LABEL_PREFIX)
- [ ] Remove all `*_from_label()` legacy parsing functions
- [ ] Remove JSON-in-label pipe escaping utilities (`escape_pipe`, `unescape_pipe` or similar)
- [ ] Tests pass

### 13.2 Verification

- [ ] Grep: zero `musicxml:` string literals in import/ and export/ code (except comments)
- [ ] Grep: zero `@label` writes with `musicxml:` content
- [ ] All 338 MusicXML roundtrip tests pass
- [ ] All 97 MEI roundtrip tests pass
- [ ] All unit + integration tests pass
- [ ] `cargo clippy --all-targets` clean

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

- [x] Import (`import/listening.rs`): write to `ext_store.listenings`
  - Uses `insert_listening()` accessor directly; no more `entry()` + ExtData
- [x] Import: stop writing mxml_json, stop setting `musicxml:listening/grouping/link/bookmark` labels
  - Removed all label assignments, mxml_json writes, and `.clone()` of JSON values
- [x] Export (`export/listening.rs`): read from `ext_store.listenings.get(id)`
  - Single `convert_mei_listening_dir()` function uses `ctx.ext_store().listening(id)` and matches on ListeningData variant
  - Dispatch in content.rs uses ExtensionStore membership instead of label prefix checks
- [x] Remove `listening_from_label()`, `grouping_from_label()`, `link_from_label()`, `bookmark_from_label()` legacy functions
  - Removed all 4 functions + 4 label prefix constants
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 2.7 FiguredBass typed struct + migration

- [x] Create `FiguredBassData` typed struct in `musicxml_ext/` (figures with prefix/number/suffix, extend, duration, offset, parentheses, font/position attrs)
  - FiguredBassData + FigureData + StyleTextData + FigureExtendData structs in musicxml_ext/mod.rs
- [x] Add `figured_basses: HashMap<String, FiguredBassData>` to ExtensionStore
  - HashMap field + `figured_bass_data`/`insert_figured_bass` accessors via macro
- [x] Import (`import/figured_bass.rs`): build `FiguredBassData` and write to `ext_store.figured_basses`
  - Uses `insert_figured_bass()` directly; no more `entry()` + ExtData
- [x] Import: stop writing mxml_json, stop setting `musicxml:figured-bass` label
  - Removed label assignment, mxml_json write, FB_LABEL_PREFIX constant
- [x] Export (`export/figured_bass.rs`): read from `ext_store.figured_basses.get(id)` and reconstruct MusicXML `FiguredBass`
  - New `build_figured_bass_from_data()` reconstructs FiguredBass from typed data (no JSON deser)
- [x] Remove `figured_bass_from_label()` legacy function
  - Removed function + legacy JSON roundtrip test
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 2.8 Note visual + extras + stem migration

- [x] Import (`import/note.rs`): write to `ext_store.note_visuals`, `ext_store.note_extras_map`, `ext_store.stem_extras_map` instead of ExtData entry
  - Uses `insert_stem_extras()`, `insert_note_visual()`, `insert_note_extras()`, `insert_lyric_extras()` directly; no more `entry()` + ExtData
- [x] Export (`export/note.rs`): read from new per-concept maps
  - Stem: uses `ctx.ext_store().stem_extras(id)` instead of `.get(id)?.stem_extras`
  - Articulations: uses `ctx.ext_store().note_extras(id)` instead of `.get(id)?.note_extras`
  - Instruments: uses `ctx.ext_store().note_extras(id)` instead of `.get(id)?.note_extras`
  - Note labels: uses `ctx.ext_store().note_extras(id)` and `ctx.ext_store().note_visual(id)`
  - Visual: new `apply_note_visual_data()` converts NoteVisualData directly (no JSON label roundtrip)
  - Lyrics: uses `ctx.ext_store().lyric_extras(&verse_key)` instead of `.get(&verse_key)?.lyric_extras`
- [x] Tests pass
  - All 2499 tests pass, clippy clean

### 2.9 Direction visual + metronome + direction sound + wedge spread

- [x] Import: write `direction_visuals`, `wedge_spreads` to per-concept maps
  - Uses `insert_direction_visual()` and `insert_wedge_spread()` accessors directly; no more `entry()` + ExtData
- [x] Import (`import/direction.rs`): write metronome data to per-concept map (still as JSON string for now)
  - Added `metronome_jsons: HashMap<String, String>` to ExtensionStore + `metronome_json_data`/`insert_metronome_json` accessors
  - Uses `insert_metronome_json()` directly; no more `entry()` + ExtData
- [x] Import: write direction sound data to per-concept map (still as JSON string for now)
  - Added `direction_sound_jsons: HashMap<String, String>` to ExtensionStore + `direction_sound_json_data`/`insert_direction_sound_json` accessors
  - Uses `insert_direction_sound_json()` directly; no more `entry()` + ExtData
- [x] Export: read from per-concept maps
  - Wedge spread: uses `ctx.ext_store().wedge_spread(id)` instead of `.get(id)?.wedge_stop_spread`
  - Direction visual: uses `ctx.ext_store().direction_visual(id)` instead of `.get(id)?.direction_visual`
  - Metronome: uses `ctx.ext_store().metronome_json_data(id)` instead of `.get(id)?.metronome_json`
  - Direction sound: uses `ctx.ext_store().direction_sound_json_data(id)` instead of `.get(id)?.direction_sound_json`
- [x] Tests pass
  - All 2499 tests pass, clippy clean

### 2.10 Staff/Part/Group fields migration

- [x] Import (`import/parts.rs`, `import/attributes.rs`): write `instruments`, `part_details`, `group_details`, `key_extras`, `time_extras`, `for_parts`, `staff_details`, `part_symbols`, `transposes`, `lyric_extras` to per-concept maps
  - Uses `insert_key_extras()`, `insert_time_extras()`, `insert_for_part()`, `insert_transpose()`, `insert_staff_details()`, `insert_part_details()`, `insert_group_details()`, `insert_part_symbol()`, `insert_instrument()` directly; no more `entry()` + ExtData
- [x] Export: read from per-concept maps
  - `extract_staff_details()`: uses `ctx.ext_store().staff_details(id)` instead of `.get(id)?.staff_details_extras`
  - `extract_key_from_label()`: uses `ctx.ext_store().key_extras(id)` instead of `.get(id)?.key_extras`
  - `extract_time_from_label()`: uses `ctx.ext_store().time_extras(id)` instead of `.get(id)?.time_extras`
  - `extract_for_parts_from_label()`: uses `ctx.ext_store().for_part(id)` instead of `.get(id)?.for_part`
  - `extract_transpose_from_ext()`: uses `ctx.ext_store().transpose(id)` instead of `.get(id)?.transpose`
  - `extract_instruments_from_staff_def()`: uses `ctx.ext_store().instrument(id)` with typed conversion; added `convert_ext_instrument_to_score_instrument()` and `convert_ext_midi_assignment()`
  - `extract_part_details_from_staff_def()`: uses `ctx.ext_store().part_details(id)` with serde_json conversion
  - `extract_group_details_from_staff_grp()`: uses `ctx.ext_store().group_details(id)` with serde_json conversion
  - `extract_part_symbol_from_staff_grp()`: uses `ctx.ext_store().part_symbol(id)` with typed conversion
- [x] Tests pass
  - All 2499 tests pass, clippy clean

### 2.11 Score header migration

- [x] Import (`import/mod.rs`): write to `ext_store.score_header` singleton
  - Uses `ext_store_mut().score_header = Some(header)` directly; no more `entry()` + ExtData
- [x] Export: read from `ext_store.score_header`
  - Uses `ext_store().score_header` singleton instead of `ext_store().get(head_id)?.score_header`
  - No longer needs meiHead @xml:id for ExtensionStore lookup
- [x] Tests pass
  - All 2499 tests pass, clippy clean

## Phase 3: Eliminate mxml_json

### 3.1 Build MusicXML structs from typed data

- [x] Harmony: implement `HarmonyData → Harmony` conversion in export (instead of deserializing mxml_json)
  - Already done in Phase 2.1: `build_harmony_from_data()` in export/harmony.rs
- [x] Barline: implement `BarlineData → Barline` conversion
  - Already done in Phase 2.2: `build_barline_from_data()` in export/content.rs
- [x] Print: implement `PrintData → Print` conversion
  - Already done in Phase 2.3: `build_print_from_data()` in export/print.rs
- [x] Sound: implement `SoundData → Sound` conversion
  - Already done in Phase 2.4: `build_sound_from_data()` in export/sound.rs
- [x] MeasureStyle: implement `MeasureStyleData → MeasureStyle` conversion
  - Already done in Phase 2.5: `build_measure_style_from_data()` in export/measure_style.rs
- [x] Listening: implement `ListeningData → MeasureContent` conversion
  - Already done in Phase 2.6: `convert_mei_listening_dir()` in export/listening.rs
- [x] FiguredBass: implement `FiguredBassData → FiguredBass` conversion
  - Already done in Phase 2.7: `build_figured_bass_from_data()` in export/figured_bass.rs
- [x] Tests pass
  - All 2499 tests pass, clippy clean

### 3.2 Remove mxml_json

- [x] Remove all `mxml_json` writes from import code
  - Already done in Phase 2: zero writes remain
- [x] Remove all `mxml_json` reads from export code
  - Already done in Phase 2: zero reads remain
- [x] Remove `mxml_json` field from `ExtData`
  - Removed field + doc comment + serde attribute from extensions/mod.rs
  - Cleaned up 4 stale mxml_json comments in import (harmony, barline, measure_style, ending)
- [x] Tests pass
  - All 2499 tests pass, clippy clean

## Phase 4: Direction types to ExtensionStore

### 4.1 Create DirectionContentData

- [x] Create `DirectionContentData` enum in `musicxml_ext/` covering all 21 direction types: Rehearsal, Segno, Coda, Symbol, Dashes, Bracket, Pedal, OctaveShift, HarpPedals, Damp, DampAll, Eyeglasses, StringMute, Scordatura, Image, PrincipalVoice, Percussion, AccordionRegistration, StaffDivide, OtherDirection, Words
  - 20-variant enum in musicxml_ext/mod.rs (Words excluded — uses native MEI dir); all variants use serde_json::Value for initial flexibility
- [x] Each variant holds the type-specific data needed for roundtrip (use existing MusicXML model types as serde_json::Value where complex, or create typed structs)
  - All variants use serde_json::Value — can be typed later; serde roundtrip verified in test
- [x] Add `direction_contents: HashMap<String, DirectionContentData>` to ExtensionStore
  - HashMap field + `direction_content`/`insert_direction_content` accessors via macro
- [x] Tests pass (new struct tests)
  - All 2500 tests pass, clippy clean

### 4.2 Direction import migration

- [x] Import (`import/direction.rs`): for each direction type, store `DirectionContentData` in `ext_store.direction_contents`
  - Replaced `dir_with_label()` with `dir_with_ext()` that stores typed data via `insert_direction_content()`
  - All 20 direction types serialize MusicXML model structs to `serde_json::Value` for storage
- [x] Stop setting `musicxml:rehearsal`, `musicxml:segno`, `musicxml:coda`, etc. labels on `<dir>` elements
  - Removed `MXML_DIR_LABEL_PREFIX` constant, `dir_with_label()`, `append_dir_label()`, and all label-setting code
  - Removed `musicxml:words-vis,` label from `convert_words()` (keeps ExtensionStore-only path)
  - Removed unused helpers: `dash_bracket_type_to_str`, `line_end_to_str`, `pedal_type_to_str`, `octave_shift_type_to_str`
- [x] Stop encoding direction data as text children of `<dir>` elements (currently stores JSON in dir text for some types)
  - `dir_with_ext()` creates bare dirs with no text children; data stored in ExtensionStore
- [x] Tests pass
  - Export updated with `build_direction_type_from_data()` to reconstruct from ExtensionStore (label fallback retained for backward compat)
  - Fixed `build_words_from_visual_data()` — proper typed conversion replacing broken serde roundtrip (WordsVisualData field names differ from Words)
  - Updated xml_compare `control_event_type_key` for "dir": uses @xml:id fallback when no label present
  - All 2500 tests pass, clippy clean

### 4.3 Direction export migration

- [x] Export (`export/direction.rs`): read from `ext_store.direction_contents.get(id)` instead of checking label prefixes
  - ExtensionStore path already primary; label fallback removed entirely
- [x] Remove the large `match label.as_deref()` block that dispatches on `musicxml:*` labels
  - Removed 20-arm match block for `musicxml:rehearsal`, `musicxml:segno`, etc.
  - Removed `musicxml:words-vis,` label fallback for Words visual data
  - Removed 4 helper functions only used by label dispatch: `parse_start_stop_continue`, `parse_bracket_payload`, `parse_pedal_type`, `parse_octave_shift_payload`
  - Cleaned up 14 unused imports (`StartStopContinue`, `Bracket`, `Coda`, `Dashes`, `HarpPedals`, `LineEnd`, `OctaveShift`, `OctaveShiftType`, `Pedal`, `PedalType`, `PrincipalVoice`, `Rehearsal`, `Segno`, `StaffDivide`, `Symbol`)
  - Updated xml_compare dir keying: removed label prefix fallback, uses @xml:id directly
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 4.4 MetronomeData typed struct

- [x] Create `MetronomeData` typed struct (replaces `metronome_json: Option<String>`)
  - MetronomeData + MetronomeContentData enum + BeatUnitTiedData + MetricModulationData + MetronomeNotesData + MetronomeNoteData + MetronomeBeamData + MetronomeTupletData in musicxml_ext/metronome.rs submodule
  - Covers all 3 content variants: BeatUnit (with tied), Modulation (metric modulation), Notes (metronome-note with arrows/beams/tuplets)
  - Visual attrs: parentheses, print_object, justify, default_x/y, halign, valign, id
- [x] Add `metronomes: HashMap<String, MetronomeData>` to ExtensionStore
  - Replaced `metronome_jsons: HashMap<String, String>` with typed map
  - Accessor pair: `metronome`/`insert_metronome` via macro
- [x] Import: store typed MetronomeData
  - `build_metronome_data()` in import/direction.rs converts MusicXML Metronome to MetronomeData
  - No more JSON serialization or pipe escaping
- [x] Export: read MetronomeData and build MusicXML Metronome
  - `build_metronome_from_data()` in export/direction.rs reconstructs Metronome from typed data
  - Direct typed conversion replaces JSON deserialization
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 4.5 Direction sound typed

- [x] Replace `direction_sound_json: Option<String>` with `direction_sounds: HashMap<String, SoundData>`
  - Replaced `direction_sound_jsons: HashMap<String, String>` with `direction_sounds: HashMap<String, SoundData>` in ExtensionStore
  - Updated accessor pair: `direction_sound`/`insert_direction_sound` via macro
- [x] Import: store typed SoundData for direction-level sounds
  - Uses existing `build_sound_data()` from import/sound.rs (already pub(crate))
  - No more JSON serialization or pipe escaping
- [x] Export: read and reconstruct
  - Made `build_sound_from_data()` in export/sound.rs `pub(crate)` for reuse
  - `restore_direction_sound()` reads typed SoundData and calls `build_sound_from_data()`
  - No more JSON deserialization or pipe unescaping
- [x] Tests pass
  - All 2500 tests pass, clippy clean

## Phase 5: Ornament details to ExtensionStore

### 5.1 Create OrnamentDetailData

- [x] Create `OrnamentDetailData` enum in `musicxml_ext/` covering: VerticalTurn, InvertedVerticalTurn, Shake, Schleifer, Haydn, UnmeasuredTremolo { type, value }, WavyLine { type, number }, OtherOrnament, OrnamentAccidentalMark { value, placement }, AccidentalMark { value, placement }, OtherNotation { type, number, smufl, text }, NonArpeggiate, Slide, NotationDynamics
  - 14-variant enum in musicxml_ext/ornament.rs submodule; placement-only types are unit variants, parameterized types use named fields with serde skip/rename
- [x] Add `ornament_details: HashMap<String, OrnamentDetailData>` to ExtensionStore
  - HashMap field + `ornament_detail`/`insert_ornament_detail` accessors via macro
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 5.2 Ornament import migration

- [x] Import (`import/note.rs`): store `OrnamentDetailData` in `ext_store.ornament_details` for each ornament type
  - All 12 ornam types: VerticalTurn, InvertedVerticalTurn, Shake, Schleifer, Haydn, UnmeasuredTremolo, WavyLine, OtherOrnament, OrnamentAccidentalMark + AccidentalMark, OtherNotation
  - Uses `insert_ornament_detail()` accessor directly; no more label assignment
- [x] Stop encoding ornament data in `<ornam>` labels (e.g., `musicxml:vertical-turn`, `musicxml:tremolo,type=...`)
  - Removed all 10 label assignments in `process_ornaments()`, `process_accidental_marks()`, `process_other_notations()`
- [x] Stop encoding data in `<arpeg>` labels (e.g., `musicxml:non-arpeggiate`)
  - Stores `OrnamentDetailData::NonArpeggiate` in ExtensionStore; removed `"musicxml:non-arpeggiate"` label
- [x] Stop encoding data in `<gliss>` labels (e.g., `musicxml:slide`)
  - Changed PendingGliss/CompletedGliss: `label: Option<String>` → `is_slide: bool`
  - `emit_gliss_events()` stores `OrnamentDetailData::Slide` in ExtensionStore; no more label on gliss element
- [x] Stop encoding data in `<dynam>` labels (e.g., `musicxml:notation-dynamics`)
  - Stores `OrnamentDetailData::NotationDynamics` in ExtensionStore; removed label assignment
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 5.3 Ornament export migration

- [x] Export (`export/content.rs`): read from `ext_store.ornament_details.get(id)` instead of parsing label strings
  - `convert_ornament_events()`: reads `ctx.ext_store().ornament_detail(ornam_id)` instead of parsing label strings
  - `convert_arpeg_events()`: uses `ctx.ext_store().ornament_detail(arpeg_id)` instead of `arpeg.common.label`
  - `convert_gliss_events()`: uses `ctx.ext_store().ornament_detail(gliss_id)` instead of `gliss.common.label`
  - `convert_notation_dynamics()`: uses `ctx.ext_store().ornament_detail(id)` instead of `dynam.common.label`
  - Main dynamics loop skip check also uses ExtensionStore instead of label
- [x] Remove ornament label-parsing code
  - Removed entire label-based if/else chain for 12 ornament types; replaced with match on OrnamentDetailData
  - Removed label-based `is_nonarp` and `is_slide` checks
  - Removed label-based notation-dynamics identification
- [x] Tests pass
  - All 2500 tests pass, clippy clean

## Phase 6: Technical details to ExtensionStore

### 6.1 Create TechnicalDetailData

- [x] Create `TechnicalDetailData` enum in `musicxml_ext/` covering all ~25 technical types: Fingering { value, substitution, alternate }, Pluck { value, placement }, Fret { value }, StringNum { value, placement }, HammerOn { type, number, text }, PullOff { type, number, text }, Tap { hand, value }, Heel { substitution }, Toe { substitution }, Bend { alter, pre_bend, release, shape }, Hole { closed, location, type, shape }, Arrow { direction_or_circular, style, arrowhead }, Handbell { value }, HarmonMute { closed, location }, Harmonic { flags }, OpenString, ThumbPosition, DoubleTongue, TripleTongue, Fingernails, BrassBend, Flip, Smear, Golpe, Stopped { smufl }, Open { smufl }, HalfMuted { smufl }, SnapPizzicato, UpBow, DownBow, OtherTechnical { smufl, text }
  - 31-variant TechnicalDetailData enum + ArrowContentData enum in musicxml_ext/technical.rs submodule
  - Covers all MusicXML technical types: 12 simple placement, 3 placement+smufl, 4 text content, 5 parameterized, 5 complex, 1 extensibility, 1 fingering (fing element), 1 tech-artic (note artic marker)
- [x] Add `technical_details: HashMap<String, TechnicalDetailData>` to ExtensionStore
  - HashMap field + `technical_detail`/`insert_technical_detail` accessors via macro
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 6.2 Technical import migration

- [x] Import (`import/note.rs`): store `TechnicalDetailData` in `ext_store.technical_details`
  - All 30 technical types stored via `insert_technical_detail()` on ornam/fing xml_id
  - Tech-artics (upbow, dnbow, snap, stop) stored in `NoteExtras.tech_artics` Vec keyed by note_id
  - Added `tech_artics: Vec<TechnicalDetailData>` field to NoteExtras in musicxml_ext/mod.rs
- [x] Stop encoding technical data in `<ornam>` labels and `<fing>` labels
  - Removed all `musicxml:` label assignments from `process_technical()`
  - Updated `emit!` macro: no label, generates xml_id, stores TechnicalDetailData in ExtensionStore
  - Fingering: no label, stores Fingering variant via `insert_technical_detail()` when sub/alt present
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 6.3 Technical export migration

- [x] Export (`export/content.rs`): read from `ext_store.technical_details.get(id)`
  - `convert_technical_events()` reads from `ctx.ext_store().technical_detail(ornam_id)` instead of label parsing
  - Fingering reads from `ctx.ext_store().technical_detail(fing_id)` for substitution/alternate attrs
  - Dispatch uses TechnicalDetailData match instead of string-based `is_technical_label()` + label parsing
- [x] Remove technical label-parsing code
  - Removed `is_technical_label()` function
  - Removed entire label-based dispatch block (400+ lines replaced with typed match)
  - `convert_mei_note_label_technical()` reads from `NoteExtras.tech_artics` instead of note label segments
  - Updated xml_compare: ornam/fing keying includes @xml:id for disambiguation (same pattern as dir)
- [x] Tests pass
  - All 2500 tests pass, clippy clean

## Phase 7: Note-level label elimination

### 7.1 Consolidate note label segments into ExtensionStore

- [x] Ensure `NoteExtras` covers ALL note label segments: instruments, notehead, notehead-text, play, listen, footnote, level, notations-footnote, notations-level
  - Already covered: NoteExtras has all fields + NoteVisualData and StemExtras in separate maps
- [x] Import (`import/note.rs`): stop encoding `musicxml:instruments,`, `musicxml:notehead,`, `musicxml:notehead-text,`, `musicxml:play,`, `musicxml:listen,`, `musicxml:footnote,`, `musicxml:level,`, `musicxml:notations-footnote,`, `musicxml:notations-level,` in note labels
  - Removed all 11 `append_note_label()` calls; data stored only in ExtensionStore via `populate_note_ext_store()`
- [x] Import: stop encoding `musicxml:visual,` in note labels (use `note_visuals` map)
  - Removed `musicxml:visual,{json}` label write from `convert_note_visual_attrs()`
- [x] Import: stop encoding `musicxml:stem,` in note labels (use `stem_extras_map`)
  - Removed `musicxml:stem,double/none` label writes; chord dedup uses `remove_stem_extras()` on ExtensionStore
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 7.2 Note export from ExtensionStore only

- [x] Export (`export/note.rs`): read ALL note-level data from ExtensionStore maps, remove label segment parsing
  - `restore_note_label_elements()`: removed label fallback, reads only from ExtensionStore
  - `convert_mei_note_label_instruments()`: removed label fallback, reads only from ExtensionStore
  - Stem (single + chord): removed `musicxml:stem,` label fallback, reads only from ExtensionStore
  - Removed dead `convert_mei_note_label_articulations()` function (breath-mark/caesura/other-artic label segments were never written to notes)
- [x] Remove `append_note_label`, `has_label_segment`, `strip_label_segment` utilities for notes
  - Removed all 3 functions from import/note.rs
  - Added `remove_*` method generation to `ext_store_accessors!` macro
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 7.3 Articulation extras

- [x] Import: move breath-mark/caesura/other-articulation JSON into NoteExtras or a new field
  - Breath-mark/caesura: value stored in ExtensionStore via `OrnamentDetailData::BreathMark`/`Caesura` variants (keyed by `<breath>`/`<caesura>` @xml:id)
  - Other-articulation: already in `NoteExtras.articulations` (full Articulations JSON, no label)
  - Tech-artic: already in `NoteExtras.tech_artics` (Phase 6.2)
- [x] Import: stop encoding `musicxml:breath-mark,`, `musicxml:caesura,`, `musicxml:other-articulation,`, `musicxml:tech-artic,` in note labels
  - Breath-mark/caesura: removed `@label` writes from `process_breath_marks()`/`process_caesuras()` — data stored in ExtensionStore only
  - Other-articulation/tech-artic: no label writes existed (already ExtensionStore-only)
- [x] Export: read from ExtensionStore
  - `convert_breath_events()`: reads value from `ctx.ext_store().ornament_detail(id)` instead of `breath.common.label`
  - `convert_caesura_events()`: reads value from `ctx.ext_store().ornament_detail(id)` instead of `caesura.common.label`
  - Updated xml_compare: breath/caesura type keys include @xml:id for disambiguation
- [x] Tests pass
  - All 2500 tests pass, clippy clean

## Phase 8: StaffDef/StaffGrp label elimination

### 8.1 StaffDef label elimination

- [x] Import (`import/parts.rs`, `import/attributes.rs`): stop writing pipe-separated `musicxml:key,`, `musicxml:time,`, `musicxml:transpose,`, `musicxml:for-part,`, `musicxml:staff-details,`, `musicxml:instrument,`, `musicxml:part-details,`, `musicxml:clef-jianpu` to staffDef labels
  - Removed all `append_label()` calls from import/attributes.rs and import/parts.rs
  - Removed `append_label()` function (no longer needed)
  - Removed 4 label prefix constants from attributes.rs: `KEY_LABEL_PREFIX`, `TIME_LABEL_PREFIX`, `FOR_PART_LABEL_PREFIX`, `TRANSPOSE_LABEL_PREFIX`
  - Removed 4 label prefix constants from parts.rs: `INSTRUMENT_LABEL_PREFIX`, `PART_DETAILS_LABEL_PREFIX`, `GROUP_DETAILS_LABEL_PREFIX`, `PART_SYMBOL_LABEL_PREFIX`
  - Removed `STAFF_DETAILS_LABEL_PREFIX` from parts.rs
  - Removed `PartExtraDetails`, `GroupExtraDetails`, `InstrumentData` (local label serialization structs)
  - Removed `append_group_label()` function from parts.rs
  - Jianpu clef: added `jianpu_clefs: HashSet<String>` to ExtensionStore; import stores staffDef IDs there
  - All instrDef @label writes removed; data stored only in ExtensionStore
  - All staffDef @label writes removed; data stored only in ExtensionStore
  - All staffGrp @label writes for group-details and part-symbol removed
- [x] Export (`export/parts.rs`, `export/attributes.rs`): read only from ExtensionStore maps
  - Removed all JSON-in-label fallback paths from export/attributes.rs: `extract_key_from_label` → `extract_key_from_ext`, `extract_time_from_label` → `extract_time_from_ext`, `extract_for_parts_from_label` → `extract_for_parts_from_ext`
  - Removed `STAFF_DETAILS_LABEL_PREFIX`, `CLEF_JIANPU_LABEL` constants from export/attributes.rs
  - Removed label fallback from `extract_transpose_from_ext` (keeps MEI attr fallback)
  - Removed label fallback from `extract_staff_details` (keeps @lines fallback)
  - `has_jianpu_clef_label()` → `has_jianpu_clef()` reads from ExtensionStore
  - Removed all `extract_label_segment` usage from export
  - export/parts.rs: removed label fallbacks from `extract_instruments_from_staff_def`, `extract_part_details_from_staff_def`, `extract_group_details_from_staff_grp`, `extract_part_symbol_from_staff_grp`
  - Removed `GROUP_DETAILS_LABEL_PREFIX` label check from `has_group_details_label` → now just checks ExtensionStore
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 8.2 StaffGrp label elimination

- [x] Import: stop writing `musicxml:group-details,`, `musicxml:part-symbol,` to staffGrp labels
  - Already done in Phase 8.1: all staffGrp label writes removed
- [x] Export: read only from ExtensionStore maps
  - Already done in Phase 8.1: export reads from ExtensionStore only
- [x] Tests pass
  - All 2500 tests pass, clippy clean

### 8.3 Remove label utilities

- [x] Remove `append_label()`, `extract_label_segment()`, `strip_label_segment()` from import/attributes.rs or wherever defined
  - Already done in Phase 8.1: all MusicXML label utilities removed (only LilyPond versions remain)
- [x] Tests pass
  - All 2500 tests pass, clippy clean

## Phase 9: Header label elimination

### 9.1 Header metadata

- [x] Import (`import/mod.rs`): stop writing `musicxml:identification,`, `musicxml:work,`, `musicxml:movement-number,`, `musicxml:movement-title,`, `musicxml:defaults,`, `musicxml:credits,` to extMeta/scoreDef labels
  - Import already writes to `ext_store.score_header` only (no extMeta elements written)
  - Removed 6 label prefix constants: `IDENTIFICATION_LABEL_PREFIX`, `WORK_LABEL_PREFIX`, `MOVEMENT_NUMBER_LABEL_PREFIX`, `MOVEMENT_TITLE_LABEL_PREFIX`, `DEFAULTS_LABEL_PREFIX`, `CREDITS_LABEL_PREFIX`
- [x] Export: read from `ext_store.score_header`
  - Removed `header_from_ext_meta()` legacy fallback function (scanned extMeta elements for JSON roundtrip data)
  - Export now reads only from `ext_store.score_header` singleton
- [x] Tests pass
  - All 2500 tests pass, clippy clean

## Phase 10: Lyric label elimination

### 10.1 Lyric extras

- [x] Import: stop writing `musicxml:lyric,` labels on `<verse>` elements
  - Removed label_parts building (name, justify, position, color, print-object, time-only, id, end-line, end-paragraph, extend, elision, extend-only, laughing, humming)
  - Removed `verse.common.label = Some("musicxml:lyric,...")` assignment
  - Removed unused `ssc_str()` helper function
  - ExtensionStore LyricExtras (written in `populate_note_ext_store`) is sole roundtrip path
- [x] Export (`export/note.rs`): read from `ext_store.lyric_extras_map.get(id)` only
  - Removed 170-line label fallback (label_val/label_has helpers, all MusicXML attr decoding, content type dispatch)
  - Removed `parse_ssc()` helper function
  - Structural MEI fallback retained for native MEI (verse/syl → lyric text, no MusicXML-specific attrs)
- [x] Tests pass
  - All 2500 tests pass, clippy clean

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

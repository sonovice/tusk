# MusicXML Coverage Plan

Comprehensive plan for achieving full MusicXML 4.1 ↔ MEI bidirectional conversion coverage. Each phase builds on the previous and is ordered by practical importance for real-world scores.

**Notation**: `[P]` = Parser, `[S]` = Serializer, `[I]` = Import (MusicXML→MEI), `[E]` = Export (MEI→MusicXML), `[T]` = Tests

---

## Current State

### What is covered

- **Structure**: `score-partwise`, `score-timewise` (auto-converted), `part-list`, `part`, `measure`, `backup`, `forward`
- **Notes**: `pitch`, `unpitched`, `rest` (incl. `measure="yes"` → `mRest`), `chord`, `grace`, `cue`, `stem`, `dots`, `duration`, `type`, `voice`, `staff`, `beam`
- **Accidentals**: written (all values incl. quarter-tone), gestural (alter → accid.ges), cautionary/editorial (parentheses, brackets)
- **Articulations**: all 17 standard types (accent, strong-accent, staccato, tenuto, detached-legato, staccatissimo, spiccato, scoop, plop, doit, falloff, breath-mark, caesura, stress, unstress, soft-accent, other-articulation)
- **Ties & Slurs**: `tie` (sound), `tied` (visual), `slur` (with cross-measure support, interval-graph numbering)
- **Directions**: `dynamics`, `wedge`/hairpin, `metronome`/tempo, `words`/dir, plus many types as generic `dir` with `musicxml:<type>` label for roundtrip (rehearsal, segno, coda, pedal, octave-shift, dashes, bracket, harp-pedals, damp, damp-all, eyeglasses, string-mute, scordatura, image, principal-voice, percussion, accordion-registration, staff-divide, other-direction)
- **Attributes**: `divisions`, `key` (fifths, mode), `time` (beats, beat-type, symbol common/cut), `clef` (sign, line, octave-change), `staves`, `instruments`, `transpose`
- **Barlines**: left/right barlines with bar-style, repeat, ending
- **Part groups**: `part-group` with symbol, barline-through, label, abbreviation → nested `staffGrp`
- **Header**: `work-title`, `movement-number`, `movement-title`, basic `identification`/`encoding`

### What is NOT covered

Everything below, organized into phases.

### Test status

- 310/311 MusicXML roundtrip tests pass (1 debug helper ignored)
- 97 MEI roundtrip tests ignored (pending MEI 5.1 deserializer alignment)

---

## Phase 1: Notations — Tuplets

**Priority**: Critical — tuplets are extremely common in real music.

Currently `time-modification` is parsed into the model but never converted to MEI `<tuplet>` containers. The `Notations` struct only holds `slurs`, `tied`, and `articulations`.

### 1.1 Model: Add Tuplet to Notations

- `[P]` Add `tuplet` field to `Notations` struct (the MusicXML `<tuplet>` notation element carries start/stop, actual-notes, normal-notes, bracket, show-number, show-type, placement)
- `[P]` Create `Tuplet` struct in `model/notations.rs` with: `tuplet_type` (start/stop), `number`, `bracket` (yes/no), `show_number` (actual/both/none), `show_type` (actual/both/none), `placement`, `actual_notes`, `normal_notes`, `actual_type`, `normal_type`
- `[P]` Parse `<tuplet>` inside `parse_notations()` — currently falls through to `skip_element`
- `[S]` Serialize `<tuplet>` in notations serializer

### 1.2 Model: Use TimeModification

- `[P]` `time-modification` is already parsed (`actual-notes`, `normal-notes`, `normal-type`, `normal-dot`) — verify completeness
- `[S]` `time-modification` serialization already exists — verify it emits `normal-dots` correctly

### 1.3 Import: MusicXML Tuplets → MEI

- `[I]` Detect tuplet boundaries from `<tuplet type="start">` / `<tuplet type="stop">` notations
- `[I]` Wrap affected notes/chords/rests in MEI `<tuplet>` container element
- `[I]` Map `time-modification` → MEI `@num` and `@numbase` on the tuplet element
- `[I]` Map `bracket`, `show-number`, `show-type` → MEI `@bracket.visible`, `@num.visible`, `@num.format`
- `[I]` Handle nested tuplets (tuplet number attribute distinguishes nesting levels)
- `[I]` Handle tuplets that span across beams

### 1.4 Export: MEI Tuplets → MusicXML

- `[E]` Detect MEI `<tuplet>` container in layer children
- `[E]` Emit `<time-modification>` on each note inside the tuplet
- `[E]` Emit `<tuplet type="start">` on first note, `<tuplet type="stop">` on last note
- `[E]` Map MEI `@num`/`@numbase` → `actual-notes`/`normal-notes`
- `[E]` Handle nested tuplets with proper numbering

### 1.5 Tests

- `[T]` Add roundtrip fixture: `tuplet_simple.musicxml` (3:2, 5:4, 6:4)
- `[T]` Add roundtrip fixture: `tuplet_nested.musicxml`
- `[T]` Add roundtrip fixture: `tuplet_across_beams.musicxml`
- `[T]` Verify existing fragment examples: `tuplet_element_regular`, `tuplet_element_nested`, `tuplet_dot_element`, `time_modification_element` now roundtrip fully

---

## Phase 2: Notations — Ornaments

**Priority**: High — trills, mordents, turns are common in classical and baroque music.

### 2.1 Model: Add Ornaments to Notations

- `[P]` Add `ornaments` field to `Notations` struct
- `[P]` Create `Ornaments` struct in `model/notations.rs` with all ornament types:
  - `trill_mark` (EmptyTrillSound: placement, start-note, trill-step, two-note-turn, accelerate, beats, second-beat, last-beat)
  - `turn`, `delayed_turn`, `inverted_turn`, `delayed_inverted_turn`, `vertical_turn`, `inverted_vertical_turn` (all: placement, start-note, trill-step, slash)
  - `shake` (placement, start-note, trill-step, etc.)
  - `mordent`, `inverted_mordent` (placement, long, approach, departure)
  - `wavy_line` (type start/stop/continue, number, placement)
  - `schleifer` (empty placement)
  - `tremolo` (type single/start/stop, value 1-8)
  - `haydn` (empty trill sound)
  - `other_ornament` (text content, placement)
  - `accidental_marks` (Vec of accidental marks within ornaments)
- `[P]` Parse all ornament types inside `parse_notations()` → `parse_ornaments()`
- `[S]` Serialize all ornament types

### 2.2 Import: MusicXML Ornaments → MEI

- `[I]` `trill-mark` → MEI `<trill>` control event with `@place`, `@startid`
- `[I]` `mordent` → MEI `<mordent>` with `@form="lower"`, `@long`
- `[I]` `inverted-mordent` → MEI `<mordent>` with `@form="upper"`
- `[I]` `turn` / `delayed-turn` → MEI `<turn>` with `@form="upper"`, `@delayed`
- `[I]` `inverted-turn` / `delayed-inverted-turn` → MEI `<turn>` with `@form="lower"`, `@delayed`
- `[I]` `vertical-turn` / `inverted-vertical-turn` → MEI `<turn>` with `@form` + vertical notation (may need label for roundtrip)
- `[I]` `shake` → MEI `<trill>` variant or `<ornam>` with label
- `[I]` `schleifer` → MEI `<ornam>` with `@label="schleifer"` or dedicated element
- `[I]` `tremolo type="single"` → MEI `<bTrem>` (bowed tremolo) container around note
- `[I]` `tremolo type="start/stop"` → MEI `<fTrem>` (fingered tremolo) container
- `[I]` `tremolo` value (number of beams) → MEI `@unitdur` or `@measperf`
- `[I]` `haydn` → MEI `<turn>` with `@form` or label
- `[I]` `wavy-line` → MEI trill extension line (link to `<trill>` via `@endid` or `@tstamp2`)
- `[I]` `accidental-mark` within ornaments → MEI `@accidlower`/`@accidupper` on the ornament

### 2.3 Export: MEI Ornaments → MusicXML

- `[E]` MEI `<trill>` → `trill-mark` (+ `wavy-line` if extended)
- `[E]` MEI `<mordent>` → `mordent` / `inverted-mordent` based on `@form`
- `[E]` MEI `<turn>` → `turn` / `inverted-turn` / `delayed-turn` / `delayed-inverted-turn` based on `@form` and `@delayed`
- `[E]` MEI `<bTrem>` → `tremolo type="single"` on contained note
- `[E]` MEI `<fTrem>` → `tremolo type="start/stop"` on contained notes

### 2.4 Tests

- `[T]` Add roundtrip fixtures: trills, mordents, turns, tremolos (single and double), wavy-line
- `[T]` Verify existing fragment examples: `trill_mark_element`, `mordent_element`, `inverted_mordent_element`, `turn_element`, `delayed_turn_element`, `inverted_turn_element`, `delayed_inverted_turn_element`, `vertical_turn_element`, `inverted_vertical_turn_element`, `shake_element`, `schleifer_element`, `tremolo_element_single`, `tremolo_element_double`, `haydn_element`, `wavy_line_element`

---

## Phase 3: Notations — Fermata, Arpeggiate, Glissando, Slide

**Priority**: High — fermatas are ubiquitous; arpeggiate/glissando common in piano and orchestral music.

### 3.1 Model: Add to Notations

- `[P]` Add `fermatas` (Vec — up to 2 per note), `arpeggiate`, `non_arpeggiate`, `glissandos`, `slides`, `accidental_marks`, `other_notations` to `Notations` struct
- `[P]` Create structs:
  - `Fermata`: shape (normal/angled/square/double-angled/double-square/double-dot/half-curve/curlew), type (upright/inverted), placement
  - `Arpeggiate`: number, direction (up/down), placement
  - `NonArpeggiate`: type (top/bottom), number, placement
  - `Glissando`: type (start/stop), number, line-type, text content, placement
  - `Slide`: type (start/stop), number, line-type, text content, placement
  - `AccidentalMark`: value, placement, parentheses
  - `OtherNotation`: type, text content, placement
- `[P]` Parse all in `parse_notations()`
- `[S]` Serialize all

### 3.2 Import

- `[I]` `fermata` → MEI `<fermata>` control event with `@shape`, `@form` (inv/norm based on type), `@place`, `@startid`
- `[I]` `arpeggiate` → MEI `<arpeg>` control event with `@order` (up/down)
- `[I]` `non-arpeggiate` → MEI `<arpeg>` with `@order="nonarp"`
- `[I]` `glissando` → MEI `<gliss>` control event with `@startid`/`@endid`, `@lform`
- `[I]` `slide` → MEI `<gliss>` with slide semantics (or separate treatment via label)
- `[I]` `accidental-mark` (standalone, not in ornaments) → MEI `<accid>` element or attribute

### 3.3 Export

- `[E]` MEI `<fermata>` → `fermata` notation with shape and type
- `[E]` MEI `<arpeg>` → `arpeggiate` or `non-arpeggiate` based on `@order`
- `[E]` MEI `<gliss>` → `glissando` or `slide`

### 3.4 Tests

- `[T]` Add roundtrip fixtures for each notation type
- `[T]` Verify fragment examples: `fermata_element`, `arpeggiate_element`, `non_arpeggiate_element`, `glissando_element_single`, `glissando_element_multiple`, `slide_element`, `accidental_mark_element_notation`

---

## Phase 4: Notations — Technical

**Priority**: Medium-High — important for string, wind, brass, and guitar music.

### 4.1 Model: Add Technical to Notations

- `[P]` Add `technical` field to `Notations` struct
- `[P]` Create `Technical` struct with all 30+ types, each as `Option<_>`:
  - **Strings**: `up_bow`, `down_bow`, `harmonic` (natural/artificial, base-pitch, touching-pitch, sounding-pitch), `open_string`, `thumb_position`, `fingering` (value, substitution, alternate, placement), `pluck`, `snap_pizzicato`, `stopped`
  - **Fretted**: `fret`, `string`, `hammer_on` (type, number, text), `pull_off` (type, number, text), `bend` (bend-alter, pre-bend, release, with-bar), `tap` (text, placement)
  - **Keyboard**: `heel` (substitution), `toe` (substitution)
  - **Wind/Brass**: `double_tongue`, `triple_tongue`, `fingernails`, `hole` (hole-type, hole-closed, hole-shape), `arrow`, `brass_bend`, `flip`, `smear`, `open`, `half_muted`, `harmon_mute` (harmon-closed), `golpe`
  - **General**: `handbell` (value), `other_technical` (text, placement)
- `[P]` Parse all in `parse_notations()` → `parse_technical()`
- `[S]` Serialize all

### 4.2 Import

- `[I]` Bowing marks (`up-bow`, `down-bow`) → MEI `<artic>` with bowing values or dedicated elements
- `[I]` `harmonic` → MEI note with `@harm` attribute or `<harm>` annotation
- `[I]` `fingering` → MEI `<fing>` element (already exists in MEI model)
- `[I]` `bend`, `hammer-on`, `pull-off` → MEI guitar/tablature elements or labels
- `[I]` For elements without direct MEI equivalents: store with `musicxml:` label for lossless roundtrip

### 4.3 Export

- `[E]` MEI `<fing>` → `fingering`
- `[E]` MEI bowing articulations → appropriate technical elements
- `[E]` Roundtrip elements from labels back to MusicXML technical

### 4.4 Tests

- `[T]` Add roundtrip fixtures for key technical notations
- `[T]` Verify fragment examples: `up_bow_element`, `down_bow_element`, `open_string_element`, `thumb_position_element`, `snap_pizzicato_element`, `stopped_element`, `double_tongue_element`, `triple_tongue_element`, `fingernails_element`, `pluck_element`, `tap_element`, `heel_element`, `toe_element`, `heel_toe_substitution`, `fingering_element_notation`, `bend_element`, `brass_bend_element`, `flip_element`, `smear_element`, `open_element`, `half_muted_element`, `harmon_mute_element`, `golpe_element`, `handbell_element`, `hole_element`, `hole_type_element`, `arrow_element`, `arrowhead_element`, `circular_arrow_element`, `pre_bend_element`, `with_bar_element`, `technical_element_tablature`

---

## Phase 5: Notations — Dynamics within Notations

**Priority**: Medium — dynamics can appear both as directions and as notation elements.

### 5.1 Model

- `[P]` Add `dynamics` field (Vec) to `Notations` struct — reuse existing `Dynamics`/`DynamicsValue` from `model/direction/dynamics.rs`
- `[P]` Parse `<dynamics>` within `parse_notations()` — currently only parsed within `<direction-type>`
- `[S]` Serialize dynamics within notations

### 5.2 Import & Export

- `[I]` Notation-level dynamics → MEI `<dynam>` with `@startid` referencing the note
- `[E]` MEI `<dynam>` attached to specific note → notation-level dynamics (vs. direction-level when position-based)
- Distinguish from direction-level dynamics by attachment method

### 5.3 Tests

- `[T]` Add roundtrip fixture with notation-level dynamics
- `[T]` Verify fragment examples with dynamics in notations context

---

## Phase 6: Lyrics

**Priority**: High — lyrics are essential for vocal music.

### 6.1 Model: Add Lyric Types

- `[P]` Create `model/lyric.rs` with:
  - `Lyric`: number, name, placement, justify, default-x/y, children (text syllables, elision, extend, laughing, humming, end-line, end-paragraph, editorial)
  - `Syllabic`: enum (single, begin, middle, end)
  - `LyricText`: text content with font/color attributes
  - `Elision`: text content, font attributes
  - `Extend`: type (start/stop/continue)
- `[P]` Add `lyrics: Vec<Lyric>` field to `Note` struct
- `[P]` Parse `<lyric>` within `parse_note()` — currently falls through to `skip_element`
- `[S]` Serialize all lyric elements

### 6.2 Import: MusicXML Lyrics → MEI

- `[I]` MusicXML `<lyric>` on notes → MEI `<syl>` children on `<note>` elements
- `[I]` Map `<syllabic>` (single/begin/middle/end) → MEI `@wordpos` and `@con` attributes
- `[I]` Map `<text>` content → MEI `<syl>` text
- `[I]` Map `<elision>` → MEI elision handling (separate `<syl>` or within same)
- `[I]` Map lyric `number` → MEI `<verse>` `@n` for multi-verse support
- `[I]` Map `<extend>` → MEI extender line (underscore continuation)
- `[I]` Handle `<humming>` and `<laughing>` special syllable types

### 6.3 Export: MEI Lyrics → MusicXML

- `[E]` MEI `<syl>` children on notes → MusicXML `<lyric>` elements
- `[E]` MEI `@wordpos`/`@con` → `<syllabic>`
- `[E]` MEI `<verse>` `@n` → lyric `number`
- `[E]` MEI extender lines → `<extend>`

### 6.4 Tests

- `[T]` Add roundtrip fixture: `lyrics_simple.musicxml` (single verse)
- `[T]` Add roundtrip fixture: `lyrics_multiverse.musicxml` (multiple verses)
- `[T]` Add roundtrip fixture: `lyrics_elision.musicxml`
- `[T]` Verify fragment examples: `lyric_element`, `syllabic_element`, `elision_element`, `extend_element_lyric`, `end_line_element`, `end_paragraph_element`, `humming_element`, `laughing_element`

---

## Phase 7: Harmony & Chord Symbols

**Priority**: High — chord symbols are essential for jazz, pop, and lead-sheet music.

### 7.1 Model: Add Harmony

- `[P]` Add `Harmony` variant to `MeasureContent` enum
- `[P]` Create `model/harmony.rs` with:
  - `Harmony`: type (explicit/implied/alternate), print-object, print-frame, placement, staff, offset, children
  - `HarmonyChord`: root, function, numeral, kind, inversion, bass, degree(s)
  - `Root`: root-step, root-alter
  - `Bass`: bass-step, bass-alter, bass-separator
  - `Kind`: value (major, minor, augmented, diminished, dominant, major-seventh, etc.), text, use-symbols, parentheses-degrees, bracket-degrees, stack-degrees
  - `Degree`: degree-value, degree-alter, degree-type (add/alter/subtract)
  - `Numeral`: numeral-root, numeral-alter
  - `Frame`: strings, frets, first-fret, frame-note(s)
  - `FrameNote`: string, fret, fingering, barre
- `[P]` Parse `<harmony>` in `parse_measure()` — currently falls through to `skip_element`
- `[S]` Serialize all harmony elements

### 7.2 Import: MusicXML Harmony → MEI

- `[I]` `harmony` → MEI `<harm>` control event with text content
- `[I]` `root` + `kind` → MEI harm text (e.g. "Cmaj7") or structured `<chordDef>` reference
- `[I]` `bass` → MEI slash notation in harm
- `[I]` `degree` alterations → MEI harm extensions
- `[I]` `frame` → MEI `<chordDef>` with `<chordMember>` for fretboard diagrams
- `[I]` `function` (Nashville/Roman numeral) → MEI `<harm>` with function text
- `[I]` Placement, offset, staff mapping

### 7.3 Export: MEI Harmony → MusicXML

- `[E]` MEI `<harm>` → MusicXML `<harmony>` with parsed root/kind/bass
- `[E]` MEI `<chordDef>` → MusicXML `<frame>`
- `[E]` Handle text-only harm (no structured decomposition available)

### 7.4 Tests

- `[T]` Add roundtrip fixtures for chord symbols, Roman numerals, Nashville numbers
- `[T]` Verify fragment examples: `kind_element`, `root_step_element`, `root_alter_element`, `bass_step_element`, `bass_alter_element`, `bass_separator_element`, `degree_value_element`, `degree_alter_element`, `degree_type_element`, `inversion_element`, `numeral_root_element`, `numeral_alter_element`, `numeral_key_element`

---

## Phase 8: Figured Bass

**Priority**: Medium — important for Baroque music and continuo parts.

### 8.1 Model

- `[P]` Add `FiguredBass` variant to `MeasureContent` enum
- `[P]` Create `model/figured_bass.rs` with:
  - `FiguredBass`: duration, parentheses, figures, editorial-voice-direction, staff
  - `Figure`: prefix, figure-number, suffix, extend
- `[P]` Parse `<figured-bass>` in `parse_measure()`
- `[S]` Serialize all

### 8.2 Import & Export

- `[I]` `figured-bass` → MEI `<fb>` with `<f>` children
- `[I]` `figure-number` → MEI `<f>` text content
- `[I]` `prefix`/`suffix` → MEI accidental attributes on `<f>`
- `[I]` `extend` → MEI `@extender` on `<f>`
- `[E]` Reverse mapping MEI `<fb>` → MusicXML `<figured-bass>`

### 8.3 Tests

- `[T]` Add roundtrip fixture for figured bass
- `[T]` Verify fragment examples: `figure_number_element`, `prefix_element`, `suffix_element`, `extend_element_figure`

---

## Phase 9: Header & Metadata Completion

**Priority**: Medium-High — important for cataloging, attribution, and professional use.

### 9.1 Identification Completion

Currently only basic encoding info is converted. Full `<identification>` support:

- `[P]` `creator` elements are already in the model — verify parsing of type attribute (composer, lyricist, arranger, etc.)
- `[I]` `creator type="composer"` → MEI `<meiHead>/<fileDesc>/<titleStmt>/<composer>/<persName>`
- `[I]` `creator type="lyricist"` → MEI `<titleStmt>/<lyricist>/<persName>`
- `[I]` `creator type="arranger"` → MEI `<titleStmt>/<arranger>/<persName>`
- `[I]` `rights` → MEI `<pubStmt>/<availability>/<useRestrict>`
- `[I]` `source` → MEI `<sourceDesc>/<source>`
- `[I]` `relation` → MEI `<relationList>/<relation>`
- `[I]` `encoding` → MEI `<encodingDesc>/<appInfo>/<application>` (software, date, supports, description)
- `[I]` `miscellaneous` → MEI `<notesStmt>/<annot>`
- `[E]` Reverse all mappings

### 9.2 Work Element Completion

- `[I]` `work-number` → MEI `<workList>/<work>/<identifier>`
- `[I]` `opus` (xlink:href) → MEI `<workList>/<work>/<identifier type="opus">`
- `[E]` Reverse mappings

### 9.3 Tests

- `[T]` Add roundtrip fixture with rich metadata
- `[T]` Verify fragment examples: `footnote_element`

---

## Phase 10: Defaults, Layout & Appearance

**Priority**: Medium — important for engraving fidelity, less so for data interchange.

### 10.1 Model Completion

The model already has `Defaults`, `Scaling`, `PageLayout`, `SystemLayout`, `StaffLayout`, `Appearance` structs. They are parsed but neither converted to MEI nor serialized completely.

- `[P]` Verify parsing completeness for all `<defaults>` children
- `[S]` Complete `Defaults` serialization (TODO at line 289 in `serializer/score.rs`: "appearance, fonts, etc.")
- `[S]` Serialize `appearance` children: `line-width`, `note-size`, `distance`, `glyph`, `other-appearance`
- `[S]` Serialize font elements: `music-font`, `word-font`, `lyric-font`, `lyric-language`
- `[S]` Serialize `scaling`: `millimeters`, `tenths`

### 10.2 Import: Defaults → MEI

- `[I]` `scaling` → MEI `<scoreDef>` `@vu.height` or `<staffDef>` spacing attributes
- `[I]` `page-layout` → MEI `<scoreDef>` `@page.height`, `@page.width`, `@page.topmar`, `@page.botmar`, `@page.leftmar`, `@page.rightmar`
- `[I]` `system-layout` → MEI `@system.leftmar`, `@system.rightmar`, `@spacing.system`
- `[I]` `staff-layout` → MEI `@spacing.staff`
- `[I]` `appearance` → MEI visual attributes or annotation
- `[I]` Font info → MEI `@fontfam`, `@fontsize`, `@fontstyle`, `@fontweight` on relevant elements
- `[I]` `concert-score` → MEI `@trans.diat`/`@trans.semi` handling

### 10.3 Export

- `[E]` Reverse layout attribute mappings
- `[E]` Note: many MEI visual attributes have no direct MusicXML equivalent (lossy)

### 10.4 Tests

- `[T]` Add roundtrip fixture with full layout information
- `[T]` Verify fragment examples: `measure_distance_element`, `staff_distance_element`, `system_distance_element`, `staff_size_element`, `line_detail_element`, `line_element`, `measure_numbering_element`, `system_dividers_element`, `glyph_element`

---

## Phase 11: Credits

**Priority**: Medium — important for title pages and printed scores.

### 11.1 Model & Parsing

- `[P]` `Credit` is already in the model — verify parsing of all `credit-words` attributes (justify, valign, font, default-x/y, halign, enclosure)
- `[P]` Parse `credit-image` and `credit-symbol` (currently may be skipped)

### 11.2 Import & Export

- `[I]` `credit` → MEI `<pgHead>` / `<pgFoot>` content with `<rend>` elements for formatting
- `[I]` `credit-words` with `justify="center"` + `valign="top"` at top → likely title in `<pgHead>`
- `[I]` `credit-words` with other positions → appropriate `<rend>` positioning
- `[I]` `credit-image` → MEI `<graphic>` within `<pgHead>`/`<pgFoot>`
- `[E]` MEI `<pgHead>`/`<pgFoot>` → MusicXML `<credit>` elements

### 11.3 Tests

- `[T]` Add roundtrip fixture with title page credits
- `[T]` Verify fragment examples: `image_element`

---

## Phase 12: Print Element

**Priority**: Medium — controls page/system breaks and measure layout.

### 12.1 Model

- `[P]` Add `Print` variant to `MeasureContent` enum
- `[P]` Create `model/print.rs` with:
  - `Print`: staff-spacing, new-system, new-page, blank-page, page-number, children (page-layout, system-layout, staff-layout(s), measure-layout, measure-numbering, part-name-display, part-abbreviation-display)
- `[P]` Parse `<print>` in `parse_measure()` — currently falls through to `skip_element`
- `[S]` Serialize all

### 12.2 Import & Export

- `[I]` `new-system="yes"` → MEI `<sb>` (system break) element
- `[I]` `new-page="yes"` → MEI `<pb>` (page break) element
- `[I]` `staff-spacing` → MEI `@spacing.staff` on measure-level override
- `[I]` Inline `page-layout`, `system-layout`, `staff-layout` → MEI `<scoreDef>` overrides within the score
- `[I]` `measure-numbering` → MEI `@mnum.visible`
- `[E]` MEI `<sb>` → `<print new-system="yes">`
- `[E]` MEI `<pb>` → `<print new-page="yes">`

### 12.3 Tests

- `[T]` Add roundtrip fixture with system/page breaks
- `[T]` Verify fragment examples: `system_attribute_only_top`, `system_attribute_also_top`, `staff_lines_element`, `staff_type_element`, `staves_element`

---

## Phase 13: Sound Element (Standalone)

**Priority**: Medium — important for playback but not for visual engraving.

Currently `<sound>` is only parsed when nested inside `<direction>`. It can also appear as a standalone measure-level element.

### 13.1 Model

- `[P]` Add `Sound` variant to `MeasureContent` enum
- `[P]` Expand `Sound` struct to include all attributes: tempo, dynamics, dacapo, segno, dalsegno, coda, tocoda, divisions, forward-repeat, fine, time-only, pizzicato, pan, elevation, damper-pedal, soft-pedal, sostenuto-pedal
- `[P]` Parse children: `instrument-change`, `midi-device`, `midi-instrument`, `play`, `swing`, `offset`
- `[P]` Parse standalone `<sound>` in `parse_measure()` — currently falls through to `skip_element`
- `[S]` Serialize standalone sound elements

### 13.2 Import & Export

- `[I]` Playback-only sound elements → MEI `<tempo>` or annotation with playback attributes
- `[I]` Repeat-related sound (dacapo, segno, dalsegno, coda, tocoda, fine, forward-repeat) → MEI repeat structure or `<dir>` with labels
- `[I]` MIDI attributes → MEI `<midi>` elements or annotation
- `[I]` `swing` → MEI `<dir>` with swing label or dedicated element
- `[E]` Reverse mappings where MEI equivalents exist

### 13.3 Tests

- `[T]` Add roundtrip fixture for standalone sound
- `[T]` Verify fragment examples: `swing_element`, `pan_and_elevation_elements`

---

## Phase 14: Multi-Staff Parts

**Priority**: High — critical for piano, organ, harp, and other multi-staff instruments.

This is a known TODO in `import/parts.rs:299`.

### 14.1 Import

- `[I]` Detect `<staves>` element in `<attributes>` to determine multi-staff parts
- `[I]` Create multiple `<staffDef>` elements within a single `<staffGrp>` for multi-staff parts
- `[I]` Route notes to correct `<staff>` based on `<staff>` child in notes
- `[I]` Ensure clefs, key signatures, and time signatures are propagated to all staves in the part
- `[I]` Handle cross-staff notation (`staff` element on notes placing them on a different staff than default)
- `[I]` Handle `backup`/`forward` across staves within a part

### 14.2 Export

- `[E]` Detect multi-`<staffDef>` within a single part `<staffGrp>`
- `[E]` Emit `<staves>` in `<attributes>`
- `[E]` Route notes from multiple `<staff>` elements into the same `<part>` with proper `<staff>` tags
- `[E]` Manage `<backup>` for cross-staff voice movement

### 14.3 Tests

- `[T]` Add roundtrip fixture: `piano_two_staves.musicxml`
- `[T]` Add roundtrip fixture: `organ_three_staves.musicxml`
- `[T]` Add roundtrip fixture: `cross_staff_notes.musicxml`

---

## Phase 15: Advanced Attributes

**Priority**: Medium — completes attribute coverage for edge cases.

### 15.1 Staff Details

- `[P]` `staff-details` already in model — ensure full parsing (staff-type, staff-lines, line-detail, staff-tuning, capo, staff-size)
- `[I]` `staff-type` → MEI `@lines` + notation type
- `[I]` `staff-lines` → MEI `@lines`
- `[I]` `staff-tuning` → MEI `<tuning>` with `<course>` children
- `[I]` `capo` → MEI `<tuning>` capo attribute
- `[I]` `staff-size` → MEI `@scale`
- `[E]` Reverse mappings
- `[T]` Verify fragment examples: `staff_tuning_element`, `capo_element`, `staff_lines_element`, `staff_size_element`

### 15.2 Part Symbol

- `[P]` `part-symbol` already in model — integrate into conversion
- `[I]` `part-symbol` → MEI `<staffGrp>` `@symbol`
- `[E]` MEI `@symbol` → `part-symbol`
- `[T]` Verify fragment example: `part_symbol_element`

### 15.3 Measure Style

- `[P]` `measure-style` already in model — integrate into conversion
- `[I]` `multiple-rest` → MEI `<multiRest>` element
- `[I]` `measure-repeat` → MEI `<mRpt>` / `<mRpt2>`
- `[I]` `beat-repeat` → MEI `<beatRpt>`
- `[I]` `slash` → MEI slash notation
- `[E]` Reverse mappings
- `[T]` Verify fragment examples: `multiple_rest_element`, `measure_repeat_element`, `beat_repeat_element`, `slash_element`, `slash_type_and_slash_dot_elements`

### 15.4 Non-Traditional Key Signatures

- `[P]` `NonTraditionalKey` already in model — integrate into conversion
- `[I]` Non-traditional key → MEI `@keysig` with explicit accidentals via `<keyAccid>` children
- `[E]` MEI `<keyAccid>` → non-traditional key elements
- `[T]` Verify fragment examples: `key_element_non_traditional`, `key_octave_element`

### 15.5 Interchangeable Time Signatures

- `[P]` `Interchangeable` already in model — integrate into conversion
- `[I]` Interchangeable time → MEI `<meterSigGrp>` with multiple `<meterSig>`
- `[E]` Reverse
- `[T]` Verify fragment example: `interchangeable_element`

### 15.6 Directive

- `[P]` Parse `<directive>` within `<attributes>`
- `[I]` Directive → MEI `<dir>` with appropriate placement
- `[E]` Reverse

---

## Phase 16: Barline Completion

**Priority**: Medium — completes barline support for repeat structures.

### 16.1 Barline Children

Currently only `bar-style`, `repeat`, and `ending` are converted. Missing:

- `[P]` `fermata` within barline (up to 2) — model may need extension
- `[P]` `wavy-line` within barline
- `[P]` `segno` within barline
- `[P]` `coda` within barline
- `[P]` `editorial` (footnote, level) within barline
- `[S]` Serialize these children

### 16.2 Import & Export

- `[I]` Barline `fermata` → MEI `<fermata>` control event attached to the barline
- `[I]` Barline `segno`/`coda` → MEI repeat marks
- `[I]` Barline `wavy-line` → MEI trill continuation at barline
- `[E]` Reverse mappings

### 16.3 Tests

- `[T]` Add roundtrip fixture for decorated barlines
- `[T]` Verify fragment examples: `barline_element`, `repeat_element`, `ending_element`, `segno_element`, `coda_element`

---

## Phase 17: Score Instruments & MIDI

**Priority**: Low-Medium — important for playback-oriented workflows.

### 17.1 Score Part Instruments

- `[P]` `score-instrument` is parsed — extend conversion
- `[P]` `midi-device`, `midi-instrument` parsed — extend conversion
- `[I]` `score-instrument` → MEI `<instrDef>` with `@midi.instrname`
- `[I]` `midi-instrument` → MEI `<instrDef>` `@midi.channel`, `@midi.instrnum`, `@midi.volume`, `@midi.pan`
- `[I]` `instrument-sound` → MEI `<instrDef>` `@label` or sound reference
- `[I]` `virtual-instrument` → MEI annotation
- `[E]` Reverse mappings

### 17.2 Note-Level Instrument

- `[P]` `<instrument>` child of `<note>` — currently parsed as empty Vec
- `[I]` Map to MEI note-level instrument reference
- `[E]` Reverse

### 17.3 Tests

- `[T]` Verify fragment examples: `midi_device_element`, `midi_instrument_element`, `midi_name_and_midi_bank_elements`, `midi_unpitched_element`, `virtual_instrument_element`, `ensemble_element`, `instrument_link_element`, `instrument_change_element`

---

## Phase 18: Part/Score Details

**Priority**: Low-Medium — completes part metadata.

### 18.1 Part Name Display

- `[P]` `part-name-display` / `part-abbreviation-display` in model — parse `<display-text>` and `<accidental-text>` children
- `[I]` Display name → MEI `<label>` with `<rend>` formatting
- `[E]` Reverse

### 18.2 Group Details

- `[P]` `group-name-display`, `group-abbreviation-display`, `group-time` — complete parsing
- `[I]` Group display → MEI `<staffGrp>` `<label>` with formatting
- `[I]` `group-time` → MEI time signature propagation in group
- `[E]` Reverse

### 18.3 Player Elements

- `[P]` `<player>` elements within `<score-part>` — parse and model
- `[I]` → MEI performer metadata
- `[E]` Reverse

### 18.4 Tests

- `[T]` Verify fragment examples: `part_name_display_element`, `part_abbreviation_display_element`, `group_name_display_element`, `group_abbreviation_display_element`, `group_time_element`, `part_link_element`

---

## Phase 19: Remaining Measure-Level Elements

**Priority**: Low — edge cases and specialized elements.

### 19.1 Listening

- `[P]` Add `Listening` variant to `MeasureContent`
- `[P]` Model: `Listening` → `sync` (type, latency, player, time-only), `other-listening`, `offset`
- `[I]` → MEI annotation or ignore (no direct MEI equivalent)
- `[T]` Verify fragment examples: `sync_element`, `wait_element`, `assess_and_player_elements`

### 19.2 Grouping

- `[P]` Add `Grouping` variant to `MeasureContent`
- `[P]` Model: `Grouping` → type (start/stop/single), member-of, number, features
- `[I]` → MEI `<expansion>` or annotation
- `[T]` Verify fragment example: `grouping_element`

### 19.3 Link & Bookmark

- `[P]` Add `Link` / `Bookmark` variants to `MeasureContent`
- `[P]` Model: `Link` → xlink attributes; `Bookmark` → id, name, position
- `[I]` → MEI `<ptr>`/`<ref>` or annotation
- `[T]` Verify fragment examples: `link_element`, `bookmark_element`

---

## Phase 20: Note-Level Completion

**Priority**: Medium — fills in remaining note details.

### 20.1 Notehead

- `[P]` `notehead` is parsed but not converted
- `[I]` `notehead` value (diamond, triangle, slash, etc.) → MEI `@head.shape`
- `[I]` `notehead@filled` → MEI `@head.fill`
- `[I]` `notehead@parentheses` → MEI parentheses rendering
- `[E]` Reverse
- `[T]` Verify fragment example: `notehead_text_element`

### 20.2 Play & Listen

- `[P]` `<play>` child of note → IPA pronunciation, mute, semi-pitched, other-play
- `[P]` `<listen>` child of note → assess, wait, other-listen
- `[I]` → MEI annotation or gestural attributes
- `[E]` Reverse where possible
- `[T]` Verify fragment examples: `ipa_element`

### 20.3 Editorial (Footnote & Level)

- `[P]` `footnote` and `level` are in the Note model but always set to None
- `[P]` Parse these elements
- `[I]` `footnote` → MEI `<annot>`
- `[I]` `level` → MEI editorial attributes
- `[E]` Reverse
- `[T]` Verify fragment example: `level_element`

---

## Phase 21: Direction Serialization Completion

**Priority**: Medium — currently many direction types roundtrip via generic labels but are not fully structured in the serializer.

### 21.1 Structured Direction Serialization

Address the TODO at `serializer/score.rs:1833`: "implement other direction types".

Currently all non-dynamics/wedge/metronome/words direction types are exported as generic `<words>` when no `musicxml:<type>` label is present. Add full serialization for:

- `[S]` `Rehearsal` → `<rehearsal>` with enclosure
- `[S]` `Segno` → `<segno>`
- `[S]` `Coda` → `<coda>`
- `[S]` `Symbol` → `<symbol>`
- `[S]` `Bracket` → `<bracket>` with type, line-end, etc.
- `[S]` `Dashes` → `<dashes>` with type, number
- `[S]` `Pedal` → `<pedal>` with type, line, sign
- `[S]` `OctaveShift` → `<octave-shift>` with type, size, number
- `[S]` `HarpPedals` → `<harp-pedals>` with pedal-tuning children
- `[S]` `Damp`, `DampAll`, `Eyeglasses` → respective elements
- `[S]` `StringMute` → `<string-mute>` with type
- `[S]` `Scordatura` → `<scordatura>` with accord children
- `[S]` `PrincipalVoice` → `<principal-voice>` with type, symbol
- `[S]` `Percussion` → `<percussion>` subtypes
- `[S]` `AccordionRegistration` → `<accordion-registration>`
- `[S]` `StaffDivide` → `<staff-divide>` with type
- `[S]` `Image` → `<image>` with source, type
- `[S]` `OtherDirection` → `<other-direction>`

### 21.2 Tests

- `[T]` Verify all direction type fragment examples serialize correctly without falling back to `<words>`

---

## Phase 22: Compressed MusicXML (.mxl)

**Priority**: Medium-High — many real-world files are distributed as .mxl.

### 22.1 Implementation

- Add `zip` crate dependency
- `[P]` Read `.mxl` archive → locate `META-INF/container.xml` → find rootfile → extract and parse MusicXML
- `[S]` Write `.mxl` archive → create `META-INF/container.xml` → compress MusicXML
- Handle multiple rootfiles (unusual but spec-valid)
- Handle accompanying files (images, sound files) in archive

### 22.2 Tests

- `[T]` Add .mxl roundtrip tests
- `[T]` Test with real-world .mxl files

---

## Phase 23: Visual & Position Attributes

**Priority**: Low — important only for pixel-perfect engraving reproduction.

### 23.1 Position Attributes

Many elements support `default-x`, `default-y`, `relative-x`, `relative-y` which are parsed in some cases but never converted:

- `[I]` Position attributes on notes → MEI `@ho`, `@vo` (horizontal/vertical offset)
- `[I]` Position attributes on directions → MEI placement offsets
- `[E]` Reverse where MEI carries position info

### 23.2 Font & Color Attributes

- `[I]` `font-family`, `font-size`, `font-style`, `font-weight` → MEI `@fontfam`, `@fontsize`, `@fontstyle`, `@fontweight`
- `[I]` `color` → MEI `@color`
- `[I]` `enclosure` → MEI `@enclose`
- `[E]` Reverse

### 23.3 Print Attributes on Elements

- `[I]` `print-object="no"` → MEI `@visible="false"`
- `[I]` `print-leger` → MEI ledger line visibility
- `[I]` `print-spacing` → MEI spacing behavior
- `[E]` Reverse

---

## Phase 24: Remaining Edge Cases & Polish

### 24.1 Additional Note Attributes

- `[I]` `end-dynamics` → MEI gestural dynamics endpoint
- `[I]` `attack` / `release` → MEI `@tstamp.ges` offset or gestural attributes
- `[I]` `pizzicato` → MEI `@pizz`

### 24.2 Stem Extensions

- `[I]` `StemValue::Double` → MEI `@stem.dir="up"` + `@stem.dir="down"` (or label)
- `[I]` `StemValue::None` → MEI `@stem.visible="false"`

### 24.3 Jianpu Clef

- `[I]` `ClefSign::Jianpu` → MEI numbered notation clef (currently mapped to G)

### 24.4 Concert Score / Transposition

- `[I]` `concert-score` element → MEI `@trans.diat`/`@trans.semi` handling
- `[I]` `for-part` with `part-clef` / `part-transpose` → MEI per-part transposition
- `[E]` Reverse
- `[T]` Verify fragment example: `concert_score_and_for_part_elements`

---

## Phase 25: Version Compatibility

### 25.1 MusicXML Version Detection & Upgrade

- Detect version from XML DOCTYPE or `version` attribute
- MusicXML 2.0 → 4.1 migration (element renames, new features)
- MusicXML 3.0 → 4.1 migration
- MusicXML 3.1 → 4.1 migration
- MusicXML 4.0 → 4.1 migration
- Version-specific export option (e.g. export as MusicXML 3.1 for broader compatibility)

### 25.2 Tests

- Cross-version roundtrip tests
- Test with real-world files from different MusicXML versions

---

## Summary: Priority Order for Implementation

| Priority | Phase | Description | Impact |
|----------|-------|-------------|--------|
| Critical | 1 | Tuplets | Every non-trivial score |
| Critical | 14 | Multi-staff parts | Piano, organ, harp |
| High | 2 | Ornaments | Classical & baroque music |
| High | 3 | Fermata, arpeggiate, glissando | Nearly all genres |
| High | 6 | Lyrics | All vocal music |
| High | 7 | Harmony & chord symbols | Jazz, pop, lead sheets |
| High | 22 | .mxl compressed format | Real-world file distribution |
| Med-High | 9 | Header/metadata completion | Professional use, cataloging |
| Medium | 4 | Technical notations | String, wind, guitar music |
| Medium | 5 | Dynamics in notations | Edge case placement |
| Medium | 8 | Figured bass | Baroque continuo |
| Medium | 10 | Defaults, layout, appearance | Engraving fidelity |
| Medium | 11 | Credits | Title pages |
| Medium | 12 | Print element | Page/system breaks |
| Medium | 13 | Standalone sound | Playback |
| Medium | 15 | Advanced attributes | Completeness |
| Medium | 16 | Barline completion | Repeat decorations |
| Medium | 20 | Note-level completion | Noteheads, editorial |
| Medium | 21 | Direction serializer completion | Clean export |
| Low-Med | 17 | Instruments & MIDI | Playback workflows |
| Low-Med | 18 | Part/score details | Part metadata |
| Low | 19 | Listening, grouping, link, bookmark | Rare elements |
| Low | 23 | Visual & position attributes | Pixel-perfect layout |
| Low | 24 | Edge cases & polish | Completeness |
| Low | 25 | Version compatibility | Legacy files |

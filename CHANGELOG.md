# Changelog

## [1.2.0] — 2026-03-01

### LilyPond parser

- **Function signature database**: 200+ known LilyPond functions with expected
  trailing music arg counts (0, 1, or 2). Unknown functions no longer greedily
  consume `{ }` / `<< >>` blocks, fixing serializer roundtrip inconsistencies.
- **Scheme fraction parsing**: `#N/M` and `#-N/M` now parse as `SchemeExpr::Fraction`
  instead of consuming the slash as a separate token.
- **Markup `\override` handling**: `\override` inside `\markup` context is now
  correctly recognized as a markup command despite being lexed as a keyword token.
- **Lyric markup support**: `\markup` expressions inside `\lyricmode` are now
  parsed as `LyricMarkup` events instead of causing parse errors.
- **Lyric post-event adjacency**: `--` (hyphen) and `__` (extender) now require
  adjacent tokens (no whitespace), preventing false matches with separate dashes
  and underscores.
- **Lyric backtracking**: single `-` or `_` in lyrics now backtracks correctly
  via lookahead instead of losing the consumed token.

### LilyPond serializer

- **Lyric string quoting**: lyric text containing non-alphabetic characters
  (parentheses, slashes, digits) is now quoted to ensure re-parse stability.

### LilyPond import (LilyPond → MEI)

- **Voice name preservation**: `find_inner_voice_name` now searches
  `staff.original_music` for named Voice contexts that were unwrapped by
  `extract_voices`, preserving `\context Voice = "name"` through MEI roundtrip.
- **DrumMode voice splitting**: `\drummode { << { } { } >> }` now correctly
  splits into separate voices.
- **Nested lyricsto attachment**: `\lyricsto` inside nested `<< >>` blocks now
  correctly attaches to the matching voice.
- **Inner voice name in StaffContext**: `inner_voice_name` field added to
  `StaffContext` extension for roundtrip preservation.

### LilyPond export (MEI → LilyPond)

- **Relative pitch in simultaneous blocks**: each voice in `<< >>` now resets
  to the entering reference pitch, matching LilyPond's `\relative` semantics.
  Fixes systematic octave drift in multi-voice passages.
- **Idempotent melisma injection**: `\set melismaBusyProperties` is no longer
  duplicated on re-export; detects existing annotations before inserting.
- **LyricsTo multi-verse**: `\lyricsto` now supports multiple verses via
  `count` field, generating separate `\new Lyrics \lyricsto` blocks per verse.
- **Voice name roundtrip**: stored `inner_voice_name` and `voice_id` from
  `LyricsTo` are restored on export instead of generating synthetic names.

### MEI codegen

- **Extra local attributes**: 8 element-specific attributes (`application/@version`,
  `div/@type`, `title/@type`, etc.) injected via `EXTRA_ATTRS` table.
- **Extra text content**: `plateNum` and `useRestrict` now support text children.
- **Expanded child elements**: 15+ additional parent→child relationships added
  to `EXTRA_CHILDREN` for header elements (`changeDesc`, `samplingDecl`,
  `editorialDecl`, `projectDesc`, `manifestationList`, `notesStmt`, etc.).

### MusicXML parser

- **Self-closing `<unpitched/>`**: `Event::Empty` handler for `<unpitched/>`
  so percussion notes with no children parse correctly.
- **Self-closing `<measure/>`**: empty measures (`<measure number="X"/>`)
  now parse instead of being silently skipped.

### MusicXML import (MusicXML → MEI)

- **Multi-staff part detection**: early-return in `convert_mei_staff_grp_to_part_list`
  when a staffGrp is itself a multi-staff part, fixing mis-split piano/organ
  staves from native MEI files.
- **Part ID collision avoidance**: auto-generated `P{n}` part IDs checked
  against pre-scanned `xml:id` values to avoid collisions with native MEI IDs.
- **`print-object="yes"` on redundant key/clef**: inline keySig/clef with
  `@visible="true"` preserves forced-display semantics through MEI roundtrip.

### MusicXML export (MEI → MusicXML)

- **`print-object="yes"` restoration**: inline keySig/clef with `@visible="true"`
  emits `print-object="yes"` on the MusicXML key/clef element.
- **Irrefutable pattern cleanup**: replaced `let X = expr;` with
  `let X = expr else { continue/return };` across export code to handle
  new MEI child enum variants without panicking.

### MEI deserializer

- **MEI 3.0 `barthru` fallback**: `bar.thru` attribute falls back to reading
  `barthru` (no dot) for MEI 3.0 and earlier files.

### Test infrastructure

- **Centralized roundtrip tests**: moved per-crate roundtrip tests into
  `tests/roundtrip/` workspace crate with `build.rs`-generated per-file tests.
- **Regression test suite**: 5933 LilyPond regression tests (serializer + MEI
  pipeline) generated from LilyPond repo fixtures.
- **Cross-format tests**: `musicxml_via_mei`, `mei_via_musicxml`,
  `lilypond_via_mei`, etc. with per-file generated tests.

## [1.1.0] — 2026-02-28

### LilyPond export (MEI → LilyPond)

- **Full-fidelity LilyPond export pipeline**: MusicXML → MEI → LilyPond with
  chord symbols, lyrics, tuplets, slurs, and pretty-printing.
- **Chord symbols**: MusicXML `<harmony>` → LilyPond `\new ChordNames` with
  `\chordmode`. Supports degree modifications, bass notes, jazz chord name
  preamble with `majorSevenSymbol` and `chordNameExceptions`.
- **Lyrics**: multi-measure lyric extraction, rest-to-note forwarding, verse ID
  tracking, auto-detection from MEI layers, trailing skip stripping.
- **Slur/phrase handling**: global slur pre-pass for cross-measure slurs,
  chord-aware post-event matching for chord child note IDs.
- **Pretty-printer**: `serialize_pretty()` with structural newlines, indentation,
  blank lines between contexts, `\with` block awareness.

### LilyPond parser/serializer

- **Figured bass**: serialize `<>` not `\<\>` for figure groups; parser accepts
  both forms.
- **Chord mode**: dot separator only between consecutive steps (`c:dim7` not
  `c:dim.7`).
- **Render tests**: 51 fixtures validated against actual LilyPond binary.

### MusicXML import

- **Tuplet rest support**: rests now process tuplet start/stop annotations.
- **Chord tuplet deduplication**: `fixup_tuplet_ids_for_chord()` replaces
  child note IDs with chord ID in pending/completed tuplets.
- **Chord ID generation**: fresh chord `xml:id` instead of stealing first
  note's ID.
- **ID preservation**: notes, rests, measure rests, and measures preserve
  original MusicXML IDs.
- **Multi-staff attribute placement**: inline clef/key/time changes tracked by
  beat position; per-staff clef filtering; key/time only on staff 1.

### MusicXML export

- **Tuplet export**: `resolve_chord_to_first_note_id()` maps chord IDs back to
  first note IDs for MusicXML tuplet notations.
- **Multi-block attribute comparison**: collects all attribute blocks per measure.

## [1.0.3] — 2026-02-26

### MusicXML ↔ MEI

- **Barlines/repeats**: repeat barlines → `@left="rptstart"` / `@right="rptend"`
  on MEI measures; export generates MusicXML `<repeat>` elements.
- **Barline style mapping**: `LightHeavy` → `End`, `HeavyLight` → `Heavy`.
- **Barline extras**: keyed by `measure:location`, no `<dir>` carriers.
- **Endings**: `@lendsym="angledown"` (stop) / `"none"` (discontinue); export
  derives `stop_type` from `@lendsym`.

## [1.0.2] — 2026-02-25

### MusicXML export

- **Harmony text**: render degree modifications (`add9`, `no3`, etc.) in
  MusicXML `<harmony>` output.

## [1.0.1] — 2026-02-25

### MusicXML ↔ MEI

- **Voice splitting**: split voices into separate MEI layers per staff.
- **Forward elements**: MusicXML `<forward>` → MEI `<space>` for voice-specific
  gaps.
- **Multi-staff export**: backup between layers, voice numbering from
  layer/staff position.
- **StaffGrp collapse**: collapse redundant single-child `<staffGrp>` wrappers.
- **Context utilities**: `local_staff_for_global()`, `current_part_id()`.

## [1.0.0] — 2026-02-20

Initial release.

- **MusicXML ↔ MEI bidirectional conversion** with high-fidelity roundtrip.
- **CLI**: `tusk convert` with `--from`/`--to` flags, stdin/stdout, `.mxl`
  support.
- **Formats**: MusicXML 2.0–4.1, MEI 5.x, compressed MusicXML (`.mxl`).
- **Coverage**: notes, rests, chords, beams, tuplets, slurs, ties, dynamics,
  ornaments, articulations, fermatas, arpeggios, glissandi, lyrics, harmony,
  figured bass, header/metadata, defaults, credits, print elements, sound,
  staff details, part symbols, measure styles, non-traditional keys,
  interchangeable time, barline children, score instruments/MIDI, part name
  display, group details, listening/grouping/link/bookmark, note-level
  completion, direction serialization, visual/position attributes,
  stem/clef/transposition edge cases, version compatibility.
- **Extension store**: typed `ExtensionStore` for lossless LilyPond-specific
  metadata roundtrip (replaces JSON-in-label approach).
- **MEI codegen**: `tusk-mei-codegen` generates Rust types from MEI RNG schema.

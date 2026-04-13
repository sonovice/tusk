# Changelog

## [1.3.11] — 2026-04-13

### LilyPond parser

- **Negative number function arguments**: `\ottava -1` and similar
  constructs now parse correctly.  Previously the `-` was not recognised
  as part of a negative integer in function argument position, so
  `\ottava -1` (8vb) was silently misparsed.
- **`\ottava` music arg count fixed (1 → 0)**: `\ottava` takes only an
  integer argument — no trailing music argument.  The parser previously
  consumed the first note after `\ottava` as a music argument, wrapping
  it inside the function scope.  This caused the note to be imported at
  the wrong octave (one octave lower) and broke cross-measure ottava
  spans where subsequent measures' notes were not transposed.

### LilyPond export (MEI → LilyPond)

- **Ottava pitch transposition (written → sounding)**: notes inside
  `\ottava` spans are now transposed from written pitch (as stored in
  MEI after MusicXML import) to sounding pitch (as LilyPond expects).
  LilyPond's `\ottava N` shifts the display by −N octaves, so providing
  written pitch caused notes to render one octave too low.  The ottava
  level is tracked across measure boundaries per staff.

## [1.3.10] — 2026-04-13

### MusicXML import (MusicXML → MEI)

- **Gestural accidental always set from `<alter>`**: `accid.ges` is now
  derived from `<alter>` even when a visual `<accidental>` element is
  present.  Previously the `is_none()` guard skipped `accid.ges` for notes
  with explicit accidentals, causing sharps/flats to be silently dropped
  (e.g. G#4 imported as G4 natural in the Chopin Waltz tuplet passages).
- **Natural alter skipped**: `<alter>0</alter>` no longer produces
  `accid.ges="n"` — natural is the default and emitting it caused MEI
  roundtrip instability.
- **Octave `@startid` resolved from tstamp**: `<octave>` elements now
  get a `@startid` pointing to the note at the ottava start position,
  resolved via `dur_ppq`-based beat calculation.  Previously only
  `@tstamp` was set, which Verovio resolved incorrectly for notes inside
  tuplets due to floating-point beat position mismatches.
- **Beam child duration uses `dur_ppq`**: beat calculation for notes
  inside beams now uses `dur_ppq / quarter_ppq` (actual sounding duration)
  instead of the notated `dur` value.  Notated duration doesn't reflect
  tuplet scaling, causing `apply_octave_spans` to transpose the wrong
  notes (e.g. notes before the ottava start were incorrectly transposed).
- **`estimate_measure_quarter_ppq` picks maximum**: the quarter-note PPQ
  estimation now collects all estimates and picks the maximum, avoiding
  tuplet notes that give artificially low values.

### LilyPond export (MEI → LilyPond)

- **Fingering direction defaults to Up**: fingerings without explicit
  `@place` now export as `^1` (above) instead of `-1` (neutral).
  Neutral caused LilyPond to place fingerings on the stem side, leading
  to inconsistent distances from noteheads.
- **Final `\sustainOff` dropped**: the last `\sustainOff` in a stream
  (no following `\sustainOn`) is removed so the pedal bracket extends
  to the final barline instead of ending at the last note's attack.

## [1.3.9] — 2026-04-12

### LilyPond export (MEI → LilyPond)

- **No more duplicate barlines around grace notes**: replaced explicit
  `\bar "|"` with `|` barchecks — explicit barlines created duplicate
  barline glyphs in PianoStaff when measures started with grace notes,
  visually bracketing them in their own mini-bar.
- **Pickup partial duration excludes spacer**: `\partial` duration in pickup
  measures no longer includes the spacer voice, fixing `\partial 1` → correct
  `\partial 4` (or appropriate duration) for anacrusis bars.
- **Ottava tstamp→note resolution**: MusicXML `<octave-shift>` directions
  now resolve `@tstamp`/`@tstamp2` to note IDs across measures, emitting
  `\ottava 1`/`\ottava 0` pairs. Previously silently dropped.
- **Tstamp resolution uses "note sounding at" strategy**: tstamp→note
  resolution now finds the last note at or before the target beat instead
  of requiring exact match, handling mid-beat direction offsets.
- **Final and special barlines emitted**: MEI `@right` barline attributes
  (`end`, `dbl`, `rptstart`, `rptend`, etc.) now produce correct LilyPond
  `\bar` commands for MusicXML-origin content.  MEI-origin content skips
  right barline emission because the LilyPond import does not reconstruct
  `measure_log.right`, which caused spacer-stripping instability.
- **Pedal retake consolidation**: consecutive `\sustainOff` → `\sustainOn`
  across measure boundaries are consolidated onto the same note, producing
  seamless bracket retakes instead of gaps.  Only for MusicXML-origin.
- **Barline-tstamp pedal stops preserved**: `\sustainOff` events whose
  tstamp falls after the last note are now attached to the last note
  (previously silently dropped), enabling the consolidation pass.

### LilyPond import (LilyPond → MEI)

- **Post-event pedal commands recognized**: `-\sustainOn`, `-\sustainOff`,
  `-\unaCorda`, `-\treCorde` written as post-event articulations are now
  correctly imported as MEI `<pedal>` elements.  Previously they were
  silently treated as articulation `<dir>` elements, causing pedal loss
  in roundtrips.

## [1.3.8] — 2026-04-12

### LilyPond export (MEI → LilyPond)

- **Pedal tstamp→note resolution**: MusicXML pedal directions that only
  carry `@tstamp` (no `@startid`) are now resolved to the nearest note at
  that beat position, emitting correct `\sustainOn`/`\sustainOff` etc.
  Previously these were silently dropped because the LilyPond export
  required `@startid` which MusicXML direction imports never set.
- **Pedal as post-event**: pedal commands are now emitted as post-events on
  notes (`c4\sustainOn`) instead of pre-note functions (`\sustainOn c4`),
  matching LilyPond semantics and eliminating "Unattached SustainEvent"
  warnings.
- **Grace notes excluded from tstamp map**: grace notes no longer participate
  in tstamp→note resolution, preventing pedal/ottava events from attaching
  to grace notes instead of the main note at that beat.

## [1.3.7] — 2026-04-11

### MusicXML import (MusicXML → MEI)

- **Ottava spans now preserve written MEI pitch correctly**: imported
  MusicXML `<octave-shift>` directions continue to become real MEI `<octave>`
  control events, and notes covered by the span are now also rewritten to the
  proper written octave in MEI instead of remaining a sounding octave too high.
- **Imported notes and chords now carry MEI `@tstamp`**: MusicXML note/chord
  events now record their beat position directly during import, which makes
  downstream control-event alignment deterministic and fixes ottava span
  application across mixed rhythmic material.
- **Pedal directions import as semantic MEI pedal events**: sustain pedal
  start/stop markers now import as real `<pedal>` elements, including the stop
  event, instead of degrading into generic directives.
- **Ottava placement normalized for Verovio and roundtrip**: downward
  octave-shift brackets now import as `dis.place="above"` and roundtrip back to
  MusicXML as `type="down"` with stable start/stop pairing.

### MusicXML export (MEI → MusicXML)

- **Pedal stop export preserved**: MEI pedal stop events now export back to
  MusicXML as proper stop pedals with `line="yes"` and `sign="no"`.
- **Ottava start/stop roundtrip stabilized**: MEI `<octave>` spans with
  `@tstamp2` or `@startid/@endid` now export back to paired MusicXML
  `<octave-shift>` start/stop directions with stable numbering, and exported
  MusicXML pitches are transposed under those spans so reimport can recover the
  written MEI pitch.

### Tests

- **Ottava/Pedal regression coverage expanded**: new unit and roundtrip-facing
  tests now cover Ottava pitch preservation, stop-boundary handling, backup
  voice positioning, and semantic pedal import/export behavior in the MusicXML
  crate.

### LilyPond import/export (LilyPond ↔ MEI)

- **Ottava commands now roundtrip as native MEI octave spans**: LilyPond
  `\ottava` changes now import as real MEI `<octave>` elements instead of
  opaque function-call directives, and MEI octave spans export back to stable
  LilyPond ottava start/stop commands across measure boundaries.
- **Pedal commands now roundtrip as native MEI pedal events**: LilyPond
  `\sustainOn`, `\sustainOff`, `\sostenutoOn`, `\sostenutoOff`, `\unaCorda`,
  and `\treCorde` now import as semantic MEI `<pedal>` events and export back
  to LilyPond without relying on `ExtensionStore`.
- **Cross-measure ottava export stabilized**: MEI-origin ottava spans are now
  collected score-wide for LilyPond export so start and stop commands survive
  spans whose anchors fall in different measures or voices.

## [1.3.6] — 2026-04-11

### LilyPond export (MEI → LilyPond)

- **MEI keysig attribute parsed correctly**: the `@keysig` attribute on MEI
  `<staffDef>` elements uses the format `Nf`/`Ns` (e.g. `3f` for 3 flats,
  `2s` for 2 sharps).  Previously this was parsed as a plain integer which
  silently failed, causing `\key` commands to be omitted from the LilyPond
  output.  Key signatures now render correctly for all MEI-origin scores.

### MusicXML import (MusicXML → MEI)

- **Written accidentals no longer double-encoded**: imported pitched notes now
  avoid emitting both `@accid.ges` and an `<accid>` child for the same written
  accidental. This removes a large class of Verovio warnings while keeping the
  written accidental itself intact.
- **Credits exported in Verovio-friendly `pgHead` form**: MusicXML credits now
  map to `pgHead/rend` instead of `pgHead/anchoredText`, eliminating Verovio
  warnings that previously caused all such header text to be ignored.
- **Numeric font sizes normalized with `pt` units**: imported MusicXML
  `music-font`, `word-font`, and `lyric-font` sizes are now serialized as MEI
  values like `20pt` and `10.2pt`, which Verovio accepts consistently.

### MusicXML export (MEI → MusicXML)

- **Font-size roundtrip restored for `pt` values**: MEI font sizes stored as
  numeric strings with a `pt` suffix now parse back into MusicXML defaults
  correctly, restoring full `mei_via_musicxml` stability.
- **Pitch alter recovered from written accidentals**: when an MEI note carries
  a written `<accid>` child but no `@accid.ges`, MusicXML export now derives
  `pitch.alter` from that written accidental so MusicXML roundtrips preserve
  sounding chromatic alteration.

### Tooling

- **Batch Verovio analysis harness added**: `scripts/analyze_musicxml_verovio.py`
  converts MusicXML fixtures to MEI, renders them with Verovio, and summarizes
  warnings, errors, and crash cases into `target/verovio-musicxml-analysis/`.

## [1.3.5] — 2026-04-11

### MusicXML import (MusicXML → MEI)

- **Filler mRest duration stored**: empty single-layer staves that receive a
  filler `<mRest>` now also get an `xml:id` and `mrest_info` entry in the
  extension store with the correct time-signature duration.  Previously these
  filler mRests had no stored duration, causing the LilyPond export to emit
  bare `R` (whole note regardless of meter) which triggered barcheck failures
  and Guile crashes in downstream engravers.

## [1.3.4] — 2026-04-11

### LilyPond export (MEI → LilyPond)

- **Invalid `\ornam` removed**: ornament accidental marks from MusicXML no
  longer emit the non-existent `\ornam` command. Only known LilyPond ornament
  names are output; unrecognised ornament detail types are silently skipped.
- **Two-note tremolo (`fTrem`) support**: MusicXML fingered tremolos
  (`<tremolo type="start/stop">`) now export as `\repeat tremolo N { … }`
  instead of dumping individual notes, which caused Guile crashes.
- **Voice directives always injected**: multi-layer measures now always get
  `\voiceOne`/`\voiceTwo` directives regardless of whether the MusicXML
  source had part-detail metadata. Fixes Guile crashes from rhythm conflicts
  in polyphonic `<< >>` blocks without stem-direction commands.
- **Pickup/timing always emitted**: `\partial`, timing resets before `\time`
  changes, and explicit barlines are now unconditional — no longer gated on
  part-detail metadata. Fixes regressions where pickup measures without
  `\partial` caused cascading barcheck failures and Guile crashes.
- **MusicXML-origin export detection hardened**: LilyPond export now keys
  spacer voices, voice commands, timing resets, and explicit barlines off
  `ExtensionStore::source_format` instead of fragile part-detail presence.
- **Skip-only companion voices normalized**: single-layer measures with spacer
  companions now stay flat, pure skip-only layers collapse cleanly, and
  trailing skip-only companion layers are dropped after real polyphony.
  This stabilizes many `lilypond_via_musicxml` roundtrips.
- **Leading control-event ordering stabilized**: initial `skipTypesetting` and
  similar control items now stay ahead of injected clef/key/time signatures,
  while `skipBars` remains after them.
- **Full-measure spacer emission tightened**: MusicXML-origin single-layer
  measures that already fill the bar no longer get an extra trailing spacer.
  This fixes `musicxml_via_lilypond` instability where simple files gained an
  extra empty `<forward>` measure on the second pass.

### LilyPond import (LilyPond → MEI)

- **Dormant repeated measure rests aligned only when needed**: runs of `R R …`
  now duplicate into split voices only when a later simultaneous voice entry
  requires that alignment, avoiding later voice drift without manufacturing
  fake trailing polyphony.
- **Cross-staff overrides reset per measure**: `\change Staff` state no longer
  leaks across resolved measure boundaries on re-import, fixing cross-staff
  regressions in glissando, slur, and voice-follower cases.

### MusicXML export (MEI → MusicXML)

- **Cross-staff item/id tracking fixed**: inserted LilyPond context changes now
  keep item positions and xml:id tracking aligned during export, which
  stabilizes cross-staff tuplets and related beam-wrapped content.
- **Multi-measure rest metadata restored**: stored LilyPond duration metadata
  for MEI `<mRest>` now roundtrips back into MusicXML note-type, dot, and
  duration values instead of collapsing to underspecified measure rests.

### MusicXML import (MusicXML → MEI)

- **Measure-rest duration metadata retained**: imported MusicXML measure rests
  now store LilyPond-oriented duration details in the extension store so the
  MusicXML/LilyPond pipelines can reconstruct exact rest spelling.

### Cross-format roundtrip stabilization

- `cargo test --workspace` is green again.
- LilyPond, MusicXML, and MEI cross-format suites now stabilize for the full
  workspace, including the previously failing `musicxml_via_lilypond`,
  `lilypond_via_mei`, `lilypond_via_musicxml`, and `mei_via_lilypond` cases.

## [1.3.3] — 2026-04-10

### MusicXML import (MusicXML → MEI)

- **Empty measure stabilization**: single-staff parts with empty layers in
  multi-staff scores now get an `<mRest>` fill during import, preventing
  empty bar-check pairs from collapsing during LilyPond re-import.

### CLI

- **UTF-16 support**: MusicXML files encoded as UTF-16 BE or UTF-16 LE
  (with BOM) are now automatically transcoded to UTF-8 before parsing.
  UTF-8 BOM is also stripped.

### LilyPond export (MEI → LilyPond)

- **Pickup measures**: measures with `metcon="false"` (MusicXML `implicit="yes"`)
  now emit `\partial dur` before the measure content, preventing LilyPond
  barcheck failures on anacrusis measures.
- **Technical notation markup**: MusicXML technical elements like `<pluck>`,
  `<tap>`, `<handbell>`, and non-numeric `<fingering>` values (e.g. "P" for
  thumb) are now exported as `^\markup { "P" }` instead of the invalid `\P`.
  `<fing>` MEI elements are also handled.
- **Tremolo/tie ordering**: tremolo shorthand (`:32`) is now serialized before
  ties (`~`), producing `c4:32~` instead of `c4~:32` which caused a LilyPond
  Guile crash (`ly:item-get-column` error).

### LilyPond parser

- **`\partcombine` variants**: `\partcombine`, `\partcombineDown`,
  `\partcombineUp` (and lowercase forms) now recognized as 2-argument
  music functions, enabling correct AST representation of combined voices.

### LilyPond import (LilyPond → MEI)

- **Polyphonic dormant-voice alignment**: resolved measure splitting now keeps
  empty measures for non-primary voices when exported bar checks/barlines mark
  flat stretches, preventing later voice content from sliding earlier on
  re-import.
- **Measure-rest barline handling**: a plain exported `\bar "|"` and trailing
  spacer after a measure rest now attach to the just-closed measure instead of
  creating a phantom empty one.
- **Control-event positioning**: auto-beam and related prefix controls now use
  the current measure start when they occur before any sounding event in the
  measure, improving roundtrip stability.
- **Exporter spacer voice extraction**: standalone exported spacer items are no
  longer broadcast into every split voice, and trailing spacer-only voices are
  trimmed after voice extraction.
- **Voice extraction from `<< prefix \new Voice {} >>` blocks**: polyphonic
  staves using `\new Voice` inside `<< >>` with prefix items (clef, key, time)
  now correctly split into separate voices instead of treating the entire block
  as a single voice.
- **Implicit duration resolution in polyphonic voices**: bare notes/rests in
  multi-voice staves (e.g. `s1 s s s`) now correctly inherit the previous
  duration for measure splitting, fixing overfull measures in fugues and
  polyphonic pieces.
- **`\partcombine` expansion**: `\partcombine` arguments expanded into
  simultaneous voices for MEI import.
- **Inline polyphony handling**: `<< { } \\ { } >>` blocks nested inside
  sequential streams now collect only voice 0's events, preventing doubled
  note content that caused wrong measure boundaries.
- **Inline `\new Voice` skipping**: secondary `\new Voice { ... }` items
  within sequential blocks are skipped during event collection when the
  block also has primary note content.
- **Time signature propagation**: polyphonic staves where only voice 0
  declares `\time` now propagate the time signature to other voices.
- **Chord repetition in multi-voice measures**: `q` (chord repetition)
  suppressed in multi-voice measure export, preventing unresolvable `q`
  after voice splitting.

### MusicXML export (MEI → MusicXML)

- **Forward-fill for empty polyphonic layers**: multi-voice measures where a
  voice has no content now emit a `<forward>` element to preserve voice
  presence, preventing voice numbering drift on MusicXML re-import.

### MusicXML import (MusicXML → MEI)

- **Voice detection from `<forward>` elements**: `<forward>` with a `<voice>`
  tag now contributes to distinct voice detection, ensuring forward-fill
  voices survive re-import.

### Cross-format roundtrip (LilyPond → MEI → MusicXML → MEI → LilyPond)

- 9 previously failing `lilypond_via_musicxml` roundtrip tests now pass
  (BWV870 Fuga, Couperin Prelude 2/3, MWP5V, MWPPT, Ballet, Pachelbel Canon,
  Greensleeves, Joplin Search-Light Rag).
- 1 previously failing `lilypond_via_mei` test fixed (Sor op35 no11).

### Internal

- **`source_format` on ExtensionStore**: new field tracks which format the
  content was imported from, enabling format-aware export decisions.

## [1.3.2] — 2026-03-04

### LilyPond parser

- **Directed post-events**: `^~ _~ -~` (directed ties), `^( _( -(` / `^) _) -)`
  (directed slurs), and `^\( _\(` / `^\) _\)` (directed phrasing slurs) now
  parsed and serialized with their direction preserved.
- **Arpeggio ornament**: `\arpeggio` recognized as a known ornament.

### LilyPond import (LilyPond → MEI)

- **Hybrid measure splitting**: new measure splitter combines duration-based
  splitting with bar check / bar line boundaries, fixing files that use
  `\bar "||"` at section ends but have no bar checks within sections (previously
  collapsed hundreds of notes into a single measure).
- **Cross-voice slur resolution**: slurs starting inside a `<< >>` voice block
  and ending outside it (or vice versa) are now correctly matched across voice
  boundaries instead of being silently dropped.
- **Directed tie/slur preservation**: tie direction (`^~ _~`) and slur curvedir
  (`^( _(`) stored in `ExtensionStore` for lossless roundtrip.
- **Multi-score ID uniqueness**: multiple `\score` blocks now generate unique
  staffDef/staffGrp IDs per score, preventing event sequences from overwriting
  each other in the extension store.
- **Multiple bare `\score` blocks**: files with multiple `\score` blocks outside
  `\book` now produce separate `<mdiv>` elements (previously only with `\book`).

### LilyPond export (MEI → LilyPond)

- **Directed slur/tie export**: MEI `@curvedir` on slurs and tie direction from
  the extension store re-emitted as `^( _( ^~ _~` etc.
- **Stable signature injection**: clef/key/time events skipped for non-first
  measures (voice splitting changes note counts between passes, causing unstable
  injection positions). Tempo/mark/markup events still injected.
- **Empty measure suppression**: measures where all staves have empty layers are
  skipped to prevent roundtrip instability.
- **Repeat wrapping fixes**: cross-measure repeat boundary detection improved;
  already-wrapped repeat IDs nulled out to prevent double-wrapping.

### Roundtrip stability

- All 2708 `lilypond_via_mei` tests pass (0 failures).
- All 402 `mei_via_lilypond` tests pass (0 regressions).

## [1.3.1] — 2026-03-03

### LilyPond parser

- **`\include` directive**: top-level `\include "file.ly"` now parsed and
  serialized as `ToplevelExpression::Include`, preserving file references
  in the AST for files that contain musical content.
- **Dotted property paths**: output-def assignments like
  `system-system-spacing.minimum-distance = #15` in `\paper`/`\layout` blocks
  now parsed correctly (converted to equivalent `#'sub-property` form internally).

## [1.3.0] — 2026-03-02

### LilyPond parser

- **`\language` directive**: top-level `\language "english"` now parsed and
  serialized as `ToplevelExpression::Language`.
- **Assignment sub-properties**: `name #'padding = value` parsed with new
  `sub_property` field on `Assignment`.
- **`\tempo` in `\midi`**: `\tempo` inside `\midi {}` blocks now parsed as
  `MidiItem::Tempo` instead of causing errors.
- **Non-ASCII identifiers**: lexer handles UTF-8 multi-byte characters in
  identifiers (accented letters, non-Latin scripts) instead of erroring.

### LilyPond import (LilyPond → MEI)

- **Simultaneous pitch context**: each voice in `<< >>` blocks now resets to
  the entering reference pitch during event collection, fixing octave drift
  when voices have different final pitches.
- **ContextedMusic pitch traversal**: `build_pitch_context_from_music` now
  traverses `\new Staff` / `\new Voice` wrappers to find the starting pitch
  for `\relative` mode.

### LilyPond export (MEI → LilyPond)

- **Grace wrapping ID sync**: `apply_grace_wrapping` now maintains `item_ids`
  in parallel with `items`, fixing repeat boundary misalignment after grace
  note groups.
- **Nested beam consistency**: `collect_layer_child_ids` and
  `collect_grace_types` now skip multi-child nested `BeamChild::Beam` entries
  that `convert_beam_child` drops, keeping parallel arrays in sync.
- **Grace types alignment**: extra items injected by `\change Staff` context
  changes now get corresponding `None` entries in `grace_types`, preventing
  off-by-one misalignment in the tuplet/grace/repeat wrapping pipeline.
- **Duplicate slur stripping**: re-parsed music arguments in function calls
  (e.g. `\shape`) have slur events stripped, since the global slur map already
  handles slur attachment. Fixes duplicate `SlurStart` causing unmatched slur
  validation errors.

### Test infrastructure

- **MutopiaProject submodule**: 5681 LilyPond files from the MutopiaProject
  added as a git submodule for real-world roundtrip testing.
- **Mutopia roundtrip tests**: build-time pipeline probe generates per-file
  `try_lilypond_via_mei` and `try_lilypond_via_musicxml` tests for all
  standalone Mutopia files that pass the first pipeline pass (655 tests).
- **`try_lilypond_via_musicxml` helper**: new cross-format pipeline helper for
  LilyPond → MusicXML → LilyPond roundtrip testing.

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

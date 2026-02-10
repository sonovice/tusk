# LilyPond Coverage Tasks

Tasks for achieving full LilyPond ↔ MEI bidirectional conversion coverage, with a parser that validates against the entire LilyPond grammar (specs/lilypond/repo/lily/parser.yy). Coverage aims to exceed the existing musicxml2ly converter.

Each task covers: `[L]` Lexer, `[P]` Parser, `[S]` Serializer, `[I]` Import (LilyPond→MEI), `[E]` Export (MEI→LilyPond), `[T]` Tests, `[V]` Validator.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: The `tusk_lilypond_coverage.sh` script runs tests/clippy, feeds results + this task list to Claude, which works on one section at a time.

**Constraint**: Every change must pass `cargo test` and `cargo clippy --all-targets` with no regressions.

**File size limit**: No hand-written `.rs` file may exceed 1500 lines. If a file is over 1500 LOC — even a pre-existing one — split it into submodules before or as part of the current task. This does **not** apply to generated files (`generated/` directories, `generated_*.rs` files, version `data.rs` files).

**Key references**:
- Coverage plan: `.cursor/plans/` (LilyPond Coverage Plan)
- LilyPond grammar: `specs/lilypond/repo/lily/parser.yy`
- LilyPond lexer: `specs/lilypond/repo/lily/lily-lexer.cc`, `lily-lexer-keywords.cc`
- musicxml2ly: `specs/lilypond/repo/scripts/musicxml2ly.py`, `specs/lilypond/repo/python/musicexp.py`
- LilyPond model/AST: `crates/formats/lilypond/src/model/`
- Lexer: `crates/formats/lilypond/src/lexer/`
- Parser: `crates/formats/lilypond/src/parser/`
- Serializer: `crates/formats/lilypond/src/serializer/`
- Import (LilyPond→MEI): `crates/formats/lilypond/src/import/`
- Export (MEI→LilyPond): `crates/formats/lilypond/src/export/`
- Validator: `crates/formats/lilypond/src/validator/`
- Test fixtures: `tests/fixtures/lilypond/`
- MEI model: `crates/core/model/src/generated/` — **DO NOT EDIT**
- MEI extensions: `crates/core/model/src/extended/`
- Format traits: `crates/core/format/src/lib.rs`

---

## Retaining element IDs (MEI xml:id ↔ LilyPond)

When exporting MEI (or the internal model) to LilyPond we must **retain element IDs** so that roundtrip and external tooling (e.g. linking, SVG/HTML export) can refer to the same elements. LilyPond supports this natively; comments can be used as a fallback.

### LilyPond mechanisms

1. **Grob `id` property** (primary): Every grob (graphical object) has an `id` property (string) on the [grob-interface](https://lilypond.org/doc/v2.24/Documentation/internals/grob_002dinterface). It is user-settable via:
   - **`\tweak GrobType.id #"value"`** for note/event-attached elements (e.g. `c4 \tweak NoteHead.id #"mei-note-42"`, `r2 \tweak Rest.id #"mei-rest-1"`, slurs/ties via `\tweak Slur.id #"..."`).
   - **`\override Context.GrobType.id = #"value"`** for context-level overrides when a single grob is targeted.
   - In **SVG output**, this id is assigned to the `<g>` group for that grob; in PostScript/PDF the property exists in the .ly source and can be recovered on re-parse.
   - Reference: `specs/lilypond/repo/scm/define-grob-properties.scm` (`(id ,string? "An id string for the grob.")`), and grob-interface docs (`output-attributes` for SVG).

2. **`output-attributes`** (SVG): For SVG backend, grobs also support `output-attributes` (alist), e.g. `'((id . "xyz"))`, which becomes `<g id="xyz">` in the SVG. Setting `id` on the grob is the standard way; output-attributes can carry additional attributes.

3. **Comments** (fallback for roundtrip): Emitting a comment like `% @id mei-note-42` immediately before or after an element gives a parseable anchor when re-importing .ly so that IDs are preserved even if we do not yet emit `\tweak id` for every element type, or if the backend does not expose grob id in the output format.

### Mapping by element type

| MEI element / concept | LilyPond grob / placement |
|------------------------|----------------------------|
| note (xml:id) | `\tweak NoteHead.id #"id"` (or Rest for rest) |
| chord (multiple note heads) | One NoteHead per pitch; use same or per-note ids as in MEI |
| rest | `\tweak Rest.id #"id"` |
| slur, tie, phrasing slur | `\tweak Slur.id` / `\tweak Tie.id` / `\tweak PhrasingSlur.id` |
| beam | `\tweak Beam.id #"id"` |
| dynamics, hairpin | `\tweak DynamicText.id` / `\tweak Hairpin.id` |
| articulations, ornaments | Script/Articulation grobs; `\tweak Script.id` etc. |
| control events (tempo, mark, etc.) | Corresponding grob (MetronomeMark, RehearsalMark, etc.) with `\tweak GrobType.id` |
| lyrics (syl) | `\tweak LyricText.id` (or LyricHyphen / LyricExtender where applicable) |

### Tasks (see Phase 26, 31, 32)

- **Parser/Serializer** (Phase 26 or when implementing \tweak): Parse and serialize `\tweak #'id #"string"` so the AST can carry an optional id per event/grob.
- **Import** (Phase 31): When building MEI from LilyPond AST, set `xml:id` on the MEI element from the AST’s tweak id (and optionally from `% @id` comment when present).
- **Export** (Phase 32): For every MEI element that has `xml:id`, emit the appropriate `\tweak GrobType.id #"xml:id-value"`; optionally also emit `% @id xml:id-value` for robustness. Ensure spanners (slur, tie, hairpin, etc.) get id on the corresponding span grob.

---

## Phase 1: Crate Scaffolding & Lexer Foundation

### 1.1 Crate & Format Trait

- [x] [P] Create `crates/formats/lilypond/` with `Cargo.toml` (deps: tusk-model, tusk-format, thiserror, tracing)
  - Cargo.toml with workspace deps; module stubs for model, lexer, parser, serializer, import, export, validator
- [x] [P] Add `crates/formats/lilypond` to workspace `members` in root `Cargo.toml`
- [x] [P] Add `tusk-lilypond` to `[workspace.dependencies]` if needed for CLI
- [x] [P] Implement `Format` trait: id `lilypond`, name `LilyPond`, extensions `["ly"]`, `detect()` via content sniff (e.g. `\version` or `\score` or `{` after optional BOM/whitespace)
  - detect() checks first 4KB for `\version`, `\score`, or leading `{`
- [x] [P] Implement `Importer` and `Exporter` (stub: return error "not implemented" or parse empty score)
  - ImportError::NotImplemented and ExportError::NotImplemented
- [x] [T] Register format in `crates/bindings/cli` and add basic test that format is detected
  - Added registry_finds_lilypond_by_extension and registry_detects_lilypond_from_content tests

### 1.2 Lexer Foundation

- [x] [L] Define `Token` enum in `lexer/tokens.rs`: keywords (`\version`, `\score`, `\book`, `\relative`, etc.), identifiers, numbers, strings, operators (`<`, `>`, `<<`, `>>`, `{`, `}`, `~`, `|`, etc.), note names, duration digits, dots, octave marks (`'`, `,`), accidental modifiers (`!`, `?`)
  - 55+ keywords, note names (Dutch convention), symbols, operators, escaped operators, compound tokens (lyric hyphen/extender)
  - `is_note_name()` validates Dutch note name convention (a-g + is/es/isis/eses/ih/eh suffixes)
  - `keyword_from_str()` maps 50+ keyword strings to Token variants
- [x] [L] Implement `Lexer` struct in `lexer/mod.rs`: input slice, position, current token; `next_token()` producing `Token` + span
  - Hand-rolled byte-level scanner with `Span` tracking
  - `tokenize_all()` convenience method for tests
  - Words are purely alphabetic (no digits) matching LilyPond's SYMBOL regex
- [x] [L] Tokenize comments (`%` to EOL) and skip whitespace
  - Line comments (`%`), block comments (`%{ ... %}`), nested block comments
- [x] [L] Tokenize string literals (`"..."` with escapes)
  - Supports `\n`, `\t`, `\\`, `\"`, `\'` escape sequences
- [x] [V] Lexer produces correct token sequence for a minimal `.ly` snippet (e.g. `\version "2.24" \score { { c4 } }`)
  - `fixture_minimal_score` test validates full token sequence
- [x] [T] Unit tests for lexer on minimal inputs
  - 47 unit tests covering: whitespace, comments, strings, numbers, keywords, note names, operators, escaped operators, compound sequences, spans, note name recognition
- [x] [T] Crate compiles; `cargo test -p tusk-lilypond` runs (47 lexer tests pass)

---

## Phase 2: Parser Core — Top-Level & Score Structure

### 2.1 Model

- [x] [P] Add AST types in `model/`: `LilyPondFile` (top-level expressions), `Version` (string), `ScoreBlock`, `BookBlock`, `BookPartBlock`, `HeaderBlock`, `ScoreBody` (score items: music, header, layout, midi, etc.)
  - LilyPondFile, Version, ToplevelExpression, ScoreBlock, ScoreItem, BookBlock, BookItem, BookPartBlock, BookPartItem, HeaderBlock, LayoutBlock, LayoutItem, MidiBlock, PaperBlock, ContextModBlock, ContextModItem, Assignment, AssignmentValue, Music (Sequential, Simultaneous, Relative, Fixed, ContextedMusic, Event, Identifier, Unparsed)
- [x] [P] Parse `\version "..."` and store in AST
  - Parser::parse_version() → Version { version: String }
- [x] [P] Parse top-level: `\score { ... }`, `\book { ... }`, `\bookpart { ... }`, `\header { ... }`, standalone music/markup (grammar: `toplevel_expression`, `score_block`, `book_block`, `bookpart_block`, `header_block`)
  - Recursive-descent parser with one-token lookahead; parse_score_block, parse_book_block, parse_bookpart_block, parse_header_block, parse_music (sequential, simultaneous, relative, fixed, context)
  - Assignment detection via lookahead for `=` with backtracking
  - Layout/midi/paper blocks with \context { } support including \consists, \remove, \ContextRef
- [x] [S] Serialize `\version`, `\score`, `\book`, `\bookpart`, `\header` blocks back to `.ly` string
  - serializer::serialize() with indentation; all block types, assignments, music expressions
- [x] [V] Parser accepts valid score-only and book-only files from fixtures
  - validator::validate() checks score has music; validates nested structures
- [x] [T] Parse `tests/fixtures/lilypond/simple.ly` and roundtrip via serializer
  - parse_simple_ly and roundtrip_simple_ly tests; parse_fragment_score_minimal and roundtrip_fragment_score_minimal
[x] [T] Fixture `simple.ly` (single staff, few notes) parses and serializes without error
  - 71 total tests: 47 lexer + 11 parser + 6 serializer + 5 validator + 2 roundtrips

---

## Phase 3: Basic Pitches, Durations, Rests

### 3.1 Model & Parser

- [x] [P] Add `Pitch` (step, alter, octave), `Duration` (log2, dots, multipliers), `NoteEvent`, `RestEvent`, `SkipEvent` in `model/note.rs`, `model/pitch.rs`, `model/duration.rs`
- [x] [P] Parse steno pitch: note name (a–g, optional accidental `is`/`es`/etc.), octave marks `'`/`,` (sup_quotes/sub_quotes), optional `!`/`?` (force/cautionary accidental), optional `= quotes` (octave check)
- [x] [P] Parse duration: integer or duration identifier, dots, optional `* n` or `* n/m` multipliers
- [x] [P] Parse rest `r`, skip `s`, optional `\rest` for pitched rest
- [x] [S] Serialize pitch (note name + octave marks + accidentals), duration, rest/skip
- [x] [V] Validator checks duration values in range and dot count
- [x] [T] Fragment fixtures: pitch-only, duration-only, rest, skip; parse and serialize
  - 109 total tests: 47 lexer + 12 model + 29 parser + 12 serializer + 9 validator

### 3.2 Import & Export

- [x] [I] Map `NoteEvent` (pitch + duration) → MEI `<note>` with @pname, @accid, @oct, @dur, @dots
  - import::import() builds full MEI hierarchy: Mei → MeiHead + Music → Body → Mdiv → Score → ScoreDef + Section → Measure → Staff → Layer → Note/Rest/MRest
  - Pitch: step → @pname, octave marks → @oct (c=3, c'=4, c,=2), alter → @accid.ges
  - Duration: base → @dur (DataDurationCmn), dots → @dots (DataAugmentdot)
  - Written accidentals (!/?) → Accid child with @accid and @func="cautionary"
  - Pitched rests → Rest with lilypond:pitched-rest label for roundtrip
  - Multi-measure rests → MRest with lilypond:mrest label encoding dur/dots/multipliers
- [x] [I] Map `RestEvent` → MEI `<rest>` with @dur, @dots
  - RestEvent → Rest with @dur, @dots; SkipEvent ignored (no MEI equivalent)
- [x] [E] MEI note/rest → LilyPond note/rest (relative pitch context handled in later phase)
  - export::export() walks MEI hierarchy, converts Note/Rest/MRest back to LilyPond AST
  - Restores pitch from @pname/@oct, alter from @accid.ges, duration from @dur/@dots
  - Pitched rest label → NoteEvent with pitched_rest=true
  - MRest label → MultiMeasureRestEvent with full duration/multipliers
  - Exporter trait wired: LilyPondFormat::export_to_string() → export() + serialize()
  - Importer trait wired: LilyPondFormat::import_from_str() → parse() + import()
- [x] [T] Roundtrip: LilyPond → MEI → LilyPond on simple note/rest fixture
  - 10 roundtrip tests: single note, accidentals, rests, dotted, flat, multiple notes, multi-measure rest, pitched rest, force accidental, cautionary accidental
  - 10 import tests: single note, accidental, rest, dotted rest, multi-measure rest, pitched rest, multiple events, skip ignored, score block, nested relative
- [x] [T] All Phase 3 fragment tests pass; no regressions in Phase 1–2
  - 129 total tests: 47 lexer + 12 model + 29 parser + 12 serializer + 9 validator + 10 import + 10 export roundtrip

---

## Phase 4: Sequential & Simultaneous Music

### 4.1 Model & Parser

- [x] [P] Add `SequentialMusic`, `SimultaneousMusic`, `MusicList`, `GroupedMusicList`; parse `{ ... }` (sequential) and `<< ... >>` (simultaneous)
  - Music::Sequential(Vec<Music>) and Music::Simultaneous(Vec<Music>) already in model from Phase 2
  - Added `\sequential { }` and `\simultaneous { }` explicit keyword forms (parse_explicit_sequential/parse_explicit_simultaneous)
  - Added `\\` voice separator handling in `<< ... \\ ... >>` simultaneous music
- [x] [P] Grammar: `braced_music_list`, `sequential_music`, `simultaneous_music`, `music_list` (reversed list)
  - parse_sequential_music, parse_simultaneous_music, parse_explicit_sequential, parse_explicit_simultaneous in parser
- [x] [S] Serialize sequential with `{ }`, simultaneous with `<< >>`
  - Already implemented in serializer from Phase 2
- [x] [V] Brace/angle bracket matching and balanced structure
  - Parser enforces via expect(); validator recursively validates music structure
- [x] [T] Parse/serialize nested `{ << { c4 d4 } { e4 f4 } >> }`
  - 9 new tests: nested sequential/simultaneous, explicit keyword forms, `\\` separator, deeply nested, empty blocks, direct notes in `<< >>`, fixture roundtrip
  - Fixture: fragment_sequential_simultaneous.ly
  - 138 total tests pass

### 4.2 Import & Export

- [x] [I] Sequential → linear MEI layer; simultaneous → multiple layers or staff groups as per MEI model
  - Sequential music flattened into single MEI layer (layer @n=1)
  - Simultaneous music (`<< { voice1 } { voice2 } >>`) → multiple layers on same staff (layer @n=1, 2, ...)
  - extract_voices() detects voice-like children (Sequential, Note, Rest, Relative, etc.) and splits into separate layers
- [x] [E] MEI layers/parallel content → LilyPond `<< >>` or `{ }` as appropriate
  - Single layer → `{ ... }` (sequential); multiple layers → `<< { voice1 } { voice2 } >>` (simultaneous)
  - build_music_from_layers() handles single/multi layer dispatch
- [x] [T] Roundtrip two-voice score
  - roundtrip_two_voices, roundtrip_three_voices, roundtrip_sequential_preserved tests
- [x] [T] Fixtures for sequential and simultaneous; roundtrip via MEI
  - Fixture: fragment_two_voices.ly; 4 import tests + 3 roundtrip tests
  - 145 total tests pass

---

## Phase 5: Contexts — Staff, Voice, \new, \context, \change, \with

### 5.1 Model & Parser

- [x] [P] Add `ContextPrefix`, `ContextModification`, `ContextDefSpec`, `OptionalContextMods`; parse `\new Staff`, `\new Voice = "name"`, `\context "Staff"`, `\change Staff = "name"`, `\with { ... }`
  - Added `ContextKeyword` enum (New/Context) to distinguish `\new` vs `\context`
  - Added `Music::ContextChange { context_type, name }` for `\change Staff = "name"`
  - Updated `ContextedMusic` to carry `keyword` field
  - `parse_context_music()` records keyword, accepts String or Symbol for context type
  - `parse_context_change()` for `\change ContextType = simple_string`
  - `parse_optional_context_mods()` supports multiple `\with` blocks per grammar
  - `expect_simple_string()` accepts quoted string or bare symbol
- [x] [P] Parse context modifier list in `\with { \consists ..., \remove ..., \override ..., \set ..., etc. }`
  - Reuses existing `parse_context_mod_item()` (\\consists, \\remove, \\ContextRef, assignments)
  - Multiple `\with` blocks merged into single list per grammar `context_modification_mods_list`
- [x] [S] Serialize context prefix and `\with` blocks
  - Serializer uses `ContextKeyword` to emit `\new` or `\context`
  - Added `Music::ContextChange` serialization: `\change Type = "name"`
- [x] [V] Context names and types consistent
  - Added `KNOWN_CONTEXT_TYPES` list (30 types: Score, Staff, Voice, PianoStaff, etc.)
  - `ValidationError::UnknownContextType` for unrecognized context types
  - Validates both `ContextedMusic` and `ContextChange`
- [x] [T] Parse `\score { \new Staff { c4 } }` and `\new PianoStaff << \new Staff { } \new Staff { } >>`
  - 10 new tests: parse_new_staff_with_name, parse_context_staff, parse_new_with_block, parse_context_change, parse_nested_new_staff, parse_piano_staff, roundtrip_context_fixture, unknown_context_type, known_context_type_passes, context_change_unknown_type
  - Fixture: fragment_contexts.ly (StaffGroup with named staves)
  - 155 total tests pass

### 5.2 Import & Export

- [x] [I] `\new Staff` / `\new Voice` → MEI staff/voice structure; map context type to MEI staffDef/scoreDef
  - analyze_staves() detects context hierarchy: StaffGroup/PianoStaff/ChoirStaff wrapping \new Staff children
  - Maps context types to MEI staffGrp @symbol (bracket/brace) and creates separate MEI staves with staffDefs
  - Staff names stored in staffDef @label; group metadata in staffGrp @label (lilypond:group,/lilypond:staff, prefix)
  - Supports StaffGroup, PianoStaff, GrandStaff, ChoirStaff + 8 staff context types
- [x] [I] `\with { }` overrides → store in conversion context or MEI extensions for roundtrip
  - \with blocks serialized to compact string and stored in staffDef/staffGrp @label for lossless roundtrip
  - Re-parsed from label on export using serialize_with_block/parse_with_block_str
- [x] [E] MEI staff/part → `\new Staff` / `\new Voice` with optional `\with`
  - extract_group_meta/extract_staff_metas reconstruct context hierarchy from MEI labels and @symbol
  - build_music_with_contexts wraps per-staff music in \new Staff/\new StaffGroup with names and \with blocks
  - Single unnamed staff with no group produces flat output (backward compatible)
- [x] [T] Roundtrip score with multiple staves
  - roundtrip_staff_group, roundtrip_piano_staff, roundtrip_named_staves, roundtrip_single_named_staff, roundtrip_staff_with_block
  - Import tests: import_staff_group_creates_multiple_staves, import_staff_group_symbol, import_piano_staff_symbol, import_named_staff_label, import_group_label, import_staff_with_block_label
- [x] [T] Piano-style score fixture; roundtrip
  - Fixture: fragment_piano.ly (PianoStaff with right/left named staves)
  - roundtrip_contexts_fixture validates fragment_contexts.ly (StaffGroup with violin/viola)
  - 169 total tests pass

---

## Phase 6: Clefs, Key Signatures, Time Signatures

### 6.1 Model & Parser

- [x] [P] Parse `\clef "treble"` (and other clef names), `\key pitch \mode`, `\time n/m` (and compound, e.g. `\time 2+3/8`)
  - `\clef` accepts quoted string or bare symbol; `\key` parses pitch + `\mode`; `\time` parses N/D and additive N+M+.../D
  - Dispatched from parse_music() via EscapedWord("clef"), EscapedWord("key"), Token::Time
- [x] [P] Add AST nodes: `Clef`, `KeySignature`, `TimeSignature`
  - model/signature.rs: Clef (name), KeySignature (pitch, Mode enum), TimeSignature (numerators vec, denominator)
  - Mode enum: Major/Minor/Ionian/Dorian/Phrygian/Lydian/Mixolydian/Aeolian/Locrian
  - Music::Clef, Music::KeySignature, Music::TimeSignature variants
- [x] [S] Serialize clef, key, time
  - `\clef "name"`, `\key pitch \mode`, `\time N+M/D` (additive numerators joined with +)
- [x] [V] Key pitch and mode valid; time numerator/denominator valid
  - Clef: validates base name against KNOWN_CLEF_NAMES (70+ entries incl. transposition suffixes)
  - Time: numerators non-empty and non-zero, denominator non-zero
- [x] [T] Fragment: `\clef bass \key d \minor \time 3/4`
  - Fixture: fragment_clef_key_time.ly (treble/bass clefs, D major/Bb minor keys, 4/4, 3/4, 2+3/8 times)
  - 13 parser tests: clef string/symbol/transposed, key major/flat/all 9 modes, time simple/compound/additive/triple-additive, combined sequence, fixture roundtrip
  - 4 serializer tests: clef, key signature, time simple, time additive
  - 8 validator tests: valid/unknown clef, transposed clef, valid key, valid/zero-denom/zero-numer/additive time
  - 193 total tests pass

### 6.2 Import & Export

- [x] [I] Clef/key/time → MEI `<scoreDef>` / `<staffDef>` clef, key, meter
  - First clef → staffDef @clef.shape, @clef.line, @clef.dis, @clef.dis.place
  - First key → staffDef @keysig (circle of fifths)
  - First time → staffDef @meter.count, @meter.unit
  - Full event sequence stored in staffDef @label as `lilypond:events,TYPE@POS;...` for roundtrip
  - clef_name_to_mei() maps 20+ clef names to MEI shape/line with transposition (_8, ^15) support
  - key_to_fifths() converts pitch+mode to circle-of-fifths value (all 9 modes)
  - 12 import tests: treble/bass/alto clef, D major/A minor/Bb major key, 3/4 and 2+3/8 time, label storage, mid-stream clef change, transposed clef
- [x] [E] MEI clef/key/meter → `\clef`, `\key`, `\time`
  - Event sequence label parsed back: clef name, key (step.alter.mode), time (N+M/D)
  - Fallback: reconstruct from staffDef attributes when no label present
  - inject_signature_events() inserts clef/key/time at correct positions in music stream
  - mei_clef_to_name() reverses MEI attributes to LilyPond clef name
  - fifths_to_key() reverses circle-of-fifths to pitch+mode
- [x] [T] Roundtrip with key and time change
  - roundtrip_clef_change_mid_stream, roundtrip_key_change, roundtrip_time_change
- [x] [T] Fixtures with various clefs, keys, times; roundtrip
  - roundtrip_clef_key_time_fixture uses existing fragment_clef_key_time.ly
  - 14 roundtrip tests: treble/bass/alto clef, D major/Bb minor key, 3/4 and 2+3/8 time, combined, mid-stream changes, transposed clef, fixture
  - 217 total tests pass

---

## Phase 7: Relative Pitch, Transpose, Octave Check

### 7.1 Model & Parser

- [x] [P] Parse `\relative c' { ... }` and `\relative pitch` (optional start pitch); parse `\transpose from to { ... }`
  - Added `Music::Transpose { from, to, body }` variant; `parse_transpose()` method
- [x] [P] Parse octave check `= '` / `= ,` after pitch
  - Added `octave_check: Option<i8>` to `Pitch` struct; parsed after accidentals, before duration
- [x] [S] Serialize `\relative`, `\transpose`, octave check
  - Transpose serialization in serializer; octave check `=` + quotes/commas in `write_pitch()`
- [x] [V] Relative start pitch and transpose from/to valid
  - Transpose from/to/body recursively validated
- [x] [T] Fragment: `\relative c'' { c d e f }` and `\transpose c d { c4 }`
  - 9 tests: parse_transpose variants, octave_check variants, relative_no_pitch, roundtrip fixture

### 7.2 Import & Export

- [x] [I] Relative/transpose → MEI as written (or expand to absolute); store relative/transpose in context for roundtrip
  - PitchContext struct tracks relative mode (ref_step/ref_oct) and transposition stack
  - collect_events() resolves relative pitches to absolute via Pitch::resolve_relative()
  - Transpose applied via Pitch::transpose() during collection
  - Pitch model extended with resolve_relative(), to_relative_marks(), transpose(), untranspose()
  - Labels stored: `lilypond:relative,STEP.ALTER.OCT` and `lilypond:transpose,FROM,TO` on staffDef
  - detect_pitch_context() walks music tree to find outermost relative/transpose wrapper
  - 5 import tests: resolves ascending/descending, label stored, transpose applies, transpose label
- [x] [E] When exporting, prefer `\relative` when all notes in a voice can be expressed relative to a single reference
  - extract_pitch_contexts() reads lilypond:relative/transpose labels from staffDefs
  - apply_pitch_contexts() converts absolute pitches back to relative marks or un-transposes
  - convert_to_relative() and untranspose_items() transform Music items
  - Wraps output in Music::Relative or Music::Transpose with original parameters
  - 9 roundtrip tests: relative basic/no-pitch/octave-jump/accidentals/descending, transpose basic/accidentals, relative-in-staff, fixture
- [x] [T] Roundtrip relative-mode score
  - roundtrip_relative_basic, roundtrip_relative_no_pitch, roundtrip_relative_octave_jump, roundtrip_relative_descending, roundtrip_relative_with_accidentals
- [x] [T] Relative and transpose fixtures; roundtrip
  - roundtrip_transpose_basic, roundtrip_transpose_with_accidentals, roundtrip_relative_in_staff, roundtrip_relative_transpose_fixture
  - 249 total tests pass

---

## Phase 8: Angle-Bracket Chords

### 8.1 Model & Parser

- [x] [P] Parse chord body `< ... >`: multiple pitches with optional accidentals/octave marks, shared duration, optional post-events
  - `parse_chord()` handles `AngleOpen`, collects pitches via `parse_chord_body_pitch()`, then `AngleClose` + optional duration
  - Each pitch in chord body supports octave marks, force/cautionary accidentals, octave check
- [x] [P] Add `ChordEvent`, `ChordBody`, chord body elements (pitch, drum pitch, post-event, function call)
  - `ChordEvent` struct in model/note.rs with `pitches: Vec<Pitch>` and `duration: Option<Duration>`
  - `Music::Chord(ChordEvent)` variant added to Music enum
- [x] [S] Serialize `< c e g >` with correct spacing and shared duration
  - `write_chord_event()` in serializer: `<pitch1 pitch2 ...>duration`
- [x] [V] Chord has at least one pitch; duration consistent
  - `ValidationError::EmptyChord` for empty pitch list; duration validated via `validate_duration()`
- [x] [T] Fragment: `<c e g>4`, `<c es g>2.`
  - Fixture: fragment_chords.ly (5 chord variants)
  - 10 parser tests: basic, accidentals, octave marks, force/cautionary, no duration, single pitch, mixed with notes, roundtrip basic, roundtrip complex, fixture roundtrip
  - 3 serializer tests: basic chord, accidentals, no duration
  - 3 validator tests: valid chord, empty chord, invalid duration
  - 265 total tests pass

### 8.2 Import & Export

- [x] [I] Chord → MEI chord (multiple note elements with same @dur, chord attribute)
  - `convert_chord()` creates MEI `<chord>` with `@dur`/`@dots` and Note children (one per pitch)
  - `convert_pitch_to_note()` creates MEI Note with pname/oct/accid (no duration) for chord children
  - Fixed `alter_to_accid_written()` to return `N` (natural) for alter=0 — enables force/cautionary on naturals
  - 5 import tests: basic, dotted, accidentals, force/cautionary, mixed with notes
- [x] [E] MEI chord → LilyPond `< ... >` chord
  - `convert_mei_chord()` extracts pitches from ChordChild::Note children + chord-level duration
  - `extract_pitch_from_note()` shared helper (also used by `convert_mei_note()` to reduce duplication)
  - `extract_chord_duration()` reads chord @dur/@dots → LilyPond Duration
- [x] [T] Roundtrip chord fixture
  - `roundtrip_chord_fixture` validates fragment_chords.ly (5 chords incl. accidentals, octaves, force/cautionary)
- [x] [T] Chord fixtures; roundtrip
  - 6 roundtrip tests: basic, dotted, accidentals, force/cautionary, mixed with notes, fixture
  - 276 total tests pass

---

## Phase 9: Ties, Slurs, Phrasing Slurs

### 9.1 Model & Parser

- [x] [P] Parse tie `~`, slur `( ... )`, phrasing slur `\( ... \)` as post-events or event identifiers
  - Added `PostEvent` enum (Tie, SlurStart, SlurEnd, PhrasingSlurStart, PhrasingSlurEnd) in model/note.rs
  - Added `post_events: Vec<PostEvent>` to NoteEvent, ChordEvent, RestEvent, SkipEvent, MultiMeasureRestEvent
  - `parse_post_events()` consumes Tilde, ParenOpen/Close, EscapedParenOpen/Close after duration
- [x] [P] Add `TieEvent`, `SlurEvent`, `PhrasingSlurEvent` (or generic span events with type)
  - Unified as PostEvent enum variants — extensible for future articulations/dynamics
- [x] [S] Serialize tie, slur, phrasing slur
  - `write_post_events()` emits `~`, `(`, `)`, `\(`, `\)` after duration on all event types
- [x] [V] Slur/phrasing slur start/stop match
  - `count_slurs()` recursively counts opens/closes; `validate_slur_balance()` checks match
  - ValidationError::UnmatchedSlur, ValidationError::UnmatchedPhrasingSlur
  - Balance checked at score and top-level music scope
- [x] [T] Fragment: `c4~ c`, `c4( d e f)`, `c4\( d e\)`
  - Fixture: fragment_ties_slurs.ly (tie, slur, phrasing slur, chord tie, combined)
  - 10 parser tests: tie, slur, phrasing slur, multiple post-events, chord tie, rest slur, roundtrip tie/slur/phrasing/fixture
  - 5 validator tests: balanced slurs/phrasing pass, unmatched slur/phrasing fail, tie-only passes
  - 291 total tests pass

### 9.2 Import & Export

- [x] [I] Tie → MEI `@tie` attr; slur → MEI `<slur>` control event; phrasing slur → `<slur label="lilypond:phrase">`
  - Tie: `@tie="i"` (initial), `"t"` (terminal), `"m"` (medial) on notes/chord-notes
  - Slurs: `MeasureChild::Slur` with `@startid`/`@endid` referencing note xml:ids
  - Phrasing slurs: same as slurs but with `@label="lilypond:phrase"` to distinguish
  - PendingSpanner pattern for tracking open slurs; make_slur() helper
- [x] [E] MEI tie/slur/phrase → LilyPond `~`, `( )`, `\( \)`
  - Ties: `@tie="i"/"m"` on notes → PostEvent::Tie; chord ties via any child note
  - Slurs: collect_slur_post_events() builds note-id→PostEvent map from MeasureChild::Slur
  - append_post_events() injects SlurStart/SlurEnd/PhrasingSlurStart/PhrasingSlurEnd
- [x] [T] Roundtrip tied and slurred phrases
  - 5 import tests: tie attr, slur control event, phrase labeled event, chord tie, combined
  - 6 roundtrip tests: tie, slur, phrasing slur, chord tie, combined, fixture
- [x] [T] Tie, slur, phrasing slur fixtures; roundtrip
  - fragment_ties_slurs.ly fixture roundtrip verified
  - 302 total tests pass

---

## Phase 10: Beaming

### 10.1 Model & Parser

- [x] [P] Parse beam start/end `[` and `]` (as event identifiers or post-events); parse `\autoBeamOn`, `\autoBeamOff`
  - BracketOpen/BracketClose → PostEvent::BeamStart/BeamEnd in parse_post_events()
  - \autoBeamOn/\autoBeamOff → Music::AutoBeamOn/AutoBeamOff in parse_music()
- [x] [P] Add `BeamEvent`, auto-beam setting in context
  - BeamStart/BeamEnd variants on PostEvent enum; AutoBeamOn/AutoBeamOff on Music enum
  - Split parser/mod.rs (2758→1389 LOC) by extracting tests to parser/tests.rs
- [x] [S] Serialize explicit beams and auto-beam commands
  - write_post_events: BeamStart→`[`, BeamEnd→`]`; write_music: AutoBeamOn/Off
- [x] [V] Beam brackets balanced
  - Refactored slur validation to SpanCounts struct; added UnmatchedBeam error
- [x] [T] Fragment: `c8[ d e f]`, `\autoBeamOff c8 d e f`
  - fragment_beams.ly fixture; 8 parser tests + 3 validator tests; 313 total pass

### 10.2 Import & Export

- [x] [I] Beams → MEI `<beam>` or beam span; auto-beam → context
  - BeamStart/BeamEnd post-events → group_beamed_notes() wraps notes in MEI `<beam>` container
  - AutoBeamOn/AutoBeamOff → encoded in staffDef event sequence label (`autobeamon@POS`, `autobeamoff@POS`)
  - layer_child_to_beam_child() converts LayerChild → BeamChild for Note/Rest/Chord
  - 5 import tests: beam creates element, multiple beams, beam with unbeamed, autobeam label, beam preserves content
- [x] [E] MEI beam → LilyPond `[ ]` or auto-beam
  - convert_layer_child_to_items() handles LayerChild::Beam by flattening children with BeamStart/BeamEnd post-events
  - convert_beam_child() converts BeamChild → Music; beam_child_xml_id() for slur post-event lookup
  - autobeamon/autobeamoff parsed from event label and injected via inject_signature_events()
- [x] [T] Roundtrip beamed passage
  - roundtrip_beam_basic, roundtrip_multiple_beams, roundtrip_beam_with_unbeamed, roundtrip_autobeam_commands
- [x] [T] Beam fixtures; roundtrip
  - roundtrip_beam_fixture validates fragment_beams.ly (manual beams + autoBeamOff/On)
  - 323 total tests pass

---

## Phase 11: Dynamics & Hairpins

### 11.1 Model & Parser

- [x] [P] Parse dynamics: `\p`, `\f`, `\ff`, `\sfz`, `\fp`, etc.; hairpins `\<`, `\>`, `\!`
  - PostEvent::Dynamic(String) for 22 known dynamics from dynamic-scripts-init.ly
  - PostEvent::Crescendo (`\<`), PostEvent::Decrescendo (`\>`), PostEvent::HairpinEnd (`\!`)
  - Parsed in parse_post_events() from EscapedAngleOpen/Close, EscapedExclamation, EscapedWord
- [x] [P] Add `DynamicsEvent`, `HairpinEvent` (cresc/dim)
  - PostEvent enum extended with Crescendo, Decrescendo, HairpinEnd, Dynamic(String)
  - KNOWN_DYNAMICS const and is_dynamic_marking() helper in model/note.rs
- [x] [S] Serialize dynamics and hairpins
  - write_post_events: Crescendo→`\<`, Decrescendo→`\>`, HairpinEnd→`\!`, Dynamic→`\name`
- [x] [V] Dynamic script and hairpin direction valid
  - SpanCounts extended with hairpin_opens/hairpin_closes
  - ValidationError::UnmatchedHairpin for unbalanced \</\>/\!
  - validate_post_events checks dynamic names against KNOWN_DYNAMICS
- [x] [T] Fragment: `c4\f d\p e\< f g\!\ff`
  - fragment_dynamics.ly fixture with 14 dynamic/hairpin events
  - 10 parser tests: parse_dynamic_f, parse_crescendo/decrescendo_hairpin, parse_multiple_dynamics, parse_all_standard_dynamics, parse_dynamics_on_chord/rest, roundtrip_dynamics/hairpins/fixture
  - 6 validator tests: balanced/unmatched hairpin, decrescendo balanced, dynamic does not affect balance, known dynamic passes
  - Split parser/tests.rs (1645→1238 LOC) by extracting Phase 9-11 tests to parser/tests_post_events.rs (423 LOC)
  - 339 total tests pass

### 11.2 Import & Export

- [ ] [I] Dynamics → MEI `<dynam>`; hairpins → MEI `<hairpin>`
- [ ] [E] MEI dynam/hairpin → LilyPond `\p`, `\f`, `\<`, etc.
- [ ] [T] Roundtrip dynamics and hairpins
- [ ] [T] Dynamics and hairpin fixtures; roundtrip

---

## Phase 12: Articulations & Script Abbreviations

### 12.1 Model & Parser

- [ ] [P] Parse script abbreviations: `.` (staccato), `-` (tenuto), `>` (accent), `^` (marcato), `+` (stopped), `!` (accent), `_` (portato); direction placement `^`, `_`, `-` for scripts
- [ ] [P] Parse fingering (digit after `-`, `^`, `_`), string number
- [ ] [P] Add `ArticulationEvent`, `ScriptAbbreviation`, `FingeringEvent`, `StringNumberEvent`
- [ ] [S] Serialize articulations and fingerings
- [ ] [V] Script direction and fingering digits valid
- [ ] [T] Fragment: `c4. c4- c4^> c4-5`

### 12.2 Import & Export

- [ ] [I] Articulations → MEI `<artic>`, fingerings → MEI fingering; script placement → @place
- [ ] [E] MEI artic/fingering → LilyPond abbreviations and fingerings
- [ ] [T] Roundtrip articulations and fingerings
- [ ] [T] Articulation and fingering fixtures; roundtrip

---

## Phase 13: Ornaments & Tremolos

### 13.1 Model & Parser

- [ ] [P] Parse ornaments: `\trill`, `\mordent`, `\turn`, `\prall`, `\prallprall`, etc.; single-note tremolo `:N` (e.g. `c8:32`)
- [ ] [P] Add `OrnamentEvent`, `TremoloEvent` (type and value)
- [ ] [S] Serialize ornaments and tremolo
- [ ] [V] Tremolo value in valid range
- [ ] [T] Fragment: `c4\trill`, `c8\mordent`, `c8:32`

### 13.2 Import & Export

- [ ] [I] Ornaments → MEI `<trill>`, `<mordent>`, `<turn>`, etc.; tremolo → MEI ornam label / bTrem
- [ ] [E] MEI ornaments and tremolo → LilyPond `\trill`, `:N`, etc.
- [ ] [T] Roundtrip ornaments and tremolo
- [ ] [T] Ornament and tremolo fixtures; roundtrip

---

## Phase 14: Fingering, String Numbers, Technical Notations

### 14.1 Model & Parser

- [ ] [P] Parse full fingering/string number syntax; other technical events (e.g. `\downbow`, `\upbow`, `\open`, `\harmonic`) as music/event functions
- [ ] [P] Add or extend technical event types in expression model
- [ ] [S] Serialize technical notations
- [ ] [V] String/fingering numbers in range
- [ ] [T] Fragment: `c4-3`, `c4\downbow`

### 14.2 Import & Export

- [ ] [I] Technical → MEI technical elements or ornam labels
- [ ] [E] MEI technical → LilyPond
- [ ] [T] Roundtrip technical fixtures
- [ ] [T] Technical notation fixtures; roundtrip

---

## Phase 15: Tuplets

### 15.1 Model & Parser

- [ ] [P] Parse `\tuplet n/m { ... }` (and nested tuplets); add `TupletMusic`, fraction, music list
- [ ] [S] Serialize `\tuplet n/m { ... }`
- [ ] [V] Tuplet fraction positive; nested tuplets well-formed
- [ ] [T] Fragment: `\tuplet 3/2 { c8 d e }`, nested tuplets

### 15.2 Import & Export

- [ ] [I] Tuplet → MEI tupletSpan / time-modification
- [ ] [E] MEI tuplet → LilyPond `\tuplet`
- [ ] [T] Roundtrip tuplet fixtures
- [ ] [T] Tuplet fixtures; roundtrip

---

## Phase 16: Grace Notes

### 16.1 Model & Parser

- [ ] [P] Parse `\grace { ... }`, `\acciaccatura { ... }`, `\appoggiatura { ... }`, `\afterGrace main { grace }`
- [ ] [P] Add `GraceMusic`, `AcciaccaturaMusic`, `AppoggiaturaMusic`, `AfterGraceMusic`
- [ ] [S] Serialize grace constructs
- [ ] [V] Grace block non-empty where required
- [ ] [T] Fragment: `\grace c16 d4`, `\acciaccatura c8 d4`

### 16.2 Import & Export

- [ ] [I] Grace → MEI grace group / @grace
- [ ] [E] MEI grace → LilyPond `\grace` or `\acciaccatura`/`\appoggiatura`
- [ ] [T] Roundtrip grace note fixtures
- [ ] [T] Grace note fixtures; roundtrip

---

## Phase 17: Repeats & Alternatives

### 17.1 Model & Parser

- [ ] [P] Parse `\repeat volta n { ... }`, `\repeat unfold n { ... }`, `\repeat percent n { ... }`, `\repeat tremolo n { ... }`; parse `\alternative { ... }` (alternative music list)
- [ ] [P] Add `RepeatedMusic` (type, count, body, alternatives), `AlternativeMusic`
- [ ] [S] Serialize \repeat and \alternative
- [ ] [V] Repeat count positive; alternative count matches repeat
- [ ] [T] Fragment: `\repeat volta 2 { c4 d e f } \alternative { { g2 } { a2 } }`

### 17.2 Import & Export

- [ ] [I] Repeat/alternative → MEI repeat/ending (volta)
- [ ] [E] MEI repeat/ending → LilyPond `\repeat` and `\alternative`
- [ ] [T] Roundtrip repeat fixtures
- [ ] [T] Repeat and alternative fixtures; roundtrip

---

## Phase 18: Multi-Measure Rests, Bar Lines, Bar Checks

### 18.1 Model & Parser

- [ ] [P] Parse multi-measure rest `R` with duration and post-events; parse bar check `|`; parse bar line commands (e.g. `\bar "|"`, `\bar "||"`)
- [ ] [P] Add `MultiMeasureRest`, `BarCheck`, `BarLine`
- [ ] [S] Serialize R, bar check, bar line
- [ ] [V] Multi-measure rest duration positive
- [ ] [T] Fragment: `R1*4`, `c4 | d4`, `\bar "|."`

### 18.2 Import & Export

- [ ] [I] Multi-measure rest → MEI multi-rest; bar line → MEI barline
- [ ] [E] MEI multi-rest and barline → LilyPond R and \bar
- [ ] [T] Roundtrip multi-rest and bar line fixtures
- [ ] [T] Multi-rest and bar line fixtures; roundtrip

---

## Phase 19: Chord Repetition

### 19.1 Model & Parser

- [ ] [P] Parse chord repetition `q` (repeats previous chord)
- [ ] [P] Add chord repetition to event chord handling
- [ ] [S] Serialize `q` with duration
- [ ] [V] Previous chord exists in context
- [ ] [T] Fragment: `<c e g>4 q q`

### 19.2 Import & Export

- [ ] [I] Chord repetition → MEI as repeated chord notes
- [ ] [E] MEI repeated chord → LilyPond `q` where applicable
- [ ] [T] Roundtrip chord repetition fixture
- [ ] [T] Chord repetition fixture; roundtrip

---

## Phase 20: Lyrics

### 20.1 Model & Parser

- [ ] [P] Parse `\lyricmode`, `\lyrics`, `\addlyrics`, `\lyricsto "voice"`; lyric elements (syllables), hyphen `--`, extender `__`; elision
- [ ] [P] Add `LyricModeMusic`, `Lyrics`, `AddLyrics`, `Lyricsto`, `LyricEvent`, `HyphenEvent`, `ExtenderEvent`
- [ ] [S] Serialize lyrics and lyric mode
- [ ] [V] Lyricsto voice reference exists
- [ ] [T] Fragment: `\addlyrics { A -- way __ }` and `\lyricsto "one" \lyricmode { One two }`

### 20.2 Import & Export

- [ ] [I] Lyrics → MEI `<verse>`, `<syl>`; syllabic and extend from hyphen/extender
- [ ] [E] MEI verse/syl → LilyPond lyrics and \addlyrics/\lyricsto
- [ ] [T] Roundtrip lyric fixtures
- [ ] [T] Lyric fixtures; roundtrip

---

## Phase 21: Markup

### 21.1 Model & Parser

- [ ] [P] Parse `\markup { ... }`, `\markuplist { ... }`; markup functions (e.g. \bold, \italic, \larger), markup list; `\score { ... }` inside markup
- [ ] [P] Add `Markup`, `MarkupList`, `MarkupFunction`, markup word/identifier
- [ ] [S] Serialize markup and markuplist
- [ ] [V] Markup braces balanced
- [ ] [T] Fragment: `\markup { \bold Hello }`, `\markup \score { { c4 } }`

### 21.2 Import & Export

- [ ] [I] Markup → MEI text/dir or annot with label for roundtrip
- [ ] [E] MEI text/dir → LilyPond \markup where applicable
- [ ] [T] Roundtrip markup fixtures
- [ ] [T] Markup fixtures; roundtrip

---

## Phase 22: Tempo, Rehearsal Marks, Segno, Coda, Text Events

### 22.1 Model & Parser

- [ ] [P] Parse `\tempo "Allegro" 4 = 120`, `\mark \default` or `\mark "A"`, segno/coda as markup or commands
- [ ] [P] Add `TempoEvent`, `MarkEvent`, text script events
- [ ] [S] Serialize tempo, mark, text events
- [ ] [V] Tempo duration and BPM valid
- [ ] [T] Fragment: `\tempo "Andante" 4 = 72`, `\mark \default`

### 22.2 Import & Export

- [ ] [I] Tempo → MEI tempo; rehearsal/mark → MEI dir or rehearsal
- [ ] [E] MEI tempo/rehearsal → LilyPond \tempo, \mark
- [ ] [T] Roundtrip tempo and mark fixtures
- [ ] [T] Tempo and mark fixtures; roundtrip

---

## Phase 23: Chord Mode

### 23.1 Model & Parser

- [ ] [P] Parse `\chordmode`, `\chords`; chord quality syntax: root, `:`, quality modifiers, `^` (omit), `/` (inversion), `/+` (bass); step numbers (e.g. `7`, `9+`, `11-`); chord modifiers (maj, min, dim, aug, etc.)
- [ ] [P] Add `ChordModeMusic`, `ChordName` (tonic, quality, bass, omit), `ChordModifier`, `ChordSeparator`
- [ ] [S] Serialize chord mode and chord names
- [ ] [V] Chord root and quality valid
- [ ] [T] Fragment: `\chordmode { c1 c:m c:7 c:dim7/f }`

### 23.2 Import & Export

- [ ] [I] Chord names → MEI harm (with label or extended for full data)
- [ ] [E] MEI harm → LilyPond chord mode
- [ ] [T] Roundtrip chord name fixtures
- [ ] [T] Chord mode fixtures; roundtrip

---

## Phase 24: Figured Bass Mode

### 24.1 Model & Parser

- [ ] [P] Parse `\figuremode`, `\figures`; figure list `\< ... \>`; bass figure (number, `\+`, `\!`, `/`, `\\`, brackets `[` `]`), figure space `_`
- [ ] [P] Add `FigureModeMusic`, `FigureList`, `BassFigure`, `FiguredBassModification`
- [ ] [S] Serialize figured bass
- [ ] [V] Figure numbers and modifications valid
- [ ] [T] Fragment: `\figures { <6 4> <7 5> }`, `\< 5\+ 3 \>`

### 24.2 Import & Export

- [ ] [I] Figured bass → MEI fb/f
- [ ] [E] MEI fb → LilyPond \figures
- [ ] [T] Roundtrip figured bass fixtures
- [ ] [T] Figured bass fixtures; roundtrip

---

## Phase 25: Drum Mode

### 25.1 Model & Parser

- [ ] [P] Parse `\drummode`, `\drums`; drum pitch names (e.g. snare, hihat, bassdrum) and drum chord body
- [ ] [P] Add `DrumModeMusic`, `DrumPitch`, drum event
- [ ] [S] Serialize drum mode
- [ ] [V] Drum pitch names recognized
- [ ] [T] Fragment: `\drummode { bd4 sn4 hh4 }`

### 25.2 Import & Export

- [ ] [I] Drum events → MEI percussion notation or note with @pname/@accid mapping for drums
- [ ] [E] MEI percussion → LilyPond drum mode
- [ ] [T] Roundtrip drum fixtures
- [ ] [T] Drum mode fixtures; roundtrip

---

## Phase 26: Property Operations

### 26.1 Model & Parser

- [ ] [P] Parse `\override Grob.property = value`, `\revert Grob.property`, `\set context.prop = value`, `\unset context.prop`; property path (grob path, context path)
- [ ] [P] Parse `\tweak property value` (post-event and standalone), including `\tweak #'id #"string"` for element ID retention (see "Retaining element IDs" section)
- [ ] [P] Add `Override`, `Revert`, `Set`, `Unset`, `Tweak`, `PropertyPath`, `GrobPropSpec`, `ContextPropSpec`; AST nodes that carry optional `id` (from tweak id) for export/roundtrip
- [ ] [S] Serialize override, revert, set, unset, tweak (including `\tweak #'id #"..."`)
- [ ] [V] Property path and value types valid
- [ ] [T] Fragment: `\override NoteHead.color = #red`, `\set Staff.instrumentName = "Piano"`, `c4 \tweak NoteHead.id #"mei-n1"`

### 26.2 Import & Export

- [ ] [I] Override/set → MEI scoreDef/staffDef/annot or label for roundtrip
- [ ] [E] MEI appearance/layout → LilyPond \override/\set where applicable
- [ ] [T] Roundtrip property fixtures
- [ ] [T] Property operation fixtures; roundtrip

---

## Phase 27: Header, Paper, Layout, MIDI Blocks

### 27.1 Model & Parser

- [ ] [P] Parse `\header { title = "..." composer = "..." }` (all standard fields); `\paper { ... }`, `\layout { ... }`, `\midi { ... }` with assignment lists
- [ ] [P] Add full header field set; `PaperBlock`, `LayoutBlock`, `MidiBlock` with body (assignments)
- [ ] [S] Serialize header, paper, layout, midi
- [ ] [V] Header keys and paper/layout/midi options valid
- [ ] [T] Fragment: full header; `\paper { indent = 0\mm }`

### 27.2 Import & Export

- [ ] [I] Header → MEI fileDesc/titleStmt/source; paper/layout/midi → store in context or MEI encodingDesc for roundtrip
- [ ] [E] MEI metadata → \header; encodingDesc → \paper/\layout/\midi where applicable
- [ ] [T] Roundtrip header and paper fixtures
- [ ] [T] Header and output-def fixtures; roundtrip

---

## Phase 28: Variables & Assignments

### 28.1 Model & Parser

- [ ] [P] Parse assignments: `name = { music }`, `name = \markup { }`, `name = 42`, etc.; reference identifiers in music/markup
- [ ] [P] Add `Assignment`, `IdentifierRef`; track variable scope in parser
- [ ] [S] Serialize assignments and expand or preserve variable references
- [ ] [V] Variable defined before use (or allow forward ref per grammar)
- [ ] [T] Fragment: `melody = { c4 d e f } \score { \new Staff \melody }`

### 28.2 Import & Export

- [ ] [I] Inline expanded music for MEI; optionally preserve variable names in label for roundtrip
- [ ] [E] MEI → LilyPond; optional variable extraction for repeated blocks
- [ ] [T] Roundtrip score with variables
- [ ] [T] Variable and assignment fixtures; roundtrip

---

## Phase 29: Music Functions

### 29.1 Model & Parser

- [ ] [P] Parse music function calls: `\functionName arg1 arg2`; optional args, backup/reparse for overloaded functions; partial functions with `\etc`
- [ ] [P] Add `MusicFunctionCall`, `FunctionArglist`, `PartialFunction`; represent built-in functions (e.g. `\tuplet`, `\grace`) and generic function call node
- [ ] [S] Serialize function calls and partial application
- [ ] [V] Function name known or allowed as identifier; arg count/type where specified
- [ ] [T] Fragment: `\grace c8 d4`, `\tuplet 3/2 { c8 d e }` (already covered) and generic `\someFunction arg`

### 29.2 Import & Export

- [ ] [I] Built-in music functions (tuplet, grace, etc.) already mapped; generic function call → MEI as control or annot with label
- [ ] [E] MEI → LilyPond; emit appropriate \functionName calls
- [ ] [T] Roundtrip scores using music functions
- [ ] [T] Music function fixtures; roundtrip

---

## Phase 30: Scheme Integration

### 30.1 Model & Parser

- [ ] [P] Parse `#expr` (Scheme expression): numbers, booleans, strings, lists, symbols; parse `##{ lilypond #}` embedded LilyPond
- [ ] [P] Add `SchemeExpr`, `EmbeddedLilyPond`; limit to common patterns (no full Guile)
- [ ] [S] Serialize #expr and ##{ #}
- [ ] [V] Scheme expr well-formed (balanced parens, etc.)
- [ ] [T] Fragment: `c4 #(ly:export (make-moment 1 4))`, `\override X.color = #red`

### 30.2 Import & Export

- [ ] [I] Scheme and embedded LilyPond → preserve as opaque or parse simple values for MEI
- [ ] [E] MEI → LilyPond; preserve stored Scheme/embedded where present
- [ ] [T] Roundtrip fixtures with simple Scheme
- [ ] [T] Scheme and embedded LilyPond fixtures; roundtrip

---

## Phase 31: Full LilyPond → MEI Import Pipeline

### 31.1 Import Completion

- [ ] [I] Wire all AST node types to MEI: notes, rests, chords, lyrics, figured bass, chord names, dynamics, articulations, ornaments, spanners, repeats, tuplets, grace, multi-rest, bar lines, tempo, marks, staff/voice structure, header/metadata
- [ ] [I] **Retain element IDs**: When AST has `\tweak #'id #"value"` (or equivalent) on an event/grob, set MEI `xml:id` on the corresponding created element; optionally parse `% @id value` comments and assign to the following element for roundtrip
- [ ] [I] Handle edge cases: cross-staff, multiple voices, nested repeats, nested tuplets
- [ ] [I] Use MEI extended/label patterns for concepts without direct MEI equivalent (e.g. LilyPond-specific overrides)
- [ ] [T] All fixture categories import without error; compare structure to reference MEI where available

### 31.2 Validator in Import

- [ ] [V] Run validator on AST before import; report clear errors for invalid structure
- [ ] [T] Invalid .ly files produce clear parse/validation errors
- [ ] [T] Full regression: every fixture in tests/fixtures/lilypond imports to MEI; no panics

---

## Phase 32: Full MEI → LilyPond Export & Roundtrip

### 32.1 Export Completion

- [ ] [E] Wire all MEI elements to LilyPond AST: generate idiomatic \relative where possible, proper indentation, \new Staff/Voice structure, all notation types
- [ ] [E] **Retain element IDs**: For every MEI element with `xml:id`, emit the appropriate `\tweak GrobType.id #"xml:id-value"` (see "Retaining element IDs" section; e.g. NoteHead, Rest, Slur, Tie, Hairpin, DynamicText, RehearsalMark, etc.). Optionally emit `% @id value` comments for robustness on re-import
- [ ] [E] Preserve roundtrip data from import (labels, extended) so LilyPond → MEI → LilyPond matches where intended
- [ ] [T] Export all fixture MEI (from Phase 31) back to .ly; compare to original or validate with parser
- [ ] [T] Roundtrip ID test: MEI with xml:id on notes/rests/slurs → LilyPond → MEI; verify same xml:id values on corresponding elements

### 32.2 Roundtrip Tests

- [ ] [T] Define roundtrip test levels (e.g. LilyPond → MEI → LilyPond parse equivalence; structural comparison)
- [ ] [T] Add roundtrip tests in `crates/formats/lilypond/tests/roundtrip.rs` for representative fixtures
- [ ] [T] All 32 phase fixtures pass roundtrip (or documented exceptions)

### 32.3 Documentation & Script

- [ ] [T] Update any docs referencing LilyPond support; ensure `tusk_lilypond_coverage.sh` runs and finds tasks
- [ ] [T] Final full test run: `cargo test`, `cargo clippy --all-targets` pass

---

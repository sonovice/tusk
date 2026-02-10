# LilyPond Coverage Tasks

Tasks for achieving full LilyPond ↔ MEI bidirectional conversion coverage, with a parser that validates against the entire LilyPond grammar (specs/lilypond/repo/lily/parser.yy). Coverage aims to exceed the existing musicxml2ly converter.

Each task covers: `[L]` Lexer, `[P]` Parser, `[S]` Serializer, `[I]` Import (LilyPond→MEI), `[E]` Export (MEI→LilyPond), `[T]` Tests, `[V]` Validator.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: The `tusk_lilypond_coverage.sh` script runs tests/clippy, feeds results + this task list to Claude, which works on one section at a time.

**Constraint**: Every change must pass `cargo test` and `cargo clippy --all-targets` with no regressions.

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

- [ ] [P] Add `ContextPrefix`, `ContextModification`, `ContextDefSpec`, `OptionalContextMods`; parse `\new Staff`, `\new Voice = "name"`, `\context "Staff"`, `\change Staff = "name"`, `\with { ... }`
- [ ] [P] Parse context modifier list in `\with { \consists ..., \remove ..., \override ..., \set ..., etc. }`
- [ ] [S] Serialize context prefix and `\with` blocks
- [ ] [V] Context names and types consistent
- [ ] [T] Parse `\score { \new Staff { c4 } }` and `\new PianoStaff << \new Staff { } \new Staff { } >>`

### 5.2 Import & Export

- [ ] [I] `\new Staff` / `\new Voice` → MEI staff/voice structure; map context type to MEI staffDef/scoreDef
- [ ] [I] `\with { }` overrides → store in conversion context or MEI extensions for roundtrip
- [ ] [E] MEI staff/part → `\new Staff` / `\new Voice` with optional `\with`
- [ ] [T] Roundtrip score with multiple staves
- [ ] [T] Piano-style score fixture; roundtrip

---

## Phase 6: Clefs, Key Signatures, Time Signatures

### 6.1 Model & Parser

- [ ] [P] Parse `\clef "treble"` (and other clef names), `\key pitch \mode`, `\time n/m` (and compound, e.g. `\time 2+3/8`)
- [ ] [P] Add AST nodes: `Clef`, `KeySignature`, `TimeSignature`
- [ ] [S] Serialize clef, key, time
- [ ] [V] Key pitch and mode valid; time numerator/denominator valid
- [ ] [T] Fragment: `\clef bass \key d \minor \time 3/4`

### 6.2 Import & Export

- [ ] [I] Clef/key/time → MEI `<scoreDef>` / `<staffDef>` clef, key, meter
- [ ] [E] MEI clef/key/meter → `\clef`, `\key`, `\time`
- [ ] [T] Roundtrip with key and time change
- [ ] [T] Fixtures with various clefs, keys, times; roundtrip

---

## Phase 7: Relative Pitch, Transpose, Octave Check

### 7.1 Model & Parser

- [ ] [P] Parse `\relative c' { ... }` and `\relative pitch` (optional start pitch); parse `\transpose from to { ... }`
- [ ] [P] Parse octave check `= '` / `= ,` after pitch
- [ ] [S] Serialize `\relative`, `\transpose`, octave check
- [ ] [V] Relative start pitch and transpose from/to valid
- [ ] [T] Fragment: `\relative c'' { c d e f }` and `\transpose c d { c4 }`

### 7.2 Import & Export

- [ ] [I] Relative/transpose → MEI as written (or expand to absolute); store relative/transpose in context for roundtrip
- [ ] [E] When exporting, prefer `\relative` when all notes in a voice can be expressed relative to a single reference
- [ ] [T] Roundtrip relative-mode score
- [ ] [T] Relative and transpose fixtures; roundtrip

---

## Phase 8: Angle-Bracket Chords

### 8.1 Model & Parser

- [ ] [P] Parse chord body `< ... >`: multiple pitches with optional accidentals/octave marks, shared duration, optional post-events
- [ ] [P] Add `ChordEvent`, `ChordBody`, chord body elements (pitch, drum pitch, post-event, function call)
- [ ] [S] Serialize `< c e g >` with correct spacing and shared duration
- [ ] [V] Chord has at least one pitch; duration consistent
- [ ] [T] Fragment: `<c e g>4`, `<c es g>2.`

### 8.2 Import & Export

- [ ] [I] Chord → MEI chord (multiple note elements with same @dur, chord attribute)
- [ ] [E] MEI chord → LilyPond `< ... >` chord
- [ ] [T] Roundtrip chord fixture
- [ ] [T] Chord fixtures; roundtrip

---

## Phase 9: Ties, Slurs, Phrasing Slurs

### 9.1 Model & Parser

- [ ] [P] Parse tie `~`, slur `( ... )`, phrasing slur `\( ... \)` as post-events or event identifiers
- [ ] [P] Add `TieEvent`, `SlurEvent`, `PhrasingSlurEvent` (or generic span events with type)
- [ ] [S] Serialize tie, slur, phrasing slur
- [ ] [V] Slur/phrasing slur start/stop match
- [ ] [T] Fragment: `c4~ c`, `c4( d e f)`, `c4\( d e\)`

### 9.2 Import & Export

- [ ] [I] Tie → MEI `<tie>`; slur → MEI `<slur>`; phrasing slur → MEI `<phrase>` or equivalent
- [ ] [E] MEI tie/slur/phrase → LilyPond `~`, `( )`, `\( \)`
- [ ] [T] Roundtrip tied and slurred phrases
- [ ] [T] Tie, slur, phrasing slur fixtures; roundtrip

---

## Phase 10: Beaming

### 10.1 Model & Parser

- [ ] [P] Parse beam start/end `[` and `]` (as event identifiers or post-events); parse `\autoBeamOn`, `\autoBeamOff`
- [ ] [P] Add `BeamEvent`, auto-beam setting in context
- [ ] [S] Serialize explicit beams and auto-beam commands
- [ ] [V] Beam brackets balanced
- [ ] [T] Fragment: `c8[ d e f]`, `\autoBeamOff c8 d e f`

### 10.2 Import & Export

- [ ] [I] Beams → MEI `<beam>` or beam span; auto-beam → context
- [ ] [E] MEI beam → LilyPond `[ ]` or auto-beam
- [ ] [T] Roundtrip beamed passage
- [ ] [T] Beam fixtures; roundtrip

---

## Phase 11: Dynamics & Hairpins

### 11.1 Model & Parser

- [ ] [P] Parse dynamics: `\p`, `\f`, `\ff`, `\sfz`, `\fp`, etc.; hairpins `\<`, `\>`, `\!`
- [ ] [P] Add `DynamicsEvent`, `HairpinEvent` (cresc/dim)
- [ ] [S] Serialize dynamics and hairpins
- [ ] [V] Dynamic script and hairpin direction valid
- [ ] [T] Fragment: `c4\f d\p e\< f g\!\ff`

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

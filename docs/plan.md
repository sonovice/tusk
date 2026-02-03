# Tusk: The Ultimate Score Konverter

## Overview

Bidirectional MusicXML ↔ MEI converter in Rust with MEI as the canonical internal model.

**Primary targets**: MEI 5.1, MusicXML 4.0

**Backwards compatibility**:
- MEI: 5.0, 4.0.1, 3.0.0 (version detection + migration)
- MusicXML: 3.1, 3.0, 2.0 (version detection + upgrade on load)

**Scope**: Full mei-all coverage (all 29 modules)

**Key principles**:
- MusicXML → MEI is lossless; MEI → MusicXML is lossy (MEI-specific features documented)
- Test-driven development: tests written before implementation
- Streaming I/O: must handle 100+ MB files (long operas) without loading entire DOM into memory

---

## Project Structure

```
tusk/
├── Cargo.toml                        # Workspace root
├── crates/
│   ├── bindings/
│   │   ├── cli/                      # CLI tool (clap, indicatif)
│   │   └── wasm/                     # WASM bindings (wasm-bindgen)
│   ├── core/
│   │   ├── model/                    # Internal MEI-based model (version-agnostic)
│   │   └── convert/                  # Conversion layer (MusicXML ↔ MEI)
│   └── formats/
│       ├── mei/                      # MEI parsing/serialization
│       │   └── src/versions/         # Version-specific (5.1, 5.0, 4.0.1, 3.0.0)
│       └── musicxml/                 # MusicXML parsing/serialization
│           └── src/versions/         # Version-specific (4.0, 3.1, 3.0, 2.0)
├── docs/                             # Requirements & architecture docs
└── tests/                            # Integration & round-trip tests
    └── fixtures/                     # Test files for each version
```

---

## Implementation Phases

### Phase 1: Foundation & Code Generation
- Cargo workspace setup with all crates
- Requirements documentation in `docs/requirements.md`
- **MEI ODD code generator** (`tools/mei-codegen/`): Parse ODD specs → generate Rust types
- Generate core model from `specs/mei/modules/*.xml`:
  - Data types from `macroSpec` → Rust enums
  - Attribute classes from `classSpec type="atts"` → Rust structs
  - Elements from `elementSpec` → Rust structs with attribute class composition
- Unit test infrastructure with coverage

### Phase 2: MEI CMN Module
- Full CMN model: note, rest, chord, measure, staff, layer
- scoreDef/staffDef/layerDef
- Control events (slur, tie, dynam, hairpin, dir, tempo, fermata)
- Beams, tuplets, grace notes
- MEI CMN parsing/serialization
- Tests against `specs/mei/examples/`

### Phase 3: MEI Header & Metadata
- Complete meiHead: fileDesc, encodingDesc, workList, revisionDesc
- sourceDesc, manifestationList
- All header-related parsing/serialization
- Metadata round-trip tests

### Phase 4: MusicXML ↔ MEI CMN
- MusicXML 4.0 intermediate model
- MusicXML parser (score-partwise and score-timewise)
- MusicXML → MEI conversion (lossless)
- MEI → MusicXML conversion (with documented limitations)
- Round-trip tests: MusicXML → MEI → MusicXML

### Phase 5: Extended MEI Modules (all mei-all modules)

**Repertoire-specific:**
- Mensural notation (MEI.mensural)
- Neume notation (MEI.neumes)
- Tablature (MEI.stringtab)

**Music content:**
- Lyrics (MEI.lyrics)
- Harmony (MEI.harmony)
- Fingering (MEI.fingering)
- CMN ornaments (MEI.cmnOrnaments) - if not fully covered in Phase 2

**Analysis & editorial:**
- Analysis (MEI.analytical)
- Editorial/transcription (MEI.edittrans)
- Critical apparatus (MEI.critapp)
- Genetic encoding (MEI.genetic)

**Performance & gestural:**
- Performance (MEI.performance)
- Gestural (MEI.gestural) - velocity, timing, performed pitch
- MIDI (MEI.midi)

**Visual & symbols:**
- Visual (MEI.visual) - rendering hints, layout
- External symbols (MEI.externalsymbols) - SMuFL references
- User symbols (MEI.usersymbols)

**Facsimile & sources:**
- Facsimile (MEI.facsimile)
- Manuscript description (MEI.msDesc)
- FRBR (MEI.frbr) - bibliographic relationships

**Text & metadata:**
- Text (MEI.text) - prose, verse, front/back matter
- Names and dates (MEI.namesdates)
- Pointers/references (MEI.ptrref)
- Figures and tables (MEI.figtable)

**Document organization:**
- Corpus (MEI.corpus) - grouping multiple MEI documents
- Drama (MEI.drama) - stage directions, spoken text

### Phase 6: Advanced Features
- Compressed MusicXML (.mxl) support
- CLI tool with format detection
- Batch conversion
- Validation against schemas

### Phase 7: Version Compatibility
- MEI version detection and migration (5.0 → 5.1, 4.0.1 → 5.1, etc.)
- MusicXML version detection and upgrade (2.0 → 4.0, 3.x → 4.0)
- Version-specific export (MEI 5.0, MusicXML 3.1 targets)
- Cross-version round-trip tests

### Phase 8: Polish & Coverage
- 100% test coverage verification
- Property-based tests for all modules
- Performance benchmarks
- Complete API documentation

---

## Key Design Decisions

### 1. MEI as Canonical Model (1:1 Mapping)

**Core principle**: Internal Rust types map 1:1 to MEI elements and attribute classes. This is not an abstraction or interpretation of MEI—it IS MEI in Rust form.

**Naming convention**:
- MEI element `<note>` → Rust struct `Note`
- MEI attribute class `att.duration.log` → Rust struct `AttDurationLog`
- MEI data type `data.DURATION` → Rust enum `DataDuration`

**Why this matters**: Loading MEI is direct deserialization (XML → Rust structs). Saving MEI is direct serialization (Rust structs → XML). No transformation logic needed.

**Code generation from MEI ODD**: The model should be generated from the MEI ODD specification (`mei-all.odd`) to ensure perfect 1:1 correspondence. Manual maintenance risks drift.

**MEI Attribute Class Hierarchy**: MEI organizes attributes into semantic domains:
- **Base classes**: Core attributes (att.common, att.linking, att.typed)
- **Logical (.log)**: Written/notated values — what's on the page (e.g., `dur`, `pname`, `oct`, `accid`)
- **Visual (.vis)**: Rendering/appearance — how it looks (e.g., `color`, `place`, `stem.dir`)
- **Gestural (.ges)**: Performed/sounding values — how it sounds (e.g., `dur.ges`, `pname.ges`, `accid.ges`)
- **Analytical (.anl)**: Analysis/interpretation — what it means (e.g., `deg`, `pclass`)

This domain separation is critical for accurate conversion. For example, `accid` (written accidental on the page) differs from `accid.ges` (sounding accidental after key signature application). The internal model preserves this distinction.

### 2. Separate MusicXML Intermediate Model
Parse MusicXML into its own types first, then convert. Cleaner debugging and enables MusicXML-specific validation.

### 3. Chunked Processing by `<mdiv>` (Critical for Large Files)
MEI files can exceed 100 MB for long operas. Process by musical division:

```
<mei>
  <meiHead>...</meiHead>        <!-- always load fully (small) -->
  <music>
    <body>
      <mdiv>...</mdiv>          <!-- chunk 1: load, process, release -->
      <mdiv>...</mdiv>          <!-- chunk 2: load, process, release -->
    </body>
  </music>
</mei>
```

**Architecture**:
- `quick-xml` pull parser (streaming, no DOM)
- `MeiReader` yields `<mdiv>` elements one at a time (iterator pattern)
- `MeiWriter` accepts `<mdiv>` stream, writes incrementally
- Header (`meiHead`) always fully loaded for cross-references (scoreDef, staffDef)

**API design**:
```rust
// Full load (small files)
let doc = MeiDocument::from_reader(file)?;

// Streaming (large files)
let mut reader = MeiReader::new(file)?;
let header = reader.header();  // always available
for mdiv in reader.mdivs() {   // lazy iteration
    process(&mdiv)?;
}
```

**Memory budget**: Single `<mdiv>` in memory at a time, target <100 MB RAM for any file size

**MusicXML handling**:
- MusicXML files are typically small (single movement) → load fully
- MEI → MusicXML: each `<mdiv>` outputs as separate `.musicxml` file
- MusicXML → MEI: each input file becomes one `<mdiv>` (batch import can merge into multi-`<mdiv>` MEI)

### 4. Conversion Context
State object tracks divisions, pending ties/slurs, ID mappings during conversion.

### 5. Version Abstraction
- Internal model always represents latest version (MEI 5.1)
- Version-specific parsers detect and upgrade older formats on load
- Export can target specific versions (with feature downgrade warnings)
- Migration logic isolated in version-specific modules

### 6. Cargo Workspace Configuration

**Root `Cargo.toml`**:
```toml
[workspace]
members = [
    "crates/core/model",
    "crates/core/convert",
    "crates/formats/mei",
    "crates/formats/musicxml",
    "crates/bindings/cli",
    "crates/bindings/wasm",
]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2024"
authors = ["Simon Waloschek"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/..."

[workspace.dependencies]
# Internal crates
model = { path = "crates/core/model" }
convert = { path = "crates/core/convert" }
mei = { path = "crates/formats/mei" }
musicxml = { path = "crates/formats/musicxml" }

# External (cargo add to update)
quick-xml = "..."
thiserror = "..."
# ...
```

**Sub-crate `Cargo.toml`** (e.g., `crates/formats/musicxml/Cargo.toml`):
```toml
[package]
name = "musicxml"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[dependencies]
model = { workspace = true }
quick-xml = { workspace = true }
thiserror = { workspace = true }
```

---

## Policies & Conventions

### Validation
- **Development/Testing**: Use `xmllint` for schema validation
  - MEI: `xmllint --relaxng specs/mei/validation/mei-all.rng`
  - MusicXML: `xmllint --schema specs/musicxml/schema/musicxml.xsd`
- **Production/WASM**: No schema validation (parser-as-validator)
  - If document parses into typed model, it's structurally valid
  - Schema validation is external tooling concern
- **Input**: Lenient by default, strict mode available via CLI flag

### ID Handling
- Preserve IDs wherever possible during conversion
- MEI uses `xml:id`, MusicXML uses various ID attributes
- Generate deterministic IDs when originals unavailable (hash-based or sequential)

### Unknown Elements/Attributes
- Warn on encounter
- Drop if they would cause output validation to fail
- Preserve in round-trip if target format supports them

### Namespaces
- Full namespace support for MEI extensions and custom namespaces
- Proper prefix handling in serialization

### Known Limitations
- MEI → MusicXML is inherently lossy (MEI-specific features have no MusicXML equivalent)
- Lost features documented per element (e.g., facsimile links, editorial markup)
- No automatic "best effort" guessing for ambiguous mappings

### Build Targets
- Native: Linux, macOS, Windows
- **WASM**: Browser target supported (`wasm32-unknown-unknown`)
  - No filesystem access in WASM (use in-memory buffers)
  - Feature flag: `--features wasm`

---

## Core Dependencies

Use `cargo add <crate>` to install latest versions.

| Crate | Purpose |
|-------|---------|
| `quick-xml` | XML parsing (streaming) |
| `thiserror` | Error types (libraries) |
| `anyhow` | Error handling (CLI/apps) |
| `smallvec` | Inline vectors |
| `indexmap` | Order-preserving maps |
| `tracing` | Structured logging |
| `zip` | .mxl support (compressed MusicXML) |
| `rayon` | Parallel batch conversion |
| `derive_more` | Reduce derive boilerplate |

**CLI only**:
| `clap` | Argument parsing |
| `indicatif` | Progress bars |

**Dev dependencies**:
| `proptest` | Property tests |
| `insta` | Snapshot tests |
| `criterion` | Benchmarks |

**WASM target** (optional):
| `wasm-bindgen` | JS interop |

---

## Testing Strategy (TDD)

**Red-Green-Refactor cycle for every feature**:
1. Write failing test first (including edge cases)
2. Implement minimal code to pass
3. Refactor while keeping tests green

**Test types**:
- **Unit tests**: 100% coverage for core model, written before implementation
- **Integration tests**: Parse/serialize real-world files from `specs/`
- **Round-trip tests**: MusicXML → MEI → MusicXML preserves all MusicXML content
- **Property tests**: Arbitrary model generation + serialization round-trips
- **Chunked processing tests**: Verify constant memory usage (process 500 MB file in <100 MB RAM)
- **Snapshot tests**: `insta` for complex serialization output

**Test fixtures**:
- Small synthetic files for unit tests (single `<mdiv>`)
- Real-world MEI/MusicXML from `specs/` for integration
- Multi-`<mdiv>` files for chunked iteration tests
- Large opera scores (or synthetic large files) for memory profiling

---

## Critical Files to Modify/Create

1. `Cargo.toml` - Workspace configuration
2. `crates/core/model/` - MEI-mirroring internal model
3. `crates/core/convert/` - Conversion logic
4. `crates/formats/mei/src/parser/` - MEI XML parser
5. `crates/formats/mei/src/serializer/` - MEI XML serializer
6. `crates/formats/musicxml/src/model/` - MusicXML intermediate types
7. `crates/formats/musicxml/src/parser/` - MusicXML parser
8. `crates/bindings/cli/` - CLI tool
9. `crates/bindings/wasm/` - WASM bindings
10. `docs/requirements.md` - Full requirements documentation

---

## Verification Plan

1. Parse sample MEI files from `specs/mei/examples/`
2. Parse MusicXML test files
3. Run round-trip: MusicXML → MEI → MusicXML, verify no data loss
4. Run `cargo test` with coverage (`cargo tarpaulin`)
5. Verify 100% test coverage

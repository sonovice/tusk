# Tusk Requirements Specification

## 1. Purpose

Tusk is a bidirectional MusicXML ↔ MEI converter written in Rust. It enables lossless conversion from MusicXML to MEI and documented lossy conversion from MEI to MusicXML.

## 2. Scope

### 2.1 In Scope

- Parse and serialize MEI 5.1, 5.0, 4.0.1, 3.0.0
- Parse and serialize MusicXML 4.0, 3.1, 3.0, 2.0
- Full mei-all coverage (all 29 modules)
- Streaming I/O for large files (100+ MB)
- CLI tool for command-line usage
- WASM bindings for browser usage
- Compressed MusicXML (.mxl) support

### 2.2 Out of Scope

- Audio/MIDI playback
- Music rendering/notation display
- Music composition/editing UI
- Optical Music Recognition (OMR)

---

## 3. Functional Requirements

### 3.1 Format Support

#### FR-3.1.1 MEI Format Support
- **Primary**: MEI 5.1 (internal model)
- **Read**: MEI 5.1, 5.0, 4.0.1, 3.0.0
- **Write**: MEI 5.1, 5.0 (version-specific export)
- **Modules**: All 29 mei-all modules

| Module | Description |
|--------|-------------|
| MEI.cmn | Common Music Notation |
| MEI.header | Document metadata |
| MEI.shared | Shared elements |
| MEI.mensural | Mensural notation |
| MEI.neumes | Neume notation |
| MEI.stringtab | Tablature |
| MEI.lyrics | Lyrics and syllables |
| MEI.harmony | Chord symbols, figured bass |
| MEI.fingering | Fingering indications |
| MEI.cmnOrnaments | CMN ornaments |
| MEI.analytical | Analysis attributes |
| MEI.edittrans | Editorial/transcription markup |
| MEI.critapp | Critical apparatus |
| MEI.genetic | Genetic encoding |
| MEI.performance | Performance metadata |
| MEI.gestural | Performed values |
| MEI.midi | MIDI data |
| MEI.visual | Rendering hints |
| MEI.externalsymbols | SMuFL references |
| MEI.usersymbols | User-defined symbols |
| MEI.facsimile | Facsimile links |
| MEI.msDesc | Manuscript description |
| MEI.frbr | Bibliographic relationships |
| MEI.text | Text content |
| MEI.namesdates | Names and dates |
| MEI.ptrref | Pointers and references |
| MEI.figtable | Figures and tables |
| MEI.corpus | Document collections |
| MEI.drama | Drama/stage directions |

#### FR-3.1.2 MusicXML Format Support
- **Primary**: MusicXML 4.0
- **Read**: MusicXML 4.0, 3.1, 3.0, 2.0
- **Write**: MusicXML 4.0, 3.1 (version-specific export)
- **Structures**: score-partwise, score-timewise
- **Compression**: .musicxml (uncompressed), .mxl (compressed)

### 3.2 Conversion

#### FR-3.2.1 MusicXML → MEI Conversion
- Shall be lossless (all MusicXML content preserved)
- Shall produce valid MEI 5.1 output
- Shall preserve IDs where possible
- Shall generate deterministic IDs when originals unavailable

#### FR-3.2.2 MEI → MusicXML Conversion
- Shall produce valid MusicXML 4.0 output
- Shall document all MEI features lost in conversion
- Shall preserve IDs where possible
- Shall not attempt "best effort" guessing for ambiguous mappings

#### FR-3.2.3 Version Migration
- Shall auto-detect input version
- Shall upgrade older versions to internal model on load
- Shall support export to specific target versions
- Shall warn when features unavailable in target version

### 3.3 Processing

#### FR-3.3.1 Streaming I/O
- Shall process files >100 MB without loading entire DOM
- Shall use chunked processing by `<mdiv>` for MEI
- Shall maintain <100 MB RAM usage for any file size
- Shall support iterator-based `<mdiv>` access

#### FR-3.3.2 Batch Processing
- Shall support converting multiple files in one invocation
- Shall support parallel processing of independent files
- Shall report progress for batch operations

### 3.4 Validation

#### FR-3.4.1 Schema Validation (Development/Testing)
- Shall validate MEI against mei-all.rng schema
- Shall validate MusicXML against musicxml.xsd schema
- Shall use xmllint for validation

#### FR-3.4.2 Structural Validation (Production)
- Shall accept any document that parses into typed model
- Shall provide strict mode via CLI flag
- Shall be lenient by default

### 3.5 User Interfaces

#### FR-3.5.1 CLI Tool
- Shall support single file conversion
- Shall support batch conversion
- Shall auto-detect input format
- Shall display progress bars for long operations
- Shall support format/version selection flags

#### FR-3.5.2 WASM Bindings
- Shall expose conversion functions to JavaScript
- Shall work with in-memory buffers (no filesystem)
- Shall be buildable with `--features wasm`

---

## 4. Non-Functional Requirements

### 4.1 Performance

#### NFR-4.1.1 Memory Efficiency
- SHALL maintain <100 MB RAM usage for files of any size
- SHALL use streaming parsers (no full DOM load)
- SHALL release `<mdiv>` memory after processing

#### NFR-4.1.2 Processing Speed
- SHALL process typical scores (<10 MB) in under 1 second
- SHALL provide benchmarks for performance tracking

### 4.2 Reliability

#### NFR-4.2.1 Error Handling
- SHALL provide clear, actionable error messages
- SHALL include file location (line/column) in parse errors
- SHALL use thiserror for library error types
- SHALL use anyhow for CLI error handling

#### NFR-4.2.2 Round-Trip Fidelity
- SHALL preserve all MusicXML content in MusicXML→MEI→MusicXML round-trip
- SHALL document any content changes in conversion notes

### 4.3 Portability

#### NFR-4.3.1 Platform Support
- SHALL build on Linux, macOS, Windows
- SHALL build for wasm32-unknown-unknown target
- SHALL have no platform-specific dependencies in core

### 4.4 Maintainability

#### NFR-4.4.1 Code Generation
- SHALL generate MEI model from ODD specification
- SHALL ensure 1:1 mapping between MEI elements and Rust types
- SHALL regenerate code when MEI spec updates

#### NFR-4.4.2 Testing
- SHALL follow TDD (test-first) methodology
- SHALL achieve 100% test coverage for core model
- SHALL include unit, integration, and property tests

#### NFR-4.4.3 Documentation
- SHALL document all public APIs with rustdoc
- SHALL document conversion limitations
- SHALL maintain architecture documentation

---

## 5. Data Requirements

### 5.1 Internal Model

The internal model SHALL:
- Map 1:1 to MEI 5.1 elements and attribute classes
- Preserve MEI's semantic domain separation:
  - Logical (.log): written/notated values
  - Visual (.vis): rendering/appearance
  - Gestural (.ges): performed/sounding values
  - Analytical (.anl): analysis/interpretation
- Be version-agnostic (represent latest MEI version)

### 5.2 ID Handling

- SHALL preserve xml:id from MEI documents
- SHALL preserve id attributes from MusicXML documents
- SHALL generate deterministic IDs when not present (hash-based or sequential)
- SHALL maintain ID cross-references during conversion

### 5.3 Namespace Handling

- SHALL support MEI namespace (http://www.music-encoding.org/ns/mei)
- SHALL support custom namespaces and extensions
- SHALL preserve namespace prefixes in serialization

---

## 6. Interface Requirements

### 6.1 Library API

```rust
// Full document load (small files)
let doc = MeiDocument::from_reader(file)?;
let doc = MusicXmlDocument::from_reader(file)?;

// Streaming load (large files)
let mut reader = MeiReader::new(file)?;
let header = reader.header();
for mdiv in reader.mdivs() {
    process(&mdiv)?;
}

// Conversion
let mei = MusicXmlToMei::convert(&musicxml_doc)?;
let musicxml = MeiToMusicXml::convert(&mei_doc)?;

// Serialization
mei_doc.write_to(&mut writer)?;
musicxml_doc.write_to(&mut writer)?;
```

### 6.2 CLI Interface

```
tusk convert <input> [output]
    --from <format>     Input format (mei|musicxml|auto)
    --to <format>       Output format (mei|musicxml)
    --version <ver>     Target version (e.g., mei-5.0, musicxml-3.1)
    --strict            Enable strict validation
    --quiet             Suppress progress output

tusk batch <input-dir> <output-dir>
    --from <format>     Input format filter
    --to <format>       Output format
    --parallel <n>      Number of parallel workers

tusk validate <input>
    --schema <path>     Custom schema path
```

### 6.3 WASM Interface

```typescript
// JavaScript/TypeScript API
import { convertMusicXmlToMei, convertMeiToMusicXml } from 'tusk';

const meiXml: string = convertMusicXmlToMei(musicXmlString);
const musicXml: string = convertMeiToMusicXml(meiString);
```

---

## 7. Quality Requirements

### 7.1 Test Coverage

| Component | Required Coverage |
|-----------|-------------------|
| Core model | 100% |
| MEI parser | 95%+ |
| MusicXML parser | 95%+ |
| Conversion logic | 95%+ |
| CLI | 80%+ |

### 7.2 Test Types

- **Unit tests**: All data types, attribute classes, elements
- **Integration tests**: Real-world files from specs/
- **Round-trip tests**: MusicXML → MEI → MusicXML
- **Property tests**: Arbitrary model generation + serialization
- **Snapshot tests**: Complex serialization outputs
- **Memory tests**: Verify constant memory for large files

---

## 8. Constraints

### 8.1 Technical Constraints

- Rust 2024 edition
- No unsafe code in core libraries (unsafe allowed in performance-critical paths with justification)
- Dependencies must support WASM target (for wasm feature)

### 8.2 Process Constraints

- Test-driven development (tests before implementation)
- All changes must pass CI (build, test, clippy, fmt)
- Documentation required for public APIs

---

## 9. Glossary

| Term | Definition |
|------|------------|
| MEI | Music Encoding Initiative - XML format for music notation |
| MusicXML | XML interchange format for music notation |
| ODD | One Document Does it all - TEI/MEI schema specification format |
| CMN | Common Music Notation - standard Western music notation |
| mdiv | Musical division - top-level structural unit in MEI |
| att class | MEI attribute class - reusable attribute group |
| .log | Logical domain - written/notated values |
| .vis | Visual domain - rendering/appearance hints |
| .ges | Gestural domain - performed/sounding values |
| .anl | Analytical domain - analysis/interpretation |
| SMuFL | Standard Music Font Layout - music symbol specification |
| FRBR | Functional Requirements for Bibliographic Records |

---

## 10. References

- [MEI Guidelines](https://music-encoding.org/guidelines/)
- [MusicXML Specification](https://www.w3.org/2021/06/musicxml40/)
- [MEI ODD Specification](https://music-encoding.org/schema/)
- [quick-xml Crate](https://docs.rs/quick-xml/)

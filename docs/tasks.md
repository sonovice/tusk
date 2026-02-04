# Tusk Development Tasks

Tasks for implementing the MusicXML ↔ MEI converter. Each task should be completed using TDD (test first, then implement).

**Legend**: `[ ]` = pending, `[x]` = done

---

## Phase 1: Foundation & Code Generation

### 1.1 Project Setup
- [x] Create Cargo workspace with all crates structure
- [x] Add requirements documentation in `docs/requirements.md`
- [x] Add project plan in `docs/plan.md`

### 1.2 MEI Code Generator (tools/mei-codegen)
- [x] Create MEI ODD parser for `specs/mei/modules/*.xml`
- [x] Generate data types from `macroSpec type="dt"` → Rust enums
- [x] Generate attribute classes from `classSpec type="atts"` → Rust structs
- [x] Generate model classes from `classSpec type="model"`
- [x] Generate pattern entities from `macroSpec type="pe"`
- [x] Generate elements from `elementSpec` → Rust structs with children
- [x] Parse Schematron constraints from `constraintSpec`
- [x] Generate validation framework from constraints
- [x] Output generated code to `crates/core/model/src/generated/`

### 1.3 Core Model Setup
- [x] Create `crates/core/model/` crate with Cargo.toml
- [x] Add re-exports and public API in `crates/core/model/src/lib.rs`
- [x] Verify generated types compile with `cargo build`
- [x] Add basic unit tests for generated data types
- [x] Add basic unit tests for generated attribute classes
- [x] Add basic unit tests for generated element structs

### 1.4 Serialization Infrastructure
- [x] Add `quick-xml` dependency for XML parsing/serialization
- [x] Implement custom serde serializer for MEI XML format
- [x] Implement custom serde deserializer for MEI XML format
- [x] Handle `xml:id` attribute serialization correctly
- [x] Handle namespace declarations and prefixes
- [x] Test round-trip serialization for simple elements

---

## Phase 2: MEI CMN Module

### 2.1 Core Note Elements
- [x] Implement MEI parsing for `<note>` element
- [x] Implement MEI parsing for `<rest>` element
- [x] Implement MEI parsing for `<chord>` element
- [x] Implement MEI parsing for `<space>` element
- [x] Add tests against `specs/mei/examples/` CMN files

### 2.2 Structural Elements
- [x] Implement MEI parsing for `<measure>` element
- [x] Implement MEI parsing for `<staff>` element
- [x] Implement MEI parsing for `<layer>` element
- [x] Implement MEI parsing for `<section>` element
- [x] Implement MEI parsing for `<mdiv>` element
- [x] Add structural hierarchy tests

### 2.3 Definition Elements
- [x] Implement MEI parsing for `<scoreDef>` element
- [x] Implement MEI parsing for `<staffDef>` element
- [x] Implement MEI parsing for `<layerDef>` element
- [x] Implement MEI parsing for `<staffGrp>` element
- [x] Add score definition tests

### 2.4 Control Events
- [x] Implement MEI parsing for `<slur>` element
- [x] Implement MEI parsing for `<tie>` element
- [x] Implement MEI parsing for `<dynam>` element
- [x] Implement MEI parsing for `<hairpin>` element
- [x] Implement MEI parsing for `<dir>` element
- [x] Implement MEI parsing for `<tempo>` element
- [x] Implement MEI parsing for `<fermata>` element
- [x] Add control event tests

### 2.5 Grouping Elements
- [x] Implement MEI parsing for `<beam>` element
- [x] Implement MEI parsing for `<tuplet>` element
- [x] Implement MEI parsing for `<graceGrp>` element
- [x] Add beam and tuplet tests

### 2.6 MEI CMN Serialization
- [x] Implement MEI serialization for all CMN elements
- [x] Add round-trip tests (parse → serialize → parse)
- [x] Validate output against `mei-all.rng` schema

---

## Phase 3: MEI Header & Metadata

### 3.1 File Description
- [x] Implement MEI parsing for `<meiHead>` element
- [x] Implement MEI parsing for `<fileDesc>` element
- [x] Implement MEI parsing for `<titleStmt>` element
- [x] Implement MEI parsing for `<pubStmt>` element
- [x] Implement MEI parsing for `<sourceDesc>` element

### 3.2 Encoding Description
- [x] Implement MEI parsing for `<encodingDesc>` element
- [x] Implement MEI parsing for `<appInfo>` element
- [x] Implement MEI parsing for `<editorialDecl>` element
- [x] Implement MEI parsing for `<projectDesc>` element

### 3.3 Work Description
- [x] Implement MEI parsing for `<workList>` element
- [x] Implement MEI parsing for `<work>` element
- [x] Implement MEI parsing for `<expressionList>` element

### 3.4 Revision Description
- [x] Implement MEI parsing for `<revisionDesc>` element
- [x] Implement MEI parsing for `<change>` element

### 3.5 Header Serialization
- [x] Implement MEI serialization for all header elements
- [x] Add header round-trip tests
- [x] Validate header output against schema

---

## Phase 4: MusicXML ↔ MEI CMN Conversion

### 4.1 MusicXML Intermediate Model
- [x] Create MusicXML 4.0 data types in `crates/formats/musicxml`
- [x] Create MusicXML element structs (score-partwise structure)
- [x] Create MusicXML element structs (part-list, part, measure)
- [x] Create MusicXML note/rest/chord types
- [x] Create MusicXML attributes types (divisions, key, time, clef)
- [x] Create MusicXML direction types

### 4.2 MusicXML Parser
- [x] Implement MusicXML parser for score-partwise documents
- [x] Implement MusicXML parser for score-timewise documents (convert to partwise)
- [x] Handle MusicXML divisions and duration calculation
- [x] Parse MusicXML note elements with pitch/duration
- [x] Parse MusicXML rest elements
- [x] Parse MusicXML chord notation (backup/forward)
- [x] Parse MusicXML attributes (key, time, clef, divisions)
- [x] Parse MusicXML direction elements
- [x] Add MusicXML parser tests

### 4.3 MusicXML → MEI Conversion
- [x] Create conversion context (state tracking, ID mapping)
- [x] Convert MusicXML score-partwise to MEI structure
- [x] Convert MusicXML part-list to MEI staffGrp/staffDef
- [x] Convert MusicXML measure to MEI measure
- [x] Convert MusicXML note to MEI note (pitch, duration, accidentals)
- [x] Convert MusicXML rest to MEI rest
- [x] Convert MusicXML chord to MEI chord
- [x] Convert MusicXML attributes to MEI scoreDef/staffDef
- [x] Convert MusicXML directions to MEI control events
- [x] Handle MusicXML divisions → MEI duration conversion
- [x] Add MusicXML → MEI conversion tests

### 4.4 MEI → MusicXML Conversion
- [x] Document MEI-only features that will be lost
- [x] Convert MEI structure to MusicXML score-partwise
- [ ] Convert MEI staffGrp/staffDef to MusicXML part-list
- [ ] Convert MEI measure to MusicXML measure
- [ ] Convert MEI note to MusicXML note
- [ ] Convert MEI rest to MusicXML rest
- [ ] Convert MEI chord to MusicXML chord
- [ ] Convert MEI scoreDef/staffDef to MusicXML attributes
- [ ] Convert MEI control events to MusicXML directions
- [ ] Handle MEI duration → MusicXML divisions conversion
- [ ] Add MEI → MusicXML conversion tests

### 4.5 Round-Trip Testing
- [ ] Create round-trip test: MusicXML → MEI → MusicXML
- [ ] Verify no data loss in round-trip for basic scores
- [ ] Test round-trip with real-world MusicXML files
- [ ] Document any conversion discrepancies

---

## Phase 5: Extended MEI Modules

### 5.1 Lyrics (MEI.lyrics)
- [ ] Implement MEI parsing for `<lyrics>` element
- [ ] Implement MEI parsing for `<syl>` element
- [ ] Implement MEI parsing for `<verse>` element
- [ ] Add lyrics tests
- [ ] Implement MusicXML ↔ MEI lyrics conversion

### 5.2 Harmony (MEI.harmony)
- [ ] Implement MEI parsing for `<harm>` element
- [ ] Implement MEI parsing for `<fb>` (figured bass) element
- [ ] Implement MEI parsing for `<chordDef>` element
- [ ] Add harmony tests
- [ ] Implement MusicXML ↔ MEI harmony conversion

### 5.3 Fingering (MEI.fingering)
- [ ] Implement MEI parsing for `<fing>` element
- [ ] Implement MEI parsing for `<fingGrp>` element
- [ ] Add fingering tests
- [ ] Implement MusicXML ↔ MEI fingering conversion

### 5.4 CMN Ornaments (MEI.cmnOrnaments)
- [ ] Implement MEI parsing for `<mordent>` element
- [ ] Implement MEI parsing for `<trill>` element
- [ ] Implement MEI parsing for `<turn>` element
- [ ] Add ornament tests
- [ ] Implement MusicXML ↔ MEI ornament conversion

### 5.5 Mensural Notation (MEI.mensural)
- [ ] Implement MEI parsing for mensural elements (`<mensur>`, `<proport>`)
- [ ] Implement MEI parsing for `<ligature>` element
- [ ] Add mensural notation tests

### 5.6 Neume Notation (MEI.neumes)
- [ ] Implement MEI parsing for `<syllable>` element
- [ ] Implement MEI parsing for `<neume>` element
- [ ] Implement MEI parsing for `<nc>` (neume component) element
- [ ] Add neume notation tests

### 5.7 Tablature (MEI.stringtab)
- [ ] Implement MEI parsing for `<tabGrp>` element
- [ ] Implement MEI parsing for tablature-specific attributes
- [ ] Add tablature tests

### 5.8 Analysis (MEI.analytical)
- [ ] Implement MEI parsing for analytical attributes
- [ ] Implement MEI parsing for `<harm>` with analytical function
- [ ] Add analysis tests

### 5.9 Editorial/Transcription (MEI.edittrans)
- [ ] Implement MEI parsing for `<app>`, `<lem>`, `<rdg>` elements
- [ ] Implement MEI parsing for `<choice>`, `<corr>`, `<sic>` elements
- [ ] Implement MEI parsing for `<add>`, `<del>`, `<subst>` elements
- [ ] Add editorial markup tests

### 5.10 Critical Apparatus (MEI.critapp)
- [ ] Implement MEI parsing for `<app>` with critical apparatus
- [ ] Implement MEI parsing for source/witness references
- [ ] Add critical apparatus tests

### 5.11 Genetic Encoding (MEI.genetic)
- [ ] Implement MEI parsing for genetic state attributes
- [ ] Add genetic encoding tests

### 5.12 Performance (MEI.performance)
- [ ] Implement MEI parsing for `<performance>` element
- [ ] Implement MEI parsing for `<recording>` element
- [ ] Add performance tests

### 5.13 Gestural Attributes (MEI.gestural)
- [ ] Implement MEI parsing for `.ges` attributes (performed values)
- [ ] Add gestural attribute tests

### 5.14 MIDI (MEI.midi)
- [ ] Implement MEI parsing for `<midi>` element
- [ ] Implement MEI parsing for MIDI-specific attributes
- [ ] Add MIDI tests

### 5.15 Visual Attributes (MEI.visual)
- [ ] Implement MEI parsing for `.vis` attributes (rendering hints)
- [ ] Add visual attribute tests

### 5.16 External Symbols (MEI.externalsymbols)
- [ ] Implement MEI parsing for SMuFL glyph references
- [ ] Add external symbols tests

### 5.17 User Symbols (MEI.usersymbols)
- [ ] Implement MEI parsing for `<symbolDef>` element
- [ ] Implement MEI parsing for `<symbol>` element
- [ ] Add user symbols tests

### 5.18 Facsimile (MEI.facsimile)
- [ ] Implement MEI parsing for `<facsimile>` element
- [ ] Implement MEI parsing for `<surface>`, `<zone>` elements
- [ ] Add facsimile tests

### 5.19 Manuscript Description (MEI.msDesc)
- [ ] Implement MEI parsing for `<msDesc>` element
- [ ] Implement MEI parsing for manuscript description children
- [ ] Add manuscript description tests

### 5.20 FRBR (MEI.frbr)
- [ ] Implement MEI parsing for FRBR elements (manifestation, item)
- [ ] Add FRBR tests

### 5.21 Text (MEI.text)
- [ ] Implement MEI parsing for prose elements (`<p>`, `<lg>`, `<l>`)
- [ ] Implement MEI parsing for `<front>`, `<back>` matter
- [ ] Add text tests

### 5.22 Names and Dates (MEI.namesdates)
- [ ] Implement MEI parsing for name elements (`<persName>`, `<corpName>`)
- [ ] Implement MEI parsing for date elements
- [ ] Add names and dates tests

### 5.23 Pointers/References (MEI.ptrref)
- [ ] Implement MEI parsing for `<ptr>` element
- [ ] Implement MEI parsing for `<ref>` element
- [ ] Add pointer/reference tests

### 5.24 Figures and Tables (MEI.figtable)
- [ ] Implement MEI parsing for `<fig>` element
- [ ] Implement MEI parsing for `<table>` element
- [ ] Add figure and table tests

### 5.25 Corpus (MEI.corpus)
- [ ] Implement MEI parsing for `<meiCorpus>` element
- [ ] Add corpus tests

### 5.26 Drama (MEI.drama)
- [ ] Implement MEI parsing for `<sp>` (speech) element
- [ ] Implement MEI parsing for `<stageDir>` element
- [ ] Add drama tests

---

## Phase 6: Advanced Features

### 6.1 Compressed MusicXML (.mxl)
- [ ] Add `zip` dependency for .mxl support
- [ ] Implement .mxl archive reading
- [ ] Implement .mxl archive writing
- [ ] Handle container.xml parsing
- [ ] Add .mxl tests

### 6.2 CLI Tool
- [ ] Create CLI crate with `clap` in `crates/bindings/cli`
- [ ] Implement format detection (MEI vs MusicXML)
- [ ] Implement single file conversion command
- [ ] Implement batch conversion command
- [ ] Add progress bars with `indicatif`
- [ ] Add CLI help and documentation
- [ ] Add CLI integration tests

### 6.3 Validation
- [ ] Implement optional schema validation mode
- [ ] Add validation error reporting
- [ ] Add validation CLI flag

---

## Phase 7: Version Compatibility

### 7.1 MEI Version Support
- [ ] Implement MEI version detection from `@meiversion` attribute
- [ ] Implement MEI 5.0 → 5.1 migration
- [ ] Implement MEI 4.0.1 → 5.1 migration
- [ ] Implement MEI 3.0.0 → 5.1 migration
- [ ] Add version-specific export option (MEI 5.0 target)
- [ ] Add MEI version migration tests

### 7.2 MusicXML Version Support
- [ ] Implement MusicXML version detection
- [ ] Implement MusicXML 3.1 → 4.0 upgrade
- [ ] Implement MusicXML 3.0 → 4.0 upgrade
- [ ] Implement MusicXML 2.0 → 4.0 upgrade
- [ ] Add version-specific export option (MusicXML 3.1 target)
- [ ] Add MusicXML version migration tests

### 7.3 Cross-Version Testing
- [ ] Add cross-version round-trip tests
- [ ] Test with real-world files from different versions

---

## Phase 8: Polish & Coverage

### 8.1 Streaming for Large Files
- [ ] Implement `MeiReader` with mdiv-by-mdiv iteration
- [ ] Implement `MeiWriter` with streaming output
- [ ] Verify memory usage stays under 100 MB for large files
- [ ] Add chunked processing tests with large synthetic files

### 8.2 Test Coverage
- [ ] Achieve 100% test coverage for core model
- [ ] Add property-based tests with `proptest`
- [ ] Add snapshot tests with `insta` for complex outputs
- [ ] Add integration tests with real-world files

### 8.3 Performance
- [ ] Add benchmarks with `criterion`
- [ ] Profile and optimize hot paths
- [ ] Benchmark against other converters if available

### 8.4 Documentation
- [ ] Add rustdoc documentation to all public APIs
- [ ] Create usage examples
- [ ] Document conversion limitations and edge cases

### 8.5 WASM Bindings
- [ ] Create WASM crate in `crates/bindings/wasm`
- [ ] Add `wasm-bindgen` bindings
- [ ] Export conversion functions to JavaScript
- [ ] Add WASM tests
- [ ] Build and publish WASM package

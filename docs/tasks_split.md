# File Splitting Tasks

Tasks for splitting large files into maintainable modules. Each task includes running tests to verify correctness.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: Create submodule(s), move code, re-export from parent, run tests.

---

## Phase 1: MEI Deserializer (13,435 lines)

Target: `crates/formats/mei/src/deserializer/impls.rs` → split into `impls/` submodules

- [x] Extract note/rest/chord/space deserializers → `impls/note.rs`
- [x] Extract measure/staff/layer/section/mdiv deserializers → `impls/structure.rs`
- [x] Extract scoreDef/staffDef/layerDef/staffGrp deserializers → `impls/defs.rs`
- [x] Extract slur/tie/dynam/hairpin/dir/tempo/fermata deserializers → `impls/control.rs`
- [x] Extract beam/tuplet/graceGrp deserializers → `impls/grouping.rs`
- [x] Extract meiHead/fileDesc/titleStmt/pubStmt deserializers → `impls/header.rs`
- [x] Extract app/lem/rdg/choice/corr/sic/add/del deserializers → `impls/editorial.rs`
- [x] Extract text/prose/annotation deserializers → `impls/text.rs`
- [x] Extract remaining deserializers → `impls/misc.rs`, verify impls.rs only has re-exports

---

## Phase 2: MEI Serializer (4,892 lines)

Target: `crates/formats/mei/src/serializer/impls.rs` → split into `impls/` submodules

- [x] Extract note/rest/chord/space serializers → `impls/note.rs`
- [x] Extract measure/staff/layer/section/mdiv serializers → `impls/structure.rs`
- [x] Extract scoreDef/staffDef/layerDef/staffGrp serializers → `impls/defs.rs`
- [x] Extract control event serializers → `impls/control.rs`
- [x] Extract header serializers → `impls/header.rs`
- [x] Extract remaining serializers → `impls/misc.rs`, verify impls.rs only has re-exports

---

## Phase 3: MEI Roundtrip Tests (10,649 lines)

Target: `crates/formats/mei/src/roundtrip_tests.rs` → split into `roundtrip_tests/` submodules

- [x] Extract note/rest/chord roundtrip tests → `roundtrip_tests/note.rs`
- [x] Extract structural element tests → `roundtrip_tests/structure.rs`
- [ ] Extract scoreDef/staffDef tests → `roundtrip_tests/defs.rs`
- [ ] Extract control event tests → `roundtrip_tests/control.rs`
- [ ] Extract header roundtrip tests → `roundtrip_tests/header.rs`
- [ ] Extract editorial element tests → `roundtrip_tests/editorial.rs`
- [ ] Extract full document roundtrip tests → `roundtrip_tests/document.rs`, verify main file only has mod declarations

---

## Phase 4: MusicXML → MEI Converter (5,938 lines)

Target: `crates/core/convert/src/musicxml_to_mei.rs` → split into `musicxml_to_mei/` submodules

- [ ] Extract note/rest/chord conversion → `musicxml_to_mei/note.rs`
- [ ] Extract key/time/clef/divisions conversion → `musicxml_to_mei/attributes.rs`
- [ ] Extract direction/dynamics/tempo conversion → `musicxml_to_mei/direction.rs`
- [ ] Extract measure/part/score structure conversion → `musicxml_to_mei/structure.rs`
- [ ] Extract part-list/staffGrp conversion → `musicxml_to_mei/parts.rs`
- [ ] Extract duration/pitch/ID helpers → `musicxml_to_mei/utils.rs`, verify main file only has entry point

---

## Phase 5: MEI → MusicXML Converter (4,716 lines)

Target: `crates/core/convert/src/mei_to_musicxml.rs` → split into `mei_to_musicxml/` submodules

- [ ] Extract note/rest/chord conversion → `mei_to_musicxml/note.rs`
- [ ] Extract scoreDef/staffDef → attributes conversion → `mei_to_musicxml/attributes.rs`
- [ ] Extract control events → direction conversion → `mei_to_musicxml/direction.rs`
- [ ] Extract measure/section/mdiv structure conversion → `mei_to_musicxml/structure.rs`
- [ ] Extract staffGrp → part-list conversion → `mei_to_musicxml/parts.rs`
- [ ] Extract duration/pitch/ID helpers → `mei_to_musicxml/utils.rs`, verify main file only has entry point

---

## Phase 6: MusicXML Model (5,870 lines total)

### 6.1 Direction Types (2,074 lines)
Target: `crates/formats/musicxml/src/model/direction.rs` → split into `direction/` submodules

- [ ] Extract dynamics types → `direction/dynamics.rs`
- [ ] Extract wedge/hairpin types → `direction/wedge.rs`
- [ ] Extract metronome types → `direction/metronome.rs`
- [ ] Extract remaining direction types → `direction/misc.rs`

### 6.2 Data Types (2,048 lines)
Target: `crates/formats/musicxml/src/model/data.rs` → split into `data/` submodules

- [ ] Extract pitch/duration data types → `data/pitch.rs`
- [ ] Extract notation data types → `data/notation.rs`
- [ ] Extract formatting data types → `data/formatting.rs`

### 6.3 Element Types (1,748 lines)
Target: `crates/formats/musicxml/src/model/elements.rs` → split into `elements/` submodules

- [ ] Extract score/part types → `elements/score.rs`
- [ ] Extract measure types → `elements/measure.rs`
- [ ] Extract barline/ending types → `elements/barline.rs`


---

## Verification

After all phases complete:
- [ ] Run full test suite: `cargo test --workspace`
- [ ] Run clippy: `cargo clippy --workspace`
- [ ] Verify no file exceeds 2000 lines (excluding generated code)

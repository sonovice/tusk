# Codegen Task: Generate ExtractAttributes/CollectAttributes Impls

Extend `mei-codegen` to auto-generate `ExtractAttributes` and `CollectAttributes` trait implementations for all ~730 attribute classes, then replace the 5,469 hand-written macro calls across 73 files.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: The `tusk_codegen_attr_impls.sh` script runs tests, feeds results + this task list to Claude, which fixes the first unchecked task per iteration.

**Constraint**: Every change must pass `cargo test` and `cargo clippy --all-targets` with no regressions.

**Key references**:
- Codegen tool: `tools/mei-codegen/src/generator.rs` (generates att structs in `generate_att_class()`)
- AST types: `tools/mei-codegen/src/ast.rs` (`AttClass`, `Attribute`, `AttributeDataType`)
- ExtractAttributes trait: `crates/formats/mei/src/deserializer/mod.rs` line ~578
- CollectAttributes trait: `crates/formats/mei/src/serializer/mod.rs` line ~250
- extract_attr! macro: `crates/formats/mei/src/deserializer/impls/mod.rs` lines 98-145
- push_attr! macro: `crates/formats/mei/src/serializer/impls/mod.rs` lines 68-89
- Helpers: `from_attr_string()` (deser), `to_attr_string()` / `serialize_vec_serde()` (ser)
- Example hand-written impls: `deserializer/impls/note.rs`, `serializer/impls/note.rs`

---

## Phase 1 — Codegen Infrastructure

- [x] [CODEGEN] Add `--mei-crate` CLI arg to mei-codegen for attribute trait impl output
  - Added `--mei-crate` optional arg to `Args` in `main.rs`
  - Added `generate_mei_attr_impls()` in `generator.rs` — calls `generate_extract_attributes_impls()` and `generate_collect_attributes_impls()`
  - Full impl generation logic included: type→macro variant mapping for extract_attr!/push_attr! based on AttributeDataType + max_occurs
  - Handles all 5 type variants: None→string/clone, Ref(known)→default, Ref(unknown)→string/clone, Primitive→depends on rng_data_to_rust, InlineValList→default, List→space_separated
  - Writes to `<path>/deserializer/generated_att_impls.rs` and `<path>/serializer/generated_att_impls.rs`
  - Prints count of generated impls
  - `cargo build -p mei-codegen` compiles, all tests pass

- [x] [CODEGEN] Implement `generate_extract_attributes()` — ExtractAttributes impls for all attribute classes
  - Implemented in `generate_extract_attributes_impl()` — done as part of task 1
  - Full type→macro variant mapping: None→string, Ref(known)→default, Ref(unknown)→string, Primitive→depends, InlineValList→default, List→space_separated
  - Unbounded variants: vec/vec_string as appropriate
  - Compilation verification deferred to Phase 2 integration

- [x] [CODEGEN] Implement `generate_collect_attributes()` — CollectAttributes impls for all attribute classes
  - Implemented in `generate_collect_attributes_impl()` — done as part of task 1
  - Full type→macro variant mapping: None→clone, Ref(known)→default, Ref(unknown)→clone, Primitive(String)→clone, others→default, unbounded→vec
  - Compilation verification deferred to Phase 2 integration

---

## Phase 2 — Generate and Wire In

- [ ] [INTEGRATION] Run codegen and wire generated files into tusk-mei module tree
  - Run: `cargo run -p mei-codegen -- -i specs/mei/canonical -o crates/core/model/src/generated --mei-crate crates/formats/mei/src`
  - Add `pub(crate) mod generated_att_impls;` to `crates/formats/mei/src/deserializer/impls/mod.rs`
  - Add `pub(crate) mod generated_att_impls;` to `crates/formats/mei/src/serializer/impls/mod.rs`
  - DO NOT remove any hand-written impls yet — there will be duplicate impl errors
  - Compile to see which attribute classes have conflicting hand-written impls
  - List ALL conflicting impls (every `impl ExtractAttributes for AttXxx` that exists in both generated and hand-written files)
  - This produces the definitive list for Phase 3 removal

---

## Phase 3 — Remove Hand-Written Attribute Impls

<!-- The tasks below will be refined in Phase 2 when the exact conflict list is known -->
<!-- Each task removes hand-written impls from one or more files and verifies tests pass -->

- [ ] [REMOVE] Remove hand-written `ExtractAttributes` impls from `deserializer/impls/note.rs`
  - Delete `impl ExtractAttributes for AttNoteLog`, `AttNoteVis`, `AttNoteGes`, `AttNoteAnl`, etc.
  - Keep `impl MeiDeserialize for Note`, `Chord`, `Rest`, etc. (element impls stay)
  - Keep all imports that element impls still need
  - Run tests — note.rs element impls should now use the generated attribute impls

- [ ] [REMOVE] Remove hand-written `CollectAttributes` impls from `serializer/impls/note.rs`
  - Same as above but for the serializer side
  - Delete `impl CollectAttributes for AttNoteLog`, etc.
  - Keep `impl MeiSerialize for Note`, `Chord`, `Rest`, etc.

- [ ] [REMOVE] Remove hand-written `ExtractAttributes` impls from `deserializer/impls/structure.rs`
  - Delete impls for structure-related attribute classes (AttMeasureLog, AttStaffLog, etc.)
  - Keep element deserialization impls

- [ ] [REMOVE] Remove hand-written `CollectAttributes` impls from `serializer/impls/structure.rs`

- [ ] [REMOVE] Remove hand-written `ExtractAttributes` impls from `deserializer/impls/defs.rs`
  - Delete impls for scoreDef/staffDef attribute classes
  - Keep element deserialization impls

- [ ] [REMOVE] Remove hand-written `CollectAttributes` impls from `serializer/impls/defs.rs`

- [ ] [REMOVE] Remove hand-written `ExtractAttributes` impls from `deserializer/impls/control/*.rs`
  - Cover all control event files: articulation.rs, curves.rs, dynamics.rs, harmony.rs, ornaments.rs, pedal.rs, reh.rs, repeats.rs, spanning.rs, text_dir.rs
  - Delete attribute class impls, keep element impls

- [ ] [REMOVE] Remove hand-written `CollectAttributes` impls from `serializer/impls/control/*.rs`

- [ ] [REMOVE] Remove hand-written attribute impls from remaining deserializer impl files
  - analysis.rs, biblio.rs, chords.rs, cmn_core.rs, drama.rs, editorial.rs, facsimile.rs, grouping.rs, header/*.rs, mensural.rs, midi.rs, misc.rs, neumes.rs, symbols.rs, tablature.rs, text.rs, text_containers.rs

- [ ] [REMOVE] Remove hand-written attribute impls from remaining serializer impl files
  - Same set of files as above on the serializer side

---

## Phase 4 — Cleanup and Verification

- [ ] [CLEANUP] Remove now-unused imports from all modified deserializer/serializer impl files
  - After removing attribute impls, some `use tusk_model::att::*` imports may become unused
  - Run `cargo clippy` and fix all unused import warnings
  - The attribute types are still used in element impls (e.g., accessing `self.note_log.dur`), so many imports will remain

- [ ] [CLEANUP] Update mei-codegen documentation and add regeneration instructions
  - Update module doc comments in `deserializer/impls/mod.rs` and `serializer/impls/mod.rs`
  - Old: "In the future, these implementations should be code-generated from the MEI ODD specification"
  - New: "Attribute class impls are auto-generated in generated_att_impls.rs. Element impls below are hand-written."
  - Add comment at top of generated files: "DO NOT EDIT - regenerate with: cargo run -p mei-codegen -- ... --mei-crate ..."

- [ ] [VERIFY] Full test suite verification and final metrics
  - Run `cargo test` — all tests must pass (MusicXML: 35/35, MEI: 91/97, fragments: 275/275)
  - Run `cargo clippy --all-targets` — no new warnings
  - Count remaining `extract_attr!` calls (should be 0 in generated impls, only in element impls if any)
  - Count remaining `push_attr!` calls (same)
  - Verify codegen is idempotent: run twice, diff output, should be identical

---

## Type Mapping Reference

For the codegen to determine the correct macro variant, it uses the attribute's `AttributeDataType` and `max_occurs`:

```
                           max_occurs
                     ┌──────────┴─────────┐
                     │ None/1             │ "unbounded"
  ┌──────────────────┼────────────────────┼────────────────────┐
  │ datatype=None    │ Option<String>     │ Vec<String>        │
  │                  │ string / clone     │ vec_string / vec   │
  ├──────────────────┼────────────────────┼────────────────────┤
  │ Ref(known)       │ Option<DataXxx>    │ Vec<DataXxx>       │
  │                  │ default / default  │ vec / vec          │
  ├──────────────────┼────────────────────┼────────────────────┤
  │ Ref(unknown)     │ Option<String>     │ Vec<String>        │
  │                  │ string / clone     │ vec_string / vec   │
  ├──────────────────┼────────────────────┼────────────────────┤
  │ Primitive        │ Option<RustPrim>   │ Vec<RustPrim>      │
  │                  │ default / default  │ vec / vec          │
  ├──────────────────┼────────────────────┼────────────────────┤
  │ InlineValList    │ Option<AttXxxEnum> │ Vec<AttXxxEnum>    │
  │                  │ default / default  │ vec / vec          │
  ├──────────────────┼────────────────────┼────────────────────┤
  │ List{inner}      │ Option<SpaceSep<T>>│ (not observed)     │
  │                  │ space_sep / default│                    │
  └──────────────────┴────────────────────┴────────────────────┘

  Legend: extract_variant / collect_variant
```

---

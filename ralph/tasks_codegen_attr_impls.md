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
- Codegen command: `cargo run -p mei-codegen -- -i specs/mei/modules -o crates/core/model/src/generated --mei-crate crates/formats/mei/src`

---

## Phase 1 — Codegen Infrastructure

- [x] [CODEGEN] Add `--mei-crate` CLI arg to mei-codegen for attribute trait impl output
  - Added `--mei-crate` optional arg to `Args` in `main.rs`
  - Added `generate_mei_attr_impls()` in `generator.rs` — calls `generate_extract_attributes_impls()` and `generate_collect_attributes_impls()`
  - Full impl generation logic included: type→macro variant mapping for extract_attr!/push_attr! based on AttributeDataType + max_occurs
  - Handles all 5 type variants: None→string/clone, Ref(known)→default, Ref(unknown)→string/clone, Primitive→depends on rng_data_to_rust, InlineValList→default, List→space_separated
  - Writes to `<path>/deserializer/impls/generated_att_impls.rs` and `<path>/serializer/impls/generated_att_impls.rs`
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

- [x] [INTEGRATION] Run codegen and wire generated files into tusk-mei module tree
  - Fixed codegen output paths: `deserializer/impls/generated_att_impls.rs` and `serializer/impls/generated_att_impls.rs` (was missing `impls/` subdirectory)
  - Fixed regeneration command in generated file headers: `specs/mei/modules` (not `specs/mei/canonical`)
  - Added `#[allow(unused_imports)]` for helper imports (macros use `$crate::` paths)
  - Ran: `cargo run -p mei-codegen -- -i specs/mei/modules -o crates/core/model/src/generated --mei-crate crates/formats/mei/src`
  - Generated 729 ExtractAttributes + 729 CollectAttributes impls (7300 + 8029 lines)
  - Added `mod generated_att_impls;` to both `deserializer/impls/mod.rs` and `serializer/impls/mod.rs`
  - Compiled and found 1077 conflicting impls across 60 files (see Phase 3 below)

---

## Phase 3 — Remove Hand-Written Attribute Impls

- [x] [REMOVE] Remove ALL 1057 hand-written attribute impls from 60 files
  - Removed 500 `impl ExtractAttributes for Att*` blocks from 30 deserializer files
  - Removed 557 `impl CollectAttributes for Att*` blocks from 30 serializer files
  - Script: brace-counting Python script that identifies `impl (Extract|Collect)Attributes for Att*` blocks and removes them while preserving all element impls
  - Fixed `extract_attr!` `space_separated` macro variant: was String-only, now uses `from_attr_string` for typed parsing (SpaceSeparated<f64>, SpaceSeparated<u64>, etc.)
  - Fixed codegen: empty CollectAttributes impls return `Vec::new()` instead of `let mut attrs` (eliminated 202 unused-mut warnings)
  - Cleaned up `deserializer/impls/mod.rs`: removed unused `tusk_model::att::*` imports and empty section comments
  - All 2929 tests pass, zero clippy errors/warnings

---

## Phase 4 — Cleanup and Verification

- [x] [CLEANUP] Remove now-unused imports from all modified deserializer/serializer impl files
  - Cleaned `deserializer/impls/mod.rs` — removed 42 unused `use tusk_model::att::*` imports
  - Zero unused import warnings from `cargo clippy`

- [x] [CLEANUP] Update mei-codegen documentation and add regeneration instructions
  - Updated module doc comments in `deserializer/impls/mod.rs` and `serializer/impls/mod.rs`
  - Old: "In the future, these implementations should be code-generated from the MEI ODD specification"
  - New: "Attribute class impls are auto-generated in generated_att_impls.rs. Element impls are hand-written below."
  - Generated files already have DO NOT EDIT header with regeneration command

- [ ] [VERIFY] Full test suite verification and final metrics
  - Run `cargo test` — all tests must pass
  - Run `cargo clippy --all-targets` — no new warnings
  - Count remaining `extract_attr!` calls (should be 0 in hand-written attribute impls)
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

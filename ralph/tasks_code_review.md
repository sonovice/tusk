# Code Review Tasks

Tasks derived from `docs/review01.md`. Each task addresses a specific maintainability, consistency, DRY, or readability issue found during the comprehensive codebase review.

**Legend**: `[ ]` = pending, `[x]` = done

**Workflow**: The `tusk_code_review.sh` script runs tests/clippy, feeds results + this task list to Claude, which fixes the first unchecked task. Tasks in Priority 1 block Priority 2, which block Priority 3.

**Constraint**: Every change must pass `cargo test` and `cargo clippy` with no regressions.

---

## Priority 1 — High Impact

<!-- These tasks address the most impactful DRY violations and structural issues -->
<!-- IMPORTANT: Complete these BEFORE Priority 2 tasks -->

- [x] [DRY] Extract child element parsing helper in `crates/formats/mei/src/deserializer/impls/grouping.rs`
  - Created `parse_grouping_child!` macro that generates the match block for each grouping element
  - Each call site specifies its child enum type and the list of supported element→variant mappings
  - Beam, Tuplet, GraceGrp now use the macro instead of duplicated match blocks
  - All roundtrip and unit tests pass

- [x] [DRY] Consolidate `strip_namespace_prefix()` into single canonical implementation
  - Removed duplicate from `xml_compare.rs`, now imports canonical version from `crate::deserializer`
  - xml_compare now correctly preserves both `xml:` and `xlink:` prefixes
  - Added `xlink:href` test case to xml_compare tests

- [x] [DRY] Consolidate XML entity reference resolution into single function
  - Extracted `resolve_xml_entity(&BytesRef, &mut String)` in `deserializer/mod.rs`
  - Handles predefined entities, char refs, and unknown entities (preserved as-is)
  - Replaced all 3 duplicated blocks: deserializer skip_to_end_and_collect, next_mixed_content, and xml_compare parse_canonical
  - Removed unused `resolve_predefined_entity` import from xml_compare

---

## Priority 2 — Medium Impact

<!-- These tasks improve readability, consistency, and debuggability -->

- [x] [READABILITY] Split `crates/formats/musicxml/src/context.rs` into focused sub-modules
  - Split into `context/mod.rs` (struct def, constructor, direction, duration, reset), `context/slurs.rs`, `context/ties.rs`, `context/positions.rs` (position, key sig, accidentals, warnings), `context/ids.rs`
  - Public API unchanged: all types re-exported from context/mod.rs
  - All tests pass, no regressions

- [x] [CONSISTENCY] Standardize import/export function naming in tusk-musicxml
  - Chose convention: import keeps `convert_*`, export uses `convert_mei_*` prefix
  - Renamed 8 export functions: `convert_score_def_to_attributes`, `convert_staff_def_to_attributes`, `convert_part_list`, `convert_staff_def_to_score_part`, `convert_staff_grp_barline`, `convert_staff_grp_symbol`, `convert_staff_grp_to_part_list`, `convert_score_content` (in content.rs)
  - Updated all call sites in attributes.rs, parts.rs, content.rs, structure.rs, mod.rs
  - All tests pass, no regressions

- [x] [ERROR_HANDLING] Add `tracing::warn!` for silently skipped unknown elements in MEI deserializer
  - Added `skip_unknown_child(&str, &str, bool)` method on `MeiReader` that logs via `tracing::warn!` and skips
  - Replaced all ~155 catch-all `_ =>` blocks across 18 deserializer impl files to use the new method
  - Updated `parse_grouping_child!` macro with new `parent:` parameter for correct parent name
  - Fixed parent names that were incorrectly propagated from unrelated parsers in the same file
  - Added `clippy::needless_borrow` to tusk-mei allow list (consistent with other suppressed lints)

- [x] [DRY] Extract beat position calculation into ConversionContext method
  - Added `divisions_to_beats(value: f64) -> f64` on ConversionContext (handles zero-division guard)
  - Added `beat_position_in_beats() -> f64` convenience wrapper
  - Replaced inline `if divisions > 0.0 { x / divisions } else { x }` pattern in `import/direction.rs`
  - Simplified `calculate_tstamp` from 14 lines to 6 lines

- [ ] [CONSISTENCY] Align `extract_attr!` / `push_attr!` macro variant naming
  - `extract_attr!` uses: default, `string`, `vec`, `vec_string`, `space_separated`
  - `push_attr!` uses: default, `clone`, `string`, ... (different variant names for equivalent operations)
  - Align the variant keywords so the same attribute uses the same keyword in both macros
  - This is a mechanical rename of macro arms + updating call sites

---

## Priority 3 — Low Impact

<!-- Cleanup tasks: dead code, deprecated deps, hygiene -->

- [ ] [DEBT] Remove deprecated `convert_staff_def` and consolidate with replacement
  - `crates/formats/musicxml/src/import/parts.rs` has `#[deprecated]` `convert_staff_def`
  - It's still re-exported from `import/mod.rs` with `#[allow(deprecated)]`
  - Remove the deprecated function entirely
  - Update any test code using it to use `convert_staff_def_from_score_part` instead
  - Remove the `#[allow(deprecated)]` from mod.rs re-exports

- [ ] [DEBT] Replace `once_cell` with `std::sync::OnceLock` in tusk-model
  - `crates/core/model/Cargo.toml` depends on `once_cell` which is deprecated since Rust 1.70
  - This is generated code, so the fix must be in `tools/mei-codegen/`
  - Update codegen to emit `std::sync::OnceLock` instead of `once_cell::sync::Lazy`
  - Regenerate model: `cargo run -p mei-codegen -- -i specs/mei/modules -o crates/core/model/src/generated --mei-crate crates/formats/mei/src`
  - Remove `once_cell` from Cargo.toml
  - Run `cargo fmt` on generated code

- [ ] [DEBT] Move per-crate pinned dependencies to workspace in root Cargo.toml
  - tusk-model pins `serde = "1.0.228"`, `regex = "1.12.3"`, `once_cell = "1.21.3"` locally
  - These should use `workspace = true` like `derive_more` and `thiserror` already do
  - Add `serde`, `regex` to `[workspace.dependencies]` in root Cargo.toml
  - Update per-crate Cargo.toml to use `{ workspace = true }` syntax

- [ ] [ERROR_HANDLING] Audit and replace production `.unwrap()` calls in export/note.rs
  - `export/note.rs` has 48 `.unwrap()` calls, many in non-test code paths
  - Replace production unwraps with proper `?` propagation or `.unwrap_or_default()` where appropriate
  - Test-only unwraps are acceptable and can stay
  - Similarly audit `model/data/formatting.rs` (39 unwrap calls)

- [ ] [TESTING] Remove trivial `crate_compiles` smoke tests
  - `crates/formats/mei/src/lib.rs` line 114: `assert!(true)` — does nothing
  - `crates/formats/musicxml/src/lib.rs` line 130-133: instantiates two enum variants — marginally useful
  - Remove the mei version entirely; keep or improve the musicxml version

- [ ] [TESTING] Deduplicate test helpers across import modules
  - `make_score_part()` helper appears in both `import/structure.rs` and `import/parts.rs` test modules
  - Extract shared test helpers into a `#[cfg(test)] mod test_utils` in the import module
  - Both test modules can then `use super::test_utils::*`

- [ ] [CONSISTENCY] Unify clippy allow lists across format crates
  - `tusk-mei/src/lib.rs` suppresses 15 clippy lints
  - `tusk-musicxml/src/lib.rs` suppresses 4 clippy lints (different set)
  - Audit which lints are actually needed per crate (run clippy without allows)
  - Remove suppressed lints that no longer trigger
  - For lints needed in both crates, keep them consistent

---

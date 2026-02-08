# MEI codegen (tusk-mei-codegen)

Generates Rust model code and MEI serializer/deserializer impls from the MEI specification. Lives under `crates/formats/mei/codegen`; no dependency on other format crates.

## Input

- **ODD (current)**: `specs/mei/modules/MEI.*.xml`
- **RNG (planned)**: `specs/mei/validation/mei-all.rng` (internal/export), `specs/mei/versions/mei-all_v*.rng` (versioned import)

## Output

- **tusk-model**: `crates/core/model/src/generated/` (data, att, elements, model, validation)
- **tusk-mei**: serializer/deserializer impls in `crates/formats/mei/src/` (generated_att_impls.rs, generated_element_impls.rs)

## Usage

```bash
# From repo root
cargo run -p tusk-mei-codegen -- \
  --input specs/mei/modules \
  --output crates/core/model/src/generated \
  --mei-crate crates/formats/mei/src
```

## Versioned import

To generate a versioned import model (e.g. MEI 5.1), use `--versioned` with `--rng` and point output into the mei crate:

```bash
cargo run -p tusk-mei-codegen -- \
  --versioned v5_1 \
  --rng specs/mei/versions/mei-all_v5.1.rng \
  --output crates/formats/mei/src/versions/v5_1
```

Other version labels: `v2_1_1`, `v3_0_0`, `v4_0_1`, `v5_0`. RNG files: `specs/mei/versions/mei-all_v2.1.1.rng`, `mei-all_v.5.0.rng`, etc. Version detection is in `tusk_mei::versions::detect_mei_version`.

## Regeneration

When the MEI spec or codegen changes, run the command above, then `cargo check` and the MEI roundtrip/schema tests.

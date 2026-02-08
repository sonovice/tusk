# tusk-mei

MEI parsing, serialization, and conversion for Tusk.

## Codegen

Rust model and serializer/deserializer impls are generated from the MEI RNG specification.

- **Codegen crate**: `crates/formats/mei/codegen/`  
  See [codegen/README.md](codegen/README.md) for usage and versioned import.
- **All versions**: `crates/formats/mei/codegen/schema/versions/mei-all_v*.rng` (including `mei-all_v6.0-dev.rng` for internal/export)

Do not edit generated files under `crates/core/model/src/generated/` or the generated impl modules in this crate; regenerate with `cargo run -p tusk-mei-codegen` (see codegen README).

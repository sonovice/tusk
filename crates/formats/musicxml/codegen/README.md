# MusicXML Codegen

Generates Rust model types from the MusicXML XSD schema. Schemas live under **`codegen/schema/versions/musicxml-X.Y/schema/`** (2.0–4.1).

## Usage

From the workspace root:

```bash
cargo run -p tusk-musicxml-codegen -- \
  --input crates/formats/musicxml/codegen/schema/versions/musicxml-4.1/schema/musicxml.xsd \
  --output path/to/output
```

## Input

- **XSD (all versions)**: `codegen/schema/versions/musicxml-{2.0,3.0,3.1,4.0,4.1}/schema/musicxml.xsd`.  
  `tusk-musicxml` build.rs generates `src/versions/v2_0/` … `v4_1/` from these.

## Output

- `data.rs` – simple types (enums and type aliases from `xs:simpleType`).
- `mod.rs` – module re-exports.

Complex types and top-level elements are parsed; full codegen for structs and root documents can be extended in the generator.

## No cross-format deps

This crate does not depend on `tusk-mei` or other format crates.

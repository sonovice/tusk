# MusicXML Codegen

Generates Rust model types from the MusicXML 4.1 XSD schema.

## Usage

From the workspace root:

```bash
cargo run -p tusk-musicxml-codegen -- \
  --input specs/musicxml/schema/musicxml.xsd \
  --output path/to/output
```

## Input

- **XSD**: `specs/musicxml/schema/musicxml.xsd` (MusicXML 4.1, single-file schema).

## Output

- `data.rs` – simple types (enums and type aliases from `xs:simpleType`).
- `mod.rs` – module re-exports.

Complex types and top-level elements are parsed; full codegen for structs and root documents can be extended in the generator.

## No cross-format deps

This crate does not depend on `tusk-mei` or other format crates.

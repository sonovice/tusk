# tusk-musicxml

MusicXML parsing, serialization, and MEI conversion for Tusk.

## Supported versions

- **4.0, 4.1** – Primary target (hand-written model + codegen data types).
- **2.0, 3.0, 3.1** – Versioned XSD-based data types generated at build time; use `versions::detect_musicxml_version()` to detect document version.

Versions 1.0 and 1.1 are DTD-only and do not have generated modules yet.

## Codegen

Rust types are generated from the MusicXML XSD schema(s).

- **Codegen crate**: `crates/formats/musicxml/codegen/`  
  See [codegen/README.md](codegen/README.md) for usage.
- **Main schema (4.1)**: `specs/musicxml/schema/musicxml.xsd`
- **Versioned schemas (2.0–4.0)**: `specs/musicxml/versions/musicxml-X.Y/schema/musicxml.xsd`  
  These are used by `build.rs` to generate `src/versions/v2_0/`, `v3_0/`, `v3_1/`, `v4_0/`.

Do not edit generated files under `src/versions/*/`; regenerate by building the crate or running the codegen manually.

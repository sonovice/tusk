# tusk-musicxml

MusicXML parsing, serialization, and MEI conversion for Tusk.

## Supported versions

- **2.0, 3.0, 3.1, 4.0, 4.1** – XSD-based data types generated at build time from each version’s schema; use `versions::detect_musicxml_version()` to detect document version.

Versions 1.0 and 1.1 are DTD-only and do not have generated modules yet.

## Codegen

Rust types are generated from the MusicXML XSD schema(s).

- **Codegen crate**: `crates/formats/musicxml/codegen/`  
  See [codegen/README.md](codegen/README.md) for usage.
- **All versions (2.0–4.1)**: `crates/formats/musicxml/codegen/schema/versions/musicxml-X.Y/schema/musicxml.xsd`  
  Used by `build.rs` to generate `src/versions/v2_0/`, `v3_0/`, `v3_1/`, `v4_0/`, `v4_1/`.

Do not edit generated files under `src/versions/*/`; regenerate by building the crate or running the codegen manually.

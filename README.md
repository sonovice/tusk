<table>
<tr>
<td width="120">
<img src="logo.svg" alt="Tusk" width="100">
</td>
<td>

# Tusk

**T**he **U**ltimate **S**core **K**onverter. A bidirectional music notation format converter in Rust.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
![Language: Rust](https://img.shields.io/badge/language-Rust-orange)

</td>
</tr>
</table>

Tusk converts between MusicXML, MEI, and LilyPond. Its internal model is based on [MEI](https://music-encoding.org/) (Music Encoding Initiative) and extended where needed to cover concepts from other formats. Every conversion passes through this shared model, preserving as much musical information as possible across formats.

## Formats

| From \ To | MusicXML | MEI | LilyPond | Extensions |
|-----------|----------|-----|----------|------------|
| **MusicXML** | ✅ | ✅ | ✅ | `.musicxml`, `.mxl` |
| **MEI** | ✅ | ✅ | ✅ | `.mei` |
| **LilyPond** | ✅ | ✅ | ✅ | `.ly` |

Output versions: MusicXML 4.1, MEI 6.0-dev, LilyPond 2.24. Older input versions are accepted but mileage may vary.

## CLI

```
tusk <INPUT> <OUTPUT>
```

The input and output formats are determined by file extension, with content-based fallback for the input.

```bash
$ tusk score.musicxml score.mei
Converted "score.musicxml" (MusicXML) -> "score.mei" (MEI)

$ tusk score.mei score.ly
Converted "score.mei" (MEI) -> "score.ly" (LilyPond)

$ tusk concert.mxl concert.ly
Converted "concert.mxl" (MusicXML) -> "concert.ly" (LilyPond)
```

## Library

```rust
use tusk_musicxml::MusicXmlFormat;
use tusk_mei::MeiFormat;
use tusk_format::{Importer, Exporter};

let (base, ext) = MusicXmlFormat.import_from_str(&musicxml_string)?;
let mei_xml = MeiFormat.export_to_string(&base, &ext)?;
```

## Building

Requires [Rust](https://www.rust-lang.org/tools/install) (edition 2024). No other external dependencies.

```bash
git clone https://github.com/sonovice/tusk.git
cd tusk
cargo build --release
```

Binary ends up at `target/release/tusk-cli`.

## Tests

```bash
cargo test
```

Additional regression tests against upstream MEI and LilyPond test suites are available via git submodules. These are not needed for the standard test suite.

```bash
git submodule update --init                                       # all submodules
git submodule update --init tests/fixtures/mei/sample-encodings   # MEI only
git submodule update --init tests/fixtures/lilypond/lilypond-repo # LilyPond only

cargo test -- --ignored   # run regression tests
```

## Project layout

```
crates/
├── core/
│   ├── model/          MEI data model, generated from RNG schema
│   └── format/         Format traits (Importer, Exporter, FormatRegistry)
├── formats/
│   ├── mei/            MEI parser, serializer, version migration
│   │   └── codegen/    RNG to Rust code generator
│   ├── musicxml/       MusicXML parser, serializer, MEI conversion
│   │   └── codegen/    XSD to Rust code generator
│   └── lilypond/       LilyPond lexer, parser, serializer, MEI conversion
└── bindings/
    └── cli/            Command-line interface
```

The MEI model is generated from the official RNG schema at build time. No manual codegen step required.

## License

[MIT](LICENSE)

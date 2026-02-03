# MEI ODD → Rust Code Generator

Generates Rust model code from MEI ODD specification files for 1:1 MEI mapping.

## Overview

The MEI ODD (One Document Does-it-all) specification files define:
- **Data types** (`macroSpec`): Enumerations like `data.DURATION.cmn`
- **Attribute classes** (`classSpec`): Groups of attributes like `att.duration.log`
- **Elements** (`elementSpec`): Elements like `<note>`, `<rest>`, etc.

This tool parses these definitions and generates corresponding Rust types.

## Input Files

Source: `specs/mei/modules/MEI.*.xml`

Key modules:
- `MEI.shared.xml` - Core elements and attribute classes
- `MEI.cmn.xml` - Common Music Notation
- `MEI.header.xml` - Metadata
- `MEI.gestural.xml` - Gestural/performed attributes
- etc.

## Generated Output

```
crates/core/model/src/generated/
├── data.rs          # Data types (enums)
├── att/             # Attribute classes (structs)
│   ├── mod.rs
│   ├── common.rs
│   ├── duration.rs
│   └── ...
├── elements/        # Elements (structs)
│   ├── mod.rs
│   ├── note.rs
│   └── ...
└── model.rs         # Model classes (traits)
```

## Naming Conventions

| MEI ODD | Rust |
|---------|------|
| `data.DURATION.cmn` | `DataDurationCmn` (enum) |
| `att.duration.log` | `AttDurationLog` (struct) |
| `att.pitch` | `AttPitch` (struct) |
| `<note>` element | `Note` (struct) |
| `model.eventLike` | `ModelEventLike` (trait) |

## Code Generation Rules

### Data Types (macroSpec)

```xml
<macroSpec ident="data.DURATION.cmn" type="dt">
  <content>
    <valList type="closed">
      <valItem ident="1"><desc>Whole note.</desc></valItem>
      <valItem ident="2"><desc>Half note.</desc></valItem>
      ...
    </valList>
  </content>
</macroSpec>
```

Generates:

```rust
/// Logical duration values for CMN repertoire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataDurationCmn {
    /// Whole note.
    #[serde(rename = "1")]
    Whole,
    /// Half note.
    #[serde(rename = "2")]
    Half,
    // ...
}
```

### Attribute Classes (classSpec)

```xml
<classSpec ident="att.duration.log" type="atts">
  <attList>
    <attDef ident="dur" usage="opt">
      <desc>Duration value.</desc>
      <datatype><rng:ref name="data.DURATION"/></datatype>
    </attDef>
  </attList>
</classSpec>
```

Generates:

```rust
/// Logical domain attributes for duration.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct AttDurationLog {
    /// Duration value.
    pub dur: Option<DataDuration>,
}
```

### Elements (elementSpec)

```xml
<elementSpec ident="note">
  <classes>
    <memberOf key="att.common"/>
    <memberOf key="att.duration.log"/>
    <memberOf key="att.pitch"/>
    <memberOf key="att.octave"/>
    ...
  </classes>
  <content>
    <rng:zeroOrMore>
      <rng:ref name="model.noteModifierLike"/>
    </rng:zeroOrMore>
  </content>
</elementSpec>
```

Generates:

```rust
/// A single pitched note.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Note {
    // Attribute classes (flattened)
    pub common: AttCommon,
    pub duration_log: AttDurationLog,
    pub pitch: AttPitch,
    pub octave: AttOctave,
    // ...

    // Child elements
    pub children: Vec<NoteChild>,
}

pub enum NoteChild {
    Accid(Accid),
    Artic(Artic),
    // ... (from model.noteModifierLike)
}
```

## Usage

```bash
# Generate all model code
cargo run -p mei-codegen -- \
    --input specs/mei/modules/ \
    --output crates/core/model/src/generated/

# Generate specific module
cargo run -p mei-codegen -- \
    --input specs/mei/modules/MEI.cmn.xml \
    --output crates/core/model/src/generated/cmn/
```

## Serde Integration

Generated types include serde derives for XML serialization:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "note")]
pub struct Note {
    #[serde(rename = "@xml:id", skip_serializing_if = "Option::is_none")]
    pub xml_id: Option<XmlId>,

    #[serde(rename = "@dur", skip_serializing_if = "Option::is_none")]
    pub dur: Option<DataDuration>,

    // ...
}
```

## Validation

Generated code is validated by:
1. `cargo check` - Type correctness
2. Round-trip tests - Parse MEI → serialize → compare
3. Schema validation - Output validates against `mei-all.rng`

## Regeneration

When MEI version updates:
1. Update ODD files in `specs/mei/modules/`
2. Run codegen: `cargo run -p mei-codegen`
3. Review changes, update conversion logic if needed
4. Run tests

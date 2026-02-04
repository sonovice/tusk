# Conversion Notes: MEI → MusicXML Lossy Conversion

This document details MEI features that are lost or simplified when converting to MusicXML.
MusicXML → MEI conversion is **lossless** (all MusicXML content is preserved in MEI).
MEI → MusicXML conversion is **lossy** (many MEI features have no MusicXML equivalent).

---

## Summary

| Category | Loss Level | Notes |
|----------|------------|-------|
| CMN Core (notes, rests, chords) | Low | Basic content preserved |
| Control Events (slurs, dynamics) | Low-Medium | Basic elements preserved, advanced attributes lost |
| Analytical Attributes (.anl) | 100% | No MusicXML equivalent |
| Gestural Attributes (.ges) | 90% | Only basic `dur.ppq` partially mapped |
| Visual Attributes (.vis) | 60% | Some rendering hints preserved |
| Editorial Markup | 100% | No MusicXML equivalent |
| Critical Apparatus | 100% | No MusicXML equivalent |
| Facsimile Links | 100% | No MusicXML equivalent |
| Mensural Notation | 100% | No MusicXML equivalent |
| Neume Notation | 100% | No MusicXML equivalent |
| Tablature | 100% | No MusicXML equivalent |
| Harmonic Analysis | 100% | No MusicXML equivalent |
| Figured Bass | 100% | No MusicXML equivalent |
| Rich Metadata | 80% | Flattened to basic identification |
| Physical Source Info | 100% | No MusicXML equivalent |

---

## 1. Notation Systems (100% Loss)

### 1.1 Mensural Notation (MEI.mensural)

MusicXML only supports Common Music Notation. The following mensural elements are lost:

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<mensur>` | Mensural meter sign with perfect/imperfect values | None |
| `<proport>` | Mensural proportion (e.g., 3:2) | None |
| `<ligature>` | Ligatures and note groupings | None |
| `<plica>` | Plica marks | None |

**Affected attributes:**
- `@mensur.sign`, `@mensur.dot`, `@mensur.slash`
- `@proport.num`, `@proport.numbase`
- `@lig` on notes (ligature context)

### 1.2 Neume Notation (MEI.neumes)

MusicXML has no support for liturgical chant notation:

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<neume>` | Individual neume | None |
| `<nc>` | Neume component | None |
| `<syllable>` | Syllable container | None |
| `<episema>` | Episema marks | None |
| `<oriscus>` | Oriscus neume form | None |
| `<quilisma>` | Quilisma neume form | None |
| `<strophicus>` | Strophicus neume form | None |
| `<liquescent>` | Liquescent endings | None |

### 1.3 Tablature (MEI.stringtab)

MusicXML has limited tablature support but MEI-specific elements are lost:

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<tabGrp>` | Tablature group | None |
| `<tabDurSym>` | Tablature duration symbol | None |
| `<tuning>` | Detailed instrument tuning | Limited (part of `<staff-details>`) |

---

## 2. MEI Domain Attributes (Significant Loss)

MEI organizes attributes into semantic domains. This separation is lost in MusicXML.

### 2.1 Logical Attributes (.log) - Mostly Preserved

Logical (notated) values generally have MusicXML equivalents:

| MEI Attribute | Description | MusicXML Equivalent |
|---------------|-------------|---------------------|
| `@dur` | Written duration | `<type>` |
| `@pname`, `@oct` | Pitch | `<step>`, `<octave>` |
| `@accid` | Written accidental | `<accidental>` |
| `@stem.dir` | Stem direction | `<stem>` |
| `@dots` | Augmentation dots | `<dot>` elements |

### 2.2 Gestural Attributes (.ges) - 90% Loss

Performed/sounding values are largely lost:

| MEI Attribute | Description | MusicXML Equivalent |
|---------------|-------------|---------------------|
| `@dur.ges` | Performed duration | Lost (only via MIDI `<duration>`) |
| `@dur.ppq` | Duration in PPQ | Partial (via `<duration>`) |
| `@pname.ges` | Sounding pitch name | Lost |
| `@oct.ges` | Sounding octave | Lost |
| `@accid.ges` | Sounding accidental | Lost (but `<alter>` partially maps) |
| `@vel` | MIDI velocity | Lost |
| `@instr` | Instrument reference | Lost |

### 2.3 Visual Attributes (.vis) - 60% Loss

Rendering hints are partially preserved:

| MEI Attribute | Description | MusicXML Equivalent |
|---------------|-------------|---------------------|
| `@color` | Element color | Limited (`<color>` on some elements) |
| `@fontsize`, `@fontfam` | Font styling | `<font>` on some elements |
| `@ho`, `@vo` | Horizontal/vertical offset | `<offset>` on some elements |
| `@place` | Above/below placement | `<placement>` |
| `@stem.mod` | Stem modifications | Lost |
| `@beam.color` | Beam coloring | Lost |
| `@head.color` | Note head coloring | Lost |
| `@head.mod` | Note head modifications | Lost |
| `@head.shape` | Note head shape | `<notehead>` |

### 2.4 Analytical Attributes (.anl) - 100% Loss

Analysis attributes have no MusicXML equivalent:

| MEI Attribute | Description | MusicXML Equivalent |
|---------------|-------------|---------------------|
| `@pclass` | Pitch class (0-11) | None |
| `@deg` | Scale degree | None |
| `@harm` | Harmonic function | None |
| `@mfunc` | Melodic function | None |
| `@solfa` | Solfège syllable | None |

---

## 3. Control Events (Medium Loss)

### 3.1 Slurs and Ties

| MEI Feature | Description | MusicXML Status |
|-------------|-------------|-----------------|
| `<slur>` basic | Start/end points | Preserved |
| `<slur>` with `<curve>` child | Bezier control points | Lost |
| `@slur.lform` | Line form (solid/dashed) | `<line-type>` |
| `@slur.lwidth` | Line width | Lost |
| `@curvedir` | Curve direction | `<orientation>` |
| Analytical slur attributes | Musical function | Lost |

### 3.2 Dynamics

| MEI Feature | Description | MusicXML Status |
|-------------|-------------|-----------------|
| `<dynam>` text | Dynamic marking | Preserved |
| `@startid`, `@endid` | Element references | Lost (uses position) |
| `@staff`, `@layer` | Precise attachment | Partial |
| Nested `<rend>` | Styled text | Lost |

### 3.3 Hairpins

| MEI Feature | Description | MusicXML Status |
|-------------|-------------|-----------------|
| `<hairpin>` | Crescendo/decrescendo | Preserved |
| `@form` | cresc/dim | Preserved (`<wedge type>`) |
| `@opening` | Opening width | Preserved (`<spread>`) |
| `@lform` | Line form | Lost |
| `@niente` | To/from nothing | `<niente>` |

### 3.4 Tempo

| MEI Feature | Description | MusicXML Status |
|-------------|-------------|-----------------|
| `<tempo>` text | Tempo marking | Preserved (`<words>`) |
| `@midi.bpm` | BPM value | Preserved (`<per-minute>`) |
| `@mm`, `@mm.unit` | Metronome mark | Preserved (`<metronome>`) |
| `@func` | rit/accel/etc. | Lost |
| `@startid`, `@endid` | Element references | Lost |

### 3.5 Fermatas

| MEI Feature | Description | MusicXML Status |
|-------------|-------------|-----------------|
| `<fermata>` | Fermata mark | Preserved |
| `@shape` | Shape variant | Preserved |
| `@place` | Above/below | Preserved |
| `@form` | Normal/inverted | Preserved |

---

## 4. Beams and Tuplets (Low-Medium Loss)

### 4.1 Beams

| MEI Feature | Description | MusicXML Status |
|-------------|-------------|-----------------|
| `<beam>` grouping | Explicit beam groups | MusicXML uses `<beam>` on notes |
| `<beamSpan>` | Cross-staff beams | Limited |
| `@beam.color` | Beam coloring | Lost |
| `@beam.rend` | Beam rendering | Lost |
| `@beam.slope` | Beam slope | Lost |

### 4.2 Tuplets

| MEI Feature | Description | MusicXML Status |
|-------------|-------------|-----------------|
| `<tuplet>` grouping | Tuplet container | Preserved (`<tuplet>`) |
| `@num`, `@numbase` | Tuplet ratio | Preserved |
| `@bracket.visible` | Show bracket | Preserved |
| `@num.visible` | Show number | Preserved |
| `<tupletSpan>` | Cross-element tuplets | Limited |
| Analytical attributes | Musical analysis | Lost |

---

## 5. Editorial & Critical Apparatus (100% Loss)

MusicXML has no support for scholarly editorial markup.

### 5.1 Editorial Markup (MEI.edittrans)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<choice>` | Editorial choice | None |
| `<sic>` | As written (error) | None |
| `<corr>` | Correction | None |
| `<reg>` | Regularization | None |
| `<orig>` | Original form | None |
| `<supplied>` | Editor-supplied | None |
| `<unclear>` | Unclear reading | None |
| `<gap>` | Gap in source | None |
| `<damage>` | Damaged area | None |
| `<del>` | Deletion | None |
| `<add>` | Addition | None |
| `<subst>` | Substitution | None |
| `<restore>` | Restoration | None |

### 5.2 Critical Apparatus (MEI.critapp)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<app>` | Apparatus entry | None |
| `<lem>` | Lemma (preferred reading) | None |
| `<rdg>` | Variant reading | None |
| `<witList>` | Witness list | None |
| `<wit>` | Witness reference | None |

### 5.3 Genetic Encoding (MEI.genetic)

| MEI Element/Attribute | Description | MusicXML Equivalent |
|-----------------------|-------------|---------------------|
| `@instant` | Genetic instant | None |
| `@state` | Genetic state | None |
| `<genState>` | Genetic state description | None |

---

## 6. Facsimile & Document Structure (100% Loss)

### 6.1 Facsimile (MEI.facsimile)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<facsimile>` | Image container | None |
| `<surface>` | Physical surface | None |
| `<zone>` | Region on surface | None |
| `@facs` | Facsimile link (on any element) | None |

### 6.2 Page/System Breaks

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<pb>` | Page break | `<print new-page="yes">` |
| `<sb>` | System break | `<print new-system="yes">` |
| `<lb>` | Line break | Lost |
| `<cb>` | Column break | Lost |

---

## 7. Harmonic Notation (100% Loss)

### 7.1 Harmony (MEI.harmony)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<harm>` | Harmonic analysis | None (MusicXML `<harmony>` is chord symbols) |
| `@deg` | Scale degree | None |
| `@mode` | Modal context | None |

### 7.2 Figured Bass

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<fb>` | Figured bass | None |
| `<f>` | Figure | None |
| `@extender` | Extender line | None |

Note: MusicXML has `<figured-bass>` but with different semantics and limited attribute support.

---

## 8. Rich Metadata (80% Loss)

### 8.1 Header Elements

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<meiHead>` | MEI header | `<identification>` (simpler) |
| `<fileDesc>` | File description | Partial mapping |
| `<encodingDesc>` | Encoding description | `<encoding>` (simpler) |
| `<workList>` | Works list | `<work>` (single work) |
| `<revisionDesc>` | Revision history | Lost |
| `<notesStmt>` | Notes | Lost |
| `<sourceDesc>` | Source description | Lost |

### 8.2 Physical Source (MEI.msDesc)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<msDesc>` | Manuscript description | None |
| `<physDesc>` | Physical description | None |
| `<binding>` | Binding info | None |
| `<watermark>` | Watermark info | None |
| `<foliation>` | Foliation | None |

### 8.3 FRBR (MEI.frbr)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<manifestation>` | FRBR manifestation | None |
| `<item>` | FRBR item | None |
| `<expression>` | FRBR expression | None |

---

## 9. Text Elements (Mostly Lost)

### 9.1 Structured Text (MEI.text)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<front>` | Front matter | None |
| `<back>` | Back matter | None |
| `<div>` | Text division | None |
| `<p>` | Paragraph | `<words>` (flattened) |
| `<lg>` | Line group | None |
| `<l>` | Line | None |
| `<titlePage>` | Title page | None |

### 9.2 Names and Dates (MEI.namesdates)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<persName>` | Person name | `<creator>` (simpler) |
| `<corpName>` | Corporate name | None |
| `<geogName>` | Geographic name | None |
| `<date>` | Date element | Flat text |
| `@role`, `@resp` | Role/responsibility | Partial |

---

## 10. Performance & MIDI (Partial Loss)

### 10.1 Performance (MEI.performance)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<performance>` | Performance container | None |
| `<recording>` | Recording info | None |
| `<perfMedium>` | Performance medium | None |
| `<perfRes>` | Performance resource | None |

### 10.2 MIDI (MEI.midi)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<midi>` | MIDI container | `<midi-instrument>` (simpler) |
| `<instrDef>` | Instrument definition | Partial |
| `@midi.channel` | MIDI channel | Preserved |
| `@midi.instrnum` | MIDI program | Preserved |
| `@midi.pan` | Pan | Preserved |
| `@midi.volume` | Volume | Preserved |

---

## 11. User-Defined Content (100% Loss)

### 11.1 User Symbols (MEI.usersymbols)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<symbolTable>` | Symbol definitions | None |
| `<symbolDef>` | Symbol definition | None |
| `<symbol>` | Symbol reference | None |

### 11.2 External Symbols (MEI.externalsymbols)

| MEI Attribute | Description | MusicXML Equivalent |
|---------------|-------------|---------------------|
| `@glyphnum` | SMuFL glyph number | None |
| `@glyphname` | SMuFL glyph name | None |

---

## 12. Drama & Corpus (100% Loss)

### 12.1 Drama (MEI.drama)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<sp>` | Speech | None |
| `<stageDir>` | Stage direction | None |
| `<speaker>` | Speaker | None |

### 12.2 Corpus (MEI.corpus)

| MEI Element | Description | MusicXML Equivalent |
|-------------|-------------|---------------------|
| `<meiCorpus>` | Corpus container | None (multi-file) |

---

## Conversion Recommendations

### When to use MEI → MusicXML conversion:

1. **Basic notation interchange** - simple scores with standard CMN
2. **Import into notation software** - Finale, Sibelius, MuseScore
3. **Cross-platform sharing** - when recipients need MusicXML

### When NOT to convert to MusicXML:

1. **Critical editions** - editorial apparatus will be lost
2. **Early music** - mensural/neume notation has no equivalent
3. **Scholarly encoding** - analytical markup will be lost
4. **Manuscripts** - facsimile links and source info will be lost
5. **Complex metadata** - rich bibliographic data will be flattened

### Preserving MEI-specific data:

Consider using custom MusicXML miscellaneous elements to store MEI-specific data
that might be recovered in a future round-trip. This is not currently implemented
but could be added as an optional feature.

---

## Version History

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-02 | Initial documentation for Phase 4.4 |
